//! Dynamic category tree (public).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::category_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories", post(list))
        .route("/categories/children", post(children))
        .route("/categories/recommended", post(recommended))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KindBody {
    #[validate(length(min = 1, max = 64))]
    pub kind: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChildrenBody {
    #[validate(length(min = 1, max = 64))]
    pub kind: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub parent_id: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RecommendedBody {
    #[validate(length(min = 1, max = 64))]
    pub kind: String,
    #[serde(default = "default_rec_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_rec_limit() -> u64 {
    20
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

/// Get all categories under a kind (flat list with parent_id; client builds the tree)
#[utoipa::path(
    post,
    path = "/v1/wap/categories",
    tag = "wap",
    request_body = KindBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<KindBody>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let list = category_service::list(&state, &b.kind).await?;
    Ok(ApiJson(list.iter().cloned().map(CatNode::from).collect()))
}

/// Get the direct children of a given parent node
#[utoipa::path(
    post,
    path = "/v1/wap/categories/children",
    tag = "wap",
    request_body = ChildrenBody,
    responses((status = 200, description = "ok"))
)]
pub async fn children(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<ChildrenBody>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let list = category_service::list_children(&state, &b.kind, b.parent_id).await?;
    Ok(ApiJson(list.iter().cloned().map(CatNode::from).collect()))
}

/// Recommended categories (hand-picked by admin via `rec=1` flag).
/// Counterpart of PHP `category::getHotJobClass(rec=1)` powering homepage
/// "热门职位类别" / "热门行业" widgets. Currently honours the flag for
/// `kind=job` and `kind=company`/`industry` (PHPYun only puts the column
/// on `phpyun_job_class` and `phpyun_comclass`); other kinds fall back to
/// "top by sort".
#[utoipa::path(
    post,
    path = "/v1/wap/categories/recommended",
    tag = "wap",
    request_body = RecommendedBody,
    responses((status = 200, description = "ok"))
)]
pub async fn recommended(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<RecommendedBody>,
) -> AppResult<ApiJson<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let limit = b.limit.clamp(1, 100);
    let list = phpyun_models::category::repo::list_recommended(state.db.reader(), &b.kind, limit)
        .await?;
    Ok(ApiJson(list.into_iter().map(CatNode::from).collect()))
}
