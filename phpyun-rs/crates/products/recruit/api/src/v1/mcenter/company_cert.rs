//! Company certification (member side). PHP `getCert` / `saveCert`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::utils::{fmt_dt, pic_n_str as pic_n};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::company_cert_service::{self, CertMine, SubmitInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company/cert", post(submit))
        .route("/company/cert/list", post(get_mine))
}

fn cert_status_name(s: i32) -> &'static str {
    match s {
        -1 => "none",
        0 => "pending",
        1 => "approved",
        2 => "rejected",
        _ => "unknown",
    }
}

fn pic_or_empty(state: &AppState, raw: &str) -> String {
    if raw.trim().is_empty() {
        String::new()
    } else {
        pic_n(state, raw)
    }
}

/// PHP `getCert_action` payload for the member 企业资质 page.
#[derive(Debug, Serialize, ToSchema)]
pub struct CertView {
    pub uid: u64,
    /// `-1` none / `0` pending / `1` approved / `2` rejected
    pub status: i32,
    pub status_n: String,
    pub statusbody: String,
    pub company_name: String,
    pub social_credit: String,
    pub check: String,
    pub check_n: String,
    pub owner_cert: String,
    pub owner_cert_n: String,
    pub wt_cert: String,
    pub wt_cert_n: String,
    pub other_cert: String,
    pub other_cert_n: String,
    pub yyzz_status: i32,
    pub com_social_credit: i32,
    pub com_cert_owner: i32,
    pub com_cert_wt: i32,
    pub com_cert_other: i32,
    pub com_cert_status: i32,
    pub exa_cert_wt: String,
    pub pic_type: String,
    pub file_maxsize: String,
    pub review_tel: String,
    pub ctime: i64,
    pub ctime_n: String,
}

fn to_view(state: &AppState, c: CertMine) -> CertView {
    CertView {
        uid: c.uid,
        status_n: cert_status_name(c.status).to_string(),
        check_n: pic_or_empty(state, &c.check),
        owner_cert_n: pic_or_empty(state, &c.owner_cert),
        wt_cert_n: pic_or_empty(state, &c.wt_cert),
        other_cert_n: pic_or_empty(state, &c.other_cert),
        exa_cert_wt: pic_or_empty(state, &c.exa_cert_wt),
        ctime: c.ctime,
        ctime_n: fmt_dt(c.ctime),
        status: c.status,
        statusbody: c.statusbody,
        company_name: c.company_name,
        social_credit: c.social_credit,
        check: c.check,
        owner_cert: c.owner_cert,
        wt_cert: c.wt_cert,
        other_cert: c.other_cert,
        yyzz_status: c.yyzz_status,
        com_social_credit: c.com_social_credit,
        com_cert_owner: c.com_cert_owner,
        com_cert_wt: c.com_cert_wt,
        com_cert_other: c.com_cert_other,
        com_cert_status: c.com_cert_status,
        pic_type: c.pic_type,
        file_maxsize: c.file_maxsize,
        review_tel: c.review_tel,
    }
}

/// My certification status (always a payload, even when no row yet).
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/cert/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CertView))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<CertView>> {
    let c = company_cert_service::get_mine(&state, &user).await?;
    Ok(ApiResponse::data(to_view(&state, c)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitForm {
    #[serde(alias = "name")]
    #[validate(length(min = 1, max = 100))]
    pub company_name: String,
    #[serde(default)]
    #[validate(length(max = 18))]
    pub social_credit: String,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub check: String,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub owner_cert: String,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub wt_cert: String,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub other_cert: String,
}

/// Submit certification
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/cert",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SubmitForm,
    responses((status = 200, description = "ok"))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SubmitForm>,
) -> AppResult<ApiResponse> {
    let key = company_cert_service::submit(
        &state,
        &user,
        SubmitInput {
            company_name: &f.company_name,
            social_credit: &f.social_credit,
            check: &f.check,
            owner_cert: &f.owner_cert,
            wt_cert: &f.wt_cert,
            other_cert: &f.other_cert,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::message(key))
}
