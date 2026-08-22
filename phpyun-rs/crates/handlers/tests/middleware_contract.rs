//! Lock the behaviour of the cross-cutting middleware stack.
//!
//! These used to be hardcoded `if path.starts_with("/v1/")` and literal path
//! comparisons inside `phpyun_core::middleware`, which meant `/v2/*` silently
//! escaped envelope normalization and every new namespace required editing
//! generic infrastructure. The policy now comes from
//! [`RouteRules`](phpyun_core::route_rules::RouteRules), and these tests assert
//! the stack honours it end to end.
//!
//! Only `Config` is required — no database or Redis — but `Config::load_for_test`
//! still reads `.env.dev`, so copy `.env.dev.example` first.

use std::net::SocketAddr;
use std::sync::OnceLock;

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Method, Request, StatusCode},
    routing::{get, post},
    Router,
};
use phpyun_core::json::{self, Value};
use phpyun_core::{middleware as mw, route_rules::RouteRules, ApiResponse, AppResult, Config};
use tower::ServiceExt;

const WECHAT_CALLBACK: &str = "/v1/wap/wechat/callback";

async fn ok() -> AppResult<ApiResponse<&'static str>> {
    Ok(ApiResponse::data("pong"))
}

/// `Config::load_for_test` clears and repopulates process-wide environment
/// variables, so concurrent test threads would tear each other's environment
/// down mid-read. Load exactly once and share the result.
fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::load_for_test().expect("Config::load_for_test (copy .env.dev.example to .env.dev)")
    })
}

/// Mirrors the real assembly in `routes.rs`: two API namespaces plus one
/// GET-exempt path owned by a route module.
fn app() -> Router {
    let config = config().clone();

    let api = Router::new()
        .route("/v1/wap/ping", post(ok))
        .route("/v2/wap/ping", post(ok))
        .route(WECHAT_CALLBACK, get(ok).post(ok));

    let rules = RouteRules::new()
        .api_namespace("/v1")
        .api_namespace("/v2")
        .allow_get(WECHAT_CALLBACK);

    Router::new()
        .route("/health", get(ok))
        .merge(mw::install(api, &config, rules))
}

/// The governor's default key extractor needs a peer address; the real server
/// supplies it via `into_make_service_with_connect_info`.
async fn send(method: Method, uri: &str) -> (StatusCode, Value) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from("{}"))
        .unwrap();
    req.extensions_mut().insert(ConnectInfo(
        "10.99.0.1:65535".parse::<SocketAddr>().unwrap(),
    ));

    let resp = app().oneshot(req).await.expect("router oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap_or_default();
    let body = json::from_str(&String::from_utf8_lossy(&bytes)).unwrap_or(Value::Null);
    (status, body)
}

fn assert_envelope(body: &Value, key: &str, code: u16, at: &str) {
    let obj = body
        .as_object()
        .unwrap_or_else(|| panic!("at {at}: expected a JSON envelope, got {body}"));
    let mut members: Vec<&str> = obj.keys().map(String::as_str).collect();
    members.sort_unstable();
    assert_eq!(members, ["code", "data", "key", "msg"], "at {at}");
    assert_eq!(body["key"], json::json!(key), "at {at}");
    assert_eq!(body["code"], json::json!(code), "at {at}");
}

#[tokio::test]
async fn business_apis_accept_post() {
    for uri in ["/v1/wap/ping", "/v2/wap/ping"] {
        let (status, body) = send(Method::POST, uri).await;
        assert_eq!(status, StatusCode::OK, "at {uri}");
        assert_eq!(body["data"], json::json!("pong"), "at {uri}");
    }
}

#[tokio::test]
async fn business_apis_reject_get_with_the_json_envelope() {
    for uri in ["/v1/wap/ping", "/v2/wap/ping"] {
        let (status, body) = send(Method::GET, uri).await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED, "at {uri}");
        assert_envelope(&body, "method_not_allowed", 405, uri);
    }
}

/// Regression: `normalize_api_rejections` used to match `/v1/` literally, so
/// `/v2/*` rejections escaped as bare text with a mismatched status.
#[tokio::test]
async fn v2_rejections_are_normalized_like_v1() {
    let (v1_status, v1_body) = send(Method::DELETE, "/v1/wap/ping").await;
    let (v2_status, v2_body) = send(Method::DELETE, "/v2/wap/ping").await;

    assert_eq!(v1_status, v2_status);
    assert_eq!(v1_body["key"], v2_body["key"]);
    assert_eq!(v1_body["code"], v2_body["code"]);
    assert_envelope(&v2_body, "method_not_allowed", 405, "/v2/wap/ping");
}

#[tokio::test]
async fn unmatched_api_routes_return_a_json_not_found() {
    let (status, body) = send(Method::POST, "/v1/wap/no-such-endpoint").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_envelope(&body, "not_found", 404, "/v1/wap/no-such-endpoint");
}

#[tokio::test]
async fn exempt_paths_still_accept_get() {
    let (status, body) = send(Method::GET, WECHAT_CALLBACK).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"], json::json!("pong"));
}

#[tokio::test]
async fn exemption_does_not_leak_to_sibling_paths() {
    let (status, _) = send(Method::GET, "/v1/wap/wechat/callback/extra").await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn ops_probes_are_outside_the_api_contract() {
    let (status, body) = send(Method::GET, "/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"], json::json!("pong"));
}
