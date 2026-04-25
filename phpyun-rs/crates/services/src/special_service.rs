//! Special recruiting events (aligned with PHPYun `wap/special`).

use phpyun_core::error::InfraError;
use phpyun_core::{background, AppError, AppResult, AppState, Paged, Pagination};
use phpyun_models::special::{
    entity::{Special, SpecialCompany},
    repo as special_repo,
};

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<Special>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        special_repo::list(db, page.offset, page.limit),
        special_repo::count(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Special> {
    let s = special_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("special_not_found".into())))?;
    if s.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("special_unavailable".into())));
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("special.view", async move {
        let _ = special_repo::incr_view(&pool, id).await;
    });
    Ok(s)
}

pub async fn list_companies(
    state: &AppState,
    sid: u64,
    page: Pagination,
) -> AppResult<Paged<SpecialCompany>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        special_repo::list_company_uids(db, sid, page.offset, page.limit),
        special_repo::count_companies(db, sid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

/// Jobs posted by companies within the special event (aggregated).
pub async fn list_jobs(
    state: &AppState,
    sid: u64,
    limit: u64,
) -> AppResult<Vec<phpyun_models::job::entity::Job>> {
    let db = state.db.reader();
    let uids = special_repo::list_company_uid_ids(db, sid, 100).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(special_repo::list_jobs_for_uids(db, &uids, now, limit).await?)
}
