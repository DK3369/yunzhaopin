//! Public job-page Q&A — counterpart of PHP `wap/ajax::pl_action` (write) and
//! the inline message panel inside `wap/job::view_action` (public read).
//!
//! Endpoints:
//!   * `GET  /v1/wap/jobs/{id}/messages`         — paginated public Q&A list
//!   * `POST /v1/wap/jobs/{id}/messages`         — jobseeker leaves a message
//!   * `POST /v1/wap/jobs/{id}/messages/{mid}/hide` — author hides their own message

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, verify::{self, VerifyKind},
    ApiJson, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::job_msg_service::{self, CreateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs/{id}/messages", get(list).post(create))
        .route("/jobs/{id}/messages/{mid}/hide", post(hide))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Public-facing message item (no internal flags exposed).
#[derive(Debug, Serialize, ToSchema)]
pub struct JobMsgView {
    pub id: u64,
    pub uid: Option<u64>,
    pub username: Option<String>,
    pub jobid: Option<u64>,
    pub job_uid: Option<u64>,
    pub content: Option<String>,
    pub reply: Option<String>,
    pub datetime: i64,
    pub datetime_n: String,
    pub reply_time: i64,
    pub reply_time_n: String,
}

impl From<phpyun_models::job_msg::entity::JobMsg> for JobMsgView {
    fn from(m: phpyun_models::job_msg::entity::JobMsg) -> Self {
        Self {
            id: m.id,
            uid: m.uid,
            username: m.username,
            jobid: m.jobid,
            job_uid: m.job_uid,
            content: m.content,
            reply: m.reply,
            datetime_n: fmt_dt(m.datetime),
            datetime: m.datetime,
            reply_time_n: fmt_dt(m.reply_time),
            reply_time: m.reply_time,
        }
    }
}

/// Public list of approved Q&A for a job.
#[utoipa::path(
    get,
    path = "/v1/wap/jobs/{id}/messages",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<JobMsgView>>> {
    let r = job_msg_service::list_public(&state, id, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(JobMsgView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMessageForm {
    /// Plain-text inquiry content. Max 1000 chars.
    #[validate(length(min = 1, max = 4000))]
    pub content: String,
    /// Image-captcha cid (anti-spam) — required, must be paired with `authcode`.
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    /// Image-captcha input (case-insensitive).
    #[validate(length(min = 1, max = 16))]
    pub authcode: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateMessageData {
    pub id: u64,
}

/// Jobseeker leaves a public message — requires login + image captcha.
/// Mirrors PHP `wap/ajax::pl_action` (auth check, blacklist check, captcha).
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/{id}/messages",
    tag = "wap",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = CreateMessageForm,
    responses(
        (status = 200, description = "Created", body = CreateMessageData),
        (status = 400, description = "Validation / captcha / blocked / job not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Only jobseekers may post messages"),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<CreateMessageForm>,
) -> AppResult<ApiJson<CreateMessageData>> {
    // Captcha — same scheme as register / sms send.
    let code = f.authcode.to_uppercase();
    if !verify::verify(&state.redis, VerifyKind::ImageCaptcha, &f.captcha_cid, &code).await? {
        return Err(AppError::captcha());
    }

    let mid = job_msg_service::create(
        &state,
        &user,
        CreateInput {
            jobid: id,
            content: &f.content,
        },
    )
    .await?;
    Ok(ApiJson(CreateMessageData { id: mid }))
}

/// Author / employer hides one of their messages.
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/{id}/messages/{mid}/hide",
    tag = "wap",
    security(("bearer" = [])),
    params(
        ("id" = u64, Path),
        ("mid" = u64, Path),
    ),
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Message not found"),
        (status = 403, description = "Not the author or job owner"),
    )
)]
pub async fn hide(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((_id, mid)): Path<(u64, u64)>,
) -> AppResult<ApiJson<json::Value>> {
    job_msg_service::hide(&state, &user, mid).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
