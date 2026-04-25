//! System broadcasts (aligned with PHPYun `sysmsg.model.php`).
//!
//! Admin broadcasts -> users see them as unread according to usertype. Read receipts are written to `phpyun_rs_broadcast_reads`.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::broadcast::{entity::Broadcast, repo as bc_repo};

pub async fn admin_create(
    state: &AppState,
    admin: &AuthenticatedUser,
    title: &str,
    body: &str,
    target_usertype: i32,
) -> AppResult<u64> {
    admin.require_admin()?;
    if !(0..=3).contains(&target_usertype) {
        return Err(AppError::new(InfraError::InvalidParam("bad_target_usertype".into())));
    }
    let id = bc_repo::create(
        state.db.pool(),
        title,
        body,
        target_usertype,
        admin.uid,
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.broadcast.create", audit::Actor::uid(admin.uid))
            .target(format!("broadcast:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn admin_delete(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    admin.require_admin()?;
    bc_repo::delete(state.db.pool(), id).await?;
    Ok(())
}

pub async fn admin_list(
    state: &AppState,
    admin: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Broadcast>> {
    admin.require_admin()?;
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        bc_repo::admin_list(db, page.offset, page.limit),
        bc_repo::admin_count(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

// ---------- User side ----------

pub async fn list_for_me(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Broadcast>> {
    let usertype = user.usertype as i32;
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        bc_repo::list_for_user(db, usertype, page.offset, page.limit),
        bc_repo::count_for_user(db, usertype),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn unread_count(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    Ok(bc_repo::count_unread(state.db.reader(), user.uid, user.usertype as i32).await?)
}

pub async fn mark_read(
    state: &AppState,
    user: &AuthenticatedUser,
    broadcast_id: u64,
) -> AppResult<()> {
    bc_repo::mark_read(state.db.pool(), user.uid, broadcast_id, clock::now_ts()).await?;
    Ok(())
}
