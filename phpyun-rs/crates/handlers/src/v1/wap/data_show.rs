//! Data-show charts (aligned with PHPYun `wap/ajax::{cityData,ageData,sexData,eduData,
//! expData,userHyChart,userRegChart,comcityData,comgmData,comxzData,
//! comLogChart,comJobChart}`).
//!
//! All endpoints are **public GET** — PHP had no auth here; the data is anonymized aggregates.
//! Paths are consolidated under the `/v1/wap/data-show/*` subtree.

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::data_show_service::{self, DistItem, TimePoint};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/data-show/resume-sex", post(resume_sex))
        .route("/data-show/resume-edu", post(resume_edu))
        .route("/data-show/resume-exp", post(resume_exp))
        .route("/data-show/resume-age", post(resume_age))
        .route("/data-show/resume-city", post(resume_city))
        .route("/data-show/company-city", post(company_city))
        .route("/data-show/company-scale", post(company_scale))
        .route("/data-show/company-property", post(company_property))
        .route("/data-show/user-register-trend", post(user_register_trend))
        .route("/data-show/company-job-trend", post(company_job_trend))
        .route("/data-show/company-login-trend", post(company_login_trend))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct CityLevelQuery {
    /// 1=province / 2=city / 3=district (default 2)
    #[serde(default = "default_level")]
    #[validate(range(min = 0, max = 99))]
    pub level: i32,
}
fn default_level() -> i32 {
    2
}

// ==================== Resume distribution ====================

#[utoipa::path(post, path = "/v1/wap/data-show/resume-sex", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn resume_sex(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::resume_sex_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/resume-edu", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn resume_edu(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::resume_edu_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/resume-exp", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn resume_exp(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::resume_exp_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

/// Age distribution. Returns 4 buckets: key=0(16-24) / 1(25-30) / 2(31-40) / 3(41-65)
#[utoipa::path(post, path = "/v1/wap/data-show/resume-age", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn resume_age(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::resume_age_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(
    post,
    path = "/v1/wap/data-show/resume-city",
    tag = "wap",
    params(CityLevelQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn resume_city(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<CityLevelQuery>,
) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::resume_city_distribution(&state, q.level).await?;
    Ok(ApiJson((*list).clone()))
}

// ==================== Company distribution ====================

#[utoipa::path(
    post,
    path = "/v1/wap/data-show/company-city",
    tag = "wap",
    params(CityLevelQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn company_city(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<CityLevelQuery>,
) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::company_city_distribution(&state, q.level).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/company-scale", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn company_scale(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::company_scale_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/company-property", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn company_property(State(state): State<AppState>) -> AppResult<ApiJson<Vec<DistItem>>> {
    let list = data_show_service::company_property_distribution(&state).await?;
    Ok(ApiJson((*list).clone()))
}

// ==================== Time series ====================

#[utoipa::path(post, path = "/v1/wap/data-show/user-register-trend", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn user_register_trend(State(state): State<AppState>) -> AppResult<ApiJson<Vec<TimePoint>>> {
    let list = data_show_service::user_register_trend(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/company-job-trend", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn company_job_trend(State(state): State<AppState>) -> AppResult<ApiJson<Vec<TimePoint>>> {
    let list = data_show_service::company_job_publish_trend(&state).await?;
    Ok(ApiJson((*list).clone()))
}

#[utoipa::path(post, path = "/v1/wap/data-show/company-login-trend", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn company_login_trend(State(state): State<AppState>) -> AppResult<ApiJson<Vec<TimePoint>>> {
    let list = data_show_service::company_login_trend(&state).await?;
    Ok(ApiJson((*list).clone()))
}
