//! Crawler source URLs. PHP-shaped `m=tool&c=thirdData`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::third_data::entity::ThirdData;
use phpyun_services::third_data_service::{self, UpsertIn};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/third-data", post(upsert))
        .route("/third-data/list", post(list))
        .route("/third-data/delete", post(delete))
}

#[utoipa::path(post, path = "/v1/admin/third-data/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<AdminPaged<ThirdData>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        third_data_service::list(&state, page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ThirdDataForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1, max = 512))]
    pub url: String,
    #[serde(default)]
    #[validate(length(max = 512))]
    pub api_url: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "default_enabled")]
    pub enabled: i32,
}

fn default_enabled() -> i32 {
    1
}

#[utoipa::path(post, path = "/v1/admin/third-data", tag = "admin", security(("bearer" = [])), request_body = ThirdDataForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ThirdDataForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = third_data_service::upsert(
        &state,
        &user,
        UpsertIn {
            id: f.id,
            name: &f.name,
            url: &f.url,
            api_url: &f.api_url,
            provider: &f.provider,
            sort: f.sort,
            enabled: f.enabled,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/third-data/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    third_data_service::delete(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}
