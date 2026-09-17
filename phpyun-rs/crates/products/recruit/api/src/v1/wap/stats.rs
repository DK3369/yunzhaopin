//! Site statistics (matching PHPYun `tongji` + `ajax::*Data`). Public viewing.

use axum::Router;
use phpyun_core::{AppResult, AppState};
use phpyun_services::stats_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
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

