//! Admin dashboard aggregation: pending counts for each review queue plus the last 24h of registrations/applications/postings overview.

use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_cert::repo as cert_repo;
use phpyun_models::feedback::repo as feedback_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::report::repo as report_repo;
use phpyun_models::stats::repo as stats_repo;
use phpyun_models::user::entity::Member;

#[derive(Debug, Default)]
pub struct AdminOverview {
    pub pending_company_certs: u64,
    pub pending_jobs: u64,
    pub pending_reports: u64,
    pub pending_feedback: u64,
    pub total_users: u64, // best-effort from admin_count
    pub active_companies: u64,
    pub active_jobs: u64,
    pub active_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

fn today_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

pub async fn overview(
    state: &AppState,
    admin: &AuthenticatedUser,
) -> AppResult<AdminOverview> {
    admin.require_admin()?;
    let db = state.db.reader();
    let today = today_ts(clock::now_ts());

    // Parallel all counts
    let (certs, jobs_pending, reports_pending, fb_pending, active_jobs, active_coms, active_res, new_j, new_r) = tokio::join!(
        cert_repo::count_pending(db),
        job_repo::admin_count(db, Some(0)),
        report_repo::count_by_status(db, Some(0)),
        feedback_repo::count_by_status(db, Some(0)),
        stats_repo::count_active_jobs(db),
        stats_repo::count_active_companies(db),
        stats_repo::count_active_resumes(db),
        stats_repo::count_jobs_since(db, today),
        stats_repo::count_resumes_since(db, today),
    );

    // total_users: approximate via admin_list_count (no filter)
    let total_users = phpyun_models::user::repo::admin_count(
        db,
        &phpyun_models::user::repo::AdminUserFilter {
            keyword: None,
            usertype: None,
            status: None,
        },
    )
    .await
    .unwrap_or(0);

    Ok(AdminOverview {
        pending_company_certs: certs.unwrap_or(0),
        pending_jobs: jobs_pending.unwrap_or(0),
        pending_reports: reports_pending.unwrap_or(0),
        pending_feedback: fb_pending.unwrap_or(0),
        total_users,
        active_companies: active_coms.unwrap_or(0),
        active_jobs: active_jobs.unwrap_or(0),
        active_resumes: active_res.unwrap_or(0),
        today_new_jobs: new_j.unwrap_or(0),
        today_new_resumes: new_r.unwrap_or(0),
    })
}

/// Most recent signups (top N)
pub async fn recent_signups(
    state: &AppState,
    admin: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<Member>> {
    admin.require_admin()?;
    let limit = limit.clamp(1, 50);
    Ok(phpyun_models::user::repo::admin_list(
        state.db.reader(),
        &phpyun_models::user::repo::AdminUserFilter {
            keyword: None,
            usertype: None,
            status: None,
        },
        0,
        limit,
    )
    .await?)
}
