//! Member center aggregate (matching PHPYun `ajax::msgNum` composite counts).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser};
use phpyun_services::dashboard_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", post(counts))
        .route("/com-dashboard", post(com_counts))
        .route("/dashboard/year-report", post(year_report))
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
    post,
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
    post,
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

// ==================== Annual report ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct YearReportView {
    pub login_days: u32,
    pub job_count: u32,
    pub view_count: u32,
    pub received_resumes: u32,
    pub viewed_resumes: u32,
    pub invited_count: u32,
    pub night_work_count: u32,
    pub last_night_work_at: i64,
    pub company_name: String,
    pub linkman: String,
}

/// HR-side yearly report data — counterpart of PHP `wap/ajax::lastYearReport_action`.
/// PHP returns a rendered PNG poster; the Rust port returns just the
/// underlying numbers and lets the frontend assemble the artwork. Restricted
/// to employers (`usertype=2`).
#[utoipa::path(
    post,
    path = "/v1/mcenter/dashboard/year-report",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok", body = YearReportView),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn year_report(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<YearReportView>> {
    let d = dashboard_service::year_report(&state, &user).await?;
    Ok(ApiJson(YearReportView {
        login_days: d.login_days,
        job_count: d.job_count,
        view_count: d.view_count,
        received_resumes: d.received_resumes,
        viewed_resumes: d.viewed_resumes,
        invited_count: d.invited_count,
        night_work_count: d.night_work_count,
        last_night_work_at: d.last_night_work_at,
        company_name: d.company_name,
        linkman: d.linkman,
    }))
}
