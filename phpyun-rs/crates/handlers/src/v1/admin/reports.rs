//! Report queue (admin).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::{dto::StatusFilterBody, ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::admin_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reports", post(list))
        .route("/reports/status", post(set_status))
        .route("/reports/batch/status", post(batch_set_status))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn report_kind_name(k: i32) -> &'static str {
    match k { 1 => "job", 2 => "company", 3 => "resume", 4 => "article", 5 => "user", _ => "unknown" }
}

fn report_status_name(s: i32) -> &'static str {
    match s { 0 => "pending", 1 => "approved", 2 => "rejected", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminReportItem {
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

impl From<phpyun_models::report::entity::Report> for AdminReportItem {
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

/// Report queue
#[utoipa::path(
    post,
    path = "/v1/admin/reports",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
) -> AppResult<ApiJson<Paged<AdminReportItem>>> {
    user.require_admin()?;
    let r = admin_service::list_reports(&state, q.status, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AdminReportItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetReportStatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// 1=approved / 2=rejected
    #[validate(range(min = 1, max = 2))]
    pub status: i32,
}

/// Process a report
#[utoipa::path(post,
    path = "/v1/admin/reports/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetReportStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetReportStatusForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    admin_service::set_report_status(&state, &user, id, f.status).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchStatusForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(range(min = 1, max = 2))]
    pub status: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchResult {
    pub requested: usize,
    pub affected: u64,
}

/// Batch process reports
#[utoipa::path(
    post,
    path = "/v1/admin/reports/batch/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BatchStatusForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BatchStatusForm>,
) -> AppResult<ApiJson<BatchResult>> {
    user.require_admin()?;
    let r = admin_service::batch_set_report_status(&state, &user, &f.ids, f.status).await?;
    Ok(ApiJson(BatchResult {
        requested: r.requested,
        affected: r.affected,
    }))
}
