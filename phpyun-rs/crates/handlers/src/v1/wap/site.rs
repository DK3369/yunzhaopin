//! Site static pages (about / privacy / protocol / contact / appDown).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::error::InfraError;
use phpyun_core::{ApiJson, AppError, AppResult, AppState};
use phpyun_services::site_page_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/site/pages/{code}", get(get_page))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SitePageView {
    pub code: String,
    pub title: String,
    pub content: String,
    pub updated_at: i64,
}

/// Site page
#[utoipa::path(
    get,
    path = "/v1/wap/site/pages/{code}",
    tag = "wap",
    params(("code" = String, Path, description = "about/privacy/protocol/contact/appDown")),
    responses((status = 200, description = "ok", body = SitePageView), (status = 404))
)]
pub async fn get_page(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<ApiJson<SitePageView>> {
    let p = site_page_service::get(&state, &code)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("page_not_found".into())))?;
    Ok(ApiJson(SitePageView {
        code: p.code,
        title: p.title,
        content: p.content,
        updated_at: p.updated_at,
    }))
}
