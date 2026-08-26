//! Email / SMS send logs (PHP `emaillog` / `messagelog`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::email_msg::entity::EmailMsg;
use phpyun_models::moblie_msg::entity::MoblieMsg;
use phpyun_services::admin_tool_service;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/email-logs", post(list_email))
        .route("/sms-logs", post(list_sms))
}

#[utoipa::path(
    post,
    path = "/v1/admin/email-logs",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_email(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<EmailMsg>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_tool_service::list_email_logs(&state, page).await?,
    ))
}

#[utoipa::path(
    post,
    path = "/v1/admin/sms-logs",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_sms(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<MoblieMsg>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_tool_service::list_sms_logs(&state, page).await?,
    ))
}
