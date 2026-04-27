//! Job-seeker resume templates: list + purchase + select (matching PHPYun `member/user/resumetpl`).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::resume_tpl_service;
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume-tpls", post(list))
        .route("/resume-tpls/buy", post(buy))
        .route("/resume-tpls/apply", post(apply))
}

fn pic_n(state: &AppState, raw: Option<&str>) -> String {
    state.storage.normalize_legacy_url(raw.unwrap_or(""), state.config.web_base_url.as_deref())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TplView {
    pub id: u64,
    pub name: String,
    pub pic: Option<String>,
    pub pic_n: String,
    pub price: i32,
    pub price_yuan: f64,
    pub status: i32,
    pub sort: i32,
}

impl TplView {
    pub fn from_with_ctx(t: phpyun_models::resume_tpl::entity::ResumeTpl, state: &AppState) -> Self {
        Self {
            pic_n: pic_n(state, t.pic.as_deref()),
            id: t.id,
            name: t.name,
            pic: t.pic,
            price_yuan: (t.price as f64) / 100.0,
            price: t.price,
            status: t.status,
            sort: t.sort,
        }
    }
}

impl From<phpyun_models::resume_tpl::entity::ResumeTpl> for TplView {
    fn from(t: phpyun_models::resume_tpl::entity::ResumeTpl) -> Self {
        Self {
            id: t.id,
            name: t.name,
            pic_n: t.pic.clone().unwrap_or_default(),
            pic: t.pic,
            price_yuan: (t.price as f64) / 100.0,
            price: t.price,
            status: t.status,
            sort: t.sort,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-tpls",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(State(state): State<AppState>) -> AppResult<ApiJson<Vec<TplView>>> {
    let list = resume_tpl_service::list(&state).await?;
    Ok(ApiJson(list.into_iter().map(|t| TplView::from_with_ctx(t, &state)).collect()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BuyView {
    pub tpl_id: u64,
    pub already_owned: bool,
    pub deducted_price: i32,
}

#[utoipa::path(post,
    path = "/v1/mcenter/resume-tpls/buy",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok", body = BuyView))
)]
pub async fn buy(State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<BuyView>> {
    let id = b.id;
    let r = resume_tpl_service::buy(&state, &user, id, &ip).await?;
    Ok(ApiJson(BuyView {
        tpl_id: r.tpl_id,
        already_owned: r.already_owned,
        deducted_price: r.deducted_price,
    }))
}

#[utoipa::path(post,
    path = "/v1/mcenter/resume-tpls/apply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn apply(State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    let n = resume_tpl_service::apply(&state, &user, id, &ip).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

