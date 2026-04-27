//! Skill CRUD (usertype=1). Single-resource delete is folded into update (`status:2` is a soft delete).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::skill::SkillInput;
use phpyun_services::resume_children_service::skill_svc;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/skills", post(create))
        .route("/resume/skills/list", post(list))
        .route("/resume/skills/update", post(update))
}

/// Skill item — **reuses** `wap::resumes::ResumeSkillItem` (7 fields, including the proficiency dictionary name).
pub type SkillItem = crate::v1::wap::resumes::ResumeSkillItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SkillForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(range(min = 0, max = 999))]
    pub level: i32,
    #[validate(range(min = 0, max = 999))]
    pub years: i32,
    /// Soft delete: pass `2` to delete.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/skills/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<SkillItem>>> {
    let list = skill_svc::list(&state, &user).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|s| crate::v1::wap::resumes::resume_skill_item_from_dict(s, &dicts))
            .collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/skills",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SkillForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SkillForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = skill_svc::create(
        &state,
        &user,
        SkillInput {
            name: &f.name,
            level: f.level,
            years: f.years,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/skills/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SkillForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SkillForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        skill_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    skill_svc::update(
        &state,
        &user,
        f.id,
        SkillInput {
            name: &f.name,
            level: f.level,
            years: f.years,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
