//! Map-based search (aligned with PHPYun `wap/map`).
//!
//! Longitude/latitude as `x` and `y`; radius capped at 50km to prevent full-table scans.

use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState};
use phpyun_models::geo::repo::{self as geo_repo, CompanyNear, JobNear};

const MAX_RADIUS_KM: f64 = 50.0;
const DEFAULT_LIMIT: u64 = 50;

fn validate(x: f64, y: f64, radius_km: f64) -> AppResult<()> {
    if !(-180.0..=180.0).contains(&x) || !(-90.0..=90.0).contains(&y) {
        return Err(AppError::new(InfraError::InvalidParam("bad_coords".into())));
    }
    if radius_km <= 0.0 || radius_km > MAX_RADIUS_KM {
        return Err(AppError::new(InfraError::InvalidParam("bad_radius".into())));
    }
    Ok(())
}

pub async fn jobs_near(
    state: &AppState,
    x: f64,
    y: f64,
    radius_km: f64,
    limit: u64,
) -> AppResult<Vec<JobNear>> {
    validate(x, y, radius_km)?;
    let limit = limit.clamp(1, 200);
    let now = clock::now_ts();
    Ok(geo_repo::list_jobs_near(state.db.reader(), x, y, radius_km, now, limit).await?)
}

pub async fn companies_near(
    state: &AppState,
    x: f64,
    y: f64,
    radius_km: f64,
    limit: u64,
) -> AppResult<Vec<CompanyNear>> {
    validate(x, y, radius_km)?;
    let limit = limit.clamp(1, 200);
    Ok(geo_repo::list_companies_near(state.db.reader(), x, y, radius_km, limit).await?)
}

pub fn default_limit() -> u64 { DEFAULT_LIMIT }
