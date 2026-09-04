//! User advice/feedback (matching PHPYun `wap/advice`) — public endpoint, anonymous submission allowed.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::{ApiResponse, AppResult, AppState, ClientIp, MaybeUser, ValidatedJson};
use phpyun_services::feedback_service::{self, FeedbackInput};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/advice", post(submit))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdviceForm {
    #[validate(length(min = 1, max = 32))]
    pub infotype: String,
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    /// Contact phone (PHPYun field name moblie)
    #[validate(length(max = 32))]
    #[serde(default)]
    pub moblie: String,
    #[validate(length(max = 20))]
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub moblie_code: String,
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub captcha_input: String,
}

/// Submit advice/feedback
#[utoipa::path(
    post,
    path = "/v1/wap/advice",
    tag = "wap",
    request_body = AdviceForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn submit(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AdviceForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    phpyun_core::verify::verify(
        &state.redis,
        phpyun_core::verify::VerifyKind::ImageCaptcha,
        &f.captcha_cid,
        &f.captcha_input.to_uppercase(),
    )
    .await?
    .then_some(())
    .ok_or_else(phpyun_core::ApiError::captcha)?;
    let need_sms = phpyun_services::site_gate_service::setting_i32(&state, "sy_advice_mobilecode").await == 1;
    if need_sms {
        if f.moblie.is_empty() || f.moblie_code.is_empty() {
            return Err(phpyun_core::ApiError::param_invalid("moblie_code"));
        }
        phpyun_core::verify::verify(
            &state.redis,
            phpyun_core::verify::VerifyKind::SmsAdvice,
            &f.moblie,
            &f.moblie_code,
        )
        .await?
        .then_some(())
        .ok_or_else(|| phpyun_core::ApiError::param_invalid("moblie_code"))?;
    }
    let id = feedback_service::submit(
        &state,
        user.as_ref(),
        FeedbackInput {
            username: &f.username,
            category: &f.infotype,
            content: &f.content,
            contact: &f.moblie,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}
