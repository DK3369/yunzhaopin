//! Versioned-route coexistence tests.
//!
//! Locks down:
//! 1. The same logical endpoint (/wap/login) works under both `/v1` and `/v2`
//! 2. Response **shape differs** (v1 uses Unix seconds `access_exp`, v2 uses RFC3339 `access_expires_at`)
//! 3. Ops endpoint `/health` has no version prefix
//! 4. Unknown version `/v99/*` → 404 (explicit routing, no fallback)
//! 5. Business endpoints without a version prefix (`/wap/login`) → 404 (force clients to declare version)
//!
//! This test uses local stub handlers; it does not start a real DB/Redis. It only proves
//! that "route assembly and dispatch" is correct.

use axum::{routing::post, Router};
use axum_test::TestServer;
use phpyun_core::json::{self, Value};
use phpyun_core::{ApiJson, AppResult};
use serde::Serialize;

// ---- Mock v1 login response (Unix seconds) ----
#[derive(Serialize)]
struct V1LoginData {
    uid: u64,
    access_exp: i64,
}

async fn v1_login_stub() -> AppResult<ApiJson<V1LoginData>> {
    Ok(ApiJson(V1LoginData {
        uid: 1,
        access_exp: 1_800_000_000,
    }))
}

// ---- Mock v2 login response (RFC3339 string) ----
#[derive(Serialize)]
struct V2LoginData {
    uid: u64,
    access_expires_at: String,
}

async fn v2_login_stub() -> AppResult<ApiJson<V2LoginData>> {
    Ok(ApiJson(V2LoginData {
        uid: 1,
        access_expires_at: "2027-01-15T08:00:00+00:00".into(),
    }))
}

async fn health_stub() -> axum::Json<Value> {
    axum::Json(json::json!({"ok": true}))
}

/// Mirrors the real repo assembly: nest /v1, /v2; health at the root path
fn mock_build_router() -> Router {
    let v1 = Router::new()
        .nest("/wap", Router::new().route("/login", post(v1_login_stub)));
    let v2 = Router::new()
        .nest("/wap", Router::new().route("/login", post(v2_login_stub)));

    Router::new()
        .nest("/v1", v1)
        .nest("/v2", v2)
        .route("/health", axum::routing::get(health_stub))
}

// ==================== Tests ====================

#[tokio::test]
async fn v1_and_v2_coexist_with_different_shapes() {
    let server = TestServer::new(mock_build_router()).unwrap();

    // v1: access_exp is a number
    let body: Value = server.post("/v1/wap/login").await.json();
    assert_eq!(body["code"], json::json!(200));
    assert_eq!(body["data"]["uid"], json::json!(1));
    assert!(body["data"]["access_exp"].is_i64(), "v1 access_exp must be a number");
    assert!(
        body["data"]["access_expires_at"].is_null(),
        "v1 should not have an access_expires_at field"
    );

    // v2: access_expires_at is an RFC3339 string
    let body: Value = server.post("/v2/wap/login").await.json();
    assert_eq!(body["code"], json::json!(200));
    assert_eq!(body["data"]["uid"], json::json!(1));
    assert!(
        body["data"]["access_expires_at"].is_string(),
        "v2 access_expires_at must be a string"
    );
    assert!(body["data"]["access_exp"].is_null(), "v2 should not have an access_exp field");
}

#[tokio::test]
async fn health_has_no_version_prefix() {
    let server = TestServer::new(mock_build_router()).unwrap();
    let resp = server.get("/health").await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["ok"], json::json!(true));
}

#[tokio::test]
async fn unknown_version_returns_404() {
    let server = TestServer::new(mock_build_router()).unwrap();
    // /v99/... is not mounted; should return 404 rather than falling back to a default version
    let resp = server.post("/v99/wap/login").await;
    resp.assert_status_not_found();
}

#[tokio::test]
async fn unversioned_business_path_returns_404() {
    let server = TestServer::new(mock_build_router()).unwrap();
    // Intentionally skip /v1 or /v2, hit /wap/login directly — should be 404, forcing clients to declare a version
    let resp = server.post("/wap/login").await;
    resp.assert_status_not_found();
}

/// The v2 router reuses unchanged endpoints via `.merge(v1 auth)`.
/// This test simulates that reuse, ensuring clients can hit /v2/wap/logout.
#[tokio::test]
async fn v2_reuses_v1_handlers_for_unchanged_endpoints() {
    async fn shared_logout() -> AppResult<ApiJson<Value>> {
        Ok(ApiJson(json::json!({"revoked": true})))
    }
    // v1 defines logout
    let v1_auth = Router::<()>::new().route("/logout", post(shared_logout));
    // v2 directly .merge(v1's auth) to reuse it
    let v1 = Router::new().nest("/wap", v1_auth.clone());
    let v2 = Router::new().nest("/wap", Router::new().merge(v1_auth));

    let app = Router::new().nest("/v1", v1).nest("/v2", v2);
    let server = TestServer::new(app).unwrap();

    // Both paths should work and return the same content
    let r1: Value = server.post("/v1/wap/logout").await.json();
    let r2: Value = server.post("/v2/wap/logout").await.json();
    assert_eq!(r1, r2);
    assert_eq!(r1["data"]["revoked"], json::json!(true));
}
