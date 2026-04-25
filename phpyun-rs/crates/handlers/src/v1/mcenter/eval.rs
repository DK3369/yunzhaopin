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
