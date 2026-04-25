//! Resume completeness score.

use axum::{extract::State, routing::get, Router};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser};
use phpyun_services::resume_score_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/resume/completion", get(completion))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Completion {
    pub score: u8,
    pub missing: Vec<String>,
}

/// My resume completeness
#[utoipa::path(
    get,
    path = "/v1/mcenter/resume/completion",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = Completion))
)]
pub async fn completion(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Completion>> {
    let r = resume_score_service::compute(&state, &user).await?;
    Ok(ApiJson(Completion {
        score: r.score,
        missing: r.missing.into_iter().map(|s| s.to_string()).collect(),
    }))
}
