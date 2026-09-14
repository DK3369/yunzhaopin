//! Friendly links (link exchange).
//!
//! 5-minute TTL cache keyed by `category` (None = all). Friend links rarely change,
//! and the list is rendered on every page footer.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{ApiError, AppResult, AppState};
use phpyun_models::friend_link::{entity::FriendLink, repo as friend_link_repo};
use phpyun_models::site_setting::repo as setting_repo;
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

pub struct ApplyInput<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub link_type: &'a str,
    pub pic: &'a str,
}

pub async fn apply(
    state: &AppState,
    input: ApplyInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let _ = client_ip;
    let allowed = setting_repo::find(state.db.reader(), "sy_linksq")
        .await?
        .map(|s| s.value.trim().to_string())
        .unwrap_or_else(|| "0".into());
    if allowed != "1" {
        return Err(ApiError::business("linksq_closed"));
    }
    let name = input.name.trim();
    let url = input.url.trim();
    if name.is_empty() || url.is_empty() {
        return Err(ApiError::param_invalid("link_required"));
    }
    let link_type = match input.link_type.trim() {
        "2" => "2",
        _ => "1",
    };
    let pic = if link_type == "2" {
        input.pic.trim()
    } else {
        ""
    };
    if pic.len() > 255 {
        return Err(ApiError::param_invalid("link_pic"));
    }
    Ok(friend_link_repo::upsert(
        state.db.pool(),
        friend_link_repo::FriendLinkUpsert {
            id: None,
            link_name: name,
            link_url: url,
            pic,
            link_type,
            link_sorting: 0,
            link_state: 0,
        },
    )
    .await?)
}
