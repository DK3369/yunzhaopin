//! Jobseeker: who viewed my resume (PHP `look` / `look_resume`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::resume_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/look-resumes/list", post(list_mine))
        .route("/look-resumes/mine", post(list_company_mine))
        .route("/look-resumes/delete", post(delete_mine))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LookResumeItem {
    pub id: u64,
    pub uid: u64,
    pub com_id: u64,
    pub resume_id: u64,
    pub com_name: String,
    pub com_job: String,
    pub com_job_num: i64,
    pub resume_name: String,
    pub datetime: i64,
    pub datetime_n: String,
}

impl From<phpyun_models::look_resume::LookResume> for LookResumeItem {
    fn from(r: phpyun_models::look_resume::LookResume) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            com_id: r.com_id,
            resume_id: r.resume_id,
            com_name: r.com_name,
            com_job: r.com_job,
            com_job_num: r.com_job_num,
            resume_name: r.resume_name,
            datetime_n: fmt_dt(r.datetime),
            datetime: r.datetime,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/look-resumes/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<LookResumeItem>>> {
    let r = resume_service::list_look_resumes(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list
            .into_iter()
            .map(LookResumeItem::from)
            .collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/look-resumes/mine",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_company_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<LookResumeItem>>> {
    let r = resume_service::list_look_resumes_mine(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list
            .into_iter()
            .map(LookResumeItem::from)
            .collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/look-resumes/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = resume_service::hide_look_resume(&state, &user, b.id).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}
