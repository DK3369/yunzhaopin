//! PHP `addYqms` from a public resume — writes `userid_msg`, no `apply_id`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::yqms_service::{self, YqmsInput};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/company/yqms/create", post(create))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct YqmsForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub seeker_uid: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
    #[serde(default)]
    #[validate(length(max = 5000))]
    pub content: String,
    #[validate(length(min = 1, max = 300))]
    pub address: String,
    #[validate(length(min = 1, max = 64))]
    pub intertime: String,
    #[validate(length(min = 1, max = 64))]
    pub linkman: String,
    #[validate(length(min = 6, max = 32))]
    pub linktel: String,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub longitude: String,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub latitude: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/yqms/create",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = YqmsForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<YqmsForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = yqms_service::create_from_resume(
        &state,
        &user,
        YqmsInput {
            seeker_uid: f.seeker_uid,
            job_id: f.job_id,
            content: &f.content,
            address: &f.address,
            intertime: &f.intertime,
            linkman: &f.linkman,
            linktel: &f.linktel,
            longitude: &f.longitude,
            latitude: &f.latitude,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}
