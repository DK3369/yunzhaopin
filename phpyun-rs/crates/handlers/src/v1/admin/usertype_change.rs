//! Admin: user-type switch request approvals (matching the PHPYun admin approval flow).

use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp,
};
use phpyun_services::usertype_change_service;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/usertype-changes/{id}/approve",
            post(approve),
        )
        .route(
            "/usertype-changes/{id}/reject",
            post(reject),
        )
}

#[utoipa::path(
    post,
    path = "/v1/admin/usertype-changes/{id}/approve",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn approve(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<phpyun_core::json::Value>> {
    user.require_admin()?;
    let n = usertype_change_service::admin_approve(&state, &user, id, &ip).await?;
    Ok(ApiJson(phpyun_core::json::json!({ "updated": n })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/usertype-changes/{id}/reject",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn reject(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<phpyun_core::json::Value>> {
    user.require_admin()?;
    let n = usertype_change_service::admin_reject(&state, &user, id, &ip).await?;
    Ok(ApiJson(phpyun_core::json::json!({ "updated": n })))
}
