//! Map-based search (aligned with PHPYun `wap/map`).
//!
//! Longitude/latitude as `x` and `y`; radius capped at 200km.
//! PHP job list has no circular `HAVING`; company list uses `distance < 20`.

use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::geo::repo::{self as geo_repo, CompanyNear, JobNear, NearQuery};

const MAX_RADIUS_KM: f64 = 200.0;
const DEFAULT_LIMIT: u64 = 10;

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

async fn near_query(
    state: &AppState,
    x: f64,
    y: f64,
    radius_km: f64,
    page: u32,
    page_size: u64,
    did: u32,
) -> AppResult<NearQuery> {
    validate(x, y, radius_km)?;
    let limit = page_size.clamp(1, 50);
    let page = page.max(1);
    let offset = u64::from(page.saturating_sub(1)).saturating_mul(limit);
    let now = clock::now_ts();
    let days = crate::site_gate_service::default_uptime_days(state, None, "sy_datacycle_job").await;
    let min_lastupdate = days
        .filter(|d| *d > 0)
        .map(|d| now.saturating_sub(i64::from(d) * 86_400))
        .unwrap_or(0);
    Ok(NearQuery {
        x,
        y,
        radius_km,
        now,
        limit,
        offset,
        did,
        min_lastupdate,
    })
}

pub struct NearPage<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

pub async fn jobs_near(
    state: &AppState,
    x: f64,
    y: f64,
    radius_km: f64,
    page: u32,
    page_size: u64,
    did: u32,
) -> AppResult<NearPage<JobNear>> {
    let q = near_query(state, x, y, radius_km, page, page_size, did).await?;
    let (list, total) = tokio::join!(
        geo_repo::list_jobs_near(state.db.reader(), q),
        geo_repo::count_jobs_near(state.db.reader(), q),
    );
    Ok(NearPage {
        list: list?,
        total: total?,
        page: page.max(1),
        page_size: q.limit as u32,
    })
}

pub async fn companies_near(
    state: &AppState,
    x: f64,
    y: f64,
    radius_km: f64,
    page: u32,
    page_size: u64,
    did: u32,
) -> AppResult<NearPage<CompanyNear>> {
    let q = near_query(state, x, y, radius_km, page, page_size, did).await?;
    let (list, total) = tokio::join!(
        geo_repo::list_companies_near(state.db.reader(), q),
        geo_repo::count_companies_near(state.db.reader(), q),
    );
    Ok(NearPage {
        list: list?,
        total: total?,
        page: page.max(1),
        page_size: q.limit as u32,
    })
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
