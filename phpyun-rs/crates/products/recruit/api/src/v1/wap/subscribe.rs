//! Public job-alert subscribe (PHP `index.php?m=subscribe`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::{
    ApiResponse, AppResult, AppState, ClientIp, MaybeUser, ValidatedJson,
};
use phpyun_services::job_alert_service::{self, SubscribeInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/subscribe", post(create))
        .route("/subscribe/send-email", post(send_email))
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubscribeMetaView {
    pub jionly: i32,
    pub cionly: i32,
    pub cycles: Vec<i32>,
}

pub(crate) async fn build_meta(state: &AppState) -> AppResult<SubscribeMetaView> {
    let m = job_alert_service::meta(state).await?;
    Ok(SubscribeMetaView {
        jionly: m.jionly,
        cionly: m.cionly,
        cycles: m.cycles,
    })
}


#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubscribeForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub job1: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub job1_son: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub job_post: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub provinceid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub cityid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub three_cityid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub minsalary: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999_999))]
    pub maxsalary: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub time: i32,
    #[serde(default)]
    #[validate(length(max = 100))]
    pub email: String,
    /// 1 = job alert (default), 2 = resume alert.
    #[serde(default = "default_type")]
    #[validate(range(min = 1, max = 2))]
    pub r#type: i32,
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub captcha_input: String,
}
fn default_type() -> i32 {
    1
}

async fn verify_captcha(state: &AppState, cid: &str, input: &str) -> AppResult<()> {
    phpyun_core::verify::verify(
        &state.redis,
        phpyun_core::verify::VerifyKind::ImageCaptcha,
        cid,
        &input.to_uppercase(),
    )
    .await?
    .then_some(())
    .ok_or_else(phpyun_core::ApiError::captcha)
}

#[utoipa::path(
    post,
    path = "/v1/wap/subscribe",
    tag = "wap",
    request_body = SubscribeForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SubscribeForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    verify_captcha(&state, &f.captcha_cid, &f.captcha_input).await?;
    let id = job_alert_service::create(
        &state,
        user.as_ref(),
        SubscribeInput {
            job1: f.job1,
            job1_son: f.job1_son,
            job_post: f.job_post,
            provinceid: f.provinceid,
            cityid: f.cityid,
            three_cityid: f.three_cityid,
            minsalary: f.minsalary,
            maxsalary: f.maxsalary,
            time: f.time,
            email: &f.email,
            r#type: f.r#type,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendEmailForm {
    #[validate(length(min = 1, max = 100))]
    pub email: String,
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub captcha_input: String,
}

#[utoipa::path(
    post,
    path = "/v1/wap/subscribe/send-email",
    tag = "wap",
    request_body = SendEmailForm,
    responses((status = 200, description = "ok"))
)]
pub async fn send_email(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SendEmailForm>,
) -> AppResult<ApiResponse> {
    verify_captcha(&state, &f.captcha_cid, &f.captcha_input).await?;
    job_alert_service::send_notice(&state, &f.email, &ip).await?;
    Ok(ApiResponse::message("ok"))
}
