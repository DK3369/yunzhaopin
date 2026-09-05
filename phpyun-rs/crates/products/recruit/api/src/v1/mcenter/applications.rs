//! Employer views received applications + mark as read + invite to interview (usertype=2).

use axum::{extract::State, routing::post, Router};
use phpyun_core::date_parse::{de_loose_i32_opt, de_loose_u64_opt};
use phpyun_core::dto::{BatchResult, IdBody, IdsBody};
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    clock, ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination,
    ValidatedJson,
};
use phpyun_models::apply::repo::ApplyFilter;
use phpyun_services::apply_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/applications", post(list_received))
        .route("/applications/state-counts", post(state_counts))
        .route("/applications/browse", post(mark_browsed))
        .route("/applications/batch-read", post(batch_read))
        .route("/applications/state", post(set_state))
        .route("/applications/delete", post(delete_received))
        .route("/applications/invite", post(invite))
}

/// Filters of the PHP employer screen `member/com/model/hr.class.php`.
#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ApplicationsQuery {
    /// Show only unread (unbrowsed)
    #[serde(default)]
    pub unread_only: Option<bool>,
    /// Show only invited
    #[serde(default)]
    pub invited_only: Option<bool>,
    /// PHP `is_browse` 1/2/3/4/5/7
    #[serde(default)]
    #[validate(range(min = 1, max = 7))]
    pub state: Option<i32>,
    /// PHP `jobid`: restrict to one of my postings
    #[serde(default, deserialize_with = "de_loose_u64_opt")]
    pub job_id: Option<u64>,
    /// PHP `rstate`: `resume_state` of the application
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    #[validate(range(min = 0, max = 9))]
    pub resume_state: Option<i32>,
    /// PHP `keyword`: applicant name
    #[serde(default)]
    #[validate(length(max = 60))]
    pub keyword: Option<String>,
    /// PHP `edu`: education level of the submitted resume
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    pub edu: Option<i32>,
    /// PHP `exp`: years of experience of the submitted resume
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    pub exp: Option<i32>,
    /// PHP `sex`: gender of the submitted resume
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    pub sex: Option<i32>,
    /// PHP `uptime`: resume updated within N days (1 = since midnight today)
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    #[validate(range(min = 1, max = 3650))]
    pub uptime: Option<i32>,
}

impl ApplicationsQuery {
    fn to_filter(&self) -> ApplyFilter {
        ApplyFilter {
            unread_only: self.unread_only,
            invited_only: self.invited_only,
            browse_state: self.state,
            job_id: self.job_id,
            resume_state: self.resume_state,
            keyword: self
                .keyword
                .as_deref()
                .map(str::trim)
                .filter(|k| !k.is_empty())
                .map(str::to_owned),
            // PHP guards these with `if ($_GET['edu'])`, so 0 means "any".
            // `rstate` instead uses `!= ''`, where 0 (pending review) is real.
            edu: self.edu.filter(|v| *v != 0),
            exp: self.exp.filter(|v| *v != 0),
            sex: self.sex.filter(|v| *v != 0),
            updated_after: self
                .uptime
                .filter(|d| *d > 0)
                .map(|d| apply_service::resume_updated_cutoff(d, clock::now_ts())),
        }
    }
}

/// Application record item — full 11 columns of phpyun_userid_job + formatted timestamps + derived unread/invited booleans.
#[derive(Debug, Serialize, ToSchema)]
pub struct ApplicantSummary {
    pub id: u64,
    /// Job seeker uid
    pub uid: u64,
    pub job_id: u64,
    /// Employer uid
    pub com_id: u64,
    /// Resume id (in PHPYun, eid equals the job seeker's uid)
    pub eid: u64,
    pub job_name: String,
    pub uname: String,
    pub datetime: i64,
    pub datetime_n: String,
    /// 1 unviewed / 2 viewed / 3 interviewed / 4 not suitable / 7 hired etc.
    pub is_browse: i32,
    /// Derived: is_browse == 1
    pub unread: bool,
    /// 1 invited / 0 not invited
    pub invited_int: i32,
    /// Derived: invited_int == 1
    pub invited: bool,
    pub invite_time: i64,
    pub invite_time_n: String,
    /// 9 normal / 0 deleted (PHPYun `isdel`)
    pub isdel: i32,
    /// Whether the job seeker has withdrawn
    pub quxiao: i32,
}

impl From<phpyun_models::apply::entity::Apply> for ApplicantSummary {
    fn from(a: phpyun_models::apply::entity::Apply) -> Self {
        Self {
            id: a.id,
            uid: a.uid,
            job_id: a.job_id,
            com_id: a.com_id,
            eid: a.eid,
            job_name: a.job_name,
            uname: a.uname,
            datetime_n: fmt_dt(a.datetime),
            datetime: a.datetime,
            unread: a.is_browse == 1,
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

/// Employer views all received applications
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ApplicationsQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_received(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ApplicationsQuery>,
) -> AppResult<ApiResponse<Paged<ApplicantSummary>>> {
    let r = apply_service::list_for_company(&state, &user, q.to_filter(), page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

/// Applicant totals per `is_browse`, for the status tabs. Counts honour every
/// other filter but ignore `state`, so switching tabs keeps the badges stable.
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/state-counts",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ApplicationsQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn state_counts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ApplicationsQuery>,
) -> AppResult<ApiResponse<StateCounts>> {
    let counts = apply_service::state_counts_for_company(&state, &user, q.to_filter()).await?;
    let at = |k: i32| counts.get(&k).copied().unwrap_or(0);
    Ok(ApiResponse::data(StateCounts {
        total: counts.values().sum(),
        pending: at(1),
        viewed: at(2),
        to_notify: at(3),
        unsuitable: at(4),
        unreachable: at(5),
        hired: at(7),
    }))
}

/// Applicant counts per PHPYun `is_browse` value.
#[derive(Debug, Serialize, ToSchema)]
pub struct StateCounts {
    pub total: u64,
    /// is_browse = 1
    pub pending: u64,
    /// is_browse = 2
    pub viewed: u64,
    /// is_browse = 3
    pub to_notify: u64,
    /// is_browse = 4
    pub unsuitable: u64,
    /// is_browse = 5
    pub unreachable: u64,
    /// is_browse = 7
    pub hired: u64,
}

/// Mark as read (idempotent)
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/browse",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mark_browsed(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    apply_service::mark_browsed(&state, &user, b.id).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

/// Mark a batch of applications as read (PHP bulk "mark viewed" checkbox action)
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/batch-read",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<BatchResult>> {
    let affected = apply_service::mark_browsed_batch(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(BatchResult {
        requested: b.ids.len(),
        affected,
    }))
}

/// Remove a received application from the employer's list
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Application does not belong to you"),
    )
)]
pub async fn delete_received(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    apply_service::delete_for_company(&state, &user, b.id, &ip).await?;
    Ok(ApiResponse::message("deleted"))
}

#[derive(Debug, serde::Deserialize, Validate, ToSchema)]
pub struct SetStateBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// Aligned with PHPYun `is_browse`: 1=unviewed / 2=viewed / 3=interviewed / 4=not suitable / 5=unreachable / 7=hired
    #[validate(range(min = 1, max = 7))]
    pub state: i32,
}

/// Set application feedback state (richer than the binary value of /browse — accepts 5 enum values)
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/state",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SetStateBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "state not in {1,2,3,4,5,7}"),
        (status = 403, description = "Application does not belong to you"),
    )
)]
pub async fn set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<SetStateBody>,
) -> AppResult<ApiResponse<json::Value>> {
    apply_service::set_browse_state(&state, &user, b.id, b.state, &ip).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "state": b.state }),
    ))
}

/// Invite to interview
#[utoipa::path(
    post,
    path = "/v1/mcenter/applications/invite",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn invite(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    apply_service::invite_interview(&state, &user, b.id, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}
