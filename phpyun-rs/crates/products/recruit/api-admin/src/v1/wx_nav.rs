//! WeChat custom menu rows (`phpyun_wxnav` / PHP `weixinmenu`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_models::wx_nav::entity::WxNav;
use phpyun_services::admin_tool_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/wx-navs", post(list))
        .route("/wx-navs/upsert", post(upsert))
        .route("/wx-navs/delete", post(delete))
}

#[utoipa::path(
    post,
    path = "/v1/admin/wx-navs",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<WxNav>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_tool_service::list_wx_navs(&state).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WxNavForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    #[serde(default)]
    pub keyid: i32,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, rename = "type")]
    pub nav_type: String,
    #[serde(default)]
    pub sort: i32,
}

#[utoipa::path(post, path = "/v1/admin/wx-navs/upsert", tag = "admin", security(("bearer" = [])), request_body = WxNavForm, responses((status = 200, description = "ok")))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<WxNavForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_tool_service::upsert_wx_nav(
        &state,
        f.id,
        &f.name,
        f.keyid,
        &f.key,
        &f.url,
        &f.nav_type,
        f.sort,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/wx-navs/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_tool_service::delete_wx_nav(&state, f.id).await?;
    Ok(ApiResponse::message("ok"))
}
