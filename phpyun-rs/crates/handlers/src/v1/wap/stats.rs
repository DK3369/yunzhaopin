//! Site statistics (matching PHPYun `tongji` + `ajax::*Data`). Public viewing.

use axum::{extract::State, routing::get, Router};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::stats_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/stats/overview", get(overview))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewView {
    pub total_jobs: u64,
    pub total_companies: u64,
    pub total_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

/// Site overview statistics
#[utoipa::path(
    get,
    path = "/v1/wap/stats/overview",
    tag = "wap",
    responses((status = 200, description = "ok", body = OverviewView))
)]
pub async fn overview(
    State(state): State<AppState>,
) -> AppResult<ApiJson<OverviewView>> {
    let o = stats_service::overview(&state).await?;
    Ok(ApiJson(OverviewView {
        total_jobs: o.total_jobs,
        total_companies: o.total_companies,
        total_resumes: o.total_resumes,
        today_new_jobs: o.today_new_jobs,
        today_new_resumes: o.today_new_resumes,
    }))
}
