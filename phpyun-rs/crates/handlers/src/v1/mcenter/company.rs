//! Member center - company (usertype=2).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::company_service::{self, CompanyUpdateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/company", post(update_mine))
        .route("/company/list", post(get_mine))
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
    pub content: Option<String>,
    pub linkman: Option<String>,
    pub linkjob: Option<String>,
    pub linkphone: Option<String>,
    pub linkmail: Option<String>,
    pub r_status: i32,
    pub hits: i32,
}

/// Get my company profile
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CompanyData))
)]pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<CompanyData>> {
    let c = company_service::get_mine(&state, &user).await?;
    Ok(ApiJson(CompanyData {
        uid: c.uid,
        name: c.name,
        shortname: c.shortname,
        hy: c.hy,
        provinceid: c.provinceid,
        cityid: c.cityid,
        three_cityid: c.three_cityid,
        logo: c.logo,
        logo_status: c.logo_status,
        content: c.content,
        linkman: c.linkman,
        linkjob: c.linkjob,
        linkphone: c.linkphone,
        linkmail: c.linkmail,
        r_status: c.r_status,
        hits: c.hits,
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
    #[validate(length(max = 255))]
    pub logo: Option<String>,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    #[validate(length(max = 50))]
    pub linkman: Option<String>,
    #[validate(length(max = 50))]
    pub linkjob: Option<String>,
    #[validate(length(max = 20))]
    pub linkphone: Option<String>,
    #[validate(email)]
    pub linkmail: Option<String>,
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
) -> AppResult<ApiJson<json::Value>> {
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
            content: f.content.as_deref(),
            linkman: f.linkman.as_deref(),
            linkjob: f.linkjob.as_deref(),
            linkphone: f.linkphone.as_deref(),
            linkmail: f.linkmail.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
