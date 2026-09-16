//! Member center aggregate (matching PHPYun `ajax::msgNum` composite counts).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_services::{dashboard_service, sign_service};
use serde::Serialize;
use utoipa::ToSchema;

use super::jobs::{job_counts_view, JobCountsView};
use super::messages::{load_unread_summary, UnreadSummary};
use super::resume_score::Completion;
use super::sign::StatusResp;

#[allow(deprecated)]
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", post(counts))
        .route("/dashboard/full", post(dashboard_full))
        .route("/com-dashboard", post(com_counts))
        .route("/com-dashboard/full", post(com_dashboard_full))
        .route("/dashboard/year-report", post(year_report))
        .route("/com-stats/today", post(today))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardView {
    pub unread_messages: u64,
    pub unread_chats: u64,
    pub apply_count: u64,
    pub interview_count: u64,
    pub favorite_count: u64,
    pub view_count: u64,
    pub integral_balance: i64,
    pub signday: u32,
    pub wkyqnum: u64,
    pub commsgnum: u64,
    pub sxnum: u64,
    pub sysnum: u64,
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
) -> AppResult<ApiResponse<DashboardView>> {
    let d = dashboard_service::counts(&state, &user).await?;
    Ok(ApiResponse::data(DashboardView {
        unread_messages: d.unread_messages,
        unread_chats: d.unread_chats,
        apply_count: d.apply_count,
        interview_count: d.interview_count,
        favorite_count: d.favorite_count,
        view_count: d.view_count,
        integral_balance: d.integral_balance,
        signday: d.signday,
        wkyqnum: d.wkyqnum,
        commsgnum: d.commsgnum,
        sxnum: d.sxnum,
        sysnum: d.sysnum,
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
    pub job_msg_unanswered: u64,
    pub integral_balance: i64,
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
) -> AppResult<ApiResponse<ComDashboardView>> {
    let d = dashboard_service::com_counts(&state, &user).await?;
    Ok(ApiResponse::data(ComDashboardView {
        applies_received: d.applies_received,
        applies_unread: d.applies_unread,
        interviews_sent: d.interviews_sent,
        resume_downloads: d.resume_downloads,
        unread_chats: d.unread_chats,
        unread_messages: d.unread_messages,
        job_msg_unanswered: d.job_msg_unanswered,
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
#[deprecated(note = "use /v1/mcenter/com-dashboard/full")]
#[utoipa::path(
    post,
    path = "/v1/mcenter/dashboard/year-report",
    tag = "mcenter",
    security(("bearer" = [])),
    description = "即将失效：请改用 POST /v1/mcenter/com-dashboard/full",
    responses(
        (status = 200, description = "ok", body = YearReportView),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn year_report(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<YearReportView>> {
    let d = dashboard_service::year_report(&state, &user).await?;
    Ok(ApiResponse::data(YearReportView {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct DayMetricView {
    pub num: u64,
    pub jzr: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ComTodayView {
    pub look_resume: DayMetricView,
    pub look_job: DayMetricView,
    pub down_resume: DayMetricView,
    pub apply: DayMetricView,
    pub invite: DayMetricView,
}

fn metric(m: dashboard_service::DayMetric) -> DayMetricView {
    DayMetricView { num: m.num, jzr: m.jzr }
}

/// PHP `zhaopin::getTodayData` — 今日五项及较昨日。
#[deprecated(note = "use /v1/mcenter/com-dashboard/full")]
#[utoipa::path(
    post,
    path = "/v1/mcenter/com-stats/today",
    tag = "mcenter",
    security(("bearer" = [])),
    description = "即将失效：请改用 POST /v1/mcenter/com-dashboard/full",
    responses((status = 200, description = "ok", body = ComTodayView))
)]
pub async fn today(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ComTodayView>> {
    let d = dashboard_service::com_today(&state, &user).await?;
    Ok(ApiResponse::data(today_view(d)))
}

fn dashboard_view(d: dashboard_service::DashboardCounts) -> DashboardView {
    DashboardView {
        unread_messages: d.unread_messages,
        unread_chats: d.unread_chats,
        apply_count: d.apply_count,
        interview_count: d.interview_count,
        favorite_count: d.favorite_count,
        view_count: d.view_count,
        integral_balance: d.integral_balance,
        signday: d.signday,
        wkyqnum: d.wkyqnum,
        commsgnum: d.commsgnum,
        sxnum: d.sxnum,
        sysnum: d.sysnum,
    }
}

fn com_dashboard_view(d: dashboard_service::ComDashboardCounts) -> ComDashboardView {
    ComDashboardView {
        applies_received: d.applies_received,
        applies_unread: d.applies_unread,
        interviews_sent: d.interviews_sent,
        resume_downloads: d.resume_downloads,
        unread_chats: d.unread_chats,
        unread_messages: d.unread_messages,
        job_msg_unanswered: d.job_msg_unanswered,
        integral_balance: d.integral_balance,
    }
}

fn year_report_view(d: dashboard_service::YearReport) -> YearReportView {
    YearReportView {
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
    }
}

fn today_view(d: dashboard_service::ComTodayStats) -> ComTodayView {
    ComTodayView {
        look_resume: metric(d.look_resume),
        look_job: metric(d.look_job),
        down_resume: metric(d.down_resume),
        apply: metric(d.apply),
        invite: metric(d.invite),
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardFull {
    pub unread_messages: u64,
    pub unread_chats: u64,
    pub apply_count: u64,
    pub interview_count: u64,
    pub favorite_count: u64,
    pub view_count: u64,
    pub integral_balance: i64,
    pub signday: u32,
    pub wkyqnum: u64,
    pub commsgnum: u64,
    pub sxnum: u64,
    pub sysnum: u64,
    pub sign: StatusResp,
    pub completion: Completion,
    pub unread: UnreadSummary,
}

/// Jobseeker first-screen bundle: dashboard counts + sign + resume completion + unread.
#[utoipa::path(
    post,
    path = "/v1/mcenter/dashboard/full",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = DashboardFull))
)]
pub async fn dashboard_full(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<DashboardFull>> {
    user.require_jobseeker()?;
    let (d, sign, completion, unread) = tokio::join!(
        dashboard_service::counts(&state, &user),
        sign_service::status(&state, &user),
        phpyun_services::resume_score_service::compute(&state, &user),
        load_unread_summary(&state, &user),
    );
    let d = dashboard_view(d?);
    let (us, signed) = sign?;
    let r = completion?;
    Ok(ApiResponse::data(DashboardFull {
        unread_messages: d.unread_messages,
        unread_chats: d.unread_chats,
        apply_count: d.apply_count,
        interview_count: d.interview_count,
        favorite_count: d.favorite_count,
        view_count: d.view_count,
        integral_balance: d.integral_balance,
        signday: d.signday,
        wkyqnum: d.wkyqnum,
        commsgnum: d.commsgnum,
        sxnum: d.sxnum,
        sysnum: d.sysnum,
        sign: StatusResp {
            signday: us.signday,
            signdays: us.signdays,
            last_date_ymd: us.last_date_ymd,
            signed_today: signed,
        },
        completion: Completion {
            score: r.score,
            missing: r.missing.into_iter().map(|s| s.to_string()).collect(),
        },
        unread,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ComDashboardFull {
    pub applies_received: u64,
    pub applies_unread: u64,
    pub interviews_sent: u64,
    pub resume_downloads: u64,
    pub unread_chats: u64,
    pub unread_messages: u64,
    pub job_msg_unanswered: u64,
    pub integral_balance: i64,
    pub today: ComTodayView,
    pub year_report: YearReportView,
    pub job_counts: JobCountsView,
}

/// Employer first-screen bundle: dashboard + today + year report + job tab counts.
#[utoipa::path(
    post,
    path = "/v1/mcenter/com-dashboard/full",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok", body = ComDashboardFull),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn com_dashboard_full(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ComDashboardFull>> {
    user.require_employer()?;
    let (d, today, year, counts) = tokio::join!(
        dashboard_service::com_counts(&state, &user),
        dashboard_service::com_today(&state, &user),
        dashboard_service::year_report(&state, &user),
        phpyun_services::job_mgmt_service::counts_by_state(&state, &user),
    );
    let d = com_dashboard_view(d?);
    Ok(ApiResponse::data(ComDashboardFull {
        applies_received: d.applies_received,
        applies_unread: d.applies_unread,
        interviews_sent: d.interviews_sent,
        resume_downloads: d.resume_downloads,
        unread_chats: d.unread_chats,
        unread_messages: d.unread_messages,
        job_msg_unanswered: d.job_msg_unanswered,
        integral_balance: d.integral_balance,
        today: today_view(today?),
        year_report: year_report_view(year?),
        job_counts: job_counts_view(counts?),
    }))
}
