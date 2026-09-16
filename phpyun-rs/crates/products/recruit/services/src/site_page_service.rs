//! Site static pages (about / privacy / protocol / contact / appDown).

use phpyun_core::cache::TieredCache;
use phpyun_core::{AppResult, AppState};
use phpyun_models::site_page::{entity::SitePage, repo as site_page_repo};
use std::sync::OnceLock;
use std::time::Duration;

const PAGE_TTL: Duration = Duration::from_secs(300);

static CACHE: OnceLock<TieredCache<Option<SitePage>>> = OnceLock::new();

fn cache() -> &'static TieredCache<Option<SitePage>> {
    CACHE.get_or_init(|| TieredCache::new(32, PAGE_TTL))
}

fn cache_key(code: &str) -> String {
    format!("site_page:{code}")
}

pub async fn invalidate(state: &AppState, code: &str) {
    cache().invalidate(&state.redis, &cache_key(code)).await;
}

pub async fn invalidate_all(state: &AppState) {
    cache().invalidate_prefix_local();
    let _ = state;
}

pub async fn get(state: &AppState, code: &str) -> AppResult<Option<SitePage>> {
    let key = cache_key(code);
    let code_owned = code.to_string();
    let st = state.clone();
    let arc = cache()
        .get_or_load(
            &state.redis,
            key,
            PAGE_TTL,
            "site_page",
            move || async move { Ok(site_page_repo::find_by_code(st.db.reader(), &code_owned).await?) },
        )
        .await?;
    Ok((*arc).clone())
}
