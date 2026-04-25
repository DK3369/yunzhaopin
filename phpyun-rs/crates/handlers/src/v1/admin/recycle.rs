//! Admin recycle bin. List snapshots / view detail / permanent delete. Restoration is handled by each business service.

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_services::recycle_bin_service::{self, RecycleView};
use serde::Deserialize;
use utoipa::IntoParams;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recycle-bin", get(list))
        .route("/recycle-bin/{id}", get(detail).post(purge))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub tablename: Option<String>,
}

/// Recycle bin list
#[utoipa::path(
    get,
    path = "/v1/admin/recycle-bin",
    tag = "admin",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Paged<RecycleView>>> {
    user.require_admin()?;
    let r = recycle_bin_service::list(&state, q.tablename.as_deref(), page).await?;
    Ok(ApiJson(r))
}

/// Single record detail
#[utoipa::path(
    get,
    path = "/v1/admin/recycle-bin/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<RecycleView>> {
    user.require_admin()?;
    Ok(ApiJson(recycle_bin_service::get(&state, id).await?))
}

/// Permanently delete
#[utoipa::path(
    post,
    path = "/v1/admin/recycle-bin/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn purge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    recycle_bin_service::purge(&state, &user, id).await?;
    Ok(ApiOk("purged"))
}
