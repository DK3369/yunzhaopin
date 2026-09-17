//! POST /v1/mcenter/company/nav · /save · /reset

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::company_nav_service::{self, NavItem, NavPack};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company/nav", post(get))
        .route("/company/nav/save", post(save))
        .route("/company/nav/reset", post(reset))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/nav",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = NavPackView))
)]
pub async fn get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<NavPackView>> {
    let p = company_nav_service::get(&state, &user).await?;
    Ok(ApiResponse::data(NavPackView::from(p)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SaveForm {
    pub items: Vec<NavItemForm>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NavItemForm {
    #[validate(length(min = 1, max = 32))]
    pub key: String,
    #[serde(default)]
    #[validate(length(max = 64))]
    pub label_key: String,
    #[validate(length(min = 1, max = 64))]
    pub to: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub show: bool,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub target: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct NavItemView {
    pub key: String,
    pub label_key: String,
    pub to: String,
    pub sort: i32,
    pub show: bool,
    pub target: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct NavPackView {
    pub is_nav: i32,
    pub items: Vec<NavItemView>,
}

impl From<NavPack> for NavPackView {
    fn from(p: NavPack) -> Self {
        Self {
            is_nav: p.is_nav,
            items: p.items.into_iter().map(NavItemView::from).collect(),
        }
    }
}

impl From<NavItem> for NavItemView {
    fn from(i: NavItem) -> Self {
        Self {
            key: i.key,
            label_key: i.label_key,
            to: i.to,
            sort: i.sort,
            show: i.show,
            target: i.target,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/nav/save",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SaveForm,
    responses((status = 200, description = "ok"))
)]
pub async fn save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SaveForm>,
) -> AppResult<ApiResponse> {
    let items: Vec<NavItem> = f
        .items
        .into_iter()
        .map(|i| NavItem {
            key: i.key,
            label_key: i.label_key,
            to: i.to,
            sort: i.sort,
            show: i.show,
            target: i.target,
        })
        .collect();
    company_nav_service::save(&state, &user, &items).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/nav/reset",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn reset(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse> {
    company_nav_service::reset(&state, &user).await?;
    Ok(ApiResponse::message("ok"))
}
