//! Personal search history (matching PHPYun `search::history`).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::search_history_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{ClearResult, IdBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/search-history", post(clear))
        .route("/search-history/list", post(list))
        .route("/search-history/delete", post(remove))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 16))]
    pub scope: Option<String>,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_limit() -> u64 { 20 }

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ClearQuery {
    #[validate(length(max = 16))]
    pub scope: Option<String>,
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

/// My search history
#[utoipa::path(
    post,
    path = "/v1/mcenter/search-history/list",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
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
    ValidatedJson(q): ValidatedJson<ClearQuery>,
) -> AppResult<ApiJson<ClearResult>> {
    let n = search_history_service::clear(&state, &user, q.scope.as_deref()).await?;
    Ok(ApiJson(ClearResult { removed: n }))
}

/// Delete a single entry
#[utoipa::path(post,
    path = "/v1/mcenter/search-history",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    search_history_service::delete_one(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

