//! Violation reports.

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::report_service::{self, ReportInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reports", post(submit))
        .route("/reports/list", post(list_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReportForm {
    /// 1=job / 2=company / 3=resume / 4=article / 5=user
    #[validate(range(min = 1, max = 5))]
    pub target_kind: i32,
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_id: u64,
    #[validate(length(min = 1, max = 32))]
    pub reason_code: String,
    #[validate(length(max = 2000))]
    pub detail: Option<String>,
}

/// Submit a report
#[utoipa::path(
    post,
    path = "/v1/mcenter/reports/list",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReportForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ReportForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = report_service::submit(
        &state,
        &user,
        ReportInput {
            target_kind: f.target_kind,
            target_id: f.target_id,
            reason_code: &f.reason_code,
            detail: f.detail.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn report_kind_name(k: i32) -> &'static str {
    match k {
        1 => "job", 2 => "company", 3 => "resume", 4 => "article", 5 => "user",
        _ => "unknown",
    }
}

fn report_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending", 1 => "approved", 2 => "rejected", _ => "unknown",
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReportItem {
    pub id: u64,
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_kind_n: String,
    pub target_id: u64,
    pub reason_code: String,
    pub detail: Option<String>,
    pub status: i32,
    pub status_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::report::entity::Report> for ReportItem {
    fn from(r: phpyun_models::report::entity::Report) -> Self {
        Self {
            id: r.id,
            reporter_uid: r.reporter_uid,
            target_kind_n: report_kind_name(r.target_kind).to_string(),
            target_kind: r.target_kind,
            target_id: r.target_id,
            reason_code: r.reason_code,
            detail: r.detail,
            status_n: report_status_name(r.status).to_string(),
            status: r.status,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
        }
    }
}

/// Reports I have submitted
#[utoipa::path(
    post,
    path = "/v1/mcenter/reports",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ReportItem>>> {
    let r = report_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ReportItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
