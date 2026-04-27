//! Employer views received applications + mark as read + invite to interview (usertype=2).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_models::apply::repo::ApplyFilter;
use phpyun_services::apply_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/applications", post(list_received))
        .route("/applications/browse", post(mark_browsed))
        .route("/applications/state", post(set_state))
        .route("/applications/invite", post(invite))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ApplicationsQuery {
    /// Show only unread (unbrowsed)
    #[serde(default)]
    pub unread_only: Option<bool>,
    /// Show only invited
    #[serde(default)]
    pub invited_only: Option<bool>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
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
    pub datetime: i64,
    pub datetime_n: String,
    /// 1 unviewed / 0 viewed / 3 interviewed / 4 not suitable / 7 hired etc. (PHP polysemous status)
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
) -> AppResult<ApiJson<Paged<ApplicantSummary>>> {
    let filter = ApplyFilter {
        unread_only: q.unread_only,
        invited_only: q.invited_only,
    };
    let r = apply_service::list_for_company(&state, &user, filter, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ApplicantSummary::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
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
) -> AppResult<ApiJson<json::Value>> {
    apply_service::mark_browsed(&state, &user, b.id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[derive(Debug, serde::Deserialize, Validate, ToSchema)]
pub struct SetStateBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// Aligned with PHPYun `is_browse`: 1=unviewed / 0=viewed / 3=interviewed / 4=not suitable / 7=hired
    #[validate(range(min = 0, max = 7))]
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
        (status = 400, description = "state not in {0,1,3,4,7}"),
        (status = 403, description = "Application does not belong to you"),
    )
)]
pub async fn set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<SetStateBody>,
) -> AppResult<ApiJson<json::Value>> {
    apply_service::set_browse_state(&state, &user, b.id, b.state, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "state": b.state })))
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
) -> AppResult<ApiJson<json::Value>> {
    apply_service::invite_interview(&state, &user, b.id, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
