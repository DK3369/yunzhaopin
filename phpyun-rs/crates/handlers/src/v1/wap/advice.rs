//! User advice/feedback (matching PHPYun `wap/advice`) — public endpoint, anonymous submission allowed.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ClientIp, MaybeUser, ValidatedJson};
use phpyun_services::feedback_service::{self, FeedbackInput};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

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
) -> AppResult<ApiJson<CreatedId>> {
    let id = feedback_service::submit(
        &state,
        user.as_ref(),
        FeedbackInput {
            category: &f.infotype,
            content: &f.content,
            contact: &f.moblie,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}
