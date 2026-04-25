//! Language skill CRUD (usertype=1). Single-resource delete is folded into update (`status:2` is a soft delete).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_models::resume::language::LanguageInput;
use phpyun_services::resume_children_service::language_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/languages", get(list).post(create))
        .route("/resume/languages/{id}", post(update))
}

/// Language skill item — all 5 columns of phpyun_resume_language (PHPYun ships this as a stub: writes return 0, reads return empty).
#[derive(Debug, Serialize, ToSchema)]
pub struct LanguageItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub level: i32,
}

impl From<phpyun_models::resume::language::Language> for LanguageItem {
    fn from(l: phpyun_models::resume::language::Language) -> Self {
        Self {
            id: l.id,
            uid: l.uid,
            eid: l.eid,
            name: l.name,
            level: l.level,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LanguageForm {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub level: i32,
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
    path = "/v1/mcenter/resume/languages",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<LanguageItem>>> {
    let list = language_svc::list(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(LanguageItem::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/languages",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = LanguageForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<LanguageForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = language_svc::create(
        &state,
        &user,
        LanguageInput {
            name: &f.name,
            level: f.level,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/languages/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = LanguageForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<LanguageForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        language_svc::delete(&state, &user, id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    language_svc::update(
        &state,
        &user,
        id,
        LanguageInput {
            name: &f.name,
            level: f.level,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
