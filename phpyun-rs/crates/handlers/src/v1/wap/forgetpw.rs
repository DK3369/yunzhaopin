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
        .route("/forgetpw/reset", post(reset))
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
