//! Site settings (aligned with PHPYun `sy_*` global toggles).
//!
//! Public endpoint: read-only access to keys with `is_public=1`. Admin endpoint: full access plus create/update/delete.

use phpyun_core::{audit, clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::site_setting::{entity::SiteSetting, repo as setting_repo};

pub async fn list_public(state: &AppState) -> AppResult<Vec<SiteSetting>> {
    Ok(setting_repo::list_public(state.db.reader()).await?)
}

pub async fn get(state: &AppState, key: &str) -> AppResult<Option<SiteSetting>> {
    Ok(setting_repo::find(state.db.reader(), key).await?)
}

// ---------- admin ----------

pub async fn admin_list(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<SiteSetting>> {
    user.require_admin()?;
    Ok(setting_repo::list_all(state.db.reader()).await?)
}

pub struct UpsertInput<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub description: &'a str,
    pub is_public: bool,
}

pub async fn admin_upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: UpsertInput<'_>,
) -> AppResult<()> {
    user.require_admin()?;
    let now = clock::now_ts();
    setting_repo::upsert(
        state.db.pool(),
        input.key,
        input.value,
        input.description,
        input.is_public,
        now,
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.upsert", audit::Actor::uid(user.uid))
            .target(format!("key:{}", input.key)),
    )
    .await;
    Ok(())
}

pub async fn admin_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    key: &str,
) -> AppResult<()> {
    user.require_admin()?;
    setting_repo::delete(state.db.pool(), key).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.delete", audit::Actor::uid(user.uid))
            .target(format!("key:{key}")),
    )
    .await;
    Ok(())
}
