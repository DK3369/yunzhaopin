//! Map search (aligned with PHPYun `wap/map`). Public.
//!
//! Query: `x`, `y` required (decimal degrees), `radius_km` default 5 / max 50, `limit` default 50 / max 200.
//!
//! NearJob / NearCompany fields align with the PHP map list page: full original-table columns + dict
//! translation (city / province) + CDN URL (logo / company logo) + distance conversion (km / m) + time
//! formatting.

use axum::{extract::State, routing::get, Router};
use phpyun_core::utils::{fmt_dt, pic_n};
use phpyun_core::{ApiResponse, AppResult, AppState, Paged, ValidatedJsonOrQuery};
use phpyun_services::map_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/map/jobs", "/v1/wap/map/companies"];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/map/jobs", get(jobs_near).post(jobs_near))
        .route("/map/companies", get(companies_near).post(companies_near))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct GeoQuery {
    pub x: f64,
    pub y: f64,
    #[serde(default = "default_radius")]
    pub radius_km: f64,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 50))]
    pub limit: u64,
    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 10_000))]
    pub page: u32,
    #[serde(default)]
    #[validate(range(max = 999))]
    pub did: u32,
}
fn default_radius() -> f64 {
    20.0
}
fn default_limit() -> u64 {
    map_service::default_limit()
}
fn default_page() -> u32 {
    1
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
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<GeoQuery>,
) -> AppResult<ApiResponse<Paged<NearJob>>> {
    let page = map_service::jobs_near(
        &state,
        q.x,
        q.y,
        q.radius_km,
        q.page,
        q.limit,
        q.did,
    )
    .await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let items = page
        .list
        .into_iter()
        .map(|j| -> AppResult<NearJob> {
            let salary_avg = (j.minsalary + j.maxsalary) / 2;
            let distance_m = phpyun_core::numeric::finite_f64_to_i64_db(
                j.distance * 1000.0,
                phpyun_core::numeric::FloatRounding::Round,
                "near_job.distance_m",
            )?;
            Ok(NearJob {
                distance_m,
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
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ApiResponse::data(Paged::new(
        items,
        page.total,
        page.page,
        page.page_size,
    )))
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
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<GeoQuery>,
) -> AppResult<ApiResponse<Paged<NearCompany>>> {
    let page = map_service::companies_near(
        &state,
        q.x,
        q.y,
        q.radius_km,
        q.page,
        q.limit,
        q.did,
    )
    .await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let items = page
        .list
        .into_iter()
        .map(|c| -> AppResult<NearCompany> {
            let distance_m = phpyun_core::numeric::finite_f64_to_i64_db(
                c.distance * 1000.0,
                phpyun_core::numeric::FloatRounding::Round,
                "near_company.distance_m",
            )?;
            Ok(NearCompany {
                logo_n: pic_n(&state, c.logo.as_deref()),
                distance_m,
                city_name: dicts.city(c.cityid).to_string(),
                uid: c.uid,
                name: c.name,
                city_id: c.cityid,
                logo: c.logo,
                x: c.x,
                y: c.y,
                distance_km: c.distance,
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ApiResponse::data(Paged::new(
        items,
        page.total,
        page.page,
        page.page_size,
    )))
}
