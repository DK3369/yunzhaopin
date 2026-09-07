//! Admin country CRUD. Mutations invalidate the in-process country cache.
//!
//! - `POST /v1/admin/countries/list`        live rows (including hidden)
//! - `POST /v1/admin/countries/select`      pick which ones the public site shows
//! - `POST /v1/admin/countries`             create (code + 中文名 is enough)
//! - `POST /v1/admin/countries/patch`       patch
//! - `POST /v1/admin/countries/delete`      soft-delete
//! - `POST /v1/admin/countries/reload`      manual cache reload

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{BatchResult, CreatedId, IdBody};
use phpyun_core::{
    clock, ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson,
};
use phpyun_models::country::entity::Country;
use phpyun_models::country::repo as country_repo;
use phpyun_services::country_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/countries/list", post(list))
        .route("/countries/select", post(select))
        .route("/countries", post(create))
        .route("/countries/patch", post(patch))
        .route("/countries/delete", post(delete))
        .route("/countries/reload", post(reload))
}

#[utoipa::path(
    post,
    path = "/v1/admin/countries/list",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<Country>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(country_service::list_admin(&state).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SelectForm {
    #[validate(length(min = 1, max = 500))]
    pub ids: Vec<u64>,
}

#[utoipa::path(
    post,
    path = "/v1/admin/countries/select",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SelectForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn select(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SelectForm>,
) -> AppResult<ApiResponse<BatchResult>> {
    user.require_admin()?;
    let requested = f.ids.len();
    let affected = country_service::select_enabled(&state, &f.ids).await?;
    Ok(ApiResponse::data(BatchResult {
        requested,
        affected,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    /// ISO 3166-1 alpha-2 (CN/US/JP/...).
    #[validate(length(equal = 2))]
    pub code: String,
    #[serde(default)]
    pub code3: Option<String>,
    #[serde(default)]
    pub numeric_code: Option<u16>,
    #[serde(default)]
    pub name_en: Option<String>,
    #[validate(length(min = 1, max = 120))]
    pub name_zh: String,
    #[serde(default)]
    pub continent: Option<String>,
    #[serde(default)]
    pub phone_code: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub flag: Option<String>,
    #[serde(default)]
    pub sort: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/admin/countries",
    tag = "admin",
    security(("bearer" = [])),
    request_body = CreateForm,
    responses(
        (status = 200, description = "Created", body = CreatedId),
        (status = 403, description = "Admin required"),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = country_service::admin_create(
        &state,
        country_service::AdminCountryCreate {
            code: f.code,
            code3: f.code3,
            numeric_code: f.numeric_code,
            name_en: f.name_en,
            name_zh: f.name_zh,
            continent: f.continent,
            phone_code: f.phone_code,
            currency: f.currency,
            flag: f.flag,
            sort: f.sort,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    pub name_en: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    pub name_zh: Option<String>,
    #[serde(default)]
    #[validate(length(equal = 2))]
    pub continent: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 8))]
    pub phone_code: Option<String>,
    #[serde(default)]
    #[validate(length(equal = 3))]
    pub currency: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 8))]
    pub flag: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i32>,
}

#[utoipa::path(post,
    path = "/v1/admin/countries/patch",
    tag = "admin",
    security(("bearer" = [])),
    request_body = PatchForm,
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Admin required"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn patch(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PatchForm>,
) -> AppResult<ApiResponse> {
    let id = f.id;
    user.require_admin()?;
    let continent = f.continent.as_deref().map(str::to_uppercase);
    let currency = f.currency.as_deref().map(str::to_uppercase);
    let affected = country_repo::update(
        state.db.pool(),
        id,
        country_repo::CountryPatch {
            name_en: f.name_en.as_deref(),
            name_zh: f.name_zh.as_deref(),
            continent: continent.as_deref(),
            phone_code: f.phone_code.as_deref(),
            currency: currency.as_deref(),
            flag: f.flag.as_deref(),
            sort: f.sort,
            status: f.status,
        },
        clock::now_ts(),
    )
    .await
    .map_err(ApiError::internal)?;
    if affected == 0 {
        return Err(ApiError::param_invalid("country_not_found"));
    }
    country_service::invalidate().await;
    Ok(ApiResponse::message("updated"))
}

/// Soft-delete (`status = 2`).
#[utoipa::path(post,
    path = "/v1/admin/countries/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "Deleted"),
        (status = 403, description = "Admin required"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    let id = b.id;
    user.require_admin()?;
    let affected = country_repo::soft_delete(state.db.pool(), id, clock::now_ts())
        .await
        .map_err(ApiError::internal)?;
    if affected == 0 {
        return Err(ApiError::param_invalid("country_not_found"));
    }
    country_service::invalidate().await;
    Ok(ApiResponse::message("deleted"))
}

/// Force a cache reload from DB. Useful after a manual SQL bulk import.
#[utoipa::path(
    post,
    path = "/v1/admin/countries/reload",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "Reloaded"))
)]
pub async fn reload(
    State(_state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    country_service::invalidate().await;
    Ok(ApiResponse::message("reloaded"))
}
