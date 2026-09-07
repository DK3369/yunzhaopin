//! Curated major-country lookup.
//!
//! Cache: a single `Arc<Vec<Country>>` keyed by `()` with a 30-minute TTL.
//! The list rarely changes (admins rarely edit it) and is rendered on
//! every page that has a country selector, so the cache pays for itself.
//! Admin mutations call `invalidate()` to force the next reader to refresh.
//!
//! `status`: `0` hidden / not yet picked, `1` show on the public site,
//! `2` deleted. Until any row is `1`, the public list still returns every
//! live row so a fresh seed is not an empty dropdown.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::country::entity::{Country, STATUS_ENABLED, STATUS_HIDDEN};
use phpyun_models::country::repo as country_repo;
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

async fn list_active_cached(state: &AppState) -> AppResult<Arc<Vec<Country>>> {
    let db = state.db.reader().clone();
    cache()
        .get_or_load((), || async move {
            country_repo::list_active(&db).await.map_err(Into::into)
        })
        .await
}

/// Public dropdown. If the admin has picked at least one country
/// (`status = 1`), only those are returned.
pub async fn list_all(state: &AppState) -> AppResult<Arc<Vec<Country>>> {
    let all = list_active_cached(state).await?;
    if all.iter().any(|c| c.status == STATUS_ENABLED) {
        Ok(Arc::new(
            all.iter()
                .filter(|c| c.status == STATUS_ENABLED)
                .cloned()
                .collect(),
        ))
    } else {
        Ok(all)
    }
}

/// Admin table: every live row, including those not currently shown.
pub async fn list_admin(state: &AppState) -> AppResult<Vec<Country>> {
    let all = list_active_cached(state).await?;
    Ok((*all).clone())
}

/// Filter helper for `?continent=AS`.
pub async fn list_by_continent(state: &AppState, continent: &str) -> AppResult<Vec<Country>> {
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

/// Unicode regional-indicator flag from an ISO alpha-2 code.
pub fn flag_from_code(code: &str) -> String {
    let b = code.as_bytes();
    if b.len() != 2 || !b[0].is_ascii_alphabetic() || !b[1].is_ascii_alphabetic() {
        return String::new();
    }
    let a = 0x1F1E6u32 + u32::from(b[0].to_ascii_uppercase() - b'A');
    let c = 0x1F1E6u32 + u32::from(b[1].to_ascii_uppercase() - b'A');
    match (char::from_u32(a), char::from_u32(c)) {
        (Some(x), Some(y)) => format!("{x}{y}"),
        _ => String::new(),
    }
}

fn trimmed(s: Option<String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

pub struct AdminCountryCreate {
    pub code: String,
    pub code3: Option<String>,
    pub numeric_code: Option<u16>,
    pub name_en: Option<String>,
    pub name_zh: String,
    pub continent: Option<String>,
    pub phone_code: Option<String>,
    pub currency: Option<String>,
    pub flag: Option<String>,
    pub sort: Option<i32>,
}

/// Add a country that can appear on the site without a city/region tree.
pub async fn admin_create(state: &AppState, f: AdminCountryCreate) -> AppResult<u64> {
    let code = f.code.trim().to_uppercase();
    if code.len() != 2 || !code.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(ApiError::param_invalid("invalid_country_code"));
    }
    let name_zh = f.name_zh.trim();
    if name_zh.is_empty() {
        return Err(ApiError::param_invalid("name_zh_required"));
    }
    if country_repo::find_by_code(state.db.reader(), &code)
        .await
        .map_err(ApiError::internal)?
        .is_some()
    {
        return Err(ApiError::param_invalid("country_code_exists"));
    }
    let name_en = trimmed(f.name_en).unwrap_or_else(|| name_zh.to_string());
    let code3 = trimmed(f.code3)
        .filter(|s| s.len() == 3 && s.chars().all(|c| c.is_ascii_alphabetic()))
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| format!("{code}X"));
    let continent = trimmed(f.continent)
        .filter(|s| s.len() == 2)
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "AS".to_string());
    let phone_code = trimmed(f.phone_code).unwrap_or_else(|| "0".to_string());
    let currency = trimmed(f.currency)
        .filter(|s| s.len() == 3)
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "XXX".to_string());
    let flag = trimmed(f.flag).unwrap_or_else(|| flag_from_code(&code));
    let sort = f.sort.unwrap_or(0);
    let numeric_code = f.numeric_code.unwrap_or(0);
    let any_picked = country_repo::list_active(state.db.reader())
        .await
        .map_err(ApiError::internal)?
        .iter()
        .any(|c| c.status == STATUS_ENABLED);
    let status = if any_picked { STATUS_ENABLED } else { STATUS_HIDDEN };
    let id = country_repo::create(
        state.db.pool(),
        country_repo::CountryCreate {
            code: &code,
            code3: &code3,
            numeric_code,
            name_en: &name_en,
            name_zh,
            continent: &continent,
            phone_code: &phone_code,
            currency: &currency,
            flag: &flag,
            sort,
            status,
        },
        clock::now_ts(),
    )
    .await
    .map_err(ApiError::internal)?;
    invalidate().await;
    Ok(id)
}

/// Replace the public set. At least one id is required.
pub async fn select_enabled(state: &AppState, ids: &[u64]) -> AppResult<u64> {
    let mut ids: Vec<u64> = ids.iter().copied().filter(|id| *id > 0).collect();
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("select_at_least_one_country"));
    }
    if ids.len() > 500 {
        return Err(ApiError::param_invalid("too_many_countries"));
    }
    let n = country_repo::set_enabled(state.db.pool(), &ids, clock::now_ts())
        .await
        .map_err(ApiError::internal)?;
    invalidate().await;
    Ok(n)
}
