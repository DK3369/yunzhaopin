//! PHP `role_user` / `role_ugroup`：读 `phpyun_admin_user*`，不改 JWT。

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::admin_rbac::repo::{AdminRbacGroup, AdminRbacUser};
use phpyun_services::admin_longtail_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rbac/users", post(list_users))
        .route("/rbac/groups", post(list_groups))
        .route("/rbac/users/status", post(set_status))
}

#[utoipa::path(post, path = "/v1/admin/rbac/users", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_users(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<AdminRbacUser>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_rbac_users(&state, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/rbac/groups", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_groups(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<AdminRbacGroup>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_rbac_groups(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    pub status: i32,
}

#[utoipa::path(post, path = "/v1/admin/rbac/users/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_longtail_service::set_rbac_user_status(&state, &user, f.uid, f.status).await?;
    Ok(ApiResponse::message("ok"))
}
