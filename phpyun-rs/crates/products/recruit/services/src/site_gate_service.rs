//! Site-wide gates aligned with PHP `common.php::toLoginPage` and Smarty `is_fun()`.

use phpyun_core::{
    extractors::AuthenticatedUser, ApiError, AppResult, AppState,
};
use phpyun_models::site_setting::repo as setting_repo;

async fn setting(state: &AppState, key: &str) -> String {
    setting_repo::find(state.db.reader(), key)
        .await
        .ok()
        .flatten()
        .map(|s| s.value)
        .unwrap_or_default()
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

/// PHP `sy_{module}_web == 2` closes the column.
pub async fn ensure_module_on(state: &AppState, key: &str) -> AppResult<()> {
    if setting(state, key).await.trim() == "2" {
        return Err(ApiError::business("module_closed"));
    }
    Ok(())
}
