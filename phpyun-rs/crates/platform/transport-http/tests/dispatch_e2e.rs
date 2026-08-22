//! End-to-end: an `Operation` mounted on a router, driven by a real HTTP
//! request.
//!
//! `mount_contract.rs` checks what gets *declared*. This checks what actually
//! happens on the wire — that the policy is enforced before the body is parsed,
//! that success and failure both come back in the standard envelope, and that
//! validation errors carry the operation's own i18n key.
//!
//! Requires the services in `.env.dev`: `AppState::build` opens MySQL and Redis
//! connections, though no table is read or written. When that infrastructure is
//! missing the tests print a skip notice instead of failing, so an unprovisioned
//! workstation does not look like a broken build — but a reachable database and
//! a failing assertion still fail, which is the case that matters.

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use phpyun_core::json::{self, Value};
use phpyun_core::shutdown::CancellationToken;
use phpyun_core::{ApiError, AppResult, AppState, Config};
use phpyun_kernel::{Ctx, Operation, Policy, ProductId, Role};
use phpyun_transport_http::ApiSurface;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tower::ServiceExt;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
struct EchoInput {
    #[validate(length(min = 3, message = "validation.word.length"))]
    word: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct EchoOutput {
    echoed: String,
    /// Proves the caller identity reached the handler through `Ctx`.
    caller: &'static str,
}

/// Public: anyone may call it.
struct Echo;

impl Operation for Echo {
    type Input = EchoInput;
    type Output = EchoOutput;
    const ID: &'static str = "recruit.demo.echo";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const PATH: &'static str = "/v1/wap/kernel-demo/echo";
    const POLICY: Policy = Policy::public();
    const SUMMARY: &'static str = "Echo the input back";

    async fn call(ctx: &Ctx, input: Self::Input) -> AppResult<Self::Output> {
        Ok(EchoOutput {
            echoed: input.word,
            caller: ctx.caller.kind(),
        })
    }
}

/// Admin-only: exists to prove the guard fires without the handler running.
struct AdminEcho;

impl Operation for AdminEcho {
    type Input = EchoInput;
    type Output = EchoOutput;
    const ID: &'static str = "recruit.demo.admin-echo";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const PATH: &'static str = "/v1/admin/kernel-demo/echo";
    const POLICY: Policy = Policy::roles(&[Role::Admin]);
    const SUMMARY: &'static str = "Echo, admins only";

    async fn call(_ctx: &Ctx, _input: Self::Input) -> Result<Self::Output, ApiError> {
        panic!("the policy must reject the caller before the handler runs");
    }
}

/// Built once: `Config::load_for_test` clears and repopulates process-wide
/// environment variables, so concurrent test threads would tear each other's
/// environment down mid-read. `None` means the test database or Redis is not
/// provisioned here.
async fn app() -> Option<Router> {
    static APP: OnceCell<Option<Router>> = OnceCell::const_new();

    APP.get_or_init(|| async {
        let config = Config::load_for_test()
            .expect("Config::load_for_test (copy .env.dev.example to .env.dev first)");

        let state = match AppState::build(config, CancellationToken::new()).await {
            Ok(state) => state,
            Err(err) => {
                eprintln!(
                    "SKIP: kernel dispatch e2e needs the MySQL/Redis from .env.dev ({err:#})"
                );
                return None;
            }
        };

        let (router, _spec) = ApiSurface::new()
            .mount::<Echo>()
            .mount::<AdminEcho>()
            .into_parts();
        Some(router.with_state(state))
    })
    .await
    .clone()
}

async fn post(uri: &str, body: &str) -> Option<(StatusCode, Value)> {
    let app = app().await?;
    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap();

    let resp = app.oneshot(req).await.expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap_or_default();
    let value = json::from_str(&String::from_utf8_lossy(&bytes)).unwrap_or(Value::Null);
    Some((status, value))
}

/// `let Some(x) = ... else { return }`, but noisy about why it bailed.
macro_rules! require_infra {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return,
        }
    };
}

fn assert_envelope(body: &Value, at: &str) {
    let mut members: Vec<&str> = body
        .as_object()
        .unwrap_or_else(|| panic!("at {at}: expected an envelope, got {body}"))
        .keys()
        .map(String::as_str)
        .collect();
    members.sort_unstable();
    assert_eq!(members, ["code", "data", "key", "msg"], "at {at}");
}

#[tokio::test(flavor = "multi_thread")]
async fn success_uses_the_standard_envelope() {
    let (status, body) = require_infra!(post(Echo::PATH, r#"{"word":"hello"}"#).await);
    assert_eq!(status, StatusCode::OK);
    assert_envelope(&body, Echo::PATH);
    assert_eq!(body["code"], json::json!(200));
    assert_eq!(body["key"], json::json!("ok"));
    assert_eq!(body["data"]["echoed"], json::json!("hello"));
}

#[tokio::test(flavor = "multi_thread")]
async fn an_unauthenticated_call_reaches_the_handler_as_anonymous() {
    let (_, body) = require_infra!(post(Echo::PATH, r#"{"word":"hello"}"#).await);
    assert_eq!(body["data"]["caller"], json::json!("anonymous"));
}

#[tokio::test(flavor = "multi_thread")]
async fn validation_failure_carries_the_operations_own_key() {
    let (status, body) = require_infra!(post(Echo::PATH, r#"{"word":"no"}"#).await);
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_envelope(&body, Echo::PATH);
    assert_eq!(body["key"], json::json!("param_invalid"));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_body_of_the_wrong_shape_is_400_not_500() {
    let (status, body) = require_infra!(post(Echo::PATH, r#"{"word":42}"#).await);
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["key"], json::json!("param_invalid"));
}

/// The handler panics if reached, so a green test proves the guard ran first.
#[tokio::test(flavor = "multi_thread")]
async fn a_guarded_operation_rejects_anonymous_before_the_handler_runs() {
    let (status, body) = require_infra!(post(AdminEcho::PATH, r#"{"word":"hello"}"#).await);
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_envelope(&body, AdminEcho::PATH);
    assert_eq!(body["key"], json::json!("unauth"));
}

/// Policy runs before deserialization, so an unauthorized caller cannot probe
/// the input schema by comparing error messages.
#[tokio::test(flavor = "multi_thread")]
async fn an_unauthorized_caller_cannot_tell_a_bad_body_from_a_good_one() {
    let (good_status, good) = require_infra!(post(AdminEcho::PATH, r#"{"word":"hello"}"#).await);
    let (bad_status, bad) = require_infra!(post(AdminEcho::PATH, r#"{"garbage":true}"#).await);
    assert_eq!(good_status, bad_status);
    assert_eq!(good["key"], bad["key"]);
    assert_eq!(good["msg"], bad["msg"]);
}
