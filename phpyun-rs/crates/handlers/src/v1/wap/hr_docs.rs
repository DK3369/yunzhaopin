//! HR toolbox public read.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedQuery};
use phpyun_services::hr_doc_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/hr-docs", get(list))
        .route("/hr-docs/{id}", get(detail))
        .route("/hr-docs/{id}/download", post(track_download))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HrQuery {
    pub cid: Option<u64>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// HR document list item -- all 9 columns of phpyun_hr_doc + body excerpt + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct HrSummary {
    pub id: u64,
    pub cid: u64,
    pub name: String,
    pub url: String,
    /// First 100 chars of body (PHP list preview)
    pub body_excerpt: String,
    pub is_show: i32,
    pub hits: u32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::hr_doc::entity::HrDoc> for HrSummary {
    fn from(d: phpyun_models::hr_doc::entity::HrDoc) -> Self {
        let body_excerpt: String = d.body.chars().take(100).collect();
        Self {
            id: d.id,
            cid: d.cid,
            name: d.name,
            url: d.url,
            body_excerpt,
            is_show: d.is_show,
            hits: d.hits,
            created_at_n: fmt_dt(d.created_at),
            created_at: d.created_at,
            updated_at_n: fmt_dt(d.updated_at),
            updated_at: d.updated_at,
        }
    }
}

/// HR document detail -- all fields (including the full body content).
#[derive(Debug, Serialize, ToSchema)]
pub struct HrDetail {
    pub id: u64,
    pub cid: u64,
    pub name: String,
    pub url: String,
    pub body: String,
    pub is_show: i32,
    pub hits: u32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::hr_doc::entity::HrDoc> for HrDetail {
    fn from(d: phpyun_models::hr_doc::entity::HrDoc) -> Self {
        Self {
            id: d.id,
            cid: d.cid,
            name: d.name,
            url: d.url,
            body: d.body,
            is_show: d.is_show,
            hits: d.hits,
            created_at_n: fmt_dt(d.created_at),
            created_at: d.created_at,
            updated_at_n: fmt_dt(d.updated_at),
            updated_at: d.updated_at,
        }
    }
}

/// HR toolbox list
#[utoipa::path(get, path = "/v1/wap/hr-docs", tag = "wap", params(HrQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<HrQuery>,
) -> AppResult<ApiJson<Paged<HrSummary>>> {
    let r = hr_doc_service::list(&state, q.cid, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(HrSummary::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// HR toolbox detail
#[utoipa::path(
    get,
    path = "/v1/wap/hr-docs/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = HrDetail))
)]
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<HrDetail>> {
    let d = hr_doc_service::get(&state, id).await?;
    Ok(ApiJson(HrDetail::from(d)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HrDownloadResp {
    /// CDN-resolved download URL the client should redirect to.
    pub url: String,
    /// Original raw URL stored on `phpyun_toolbox_doc.url`.
    pub raw_url: String,
    /// New `downnum` value after the increment.
    pub hits: u32,
}

/// Track a download click — counterpart of PHP `hr/index::ajax_action`.
/// Atomically `downnum +=1` then returns the file URL so the client can
/// redirect (PHP echoes `checkpic($row['url'])` directly; we wrap in JSON).
#[utoipa::path(
    post,
    path = "/v1/wap/hr-docs/{id}/download",
    tag = "wap",
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok", body = HrDownloadResp),
        (status = 404, description = "Not found"),
    )
)]
pub async fn track_download(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<HrDownloadResp>> {
    let _ = phpyun_models::hr_doc::repo::incr_hit(state.db.pool(), id).await?;
    let d = hr_doc_service::get(&state, id).await?;
    let web_base = state.config.web_base_url.as_deref();
    let url_n = state.storage.normalize_legacy_url(&d.url, web_base);
    Ok(ApiJson(HrDownloadResp {
        url: url_n,
        raw_url: d.url,
        hits: d.hits,
    }))
}
