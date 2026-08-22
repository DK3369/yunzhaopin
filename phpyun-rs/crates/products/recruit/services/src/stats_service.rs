//! Site statistics (aligned with PHPYun `tongji` / `ajax::*Data`).
//!
//! Only lightweight COUNT aggregations are computed and dispatched concurrently; heavy chart aggregations should be precomputed offline and stored in Kv.

use phpyun_core::{clock, AppResult, AppState};
use phpyun_models::stats::repo as stats_repo;

#[derive(Debug, Default)]
pub struct Overview {
    pub total_jobs: u64,
    pub total_companies: u64,
    pub total_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

fn today_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

pub async fn overview(state: &AppState) -> AppResult<Overview> {
    let db = state.db.reader();
    let t0 = today_ts(clock::now_ts());
    let (jobs, coms, res, new_j, new_r) = tokio::join!(
        stats_repo::count_active_jobs(db),
        stats_repo::count_active_companies(db),
        stats_repo::count_active_resumes(db),
        stats_repo::count_jobs_since(db, t0),
        stats_repo::count_resumes_since(db, t0),
    );
    Ok(Overview {
        total_jobs: jobs.unwrap_or(0),
        total_companies: coms.unwrap_or(0),
        total_resumes: res.unwrap_or(0),
        today_new_jobs: new_j.unwrap_or(0),
        today_new_resumes: new_r.unwrap_or(0),
    })
}
