//! Cached public `/v1/wap/initjobs` payload (per language).
//!
//! The blob is language-keyed and shared. Per-IP `sy_client_ip_banned` is
//! applied by the handler after a hit — do not store it in this cache.

use phpyun_core::cache::TieredCache;
use phpyun_core::{AppResult, AppState};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

const TTL: Duration = Duration::from_secs(60);
const LANGS: &[&str] = &["zh-CN", "zh-TW", "en"];

fn cache_key(lang: &str) -> String {
    format!("initjobs:{lang}")
}

static CACHE: OnceLock<TieredCache<String>> = OnceLock::new();

fn cache() -> &'static TieredCache<String> {
    CACHE.get_or_init(|| TieredCache::new(8, TTL))
}

pub async fn get_or_load<F, Fut>(state: &AppState, lang: &str, loader: F) -> AppResult<Arc<String>>
where
    F: FnOnce() -> Fut + Send,
    Fut: std::future::Future<Output = AppResult<String>> + Send,
{
    cache()
        .get_or_load(&state.redis, cache_key(lang), TTL, "initjobs", loader)
        .await
}

pub async fn invalidate(state: &AppState) {
    cache().invalidate_prefix_local();
    for lang in LANGS {
        cache().invalidate(&state.redis, &cache_key(lang)).await;
    }
}
