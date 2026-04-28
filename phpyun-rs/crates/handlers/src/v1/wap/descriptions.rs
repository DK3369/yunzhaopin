//! Public single-page CMS (mirrors PHPYun `description`): class list / list / detail.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::description_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/descriptions/classes", post(list_classes))
        .route("/descriptions", post(list))
        .route("/descriptions/get", post(get_one))
        .route("/descriptions/by-name", post(get_by_name))
        .route("/legal", post(get_legal_page))
}


/// Class item -- all 4 columns of phpyun_desc_class.
#[derive(Debug, Serialize, ToSchema)]
pub struct ClassItem {
    pub id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::description::entity::DescClass> for ClassItem {
    fn from(c: phpyun_models::description::entity::DescClass) -> Self {
        Self {
            id: c.id,
            name: c.name,
            sort: c.sort,
            created_at_n: fmt_dt(c.created_at),
            created_at: c.created_at,
        }
    }
}

/// Class list
#[utoipa::path(
    post,
    path = "/v1/wap/descriptions/classes",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn list_classes(
    State(state): State<AppState>,
) -> AppResult<ApiJson<Vec<ClassItem>>> {
    let l = description_service::list_classes(&state).await?;
    Ok(ApiJson(l.iter().cloned().map(ClassItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub class_id: Option<u64>,
}

/// Single-page list item -- all 10 columns of phpyun_description + truncated content + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct DescItem {
    pub id: u64,
    pub class_id: u64,
    pub title: String,
    /// First 100 characters of content
    pub content_excerpt: String,
    pub is_type: i32,
    pub link_url: String,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::description::entity::Description> for DescItem {
    fn from(d: phpyun_models::description::entity::Description) -> Self {
        let content_excerpt: String = d.content.chars().take(100).collect();
        Self {
            id: d.id,
            class_id: d.class_id,
            title: d.title,
            content_excerpt,
            is_type: d.is_type,
            link_url: d.link_url,
            sort: d.sort,
            status: d.status,
            created_at_n: fmt_dt(d.created_at),
            created_at: d.created_at,
            updated_at_n: fmt_dt(d.updated_at),
            updated_at: d.updated_at,
        }
    }
}

/// Single-page list (visible only)
#[utoipa::path(
    post,
    path = "/v1/wap/descriptions",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<DescItem>>> {
    let r = description_service::list(&state, q.class_id, true, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Single-page detail -- all 10 columns (including full content) + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct DescDetail {
    pub id: u64,
    pub class_id: u64,
    pub title: String,
    pub content: String,
    pub is_type: i32,
    pub link_url: String,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::description::entity::Description> for DescDetail {
    fn from(d: phpyun_models::description::entity::Description) -> Self {
        Self {
            id: d.id,
            class_id: d.class_id,
            title: d.title,
            content: d.content,
            is_type: d.is_type,
            link_url: d.link_url,
            sort: d.sort,
            status: d.status,
            created_at_n: fmt_dt(d.created_at),
            created_at: d.created_at,
            updated_at_n: fmt_dt(d.updated_at),
            updated_at: d.updated_at,
        }
    }
}

/// Single-page detail
#[utoipa::path(post,
    path = "/v1/wap/descriptions/get",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn get_one(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<DescDetail>> {
    let id = b.id;
    let d = description_service::get(&state, id).await?;
    if d.status != 1 {
        return Err(phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
            "description_hidden".into(),
        )));
    }
    Ok(ApiJson(d.into()))
}

/// Look up a description by its hand-typed `name` (PHPYun's `phpyun_description.name`).
/// Used for any `getDes(array('name' => ...))` PHP equivalent — accepts the
/// raw URL-encoded Chinese string.
#[utoipa::path(post,
    path = "/v1/wap/descriptions/by-name",
    tag = "wap",
    request_body = GetByNameBody,
    responses(
        (status = 200, description = "ok", body = DescDetail),
        (status = 404, description = "Not found"),
    )
)]
pub async fn get_by_name(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<GetByNameBody>) -> AppResult<ApiJson<DescDetail>> {
    let name = b.name;
    phpyun_core::validators::ensure_path_token(&name)?;
    let row = phpyun_models::description::repo::find_by_name(state.db.reader(), &name).await?;
    let d = row.ok_or_else(|| {
        phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
            "description_not_found".into(),
        ))
    })?;
    Ok(ApiJson(d.into()))
}

/// Stable-slug shortcut for PHP `wap/index::about/contact/privacy/protocol`.
/// Maps `slug` → the PHP-defined Chinese `phpyun_description.name` value:
///
/// - `about`    → `关于我们`
/// - `contact`  → `联系我们`
/// - `privacy`  → `隐私政策`
/// - `protocol` → `注册协议`
///
/// Anything else returns 400.
#[utoipa::path(post,
    path = "/v1/wap/legal",
    tag = "wap",
    request_body = GetLegalPageBody,
    responses(
        (status = 200, description = "ok", body = DescDetail),
        (status = 400, description = "Unknown slug"),
        (status = 404, description = "No matching description configured"),
    )
)]
pub async fn get_legal_page(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<GetLegalPageBody>) -> AppResult<ApiJson<DescDetail>> {
    let slug = b.slug;
    phpyun_core::validators::ensure_path_token(&slug)?;
    let name = match slug.as_str() {
        "about" => "关于我们",
        "contact" => "联系我们",
        "privacy" => "隐私政策",
        "protocol" => "注册协议",
        _ => {
            return Err(phpyun_core::AppError::param_invalid(format!(
                "slug: {slug}"
            )))
        }
    };
    let row = phpyun_models::description::repo::find_by_name(state.db.reader(), name).await?;
    let d = row.ok_or_else(|| {
        phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
            "description_not_found".into(),
        ))
    })?;
    Ok(ApiJson(d.into()))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct GetByNameBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub name: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct GetLegalPageBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub slug: String,
}
