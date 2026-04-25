//! Points mall (public): item list / detail.

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination};
use phpyun_services::integral_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/integral/items", get(list_items))
        .route("/integral/items/{id}", get(item_detail))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IntegralItemView {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub description: String,
    pub cost: u32,
    pub stock: i32,
}

impl From<phpyun_models::integral::entity::IntegralItem> for IntegralItemView {
    fn from(i: phpyun_models::integral::entity::IntegralItem) -> Self {
        Self {
            id: i.id,
            name: i.name,
            image: i.image,
            description: i.description,
            cost: i.cost,
            stock: i.stock,
        }
    }
}

/// List points-mall items
#[utoipa::path(get, path = "/v1/wap/integral/items", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list_items(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<IntegralItemView>>> {
    let r = integral_service::list_items(&state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(IntegralItemView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Points-mall item detail
#[utoipa::path(
    get,
    path = "/v1/wap/integral/items/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = IntegralItemView))
)]
pub async fn item_detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<IntegralItemView>> {
    let it = integral_service::get_item(&state, id).await?;
    Ok(ApiJson(IntegralItemView::from(it)))
}
