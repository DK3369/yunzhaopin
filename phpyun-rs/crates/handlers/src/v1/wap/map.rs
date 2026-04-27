//! Map search (aligned with PHPYun `wap/map`). Public.
//!
//! Query: `x`, `y` required (decimal degrees), `radius_km` default 5 / max 50, `limit` default 50 / max 200.
//!
//! NearJob / NearCompany fields align with the PHP map list page: full original-table columns + dict
//! translation (city / province) + CDN URL (logo / company logo) + distance conversion (km / m) + time
//! formatting.

use axum::{
    extract::{State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::map_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/map/jobs", post(jobs_near))
        .route("/map/companies", post(companies_near))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: Option<&str>) -> String {
    state.storage.normalize_legacy_url(
        raw.unwrap_or(""),
        state.config.web_base_url.as_deref(),
    )
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct GeoQuery {
    pub x: f64,
    pub y: f64,
    #[serde(default = "default_radius")]
    pub radius_km: f64,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_radius() -> f64 {
    5.0
}
fn default_limit() -> u64 {
    map_service::default_limit()
}

/// Nearby job item -- phpyun_company_job projection + dict + derived distance + time formatting.
#[derive(Debug, Serialize, ToSchema)]
pub struct NearJob {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub province_id: i32,
    pub province_name: String,
    pub city_id: i32,
    pub city_name: String,
    pub min_salary: i32,
    pub max_salary: i32,
    /// (min + max) / 2, convenient for front-end sorting
    pub salary_avg: i32,
    pub x: f64,
    pub y: f64,
    pub distance_km: f64,
    /// Meter precision (distance_km * 1000, rounded)
    pub distance_m: i64,
    pub lastupdate: i64,
    pub lastupdate_n: String,
}

/// Nearby company item -- phpyun_company projection + dict + CDN + derived distance.
#[derive(Debug, Serialize, ToSchema)]
pub struct NearCompany {
    pub uid: u64,
    pub name: Option<String>,
    pub city_id: i32,
    pub city_name: String,
    pub logo: Option<String>,
    pub logo_n: String,
    pub x: f64,
    pub y: f64,
    pub distance_km: f64,
    pub distance_m: i64,
}

/// Nearby jobs
#[utoipa::path(
    post,
    path = "/v1/wap/map/jobs",
    tag = "wap",
    params(GeoQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn jobs_near(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<GeoQuery>,
) -> AppResult<ApiJson<Vec<NearJob>>> {
    let list = map_service::jobs_near(&state, q.x, q.y, q.radius_km, q.limit).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|j| {
                let salary_avg = (j.minsalary + j.maxsalary) / 2;
                NearJob {
                    distance_m: (j.distance * 1000.0).round() as i64,
                    lastupdate_n: fmt_dt(j.lastupdate),
                    province_name: dicts.city(j.provinceid).to_string(),
                    city_name: dicts.city(j.cityid).to_string(),
                    id: j.id,
                    uid: j.uid,
                    name: j.name,
                    com_name: j.com_name,
                    province_id: j.provinceid,
                    city_id: j.cityid,
                    min_salary: j.minsalary,
                    max_salary: j.maxsalary,
                    salary_avg,
                    x: j.x,
                    y: j.y,
                    distance_km: j.distance,
                    lastupdate: j.lastupdate,
                }
            })
            .collect(),
    ))
}

/// Nearby companies
#[utoipa::path(
    post,
    path = "/v1/wap/map/companies",
    tag = "wap",
    params(GeoQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn companies_near(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<GeoQuery>,
) -> AppResult<ApiJson<Vec<NearCompany>>> {
    let list = map_service::companies_near(&state, q.x, q.y, q.radius_km, q.limit).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|c| NearCompany {
                logo_n: pic_n(&state, c.logo.as_deref()),
                distance_m: (c.distance * 1000.0).round() as i64,
                city_name: dicts.city(c.cityid).to_string(),
                uid: c.uid,
                name: c.name,
                city_id: c.cityid,
                logo: c.logo,
                x: c.x,
                y: c.y,
                distance_km: c.distance,
            })
            .collect(),
    ))
}
