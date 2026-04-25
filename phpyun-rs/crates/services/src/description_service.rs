//! Generic single-page CMS (aligned with PHPYun `description` + `desc_class`).
//!
//! Complements `site_page_service` (which covers fixed pages like about/privacy/contact) —
//! here admins can freely create categories and pages, and the frontend references them by id.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::description::{
    entity::{DescClass, Description},
    repo as desc_repo,
};

// ---------- classes ----------

/// 60s TTL cache: single-page categories rarely change yet are requested on every site visit.
/// moka's `try_get_with` provides built-in singleflight, so the cache-stampede scenario is safe.
static CLASSES_CACHE: std::sync::OnceLock<
    moka::future::Cache<(), std::sync::Arc<Vec<DescClass>>>,
> = std::sync::OnceLock::new();

fn classes_cache() -> &'static moka::future::Cache<(), std::sync::Arc<Vec<DescClass>>> {
    CLASSES_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(1)
            .time_to_live(std::time::Duration::from_secs(60))
            .build()
    })
}

/// Invalidate after writes (call after admin create/update/delete)
async fn invalidate_classes_cache() {
    if let Some(c) = CLASSES_CACHE.get() {
        c.invalidate(&()).await;
    }
}

pub async fn list_classes(state: &AppState) -> AppResult<std::sync::Arc<Vec<DescClass>>> {
    let cache = classes_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with((), async move {
            let list = desc_repo::list_classes(&db).await?;
            Ok::<_, AppError>(std::sync::Arc::new(list))
        })
        .await
        .map_err(AppError::from_arc)
}

pub async fn create_class(
    state: &AppState,
    admin: &AuthenticatedUser,
    name: &str,
    sort: i32,
) -> AppResult<u64> {
    let id = desc_repo::insert_class(state.db.pool(), name, sort, clock::now_ts()).await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.create", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn update_class_sort(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    sort: i32,
) -> AppResult<()> {
    let n = desc_repo::update_class_sort(state.db.pool(), id, sort).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("class_not_found".into())));
    }
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.update_sort", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}"))
            .meta(&serde_json::json!({ "sort": sort })),
    )
    .await;
    Ok(())
}

pub async fn delete_class(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let n = desc_repo::delete_class(state.db.pool(), id).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("class_not_found".into())));
    }
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.delete", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}")),
    )
    .await;
    Ok(())
}

// ---------- descriptions ----------

pub async fn list(
    state: &AppState,
    class_id: Option<u64>,
    only_visible: bool,
    page: Pagination,
) -> AppResult<Paged<Description>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        desc_repo::list(db, class_id, only_visible, page.offset, page.limit),
        desc_repo::count(db, class_id, only_visible),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Description> {
    desc_repo::get(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("description_not_found".into())))
}

pub struct UpsertForm<'a> {
    pub id: Option<u64>,
    pub class_id: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub is_type: i32,
    pub link_url: &'a str,
    pub sort: i32,
    pub status: i32,
}

pub async fn upsert(
    state: &AppState,
    admin: &AuthenticatedUser,
    f: &UpsertForm<'_>,
) -> AppResult<u64> {
    let id = desc_repo::upsert(
        state.db.pool(),
        &desc_repo::UpsertDesc {
            id: f.id,
            class_id: f.class_id,
            title: f.title,
            content: f.content,
            is_type: f.is_type,
            link_url: f.link_url,
            sort: f.sort,
            status: f.status,
        },
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new(
            if f.id.is_some() { "admin.description.update" } else { "admin.description.create" },
            Actor::uid(admin.uid),
        )
        .target(format!("description:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn delete(state: &AppState, admin: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = desc_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("description_not_found".into())));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.description.delete", Actor::uid(admin.uid))
            .target(format!("description:{id}")),
    )
    .await;
    Ok(())
}
