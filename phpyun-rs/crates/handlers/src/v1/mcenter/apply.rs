//! Job seeker submitting resumes + my applications (usertype=1).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::apply_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/apply", post(apply_to_job))
        .route("/my-applications", post(list_mine))
        .route("/my-applications/withdraw", post(withdraw))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ApplyForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplyCreated {
    pub id: u64,
    pub job_id: u64,
}

/// Job seeker submits a resume application to a job
#[utoipa::path(
    post,
    path = "/v1/mcenter/apply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ApplyForm,
    responses(
        (status = 200, description = "Application submitted", body = ApplyCreated),
        (status = 400, description = "Cannot apply to your own job"),
        (status = 404, description = "Job not found"),
        (status = 409, description = "Already applied"),
        (status = 410, description = "Job off-shelf / expired"),
    )
)]
pub async fn apply_to_job(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ApplyForm>,
) -> AppResult<ApiJson<ApplyCreated>> {
    let r = apply_service::apply_to_job(&state, &user, f.job_id, &ip).await?;
    Ok(ApiJson(ApplyCreated {
        id: r.id,
        job_id: r.job_id,
    }))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// My application item — full 11 columns of phpyun_userid_job + formatted timestamps + derived employer_viewed/invited booleans.
#[derive(Debug, Serialize, ToSchema)]
pub struct MyApplySummary {
    pub id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub com_id: u64,
    pub eid: u64,
    pub datetime: i64,
    pub datetime_n: String,
    /// 1 unviewed / 0 viewed / 3 interviewed / 4 not suitable / 7 hired
    pub is_browse: i32,
    /// Derived: whether the employer has viewed (is_browse == 0)
    pub employer_viewed: bool,
    pub invited_int: i32,
    pub invited: bool,
    pub invite_time: i64,
    pub invite_time_n: String,
    pub isdel: i32,
    pub quxiao: i32,
}

impl From<phpyun_models::apply::entity::Apply> for MyApplySummary {
    fn from(a: phpyun_models::apply::entity::Apply) -> Self {
        Self {
            id: a.id,
            uid: a.uid,
            job_id: a.job_id,
            com_id: a.com_id,
            eid: a.eid,
            datetime_n: fmt_dt(a.datetime),
            datetime: a.datetime,
            employer_viewed: a.is_browse == 0,
            is_browse: a.is_browse,
            invited: a.invited == 1,
            invited_int: a.invited,
            invite_time_n: fmt_dt(a.invite_time),
            invite_time: a.invite_time,
            isdel: a.isdel,
            quxiao: a.quxiao,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MyAppliesQuery {
    /// Filter by application feedback state (aligned with phpyun `is_browse` enum):
    /// 1=unviewed / 0=viewed / 3=interviewed / 4=not suitable / 7=hired.
    /// Omitted = all.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub state: Option<i32>,
}

/// Job seeker views their own application list
#[utoipa::path(
    post,
    path = "/v1/mcenter/my-applications",
    tag = "mcenter",
    security(("bearer" = [])),
    params(MyAppliesQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<MyAppliesQuery>,
) -> AppResult<ApiJson<Paged<MyApplySummary>>> {
    let r = apply_service::list_mine(&state, &user, q.state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyApplySummary::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Withdraw an application
#[utoipa::path(
    post,
    path = "/v1/mcenter/my-applications/withdraw",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn withdraw(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    apply_service::withdraw(&state, &user, b.id, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
