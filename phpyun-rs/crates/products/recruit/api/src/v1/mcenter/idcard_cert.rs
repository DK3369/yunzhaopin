//! Jobseeker identity-card cert. PHP `upidcardInfo` → `phpyun_resume`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::utils::{fmt_dt, pic_n_str as pic_n};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::idcard_cert_service::{self, SubmitInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cert/idcard/status", post(status))
        .route("/cert/idcard/submit", post(submit))
}

fn status_name(s: i32) -> &'static str {
    match s {
        -1 => "none",
        0 => "pending",
        1 => "approved",
        2 => "rejected",
        _ => "unknown",
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IdcardView {
    /// `-1` 未提交（无图） / `0` 审核中 / `1` 通过 / `2` 驳回
    pub status: i32,
    pub status_n: String,
    pub statusbody: String,
    /// Masked id number
    pub idcard: String,
    pub idcard_pic: String,
    pub idcard_pic_n: String,
    pub name: String,
    pub cert_time: i64,
    pub cert_time_n: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdcardSubmitForm {
    #[validate(length(min = 15, max = 18))]
    pub idcard: String,
    #[serde(default)]
    #[validate(length(max = 25))]
    pub name: Option<String>,
    /// Key returned by `POST /v1/wap/upload/cert`
    #[validate(length(min = 1, max = 255))]
    pub idcard_pic: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/cert/idcard/status",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = IdcardView))
)]
pub async fn status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<IdcardView>> {
    let s = idcard_cert_service::status(&state, &user).await?;
    Ok(ApiResponse::data(IdcardView {
        status_n: status_name(s.status).to_string(),
        status: s.status,
        statusbody: s.statusbody,
        idcard: s.idcard,
        idcard_pic_n: if s.idcard_pic.is_empty() {
            String::new()
        } else {
            pic_n(&state, &s.idcard_pic)
        },
        idcard_pic: s.idcard_pic,
        name: s.name,
        cert_time_n: fmt_dt(s.cert_time),
        cert_time: s.cert_time,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/cert/idcard/submit",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdcardSubmitForm,
    responses((status = 200, description = "ok", body = IdcardView))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<IdcardSubmitForm>,
) -> AppResult<ApiResponse<IdcardView>> {
    let s = idcard_cert_service::submit(
        &state,
        &user,
        SubmitInput {
            idcard: &f.idcard,
            name: f.name.as_deref(),
            idcard_pic: &f.idcard_pic,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(IdcardView {
        status_n: status_name(s.status).to_string(),
        status: s.status,
        statusbody: s.statusbody,
        idcard: s.idcard,
        idcard_pic_n: if s.idcard_pic.is_empty() {
            String::new()
        } else {
            pic_n(&state, &s.idcard_pic)
        },
        idcard_pic: s.idcard_pic,
        name: s.name,
        cert_time_n: fmt_dt(s.cert_time),
        cert_time: s.cert_time,
    }))
}
