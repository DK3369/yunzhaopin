//! Ad slots (aligned with PHPYun `ad.model.php`).
//!
//! Public endpoint returns the currently active ads for a `slot`; admin endpoint performs CRUD.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::ad::{entity::Ad, repo as ad_repo};

pub async fn list_active(
    state: &AppState,
    slot: &str,
    limit: u64,
) -> AppResult<Vec<Ad>> {
    let now = clock::now_ts();
    Ok(ad_repo::list_active(state.db.reader(), slot, now, limit.clamp(1, 50)).await?)
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
        return Err(AppError::new(InfraError::InvalidParam("ad_not_found".into())));
    }
    Ok(())
}

pub async fn admin_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_admin()?;
    ad_repo::delete(state.db.pool(), id).await?;
    Ok(())
}
