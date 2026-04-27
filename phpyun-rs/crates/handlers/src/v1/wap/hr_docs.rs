//! HR toolbox public read.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::hr_doc_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/hr-docs", post(list))
        .route("/hr-docs/detail", post(detail))
        .route("/hr-docs/download", post(track_download))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HrQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub cid: Option<u64>,
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
#[utoipa::path(post, path = "/v1/wap/hr-docs/detail", tag = "wap", params(HrQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<HrQuery>,
) -> AppResult<ApiJson<Paged<HrSummary>>> {
    let r = hr_doc_service::list(&state, q.cid, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// HR toolbox detail
#[utoipa::path(post,
    path = "/v1/wap/hr-docs",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = HrDetail))
)]
pub async fn detail(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<HrDetail>> {
    let id = b.id;
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
#[utoipa::path(post,
    path = "/v1/wap/hr-docs/download",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = HrDownloadResp),
        (status = 404, description = "Not found"),
    )
)]
pub async fn track_download(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<HrDownloadResp>> {
    let id = b.id;
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

