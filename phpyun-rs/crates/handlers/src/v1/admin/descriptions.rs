//! Admin single-page CMS: class and page CRUD.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson, ValidatedQuery
};
use phpyun_services::description_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/desc-classes", get(list_classes).post(create_class))
        .route("/desc-classes/{id}", post(update_class))
        .route("/descriptions", get(list).post(upsert))
        .route("/descriptions/{id}", post(delete_one))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClassItem {
    pub id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
}

impl From<phpyun_models::description::entity::DescClass> for ClassItem {
    fn from(c: phpyun_models::description::entity::DescClass) -> Self {
        Self { id: c.id, name: c.name, sort: c.sort, created_at: c.created_at }
    }
}

#[utoipa::path(
    get,
    path = "/v1/admin/desc-classes",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<ClassItem>>> {
    user.require_admin()?;
    let l = description_service::list_classes(&state).await?;
    Ok(ApiJson(l.iter().cloned().map(ClassItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClassForm {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(
    post,
    path = "/v1/admin/desc-classes",
    tag = "admin",
    security(("bearer" = [])),
    request_body = ClassForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create_class(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ClassForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id = description_service::create_class(&state, &user, &f.name, f.sort).await?;
    Ok(ApiJson(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClassPatchForm {
    #[serde(default)]
    pub sort: Option<i32>,
    /// Soft delete: pass `2` to delete the class.
    #[serde(default)]
    pub status: Option<i32>,
}

/// Update or soft-delete a class (passing `"status":2` in the body means delete)
#[utoipa::path(
    post,
    path = "/v1/admin/desc-classes/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = ClassPatchForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_class(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<ClassPatchForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    if f.status == Some(2) {
        description_service::delete_class(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    if let Some(sort) = f.sort {
        description_service::update_class_sort(&state, &user, id, sort).await?;
    }
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    pub class_id: Option<u64>,
    /// Default false (admin sees everything)
    #[serde(default)]
    pub only_visible: bool,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Single-page admin item — all 10 phpyun_description columns + time formatting (incl. content).
#[derive(Debug, Serialize, ToSchema)]
pub struct DescItem {
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

impl From<phpyun_models::description::entity::Description> for DescItem {
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

#[utoipa::path(
    get,
    path = "/v1/admin/descriptions",
    tag = "admin",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ListQuery>,
) -> AppResult<ApiJson<Paged<DescItem>>> {
    user.require_admin()?;
    let r = description_service::list(&state, q.class_id, q.only_visible, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(DescItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertForm {
    pub id: Option<u64>,
    #[serde(default)]
    pub class_id: u64,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[serde(default)]
    pub content: String,
    /// 1=custom page  2=internal link  3=external link
    #[validate(range(min = 1, max = 3))]
    pub is_type: i32,
    #[serde(default)]
    pub link_url: String,
    #[serde(default)]
    pub sort: i32,
    /// 0=hidden  1=visible
    #[validate(range(min = 0, max = 1))]
    pub status: i32,
}

#[utoipa::path(
    post,
    path = "/v1/admin/descriptions",
    tag = "admin",
    security(("bearer" = [])),
    request_body = UpsertForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UpsertForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id = description_service::upsert(
        &state,
        &user,
        &description_service::UpsertForm {
            id: f.id,
            class_id: f.class_id,
            title: &f.title,
            content: &f.content,
            is_type: f.is_type,
            link_url: &f.link_url,
            sort: f.sort,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/admin/descriptions/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn delete_one(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    description_service::delete(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}
