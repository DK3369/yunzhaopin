//! Email / SMS send logs (PHP `emaillog` / `messagelog`) + login / admin logs.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::admin_msg::repo::{AdminLogRow, LoginLogRow};
use phpyun_models::email_msg::entity::EmailMsg;
use phpyun_models::moblie_msg::entity::MoblieMsg;
use phpyun_services::admin_tool_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/email-logs", post(list_email))
        .route("/sms-logs", post(list_sms))
        .route("/login-logs", post(list_login))
        .route("/admin-logs", post(list_admin))
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

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginLogQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub usertype: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub uid: Option<u64>,
}

/// PHP `admin_loginlog` / `phpyun_login_log`.
#[utoipa::path(post, path = "/v1/admin/login-logs", tag = "admin", security(("bearer" = [])), request_body = LoginLogQuery, responses((status = 200, description = "ok")))]
pub async fn list_login(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<LoginLogQuery>,
) -> AppResult<ApiResponse<Paged<LoginLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_tool_service::list_login_logs(&state, q.usertype, q.uid, page).await?,
    ))
}

/// PHP `logrecord` / `phpyun_admin_log`.
#[utoipa::path(post, path = "/v1/admin/admin-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_admin(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<AdminLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_tool_service::list_admin_logs(&state, page).await?,
    ))
}
