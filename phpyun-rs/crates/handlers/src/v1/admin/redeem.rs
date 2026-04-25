//! Admin integral mall: reward CRUD / classes / order approval.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::redeem_service::{self, NewRewardForm};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // classes
        .route("/redeem-classes", get(list_classes).post(create_class))
        .route("/redeem-classes/{id}", post(delete_class))
        // rewards
        .route("/rewards", get(list_rewards).post(create_reward))
        .route("/rewards/{id}", post(delete_reward))
        .route("/rewards/{id}/status", post(set_reward_status))
        .route("/rewards/{id}/flags", post(set_reward_flags))
        // order approval
        .route("/redeem-orders", get(list_orders))
        .route("/redeem-orders/{id}/approve", post(approve_order))
        .route("/redeem-orders/{id}/reject", post(reject_order))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ClassQuery {
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
    get,
    path = "/v1/admin/redeem-classes",
    tag = "admin",
    security(("bearer" = [])),
    params(ClassQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<ClassQuery>,
) -> AppResult<ApiJson<Vec<ClassItem>>> {
    user.require_admin()?;
    let l = redeem_service::list_classes(&state, q.parent_id).await?;
    Ok(ApiJson(l.iter().cloned().map(ClassItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClassForm {
    #[serde(default)]
    pub parent_id: u64,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[serde(default)]
    pub sort: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
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
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-classes/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn delete_class(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::delete_class(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

// ---------- rewards ----------

#[derive(Debug, Deserialize, IntoParams)]
pub struct RewardListQuery {
    /// Default false (admin sees everything)
    #[serde(default)]
    pub only_active: bool,
    pub nid: Option<u64>,
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
    get,
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
    Query(q): Query<RewardListQuery>,
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
    pub pic: String,
    #[serde(default)]
    pub content: String,
    #[validate(range(min = 1))]
    pub integral: u32,
    #[serde(default)]
    pub stock: u32,
    #[serde(default)]
    pub restriction: u32,
    #[serde(default)]
    pub nid: u64,
    #[serde(default)]
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
#[utoipa::path(
    post,
    path = "/v1/admin/rewards/{id}",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn delete_reward(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::delete_reward(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatusForm {
    /// 0=offline  1=online
    #[validate(range(min = 0, max = 1))]
    pub status: i32,
}

/// Toggle online/offline
#[utoipa::path(
    post,
    path = "/v1/admin/rewards/{id}/status",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = StatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_reward_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<StatusForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::set_reward_status(&state, &user, id, f.status).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FlagsForm {
    pub is_rec: Option<i32>,
    pub is_hot: Option<i32>,
}

/// Recommended / hot flags
#[utoipa::path(
    post,
    path = "/v1/admin/rewards/{id}/flags",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = FlagsForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_reward_flags(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    axum::Json(f): axum::Json<FlagsForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::set_reward_flags(&state, &user, id, f.is_rec, f.is_hot).await?;
    Ok(ApiOk("ok"))
}

// ---------- orders ----------

#[derive(Debug, Deserialize, IntoParams)]
pub struct OrderListQuery {
    pub status: Option<i32>,
}

fn order_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending", 1 => "approved", 2 => "shipped",
        3 => "completed", 4 => "rejected", _ => "unknown",
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItem {
    pub id: u64,
    pub uid: u64,
    pub gid: u64,
    pub name: String,
    pub linkman: String,
    pub linktel: String,
    pub address: String,
    pub integral: u32,
    pub num: u32,
    pub total_integral: u64,
    pub status: i32,
    pub status_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::redeem::entity::RedeemOrder> for OrderItem {
    fn from(o: phpyun_models::redeem::entity::RedeemOrder) -> Self {
        Self {
            id: o.id,
            uid: o.uid,
            gid: o.gid,
            name: o.name,
            linkman: o.linkman,
            linktel: o.linktel,
            address: o.address,
            total_integral: (o.integral as u64) * (o.num as u64),
            integral: o.integral,
            num: o.num,
            status_n: order_status_name(o.status).to_string(),
            status: o.status,
            created_at_n: fmt_dt(o.created_at),
            created_at: o.created_at,
        }
    }
}

/// Order list
#[utoipa::path(
    get,
    path = "/v1/admin/redeem-orders",
    tag = "admin",
    security(("bearer" = [])),
    params(OrderListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<OrderListQuery>,
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
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-orders/{id}/approve",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn approve_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::approve_order(&state, &user, id).await?;
    Ok(ApiOk("approved"))
}

/// Reject order (refund integral + restore stock)
#[utoipa::path(
    post,
    path = "/v1/admin/redeem-orders/{id}/reject",
    tag = "admin",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn reject_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    redeem_service::reject_order(&state, &user, id).await?;
    Ok(ApiOk("rejected"))
}
