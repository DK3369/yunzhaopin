//! Map-based search (aligned with PHPYun `wap/map`).
//!
//! Longitude/latitude as `x` and `y`; radius capped at 50km to prevent full-table scans.

use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::geo::repo::{self as geo_repo, CompanyNear, JobNear};

const MAX_RADIUS_KM: f64 = 50.0;
const DEFAULT_LIMIT: u64 = 50;

fn validate(x: f64, y: f64, radius_km: f64) -> AppResult<()> {
    if !x.is_finite()
        || !y.is_finite()
        || !(-180.0..=180.0).contains(&x)
        || !(-90.0..=90.0).contains(&y)
    {
        return Err(ApiError::param_invalid("bad_coords"));
    }
    if !radius_km.is_finite() || radius_km <= 0.0 || radius_km > MAX_RADIUS_KM {
        return Err(ApiError::param_invalid("bad_radius"));
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

pub fn default_limit() -> u64 {
    DEFAULT_LIMIT
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn rejects_non_finite_coordinates_and_radius() {
        for (x, y) in [
            (f64::NAN, 0.0),
            (f64::INFINITY, 0.0),
            (0.0, f64::NEG_INFINITY),
        ] {
            let error = validate(x, y, 5.0).unwrap_err();
            assert_eq!(error.code(), 400);
            assert!(error.tag().contains("bad_coords"));
        }

        for radius in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = validate(0.0, 0.0, radius).unwrap_err();
            assert_eq!(error.code(), 400);
            assert!(error.tag().contains("bad_radius"));
        }
    }
}
