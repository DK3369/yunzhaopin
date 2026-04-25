//! Hot search keywords (public endpoint).

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::hot_search_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new().route("/hot-searches", get(list))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct HotQuery {
    /// job / resume / company / article
    #[serde(default = "default_scope")]
    pub scope: String,
    #[serde(default = "default_limit")]
    pub limit: u64,
}
fn default_scope() -> String {
    "job".to_string()
}
fn default_limit() -> u64 {
    10
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Hot search keyword item — all 5 columns of phpyun_hot_search + formatted timestamp.
#[derive(Debug, Serialize, ToSchema)]
pub struct HotItem {
    pub id: u64,
    pub scope: String,
    pub keyword: String,
    pub hits: i32,
    pub last_hit_at: i64,
    pub last_hit_at_n: String,
}

impl From<phpyun_models::hot_search::entity::HotSearch> for HotItem {
    fn from(h: phpyun_models::hot_search::entity::HotSearch) -> Self {
        Self {
            id: h.id,
            scope: h.scope,
            keyword: h.keyword,
            hits: h.hits,
            last_hit_at_n: fmt_dt(h.last_hit_at),
            last_hit_at: h.last_hit_at,
        }
    }
}

/// Top N hot search keywords
#[utoipa::path(
    get,
    path = "/v1/wap/hot-searches",
    tag = "wap",
    params(HotQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<HotQuery>,
) -> AppResult<ApiJson<Vec<HotItem>>> {
    let list = hot_search_service::top(&state, &q.scope, q.limit).await?;
    Ok(ApiJson(list.iter().cloned().map(HotItem::from).collect()))
}
