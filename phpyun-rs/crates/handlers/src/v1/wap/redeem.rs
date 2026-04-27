//! Public points-mall endpoints (no login required): classes, reward list, reward detail.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::redeem_service::{self, RewardFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};
use phpyun_core::utils::{fmt_dt, pic_n_str as pic_n};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/redeem/classes", post(list_classes))
        .route("/redeem/rewards", post(list_rewards))
        .route("/redeem/rewards/detail", post(get_reward))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ClassQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
}

/// Redeem class item — all 5 columns of phpyun_redeem_class.
#[derive(Debug, Serialize, ToSchema)]
pub struct ClassItem {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::redeem::entity::RedeemClass> for ClassItem {
    fn from(c: phpyun_models::redeem::entity::RedeemClass) -> Self {
        Self {
            id: c.id,
            parent_id: c.parent_id,
            name: c.name,
            sort: c.sort,
            created_at_n: fmt_dt(c.created_at),
            created_at: c.created_at,
        }
    }
}

/// Redeem mall classes
#[utoipa::path(
    post,
    path = "/v1/wap/redeem/rewards/detail",
    tag = "wap",
    params(ClassQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_classes(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<ClassQuery>,
) -> AppResult<ApiJson<Vec<ClassItem>>> {
    let list = redeem_service::list_classes(&state, q.parent_id).await?;
    Ok(ApiJson(list.iter().cloned().map(ClassItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RewardListQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub nid: Option<u64>,
    #[validate(range(min = 1, max = 99_999_999))]
    pub tnid: Option<u64>,
}

/// Reward list item — all 14 columns of phpyun_redeem + CDN URL + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct RewardItem {
    pub id: u64,
    pub name: String,
    /// Excerpt (first 80 chars of content)
    pub content_excerpt: String,
    pub pic: String,
    pub pic_n: String,
    pub integral: u32,
    pub stock: u32,
    pub sold: u32,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
    pub status: i32,
    pub is_rec: i32,
    pub is_hot: i32,
    pub created_at: i64,
    pub created_at_n: String,
    /// stock - sold (remaining inventory)
    pub remaining: i64,
    /// stock <= sold means sold out
    pub sold_out: bool,
}

impl RewardItem {
    pub fn from_with_ctx(
        r: phpyun_models::redeem::entity::Reward,
        state: &AppState,
    ) -> Self {
        let content_excerpt: String = r.content.chars().take(80).collect();
        let remaining = (r.stock as i64) - (r.sold as i64);
        Self {
            pic_n: pic_n(state, &r.pic),
            id: r.id,
            name: r.name,
            content_excerpt,
            pic: r.pic,
            integral: r.integral,
            stock: r.stock,
            sold: r.sold,
            restriction: r.restriction,
            nid: r.nid,
            tnid: r.tnid,
            status: r.status,
            is_rec: r.is_rec,
            is_hot: r.is_hot,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
            sold_out: remaining <= 0,
            remaining,
        }
    }
}

impl From<phpyun_models::redeem::entity::Reward> for RewardItem {
    fn from(r: phpyun_models::redeem::entity::Reward) -> Self {
        let content_excerpt: String = r.content.chars().take(80).collect();
        let remaining = (r.stock as i64) - (r.sold as i64);
        Self {
            id: r.id,
            name: r.name,
            content_excerpt,
            pic: r.pic.clone(),
            pic_n: r.pic,
            integral: r.integral,
            stock: r.stock,
            sold: r.sold,
            restriction: r.restriction,
            nid: r.nid,
            tnid: r.tnid,
            status: r.status,
            is_rec: r.is_rec,
            is_hot: r.is_hot,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
            sold_out: remaining <= 0,
            remaining,
        }
    }
}

/// Reward list (active only)
#[utoipa::path(
    post,
    path = "/v1/wap/redeem/rewards",
    tag = "wap",
    params(RewardListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_rewards(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<RewardListQuery>,
) -> AppResult<ApiJson<Paged<RewardItem>>> {
    let f = RewardFilter { only_active: true, nid: q.nid, tnid: q.tnid };
    let r = redeem_service::list_rewards(&state, &f, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|x| RewardItem::from_with_ctx(x, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Reward detail — all 14 columns + derived fields.
#[derive(Debug, Serialize, ToSchema)]
pub struct RewardDetail {
    pub id: u64,
    pub name: String,
    pub pic: String,
    pub pic_n: String,
    pub content: String,
    pub integral: u32,
    pub stock: u32,
    pub sold: u32,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
    pub status: i32,
    pub is_rec: i32,
    pub is_hot: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub remaining: i64,
    pub sold_out: bool,
}

impl RewardDetail {
    pub fn from_with_ctx(
        r: phpyun_models::redeem::entity::Reward,
        state: &AppState,
    ) -> Self {
        let remaining = (r.stock as i64) - (r.sold as i64);
        Self {
            pic_n: pic_n(state, &r.pic),
            id: r.id,
            name: r.name,
            pic: r.pic,
            content: r.content,
            integral: r.integral,
            stock: r.stock,
            sold: r.sold,
            restriction: r.restriction,
            nid: r.nid,
            tnid: r.tnid,
            status: r.status,
            is_rec: r.is_rec,
            is_hot: r.is_hot,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
            sold_out: remaining <= 0,
            remaining,
        }
    }
}

impl From<phpyun_models::redeem::entity::Reward> for RewardDetail {
    fn from(r: phpyun_models::redeem::entity::Reward) -> Self {
        let remaining = (r.stock as i64) - (r.sold as i64);
        Self {
            id: r.id,
            name: r.name,
            pic: r.pic.clone(),
            pic_n: r.pic,
            content: r.content,
            integral: r.integral,
            stock: r.stock,
            sold: r.sold,
            restriction: r.restriction,
            nid: r.nid,
            tnid: r.tnid,
            status: r.status,
            is_rec: r.is_rec,
            is_hot: r.is_hot,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
            sold_out: remaining <= 0,
            remaining,
        }
    }
}

/// Reward detail
#[utoipa::path(post,
    path = "/v1/wap/redeem/rewards",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn get_reward(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<RewardDetail>> {
    let id = b.id;
    let r = redeem_service::get_reward(&state, id).await?;
    Ok(ApiJson(RewardDetail::from_with_ctx(r, &state)))
}

