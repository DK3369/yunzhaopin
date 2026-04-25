//! App version check (aligned with PHPYun `version.model.php`).

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::app_version::{entity::AppVersion, repo as ver_repo};

pub async fn latest(state: &AppState, platform: &str) -> AppResult<Option<AppVersion>> {
    if platform.is_empty() {
        return Err(AppError::new(InfraError::MissingParam("platform")));
    }
    Ok(ver_repo::latest_for_platform(state.db.reader(), platform).await?)
}

// ---------- admin ----------

pub async fn admin_list(
    state: &AppState,
    admin: &AuthenticatedUser,
    platform: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<AppVersion>> {
    admin.require_admin()?;
    // best-effort count via admin_list.len (small table)
    let list =
        ver_repo::admin_list(state.db.reader(), platform, page.offset, page.limit).await?;
    let total = list.len() as u64 + page.offset;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct VersionInput<'a> {
    pub platform: &'a str,
    pub version: &'a str,
    pub version_code: u32,
    pub is_force: bool,
    pub download_url: &'a str,
    pub changelog: &'a str,
    pub released_at: i64,
}

pub async fn admin_create(
    state: &AppState,
    admin: &AuthenticatedUser,
    input: VersionInput<'_>,
) -> AppResult<u64> {
    admin.require_admin()?;
    let id = ver_repo::create(
        state.db.pool(),
        ver_repo::VersionCreate {
            platform: input.platform,
            version: input.version,
            version_code: input.version_code,
            is_force: input.is_force,
            download_url: input.download_url,
            changelog: input.changelog,
            released_at: input.released_at,
        },
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.app_version.create", audit::Actor::uid(admin.uid))
            .target(format!("version:{id}"))
            .meta(&serde_json::json!({
                "platform": input.platform,
                "version": input.version,
                "code": input.version_code,
            })),
    )
    .await;
    Ok(id)
}

pub async fn admin_delete(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    admin.require_admin()?;
    ver_repo::delete(state.db.pool(), id).await?;
    Ok(())
}
