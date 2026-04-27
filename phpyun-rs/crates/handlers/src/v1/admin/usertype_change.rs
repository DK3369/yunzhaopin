//! Admin: user-type switch request approvals (matching the PHPYun admin approval flow).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::usertype_change_service;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/usertype-changes/approve",
            post(approve),
        )
        .route("/usertype-changes/reject",
            post(reject),
        )
}

#[utoipa::path(post,
    path = "/v1/admin/usertype-changes/approve",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn approve(State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<phpyun_core::json::Value>> {
    let id = b.id;
    user.require_admin()?;
    let n = usertype_change_service::admin_approve(&state, &user, id, &ip).await?;
    Ok(ApiJson(phpyun_core::json::json!({ "updated": n })))
}

#[utoipa::path(post,
    path = "/v1/admin/usertype-changes/reject",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn reject(State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<phpyun_core::json::Value>> {
    let id = b.id;
    user.require_admin()?;
    let n = usertype_change_service::admin_reject(&state, &user, id, &ip).await?;
    Ok(ApiJson(phpyun_core::json::json!({ "updated": n })))
}

