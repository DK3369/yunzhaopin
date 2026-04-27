//! Share links (aligned with PHPYun `wap/job::share` / `wap/company::share` / `ajax::pubqrcode` etc.).
//!
//! Only returns URL strings; QR codes are rendered on the client with libraries (qrcode.js etc.).
//! The server does not render PNGs, avoiding the image dependency and reducing payload size.
//!
//! `base_url` is read from `state.config.web_base_url`.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{IdBody, UidBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/share/jobs", post(job_share))
        .route("/share/companies", post(company_share))
        .route("/share/resumes", post(resume_share))
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
#[utoipa::path(post, path = "/v1/wap/share/jobs", tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn job_share(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/jobs/{}", b.id));
    Ok(ApiJson(ShareUrl { kind: "job".into(), id: b.id, url }))
}

/// Company share link
#[utoipa::path(post, path = "/v1/wap/share/companies", tag = "wap",
    request_body = UidBody,
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn company_share(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/companies/{}", b.uid));
    Ok(ApiJson(ShareUrl { kind: "company".into(), id: b.uid, url }))
}

/// Public resume share link (non-token version — login required to view)
#[utoipa::path(post, path = "/v1/wap/share/resumes", tag = "wap",
    request_body = UidBody,
    responses((status = 200, description = "ok", body = ShareUrl)))]
pub async fn resume_share(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiJson<ShareUrl>> {
    let base = base_of(&state);
    let url = build_url(&base, &format!("/wap/resumes/{}", b.uid));
    Ok(ApiJson(ShareUrl { kind: "resume".into(), id: b.uid, url }))
}
