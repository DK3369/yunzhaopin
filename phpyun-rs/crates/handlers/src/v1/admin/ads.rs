//! Ad slot management (admin).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::ad_service::{self, AdInput, AdPatch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ads", post(create))
        .route("/ads/list", post(list))
        .route("/ads/update", post(update))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 100))]
    pub slot: Option<String>,
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
    #[validate(range(min = 0, max = 9_999))]
    pub weight: i32,
    #[serde(default)]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub start_at: i64,
    #[serde(default)]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub end_at: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdPatchForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 64))]
    pub slot: Option<String>,
    #[validate(length(max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub image: Option<String>,
    #[validate(length(min = 1, max = 500))]
    pub link: Option<String>,
    #[validate(range(min = 0, max = 9_999))]
    pub weight: Option<i32>,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub start_at: Option<i64>,
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub end_at: Option<i64>,
    /// 0=offline / 1=online / 2=deleted (soft delete)
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/ads/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<AdItem>>> {
    user.require_admin()?;
    let r = ad_service::admin_list(&state, &user, q.slot.as_deref(), page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[utoipa::path(post, path = "/v1/admin/ads", tag = "admin", security(("bearer" = [])), request_body = AdForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AdForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
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
#[utoipa::path(post, path = "/v1/admin/ads", tag = "admin", security(("bearer" = [])), request_body = AdPatchForm, responses((status = 200, description = "ok")))]
pub async fn update(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AdPatchForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
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
