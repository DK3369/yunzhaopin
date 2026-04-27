//! Project experience CRUD (usertype=1). Single-resource delete is folded into update (`status:2` is a soft delete).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::project::ProjectInput;
use phpyun_services::resume_children_service::project_svc;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/projects", post(create))
        .route("/resume/projects/list", post(list))
        .route("/resume/projects/update", post(update))
}

/// Project experience item — **reuses** `wap::resumes::ResumeProjectItem` (10 fields).
pub type ProjectItem = crate::v1::wap::resumes::ResumeProjectItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProjectForm {
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
    pub role: Option<String>,
    #[validate(length(max = 5000))]
    pub content: Option<String>,
    /// Soft delete: pass `2` to delete.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/projects/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
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
    path = "/v1/mcenter/resume/projects/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ProjectForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ProjectForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        project_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    project_svc::update(
        &state,
        &user,
        f.id,
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
