//! User redeem orders: submit redemption / my list / cancel (pending only).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::redeem_service::{self, RedeemForm};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/redeem/rewards/{id}/redeem", post(redeem))
        .route("/redeem/orders", get(list_mine))
        .route("/redeem/orders/{id}", post(cancel_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RedeemSubmit {
    #[validate(length(min = 1, max = 64))]
    pub linkman: String,
    #[validate(length(min = 6, max = 32))]
    pub linktel: String,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub address: String,
    #[validate(range(min = 1, max = 999))]
    pub num: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RedeemCreated {
    pub order_id: u64,
}

/// Submit a redeem order
#[utoipa::path(
    post,
    path = "/v1/mcenter/redeem/rewards/{id}/redeem",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = RedeemSubmit,
    responses((status = 200, description = "ok", body = RedeemCreated))
)]
pub async fn redeem(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(reward_id): Path<u64>,
    ValidatedJson(f): ValidatedJson<RedeemSubmit>,
) -> AppResult<ApiJson<RedeemCreated>> {
    let id = redeem_service::redeem(
        &state,
        &user,
        reward_id,
        &RedeemForm { linkman: &f.linkman, linktel: &f.linktel, address: &f.address, num: f.num },
    )
    .await?;
    Ok(ApiJson(RedeemCreated { order_id: id }))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListMineQuery {
    pub status: Option<i32>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn order_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "approved",
        2 => "shipped",
        3 => "completed",
        4 => "rejected",
        _ => "unknown",
    }
}

/// Redeem order item — full 11 columns of phpyun_redeem_order + formatted timestamp + status name + derived total_integral.
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
    /// integral × num (total integral consumed by the order)
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

/// My redeem orders
#[utoipa::path(
    get,
    path = "/v1/mcenter/redeem/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListMineQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListMineQuery>,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    let r = redeem_service::list_my_orders(&state, &user, q.status, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OrderItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Cancel my pending order
#[utoipa::path(
    post,
    path = "/v1/mcenter/redeem/orders/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn cancel_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    redeem_service::cancel_my_order(&state, &user, id).await?;
    Ok(ApiOk("cancelled"))
}
