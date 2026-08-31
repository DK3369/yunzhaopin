//! Part-time job review. PHP `pid` + `status` → column `state`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::part::entity::PartJob;
use phpyun_services::admin_cms_service;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/parts", post(list))
        .route("/parts/state", post(set_state))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[serde(default, alias = "status", deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub state: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/parts", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<AdminPaged<PartJob>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_cms_service::list_parts(&state, q.state, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetPartStateForm {
    #[serde(alias = "pid")]
    #[validate(range(min = 1))]
    pub id: u64,
    #[serde(alias = "status")]
    pub state: i32,
    #[serde(default)]
    pub statusbody: String,
}

#[utoipa::path(post, path = "/v1/admin/parts/state", tag = "admin", security(("bearer" = [])), request_body = SetPartStateForm, responses((status = 200, description = "ok")))]
pub async fn set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetPartStateForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::set_part_state(&state, &user, f.id, f.state, &f.statusbody).await?;
    Ok(ApiResponse::message("ok"))
}
