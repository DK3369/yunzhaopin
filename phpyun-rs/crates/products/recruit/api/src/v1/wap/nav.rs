//! Navigation menu public read.

use axum::{extract::State, routing::get, Router};
use phpyun_core::utils::{fmt_dt, pic_n_str as icon_n};
use phpyun_core::{ApiResponse, AppResult, AppState, ValidatedJsonOrQuery};
use phpyun_services::nav_menu_service;
use serde::Serialize;
use utoipa::ToSchema;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/nav"];

pub fn routes() -> Router<AppState> {
    Router::new().route("/nav", get(list).post(list))
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
#[utoipa::path(post,
    path = "/v1/wap/nav",
    tag = "wap",
    request_body = ListBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<ListBody>,
) -> AppResult<ApiResponse<Vec<NavItem>>> {
    let position = b.position;
    phpyun_core::validators::ensure_path_token(&position)?;
    let list = nav_menu_service::list(&state, &position).await?;
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|n| NavItem::from_with_ctx(n, &state))
            .collect(),
    ))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct ListBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub position: String,
}
