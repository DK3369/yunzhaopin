//! Company skin settings: template (`comtpl`) + banner (`banner`).
//!
//! Aligned with PHPYun `member/com/comtpl` + `member/com/banner`.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::{company_banner_service, company_tpl_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{IdBody, IdsBody};
use phpyun_core::utils::{fmt_dt, pic_n};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Templates
        .route("/company-tpls", post(tpl_list))
        .route("/company-tpls/apply", post(tpl_apply))
        // Banners
        .route("/company-banners", post(banner_add))
        .route("/company-banners/list", post(banner_list))
        .route("/company-banners/delete", post(banner_delete))
        .route("/company-banners/update", post(banner_update))
}

// ==================== Template ====================

fn kind_name(k: i32) -> &'static str {
    match k { 1 => "integral", 2 => "balance", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TplView {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub pic: Option<String>,
    pub pic_n: String,
    /// 1=integral / 2=balance
    pub kind: i32,
    pub kind_n: String,
    pub price: i32,
    pub price_yuan: f64,
    pub status: i32,
    pub sort: i32,
}

impl TplView {
    pub fn from_with_ctx(t: phpyun_models::company_tpl::entity::CompanyTpl, state: &AppState) -> Self {
        Self {
            pic_n: pic_n(state, t.pic.as_deref()),
            id: t.id,
            name: t.name,
            url: t.url,
            pic: t.pic,
            kind_n: kind_name(t.r#type).to_string(),
            kind: t.r#type,
            price_yuan: (t.price as f64) / 100.0,
            price: t.price,
            status: t.status,
            sort: t.sort,
        }
    }
}

impl From<phpyun_models::company_tpl::entity::CompanyTpl> for TplView {
    fn from(t: phpyun_models::company_tpl::entity::CompanyTpl) -> Self {
        Self {
            id: t.id,
            name: t.name,
            url: t.url,
            pic_n: t.pic.clone().unwrap_or_default(),
            pic: t.pic,
            kind_n: kind_name(t.r#type).to_string(),
            kind: t.r#type,
            price_yuan: (t.price as f64) / 100.0,
            price: t.price,
            status: t.status,
            sort: t.sort,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company-tpls",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn tpl_list(State(state): State<AppState>) -> AppResult<ApiJson<Vec<TplView>>> {
    let list = company_tpl_service::list(&state).await?;
    Ok(ApiJson(list.into_iter().map(|t| TplView::from_with_ctx(t, &state)).collect()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplyView {
    pub tpl_id: u64,
    pub tpl_url: String,
    pub newly_purchased: bool,
    pub deducted_price: i32,
    /// 1=integral / 2=balance
    pub deducted_kind: i32,
}

#[utoipa::path(post,
    path = "/v1/mcenter/company-tpls/apply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = ApplyView),
        (status = 400, description = "Template not found or disabled / insufficient balance"),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn tpl_apply(State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<ApplyView>> {
    let id = b.id;
    let r = company_tpl_service::apply(&state, &user, id, &ip).await?;
    Ok(ApiJson(ApplyView {
        tpl_id: r.tpl_id,
        tpl_url: r.tpl_url,
        newly_purchased: r.newly_purchased,
        deducted_price: r.deducted_price,
        deducted_kind: r.deducted_kind,
    }))
}

// ==================== Banner ====================


#[derive(Debug, Serialize, ToSchema)]
pub struct BannerView {
    pub id: u64,
    pub uid: u64,
    pub pic: String,
    pub pic_n: String,
    pub link: Option<String>,
    pub sort: i32,
    pub addtime: i64,
    pub addtime_n: String,
}

impl BannerView {
    pub fn from_with_ctx(b: phpyun_models::company_banner::entity::CompanyBanner, state: &AppState) -> Self {
        Self {
            pic_n: pic_n(state, Some(&b.pic)),
            id: b.id,
            uid: b.uid,
            pic: b.pic,
            link: b.link,
            sort: b.sort,
            addtime_n: fmt_dt(b.addtime),
            addtime: b.addtime,
        }
    }
}

impl From<phpyun_models::company_banner::entity::CompanyBanner> for BannerView {
    fn from(b: phpyun_models::company_banner::entity::CompanyBanner) -> Self {
        Self {
            id: b.id,
            uid: b.uid,
            pic_n: b.pic.clone(),
            pic: b.pic,
            link: b.link,
            sort: b.sort,
            addtime_n: fmt_dt(b.addtime),
            addtime: b.addtime,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company-banners/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn banner_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<BannerView>>> {
    let list = company_banner_service::list_mine(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(|b| BannerView::from_with_ctx(b, &state)).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BannerAddForm {
    #[validate(length(min = 1, max = 512))]
    pub pic: String,
    #[validate(length(max = 512))]
    pub link: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
    /// Aligned with PHP `com_banner_num`. 0 = unlimited
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub max_per_company: u64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company-banners",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BannerAddForm,
    responses((status = 200, description = "ok"))
)]
pub async fn banner_add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<BannerAddForm>,
) -> AppResult<ApiJson<json::Value>> {
    let id = company_banner_service::add(
        &state,
        &user,
        &company_banner_service::AddInput {
            pic: &f.pic,
            link: f.link.as_deref(),
            sort: f.sort,
            max_per_company: f.max_per_company,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "id": id })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BannerUpdateForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 512))]
    pub pic: Option<String>,
    #[validate(length(max = 512))]
    pub link: Option<String>,
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
}

#[utoipa::path(post,
    path = "/v1/mcenter/company-banners/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BannerUpdateForm,
    responses((status = 200, description = "ok"))
)]
pub async fn banner_update(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BannerUpdateForm>) -> AppResult<ApiJson<json::Value>> {
    let id = f.id;
    let n = company_banner_service::update(
        &state,
        &user,
        id,
        &company_banner_service::UpdateInput {
            pic: f.pic.as_deref(),
            link: f.link.as_deref(),
            sort: f.sort,
        },
    )
    .await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company-banners/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn banner_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = company_banner_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

