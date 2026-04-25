//! Ad slot management (admin).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::ad_service::{self, AdInput, AdPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ads", get(list).post(create))
        .route("/ads/{id}", post(update))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub slot: Option<String>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Ad management item — all 14 columns of phpyun_ad + formatted timestamps + derived is_active hint for the frontend button.
#[derive(Debug, Serialize, ToSchema)]
pub struct AdItem {
    pub id: u64,
    pub slot: String,
    pub title: String,
    pub image: String,
    pub link: String,
    pub weight: i32,
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    /// PHP `target`: 1=current window / 2=new window
    pub target: i32,
    pub pic_width: String,
    pub pic_height: String,
    pub pic_content: String,
}

impl From<phpyun_models::ad::entity::Ad> for AdItem {
    fn from(a: phpyun_models::ad::entity::Ad) -> Self {
        Self {
            id: a.id,
            slot: a.slot,
            title: a.title,
            image: a.image,
            link: a.link,
            weight: a.weight,
            start_at_n: fmt_dt(a.start_at),
            start_at: a.start_at,
            end_at_n: fmt_dt(a.end_at),
            end_at: a.end_at,
            status: a.status,
            created_at_n: fmt_dt(a.created_at),
            created_at: a.created_at,
            target: a.target,
            pic_width: a.pic_width,
            pic_height: a.pic_height,
            pic_content: a.pic_content,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdForm {
    #[validate(length(min = 1, max = 64))]
    pub slot: String,
    #[validate(length(max = 200))]
    #[serde(default)]
    pub title: String,
    #[validate(length(min = 1, max = 500))]
    pub image: String,
    #[validate(length(min = 1, max = 500))]
    pub link: String,
    #[serde(default)]
    pub weight: i32,
    #[serde(default)]
    pub start_at: i64,
    #[serde(default)]
    pub end_at: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdPatchForm {
    #[validate(length(min = 1, max = 64))]
    pub slot: Option<String>,
    #[validate(length(max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub image: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub link: Option<String>,
    pub weight: Option<i32>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[utoipa::path(get, path = "/v1/admin/ads", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Paged<AdItem>>> {
    let r = ad_service::admin_list(&state, &user, q.slot.as_deref(), page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AdItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[utoipa::path(post, path = "/v1/admin/ads", tag = "admin", security(("bearer" = [])), request_body = AdForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AdForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = ad_service::admin_create(
        &state,
        &user,
        AdInput {
            slot: &f.slot,
            title: &f.title,
            image: &f.image,
            link: &f.link,
            weight: f.weight,
            start_at: f.start_at,
            end_at: f.end_at,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete an ad (sending `"status":2` deletes; underlying UPDATE sets status=2)
#[utoipa::path(post, path = "/v1/admin/ads/{id}", tag = "admin", security(("bearer" = [])), params(("id" = u64, Path)), request_body = AdPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<AdPatchForm>,
) -> AppResult<ApiOk> {
    if f.status == Some(2) {
        ad_service::admin_delete(&state, &user, id).await?;
        return Ok(ApiOk("deleted"));
    }
    ad_service::admin_update(
        &state,
        &user,
        id,
        AdPatch {
            slot: f.slot.as_deref(),
            title: f.title.as_deref(),
            image: f.image.as_deref(),
            link: f.link.as_deref(),
            weight: f.weight,
            start_at: f.start_at,
            end_at: f.end_at,
            status: f.status,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}
