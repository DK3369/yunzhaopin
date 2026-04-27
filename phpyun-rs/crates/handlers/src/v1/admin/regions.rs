//! Admin region CRUD. Mutations trigger a cluster-wide cache reload via Redis pubsub.
//!
//! - `POST /v1/admin/regions`         create
//! - `POST /v1/admin/regions/{id}`    patch
//! - `POST /v1/admin/regions/{id}/delete`  soft-delete
//! - `POST /v1/admin/regions/reload`  manual cache reload

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{clock, ApiJson, ApiOk, AppError, AppResult, AppState, AuthenticatedUser, InfraError, ValidatedJson};
use phpyun_models::region::repo as region_repo;
use phpyun_services::region_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/regions", post(create))
        .route("/regions/patch", post(patch))
        .route("/regions/delete", post(delete))
        .route("/regions/reload", post(reload))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    /// `NULL` for country-level rows; otherwise a parent region's id.
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
    /// ISO 3166-1 alpha-2 (CN/US/...). For non-country rows this should match the country chain.
    #[validate(length(equal = 2))]
    pub country_code: String,
    /// Stable identifier. ISO 3166-2 form: `CN-BJ` / `US-CA`. Lower levels add a custom suffix: `CN-BJ-CY`.
    #[validate(length(min = 2, max = 20))]
    pub code: String,
    /// 0=country, 1=state/province, 2=city, 3=district.
    #[validate(range(min = 0, max = 3))]
    pub level: i32,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    /// Optional `AF/AN/AS/EU/NA/OC/SA` (country-level only).
    #[serde(default)]
    #[validate(length(max = 500))]
    pub continent: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

#[utoipa::path(
    post,
    path = "/v1/admin/regions",
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
    let id = region_repo::create(
        state.db.pool(),
        region_repo::RegionCreate {
            parent_id: f.parent_id,
            country_code: &f.country_code.to_uppercase(),
            code: &f.code,
            level: f.level,
            name: &f.name,
            continent: f.continent.as_deref(),
            sort: f.sort,
        },
        clock::now_ts(),
    )
    .await
    .map_err(AppError::internal)?;
    region_service::reload(&state).await?;
    Ok(ApiJson(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub continent: Option<String>,
}

#[utoipa::path(post,
    path = "/v1/admin/regions",
    tag = "admin",
    security(("bearer" = [])),
    request_body = PatchForm,
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Admin required"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn patch(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PatchForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    let affected = region_repo::update(
        state.db.pool(),
        id,
        region_repo::RegionPatch {
            name: f.name.as_deref(),
            sort: f.sort,
            continent: f.continent.as_deref(),
        },
        clock::now_ts(),
    )
    .await
    .map_err(AppError::internal)?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "region_not_found".into(),
        )));
    }
    region_service::reload(&state).await?;
    Ok(ApiOk("updated"))
}

/// Soft-delete (`status=2`). Children are not auto-cascaded — the cache filters
/// by `status != 2`, so descendants remain visible until the admin explicitly
/// deletes them.
#[utoipa::path(post,
    path = "/v1/admin/regions/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "Deleted"),
        (status = 403, description = "Admin required"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn delete(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    let affected = region_repo::soft_delete(state.db.pool(), id, clock::now_ts())
        .await
        .map_err(AppError::internal)?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "region_not_found".into(),
        )));
    }
    region_service::reload(&state).await?;
    Ok(ApiOk("deleted"))
}

/// Force a cluster-wide cache reload from DB (rarely needed; useful after a manual SQL bulk import).
#[utoipa::path(
    post,
    path = "/v1/admin/regions/reload",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "Reloaded"))
)]
pub async fn reload(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    region_service::reload(&state).await?;
    Ok(ApiOk("reloaded"))
}

