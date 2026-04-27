//! Audit log queries (admin).

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::audit_log_service::{self, Filter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/audit-log", post(list))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct AuditQuery {
    /// e.g. `admin.` / `user.`
    #[validate(length(max = 100))]
    pub action_prefix: Option<String>,
    #[validate(range(min = 1, max = 99_999_999))]
    pub actor_uid: Option<u64>,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub since: Option<i64>,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub until: Option<i64>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuditItem {
    pub id: u64,
    pub actor_uid: Option<u64>,
    pub actor_ip: String,
    pub actor_ua: String,
    pub action: String,
    pub target: String,
    pub success: i32,
    pub meta: Option<String>,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::audit_log::entity::AuditLog> for AuditItem {
    fn from(a: phpyun_models::audit_log::entity::AuditLog) -> Self {
        Self {
            id: a.id,
            actor_uid: a.actor_uid,
            actor_ip: a.actor_ip,
            actor_ua: a.actor_ua,
            action: a.action,
            target: a.target,
            success: a.success,
            meta: a.meta,
            created_at_n: fmt_dt(a.created_at),
            created_at: a.created_at,
        }
    }
}

/// List audit log entries
#[utoipa::path(
    post,
    path = "/v1/admin/audit-log",
    tag = "admin",
    security(("bearer" = [])),
    params(AuditQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<AuditQuery>,
) -> AppResult<ApiJson<Paged<AuditItem>>> {
    user.require_admin()?;
    let f = Filter {
        action_prefix: q.action_prefix.as_deref(),
        actor_uid: q.actor_uid,
        since: q.since,
        until: q.until,
    };
    let r = audit_log_service::admin_list(&state, &user, &f, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AuditItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
