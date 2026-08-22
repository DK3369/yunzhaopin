//! The handshake is an ordinary authenticated HTTP request.
//!
//! The claim this file checks is that the socket is not a second front door:
//! an upgrade with no credentials is refused by the same extractor, with the
//! same status and the same envelope, as any REST endpoint.
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
use phpyun_transport_ws::{routes, Hub, WS_PATH};
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
            Ok(state) => Some(routes(Hub::new()).with_state(state)),
            Err(e) => {
                eprintln!("SKIP: ws handshake tests need MySQL and Redis from .env.dev ({e})");
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

/// A browser's upgrade request, minus whatever the test wants to leave out.
fn upgrade_request() -> axum::http::request::Builder {
    Request::builder()
        .uri(WS_PATH)
        .header(header::CONNECTION, "Upgrade")
        .header(header::UPGRADE, "websocket")
        .header(header::SEC_WEBSOCKET_VERSION, "13")
        .header(header::SEC_WEBSOCKET_KEY, "dGhlIHNhbXBsZSBub25jZQ==")
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .expect("body");
    json::from_str(&String::from_utf8_lossy(&bytes)).expect("json body")
}

#[tokio::test]
async fn an_anonymous_upgrade_is_refused() {
    let app = require_infra!();

    let response = app
        .oneshot(upgrade_request().body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = body_json(response).await;
    assert_eq!(body["code"], 401);
    assert_eq!(
        body["key"], "unauth",
        "the socket must speak the same error vocabulary as the REST API"
    );
}

#[tokio::test]
async fn a_forged_token_does_not_open_a_socket() {
    let app = require_infra!();

    let response = app
        .oneshot(
            upgrade_request()
                .header(header::AUTHORIZATION, "Bearer not.a.jwt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Authentication runs before the upgrade is negotiated, so a caller without
/// credentials cannot even tell whether their handshake was well-formed.
#[tokio::test]
async fn credentials_are_checked_before_the_upgrade_headers() {
    let app = require_infra!();

    let response = app
        .oneshot(Request::builder().uri(WS_PATH).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "a missing Upgrade header must not leak a different status"
    );
}

/// The route is deliberately outside `/v1`, where the method filter would turn
/// this GET into a 405.
#[test]
fn the_socket_path_is_outside_the_post_only_api_namespace() {
    assert!(!WS_PATH.starts_with("/v1"));
    assert!(!WS_PATH.starts_with("/v2"));
}
