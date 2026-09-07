//! 顾问推送简历记录（PHP `record.htm`）。

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdsBody;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    json, ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::entrust_record_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/entrust-records/list", post(list_mine))
        .route("/entrust-records/delete", post(delete_mine))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EntrustRecordView {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub jobid: u64,
    pub user_name: String,
    pub job_name: String,
    pub ctime: i64,
    pub ctime_n: String,
}

impl From<phpyun_models::entrust_record::EntrustRecord> for EntrustRecordView {
    fn from(r: phpyun_models::entrust_record::EntrustRecord) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            eid: r.eid,
            jobid: r.jobid,
            user_name: r.user_name,
            job_name: r.job_name,
            ctime_n: fmt_dt(r.ctime),
            ctime: r.ctime,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/entrust-records/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<EntrustRecordView>>> {
    let r = entrust_record_service::list_mine(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(EntrustRecordView::from).collect(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/entrust-records/delete",
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
    let n = entrust_record_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "affected": n })))
}
