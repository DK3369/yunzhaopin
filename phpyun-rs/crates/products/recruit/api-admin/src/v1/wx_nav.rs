//! WeChat custom menu rows (`phpyun_wxnav` / PHP `weixinmenu`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_models::wx_nav::entity::WxNav;
use phpyun_services::admin_tool_service;

pub fn routes() -> Router<AppState> {
    Router::new().route("/wx-navs", post(list))
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
