//! Dynamic category tree (public).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{i18n, ApiResponse, AppResult, AppState, Lang, ValidatedJson};
use phpyun_services::{category_service, dict_service};
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

impl CatNode {
    fn from_category(c: phpyun_models::category::entity::Category, name: String) -> Self {
        Self {
            id: c.id,
            parent_id: c.parent_id,
            name,
            sort: c.sort,
        }
    }
}

fn localized_name(
    dicts: &dict_service::LocalizedDicts,
    kind: &str,
    id: u64,
    fallback: &str,
) -> AppResult<String> {
    // The legacy database stores Chinese as the default value. For the
    // public English category tree, prefer embedded JSON translations so a
    // missing DB translation cannot silently turn the response Chinese.
    if dicts.lang() == Lang::En {
        let key = format!("categories.{kind}.{id}");
        let translated = i18n::t(&key, Lang::En);
        if translated != key {
            return Ok(translated);
        }
    }

    let id = phpyun_core::numeric::checked_db(id, "category.id")?;
    let name = match kind {
        "job" => dicts.job(id),
        "company" | "industry" | "com" | "comclass" => dicts.comclass(id),
        "city" => dicts.city(id),
        "part" | "partclass" => dicts.part(id),
        "question" | "qa" | "q" | "q_class" => dicts.question(id),
        _ => "",
    };
    if name.is_empty() {
        Ok(fallback.to_owned())
    } else {
        Ok(name.to_owned())
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
) -> AppResult<ApiResponse<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let list = category_service::list(&state, &b.kind).await?;
    let dicts = dict_service::get(&state).await?;
    Ok(ApiResponse::data(
        list.iter()
            .cloned()
            .map(|c| -> AppResult<CatNode> {
                let name = localized_name(&dicts, &b.kind, c.id, &c.name)?;
                Ok(CatNode::from_category(c, name))
            })
            .collect::<AppResult<Vec<_>>>()?,
    ))
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
) -> AppResult<ApiResponse<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let list = category_service::list_children(&state, &b.kind, b.parent_id).await?;
    let dicts = dict_service::get(&state).await?;
    Ok(ApiResponse::data(
        list.iter()
            .cloned()
            .map(|c| -> AppResult<CatNode> {
                let name = localized_name(&dicts, &b.kind, c.id, &c.name)?;
                Ok(CatNode::from_category(c, name))
            })
            .collect::<AppResult<Vec<_>>>()?,
    ))
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
) -> AppResult<ApiResponse<Vec<CatNode>>> {
    phpyun_core::validators::ensure_path_token(&b.kind)?;
    let limit = b.limit.clamp(1, 100);
    let list =
        phpyun_models::category::repo::list_recommended(state.db.reader(), &b.kind, limit).await?;
    let dicts = dict_service::get(&state).await?;
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|c| -> AppResult<CatNode> {
                let name = localized_name(&dicts, &b.kind, c.id, &c.name)?;
                Ok(CatNode::from_category(c, name))
            })
            .collect::<AppResult<Vec<_>>>()?,
    ))
}
