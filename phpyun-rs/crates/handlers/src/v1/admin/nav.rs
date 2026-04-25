//! Navigation management (admin).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::nav_menu_service::{self, NavInput, NavPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/nav", get(list).post(create))
        .route("/nav/{id}", post(update))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub position: Option<String>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn icon_n(state: &AppState, raw: &str) -> String {
    state.storage.normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NavItem {
    pub id: u64,
    pub position: String,
    pub label: String,
    pub url: String,
    pub icon: String,
    pub icon_n: String,
    pub parent_id: u64,
    pub sort: i32,
    pub status: i32,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl NavItem {
    pub fn from_with_ctx(n: phpyun_models::nav_menu::entity::NavMenu, state: &AppState) -> Self {
        Self {
            icon_n: icon_n(state, &n.icon),
            id: n.id,
            position: n.position,
            label: n.label,
            url: n.url,
            icon: n.icon,
            parent_id: n.parent_id,
            sort: n.sort,
            status: n.status,
            updated_at_n: fmt_dt(n.updated_at),
            updated_at: n.updated_at,
        }
    }
}

impl From<phpyun_models::nav_menu::entity::NavMenu> for NavItem {
    fn from(n: phpyun_models::nav_menu::entity::NavMenu) -> Self {
        Self {
            id: n.id,
            position: n.position,
            label: n.label,
            url: n.url,
            icon: n.icon.clone(),
            icon_n: n.icon,
            parent_id: n.parent_id,
            sort: n.sort,
            status: n.status,
            updated_at_n: fmt_dt(n.updated_at),
            updated_at: n.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NavForm {
    #[validate(length(min = 1, max = 32))]
    pub position: String,
    #[validate(length(min = 1, max = 120))]
    pub label: String,
    #[validate(length(min = 1, max = 500))]
    pub url: String,
    #[validate(length(max = 120))]
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub parent_id: u64,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NavPatchForm {
    #[validate(length(min = 1, max = 120))]
    pub label: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub url: Option<String>,
    #[validate(length(max = 120))]
    pub icon: Option<String>,
    pub parent_id: Option<u64>,
    pub sort: Option<i32>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(get, path = "/v1/admin/nav", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Vec<NavItem>>> {
    let list = nav_menu_service::admin_list(&state, &user, q.position.as_deref()).await?;
    Ok(ApiJson(list.into_iter().map(NavItem::from).collect()))
}

#[utoipa::path(post, path = "/v1/admin/nav", tag = "admin", security(("bearer" = [])), request_body = NavForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NavForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = nav_menu_service::admin_create(
        &state,
        &user,
        NavInput {
            position: &f.position,
            label: &f.label,
            url: &f.url,
            icon: &f.icon,
            parent_id: f.parent_id,
            sort: f.sort,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete a navigation entry (sending `"status":2` deletes it)
#[utoipa::path(post, path = "/v1/admin/nav/{id}", tag = "admin", security(("bearer" = [])), params(("id" = u64, Path)), request_body = NavPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<NavPatchForm>,
) -> AppResult<ApiOk> {
    if f.status == Some(2) {
        nav_menu_service::admin_delete(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    nav_menu_service::admin_update(
        &state,
        &user,
        id,
        NavPatch {
            label: f.label.as_deref(),
            url: f.url.as_deref(),
            icon: f.icon.as_deref(),
            parent_id: f.parent_id,
            sort: f.sort,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}
