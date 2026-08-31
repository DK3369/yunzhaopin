//! Category tree management (admin).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::category_service::{self, CatInput, CatPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories", post(create))
        .route("/categories/list", post(list))
        .route("/categories/update", post(update))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// PHP Vue index often omits this; default matches 职位分类.
    #[serde(default)]
    #[validate(length(max = 200))]
    pub kind: String,
}

fn list_kind(kind: &str) -> &str {
    let k = kind.trim();
    if k.is_empty() { "job" } else { k }
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
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64")]
    #[validate(range(min = 0, max = 99_999_999))]
    pub parent_id: u64,
    #[validate(length(min = 1, max = 32))]
    pub kind: String,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CatPatchForm {
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_u64")]
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/categories/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Vec<CatItem>>> {
    user.require_admin()?;
    let list = category_service::admin_list(&state, &user, list_kind(&q.kind)).await?;
    Ok(ApiResponse::data(
        list.into_iter().map(CatItem::from).collect(),
    ))
}

#[utoipa::path(post, path = "/v1/admin/categories", tag = "admin", security(("bearer" = [])), request_body = CatForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CatForm>,
) -> AppResult<ApiResponse<CreatedId>> {
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
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/categories/update", tag = "admin", security(("bearer" = [])), request_body = CatPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CatPatchForm>,
) -> AppResult<ApiResponse> {
    let id = f.id;
    user.require_admin()?;
    if f.status == Some(2) {
        category_service::admin_delete(&state, &user, id).await?;
        return Ok(ApiResponse::message("deleted"));
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
    Ok(ApiResponse::message("ok"))
}
