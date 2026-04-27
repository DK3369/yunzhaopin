//! Admin dashboard aggregate.

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::admin_dashboard_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/overview", post(overview))
        .route("/dashboard/recent-signups", post(recent_signups))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewView {
    pub pending_company_certs: u64,
    pub pending_jobs: u64,
    pub pending_reports: u64,
    pub pending_feedback: u64,
    pub total_users: u64,
    pub active_companies: u64,
    pub active_jobs: u64,
    pub active_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

/// Review queue + activity snapshot
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/overview",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = OverviewView))
)]
pub async fn overview(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<OverviewView>> {
    user.require_admin()?;
    let o = admin_dashboard_service::overview(&state, &user).await?;
    Ok(ApiJson(OverviewView {
        pending_company_certs: o.pending_company_certs,
        pending_jobs: o.pending_jobs,
        pending_reports: o.pending_reports,
        pending_feedback: o.pending_feedback,
        total_users: o.total_users,
        active_companies: o.active_companies,
        active_jobs: o.active_jobs,
        active_resumes: o.active_resumes,
        today_new_jobs: o.today_new_jobs,
        today_new_resumes: o.today_new_resumes,
    }))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RecentQuery {
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_limit() -> u64 { 10 }


fn usertype_name(t: i32) -> &'static str {
    match t { 1 => "jobseeker", 2 => "company", 3 => "admin", _ => "unknown" }
}

fn user_status_name(s: i32) -> &'static str {
    match s { 0 => "pending", 1 => "active", 2 => "locked", 3 => "deleted", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecentUser {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: i32,
    pub usertype_n: String,
    pub status: i32,
    pub status_n: String,
    pub did: u64,
    pub reg_date: i64,
    pub reg_date_n: String,
    pub login_date: Option<i64>,
    pub login_date_n: String,
}

/// Recent signups
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/recent-signups",
    tag = "admin",
    security(("bearer" = [])),
    params(RecentQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn recent_signups(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<RecentQuery>,
) -> AppResult<ApiJson<Vec<RecentUser>>> {
    user.require_admin()?;
    let list = admin_dashboard_service::recent_signups(&state, &user, q.limit).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|m| RecentUser {
                uid: m.uid,
                username: m.username,
                email: m.email,
                moblie: m.moblie,
                usertype_n: usertype_name(m.usertype).to_string(),
                usertype: m.usertype,
                status_n: user_status_name(m.status).to_string(),
                status: m.status,
                did: m.did,
                reg_date_n: fmt_dt(m.reg_date),
                reg_date: m.reg_date,
                login_date_n: fmt_dt(m.login_date.unwrap_or(0)),
                login_date: m.login_date,
            })
            .collect(),
    ))
}
