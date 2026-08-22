//! Lock the API response contract for the unified application error type.
//!
//! 1. **Contract**: every response carries `{code, key, msg, data}`. Success is
//!    `{200, "ok", "ok", <payload>}`; failure is `{<HTTP>, <stable key>,
//!    <localized copy>, ""}`.
//! 2. **`code` equals the HTTP status**, and each `ApiErrorKind` maps to the
//!    status that actually describes it — client mistakes are 4xx, only genuine
//!    backend faults are 5xx.
//! 3. **`key` is machine-readable and free of detail text**; `msg` is localized
//!    and display-only.

use axum::{routing::get, Router};
use axum_test::TestServer;
use phpyun_core::i18n::{self, Lang};
use phpyun_core::json::{self, Value};
use phpyun_core::{ApiError, ApiResponse, AppResult};
use serde::Serialize;

/// Resolve `errors.<tag>` through i18n the same way `ApiError::into_response`
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

// ==================== handlers ====================

async fn ok_handler() -> AppResult<ApiResponse<Payload>> {
    Ok(ApiResponse::data(Payload {
        name: "alice",
        n: 42,
    }))
}

async fn err_unauth() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::unauth())
}

async fn err_session() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::session_expired())
}

async fn err_locked() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::locked())
}

async fn err_rate() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::rate_limit())
}

async fn err_upstream() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::upstream("sms gateway timeout"))
}

async fn err_internal() -> AppResult<ApiResponse<Payload>> {
    // sqlx::Error auto-converts to ApiError (database errors are 500 / db).
    Err(sqlx::Error::RowNotFound.into())
}

async fn err_param() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::param_invalid("bad email"))
}

async fn err_business() -> AppResult<ApiResponse<Payload>> {
    Err(ApiError::business("job_not_found"))
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
        .route("/err/business", get(err_business))
}

// ==================== Contract tests ====================

/// The envelope has exactly these members — no more, no less. Adding a field
/// silently is a breaking change for every client, so it has to break here
/// first.
fn assert_envelope_shape(body: &Value, at: &str) {
    let obj = body.as_object().expect("envelope is a JSON object");
    let mut members: Vec<&str> = obj.keys().map(String::as_str).collect();
    members.sort_unstable();
    assert_eq!(members, ["code", "data", "key", "msg"], "at {at}");
}

#[tokio::test]
async fn success_is_200_ok_with_data() {
    let server = TestServer::new(router()).unwrap();
    let resp = server.get("/ok").await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_envelope_shape(&body, "/ok");
    assert_eq!(body["code"], json::json!(200));
    assert_eq!(body["key"], json::json!("ok"));
    assert_eq!(body["msg"], json::json!("ok"));
    assert_eq!(body["data"]["name"], json::json!("alice"));
    assert_eq!(body["data"]["n"], json::json!(42));
}

#[tokio::test]
async fn auth_errors_are_401_with_specific_keys() {
    // Contract: `body.msg` is the **i18n-translated** copy of `errors.<key>`,
    // not the raw English tag. Tests resolve the same key through `i18n::t`
    // so they pass regardless of which language is the default.
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/unauth").await.json();
    assert_envelope_shape(&body, "/err/unauth");
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["key"], json::json!("unauth"));
    assert_eq!(body["data"], json::json!(""));
    assert_eq!(body["msg"], json::json!(translated_msg("unauth")));

    let body: Value = server.get("/err/session").await.json();
    assert_eq!(body["code"], json::json!(401));
    assert_eq!(body["key"], json::json!("session_expired"));
    assert_eq!(body["msg"], json::json!(translated_msg("session_expired")));
}

#[tokio::test]
async fn client_faults_are_4xx_and_server_faults_are_5xx() {
    let server = TestServer::new(router()).unwrap();
    for (path, expected_code, expected_key) in [
        ("/err/param", 400u16, "param_invalid"),
        ("/err/unauth", 401, "unauth"),
        ("/err/session", 401, "session_expired"),
        ("/err/locked", 403, "locked"),
        ("/err/business", 422, "job_not_found"),
        ("/err/rate", 429, "rate_limit"),
        ("/err/internal", 500, "db"),
        ("/err/upstream", 502, "upstream"),
    ] {
        let resp = server.get(path).await;
        assert_eq!(
            resp.status_code().as_u16(),
            expected_code,
            "HTTP status at {path}"
        );

        let body: Value = resp.json();
        assert_envelope_shape(&body, path);
        assert_eq!(
            body["code"],
            json::json!(expected_code),
            "body.code must equal the HTTP status at {path}"
        );
        assert_eq!(body["key"], json::json!(expected_key), "body.key at {path}");
        assert_eq!(body["data"], json::json!(""), "body.data at {path}");
    }
}

#[tokio::test]
async fn msg_is_always_translated_never_a_raw_key() {
    let server = TestServer::new(router()).unwrap();
    for path in [
        "/err/param",
        "/err/locked",
        "/err/rate",
        "/err/upstream",
        "/err/internal",
        "/err/business",
    ] {
        let body: Value = server.get(path).await.json();
        let msg = body["msg"].as_str().expect("msg is a string");
        assert!(!msg.is_empty(), "at {path}: msg must not be empty");
        assert!(
            !msg.starts_with("errors."),
            "at {path}: msg must be translated, got {msg:?}"
        );
    }

    // Detail-free keys resolve to the i18n table verbatim.
    let body: Value = server.get("/err/locked").await.json();
    assert_eq!(body["msg"], json::json!(translated_msg("locked")));
    let body: Value = server.get("/err/rate").await.json();
    assert_eq!(body["msg"], json::json!(translated_msg("rate_limit")));
    let body: Value = server.get("/err/internal").await.json();
    assert_eq!(body["msg"], json::json!(translated_msg("db")));
}

#[tokio::test]
async fn detail_bearing_errors_keep_a_detail_free_key() {
    // `param_invalid("bad email")` and `upstream("sms gateway timeout")` carry
    // free text. That text may appear in `msg` via the `*_with` template, but
    // `key` must stay stable so clients can match on it.
    let server = TestServer::new(router()).unwrap();
    let body: Value = server.get("/err/param").await.json();
    assert_eq!(body["key"], json::json!("param_invalid"));

    let body: Value = server.get("/err/upstream").await.json();
    assert_eq!(body["key"], json::json!("upstream"));
}
