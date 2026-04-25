//! Remarks/notes (aligned with PHPYun `remark.model.php`).

use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::remark::{entity::Remark, repo as remark_repo};

pub async fn get(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    kind: i32,
) -> AppResult<Option<Remark>> {
    Ok(remark_repo::get(state.db.reader(), user.uid, target_uid, kind).await?)
}

pub async fn upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    kind: i32,
    note: &str,
) -> AppResult<()> {
    remark_repo::upsert(
        state.db.pool(),
        user.uid,
        target_uid,
        kind,
        note,
        clock::now_ts(),
    )
    .await?;
    Ok(())
}

pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    kind: i32,
) -> AppResult<()> {
    remark_repo::delete(state.db.pool(), user.uid, target_uid, kind).await?;
    Ok(())
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Remark>> {
    let db = state.db.reader();
    // Simplified: derive total from the returned list length
    let list = remark_repo::list_by_user(db, user.uid, kind, page.offset, page.limit).await?;
    let total = list.len() as u64 + page.offset; // best-effort; an exact count would need a separate SQL
    Ok(Paged::new(list, total, page.page, page.page_size))
}
