//! Friendly links (link exchange).
//!
//! 5-minute TTL cache keyed by `category` (None = all). Friend links rarely change,
//! and the list is rendered on every page footer.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{AppResult, AppState};
use phpyun_models::friend_link::{entity::FriendLink, repo as friend_link_repo};
use std::sync::Arc;

const TTL_SECS: u64 = 300;

static CACHE: std::sync::OnceLock<SimpleCache<Option<String>, Vec<FriendLink>>> =
    std::sync::OnceLock::new();

fn cache() -> &'static SimpleCache<Option<String>, Vec<FriendLink>> {
    CACHE.get_or_init(|| SimpleCache::new(64, std::time::Duration::from_secs(TTL_SECS)))
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
        .get_or_load(key, move || async move {
            Ok(friend_link_repo::list_active(&db, key_clone.as_deref()).await?)
        })
        .await
}
