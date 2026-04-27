//! Member center - Job management (usertype=2 employer).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson, ValidatedQuery
};
use phpyun_services::job_mgmt_service::{self, CreateJobInput, UpdateJobInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(list_mine).post(create))
        .route("/jobs/counts", get(counts_by_state))
        .route("/jobs/{id}", get(detail).post(update))
        .route("/jobs/{id}/status", post(set_status))
        .route("/jobs/{id}/refresh", post(refresh))
        .route("/jobs/batch/refresh", post(batch_refresh))
        .route("/jobs/batch/close", post(batch_close))
        .route("/jobs/batch/delete", post(batch_delete))
}

// ==================== Create ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateJobForm {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    pub job1: i32,
    #[serde(default)]
    pub job1_son: i32,
    #[serde(default)]
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    #[serde(default)]
    pub three_cityid: i32,
    pub salary: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub r#type: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    #[validate(length(max = 100000))]
    pub content: Option<String>,
    #[validate(length(max = 1000))]
    pub wel: Option<String>,
    pub sdate: i64,
    pub edate: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateJobData {
    pub id: u64,
}

/// Publish job
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateJobForm,
    responses((status = 200, description = "Published (pending review)", body = CreateJobData))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CreateJobForm>,
) -> AppResult<ApiJson<CreateJobData>> {
    // Company name comes from the company table; leave None here for now (employer side syncs `com_name` on update).
    let id = job_mgmt_service::create(
        &state,
        &user,
        CreateJobInput {
            name: &f.name,
            job1: f.job1,
            job1_son: f.job1_son,
            job_post: f.job_post,
            provinceid: f.provinceid,
            cityid: f.cityid,
            three_cityid: f.three_cityid,
            minsalary: f.minsalary,
            maxsalary: f.maxsalary,
            job_type: f.r#type,
            number: f.number,
            exp: f.exp,
            edu: f.edu,
            content: f.content.as_deref(),
            wel: f.wel.as_deref(),
            sdate: f.sdate,
            edate: f.edate,
        },
        None,
        &ip,
    )
    .await?;
    Ok(ApiJson(CreateJobData { id }))
}

// ==================== Update ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateJobForm {
    #[validate(length(min = 2, max = 50))]
    pub name: Option<String>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    pub salary: Option<i32>,
    pub minsalary: Option<i32>,
    pub maxsalary: Option<i32>,
    pub r#type: Option<i32>,
    pub number: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    #[validate(length(max = 100000))]
    pub content: Option<String>,
    #[validate(length(max = 1000))]
    pub wel: Option<String>,
    pub sdate: Option<i64>,
    pub edate: Option<i64>,
}

/// Update job (re-enters review after editing)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = UpdateJobForm,
    responses((status = 200, description = "Saved (pending review)"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<UpdateJobForm>,
) -> AppResult<ApiJson<json::Value>> {
    job_mgmt_service::update(
        &state,
        &user,
        id,
        UpdateJobInput {
            name: f.name.as_deref(),
            job1: f.job1,
            job1_son: f.job1_son,
            job_post: f.job_post,
            provinceid: f.provinceid,
            cityid: f.cityid,
            three_cityid: f.three_cityid,
            minsalary: f.minsalary,
            maxsalary: f.maxsalary,
            job_type: f.r#type,
            number: f.number,
            exp: f.exp,
            edu: f.edu,
            content: f.content.as_deref(),
            wel: f.wel.as_deref(),
            sdate: f.sdate,
            edate: f.edate,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// ==================== Status group counts ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct JobCountsView {
    /// Recruiting (state=0)
    pub online: u64,
    /// Pending review (state=1)
    pub pending: u64,
    /// Closed (state=2)
    pub closed: u64,
    pub total: u64,
}

/// My jobs grouped by state (used for job management tab badges)
#[utoipa::path(
    get,
    path = "/v1/mcenter/jobs/counts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = JobCountsView))
)]
pub async fn counts_by_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<JobCountsView>> {
    let c = job_mgmt_service::counts_by_state(&state, &user).await?;
    Ok(ApiJson(JobCountsView {
        online: c.online,
        pending: c.pending,
        closed: c.closed,
        total: c.online + c.pending + c.closed,
    }))
}

// ==================== Status / Refresh / Delete ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    /// 0 = online, 2 = closed
    pub status: i32,
}

/// Open / close (online / offline)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/{id}/status",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiJson<json::Value>> {
    job_mgmt_service::set_status(&state, &user, id, f.status, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "status": f.status })))
}

/// Refresh job (bumps `lastupdate` so it sorts to the top of the public list)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/{id}/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    job_mgmt_service::refresh(&state, &user, id, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// Delete job — **merged into update**:
// The client sends `POST /v1/mcenter/jobs/{id}/status` with body `{"status": 2}` to trigger a soft delete.
// The repo-layer `delete()` has been changed to `UPDATE ... SET state=2`; no physical DELETE is performed.

// ==================== List + Detail ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MyJobsQuery {
    /// Optional review-state filter: 0 pending / 1 approved / 2 closed / 3 rejected
    pub state: Option<i32>,
}

/// Employer's own job item — **reuses** `wap::jobs::JobSummary` (34 fields, full dict translation + promotion status derivation + formatted time).
///
/// Single field schema: the management backend and the public list / homepage `hot_jobs` / global search results all share the same Summary,
/// front-end templates are reused, and i18n applies in one place.
pub type MyJobSummary = crate::v1::wap::jobs::JobSummary;

/// Employer views their own list of published jobs
#[utoipa::path(
    get,
    path = "/v1/mcenter/jobs",
    tag = "mcenter",
    security(("bearer" = [])),
    params(MyJobsQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<MyJobsQuery>,
) -> AppResult<ApiJson<Paged<MyJobSummary>>> {
    let r = job_mgmt_service::list_mine(&state, &user, q.state, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|j| MyJobSummary::from_with_dict(j, &dicts, now))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Employer views the details of one of their own jobs
#[utoipa::path(
    get,
    path = "/v1/mcenter/jobs/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    user.require_employer()?;
    let j = phpyun_models::job::repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or(phpyun_services::JobError::NotFound)?;
    Ok(ApiJson(json::to_value(&j)?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchIdsForm {
    /// Up to 100 ids
    #[validate(length(min = 1, max = 100))]
    pub ids: Vec<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchResult {
    pub requested: usize,
    pub affected: u64,
}

impl From<phpyun_services::job_mgmt_service::BatchReport> for BatchResult {
    fn from(r: phpyun_services::job_mgmt_service::BatchReport) -> Self {
        Self { requested: r.requested, affected: r.affected }
    }
}

/// Batch refresh
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/batch/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BatchIdsForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<BatchIdsForm>,
) -> AppResult<ApiJson<BatchResult>> {
    let r = job_mgmt_service::batch_refresh(&state, &user, &f.ids, &ip).await?;
    Ok(ApiJson(BatchResult::from(r)))
}

/// Batch close
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/batch/close",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BatchIdsForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_close(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<BatchIdsForm>,
) -> AppResult<ApiJson<BatchResult>> {
    let r = job_mgmt_service::batch_close(&state, &user, &f.ids, &ip).await?;
    Ok(ApiJson(BatchResult::from(r)))
}

/// Batch delete
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/batch/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BatchIdsForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<BatchIdsForm>,
) -> AppResult<ApiJson<BatchResult>> {
    let r = job_mgmt_service::batch_delete(&state, &user, &f.ids, &ip).await?;
    Ok(ApiJson(BatchResult::from(r)))
}
