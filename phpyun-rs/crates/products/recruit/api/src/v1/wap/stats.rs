//! Site statistics (matching PHPYun `tongji` + `ajax::*Data`). Public viewing.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState};
use phpyun_services::stats_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(deprecated)]
pub fn routes() -> Router<AppState> {
    Router::new().route("/stats/overview", post(overview))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SiteOverviewView {
    pub total_jobs: u64,
    pub total_companies: u64,
    pub total_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

pub(crate) async fn build_overview(state: &AppState) -> AppResult<SiteOverviewView> {
    let o = stats_service::overview(state).await?;
    Ok(SiteOverviewView {
        total_jobs: o.total_jobs,
        total_companies: o.total_companies,
        total_resumes: o.total_resumes,
        today_new_jobs: o.today_new_jobs,
        today_new_resumes: o.today_new_resumes,
    })
}

/// Site overview statistics
#[deprecated(note = "use /v1/wap/initjobs")]
#[utoipa::path(
    post,
    path = "/v1/wap/stats/overview",
    tag = "wap",
    description = "即将失效：请改用 GET/POST /v1/wap/initjobs（data.stats）",
    responses((status = 200, description = "ok", body = SiteOverviewView))
)]
pub async fn overview(State(state): State<AppState>) -> AppResult<ApiResponse<SiteOverviewView>> {
    Ok(ApiResponse::data(build_overview(&state).await?))
}
