//! Project experience CRUD (usertype=1). Single-resource delete is folded into update (`status:2` is a soft delete).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_models::resume::project::ProjectInput;
use phpyun_services::resume_children_service::project_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/projects", get(list).post(create))
        .route("/resume/projects/{id}", post(update))
}

/// Project experience item — **reuses** `wap::resumes::ResumeProjectItem` (10 fields).
pub type ProjectItem = crate::v1::wap::resumes::ResumeProjectItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProjectForm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub sdate: i64,
    pub edate: i64,
    #[validate(length(max = 50))]
    pub role: Option<String>,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    /// Soft delete: pass `2` to delete.
    #[serde(default)]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(
    get,
    path = "/v1/mcenter/resume/projects",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<ProjectItem>>> {
    let list = project_svc::list(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(ProjectItem::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/projects",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ProjectForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ProjectForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = project_svc::create(
        &state,
        &user,
        ProjectInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            role: f.role.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/projects/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = ProjectForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<ProjectForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        project_svc::delete(&state, &user, id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    project_svc::update(
        &state,
        &user,
        id,
        ProjectInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            role: f.role.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
