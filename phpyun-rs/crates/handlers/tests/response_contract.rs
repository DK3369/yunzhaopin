//! Lock the API response contract + verify the pluggable error architecture.
//!
//! 1. **Contract**: success `{code: 200, msg: "ok", data}` / failure `{code: <HTTP>, msg: <tag>, data: null}`
//! 2. **Pluggable**: this file **defines a local DomainError** inside the handlers crate, just
//!    impls `ApiError` — without touching core or services — and is able to return new codes and tags.
//!    If this works, it proves that future new domains like payment / order / resume can be
//!    added the same way: just define their own error enum + impl ApiError.

use axum::{routing::get, Router};
use axum_test::TestServer;
use phpyun_core::json::{self, Value};
use phpyun_core::{ApiError, ApiJson, AppError, AppResult, InfraError};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize)]
struct Payload {
    name: &'static str,
    n: u32,
}

// ==================== Local domain error (demonstrates "pluggability") ====================

#[derive(thiserror::Error, Debug, Clone)]
enum DemoDomainError {
    #[error("demo resource expired")]
    Expired,
    #[error("demo quota exceeded: {0}")]
    QuotaExceeded(u64),
}

impl ApiError for DemoDomainError {
    fn code(&self) -> u16 {
        match self {
            Self::Expired => 410, // HTTP 410 Gone
            Self::QuotaExceeded(_) => 402, // HTTP 402 Payment Required
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::Expired => "demo_expired".into(),
            Self::QuotaExceeded(_) => "demo_quota".into(),
        }
    }
}

// ==================== handlers ====================

async fn ok_handler() -> AppResult<ApiJson<Payload>> {
    Ok(ApiJson(Payload { name: "alice", n: 42 }))
}

async fn err_unauth() -> AppResult<ApiJson<Payload>> {
    Err(AppError::unauth())
}

async fn err_session() -> AppResult<ApiJson<Payload>> {
    Err(AppError::session_expired())
}

async fn err_locked() -> AppResult<ApiJson<Payload>> {
    Err(AppError::locked())
}

async fn err_rate() -> AppResult<ApiJson<Payload>> {
    Err(AppError::rate_limit())
}

async fn err_upstream() -> AppResult<ApiJson<Payload>> {
    Err(AppError::upstream("sms gateway timeout"))
}

async fn err_internal() -> AppResult<ApiJson<Payload>> {
    // sqlx::Error auto-converts to AppError via From (SystemError::Database → 500 / db)
    Err(sqlx::Error::RowNotFound.into())
}

async fn err_param() -> AppResult<ApiJson<Payload>> {
    Err(InfraError::InvalidParam("bad email".into()).into())
}

// — Key: the two handlers below use "domain-defined custom errors" —
async fn err_demo_expired() -> AppResult<ApiJson<Payload>> {
    Err(DemoDomainError::Expired.into())
}

async fn err_demo_quota() -> AppResult<ApiJson<Payload>> {
    Err(DemoDomainError::QuotaExceeded(1024).into())
}

fn router() -> Router {
    Router::new()
        .route("/ok", get(ok_handler))
        .route("/err/unauth", get(err_unauth))
        .route("/err/session", get(err_session))
        .route("/err/locked", get(err_locked))
        .route("/err/rate", get(err_rate))
        .route("/err/upstream", get(err_upstream))
        .route("/err/internal", get(err_internal))
        .route("/err/param", get(err_param))
        .route("/err/demo/expired", get(err_demo_expired))
        .route("/err/demo/quota", get(err_demo_quota))
}

// ==================== Contract tests ====================

#[tokio::test]
async fn success_is_200_ok_with_data() {
    let server = TestServer::new(router()).unwrap();
    let resp = server.get("/ok").await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["code"], json::json!(200));
    assert_eq!(body["msg"], json::json!("ok"));
    assert_eq!(body["data"]["name"], json::json!("alice"));
    assert_eq!(body["data"]["n"], json::json!(42));
}

#[tokio::test]
async fn auth_errors_401_with_specific_tags() {
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/unauth").await.json();
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["msg"], json::json!("unauth"));

    let body: Value = server.get("/err/session").await.json();
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["msg"], json::json!("session_expired"));
}

#[tokio::test]
async fn locked_403_rate_429_upstream_502() {
    let server = TestServer::new(router()).unwrap();
    for (path, expected_code, expected_tag) in [
        ("/err/locked", 403, "locked"),
        ("/err/rate", 429, "rate_limit"),
        ("/err/upstream", 502, "upstream"),
    ] {
        let body: Value = server.get(path).await.json();
        assert_eq!(body["code"], json::json!(expected_code), "at {path}");
        assert_eq!(body["msg"], json::json!(expected_tag), "at {path}");
    }
}

#[tokio::test]
async fn param_invalid_400() {
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/param").await.json();
    assert_eq!(body["code"], json::json!(400));
    assert_eq!(body["msg"], json::json!("param_invalid"));
}

#[tokio::test]
async fn sqlx_auto_converts_to_500_db() {
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/internal").await.json();
    assert_eq!(body["code"], json::json!(500));
    assert_eq!(body["msg"], json::json!("db"));
}

// ==================== Pluggability tests (the key proof) ====================

#[tokio::test]
async fn custom_domain_error_plugs_in_with_zero_core_changes() {
    let server = TestServer::new(router()).unwrap();

    // 410 / demo_expired — this code and tag are defined only in this file; core has never seen them
    let body: Value = server.get("/err/demo/expired").await.json();
    assert_eq!(body["code"], json::json!(410));
    assert_eq!(body["msg"], json::json!("demo_expired"));

    // 402 / demo_quota — same as above
    let body: Value = server.get("/err/demo/quota").await.json();
    assert_eq!(body["code"], json::json!(402));
    assert_eq!(body["msg"], json::json!("demo_quota"));
}

#[tokio::test]
async fn body_code_equals_http_status_across_all_variants() {
    let server = TestServer::new(router()).unwrap();
    for (path, expected) in [
        ("/err/unauth", 401u16),
        ("/err/locked", 403),
        ("/err/rate", 429),
        ("/err/upstream", 502),
        ("/err/internal", 500),
        ("/err/param", 400),
        ("/err/demo/expired", 410),
        ("/err/demo/quota", 402),
    ] {
        let resp = server.get(path).await;
        assert_eq!(resp.status_code().as_u16(), expected, "HTTP status at {path}");
        let body: Value = resp.json();
        assert_eq!(
            body["code"],
            json::json!(expected),
            "body.code must match HTTP status at {path}"
        );
    }
}
