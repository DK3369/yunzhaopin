//! POST /v1/wap/login -- username/password login (aligned with PHPYun `wap/login::mlogin_action`).
//!
//! Request parameters:
//! - `username`    -- username / mobile / email (PHPYun supports any of the three)
//! - `password`    -- password
//! - `authcode`    -- image captcha (**mandatory in PHP**; here it is opt-in: only verified together with `captcha_cid`)
//! - `captcha_cid` -- image captcha cid (specific to phpyun-rs; PHP uses session)
//!
//! Note: SMS dynamic-code login (PHP's `act_login=1 + moblie + dynamiccode`) is not yet implemented; scheduled for the next round.

use axum::{
    extract::State,
    http::{header, HeaderMap},
    routing::post,
    Router,
};
use phpyun_core::dto::AuthTokenData;
use phpyun_core::validators;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, ClientIp, ValidatedJson};
use phpyun_services::user_service::{self, LoginContext};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(mlogin))
        .route("/login/sms", post(login_sms))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginForm {
    // message holds an i18n key (translated per Accept-Language when materialized in IntoResponse)
    #[validate(length(min = 3, max = 64, message = "validation.username.length"))]
    pub username: String,
    #[validate(length(min = 6, max = 128, message = "validation.password.length"))]
    pub password: String,

    /// Image captcha (PHP field name `authcode`) -- optional.
    /// Only verified when paired with `captcha_cid`; if neither or only one is supplied, verification is skipped (aligned with PHPYun `wap/login::mlogin` behavior).
    #[serde(default)]
    #[validate(length(max = 500))]
    pub authcode: Option<String>,

    /// Image captcha cid (specific to phpyun-rs; PHP uses session) -- optional.
    #[serde(default)]
    #[validate(length(max = 500))]
    pub captcha_cid: Option<String>,
}

/// Username/password login
#[utoipa::path(
    post,
    path = "/v1/wap/login",
    tag = "auth",
    request_body = LoginForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 400, description = "Invalid captcha / invalid parameter"),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many login attempts"),
    )
)]
pub async fn mlogin(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: HeaderMap,
    ValidatedJson(form): ValidatedJson<LoginForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    // Opt-in verification: only verify when cid + authcode are supplied together (aligned with PHP mlogin behavior).
    if let (Some(cid), Some(code)) = (form.captcha_cid.as_deref(), form.authcode.as_deref()) {
        if !cid.is_empty() && !code.is_empty() {
            let code_up = code.to_uppercase();
            if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, cid, &code_up).await? {
                return Err(AppError::captcha());
            }
        }
    }

    let ua = ua_from(&headers);
    let r = user_service::login(
        &state,
        &form.username,
        &form.password,
        LoginContext { ip: &ip, ua: &ua },
    )
    .await?;
    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
    }))
}

// ==================== SMS dynamic-code login ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginSmsForm {
    #[validate(custom(function = "validators::cn_mobile"))]
    pub moblie: String,

    /// SMS verification code (PHP field name `dynamiccode`)
    #[validate(custom(function = "validators::captcha"))]
    pub dynamiccode: String,
}

/// SMS dynamic-code login (aligned with the `act_login=1` branch of PHPYun `mlogin_action`)
///
/// Prerequisite: first call `POST /v1/wap/sms/send` with `scene=login` to send the code.
#[utoipa::path(
    post,
    path = "/v1/wap/login/sms",
    tag = "auth",
    request_body = LoginSmsForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 401, description = "Invalid code / account not found"),
        (status = 429, description = "Too many login attempts"),
    )
)]
pub async fn login_sms(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: HeaderMap,
    ValidatedJson(form): ValidatedJson<LoginSmsForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    let ua = ua_from(&headers);
    let r = user_service::login_with_sms_code(
        &state,
        &form.moblie,
        &form.dynamiccode,
        LoginContext { ip: &ip, ua: &ua },
    )
    .await?;
    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
    }))
}

/// Pull the User-Agent header out of a request, lossily — empty string when
/// missing or non-UTF-8.
fn ua_from(headers: &HeaderMap) -> String {
    headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}
