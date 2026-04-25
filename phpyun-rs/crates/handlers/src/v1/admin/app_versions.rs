//! App version management (admin).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::app_version_service::{self, VersionInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/app-versions", get(list).post(create))
        .route("/app-versions/{id}", post(remove))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub platform: Option<String>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
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
    pub version_code: u32,
    #[serde(default)]
    pub is_force: bool,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub download_url: String,
    #[validate(length(max = 5000))]
    #[serde(default)]
    pub changelog: String,
    #[serde(default)]
    pub released_at: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(get, path = "/v1/admin/app-versions", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Paged<VersionItem>>> {
    let r = app_version_service::admin_list(&state, &user, q.platform.as_deref(), page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(VersionItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[utoipa::path(post, path = "/v1/admin/app-versions", tag = "admin", security(("bearer" = [])), request_body = CreateForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiJson<CreatedId>> {
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
            released_at: if f.released_at > 0 { f.released_at } else { phpyun_core::clock::now_ts() },
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(delete, path = "/v1/admin/app-versions/{id}", tag = "admin", security(("bearer" = [])), params(("id" = u64, Path)), responses((status = 200, description = "ok")))]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    app_version_service::admin_delete(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}
