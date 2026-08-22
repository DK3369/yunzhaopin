//! Opening a stream is an ordinary authenticated HTTP request.
//!
//! The claim this file checks is that `/sse` is not a second front door. A
//! stream is long-lived and unattended, so an anonymous one that merely stays
//! empty would be worse than a rejected request: it holds a session slot, and
//! the caller has no error to act on. Authentication therefore runs before
//! anything else and answers with the same status and the same envelope as any
//! REST endpoint.
//!
//! The authenticated path needs a live session row, which only the product's
//! login flow writes; it is covered end to end against a running server rather
//! than reconstructed here.
//!
//! Requires the services in `.env.dev`; where they are missing the test prints
//! a skip notice rather than failing.

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    Router,
};
use phpyun_core::json::{self, Value};
use phpyun_core::shutdown::CancellationToken;
use phpyun_core::{AppState, Config};
use phpyun_push::Hub;
use phpyun_transport_sse::{routes, Replays, SSE_PATH};
use tokio::sync::OnceCell;
use tower::ServiceExt;

/// Built once: `Config::load_for_test` clears and repopulates process-wide
/// environment variables, so concurrent test threads would tear each other's
/// environment down mid-read.
async fn app() -> Option<Router> {
    static APP: OnceCell<Option<Router>> = OnceCell::const_new();

    APP.get_or_init(|| async {
        let config = Config::load_for_test()
            .expect("Config::load_for_test (copy .env.dev.example to .env.dev first)");
        match AppState::build(config, CancellationToken::new()).await {
            Ok(state) => Some(routes(Hub::new(), Replays::new()).with_state(state)),
            Err(e) => {
                eprintln!("SKIP: sse handshake tests need MySQL and Redis from .env.dev ({e})");
                None
            }
        }
    })
    .await
    .clone()
}

macro_rules! require_infra {
    () => {
        match app().await {
            Some(app) => app,
            None => return,
        }
    };
}

fn subscribe(query: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("{SSE_PATH}{query}"))
        .header(header::ACCEPT, "text/event-stream")
        .body(Body::empty())
        .unwrap()
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .expect("body");
    json::from_str(&String::from_utf8_lossy(&bytes)).expect("json body")
}

#[tokio::test]
async fn an_anonymous_subscription_is_refused() {
    let app = require_infra!();

    let response = app.oneshot(subscribe("?topics=chat")).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_ne!(
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok()),
        Some("text/event-stream"),
        "a refusal must be an answer, not an empty stream"
    );

    let body = body_json(response).await;
    assert_eq!(body["code"], 401);
    assert_eq!(
        body["key"], "unauth",
        "the stream must speak the same error vocabulary as the REST API"
    );
}

#[tokio::test]
async fn a_forged_token_does_not_open_a_stream() {
    let app = require_infra!();

    let response = app
        .oneshot(
            Request::builder()
                .uri(SSE_PATH)
                .header(header::AUTHORIZATION, "Bearer not.a.jwt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Authentication runs before the subscription is parsed, so a caller without
/// credentials learns nothing about which topics exist.
#[tokio::test]
async fn credentials_are_checked_before_the_topic_list() {
    let app = require_infra!();

    let response = app.oneshot(subscribe("?topics=admin.ops")).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "an anonymous caller must not be able to probe the catalogue"
    );
}

/// A cursor is untrusted input that reaches a database query on the
/// authenticated path; a malformed one must not change the answer here.
#[tokio::test]
async fn a_hostile_cursor_still_just_gets_a_401() {
    let app = require_infra!();

    for cursor in ["chat:9999999999999999999999", "chat:-1", "'; DROP TABLE--"] {
        let response = app
            .clone()
            .oneshot(subscribe(&format!("?since={cursor}")))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{cursor:?}");
    }
}

/// The route is deliberately outside `/v1`, where the method filter would turn
/// this GET into a 405.
#[test]
fn the_stream_path_is_outside_the_post_only_api_namespace() {
    assert!(!SSE_PATH.starts_with("/v1"));
    assert!(!SSE_PATH.starts_with("/v2"));
}
