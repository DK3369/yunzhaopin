//! Work experience CRUD (usertype=1).
//!
//! Only GET/POST are exposed: single-resource `delete` is folded into `update` — sending `"status":2` performs a soft delete.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::work::WorkInput;
use phpyun_services::resume_children_service::work_svc;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/works", post(create))
        .route("/resume/works/list", post(list))
        .route("/resume/works/update", post(update))
}

/// Work experience item — **reuses** `wap::resumes::ResumeWorkItem` (11 fields).
pub type WorkItem = crate::v1::wap::resumes::ResumeWorkItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WorkForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub sdate: i64,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub edate: i64,
    #[validate(length(max = 50))]
    pub department: Option<String>,
    #[validate(length(max = 50))]
    pub title: Option<String>,
    #[validate(length(max = 5000))]
    pub content: Option<String>,
    /// Soft delete: pass `2` to delete. Other values or None perform an update.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/works/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<WorkItem>>> {
    let list = work_svc::list(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(WorkItem::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/works",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = WorkForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<WorkForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = work_svc::create(
        &state,
        &user,
        WorkInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            department: f.department.as_deref(),
            title: f.title.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete a work experience entry (sending `"status":2` deletes).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/works/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = WorkForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<WorkForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        work_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    work_svc::update(
        &state,
        &user,
        f.id,
        WorkInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            department: f.department.as_deref(),
            title: f.title.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
