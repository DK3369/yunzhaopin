//! Admin single-page CMS: class and page CRUD.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::description_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/desc-classes", post(create_class))
        .route("/desc-classes/list", post(list_classes))
        .route("/desc-classes/update", post(update_class))
        .route("/descriptions", post(upsert))
        .route("/descriptions/list", post(list))
        .route("/descriptions/delete", post(delete_one))
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
    post,
    path = "/v1/admin/desc-classes/list",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list_classes(
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
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    /// Soft delete: pass `2` to delete the class.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

/// Update or soft-delete a class (passing `"status":2` in the body means delete)
#[utoipa::path(post,
    path = "/v1/admin/desc-classes/update",
    tag = "admin",
    security(("bearer" = [])),
    request_body = ClassPatchForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_class(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ClassPatchForm>) -> AppResult<ApiOk> {
    let id = f.id;
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub class_id: Option<u64>,
    /// Default false (admin sees everything)
    #[serde(default)]
    pub only_visible: bool,
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
    post,
    path = "/v1/admin/descriptions/list",
    tag = "admin",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<DescItem>>> {
    user.require_admin()?;
    let r = description_service::list(&state, q.class_id, q.only_visible, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: Option<u64>,
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub class_id: u64,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[serde(default)]
    #[validate(length(min = 1, max = 5000))]
    pub content: String,
    /// 1=custom page  2=internal link  3=external link
    #[validate(range(min = 1, max = 3))]
    pub is_type: i32,
    #[serde(default)]
    #[validate(length(max = 1024))]
    pub link_url: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
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

#[utoipa::path(post,
    path = "/v1/admin/descriptions/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_one(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    description_service::delete(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

