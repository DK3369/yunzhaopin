//! Assessment submission + my history.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::{CreatedId, IdBody}, ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::eval_service;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/eval-papers/submit", post(submit))
        .route("/eval-logs", post(list_logs))
        .route("/eval-logs/detail", post(get_log))
        .route("/eval-papers/messages/post", post(post_message))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// In the form `{"<question_id>": "<option_label>"}`. Capped at 120
    /// entries with key/value lengths bounded to prevent DoS via huge map.
    #[validate(custom(function = "validate_answers"))]
    pub answers: HashMap<String, String>,
}

fn validate_answers(
    answers: &HashMap<String, String>,
) -> Result<(), validator::ValidationError> {
    if answers.len() > 120 {
        return Err(validator::ValidationError::new("answers_too_many"));
    }
    for (k, v) in answers {
        if k.len() > 64 || v.len() > 256 {
            return Err(validator::ValidationError::new("answer_entry_too_long"));
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubmitResult {
    pub log_id: u64,
    pub score: i32,
}

/// Submit assessment answers
#[utoipa::path(post,
    path = "/v1/mcenter/eval-papers/submit",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SubmitForm,
    responses((status = 200, description = "ok", body = SubmitResult))
)]
pub async fn submit(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SubmitForm>) -> AppResult<ApiJson<SubmitResult>> {
    let id = f.id;
    let (log_id, score) = eval_service::submit(&state, &user, id, f.answers).await?;
    Ok(ApiJson(SubmitResult { log_id, score }))
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
    post,
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

// ==================== Leave a message on a paper ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaperMessageForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// Free-text comment, 1..=512 chars.
    #[validate(length(min = 1, max = 512))]
    pub message: String,
}

/// Leave a public message on an assessment paper. Counterpart of PHP
/// `evaluate/exampaper::message_action`. The list view lives at
/// `GET /v1/wap/eval-papers/{id}/messages`.
#[utoipa::path(post,
    path = "/v1/mcenter/eval-papers/messages/post",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PaperMessageForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn post_message(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PaperMessageForm>) -> AppResult<ApiJson<CreatedId>> {
    let id = f.id;
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
    Ok(ApiJson(CreatedId { id: new_id }))
}

// ==================== Single eval log detail ====================

/// Look up one of the caller's own assessment results — counterpart of PHP
/// `evaluate/exampaper::gradeshow_action` (the data slice; the `examinee`
/// sidebar lives at `/v1/wap/eval-papers/{id}/recent-examinees`).
#[utoipa::path(
    post,
    path = "/v1/mcenter/eval-logs/detail",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = LogItem),
        (status = 404, description = "Not found / not yours"),
    )
)]
pub async fn get_log(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<LogItem>> {
    let log_id = b.id;
    let row = phpyun_models::eval::repo::find_log_for_owner(state.db.reader(), log_id, user.uid)
        .await?
        .ok_or_else(|| {
            phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
                "log_not_found".into(),
            ))
        })?;
    Ok(ApiJson(LogItem::from(row)))
}
