//! My resume activity timeline.

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser};
use phpyun_services::resume_timeline_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new().route("/resume/timeline", get(list))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TimelineQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
}
fn default_limit() -> usize {
    30
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TimelineItem {
    /// view / download / interview
    pub kind: String,
    pub ts: i64,
    pub actor_uid: u64,
    pub ref_id: u64,
}

/// My resume timeline (newest first)
#[utoipa::path(
    get,
    path = "/v1/mcenter/resume/timeline",
    tag = "mcenter",
    security(("bearer" = [])),
    params(TimelineQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<TimelineQuery>,
) -> AppResult<ApiJson<Vec<TimelineItem>>> {
    let list = resume_timeline_service::list(&state, &user, q.limit).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|e| TimelineItem {
                kind: e.kind.to_string(),
                ts: e.ts,
                actor_uid: e.actor_uid,
                ref_id: e.ref_id,
            })
            .collect(),
    ))
}
