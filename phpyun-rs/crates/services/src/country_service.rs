//! Curated major-country lookup.
//!
//! Cache: a single `Arc<Vec<Country>>` keyed by `()` with a 30-minute TTL.
//! The list rarely changes (admins rarely edit it) and is rendered on
//! every page that has a country selector, so the cache pays for itself.
//! Admin mutations call `invalidate()` to force the next reader to refresh.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{AppResult, AppState};
use phpyun_models::country::{entity::Country, repo as country_repo};
use std::sync::Arc;
use std::time::Duration;

const TTL_SECS: u64 = 1800;

static CACHE: std::sync::OnceLock<SimpleCache<(), Vec<Country>>> = std::sync::OnceLock::new();

fn cache() -> &'static SimpleCache<(), Vec<Country>> {
    CACHE.get_or_init(|| SimpleCache::new(1, Duration::from_secs(TTL_SECS)))
}

pub async fn invalidate() {
    cache().invalidate(&()).await;
}

/// All active major countries, sorted by `(sort ASC, id ASC)`.
pub async fn list_all(state: &AppState) -> AppResult<Arc<Vec<Country>>> {
    let db = state.db.reader().clone();
    cache()
        .get_or_load((), || async move {
            country_repo::list_active(&db).await.map_err(Into::into)
        })
        .await
}

/// Filter helper for `?continent=AS`.
pub async fn list_by_continent(
    state: &AppState,
    continent: &str,
) -> AppResult<Vec<Country>> {
    let all = list_all(state).await?;
    Ok(all
        .iter()
        .filter(|c| c.continent.eq_ignore_ascii_case(continent))
        .cloned()
        .collect())
}

pub async fn find_by_code(state: &AppState, code: &str) -> AppResult<Option<Country>> {
    let all = list_all(state).await?;
    let upper = code.to_uppercase();
    Ok(all.iter().find(|c| c.code == upper).cloned())
}

pub async fn find_by_id(state: &AppState, id: u64) -> AppResult<Option<Country>> {
    let all = list_all(state).await?;
    Ok(all.iter().find(|c| c.id == id).cloned())
}
