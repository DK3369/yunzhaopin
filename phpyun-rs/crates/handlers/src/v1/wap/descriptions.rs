//! Public single-page CMS (mirrors PHPYun `description`): class list / list / detail.

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedQuery};
use phpyun_services::description_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/descriptions/classes", get(list_classes))
        .route("/descriptions", get(list))
        .route("/descriptions/{id}", get(get_one))
        .route("/descriptions/by-name/{name}", get(get_by_name))
        .route("/legal/{slug}", get(get_legal_page))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
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
    get,
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
    get,
    path = "/v1/wap/descriptions",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ListQuery>,
) -> AppResult<ApiJson<Paged<DescItem>>> {
    let r = description_service::list(&state, q.class_id, true, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(DescItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
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
#[utoipa::path(
    get,
    path = "/v1/wap/descriptions/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<DescDetail>> {
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
#[utoipa::path(
    get,
    path = "/v1/wap/descriptions/by-name/{name}",
    tag = "wap",
    params(("name" = String, Path)),
    responses(
        (status = 200, description = "ok", body = DescDetail),
        (status = 404, description = "Not found"),
    )
)]
pub async fn get_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> AppResult<ApiJson<DescDetail>> {
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
#[utoipa::path(
    get,
    path = "/v1/wap/legal/{slug}",
    tag = "wap",
    params(("slug" = String, Path, description = "about / contact / privacy / protocol")),
    responses(
        (status = 200, description = "ok", body = DescDetail),
        (status = 400, description = "Unknown slug"),
        (status = 404, description = "No matching description configured"),
    )
)]
pub async fn get_legal_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<ApiJson<DescDetail>> {
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
