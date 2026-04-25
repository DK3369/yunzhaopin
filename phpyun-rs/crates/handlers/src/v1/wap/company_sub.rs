//! Company sub-pages: products / news (aligned with PHPYun `company::productshow` / `company::newsshow`).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination};
use phpyun_services::company_sub_service;
use serde::Serialize;
use utoipa::ToSchema;

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies/{uid}/products", get(list_products))
        .route("/companies/{uid}/products/{id}", get(product_detail))
        .route("/companies/{uid}/news", get(list_news))
        .route("/companies/{uid}/news/{id}", get(news_detail))
}

/// Company product list item — all phpyun_company_product columns + CDN URL + formatted time.
#[derive(Debug, Serialize, ToSchema)]
pub struct ProductSummary {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    /// Summary (first 100 chars of body, truncated)
    pub body_excerpt: String,
    pub cover: String,
    pub cover_n: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl ProductSummary {
    pub fn from_with_ctx(
        p: phpyun_models::company_sub::entity::CompanyProduct,
        state: &AppState,
    ) -> Self {
        let body_excerpt: String = p.body.chars().take(100).collect();
        Self {
            cover_n: pic_n(state, &p.cover),
            id: p.id,
            uid: p.uid,
            title: p.title,
            body_excerpt,
            cover: p.cover,
            status: p.status,
            sort: p.sort,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
            updated_at_n: fmt_dt(p.updated_at),
            updated_at: p.updated_at,
        }
    }
}

/// Compatibility for legacy callers (no state) — cover_n keeps the original cover value.
impl From<phpyun_models::company_sub::entity::CompanyProduct> for ProductSummary {
    fn from(p: phpyun_models::company_sub::entity::CompanyProduct) -> Self {
        let body_excerpt: String = p.body.chars().take(100).collect();
        Self {
            id: p.id,
            uid: p.uid,
            title: p.title,
            body_excerpt,
            cover: p.cover.clone(),
            cover_n: p.cover,
            status: p.status,
            sort: p.sort,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
            updated_at_n: fmt_dt(p.updated_at),
            updated_at: p.updated_at,
        }
    }
}

/// Product detail — all fields (including full body content).
#[derive(Debug, Serialize, ToSchema)]
pub struct ProductDetail {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub cover: String,
    pub cover_n: String,
    pub body: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl ProductDetail {
    pub fn from_with_ctx(
        p: phpyun_models::company_sub::entity::CompanyProduct,
        state: &AppState,
    ) -> Self {
        Self {
            cover_n: pic_n(state, &p.cover),
            id: p.id,
            uid: p.uid,
            title: p.title,
            cover: p.cover,
            body: p.body,
            status: p.status,
            sort: p.sort,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
            updated_at_n: fmt_dt(p.updated_at),
            updated_at: p.updated_at,
        }
    }
}

/// Company news list item — all phpyun_company_news columns.
#[derive(Debug, Serialize, ToSchema)]
pub struct NewsSummary {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub summary: String,
    /// First 100 chars of body (used by PHP list preview)
    pub body_excerpt: String,
    pub status: i32,
    pub hits: u32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::company_sub::entity::CompanyNews> for NewsSummary {
    fn from(n: phpyun_models::company_sub::entity::CompanyNews) -> Self {
        let body_excerpt: String = n.body.chars().take(100).collect();
        Self {
            id: n.id,
            uid: n.uid,
            title: n.title,
            summary: n.summary,
            body_excerpt,
            status: n.status,
            hits: n.hits,
            created_at_n: fmt_dt(n.created_at),
            created_at: n.created_at,
            updated_at_n: fmt_dt(n.updated_at),
            updated_at: n.updated_at,
        }
    }
}

/// Company news detail — all fields (including full body content).
#[derive(Debug, Serialize, ToSchema)]
pub struct NewsDetail {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub status: i32,
    pub hits: u32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::company_sub::entity::CompanyNews> for NewsDetail {
    fn from(n: phpyun_models::company_sub::entity::CompanyNews) -> Self {
        Self {
            id: n.id,
            uid: n.uid,
            title: n.title,
            summary: n.summary,
            body: n.body,
            status: n.status,
            hits: n.hits,
            created_at_n: fmt_dt(n.created_at),
            created_at: n.created_at,
            updated_at_n: fmt_dt(n.updated_at),
            updated_at: n.updated_at,
        }
    }
}

/// Company product list
#[utoipa::path(get, path = "/v1/wap/companies/{uid}/products", tag = "wap", params(("uid" = u64, Path)), responses((status = 200, description = "ok")))]
pub async fn list_products(
    State(state): State<AppState>,
    Path(uid): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ProductSummary>>> {
    let r = company_sub_service::list_products(&state, uid, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|p| ProductSummary::from_with_ctx(p, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Company product detail
#[utoipa::path(get, path = "/v1/wap/companies/{uid}/products/{id}", tag = "wap",
    params(("uid" = u64, Path), ("id" = u64, Path)),
    responses((status = 200, description = "ok", body = ProductDetail), (status = 404)))]
pub async fn product_detail(
    State(state): State<AppState>,
    Path((uid, id)): Path<(u64, u64)>,
) -> AppResult<ApiJson<ProductDetail>> {
    let p = company_sub_service::get_product(&state, uid, id).await?;
    Ok(ApiJson(ProductDetail::from_with_ctx(p, &state)))
}

/// Company news list
#[utoipa::path(get, path = "/v1/wap/companies/{uid}/news", tag = "wap", params(("uid" = u64, Path)), responses((status = 200, description = "ok")))]
pub async fn list_news(
    State(state): State<AppState>,
    Path(uid): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<NewsSummary>>> {
    let r = company_sub_service::list_news(&state, uid, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(NewsSummary::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Company news detail
#[utoipa::path(get, path = "/v1/wap/companies/{uid}/news/{id}", tag = "wap",
    params(("uid" = u64, Path), ("id" = u64, Path)),
    responses((status = 200, description = "ok", body = NewsDetail), (status = 404)))]
pub async fn news_detail(
    State(state): State<AppState>,
    Path((uid, id)): Path<(u64, u64)>,
) -> AppResult<ApiJson<NewsDetail>> {
    let n = company_sub_service::get_news(&state, uid, id).await?;
    Ok(ApiJson(NewsDetail::from(n)))
}
