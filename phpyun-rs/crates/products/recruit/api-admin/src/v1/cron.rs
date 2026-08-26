//! Cron inventory (PHP `set_cron`). Scheduler lives in the server binary; this is a read-only list.

use axum::{routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_services::admin_tool_service::{self, CronJobItem};

pub fn routes() -> Router<AppState> {
    Router::new().route("/cron", post(list))
}

#[utoipa::path(
    post,
    path = "/v1/admin/cron",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(user: AuthenticatedUser) -> AppResult<ApiResponse<Vec<CronJobItem>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_tool_service::list_cron_jobs()))
}
