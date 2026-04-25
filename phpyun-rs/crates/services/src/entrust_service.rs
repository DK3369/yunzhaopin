//! Jobseeker ↔ headhunter bindings (`phpyun_entrust`).
//!
//! PHPYun's `phpyun_entrust` is a 5-column join table tying a jobseeker
//! (`uid`, usertype=1) to one or more headhunters (`lt_uid`, usertype=4).
//! There is **no** "desired job / city / salary" payload — that was a
//! previous misread; nothing in PHP supports it.
//!
//! Endpoints expose:
//!   - list my bindings
//!   - bind a headhunter (idempotent — duplicate is silently OK)
//!   - unbind by lt_uid or by row id
//!   - count my bindings (used by dashboard)

use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::entrust::{entity::Entrust, repo as entrust_repo};

pub async fn count_mine(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    user.require_jobseeker()?;
    Ok(entrust_repo::count_by_uid(state.db.reader(), user.uid).await?)
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Entrust>> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        entrust_repo::count_by_uid(state.db.reader(), user.uid),
        entrust_repo::list_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

/// Bind to a headhunter. Idempotent — re-binding the same `lt_uid` returns
/// the existing row's id rather than erroring.
pub async fn bind(
    state: &AppState,
    user: &AuthenticatedUser,
    lt_uid: u64,
) -> AppResult<u64> {
    user.require_jobseeker()?;
    if lt_uid == 0 {
        return Err(AppError::param_invalid("lt_uid"));
    }
    if lt_uid == user.uid {
        // Can't entrust yourself — application-layer rule.
        return Err(AppError::param_invalid("entrust_self"));
    }
    if let Some(existing) =
        entrust_repo::find_binding(state.db.reader(), user.uid, lt_uid).await?
    {
        return Ok(existing.id);
    }
    let id = entrust_repo::insert(state.db.pool(), user.uid, lt_uid, clock::now_ts()).await?;
    Ok(id)
}

/// Unbind by lt_uid (idempotent).
pub async fn unbind(
    state: &AppState,
    user: &AuthenticatedUser,
    lt_uid: u64,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let _ = entrust_repo::delete(state.db.pool(), user.uid, lt_uid).await?;
    Ok(())
}

/// Unbind by row id (uid-scoped — a user cannot delete others' rows).
pub async fn unbind_by_id(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let _ = entrust_repo::delete_by_id(state.db.pool(), id, user.uid).await?;
    Ok(())
}
