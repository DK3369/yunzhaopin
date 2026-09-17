//! App version management (admin).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::app_version_service::{self, VersionInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/app-versions", post(create))
        .route("/app-versions/list", post(list))
        .route("/app-versions/update", post(update))
        .route("/app-versions/delete", post(remove))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 100))]
    pub platform: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionItem {
    pub id: u64,
    pub platform: String,
    pub version: String,
    pub version_code: u32,
    pub is_force: i32,
    pub download_url: String,
    pub changelog: String,
    pub status: i32,
    pub released_at: i64,
    pub released_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::app_version::entity::AppVersion> for VersionItem {
    fn from(v: phpyun_models::app_version::entity::AppVersion) -> Self {
        Self {
            id: v.id,
            platform: v.platform,
            version: v.version,
            version_code: v.version_code,
            is_force: v.is_force,
            download_url: v.download_url,
            changelog: v.changelog,
            status: v.status,
            released_at_n: fmt_dt(v.released_at),
            released_at: v.released_at,
            created_at_n: fmt_dt(v.created_at),
            created_at: v.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    #[validate(length(min = 1, max = 16))]
    pub platform: String,
    #[validate(length(min = 1, max = 32))]
    pub version: String,
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_u32")]
    #[validate(range(min = 0, max = 2_147_483_647))]
    pub version_code: u32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub is_force: bool,
    #[validate(length(max = 255), custom(function = "phpyun_core::validators::http_or_site_url"))]
    #[serde(default)]
    pub download_url: String,
    #[validate(length(max = 5000))]
    #[serde(default)]
    pub changelog: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i64")]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub released_at: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateForm {
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_u64")]
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 16))]
    pub platform: String,
    #[validate(length(min = 1, max = 32))]
    pub version: String,
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_u32")]
    #[validate(range(min = 0, max = 2_147_483_647))]
    pub version_code: u32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub is_force: bool,
    #[validate(length(max = 255), custom(function = "phpyun_core::validators::http_or_site_url"))]
    #[serde(default)]
    pub download_url: String,
    #[validate(length(max = 5000))]
    #[serde(default)]
    pub changelog: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i64")]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub released_at: i64,
}

fn released_or_now(ts: i64) -> i64 {
    if ts > 0 {
        ts
    } else {
        phpyun_core::clock::now_ts()
    }
}

#[utoipa::path(post, path = "/v1/admin/app-versions/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<VersionItem>>> {
    user.require_admin()?;
    let r = app_version_service::admin_list(&state, &user, q.platform.as_deref(), page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(post, path = "/v1/admin/app-versions", tag = "admin", security(("bearer" = [])), request_body = CreateForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = app_version_service::admin_create(
        &state,
        &user,
        VersionInput {
            platform: &f.platform,
            version: &f.version,
            version_code: f.version_code,
            is_force: f.is_force,
            download_url: &f.download_url,
            changelog: &f.changelog,
            released_at: released_or_now(f.released_at),
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/app-versions/update", tag = "admin", security(("bearer" = [])), request_body = UpdateForm, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UpdateForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    app_version_service::admin_update(
        &state,
        &user,
        f.id,
        VersionInput {
            platform: &f.platform,
            version: &f.version,
            version_code: f.version_code,
            is_force: f.is_force,
            download_url: &f.download_url,
            changelog: &f.changelog,
            released_at: released_or_now(f.released_at),
        },
    )
    .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/app-versions/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    app_version_service::admin_delete(&state, &user, b.id).await?;
    Ok(ApiResponse::message("deleted"))
}
