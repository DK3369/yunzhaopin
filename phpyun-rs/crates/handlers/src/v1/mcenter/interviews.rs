//! Interview invitations — job seekers view / respond; employers create / cancel.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::interview_service::{self, InterviewCreateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/interviews", get(list_mine))
        .route("/interviews/{id}/accept", post(accept))
        .route("/interviews/{id}/reject", post(reject))
        .route("/company/interviews", get(list_by_company))
        .route("/company/interviews/create", post(create))
        .route("/company/interviews/{id}/cancel", post(cancel))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn interview_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "accepted",
        2 => "rejected",
        3 => "cancelled",
        _ => "unknown",
    }
}

/// Interview invitation item — all 12 columns of `phpyun_interview` + formatted time + status name.
#[derive(Debug, Serialize, ToSchema)]
pub struct InterviewItem {
    pub id: u64,
    pub apply_id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub inter_time: i64,
    pub inter_time_n: String,
    pub address: String,
    pub linkman: String,
    pub linktel: String,
    pub remark: Option<String>,
    /// 0 pending / 1 accepted / 2 rejected / 3 cancelled
    pub status: i32,
    /// status text (pending/accepted/rejected/cancelled)
    pub status_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::interview::entity::Interview> for InterviewItem {
    fn from(i: phpyun_models::interview::entity::Interview) -> Self {
        Self {
            id: i.id,
            apply_id: i.apply_id,
            com_id: i.com_id,
            uid: i.uid,
            job_id: i.job_id,
            inter_time_n: fmt_dt(i.inter_time),
            inter_time: i.inter_time,
            address: i.address,
            linkman: i.linkman,
            linktel: i.linktel,
            remark: i.remark,
            status_n: interview_status_name(i.status).to_string(),
            status: i.status,
            created_at_n: fmt_dt(i.created_at),
            created_at: i.created_at,
        }
    }
}

// ==================== Job seeker side ====================

/// Job seeker views received interview invitations
#[utoipa::path(
    get,
    path = "/v1/mcenter/interviews",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<InterviewItem>>> {
    let r = interview_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(InterviewItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Accept interview
#[utoipa::path(
    post,
    path = "/v1/mcenter/interviews/{id}/accept",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn accept(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    interview_service::respond(&state, &user, id, 1, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "status": 1 })))
}

/// Reject interview
#[utoipa::path(
    post,
    path = "/v1/mcenter/interviews/{id}/reject",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn reject(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    interview_service::respond(&state, &user, id, 2, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "status": 2 })))
}

// ==================== Employer side ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInterviewForm {
    pub apply_id: u64,
    pub inter_time: i64,
    #[validate(length(min = 1, max = 255))]
    pub address: String,
    #[validate(length(min = 1, max = 50))]
    pub linkman: String,
    #[validate(length(min = 1, max = 50))]
    pub linktel: String,
    #[validate(length(max = 2000))]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// Employer creates an interview invitation (based on an apply record)
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/interviews/create",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateInterviewForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CreateInterviewForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = interview_service::create_by_company(
        &state,
        &user,
        InterviewCreateInput {
            apply_id: f.apply_id,
            inter_time: f.inter_time,
            address: &f.address,
            linkman: &f.linkman,
            linktel: &f.linktel,
            remark: f.remark.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Employer views interview invitations they have sent
#[utoipa::path(
    get,
    path = "/v1/mcenter/company/interviews",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_by_company(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<InterviewItem>>> {
    let r = interview_service::list_by_company(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(InterviewItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Employer cancels an interview
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/interviews/{id}/cancel",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn cancel(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    interview_service::cancel(&state, &user, id, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
