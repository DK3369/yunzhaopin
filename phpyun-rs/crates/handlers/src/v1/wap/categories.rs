//! Dynamic category tree (public).

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::category_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories/{kind}", get(list))
        .route("/categories/{kind}/children", get(children))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CatNode {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub sort: i32,
}

impl From<phpyun_models::category::entity::Category> for CatNode {
    fn from(c: phpyun_models::category::entity::Category) -> Self {
        Self {
            id: c.id,
            parent_id: c.parent_id,
            name: c.name,
            sort: c.sort,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ChildrenQuery {
    #[serde(default)]
    pub parent_id: u64,
}

/// Get all categories under a kind (flat list with parent_id; client builds the tree)
#[utoipa::path(
    get,
    path = "/v1/wap/categories/{kind}",
    tag = "wap",
    params(("kind" = String, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    Path(kind): Path<String>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    let list = category_service::list(&state, &kind).await?;
    Ok(ApiJson(list.iter().cloned().map(CatNode::from).collect()))
}

/// Get the direct children of a given parent node
#[utoipa::path(
    get,
    path = "/v1/wap/categories/{kind}/children",
    tag = "wap",
    params(("kind" = String, Path), ChildrenQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn children(
    State(state): State<AppState>,
    Path(kind): Path<String>,
    Query(q): Query<ChildrenQuery>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    let list = category_service::list_children(&state, &kind, q.parent_id).await?;
    Ok(ApiJson(list.iter().cloned().map(CatNode::from).collect()))
}
