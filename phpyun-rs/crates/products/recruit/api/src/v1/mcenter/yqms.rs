//! PHP `addYqms` from a public resume — writes `userid_msg`, no `apply_id`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::yqms_service::{self, YqmsInput, YqmsResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company/yqms/create", post(create))
        .route("/yqms/list", post(list_mine))
        .route("/yqms/accept", post(accept))
        .route("/yqms/reject", post(reject))
        .route("/yqms/delete", post(delete_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct YqmsForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub seeker_uid: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub job_id: u64,
    #[serde(default)]
    #[validate(length(max = 5000))]
    pub content: String,
    #[validate(length(min = 1, max = 300))]
    pub address: String,
    #[validate(length(min = 1, max = 64))]
    pub intertime: String,
    #[validate(length(min = 1, max = 64))]
    pub linkman: String,
    #[validate(length(min = 6, max = 32))]
    pub linktel: String,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub longitude: String,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub latitude: String,
    #[serde(default)]
    #[validate(length(max = 255))]
    pub mappic: String,
    #[serde(default)]
    pub save_yqmb: bool,
    #[serde(default)]
    pub ymid: u64,
    /// PHP second-step confirm for integral/cash single purchase.
    #[serde(default)]
    pub confirm: bool,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/yqms/create",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = YqmsForm,
    responses((status = 200, description = "ok"))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<YqmsForm>,
) -> AppResult<ApiResponse<YqmsResult>> {
    let mappic = if f.mappic.trim().is_empty() {
        None
    } else {
        Some(f.mappic.as_str())
    };
    let result = yqms_service::create_from_resume(
        &state,
        &user,
        YqmsInput {
            seeker_uid: f.seeker_uid,
            job_id: f.job_id,
            content: &f.content,
            address: &f.address,
            intertime: &f.intertime,
            linkman: &f.linkman,
            linktel: &f.linktel,
            longitude: &f.longitude,
            latitude: &f.latitude,
            mappic,
            save_yqmb: f.save_yqmb,
            ymid: f.ymid,
        },
        f.confirm,
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(result))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct YqmsItem {
    pub id: u64,
    pub uid: u64,
    pub fid: u64,
    pub fname: String,
    pub job_id: u64,
    pub job_name: String,
    pub title: String,
    pub content: String,
    pub address: String,
    pub intertime: String,
    pub linkman: String,
    pub linktel: String,
    pub is_browse: i32,
    pub datetime: i64,
    pub datetime_n: String,
    pub remark: String,
}

impl From<phpyun_models::userid_msg::entity::UseridMsg> for YqmsItem {
    fn from(r: phpyun_models::userid_msg::entity::UseridMsg) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            fid: r.fid,
            fname: r.fname,
            job_id: r.jobid,
            job_name: r.jobname,
            title: r.title,
            content: r.content,
            address: r.address,
            intertime: r.intertime,
            linkman: r.linkman,
            linktel: r.linktel,
            is_browse: r.is_browse,
            datetime_n: fmt_dt(r.datetime),
            datetime: r.datetime,
            remark: r.remark,
        }
    }
}

/// PHP `member/user/invite` — interview invitations in `userid_msg`.
#[utoipa::path(
    post,
    path = "/v1/mcenter/yqms/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<YqmsItem>>> {
    let r = yqms_service::list_mine(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(YqmsItem::from).collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct YqmsRejectForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub remark: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/yqms/accept",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn accept(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    yqms_service::respond(&state, &user, b.id, 3, "", &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "is_browse": 3 })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/yqms/reject",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = YqmsRejectForm,
    responses((status = 200, description = "ok"))
)]
pub async fn reject(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<YqmsRejectForm>,
) -> AppResult<ApiResponse<json::Value>> {
    yqms_service::respond(&state, &user, f.id, 4, &f.remark, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "is_browse": 4 })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/yqms/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = yqms_service::hide_mine(&state, &user, b.id).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}
