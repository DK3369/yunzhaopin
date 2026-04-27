//! Dynamic category tree (public).

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedQuery};
use phpyun_services::category_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories/{kind}", get(list))
        .route("/categories/{kind}/children", get(children))
        .route("/categories/{kind}/recommended", get(recommended))
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

#[derive(Debug, Deserialize, Validate, IntoParams)]
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
    ValidatedQuery(q): ValidatedQuery<ChildrenQuery>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    let list = category_service::list_children(&state, &kind, q.parent_id).await?;
    Ok(ApiJson(list.iter().cloned().map(CatNode::from).collect()))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RecommendedQuery {
    #[serde(default = "default_rec_limit")]
    pub limit: u64,
}
fn default_rec_limit() -> u64 {
    20
}

/// Recommended categories (hand-picked by admin via `rec=1` flag).
/// Counterpart of PHP `category::getHotJobClass(rec=1)` powering homepage
/// "热门职位类别" / "热门行业" widgets. Currently honours the flag for
/// `kind=job` and `kind=company`/`industry` (PHPYun only puts the column
/// on `phpyun_job_class` and `phpyun_comclass`); other kinds fall back to
/// "top by sort".
#[utoipa::path(
    get,
    path = "/v1/wap/categories/{kind}/recommended",
    tag = "wap",
    params(("kind" = String, Path), RecommendedQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn recommended(
    State(state): State<AppState>,
    Path(kind): Path<String>,
    ValidatedQuery(q): ValidatedQuery<RecommendedQuery>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    let limit = q.limit.clamp(1, 100);
    let list = phpyun_models::category::repo::list_recommended(state.db.reader(), &kind, limit)
        .await?;
    Ok(ApiJson(list.into_iter().map(CatNode::from).collect()))
}
