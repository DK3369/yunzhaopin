//! Share links (aligned with PHPYun `wap/job::share` / `wap/company::share` / `ajax::pubqrcode` etc.).
//!
//! Only returns URL strings; QR codes are rendered on the client with libraries (qrcode.js etc.).
//! The server does not render PNGs, avoiding the image dependency and reducing payload size.
//!
//! `base_url` is read from `state.config.web_base_url`.

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/share/jobs/{id}", get(job_share))
        .route("/share/companies/{uid}", get(company_share))
        .route("/share/resumes/{uid}", get(resume_share)) // eid = 0 indicates the user's own uid
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ShareUrl {
    pub kind: String,
    pub id: u64,
    pub url: String,
}

fn build_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    format!("{base}{path}")
}

fn base_of(state: &AppState) -> String {
    state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://example.com".to_string())
}

/// Job share link
#[utoipa::path(get, path = "/v1/wap/share/jobs/{id}", tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn job_share(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/jobs/{id}"));
    Ok(ApiJson(ShareUrl { kind: "job".into(), id, url }))
}

/// Company share link
#[utoipa::path(get, path = "/v1/wap/share/companies/{uid}", tag = "wap",
    params(("uid" = u64, Path)),
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn company_share(
    State(state): State<AppState>,
    Path(uid): Path<u64>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/companies/{uid}"));
    Ok(ApiJson(ShareUrl { kind: "company".into(), id: uid, url }))
}

/// Public resume share link (non-token version — login required to view)
#[utoipa::path(get, path = "/v1/wap/share/resumes/{uid}", tag = "wap",
    params(("uid" = u64, Path)),
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn resume_share(
    State(state): State<AppState>,
    Path(uid): Path<u64>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/resumes/{uid}"));
    Ok(ApiJson(ShareUrl { kind: "resume".into(), id: uid, url }))
}
