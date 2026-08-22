//! Points mall (public): item list / detail.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::{ApiResponse, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::integral_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/integral/items", post(list_items))
        .route("/integral/items/detail", post(item_detail))
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
#[utoipa::path(post, path = "/v1/wap/integral/items", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list_items(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<IntegralItemView>>> {
    let r = integral_service::list_items(&state, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

/// Points-mall item detail
#[utoipa::path(post,
    path = "/v1/wap/integral/items/detail",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = IntegralItemView))
)]
pub async fn item_detail(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<IntegralItemView>> {
    let id = b.id;
    let it = integral_service::get_item(&state, id).await?;
    Ok(ApiResponse::data(IntegralItemView::from(it)))
}
