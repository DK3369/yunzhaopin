//! Friendly links (link exchange).
//!
//! 5-minute TTL cache keyed by `category` (None = all). Friend links rarely change,
//! and the list is rendered on every page footer.

use phpyun_core::{AppError, AppResult, AppState};
use phpyun_models::friend_link::{entity::FriendLink, repo as friend_link_repo};
use std::sync::Arc;

const TTL_SECS: u64 = 300;

static CACHE: std::sync::OnceLock<
    moka::future::Cache<Option<String>, Arc<Vec<FriendLink>>>,
> = std::sync::OnceLock::new();

fn cache() -> &'static moka::future::Cache<Option<String>, Arc<Vec<FriendLink>>> {
    CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(64)
            .time_to_live(std::time::Duration::from_secs(TTL_SECS))
            .build()
    })
}

pub async fn invalidate_all() {
    cache().invalidate_all();
}

pub async fn list(
    state: &AppState,
    category: Option<&str>,
) -> AppResult<Arc<Vec<FriendLink>>> {
    let key = category.map(str::to_owned);
    let db = state.db.reader().clone();
    let key_clone = key.clone();
    cache()
        .try_get_with(key, async move {
            let list =
                friend_link_repo::list_active(&db, key_clone.as_deref()).await?;
            Ok::<_, AppError>(Arc::new(list))
        })
        .await
        .map_err(AppError::from_arc)
}
