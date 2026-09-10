//! Member center - Job management (usertype=2 employer).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{BatchResult, CreatedId, IdBody};
use phpyun_core::json;
use phpyun_core::ApiError;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::job_mgmt_service::{self, CreateJobInput, UpdateJobInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", post(create))
        .route("/jobs/list", post(list_mine))
        .route("/jobs/counts", post(counts_by_state))
        .route("/jobs/detail", post(detail))
        .route("/jobs/update", post(update))
        .route("/jobs/status", post(set_status))
        .route("/jobs/refresh", post(refresh))
        .route("/jobs/batch/refresh", post(batch_refresh))
        .route("/jobs/batch/close", post(batch_close))
        .route("/jobs/batch/delete", post(batch_delete))
        .route("/jobs/reserve", post(reserve))
        .route("/jobs/reserve/get", post(reserve_get))
        .route("/jobs/promote/quote", post(promote_quote))
        .route("/jobs/promote", post(promote))
        .route("/jobs/promote/close", post(promote_close))
}

// ==================== Create ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateJobForm {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(range(min = 0, max = 99_999))]
    pub job1: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub job1_son: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub job_post: i32,
    #[validate(range(min = 0, max = 99_999))]
    pub provinceid: i32,
    #[validate(range(min = 0, max = 99_999))]
    pub cityid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_cityid: i32,
    #[validate(range(min = 0, max = 999))]
    pub salary: i32,
    /// Salary in CNY (yuan); cap to 1M to avoid overflow.
    #[validate(range(min = 0, max = 1_000_000))]
    pub minsalary: i32,
    #[validate(range(min = 0, max = 1_000_000))]
    pub maxsalary: i32,
    /// 57 = full-time / 58 = part-time / 59 = internship / 60 = temporary
    #[validate(range(min = 0, max = 99))]
    pub r#type: i32,
    #[validate(range(min = 0, max = 999))]
    pub number: i32,
    #[validate(range(min = 0, max = 99))]
    pub exp: i32,
    #[validate(range(min = 0, max = 99))]
    pub edu: i32,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    #[validate(length(max = 500))]
    pub wel: Option<String>,
    /// Start date — accepts unix-ts or `"YYYY-MM"` / `"YYYY-MM-DD"` strings.
    #[serde(
        default,
        alias = "sdate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub sdate: i64,
    #[serde(
        default,
        alias = "edate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub edate: i64,
}

/// Publish job
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateJobForm,
    responses((status = 200, description = "Published (pending review)", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CreateJobForm>,
) -> AppResult<ApiResponse<CreatedId>> {
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
    Ok(ApiResponse::data(CreatedId { id }))
}

// ==================== Update ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateJobForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 2, max = 50))]
    pub name: Option<String>,
    #[validate(range(min = 0, max = 99_999))]
    pub job1: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub job1_son: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub job_post: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub provinceid: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub cityid: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub three_cityid: Option<i32>,
    #[validate(range(min = 0, max = 999))]
    pub salary: Option<i32>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub minsalary: Option<i32>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub maxsalary: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub r#type: Option<i32>,
    #[validate(range(min = 0, max = 999))]
    pub number: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub exp: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub edu: Option<i32>,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    #[validate(length(max = 500))]
    pub wel: Option<String>,
    /// Start date — accepts unix-ts or `"YYYY-MM"` / `"YYYY-MM-DD"`. `None`
    /// when omitted by the client (partial update leaves DB column untouched).
    #[serde(
        default,
        alias = "sdate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts_opt"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub sdate: Option<i64>,
    #[serde(
        default,
        alias = "edate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts_opt"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub edate: Option<i64>,
}

/// Update job (re-enters review after editing)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateJobForm,
    responses((status = 200, description = "Saved (pending review)"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateJobForm>,
) -> AppResult<ApiResponse<json::Value>> {
    job_mgmt_service::update(
        &state,
        &user,
        f.id,
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
    Ok(ApiResponse::data(json::json!({ "ok": true })))
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
    pub breakjob_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

/// My jobs grouped by state (used for job management tab badges)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/counts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = JobCountsView))
)]
pub async fn counts_by_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<JobCountsView>> {
    let c = job_mgmt_service::counts_by_state(&state, &user).await?;
    Ok(ApiResponse::data(JobCountsView {
        online: c.online,
        pending: c.pending,
        closed: c.closed,
        total: c.online + c.pending + c.closed,
        breakjob_num: c.breakjob_num,
        top_num: c.top_num,
        rec_num: c.rec_num,
        urgent_num: c.urgent_num,
    }))
}

// ==================== Status / Refresh / Delete ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// 0 = recruiting, 1 = unlisted (PHP `company_job.status`; legacy 2 is accepted as 1)
    #[validate(range(min = 0, max = 9))]
    pub status: i32,
}

/// Open / close (online / offline)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse<json::Value>> {
    job_mgmt_service::set_status(&state, &user, f.id, f.status, &ip).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "status": f.status }),
    ))
}

/// Refresh job (bumps `lastupdate` so it sorts to the top of the public list)
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    job_mgmt_service::refresh(&state, &user, b.id, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

// Delete job — **merged into update**:
// The client sends `POST /v1/mcenter/jobs/{id}/status` with body `{"status": 2}` to trigger a soft delete.
// The repo-layer `delete()` has been changed to `UPDATE ... SET state=2`; no physical DELETE is performed.

// ==================== List + Detail ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MyJobsQuery {
    /// Optional review-state filter: 0 pending / 1 approved / 2 closed / 3 rejected
    #[validate(range(min = 0, max = 99))]
    pub state: Option<i32>,
}

/// Employer's own job item — **reuses** `wap::jobs::JobSummary` (34 fields, full dict translation + promotion status derivation + formatted time).
///
/// Single field schema: the management backend and the public list / homepage `hot_jobs` / global search results all share the same Summary,
/// front-end templates are reused, and i18n applies in one place.
pub type MyJobSummary = crate::v1::wap::jobs::JobSummary;

/// Employer views their own list of published jobs
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/list",
    tag = "mcenter",
    security(("bearer" = [])),
    params(MyJobsQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<MyJobsQuery>,
) -> AppResult<ApiResponse<Paged<MyJobSummary>>> {
    let r = job_mgmt_service::list_mine(&state, &user, q.state, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiResponse::data(Paged::new(
        r.list
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Employer views the details of one of their own jobs
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/detail",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    user.require_employer()?;
    let j = phpyun_models::job::repo::find_by_id(state.db.reader(), b.id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    Ok(ApiResponse::data(json::to_value(&j)?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchIdsForm {
    /// Up to 100 ids
    #[validate(length(min = 1, max = 100))]
    pub ids: Vec<u64>,
}

fn batch_result(r: phpyun_services::job_mgmt_service::BatchReport) -> BatchResult {
    BatchResult {
        requested: r.requested,
        affected: r.affected,
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
) -> AppResult<ApiResponse<BatchResult>> {
    let r = job_mgmt_service::batch_refresh(&state, &user, &f.ids, &ip).await?;
    Ok(ApiResponse::data(batch_result(r)))
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
) -> AppResult<ApiResponse<BatchResult>> {
    let r = job_mgmt_service::batch_close(&state, &user, &f.ids, &ip).await?;
    Ok(ApiResponse::data(batch_result(r)))
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
) -> AppResult<ApiResponse<BatchResult>> {
    let r = job_mgmt_service::batch_delete(&state, &user, &f.ids, &ip).await?;
    Ok(ApiResponse::data(batch_result(r)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PromoteQuoteForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
    #[validate(length(min = 1, max = 16))]
    pub kind: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PromoteQuoteView {
    pub kind: String,
    pub remain: i32,
    pub expire_at: i64,
    pub active: bool,
}

/// 套餐推广报价：剩余次数 + 当前到期时间
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/promote/quote",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PromoteQuoteForm,
    responses((status = 200, description = "ok", body = PromoteQuoteView))
)]
pub async fn promote_quote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PromoteQuoteForm>,
) -> AppResult<ApiResponse<PromoteQuoteView>> {
    let q = job_mgmt_service::quote_promote(&state, &user, f.job_id, &f.kind).await?;
    Ok(ApiResponse::data(PromoteQuoteView {
        kind: q.kind,
        remain: q.remain,
        expire_at: q.expire_at,
        active: q.active,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PromoteForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
    #[validate(length(min = 1, max = 16))]
    pub kind: String,
    #[validate(range(min = 1, max = 365))]
    pub days: i32,
}

/// 职位置顶 / 推荐 / 紧急（扣套餐次数）
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/promote",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PromoteForm,
    responses((status = 200, description = "ok", body = PromoteQuoteView))
)]
pub async fn promote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<PromoteForm>,
) -> AppResult<ApiResponse<PromoteQuoteView>> {
    let q = job_mgmt_service::promote(&state, &user, f.job_id, &f.kind, f.days, &ip).await?;
    Ok(ApiResponse::data(PromoteQuoteView {
        kind: q.kind,
        remain: q.remain,
        expire_at: q.expire_at,
        active: q.active,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PromoteCloseView {
    pub ok: bool,
    pub refunded: i32,
}

/// 关闭职位置顶 / 推荐 / 紧急；`tg_back=1` 时退回剩余天数
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/promote/close",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PromoteQuoteForm,
    responses((status = 200, description = "ok", body = PromoteCloseView))
)]
pub async fn promote_close(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<PromoteQuoteForm>,
) -> AppResult<ApiResponse<PromoteCloseView>> {
    let r = job_mgmt_service::close_promote(&state, &user, f.job_id, &f.kind, &ip).await?;
    Ok(ApiResponse::data(PromoteCloseView {
        ok: true,
        refunded: r.refunded,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReserveForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
    /// `YYYY-MM-DD` or unix seconds.
    #[serde(default)]
    #[validate(length(max = 32))]
    pub end_time: String,
    #[validate(range(min = 0, max = 10_080))]
    pub interval: i32,
    /// 1 = open schedule, 2 = close.
    #[validate(range(min = 1, max = 2))]
    pub status: i32,
    #[serde(default)]
    #[validate(length(max = 8))]
    pub s_time: String,
    #[serde(default)]
    #[validate(length(max = 8))]
    pub e_time: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReserveOk {
    pub ok: bool,
}

/// PHP member `job::reserveUpJob` — schedule or cancel auto-refresh for one job.
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/reserve",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReserveForm,
    responses((status = 200, description = "ok", body = ReserveOk))
)]
pub async fn reserve(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReserveForm>,
) -> AppResult<ApiResponse<ReserveOk>> {
    user.require_employer()?;
    job_mgmt_service::up_reserve(
        &state,
        user.uid,
        &[f.job_id],
        f.status,
        &f.end_time,
        f.interval,
        &f.s_time,
        &f.e_time,
    )
    .await?;
    Ok(ApiResponse::data(ReserveOk { ok: true }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReserveGetForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReserveInfoView {
    pub status: i32,
    pub interval: i32,
    pub s_time: String,
    pub e_time: String,
    pub end_time: i64,
}

/// PHP member `job::reserveInfo` — current auto-refresh schedule for one job.
#[utoipa::path(
    post,
    path = "/v1/mcenter/jobs/reserve/get",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReserveGetForm,
    responses((status = 200, description = "ok", body = ReserveInfoView))
)]
pub async fn reserve_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReserveGetForm>,
) -> AppResult<ApiResponse<ReserveInfoView>> {
    let r = job_mgmt_service::get_reserve(&state, &user, f.job_id).await?;
    Ok(ApiResponse::data(ReserveInfoView {
        status: r.status,
        interval: r.interval,
        s_time: r.s_time,
        e_time: r.e_time,
        end_time: r.end_time,
    }))
}
