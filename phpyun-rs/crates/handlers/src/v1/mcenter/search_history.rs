//! Personal search history (matching PHPYun `search::history`).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser};
use phpyun_services::search_history_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/search-history", get(list).post(clear))
        .route("/search-history/{id}", post(remove))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub scope: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u64,
}
fn default_limit() -> u64 { 20 }

#[derive(Debug, Deserialize, IntoParams)]
pub struct ClearQuery {
    pub scope: Option<String>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HistoryItem {
    pub id: u64,
    pub uid: u64,
    pub scope: String,
    pub keyword: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::search_history::entity::SearchHistory> for HistoryItem {
    fn from(h: phpyun_models::search_history::entity::SearchHistory) -> Self {
        Self {
            id: h.id,
            uid: h.uid,
            scope: h.scope,
            keyword: h.keyword,
            created_at_n: fmt_dt(h.created_at),
            created_at: h.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClearResult {
    pub removed: u64,
}

/// My search history
#[utoipa::path(
    get,
    path = "/v1/mcenter/search-history",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Vec<HistoryItem>>> {
    let list = search_history_service::list(&state, &user, q.scope.as_deref(), q.limit).await?;
    Ok(ApiJson(list.into_iter().map(HistoryItem::from).collect()))
}

/// Clear search history (optionally filtered by scope)
#[utoipa::path(
    post,
    path = "/v1/mcenter/search-history",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ClearQuery),
    responses((status = 200, description = "ok", body = ClearResult))
)]
pub async fn clear(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<ClearQuery>,
) -> AppResult<ApiJson<ClearResult>> {
    let n = search_history_service::clear(&state, &user, q.scope.as_deref()).await?;
    Ok(ApiJson(ClearResult { removed: n }))
}

/// Delete a single entry
#[utoipa::path(
    post,
    path = "/v1/mcenter/search-history/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    search_history_service::delete_one(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}
