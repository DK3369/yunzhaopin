//! Warning management (admin issues warnings).

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::warning_service::{self, WarnInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new().route("/warnings", post(issue))
        .route("/warnings/list", post(list))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(range(min = 0, max = 99))]
    pub kind: Option<i32>,
}


fn warn_kind_name(k: i32) -> &'static str {
    match k { 1 => "user", 2 => "company", 3 => "job", 4 => "resume", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WarningItem {
    pub id: u64,
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_kind_n: String,
    pub target_id: u64,
    pub reason: String,
    pub is_read: i32,
    pub is_read_bool: bool,
    pub issuer_uid: u64,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::warning::entity::Warning> for WarningItem {
    fn from(w: phpyun_models::warning::entity::Warning) -> Self {
        Self {
            id: w.id,
            target_uid: w.target_uid,
            target_kind_n: warn_kind_name(w.target_kind).to_string(),
            target_kind: w.target_kind,
            target_id: w.target_id,
            reason: w.reason,
            is_read_bool: w.is_read == 1,
            is_read: w.is_read,
            issuer_uid: w.issuer_uid,
            created_at_n: fmt_dt(w.created_at),
            created_at: w.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WarnForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_uid: u64,
    /// 1=user 2=company 3=job 4=resume
    #[validate(range(min = 1, max = 4))]
    pub target_kind: i32,
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_id: u64,
    #[validate(length(min = 1, max = 500))]
    pub reason: String,
}

/// Admin: list warnings
#[utoipa::path(
    post,
    path = "/v1/admin/warnings/list",
    tag = "admin",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<WarningItem>>> {
    user.require_admin()?;
    let r = warning_service::admin_list(&state, q.kind, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Admin: issue a warning
#[utoipa::path(
    post,
    path = "/v1/admin/warnings",
    tag = "admin",
    security(("bearer" = [])),
    request_body = WarnForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn issue(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<WarnForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id = warning_service::admin_issue(
        &state,
        &user,
        WarnInput {
            target_uid: f.target_uid,
            target_kind: f.target_kind,
            target_id: f.target_id,
            reason: &f.reason,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}
