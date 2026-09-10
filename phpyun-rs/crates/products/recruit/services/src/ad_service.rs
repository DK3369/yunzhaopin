//! Ad slots (aligned with PHPYun `ad.model.php`).
//!
//! `/v1/wap/ads` and `/v1/wap/initads` share one in-process cache. It is dropped
//! by admin 清缓存, 广告页「更新缓存」(cache_ad), and ad write paths so the
//! next public read hits the DB.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{
    audit, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::ad::{entity::Ad, repo as ad_repo};
use sqlx::MySqlPool;
use std::collections::BTreeMap;

const TTL_SECS: u64 = 300;

static CACHE: std::sync::OnceLock<SimpleCache<String, BTreeMap<String, Vec<Ad>>>> =
    std::sync::OnceLock::new();

fn cache() -> &'static SimpleCache<String, BTreeMap<String, Vec<Ad>>> {
    CACHE.get_or_init(|| SimpleCache::new(256, std::time::Duration::from_secs(TTL_SECS)))
}

pub fn invalidate_all() {
    cache().invalidate_all();
}

pub struct SlotNeed {
    pub slot: String,
    pub limit: u64,
}

pub async fn list_active(state: &AppState, slot: &str, limit: u64) -> AppResult<Vec<Ad>> {
    let mut map = list_active_many(
        state,
        &[SlotNeed {
            slot: slot.to_string(),
            limit,
        }],
    )
    .await?;
    Ok(map.remove(slot).unwrap_or_default())
}

/// One SQL for many slots. Missing / empty slots still appear as `[]`.
pub async fn list_active_many(
    state: &AppState,
    needs: &[SlotNeed],
) -> AppResult<BTreeMap<String, Vec<Ad>>> {
    let mut cleaned: Vec<(String, u64)> = Vec::new();
    for n in needs.iter().take(32) {
        let slot = n.slot.trim();
        if slot.is_empty() {
            continue;
        }
        cleaned.push((slot.to_string(), n.limit.clamp(1, 50)));
    }
    if cleaned.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut key_parts: Vec<String> = cleaned
        .iter()
        .map(|(s, l)| format!("{s}:{l}"))
        .collect();
    key_parts.sort();
    let cache_key = key_parts.join("|");
    let db = state.db.reader().clone();
    let cached = cache()
        .get_or_load(cache_key, move || async move { load_many(&db, &cleaned).await })
        .await?;
    Ok((*cached).clone())
}

async fn load_many(db: &MySqlPool, needs: &[(String, u64)]) -> AppResult<BTreeMap<String, Vec<Ad>>> {
    let now = clock::now_ts();
    let mut limits: BTreeMap<String, usize> = BTreeMap::new();
    let mut id_to_key: BTreeMap<i32, String> = BTreeMap::new();
    let mut class_ids: Vec<i32> = Vec::new();
    for (slot, limit) in needs {
        let cap = usize::try_from(*limit).unwrap_or(50).clamp(1, 50);
        limits.insert(slot.clone(), cap);
        let id: i32 = slot.parse().unwrap_or(0);
        id_to_key.entry(id).or_insert_with(|| slot.clone());
        if !class_ids.contains(&id) {
            class_ids.push(id);
        }
    }
    let rows = ad_repo::list_active_in_slots(db, &class_ids, now).await?;
    let mut grouped: BTreeMap<String, Vec<Ad>> = BTreeMap::new();
    for slot in limits.keys() {
        grouped.insert(slot.clone(), Vec::new());
    }
    for ad in rows {
        let cid: i32 = ad.slot.parse().unwrap_or(0);
        let key = id_to_key.get(&cid).cloned().unwrap_or_else(|| ad.slot.clone());
        let Some(cap) = limits.get(&key).copied() else {
            continue;
        };
        let bucket = grouped.entry(key).or_default();
        if bucket.len() < cap {
            bucket.push(ad);
        }
    }
    Ok(grouped)
}

// ---------- admin ----------

pub async fn admin_list(
    state: &AppState,
    user: &AuthenticatedUser,
    slot: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<Ad>> {
    user.require_admin()?;
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        ad_repo::list_all(db, slot, page.offset, page.limit),
        ad_repo::count_all(db, slot),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub struct AdInput<'a> {
    pub slot: &'a str,
    pub title: &'a str,
    pub image: &'a str,
    pub link: &'a str,
    pub weight: i32,
    pub start_at: i64,
    pub end_at: i64,
}

pub async fn admin_create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: AdInput<'_>,
) -> AppResult<u64> {
    user.require_admin()?;
    let id = ad_repo::create(
        state.db.pool(),
        ad_repo::AdCreate {
            slot: input.slot,
            title: input.title,
            image: input.image,
            link: input.link,
            weight: input.weight,
            start_at: input.start_at,
            end_at: input.end_at,
        },
        clock::now_ts(),
    )
    .await?;
    invalidate_all();
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.ad.create", audit::Actor::uid(user.uid))
            .target(format!("ad:{id}")),
    )
    .await;
    Ok(id)
}

pub struct AdPatch<'a> {
    pub slot: Option<&'a str>,
    pub title: Option<&'a str>,
    pub image: Option<&'a str>,
    pub link: Option<&'a str>,
    pub weight: Option<i32>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    pub status: Option<i32>,
}

pub async fn admin_update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    patch: AdPatch<'_>,
) -> AppResult<()> {
    user.require_admin()?;
    let affected = ad_repo::update(
        state.db.pool(),
        id,
        ad_repo::AdUpdate {
            slot: patch.slot,
            title: patch.title,
            image: patch.image,
            link: patch.link,
            weight: patch.weight,
            start_at: patch.start_at,
            end_at: patch.end_at,
            status: patch.status,
        },
    )
    .await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("ad_not_found"));
    }
    invalidate_all();
    Ok(())
}

pub async fn admin_delete(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<()> {
    crate::admin_auth_service::require_active_admin(state, user).await?;
    ad_repo::delete(state.db.pool(), id).await?;
    invalidate_all();
    Ok(())
}
