//! Black-box integration test scaffolding (axum-test).
//!
//! Wires up the minimal router only, with no DB/Redis — fast for CI to verify the middleware stack and response shape.
//! End-to-end tests that need a real backend (login/logout/cache) live in crates/app/tests/ or xtask.
//!
//! Note: test code also routes through the `phpyun_core::json` facade for consistency.

use axum::{routing::get, Json, Router};
use axum_test::TestServer;
use phpyun_core::json::{self, Value};

fn dummy_router() -> Router {
    Router::new().route("/health", get(|| async { Json(json::json!({"ok": true})) }))
}

#[tokio::test]
async fn health_returns_ok() {
    let server = TestServer::new(dummy_router()).unwrap();
    let resp = server.get("/health").await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["ok"], json::json!(true));
}

#[tokio::test]
async fn unknown_path_404() {
    let server = TestServer::new(dummy_router()).unwrap();
    let resp = server.get("/no-such-path").await;
    resp.assert_status_not_found();
}
