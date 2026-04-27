//! Education history CRUD (usertype=1).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::edu::EduInput;
use phpyun_services::resume_children_service::edu_svc;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/edus", post(create))
        .route("/resume/edus/list", post(list))
        .route("/resume/edus/update", post(update))
}

/// Education history item — **reuses** `wap::resumes::ResumeEduItem` (11 fields, includes education dict name + formatted timestamps).
pub type EduItem = crate::v1::wap::resumes::ResumeEduItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EduForm {
    /// Optional — only required for the update endpoint.
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
    pub specialty: Option<String>,
    #[validate(range(min = 0, max = 999))]
    pub title: i32,
    /// Soft delete: pass `2` to delete the entry (equivalent to the original DELETE).
    /// Other values or None go through the update path.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

/// Education history list
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/edus/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<EduItem>>> {
    let list = edu_svc::list(&state, &user).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|e| crate::v1::wap::resumes::resume_edu_item_from_dict(e, &dicts))
            .collect(),
    ))
}

/// Create an education history entry
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/edus",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = EduForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<EduForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = edu_svc::create(
        &state,
        &user,
        EduInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            specialty: f.specialty.as_deref(),
            title: f.title,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update an education history entry (or soft delete — body with `"status":2` means delete).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/edus/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = EduForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<EduForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        edu_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    edu_svc::update(
        &state,
        &user,
        f.id,
        EduInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            specialty: f.specialty.as_deref(),
            title: f.title,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
