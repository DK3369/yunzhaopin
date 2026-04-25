//! Member center aggregate (matching PHPYun `ajax::msgNum` composite counts).

use axum::{extract::State, routing::get, Router};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser};
use phpyun_services::dashboard_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(counts))
        .route("/com-dashboard", get(com_counts))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardView {
    pub unread_messages: u64,
    pub unread_chats: u64,
    pub apply_count: u64,
    pub interview_count: u64,
    pub favorite_count: u64,
    pub view_count: u64,
    pub integral_balance: i32,
    pub signday: u32,
}

/// Member center — first-screen aggregate counts
#[utoipa::path(
    get,
    path = "/v1/mcenter/dashboard",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = DashboardView))
)]
pub async fn counts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<DashboardView>> {
    let d = dashboard_service::counts(&state, &user).await?;
    Ok(ApiJson(DashboardView {
        unread_messages: d.unread_messages,
        unread_chats: d.unread_chats,
        apply_count: d.apply_count,
        interview_count: d.interview_count,
        favorite_count: d.favorite_count,
        view_count: d.view_count,
        integral_balance: d.integral_balance,
        signday: d.signday,
    }))
}

// ==================== Company-side dashboard ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ComDashboardView {
    pub applies_received: u64,
    pub applies_unread: u64,
    pub interviews_sent: u64,
    pub resume_downloads: u64,
    pub unread_chats: u64,
    pub unread_messages: u64,
    pub integral_balance: i32,
}

/// Company center — first-screen aggregate counts (matching PHPYun `member/com/tongji`).
#[utoipa::path(
    get,
    path = "/v1/mcenter/com-dashboard",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok", body = ComDashboardView),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn com_counts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<ComDashboardView>> {
    let d = dashboard_service::com_counts(&state, &user).await?;
    Ok(ApiJson(ComDashboardView {
        applies_received: d.applies_received,
        applies_unread: d.applies_unread,
        interviews_sent: d.interviews_sent,
        resume_downloads: d.resume_downloads,
        unread_chats: d.unread_chats,
        unread_messages: d.unread_messages,
        integral_balance: d.integral_balance,
    }))
}
