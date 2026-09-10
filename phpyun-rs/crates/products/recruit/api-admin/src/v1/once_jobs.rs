//! Shop-recruit (once) review. PHP `weipin_once`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::once_job::entity::OnceJob;
use phpyun_services::admin_cms_service;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs", post(list))
        .route("/once-jobs/status", post(set_status))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/once-jobs", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<OnceJob>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_once(&state, q.status, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub ids: Vec<u64>,
    pub status: i32,
}

fn status_ids(id: u64, mut ids: Vec<u64>) -> Vec<u64> {
    if id > 0 && !ids.contains(&id) {
        ids.insert(0, id);
    }
    ids.retain(|&x| x > 0);
    ids.sort_unstable();
    ids.dedup();
    ids
}

#[utoipa::path(post, path = "/v1/admin/once-jobs/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let ids = status_ids(f.id, f.ids);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("id"));
    }
    for id in ids {
        admin_cms_service::set_once_status(&state, &user, id, f.status).await?;
    }
    Ok(ApiResponse::message("ok"))
}
