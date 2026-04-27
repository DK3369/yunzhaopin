//! Category tree (aligned with PHPYun `category.model.php`).
//!
//! Public endpoint returns all active nodes by `kind`; admin endpoint performs CRUD and reorders within the same `kind`.
//!
//! Public reads are fronted by a 5-minute TTL cache keyed by (kind) and (kind,parent_id).
//! Admin writes invalidate the entire cache — categories don't churn fast enough for
//! finer-grained invalidation to matter.

use phpyun_core::cache::SimpleCache;
use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::category::{entity::Category, repo as cat_repo};
use std::sync::Arc;

const TTL_SECS: u64 = 300;

static LIST_CACHE: std::sync::OnceLock<SimpleCache<String, Vec<Category>>> =
    std::sync::OnceLock::new();

static CHILDREN_CACHE: std::sync::OnceLock<SimpleCache<(String, u64), Vec<Category>>> =
    std::sync::OnceLock::new();

fn list_cache() -> &'static SimpleCache<String, Vec<Category>> {
    LIST_CACHE.get_or_init(|| SimpleCache::new(64, std::time::Duration::from_secs(TTL_SECS)))
}

fn children_cache() -> &'static SimpleCache<(String, u64), Vec<Category>> {
    CHILDREN_CACHE.get_or_init(|| SimpleCache::new(256, std::time::Duration::from_secs(TTL_SECS)))
}

fn invalidate_all() {
    list_cache().invalidate_all();
    children_cache().invalidate_all();
}

pub async fn list(state: &AppState, kind: &str) -> AppResult<Arc<Vec<Category>>> {
    let key = kind.to_owned();
    let db = state.db.reader().clone();
    let kind_clone = key.clone();
    list_cache()
        .get_or_load(key, move || async move {
            Ok(cat_repo::list_all(&db, &kind_clone).await?)
        })
        .await
}

pub async fn list_children(
    state: &AppState,
    kind: &str,
    parent_id: u64,
) -> AppResult<Arc<Vec<Category>>> {
    let key = (kind.to_owned(), parent_id);
    let kind_clone = key.0.clone();
    let db = state.db.reader().clone();
    children_cache()
        .get_or_load(key, move || async move {
            Ok(cat_repo::list_children(&db, &kind_clone, parent_id).await?)
        })
        .await
}

// ---------- admin ----------

pub async fn admin_list(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: &str,
) -> AppResult<Vec<Category>> {
    user.require_admin()?;
    Ok(cat_repo::admin_list_by_kind(state.db.reader(), kind).await?)
}

pub struct CatInput<'a> {
    pub parent_id: u64,
    pub kind: &'a str,
    pub name: &'a str,
    pub sort: i32,
}

pub async fn admin_create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CatInput<'_>,
) -> AppResult<u64> {
    user.require_admin()?;
    let id = cat_repo::create(
        state.db.pool(),
        cat_repo::CatCreate {
            parent_id: input.parent_id,
            kind: input.kind,
            name: input.name,
            sort: input.sort,
        },
        clock::now_ts(),
    )
    .await?;
    invalidate_all();
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.category.create", audit::Actor::uid(user.uid))
            .target(format!("cat:{id}")),
    )
    .await;
    Ok(id)
}

pub struct CatPatch<'a> {
    pub parent_id: Option<u64>,
    pub name: Option<&'a str>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn admin_update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    patch: CatPatch<'_>,
) -> AppResult<()> {
    user.require_admin()?;
    let affected = cat_repo::update(
        state.db.pool(),
        id,
        cat_repo::CatUpdate {
            parent_id: patch.parent_id,
            name: patch.name,
            sort: patch.sort,
            status: patch.status,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam("cat_not_found".into())));
    }
    invalidate_all();
    Ok(())
}

pub async fn admin_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_admin()?;
    cat_repo::delete(state.db.pool(), id).await?;
    invalidate_all();
    Ok(())
}
