//! Forgot password (aligned with PHPYun `wap/forgetpw`). Parameter names match PHP: `moblie` / `moblie_code` / `password`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::json;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{
    validators, ApiJson, ApiOk, AppError, AppResult, AppState, ClientIp, ValidatedJson,
};
use phpyun_services::password_reset_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/forgetpw/send-sms", post(send_sms))
        .route("/forgetpw/send-email", post(send_email))
        .route("/forgetpw/reset", post(reset))
        .route("/forgetpw/reset-by-email", post(reset_by_email))
        .route("/forgetpw/appeal", post(submit_appeal))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendSmsForm {
    /// PHPYun field name `moblie`
    #[validate(custom(function = "validators::cn_mobile"))]
    pub moblie: String,

    /// Image captcha cid -- required (prevents SMS bombing / phone number enumeration)
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    /// Image captcha -- required
    #[validate(length(min = 1, max = 16))]
    pub authcode: String,
}

/// Send forgot-password verification code (call `GET /v1/wap/captcha` first to obtain cid + image)
#[utoipa::path(
    post,
    path = "/v1/wap/forgetpw/send-sms",
    tag = "auth",
    request_body = SendSmsForm,
    responses(
        (status = 200, description = "Sent"),
        (status = 400, description = "Invalid captcha"),
        (status = 429, description = "Too many requests"),
    )
)]
pub async fn send_sms(
    State(state): State<AppState>,
    ValidatedJson(f): ValidatedJson<SendSmsForm>,
) -> AppResult<ApiOk> {
    // Mandatory image captcha
    let code = f.authcode.to_uppercase();
    if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, &f.captcha_cid, &code).await? {
        return Err(AppError::captcha());
    }

    password_reset_service::send_sms_code(&state, &f.moblie).await?;
    Ok(ApiOk("sent"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetForm {
    #[validate(custom(function = "validators::cn_mobile"))]
    pub moblie: String,

    /// SMS verification code, PHP field name `moblie_code`
    #[validate(custom(function = "validators::captcha"))]
    pub moblie_code: String,

    /// New password, PHP field name `password`
    #[validate(custom(function = "validators::strong_password"))]
    pub password: String,
}

/// Reset password
#[utoipa::path(
    post,
    path = "/v1/wap/forgetpw/reset",
    tag = "auth",
    request_body = ResetForm,
    responses(
        (status = 200, description = "Reset successful"),
        (status = 400, description = "Invalid code / mobile not registered"),
    )
)]
pub async fn reset(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ResetForm>,
) -> AppResult<ApiJson<json::Value>> {
    password_reset_service::reset_with_sms(&state, &f.moblie, &f.moblie_code, &f.password, &ip)
        .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// ==================== Email channel ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendEmailForm {
    #[validate(email)]
    pub email: String,

    /// Image captcha cid — required (anti-bombing / anti-enumeration), same
    /// scheme as the SMS path.
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub authcode: String,
}

/// Send a 6-digit password-reset code via email.
/// Counterpart of PHP `forgetpw/index::sendCode_action` with `sendtype=email`.
#[utoipa::path(
    post,
    path = "/v1/wap/forgetpw/send-email",
    tag = "auth",
    request_body = SendEmailForm,
    responses(
        (status = 200, description = "Sent"),
        (status = 400, description = "Invalid captcha / bad email"),
        (status = 429, description = "Too many requests"),
    )
)]
pub async fn send_email(
    State(state): State<AppState>,
    ValidatedJson(f): ValidatedJson<SendEmailForm>,
) -> AppResult<ApiOk> {
    let code = f.authcode.to_uppercase();
    if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, &f.captcha_cid, &code).await? {
        return Err(AppError::captcha());
    }
    password_reset_service::send_email_code(&state, &f.email).await?;
    Ok(ApiOk("sent"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetByEmailForm {
    #[validate(email)]
    pub email: String,

    /// 6-digit code emailed by `send-email`.
    #[validate(custom(function = "validators::captcha"))]
    pub email_code: String,

    #[validate(custom(function = "validators::strong_password"))]
    pub password: String,
}

/// Reset the password using an emailed code.
#[utoipa::path(
    post,
    path = "/v1/wap/forgetpw/reset-by-email",
    tag = "auth",
    request_body = ResetByEmailForm,
    responses(
        (status = 200, description = "Reset successful"),
        (status = 400, description = "Invalid code / email not registered"),
    )
)]
pub async fn reset_by_email(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ResetByEmailForm>,
) -> AppResult<ApiJson<json::Value>> {
    password_reset_service::reset_with_email(&state, &f.email, &f.email_code, &f.password, &ip)
        .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// ==================== Manual appeal (last-resort recovery) ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AppealForm {
    /// Account identifier — username / registered email / registered mobile.
    #[validate(length(min = 3, max = 100))]
    pub account: String,
    #[validate(length(min = 1, max = 30))]
    pub linkman: String,
    #[validate(length(min = 5, max = 30))]
    pub linkphone: String,
    #[validate(length(max = 60))]
    #[serde(default)]
    pub linkemail: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct AppealResponse {
    /// Submitted ticket id (the matched user's uid). Admin reviews via the admin
    /// console; client should display "appeal submitted" and stop polling.
    pub ticket_uid: u64,
}

/// Submit an account appeal — counterpart of PHP `forgetpw/index::checklink_action`.
/// Used as a last resort when both SMS and email channels are unavailable.
#[utoipa::path(
    post,
    path = "/v1/wap/forgetpw/appeal",
    tag = "auth",
    request_body = AppealForm,
    responses(
        (status = 200, description = "Submitted", body = AppealResponse),
        (status = 400, description = "Validation failed / account not found"),
    )
)]
pub async fn submit_appeal(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AppealForm>,
) -> AppResult<ApiJson<AppealResponse>> {
    let uid = password_reset_service::submit_appeal(
        &state,
        password_reset_service::AppealInput {
            account: &f.account,
            linkman: &f.linkman,
            linkphone: &f.linkphone,
            linkemail: &f.linkemail,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(AppealResponse { ticket_uid: uid }))
}
