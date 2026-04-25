//! Public single-page CMS (mirrors PHPYun `description`): class list / list / detail.

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination};
use phpyun_services::description_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/descriptions/classes", get(list_classes))
        .route("/descriptions", get(list))
        .route("/descriptions/{id}", get(get_one))
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

#[derive(Debug, Deserialize, IntoParams)]
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
    Query(q): Query<ListQuery>,
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
