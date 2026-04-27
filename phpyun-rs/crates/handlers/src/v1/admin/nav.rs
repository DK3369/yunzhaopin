//! Navigation management (admin).

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::nav_menu_service::{self, NavInput, NavPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/nav", post(create))
        .route("/nav/list", post(list))
        .route("/nav/update", post(update))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
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

// Reuse wap's `NavItem` (identical shape and `From<NavMenu>`); admin needs no
// extra fields here, just a different list filter on the service layer.
pub type NavItem = crate::v1::wap::nav::NavItem;

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
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: u64,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NavPatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 120))]
    pub label: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub url: Option<String>,
    #[validate(length(max = 120))]
    pub icon: Option<String>,
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/nav/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Vec<NavItem>>> {
    user.require_admin()?;
    let list = nav_menu_service::admin_list(&state, &user, q.position.as_deref()).await?;
    Ok(ApiJson(list.into_iter().map(NavItem::from).collect()))
}

#[utoipa::path(post, path = "/v1/admin/nav", tag = "admin", security(("bearer" = [])), request_body = NavForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NavForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
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
#[utoipa::path(post, path = "/v1/admin/nav", tag = "admin", security(("bearer" = [])), request_body = NavPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NavPatchForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
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
