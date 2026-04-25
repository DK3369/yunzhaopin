//! Blacklist (aligned with PHPYun `black.model.php`).
//!
//! Business rule: a company can blacklist a jobseeker uid and vice versa. Once blacklisted, chat / invite / etc. should consult `is_blocked`.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::blacklist::{entity::BlacklistEntry, repo as bl_repo};

pub async fn add(
    state: &AppState,
    user: &AuthenticatedUser,
    blocked_uid: u64,
    reason: &str,
) -> AppResult<()> {
    if blocked_uid == user.uid {
        return Err(AppError::new(InfraError::InvalidParam("cannot_block_self".into())));
    }
    bl_repo::add(state.db.pool(), user.uid, blocked_uid, reason, clock::now_ts()).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("blacklist.add", audit::Actor::uid(user.uid))
            .target(format!("uid:{blocked_uid}")),
    )
    .await;
    Ok(())
}

pub async fn remove(
    state: &AppState,
    user: &AuthenticatedUser,
    blocked_uid: u64,
) -> AppResult<()> {
    bl_repo::remove(state.db.pool(), user.uid, blocked_uid).await?;
    Ok(())
}

/// Clears the current user's blacklist. Returns the number of entries deleted.
pub async fn clear_all(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    let removed = bl_repo::remove_all(state.db.pool(), user.uid).await?;
    if removed > 0 {
        let _ = audit::emit(
            state,
            audit::AuditEvent::new("blacklist.clear", audit::Actor::uid(user.uid))
                .meta(&serde_json::json!({ "removed": removed })),
        )
        .await;
    }
    Ok(removed)
}

pub async fn is_blocked(
    state: &AppState,
    uid: u64,
    blocked_uid: u64,
) -> AppResult<bool> {
    Ok(bl_repo::is_blocked(state.db.reader(), uid, blocked_uid).await?)
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<BlacklistEntry>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        bl_repo::list_by_uid(db, user.uid, page.offset, page.limit),
        bl_repo::count_by_uid(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
