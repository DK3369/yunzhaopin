//! Site-wide gates aligned with PHP `common.php::toLoginPage` and Smarty `is_fun()`.

use phpyun_core::cache::TieredCache;
use phpyun_core::{
    extractors::AuthenticatedUser, rate_limit, ApiError, AppResult, AppState,
};
use phpyun_models::site_setting::repo as setting_repo;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

const SETTING_TTL: Duration = Duration::from_secs(30);

static ALL_CACHE: OnceLock<TieredCache<HashMap<String, String>>> = OnceLock::new();

fn all_cache() -> &'static TieredCache<HashMap<String, String>> {
    ALL_CACHE.get_or_init(|| TieredCache::new(4, SETTING_TTL))
}

/// Full `phpyun_admin_config` map (one MySQL read per TTL).
pub async fn config_map(state: &AppState) -> Result<Arc<HashMap<String, String>>, ApiError> {
    let pool = state.db.reader().clone();
    all_cache()
        .get_or_load(
            &state.redis,
            phpyun_core::cache::SITE_SETTINGS_ALL_KEY.to_string(),
            SETTING_TTL,
            "site_settings.all",
            move || async move {
                let rows = setting_repo::list_all(&pool).await?;
                Ok(rows
                    .into_iter()
                    .map(|s| (s.key_name, s.value))
                    .collect::<HashMap<String, String>>())
            },
        )
        .await
}

/// Drop `site_settings:all` (L1 + Redis). Per-name `site_setting:{name}` keys
/// still expire on their own TTL / explicit `invalidate`.
pub async fn invalidate_settings_bundle(state: &AppState) {
    all_cache()
        .invalidate(&state.redis, phpyun_core::cache::SITE_SETTINGS_ALL_KEY)
        .await;
}

/// Cached `phpyun_admin_config` value. Empty string when missing.
pub async fn config_str(state: &AppState, key: &str) -> String {
    match config_map(state).await {
        Ok(map) => map.get(key).cloned().unwrap_or_default(),
        Err(_) => String::new(),
    }
}

async fn setting(state: &AppState, key: &str) -> String {
    config_str(state, key).await
}

pub async fn setting_i32(state: &AppState, key: &str) -> i32 {
    setting(state, key).await.trim().parse().unwrap_or(0)
}

/// PHP data-cycle: when the caller did not pass `uptime`, use `sy_datacycle*`.
pub async fn default_uptime_days(
    state: &AppState,
    explicit: Option<i32>,
    key: &str,
) -> Option<i32> {
    if explicit.filter(|d| *d > 0).is_some() {
        return explicit;
    }
    let n = setting_i32(state, key).await;
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

/// PHP `sy_web_online == 2` → whole frontend closed.
pub async fn ensure_site_online(state: &AppState) -> AppResult<()> {
    if setting(state, "sy_web_online").await.trim() == "2" {
        return Err(ApiError::business("site_closed"));
    }
    Ok(())
}

/// PHP `sy_bannedip` pipe-separated list.
pub async fn ensure_ip_allowed(state: &AppState, ip: &str) -> AppResult<()> {
    let raw = setting(state, "sy_bannedip").await;
    if raw.trim().is_empty() {
        return Ok(());
    }
    let ip = ip.trim();
    if ip.is_empty() || ip == "0.0.0.0" {
        return Ok(());
    }
    for part in raw.split('|') {
        let needle = part.trim();
        if !needle.is_empty() && ip.contains(needle) {
            return Err(ApiError::business("ip_banned"));
        }
    }
    Ok(())
}

/// PHP `reg_user_stop != 1` → registration closed.
pub async fn ensure_registration_open(state: &AppState) -> AppResult<()> {
    let v = setting(state, "reg_user_stop").await;
    let v = v.trim();
    if !v.is_empty() && v != "1" {
        return Err(ApiError::business("registration_closed"));
    }
    Ok(())
}

/// PHP `toLoginPage`: `sy_list_login==1` and guest, unless UA matches `sy_list_agent`.
pub async fn ensure_list_login(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    user_agent: &str,
) -> AppResult<()> {
    if user.is_some() {
        return Ok(());
    }
    if setting(state, "sy_list_login").await.trim() != "1" {
        return Ok(());
    }
    let agents = setting(state, "sy_list_agent").await;
    if !agents.trim().is_empty() {
        let ua = user_agent.to_ascii_lowercase();
        for line in agents.lines() {
            let needle = line.trim();
            if !needle.is_empty() && ua.contains(&needle.to_ascii_lowercase()) {
                return Ok(());
            }
        }
    }
    Err(ApiError::unauth())
}

/// Public list endpoints: per-IP Redis cap (Governor is the coarse layer).
pub async fn ensure_public_list_rate(state: &AppState, ip: &str) -> AppResult<()> {
    rate_limit::check_wap_list(&state.redis, ip).await
}

/// Job / resume / company detail: per-uid cap after login.
pub async fn ensure_public_detail_rate(state: &AppState, uid: u64) -> AppResult<()> {
    rate_limit::check_wap_detail(&state.redis, uid).await
}

/// PHP `sy_{module}_web == 2` closes the column.
pub async fn ensure_module_on(state: &AppState, key: &str) -> AppResult<()> {
    if setting(state, key).await.trim() == "2" {
        return Err(ApiError::business("module_closed"));
    }
    Ok(())
}
