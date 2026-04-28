//! Lock the API response contract + verify the pluggable error architecture.
//!
//! 1. **Contract**: success `{code: 200, msg: "ok", data}` / failure `{code: <HTTP>, msg: <tag>, data: null}`
//! 2. **Pluggable**: this file **defines a local DomainError** inside the handlers crate, just
//!    impls `ApiError` — without touching core or services — and is able to return new codes and tags.
//!    If this works, it proves that future new domains like payment / order / resume can be
//!    added the same way: just define their own error enum + impl ApiError.

use axum::{routing::get, Router};
use axum_test::TestServer;
use phpyun_core::i18n::{self, Lang};
use phpyun_core::json::{self, Value};
use phpyun_core::{ApiError, ApiJson, AppError, AppResult, InfraError};
use serde::Serialize;
use std::borrow::Cow;

/// Resolve `errors.<tag>` through i18n the same way `AppError::into_response`
/// does on a request without the `lang_layer` middleware (default lang).
/// Tests use this to assert against the real translated message instead of
/// hard-coding either the English tag or specific Chinese copy.
fn translated_msg(tag: &str) -> String {
    let lang = Lang::default();
    let key = format!("errors.{tag}");
    let translated = i18n::t(&key, lang);
    if translated == key {
        // No translation registered — falls back to the raw tag.
        tag.to_string()
    } else {
        translated
    }
}

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
    // Contract: `body.msg` is the **i18n-translated** copy of `errors.<tag>`,
    // not the raw English tag. Tests resolve the same key through `i18n::t`
    // so they pass regardless of which language is the default.
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/unauth").await.json();
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["msg"], json::json!(translated_msg("unauth")));

    let body: Value = server.get("/err/session").await.json();
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["msg"], json::json!(translated_msg("session_expired")));
}

#[tokio::test]
async fn locked_403_rate_429_upstream_502() {
    let server = TestServer::new(router()).unwrap();
    for (path, expected_code, tag) in [
        ("/err/locked", 403, "locked"),
        ("/err/rate", 429, "rate_limit"),
        // `upstream` carries a free-text detail; the response uses the
        // `errors.upstream_with` template when available, otherwise the bare
        // `errors.upstream`. We only assert that translation happened.
        ("/err/upstream", 502, "upstream"),
    ] {
        let body: Value = server.get(path).await.json();
        assert_eq!(body["code"], json::json!(expected_code), "at {path}");
        let msg = body["msg"].as_str().expect("msg is a string");
        assert!(
            !msg.is_empty(),
            "at {path}: msg should be a non-empty translated string"
        );
        // Sanity: msg should not still contain the raw `errors.` namespace.
        assert!(
            !msg.starts_with("errors."),
            "at {path}: msg should be translated, got {msg:?}"
        );
        if tag == "locked" || tag == "rate_limit" {
            // Stable single-key lookups should match the i18n table exactly.
            assert_eq!(msg, translated_msg(tag), "at {path}");
        }
    }
}

#[tokio::test]
async fn param_invalid_400() {
    // `param_invalid` is raised with a detail (`"bad email"`); the response
    // uses the `errors.param_invalid_with` template (with `%{detail}`) when
    // available, otherwise the bare `errors.param_invalid`. Either way the
    // translated copy should be a non-empty, non-key string and contain the
    // detail when the template was used.
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/param").await.json();
    assert_eq!(body["code"], json::json!(400));
    let msg = body["msg"].as_str().expect("msg is a string");
    assert!(!msg.is_empty(), "msg should be a non-empty translated string");
    assert!(!msg.starts_with("errors."), "msg should be translated, got {msg:?}");
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
