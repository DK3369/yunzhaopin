//! Admin recycle bin. List snapshots / view detail / permanent delete. Restoration is handled by each business service.

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::recycle_bin_service::{self, RecycleView};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recycle-bin", post(list))
        .route("/recycle-bin/detail", post(detail))
        .route("/recycle-bin/purge", post(purge))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 100))]
    pub tablename: Option<String>,
}

/// Recycle bin list
#[utoipa::path(
    post,
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
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<RecycleView>>> {
    user.require_admin()?;
    let r = recycle_bin_service::list(&state, q.tablename.as_deref(), page).await?;
    Ok(ApiJson(r))
}

/// Single record detail
#[utoipa::path(post,
    path = "/v1/admin/recycle-bin",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn detail(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<RecycleView>> {
    let id = b.id;
    user.require_admin()?;
    Ok(ApiJson(recycle_bin_service::get(&state, id).await?))
}

/// Permanently delete
#[utoipa::path(post,
    path = "/v1/admin/recycle-bin",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn purge(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    recycle_bin_service::purge(&state, &user, id).await?;
    Ok(ApiOk("purged"))
}

