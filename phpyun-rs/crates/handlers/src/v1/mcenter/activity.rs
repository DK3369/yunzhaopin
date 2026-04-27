//! My activity log (filtered from `yun_rs_audit_log` where actor_uid = self).

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedQuery};
use phpyun_services::audit_log_service::{self, Filter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/activity", get(list))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ActivityQuery {
    pub action_prefix: Option<String>,
    pub since: Option<i64>,
    pub until: Option<i64>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Personal activity log item — all 9 columns of phpyun_audit_log + formatted timestamp (full uid/ip/ua audit info).
#[derive(Debug, Serialize, ToSchema)]
pub struct ActivityItem {
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

impl From<phpyun_models::audit_log::entity::AuditLog> for ActivityItem {
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

/// My activity log
#[utoipa::path(
    get,
    path = "/v1/mcenter/activity",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ActivityQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ActivityQuery>,
) -> AppResult<ApiJson<Paged<ActivityItem>>> {
    let f = Filter {
        action_prefix: q.action_prefix.as_deref(),
        actor_uid: None, // ignored; service forces self
        since: q.since,
        until: q.until,
    };
    let r = audit_log_service::list_mine(&state, &user, &f, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ActivityItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
