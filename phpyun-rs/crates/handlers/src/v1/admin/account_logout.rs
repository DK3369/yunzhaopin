//! Admin: account deletion request approvals.

use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp,
};
use phpyun_services::member_logout_service;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/account-logouts/{id}/approve", post(approve))
        .route("/account-logouts/{id}/reject", post(reject))
}

#[utoipa::path(
    post,
    path = "/v1/admin/account-logouts/{id}/approve",
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
) -> AppResult<ApiJson<json::Value>> {
    user.require_admin()?;
    let n = member_logout_service::admin_approve(&state, &user, id, &ip).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/account-logouts/{id}/reject",
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
) -> AppResult<ApiJson<json::Value>> {
    user.require_admin()?;
    let n = member_logout_service::admin_reject(&state, &user, id, &ip).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}
