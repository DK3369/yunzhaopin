//! Interview invitation templates (aligned with PHPYun `yqmb`) — employer-side CRUD.

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::interview_template_service::{self, TplInput, TplPatch};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/interview-templates", post(create))
        .route("/interview-templates/list", post(list))
        .route("/interview-templates/update", post(update))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Interview template item — all 11 columns of `phpyun_interview_template` + formatted time.
#[derive(Debug, Serialize, ToSchema)]
pub struct TplItem {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub content: String,
    pub address: String,
    pub linkman: String,
    pub linktel: String,
    pub intertime: i64,
    pub intertime_n: String,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::interview_template::entity::InterviewTemplate> for TplItem {
    fn from(t: phpyun_models::interview_template::entity::InterviewTemplate) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            name: t.name,
            content: t.content,
            address: t.address,
            linkman: t.linkman,
            linktel: t.linktel,
            intertime_n: fmt_dt(t.intertime),
            intertime: t.intertime,
            status: t.status,
            created_at_n: fmt_dt(t.created_at),
            created_at: t.created_at,
            updated_at_n: fmt_dt(t.updated_at),
            updated_at: t.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TplForm {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(min = 1, max = 5000))]
    pub content: String,
    #[validate(length(min = 1, max = 300))]
    pub address: String,
    #[validate(length(min = 1, max = 64))]
    pub linkman: String,
    #[validate(length(min = 6, max = 32))]
    pub linktel: String,
    /// unix ts; 0 = unspecified
    #[serde(default)]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub intertime: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TplPatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 5000))]
    pub content: Option<String>,
    #[validate(length(min = 1, max = 300))]
    pub address: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub linkman: Option<String>,
    #[validate(length(min = 6, max = 32))]
    pub linktel: Option<String>,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub intertime: Option<i64>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

/// Interview template list
#[utoipa::path(
    post,
    path = "/v1/mcenter/interview-templates/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<TplItem>>> {
    user.require_employer()?;
    let list = interview_template_service::list(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(TplItem::from).collect()))
}

/// Create interview template
#[utoipa::path(
    post,
    path = "/v1/mcenter/interview-templates",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = TplForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TplForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_employer()?;
    let id = interview_template_service::create(
        &state,
        &user,
        TplInput {
            name: &f.name,
            content: &f.content,
            address: &f.address,
            linkman: &f.linkman,
            linktel: &f.linktel,
            intertime: f.intertime,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete an interview template (body with `"status":2` triggers deletion)
#[utoipa::path(
    post,
    path = "/v1/mcenter/interview-templates/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = TplPatchForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TplPatchForm>,
) -> AppResult<ApiOk> {
    user.require_employer()?;
    if f.status == Some(2) {
        interview_template_service::delete(&state, &user, f.id).await?;
        return Ok(ApiOk("deleted"));
    }
    interview_template_service::update(
        &state,
        &user,
        f.id,
        TplPatch {
            name: f.name.as_deref(),
            content: f.content.as_deref(),
            address: f.address.as_deref(),
            linkman: f.linkman.as_deref(),
            linktel: f.linktel.as_deref(),
            intertime: f.intertime,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}
