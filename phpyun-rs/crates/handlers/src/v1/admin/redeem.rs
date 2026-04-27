//! Admin integral mall: reward CRUD / classes / order approval.

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::redeem_service::{self, NewRewardForm};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody, StatusFilterBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        // classes
        .route("/redeem-classes", post(create_class))
        .route("/redeem-classes/list", post(list_classes))
        .route("/redeem-classes/delete", post(delete_class))
        // rewards
        .route("/rewards", post(create_reward))
        .route("/rewards/list", post(list_rewards))
        .route("/rewards/delete", post(delete_reward))
        .route("/rewards/status", post(set_reward_status))
        .route("/rewards/flags", post(set_reward_flags))
        // order approval
        .route("/redeem-orders", post(list_orders))
        .route("/redeem-orders/approve", post(approve_order))
        .route("/redeem-orders/reject", post(reject_order))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ClassQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClassItem {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
}

impl From<phpyun_models::redeem::entity::RedeemClass> for ClassItem {
    fn from(c: phpyun_models::redeem::entity::RedeemClass) -> Self {
        Self { id: c.id, parent_id: c.parent_id, name: c.name, sort: c.sort, created_at: c.created_at }
    }
}

/// List classes
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-classes/list",
    tag = "admin",
    security(("bearer" = [])),
    params(ClassQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ClassQuery>,
) -> AppResult<ApiJson<Vec<ClassItem>>> {
    user.require_admin()?;
    let l = redeem_service::list_classes(&state, q.parent_id).await?;
    Ok(ApiJson(l.iter().cloned().map(ClassItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClassForm {
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub parent_id: u64,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

/// Create class
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-classes",
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
    let id = redeem_service::create_class(&state, &user, f.parent_id, &f.name, f.sort).await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Delete class (including children)
#[utoipa::path(post,
    path = "/v1/admin/redeem-classes",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_class(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    redeem_service::delete_class(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

// ---------- rewards ----------

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RewardListQuery {
    /// Default false (admin sees everything)
    #[serde(default)]
    pub only_active: bool,
    #[validate(range(min = 1, max = 99_999_999))]
    pub nid: Option<u64>,
    #[validate(range(min = 1, max = 99_999_999))]
    pub tnid: Option<u64>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RewardItem {
    pub id: u64,
    pub name: String,
    pub content: String,
    pub pic: String,
    pub integral: u32,
    pub stock: u32,
    pub sold: u32,
    pub remaining: i64,
    pub sold_out: bool,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
    pub status: i32,
    pub is_rec: i32,
    pub is_hot: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::redeem::entity::Reward> for RewardItem {
    fn from(r: phpyun_models::redeem::entity::Reward) -> Self {
        let remaining = (r.stock as i64) - (r.sold as i64);
        Self {
            id: r.id,
            name: r.name,
            content: r.content,
            pic: r.pic,
            integral: r.integral,
            stock: r.stock,
            sold: r.sold,
            remaining,
            sold_out: remaining <= 0,
            restriction: r.restriction,
            nid: r.nid,
            tnid: r.tnid,
            status: r.status,
            is_rec: r.is_rec,
            is_hot: r.is_hot,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
        }
    }
}

/// Reward list
#[utoipa::path(
    post,
    path = "/v1/admin/rewards",
    tag = "admin",
    security(("bearer" = [])),
    params(RewardListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_rewards(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<RewardListQuery>,
) -> AppResult<ApiJson<Paged<RewardItem>>> {
    user.require_admin()?;
    let f = redeem_service::RewardFilter { only_active: q.only_active, nid: q.nid, tnid: q.tnid };
    let r = redeem_service::list_rewards(&state, &f, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(RewardItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RewardForm {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default)]
    #[validate(length(max = 1024))]
    pub pic: String,
    #[serde(default)]
    #[validate(length(min = 1, max = 5000))]
    pub content: String,
    #[validate(range(min = 1))]
    pub integral: u32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub stock: u32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub restriction: u32,
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub nid: u64,
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub tnid: u64,
}

/// Create reward
#[utoipa::path(
    post,
    path = "/v1/admin/rewards",
    tag = "admin",
    security(("bearer" = [])),
    request_body = RewardForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create_reward(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RewardForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id = redeem_service::create_reward(
        &state,
        &user,
        &NewRewardForm {
            name: &f.name,
            pic: &f.pic,
            content: &f.content,
            integral: f.integral,
            stock: f.stock,
            restriction: f.restriction,
            nid: f.nid,
            tnid: f.tnid,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Delete reward
#[utoipa::path(post,
    path = "/v1/admin/rewards",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_reward(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    redeem_service::delete_reward(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// 0=offline  1=online
    #[validate(range(min = 0, max = 1))]
    pub status: i32,
}

/// Toggle online/offline
#[utoipa::path(post,
    path = "/v1/admin/rewards/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_reward_status(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<StatusForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    redeem_service::set_reward_status(&state, &user, id, f.status).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FlagsForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// 0 = unset, 1 = recommended. Anything else is rejected at validation.
    #[validate(range(min = 0, max = 1))]
    pub is_rec: Option<i32>,
    /// 0 = unset, 1 = hot.
    #[validate(range(min = 0, max = 1))]
    pub is_hot: Option<i32>,
}

/// Recommended / hot flags
#[utoipa::path(post,
    path = "/v1/admin/rewards/flags",
    tag = "admin",
    security(("bearer" = [])),
    request_body = FlagsForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_reward_flags(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FlagsForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    redeem_service::set_reward_flags(&state, &user, id, f.is_rec, f.is_hot).await?;
    Ok(ApiOk("ok"))
}

// ---------- orders ----------

fn order_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending", 1 => "approved", 2 => "shipped",
        3 => "completed", 4 => "rejected", _ => "unknown",
    }
}

// Reuse mcenter's `OrderItem` — same shape and same `From<RedeemOrder>` impl.
pub type OrderItem = crate::v1::mcenter::redeem::OrderItem;

/// Order list
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-orders",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    user.require_admin()?;
    let r = redeem_service::list_orders_admin(&state, q.status, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OrderItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Approve order (no refund, awaiting shipment)
#[utoipa::path(post,
    path = "/v1/admin/redeem-orders/approve",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn approve_order(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    redeem_service::approve_order(&state, &user, id).await?;
    Ok(ApiOk("approved"))
}

/// Reject order (refund integral + restore stock)
#[utoipa::path(post,
    path = "/v1/admin/redeem-orders/reject",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn reject_order(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    user.require_admin()?;
    redeem_service::reject_order(&state, &user, id).await?;
    Ok(ApiOk("rejected"))
}

