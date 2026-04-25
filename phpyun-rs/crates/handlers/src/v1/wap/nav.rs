//! Navigation menu public read.

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::nav_menu_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/nav/{position}", get(list))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn icon_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

/// Navigation item — all 9 columns of phpyun_navigation + CDN URL + formatted timestamp.
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

/// Get navigation for the specified position (header/footer/sidebar/mobile)
#[utoipa::path(
    get,
    path = "/v1/wap/nav/{position}",
    tag = "wap",
    params(("position" = String, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    Path(position): Path<String>,
) -> AppResult<ApiJson<Vec<NavItem>>> {
    let list = nav_menu_service::list(&state, &position).await?;
    Ok(ApiJson(list.into_iter().map(NavItem::from).collect()))
}
