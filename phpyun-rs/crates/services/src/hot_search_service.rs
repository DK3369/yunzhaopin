//! Hot searches — bumped on each search, queried as the top-N on the home page.
//!
//! Callers invoke `bump_async` fire-and-forget so it never blocks the search itself.
//!
//! `top` is fronted by a 60-second TTL cache keyed by (scope, limit). Hot searches
//! are inherently a recency signal; one minute of staleness is fine and saves a
//! `phpyun_hot_search ORDER BY hits DESC` scan for every home-page hit.

use phpyun_core::{background, clock, AppError, AppResult, AppState};
use phpyun_models::hot_search::{entity::HotSearch, repo as hot_search_repo};
use std::sync::Arc;

const TTL_SECS: u64 = 60;

static CACHE: std::sync::OnceLock<
    moka::future::Cache<(String, u64), Arc<Vec<HotSearch>>>,
> = std::sync::OnceLock::new();

fn cache() -> &'static moka::future::Cache<(String, u64), Arc<Vec<HotSearch>>> {
    CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(128)
            .time_to_live(std::time::Duration::from_secs(TTL_SECS))
            .build()
    })
}

pub async fn invalidate_all() {
    cache().invalidate_all();
}

/// Records a search asynchronously (fire-and-forget; the main path does not wait).
pub fn bump_async(state: &AppState, scope: &'static str, keyword: String) {
    if keyword.is_empty() || keyword.len() > 64 {
        return;
    }
    let db = state.db.pool().clone();
    background::spawn_best_effort("hot_search.bump", async move {
        let _ = hot_search_repo::bump(&db, scope, &keyword, clock::now_ts()).await;
    });
}

pub async fn top(
    state: &AppState,
    scope: &str,
    limit: u64,
) -> AppResult<Arc<Vec<HotSearch>>> {
    let limit = limit.clamp(1, 50);
    let key = (scope.to_owned(), limit);
    let scope_owned = key.0.clone();
    let db = state.db.reader().clone();
    cache()
        .try_get_with(key, async move {
            let list = hot_search_repo::top(&db, &scope_owned, limit).await?;
            Ok::<_, AppError>(Arc::new(list))
        })
        .await
        .map_err(AppError::from_arc)
}
