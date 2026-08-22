//! Audit log queries (admin only). Reads `yun_rs_audit_log` (written by `phpyun_core::audit::emit`).

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::audit_log::{entity::AuditLog, repo as audit_repo};

pub struct Filter<'a> {
    pub action_prefix: Option<&'a str>,
    pub actor_uid: Option<u64>,
    pub since: Option<i64>,
    pub until: Option<i64>,
}

pub async fn admin_list(
    state: &AppState,
    admin: &AuthenticatedUser,
    f: &Filter<'_>,
    page: Pagination,
) -> AppResult<Paged<AuditLog>> {
    admin.require_admin()?;
    let filter = audit_repo::AuditFilter {
        action_prefix: f.action_prefix,
        actor_uid: f.actor_uid,
        since: f.since,
        until: f.until,
    };
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        audit_repo::list(db, &filter, page.offset, page.limit),
        audit_repo::count(db, &filter),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

/// Lets the current user view their own activity log (filtered by actor_uid; no admin role required).
pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    f: &Filter<'_>,
    page: Pagination,
) -> AppResult<Paged<AuditLog>> {
    let filter = audit_repo::AuditFilter {
        action_prefix: f.action_prefix,
        actor_uid: Some(user.uid),
        since: f.since,
        until: f.until,
    };
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        audit_repo::list(db, &filter, page.offset, page.limit),
        audit_repo::count(db, &filter),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
