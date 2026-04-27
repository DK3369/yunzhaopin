//! POST /v1/wap/sms/send — generic SMS code dispatch (register / login / reset_pw scenes).

use axum::{extract::State, routing::post, Router};
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{validators, ApiOk, AppError, AppResult, AppState, ValidatedJson};
use phpyun_services::sms_service::{self, SmsScene};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/sms/send", post(send))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SmsSendForm {
    /// PHPYun field name `moblie`
    #[validate(custom(function = "validators::cn_mobile"))]
    pub moblie: String,

    /// `register` / `login` / `reset_pw` (added in phpyun-rs; PHP splits into multiple separate actions, we merge them into one endpoint distinguished by scene)
    pub scene: String,

    /// Image captcha cid — required (anti SMS-bombing / mobile-enumeration)
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    /// Image captcha — required
    #[validate(length(min = 1, max = 16))]
    pub authcode: String,
}

/// Send SMS code (call `GET /v1/wap/captcha` first to obtain cid + image)
#[utoipa::path(
    post,
    path = "/v1/wap/sms/send",
    tag = "auth",
    request_body = SmsSendForm,
    responses(
        (status = 200, description = "Sent"),
        (status = 400, description = "Invalid captcha"),
        (status = 429, description = "Rate limited"),
    )
)]
pub async fn send(
    State(state): State<AppState>,
    ValidatedJson(f): ValidatedJson<SmsSendForm>,
) -> AppResult<ApiOk> {
    // 1. Mandatory image-captcha validation (anti SMS-bombing / mobile-enumeration)
    let code = f.authcode.to_uppercase();
    if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, &f.captcha_cid, &code).await? {
        return Err(AppError::captcha());
    }

    let scene = match f.scene.as_str() {
        "register" => SmsScene::Register,
        "login" => SmsScene::Login,
        "reset_pw" => SmsScene::ResetPw,
        // Anonymous-posting scenes — counterparts of PHP `wap/once::sendmsg`
        // and `wap/tiny::sendmsg`. The downstream `once::create` /
        // `tiny::create` flows can later consume the issued code via
        // `verify::verify(SmsOnceJob | SmsTinyResume)`.
        "once" | "once_job" => SmsScene::OnceJob,
        "tiny" | "tiny_resume" => SmsScene::TinyResume,
        _ => return Err(AppError::param_invalid("scene")),
    };
    sms_service::send_sms_code(&state, &f.moblie, scene).await?;
    Ok(ApiOk("sent"))
}
