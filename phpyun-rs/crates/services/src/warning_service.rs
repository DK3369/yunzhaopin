//! Warning system (aligned with PHPYun `warning.model.php`).
//!
//! Admins issue warnings to accounts -> users see the unread warnings in mcenter.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::warning::{entity::Warning, repo as warn_repo};

pub struct WarnInput<'a> {
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason: &'a str,
}

pub async fn admin_issue(
    state: &AppState,
    admin: &AuthenticatedUser,
    input: WarnInput<'_>,
) -> AppResult<u64> {
    admin.require_admin()?;
    if !(1..=4).contains(&input.target_kind) {
        return Err(AppError::new(InfraError::InvalidParam("bad_target_kind".into())));
    }
    let id = warn_repo::create(
        state.db.pool(),
        warn_repo::WarnCreate {
            target_uid: input.target_uid,
            target_kind: input.target_kind,
            target_id: input.target_id,
            reason: input.reason,
            issuer_uid: admin.uid,
        },
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.warning.issue", audit::Actor::uid(admin.uid))
            .target(format!("uid:{}", input.target_uid))
            .meta(&serde_json::json!({
                "kind": input.target_kind,
                "id": input.target_id,
                "warning_id": id,
            })),
    )
    .await;
    Ok(id)
}

pub async fn admin_list(
    state: &AppState,
    kind: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Warning>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        warn_repo::admin_list(db, kind, page.offset, page.limit),
        warn_repo::admin_count(db, kind),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

// ---------- Current user ----------

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Warning>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        warn_repo::list_mine(db, user.uid, page.offset, page.limit),
        warn_repo::count_mine(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn unread_count(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    Ok(warn_repo::count_unread(state.db.reader(), user.uid).await?)
}

pub async fn mark_read(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    warn_repo::mark_read(state.db.pool(), id, user.uid).await?;
    Ok(())
}
