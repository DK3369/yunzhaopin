//! 顾问投诉（PHP `report.htm`，`phpyun_report.type=2`）。与违规举报 `/reports` 分开。

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    json, ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination,
    ValidatedJson,
};
use phpyun_services::report_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/crm-reports", post(submit))
        .route("/crm-reports/list", post(list_mine))
        .route("/crm-reports/delete", post(delete_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrmReportForm {
    #[validate(length(min = 1, max = 200))]
    pub reason: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/crm-reports",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CrmReportForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CrmReportForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = report_service::submit_crm(&state, &user, &f.reason, &ip).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CrmReportView {
    pub id: u64,
    pub eid: u64,
    pub r_name: String,
    pub username: String,
    pub r_reason: String,
    pub result: Option<String>,
    pub status: i32,
    pub inputtime: i64,
    pub inputtime_n: String,
}

impl From<phpyun_models::report::entity::CrmReport> for CrmReportView {
    fn from(r: phpyun_models::report::entity::CrmReport) -> Self {
        Self {
            id: r.id,
            eid: r.eid,
            r_name: r.r_name,
            username: r.username,
            r_reason: r.r_reason,
            result: r.result,
            status: r.status,
            inputtime_n: fmt_dt(r.inputtime),
            inputtime: r.inputtime,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/crm-reports/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<CrmReportView>>> {
    let r = report_service::list_crm(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(CrmReportView::from).collect(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/crm-reports/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = report_service::delete_crm(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "affected": n })))
}
