//! Company sub-page management: products / news (requires `require_employer`).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::company_sub_service::{self, NewsInput, NewsUpdateInput, ProductInput, ProductUpdateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // products
        .route("/company/products", get(list_products).post(create_product))
        .route("/company/products/{id}", post(update_product))
        // news
        .route("/company/news", get(list_news).post(create_news))
        .route("/company/news/{id}", post(update_news))
}

// ---------- Products ----------

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnProduct {
    pub id: u64,
    pub title: String,
    pub cover: String,
    pub status: i32,
    pub sort: i32,
    pub updated_at: i64,
}

impl From<phpyun_models::company_sub::entity::CompanyProduct> for OwnProduct {
    fn from(p: phpyun_models::company_sub::entity::CompanyProduct) -> Self {
        Self {
            id: p.id,
            title: p.title,
            cover: p.cover,
            status: p.status,
            sort: p.sort,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProductForm {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub cover: String,
    #[validate(length(min = 1, max = 100_000))]
    pub body: String,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProductPatch {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 500))]
    pub cover: Option<String>,
    #[validate(length(min = 1, max = 100_000))]
    pub body: Option<String>,
    pub sort: Option<i32>,
    /// 0=offline 1=online
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// My product list
#[utoipa::path(get, path = "/v1/mcenter/company/products", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_products(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<OwnProduct>>> {
    user.require_employer()?;
    let r = company_sub_service::list_own_products(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OwnProduct::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Create product
#[utoipa::path(post, path = "/v1/mcenter/company/products", tag = "mcenter", security(("bearer" = [])), request_body = ProductForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create_product(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ProductForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_employer()?;
    let id = company_sub_service::create_product(
        &state,
        &user,
        ProductInput {
            title: &f.title,
            cover: &f.cover,
            body: &f.body,
            sort: f.sort,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete a product (body with `"status":2` triggers deletion)
#[utoipa::path(post, path = "/v1/mcenter/company/products/{id}", tag = "mcenter", security(("bearer" = [])), params(("id" = u64, Path)), request_body = ProductPatch, responses((status = 200, description = "ok")))]
pub async fn update_product(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<ProductPatch>,
) -> AppResult<ApiOk> {
    user.require_employer()?;
    if f.status == Some(2) {
        company_sub_service::delete_product(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    company_sub_service::update_product(
        &state,
        &user,
        id,
        ProductUpdateInput {
            title: f.title.as_deref(),
            cover: f.cover.as_deref(),
            body: f.body.as_deref(),
            sort: f.sort,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}

// ---------- News ----------

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnNews {
    pub id: u64,
    pub title: String,
    pub summary: String,
    pub status: i32,
    pub hits: u32,
    pub updated_at: i64,
}

impl From<phpyun_models::company_sub::entity::CompanyNews> for OwnNews {
    fn from(n: phpyun_models::company_sub::entity::CompanyNews) -> Self {
        Self {
            id: n.id,
            title: n.title,
            summary: n.summary,
            status: n.status,
            hits: n.hits,
            updated_at: n.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewsForm {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub summary: String,
    #[validate(length(min = 1, max = 100_000))]
    pub body: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewsPatch {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 500))]
    pub summary: Option<String>,
    #[validate(length(min = 1, max = 100_000))]
    pub body: Option<String>,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i32>,
}

/// My news list
#[utoipa::path(get, path = "/v1/mcenter/company/news", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_news(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<OwnNews>>> {
    user.require_employer()?;
    let r = company_sub_service::list_own_news(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OwnNews::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Create news
#[utoipa::path(post, path = "/v1/mcenter/company/news", tag = "mcenter", security(("bearer" = [])), request_body = NewsForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create_news(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NewsForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_employer()?;
    let id = company_sub_service::create_news(
        &state,
        &user,
        NewsInput {
            title: &f.title,
            summary: &f.summary,
            body: &f.body,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete a news entry (body with `"status":2` triggers deletion)
#[utoipa::path(post, path = "/v1/mcenter/company/news/{id}", tag = "mcenter", security(("bearer" = [])), params(("id" = u64, Path)), request_body = NewsPatch, responses((status = 200, description = "ok")))]
pub async fn update_news(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<NewsPatch>,
) -> AppResult<ApiOk> {
    user.require_employer()?;
    if f.status == Some(2) {
        company_sub_service::delete_news(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    company_sub_service::update_news(
        &state,
        &user,
        id,
        NewsUpdateInput {
            title: f.title.as_deref(),
            summary: f.summary.as_deref(),
            body: f.body.as_deref(),
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}
