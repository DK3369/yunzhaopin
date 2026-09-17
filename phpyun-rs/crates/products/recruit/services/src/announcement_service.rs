//! Announcement service (public, no auth).

use phpyun_core::cache::TieredCache;
use phpyun_core::{background, AppResult, AppState, Paged, Pagination};
use phpyun_models::announcement::{entity::Announcement, repo as ann_repo};
use std::sync::OnceLock;
use std::time::Duration;

const LIST_TTL: Duration = Duration::from_secs(60);

static CACHE: OnceLock<TieredCache<Paged<Announcement>>> = OnceLock::new();

fn cache() -> &'static TieredCache<Paged<Announcement>> {
    CACHE.get_or_init(|| TieredCache::new(64, LIST_TTL))
}

fn cache_key(did: u32, page: &Pagination) -> String {
    format!("announcement:{did}:{}:{}", page.page, page.page_size)
}

pub async fn invalidate_all(state: &AppState) {
    cache().invalidate_prefix_local();
    let _ = state;
}

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<Announcement>> {
    let did = 0u32;
    let key = cache_key(did, &page);
    let st = state.clone();
    let arc = cache()
        .get_or_load(
            &state.redis,
            key,
            LIST_TTL,
            "announcement",
            move || async move {
                let db = st.db.reader();
                let list = ann_repo::list_published(db, did, page.offset, page.limit).await?;
                let total = ann_repo::count_published(db, did).await?;
                Ok(Paged::new(list, total, page.page, page.page_size))
            },
        )
        .await?;
    Ok((*arc).clone())
}

pub async fn get_detail(state: &AppState, id: u64) -> AppResult<Option<Announcement>> {
    let db = state.db.reader();
    let row = ann_repo::find_by_id(db, id).await?;
    if row.is_some() {
        // fire-and-forget: PHPYun bumps the counter before fetching details; doing it asynchronously is fine here (does not affect the returned data)
        let pool = state.db.pool().clone();
        background::spawn_best_effort("announcement.view", async move {
            let _ = ann_repo::incr_view(&pool, id).await;
        });
    }
    Ok(row.map(|mut a| {
        a.content = phpyun_core::html::sanitize_html(&a.content);
        a
    }))
}

pub async fn neighbors(
    state: &AppState,
    a: &Announcement,
) -> AppResult<(
    Option<phpyun_models::announcement::repo::Neighbor>,
    Option<phpyun_models::announcement::repo::Neighbor>,
)> {
    Ok(ann_repo::neighbors(state.db.reader(), a.id, a.datetime).await?)
}
