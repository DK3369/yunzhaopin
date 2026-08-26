//! Tiny resume review. PHP `weipin_tiny`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::tiny::entity::TinyResume;
use phpyun_services::admin_cms_service;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tiny", post(list))
        .route("/tiny/status", post(set_status))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/tiny", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<TinyResume>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_tiny(&state, q.status, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub status: i32,
}

#[utoipa::path(post, path = "/v1/admin/tiny/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::set_tiny_status(&state, &user, f.id, f.status).await?;
    Ok(ApiResponse::message("ok"))
}
