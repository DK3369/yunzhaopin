//! Navigation menu (aligned with PHPYun `navigation.model.php`).

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::nav_menu::{entity::NavMenu, repo as nav_repo};

pub async fn list(state: &AppState, position: &str) -> AppResult<Vec<NavMenu>> {
    Ok(nav_repo::list_public(state.db.reader(), position).await?)
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
        return Err(AppError::new(InfraError::InvalidParam("nav_not_found".into())));
    }
    Ok(())
}

pub async fn admin_delete(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    admin.require_admin()?;
    nav_repo::delete(state.db.pool(), id).await?;
    Ok(())
}
