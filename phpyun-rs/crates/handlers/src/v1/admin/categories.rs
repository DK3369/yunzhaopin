//! Category tree management (admin).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::category_service::{self, CatInput, CatPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list).post(create))
        .route("/categories/{id}", post(update))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub kind: String,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CatItem {
    pub id: u64,
    pub parent_id: u64,
    pub kind: String,
    pub name: String,
    pub sort: i32,
    pub status: i32,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::category::entity::Category> for CatItem {
    fn from(c: phpyun_models::category::entity::Category) -> Self {
        Self {
            id: c.id,
            parent_id: c.parent_id,
            kind: c.kind,
            name: c.name,
            sort: c.sort,
            status: c.status,
            updated_at_n: fmt_dt(c.updated_at),
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CatForm {
    #[serde(default)]
    pub parent_id: u64,
    #[validate(length(min = 1, max = 32))]
    pub kind: String,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CatPatchForm {
    pub parent_id: Option<u64>,
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    pub sort: Option<i32>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(get, path = "/v1/admin/categories", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Vec<CatItem>>> {
    let list = category_service::admin_list(&state, &user, &q.kind).await?;
    Ok(ApiJson(list.into_iter().map(CatItem::from).collect()))
}

#[utoipa::path(post, path = "/v1/admin/categories", tag = "admin", security(("bearer" = [])), request_body = CatForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CatForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = category_service::admin_create(
        &state,
        &user,
        CatInput {
            parent_id: f.parent_id,
            kind: &f.kind,
            name: &f.name,
            sort: f.sort,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/categories/{id}", tag = "admin", security(("bearer" = [])), params(("id" = u64, Path)), request_body = CatPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<CatPatchForm>,
) -> AppResult<ApiOk> {
    if f.status == Some(2) {
        category_service::admin_delete(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    category_service::admin_update(
        &state,
        &user,
        id,
        CatPatch {
            parent_id: f.parent_id,
            name: f.name.as_deref(),
            sort: f.sort,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}
