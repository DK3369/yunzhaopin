//! Job-message employer side — read / reply.
//!
//! Counterpart of PHP `member/com/jobmsg`-style management screens. Posts /
//! lists are grouped under `/v1/mcenter/job-messages` so they don't collide
//! with the user-facing `/v1/wap/jobs/{id}/messages` write endpoint.

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::job_msg_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/job-messages", post(list))
        .route("/job-messages/reply", post(reply))
        .route("/job-messages/hide", post(hide))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmployerMsgItem {
    pub id: u64,
    pub uid: Option<u64>,
    pub username: Option<String>,
    pub jobid: Option<u64>,
    pub job_name: Option<String>,
    pub content: Option<String>,
    pub reply: Option<String>,
    pub datetime: i64,
    pub datetime_n: String,
    pub reply_time: i64,
    pub reply_time_n: String,
    /// Whether the message has been answered.
    pub answered: bool,
}

impl From<phpyun_models::job_msg::entity::JobMsg> for EmployerMsgItem {
    fn from(m: phpyun_models::job_msg::entity::JobMsg) -> Self {
        let answered = m.reply.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
        Self {
            id: m.id,
            uid: m.uid,
            username: m.username,
            jobid: m.jobid,
            job_name: m.job_name,
            content: m.content,
            reply: m.reply,
            datetime_n: fmt_dt(m.datetime),
            datetime: m.datetime,
            reply_time_n: fmt_dt(m.reply_time),
            reply_time: m.reply_time,
            answered,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// `true` keeps only messages that have not yet been replied to (badge view).
    #[serde(default)]
    pub only_unanswered: bool,
}

/// Employer lists every message left on any of their jobs.
#[utoipa::path(
    post,
    path = "/v1/mcenter/job-messages",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Not an employer"),
    )
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<EmployerMsgItem>>> {
    let r = job_msg_service::list_for_employer(&state, &user, q.only_unanswered, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(EmployerMsgItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReplyForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 4000))]
    pub reply: String,
}

/// Employer answers a single message.
#[utoipa::path(post,
    path = "/v1/mcenter/job-messages/reply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReplyForm,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Empty reply / message not found / not yours"),
        (status = 403, description = "Not an employer"),
    )
)]
pub async fn reply(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReplyForm>) -> AppResult<ApiJson<json::Value>> {
    let id = f.id;
    job_msg_service::employer_reply(&state, &user, id, &f.reply).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

/// Employer hides a message they own.
#[utoipa::path(post,
    path = "/v1/mcenter/job-messages/hide",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn hide(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    job_msg_service::hide(&state, &user, id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

