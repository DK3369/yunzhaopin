//! PHP `user/company` 企业档案列表与 r_status；CSV 导出（Excel 可开）。

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::{
    ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};

use crate::dto::AdminPaged;
use phpyun_models::company::repo::AdminCompanyRow;
use phpyun_services::admin_longtail_service::{self, CsvExport};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies", post(list))
        .route("/companies/status", post(set_status))
        .route("/companies/export", post(export_csv))
        .route("/companies/ratings", post(list_ratings))
        .route("/companies/rating", post(set_rating))
        .route("/companies/php-cache", post(php_cache))
        .route("/companies/php-add-form", post(php_add_form))
        .route("/companies/check-username", post(check_username))
        .route("/companies/check-com-name", post(check_com_name))
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ListQuery {
    pub r_status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/companies", tag = "admin", security(("bearer" = [])), request_body = ListQuery, responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminCompanyRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_longtail_service::list_companies(&state, q.r_status, q.keyword.as_deref(), page)
            .await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    pub r_status: i32,
}

#[utoipa::path(post, path = "/v1/admin/companies/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_longtail_service::set_company_r_status(&state, &user, f.uid, f.r_status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/companies/ratings", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_ratings(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<phpyun_models::company::repo::CompanyRatingOpt>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_company_ratings(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetRatingForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    #[validate(range(min = 1))]
    pub rating: i32,
}

#[utoipa::path(post, path = "/v1/admin/companies/rating", tag = "admin", security(("bearer" = [])), request_body = SetRatingForm, responses((status = 200, description = "ok")))]
pub async fn set_rating(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetRatingForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_longtail_service::set_company_rating(&state, &user, f.uid, f.rating).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/companies/export", tag = "admin", security(("bearer" = [])), request_body = ListQuery, responses((status = 200, description = "ok")))]
pub async fn export_csv(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<CsvExport>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::export_companies_csv(&state, q.r_status, q.keyword.as_deref())
            .await?,
    ))
}

/// PHP `company::getCache_action`.
#[utoipa::path(post, path = "/v1/admin/companies/php-cache", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_cache(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_longtail_service::company_php_cache(&state, &user).await?,
    ))
}

/// PHP `company::add_action` GET (form cache + mapurl). Create POST is a later batch.
#[utoipa::path(post, path = "/v1/admin/companies/php-add-form", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_add_form(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    let username = body
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if !username.is_empty() || body.get("submit").is_some() {
        return Err(ApiError::business("company_create_unmapped"));
    }
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let cities = phpyun_services::category_service::list(&state, "city").await?;
    let city_nodes: Vec<(u64, u64, String)> = cities
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let jobs = phpyun_services::category_service::list(&state, "job").await?;
    let job_nodes: Vec<(u64, u64, String)> = jobs
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let mut payload = phpyun_services::admin_dashboard_service::php_cache_payload(
        &job_nodes,
        &city_nodes,
        &dicts.comclass_by_variable("job_edu"),
        &dicts.comclass_by_variable("job_exp"),
    );
    let pr = dicts.comclass_by_variable("job_pr");
    let mun = dicts.comclass_by_variable("job_mun");
    let mut comclass_name = serde_json::Map::new();
    let mut job_pr = Vec::new();
    for (id, name) in &pr {
        job_pr.push(*id);
        comclass_name.insert(id.to_string(), serde_json::Value::String(name.clone()));
    }
    let mut job_mun = Vec::new();
    for (id, name) in &mun {
        job_mun.push(*id);
        comclass_name.insert(id.to_string(), serde_json::Value::String(name.clone()));
    }
    let hy = dicts.industry_all();
    let industry_index: Vec<i32> = hy.iter().map(|(id, _)| *id).collect();
    let mut industry_name = serde_json::Map::new();
    for (id, name) in hy {
        industry_name.insert(id.to_string(), serde_json::Value::String(name));
    }
    let cities = payload.get("city_types").cloned().unwrap_or(serde_json::json!([]));
    let cionly = if cities.as_array().map(|a| a.is_empty()).unwrap_or(true) {
        1
    } else {
        0
    };
    payload["cache"] = serde_json::json!({
        "cities": cities,
        "industry_index": industry_index,
        "industry_name": industry_name,
        "comdata": { "job_pr": job_pr, "job_mun": job_mun },
        "comclass_name": comclass_name,
    });
    let map_key = phpyun_models::site_setting::repo::find(state.db.reader(), "map_key")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let map_secret = phpyun_models::site_setting::repo::find(state.db.reader(), "map_secret")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    phpyun_services::admin_dashboard_service::attach_amap(&mut payload, &map_key, &map_secret);
    let com_rating = phpyun_models::site_setting::repo::find(state.db.reader(), "com_rating")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    payload["com_rating"] = serde_json::Value::String(com_rating);
    payload["cionly"] = serde_json::json!(cionly);
    Ok(ApiResponse::data(payload))
}

/// PHP `company::checkUsername_action` / `users_resume::checkUsername`.
#[utoipa::path(post, path = "/v1/admin/companies/check-username", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn check_username(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    let username = body
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    admin_longtail_service::check_member_username(&state, &user, username).await?;
    Ok(ApiResponse::message("ok"))
}

/// PHP `company::checkComName_action`.
#[utoipa::path(post, path = "/v1/admin/companies/check-com-name", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn check_com_name(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let name = body
        .get("companyName")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(ApiResponse::data(
        admin_longtail_service::check_com_name(&state, &user, name).await?,
    ))
}
