//! POST /v2/wap/login — v2 login (response time format changed to RFC3339).
//!
//! **Breaking changes vs v1**:
//! - `access_exp` (i64 Unix seconds) → `access_expires_at` (String RFC3339)
//! - `refresh_exp` → `refresh_expires_at`
//!
//! Value for clients: skip a `new Date(x * 1000)` call; JSON decoders in many
//! languages will directly produce a Date type.
//!
//! Zero changes in the service layer — only the DTO mapping here is different.

use axum::{
    extract::State,
    http::{header, HeaderMap},
    routing::post,
    Router,
};
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{clock, ApiJson, AppError, AppResult, AppState, ClientIp, ValidatedJson};
use phpyun_services::user_service::{self, LoginContext};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(mlogin))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginForm {
    #[validate(length(min = 3, max = 64))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
    /// Image captcha (PHP field name `authcode`) — optional; only validated together with `captcha_cid`.
    #[serde(default)]
    pub authcode: Option<String>,
    /// Image captcha cid — optional.
    #[serde(default)]
    pub captcha_cid: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginData {
    pub uid: u64,
    pub usertype: u8,
    pub access_token: String,
    /// RFC3339 format: `"2026-04-23T12:34:56+00:00"`
    pub access_expires_at: String,
    pub refresh_token: String,
    pub refresh_expires_at: String,
}

/// Username/password login (v2 time fields changed to RFC3339)
#[utoipa::path(
    post,
    path = "/v2/wap/login",
    tag = "auth",
    request_body = LoginForm,
    responses(
        (status = 200, description = "Login successful", body = LoginData),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many login attempts"),
    )
)]
pub async fn mlogin(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: HeaderMap,
    ValidatedJson(form): ValidatedJson<LoginForm>,
) -> AppResult<ApiJson<LoginData>> {
    if let (Some(cid), Some(code)) = (form.captcha_cid.as_deref(), form.authcode.as_deref()) {
        if !cid.is_empty() && !code.is_empty() {
            let code_up = code.to_uppercase();
            if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, cid, &code_up).await? {
                return Err(AppError::captcha());
            }
        }
    }
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = user_service::login(
        &state,
        &form.username,
        &form.password,
        LoginContext { ip: &ip, ua: &ua },
    )
    .await?;
    Ok(ApiJson(LoginData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        access_expires_at: clock::ts_to_rfc3339(r.access_exp),
        refresh_token: r.refresh,
        refresh_expires_at: clock::ts_to_rfc3339(r.refresh_exp),
    }))
}
