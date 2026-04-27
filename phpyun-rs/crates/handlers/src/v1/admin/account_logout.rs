//! Admin: account deletion request approvals.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{
    dto::IdBody, json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::member_logout_service;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/account-logouts/approve", post(approve))
        .route("/account-logouts/reject", post(reject))
}

#[utoipa::path(
    post,
    path = "/v1/admin/account-logouts/approve",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn approve(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    user.require_admin()?;
    let n = member_logout_service::admin_approve(&state, &user, b.id, &ip).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/account-logouts/reject",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn reject(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    user.require_admin()?;
    let n = member_logout_service::admin_reject(&state, &user, b.id, &ip).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}
