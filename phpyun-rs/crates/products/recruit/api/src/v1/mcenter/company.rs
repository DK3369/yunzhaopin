//! Member center - company (usertype=2).

use axum::{extract::State, routing::post, Router};
use phpyun_core::json;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::company_service::{self, CompanyUpdateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company", post(update_mine))
        .route("/company/list", post(get_mine))
        .route("/company/map", post(set_map))
        .route("/company/check", post(check_used))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompanyData {
    pub uid: u64,
    pub name: Option<String>,
    pub shortname: Option<String>,
    pub hy: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub logo: Option<String>,
    pub logo_status: i32,
    pub comqcode: Option<String>,
    pub content: Option<String>,
    pub linkman: Option<String>,
    pub linkjob: Option<String>,
    pub linkphone: Option<String>,
    pub linktel: Option<String>,
    pub linkmail: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub linkqq: Option<String>,
    pub sdate: Option<String>,
    pub money: i32,
    pub moneytype: i32,
    pub infostatus: i32,
    pub welfare: Option<String>,
    pub busstops: Option<String>,
    pub not_disturb: Option<String>,
    pub r_status: i32,
    pub yyzz_status: i32,
    pub moblie_status: i32,
    pub email_status: i32,
    pub hits: i32,
    pub x: Option<String>,
    pub y: Option<String>,
    pub pr: i32,
    pub mun: i32,
}

/// Get my company profile
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CompanyData))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<CompanyData>> {
    let c = company_service::get_mine(&state, &user).await?;
    Ok(ApiResponse::data(CompanyData {
        uid: c.uid,
        name: c.name,
        shortname: c.shortname,
        hy: c.hy,
        provinceid: c.provinceid,
        cityid: c.cityid,
        three_cityid: c.three_cityid,
        logo: c.logo,
        logo_status: c.logo_status,
        comqcode: c.comqcode,
        content: c.content,
        linkman: c.linkman,
        linkjob: c.linkjob,
        linkphone: c.linkphone,
        linktel: c.linktel,
        linkmail: c.linkmail,
        address: c.address,
        website: c.website,
        linkqq: c.linkqq,
        sdate: c.sdate,
        money: c.money,
        moneytype: c.moneytype,
        infostatus: c.infostatus,
        welfare: c.welfare,
        busstops: c.busstops,
        not_disturb: c.not_disturb,
        r_status: c.r_status,
        yyzz_status: c.yyzz_status,
        moblie_status: c.moblie_status,
        email_status: c.email_status,
        hits: c.hits,
        x: c.x,
        y: c.y,
        pr: c.pr,
        mun: c.mun,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyForm {
    #[validate(length(min = 2, max = 25))]
    pub name: Option<String>,
    #[validate(length(max = 25))]
    pub shortname: Option<String>,
    #[validate(range(min = 0, max = 99_999))]
    pub hy: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub provinceid: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub cityid: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub three_cityid: Option<i32>,
    /// Upload `key` from `/v1/wap/upload/*`. `logo_status` is admin-only, not set here.
    #[validate(length(max = 255))]
    pub logo: Option<String>,
    #[validate(length(max = 255))]
    pub comqcode: Option<String>,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    #[validate(length(max = 50))]
    pub linkman: Option<String>,
    #[validate(length(max = 50))]
    pub linkjob: Option<String>,
    #[validate(length(max = 20))]
    pub linkphone: Option<String>,
    #[validate(length(max = 20))]
    pub linktel: Option<String>,
    #[validate(email)]
    pub linkmail: Option<String>,
    #[validate(length(max = 100))]
    pub address: Option<String>,
    #[validate(length(max = 100))]
    pub website: Option<String>,
    #[validate(length(max = 20))]
    pub linkqq: Option<String>,
    #[validate(length(max = 20))]
    pub sdate: Option<String>,
    #[validate(range(min = 0, max = 99_999_999))]
    pub money: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub moneytype: Option<i32>,
    #[validate(range(min = 0, max = 2))]
    pub infostatus: Option<i32>,
    #[validate(length(max = 500))]
    pub welfare: Option<String>,
    #[validate(length(max = 500))]
    pub busstops: Option<String>,
    #[validate(length(max = 20))]
    pub not_disturb: Option<String>,
    #[validate(length(max = 32))]
    pub x: Option<String>,
    #[validate(length(max = 32))]
    pub y: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub pr: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub mun: Option<i32>,
}

/// Update company profile
#[utoipa::path(
    post,
    path = "/v1/mcenter/company",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateCompanyForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateCompanyForm>,
) -> AppResult<ApiResponse<json::Value>> {
    company_service::update_mine(
        &state,
        &user,
        CompanyUpdateInput {
            name: f.name.as_deref(),
            shortname: f.shortname.as_deref(),
            hy: f.hy,
            provinceid: f.provinceid,
            cityid: f.cityid,
            three_cityid: f.three_cityid,
            logo: f.logo.as_deref(),
            comqcode: f.comqcode.as_deref(),
            content: f.content.as_deref(),
            linkman: f.linkman.as_deref(),
            linkjob: f.linkjob.as_deref(),
            linkphone: f.linkphone.as_deref(),
            linktel: f.linktel.as_deref(),
            linkmail: f.linkmail.as_deref(),
            address: f.address.as_deref(),
            website: f.website.as_deref(),
            linkqq: f.linkqq.as_deref(),
            sdate: f.sdate.as_deref(),
            money: f.money,
            moneytype: f.moneytype,
            infostatus: f.infostatus,
            welfare: f.welfare.as_deref(),
            busstops: f.busstops.as_deref(),
            not_disturb: f.not_disturb.as_deref(),
            x: f.x.as_deref(),
            y: f.y.as_deref(),
            pr: f.pr,
            mun: f.mun,
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetMapForm {
    #[validate(length(min = 1, max = 32))]
    pub x: String,
    #[validate(length(min = 1, max = 32))]
    pub y: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/map",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SetMapForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_map(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SetMapForm>,
) -> AppResult<ApiResponse<json::Value>> {
    company_service::set_map(&state, &user, &f.x, &f.y, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CompanyCheckForm {
    #[validate(length(min = 1, max = 16))]
    pub type_str: String,
    #[validate(length(min = 1, max = 80))]
    pub check_str: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/check",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CompanyCheckForm,
    responses((status = 200, description = "ok"))
)]
pub async fn check_used(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CompanyCheckForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let used = company_service::check_used(&state, &user, &f.type_str, &f.check_str).await?;
    Ok(ApiResponse::data(json::json!({
        "used": used,
        "type": f.type_str,
    })))
}
