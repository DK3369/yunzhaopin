//! Admin country CRUD. Mutations invalidate the in-process country cache.
//!
//! - `POST /v1/admin/countries`             create
//! - `POST /v1/admin/countries/{id}`        patch
//! - `POST /v1/admin/countries/{id}/delete` soft-delete
//! - `POST /v1/admin/countries/reload`      manual cache reload

use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use phpyun_core::{
    clock, ApiJson, ApiOk, AppError, AppResult, AppState, AuthenticatedUser, InfraError,
    ValidatedJson,
};
use phpyun_models::country::repo as country_repo;
use phpyun_services::country_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/countries", post(create))
        .route("/countries/{id}", post(patch))
        .route("/countries/{id}/delete", post(delete))
        .route("/countries/reload", post(reload))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    /// ISO 3166-1 alpha-2 (CN/US/JP/...).
    #[validate(length(equal = 2))]
    pub code: String,
    /// ISO 3166-1 alpha-3 (CHN/USA/JPN/...).
    #[validate(length(equal = 3))]
    pub code3: String,
    /// ISO 3166-1 numeric (156/840/...).
    pub numeric_code: u16,
    #[validate(length(min = 1, max = 120))]
    pub name_en: String,
    #[validate(length(min = 1, max = 120))]
    pub name_zh: String,
    /// `AF/AN/AS/EU/NA/OC/SA`.
    #[validate(length(equal = 2))]
    pub continent: String,
    /// International dialing prefix without `+` (e.g. `86`).
    #[validate(length(min = 1, max = 8))]
    pub phone_code: String,
    /// ISO 4217 (CNY/USD/...).
    #[validate(length(equal = 3))]
    pub currency: String,
    /// Unicode flag emoji (e.g. 🇨🇳).
    #[validate(length(min = 1, max = 8))]
    pub flag: String,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
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
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id = country_repo::create(
        state.db.pool(),
        country_repo::CountryCreate {
            code: &f.code.to_uppercase(),
            code3: &f.code3.to_uppercase(),
            numeric_code: f.numeric_code,
            name_en: &f.name_en,
            name_zh: &f.name_zh,
            continent: &f.continent.to_uppercase(),
            phone_code: &f.phone_code,
            currency: &f.currency.to_uppercase(),
            flag: &f.flag,
            sort: f.sort,
        },
        clock::now_ts(),
    )
    .await
    .map_err(AppError::internal)?;
    country_service::invalidate().await;
    Ok(ApiJson(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PatchForm {
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
    pub sort: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/admin/countries/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
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
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<PatchForm>,
) -> AppResult<ApiOk> {
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
        },
        clock::now_ts(),
    )
    .await
    .map_err(AppError::internal)?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "country_not_found".into(),
        )));
    }
    country_service::invalidate().await;
    Ok(ApiOk("updated"))
}

/// Soft-delete (`status = 2`).
#[utoipa::path(
    post,
    path = "/v1/admin/countries/{id}/delete",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "Deleted"),
        (status = 403, description = "Admin required"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    let affected = country_repo::soft_delete(state.db.pool(), id, clock::now_ts())
        .await
        .map_err(AppError::internal)?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "country_not_found".into(),
        )));
    }
    country_service::invalidate().await;
    Ok(ApiOk("deleted"))
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
) -> AppResult<ApiOk> {
    user.require_admin()?;
    country_service::invalidate().await;
    Ok(ApiOk("reloaded"))
}
