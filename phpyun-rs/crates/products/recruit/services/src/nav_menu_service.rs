//! Navigation menu (aligned with PHPYun `navigation.model.php`).

use phpyun_core::cache::TieredCache;
use phpyun_core::{audit, clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::nav_menu::{entity::NavMenu, repo as nav_repo};
use std::sync::OnceLock;
use std::time::Duration;

const LIST_TTL: Duration = Duration::from_secs(60);

static CACHE: OnceLock<TieredCache<Vec<NavMenu>>> = OnceLock::new();

fn cache() -> &'static TieredCache<Vec<NavMenu>> {
    CACHE.get_or_init(|| TieredCache::new(32, LIST_TTL))
}

fn cache_key(position: &str) -> String {
    format!("nav:{position}")
}

pub async fn invalidate_all(state: &AppState) {
    cache().invalidate_prefix_local();
    for p in ["0", "1", "2", "top", "bottom"] {
        cache().invalidate(&state.redis, &cache_key(p)).await;
    }
}

async fn invalidate_position(state: &AppState, position: &str) {
    cache().invalidate_prefix_local();
    cache().invalidate(&state.redis, &cache_key(position)).await;
}

pub async fn list(state: &AppState, position: &str) -> AppResult<Vec<NavMenu>> {
    let key = cache_key(position);
    let pos = position.to_string();
    let st = state.clone();
    let arc = cache()
        .get_or_load(
            &state.redis,
            key,
            LIST_TTL,
            "nav",
            move || async move { Ok(nav_repo::list_public(st.db.reader(), &pos).await?) },
        )
        .await?;
    Ok((*arc).clone())
}

// ---------- admin ----------

pub async fn admin_list(
    state: &AppState,
    admin: &AuthenticatedUser,
    position: Option<&str>,
) -> AppResult<Vec<NavMenu>> {
    admin.require_admin()?;
    Ok(nav_repo::admin_list(state.db.reader(), position).await?)
}

pub struct NavInput<'a> {
    pub position: &'a str,
    pub label: &'a str,
    pub url: &'a str,
    pub icon: &'a str,
    pub parent_id: u64,
    pub sort: i32,
}

pub async fn admin_create(
    state: &AppState,
    admin: &AuthenticatedUser,
    input: NavInput<'_>,
) -> AppResult<u64> {
    admin.require_admin()?;
    let id = nav_repo::create(
        state.db.pool(),
        nav_repo::NavCreate {
            position: input.position,
            label: input.label,
            url: input.url,
            icon: input.icon,
            parent_id: input.parent_id,
            sort: input.sort,
        },
        clock::now_ts(),
    )
    .await?;
    invalidate_position(state, input.position).await;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.nav_menu.create", audit::Actor::uid(admin.uid))
            .target(format!("nav:{id}")),
    )
    .await;
    Ok(id)
}

pub struct NavPatch<'a> {
    pub label: Option<&'a str>,
    pub url: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub parent_id: Option<u64>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn admin_update(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    patch: NavPatch<'_>,
) -> AppResult<()> {
    admin.require_admin()?;
    let affected = nav_repo::update(
        state.db.pool(),
        id,
        nav_repo::NavUpdate {
            label: patch.label,
            url: patch.url,
            icon: patch.icon,
            parent_id: patch.parent_id,
            sort: patch.sort,
            status: patch.status,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("nav_not_found"));
    }
    invalidate_all(state).await;
    Ok(())
}

pub async fn admin_delete(state: &AppState, admin: &AuthenticatedUser, id: u64) -> AppResult<()> {
    crate::admin_auth_service::require_active_admin(state, admin).await?;
    nav_repo::delete(state.db.pool(), id).await?;
    invalidate_all(state).await;
    Ok(())
}
