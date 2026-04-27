//! Joint recruitment (aligned with PHPYun `wap/gongzhao`).

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedQuery};
use phpyun_services::gongzhao_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/gongzhao", get(list))
        .route("/gongzhao/{id}", get(detail))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    pub tag: Option<String>,
}

/// Joint recruitment list item -- all 11 columns of phpyun_gongzhao + CDN URL + formatted timestamps + tag array.
#[derive(Debug, Serialize, ToSchema)]
pub struct GzSummary {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub cover: String,
    pub cover_n: String,
    /// Raw CSV value
    pub tag: String,
    /// CSV split (commonly used on the PHP list page)
    pub tag_arr: Vec<String>,
    pub status: i32,
    pub view_count: u32,
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl GzSummary {
    pub fn from_with_ctx(
        g: phpyun_models::gongzhao::entity::Gongzhao,
        state: &AppState,
    ) -> Self {
        let tag_arr = g
            .tag
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self {
            cover_n: pic_n(state, &g.cover),
            id: g.id,
            title: g.title,
            description: g.description,
            cover: g.cover,
            tag: g.tag,
            tag_arr,
            status: g.status,
            view_count: g.view_count,
            start_at_n: fmt_date(g.start_at),
            start_at: g.start_at,
            end_at_n: fmt_date(g.end_at),
            end_at: g.end_at,
            created_at_n: fmt_dt(g.created_at),
            created_at: g.created_at,
        }
    }
}

impl From<phpyun_models::gongzhao::entity::Gongzhao> for GzSummary {
    fn from(g: phpyun_models::gongzhao::entity::Gongzhao) -> Self {
        let tag_arr = g
            .tag
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self {
            id: g.id,
            title: g.title,
            description: g.description,
            cover: g.cover.clone(),
            cover_n: g.cover,
            tag: g.tag,
            tag_arr,
            status: g.status,
            view_count: g.view_count,
            start_at_n: fmt_date(g.start_at),
            start_at: g.start_at,
            end_at_n: fmt_date(g.end_at),
            end_at: g.end_at,
            created_at_n: fmt_dt(g.created_at),
            created_at: g.created_at,
        }
    }
}

/// Joint recruitment detail -- all Summary fields + body.
#[derive(Debug, Serialize, ToSchema)]
pub struct GzDetail {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub cover: String,
    pub cover_n: String,
    pub body: String,
    pub tag: String,
    pub tag_arr: Vec<String>,
    pub status: i32,
    pub view_count: u32,
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl GzDetail {
    pub fn from_with_ctx(
        g: phpyun_models::gongzhao::entity::Gongzhao,
        state: &AppState,
    ) -> Self {
        let tag_arr = g
            .tag
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self {
            cover_n: pic_n(state, &g.cover),
            id: g.id,
            title: g.title,
            description: g.description,
            cover: g.cover,
            body: g.body,
            tag: g.tag,
            tag_arr,
            status: g.status,
            view_count: g.view_count,
            start_at_n: fmt_date(g.start_at),
            start_at: g.start_at,
            end_at_n: fmt_date(g.end_at),
            end_at: g.end_at,
            created_at_n: fmt_dt(g.created_at),
            created_at: g.created_at,
        }
    }
}

/// Joint recruitment list
#[utoipa::path(
    get,
    path = "/v1/wap/gongzhao",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ListQuery>,
) -> AppResult<ApiJson<Paged<GzSummary>>> {
    let r = gongzhao_service::list(&state, q.tag.as_deref(), page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|g| GzSummary::from_with_ctx(g, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Joint recruitment detail
#[utoipa::path(
    get,
    path = "/v1/wap/gongzhao/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = GzDetail))
)]
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<GzDetail>> {
    let g = gongzhao_service::get(&state, id).await?;
    Ok(ApiJson(GzDetail::from_with_ctx(g, &state)))
}
