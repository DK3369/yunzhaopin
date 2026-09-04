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

pub async fn list(state: &AppState, category: Option<&str>) -> AppResult<Arc<Vec<FriendLink>>> {
    let key = category.map(str::to_owned);
    let db = state.db.reader().clone();
    let key_clone = key.clone();
    cache()
        .get_or_load(key, move || async move {
            Ok(friend_link_repo::list_active(&db, key_clone.as_deref()).await?)
        })
        .await
}

pub async fn apply(
    state: &AppState,
    name: &str,
    url: &str,
    client_ip: &str,
) -> AppResult<u64> {
    let name = name.trim();
    let url = url.trim();
    if name.is_empty() || url.is_empty() {
        return Err(phpyun_core::ApiError::param_invalid("link_required"));
    }
    let _ = client_ip;
    Ok(friend_link_repo::upsert(
        state.db.pool(),
        friend_link_repo::FriendLinkUpsert {
            id: None,
            link_name: name,
            link_url: url,
            pic: "",
            link_type: "1",
            link_sorting: 0,
            link_state: 0,
        },
    )
    .await?)
}
