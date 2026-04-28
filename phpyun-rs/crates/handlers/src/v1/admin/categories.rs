//! Category tree management (admin).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::category_service::{self, CatInput, CatPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories", post(create))
        .route("/categories/list", post(list))
        .route("/categories/update", post(update))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(min = 1, max = 200))]
    pub kind: String,
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: u64,
    #[validate(length(min = 1, max = 32))]
    pub kind: String,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CatPatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/categories/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Vec<CatItem>>> {
    user.require_admin()?;
    let list = category_service::admin_list(&state, &user, &q.kind).await?;
    Ok(ApiJson(list.into_iter().map(CatItem::from).collect()))
}

#[utoipa::path(post, path = "/v1/admin/categories", tag = "admin", security(("bearer" = [])), request_body = CatForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CatForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
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

#[utoipa::path(post, path = "/v1/admin/categories/update", tag = "admin", security(("bearer" = [])), request_body = CatPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CatPatchForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
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
