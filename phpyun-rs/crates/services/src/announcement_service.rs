//! Announcement service (public, no auth).

use phpyun_core::{background, AppResult, AppState, Paged, Pagination};
use phpyun_models::announcement::{entity::Announcement, repo as ann_repo};

pub async fn list(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<Announcement>> {
    let db = state.db.reader();
    let list = ann_repo::list_published(db, page.offset, page.limit).await?;
    let total = ann_repo::count_published(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_detail(
    state: &AppState,
    id: u64,
) -> AppResult<Option<Announcement>> {
    let db = state.db.reader();
    let row = ann_repo::find_by_id(db, id).await?;
    if row.is_some() {
        // fire-and-forget: PHPYun bumps the counter before fetching details; doing it asynchronously is fine here (does not affect the returned data)
        let pool = state.db.pool().clone();
        background::spawn_best_effort("announcement.view", async move {
            let _ = ann_repo::incr_view(&pool, id).await;
        });
    }
    Ok(row)
}
