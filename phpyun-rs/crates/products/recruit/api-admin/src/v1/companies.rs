//! PHP `user/company` 企业档案列表与 r_status；CSV 导出（Excel 可开）。

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
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
) -> AppResult<ApiResponse<Paged<AdminCompanyRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_companies(&state, q.r_status, q.keyword.as_deref(), page)
            .await?,
    ))
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
