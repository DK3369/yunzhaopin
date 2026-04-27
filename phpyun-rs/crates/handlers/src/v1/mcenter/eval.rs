//! Assessment submission + my history.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::eval_service;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/eval-papers/{id}/submit", post(submit))
        .route("/eval-logs", get(list_logs))
        .route("/eval-logs/{id}", get(get_log))
        .route("/eval-papers/{id}/messages", post(post_message))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitForm {
    /// In the form `{"<question_id>": "<option_label>"}`
    pub answers: HashMap<String, String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubmitResult {
    pub log_id: u64,
    pub score: i32,
}

/// Submit assessment answers
#[utoipa::path(
    post,
    path = "/v1/mcenter/eval-papers/{id}/submit",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = SubmitForm,
    responses((status = 200, description = "ok", body = SubmitResult))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<SubmitForm>,
) -> AppResult<ApiJson<SubmitResult>> {
    let (log_id, score) = eval_service::submit(&state, &user, id, f.answers).await?;
    Ok(ApiJson(SubmitResult { log_id, score }))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Assessment history item — all 6 columns of phpyun_eval_log + formatted timestamp.
#[derive(Debug, Serialize, ToSchema)]
pub struct LogItem {
    pub id: u64,
    pub uid: u64,
    pub paper_id: u64,
    pub score: i32,
    pub answers: phpyun_core::json::Value,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::eval::entity::EvalLog> for LogItem {
    fn from(l: phpyun_models::eval::entity::EvalLog) -> Self {
        Self {
            id: l.id,
            uid: l.uid,
            paper_id: l.paper_id,
            score: l.score,
            answers: l.answers,
            created_at_n: fmt_dt(l.created_at),
            created_at: l.created_at,
        }
    }
}

/// My assessment history
#[utoipa::path(
    get,
    path = "/v1/mcenter/eval-logs",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<LogItem>>> {
    let r = eval_service::list_my_logs(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(LogItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== Leave a message on a paper ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaperMessageForm {
    /// Free-text comment, 1..=512 chars.
    #[validate(length(min = 1, max = 512))]
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaperMessageCreated {
    pub id: u64,
}

/// Leave a public message on an assessment paper. Counterpart of PHP
/// `evaluate/exampaper::message_action`. The list view lives at
/// `GET /v1/wap/eval-papers/{id}/messages`.
#[utoipa::path(
    post,
    path = "/v1/mcenter/eval-papers/{id}/messages",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = PaperMessageForm,
    responses((status = 200, description = "ok", body = PaperMessageCreated))
)]
pub async fn post_message(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<PaperMessageForm>,
) -> AppResult<ApiJson<PaperMessageCreated>> {
    let id_u32 = id as u32;
    let now = phpyun_core::clock::now_ts();
    let new_id = phpyun_models::eval::repo::insert_paper_message(
        state.db.pool(),
        id_u32,
        user.uid,
        user.usertype as i32,
        f.message.trim(),
        now,
    )
    .await?;
    Ok(ApiJson(PaperMessageCreated { id: new_id }))
}

// ==================== Single eval log detail ====================

/// Look up one of the caller's own assessment results — counterpart of PHP
/// `evaluate/exampaper::gradeshow_action` (the data slice; the `examinee`
/// sidebar lives at `/v1/wap/eval-papers/{id}/recent-examinees`).
#[utoipa::path(
    get,
    path = "/v1/mcenter/eval-logs/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok", body = LogItem),
        (status = 404, description = "Not found / not yours"),
    )
)]
pub async fn get_log(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(log_id): Path<u64>,
) -> AppResult<ApiJson<LogItem>> {
    let row = phpyun_models::eval::repo::find_log_for_owner(state.db.reader(), log_id, user.uid)
        .await?
        .ok_or_else(|| {
            phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
                "log_not_found".into(),
            ))
        })?;
    Ok(ApiJson(LogItem::from(row)))
}
