//! User redeem orders: submit redemption / my list / cancel (pending only).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::redeem_service::{self, RedeemForm};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{IdBody, StatusFilterBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/redeem/rewards/redeem", post(redeem))
        .route("/redeem/orders", post(list_mine))
        .route("/redeem/orders/cancel", post(cancel_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RedeemSubmit {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
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
    path = "/v1/mcenter/redeem/rewards/redeem",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = RedeemSubmit,
    responses((status = 200, description = "ok", body = RedeemCreated))
)]
pub async fn redeem(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RedeemSubmit>,
) -> AppResult<ApiJson<RedeemCreated>> {
    let id = redeem_service::redeem(
        &state,
        &user,
        f.id,
        &RedeemForm { linkman: &f.linkman, linktel: &f.linktel, address: &f.address, num: f.num },
    )
    .await?;
    Ok(ApiJson(RedeemCreated { order_id: id }))
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
    post,
    path = "/v1/mcenter/redeem/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
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
    path = "/v1/mcenter/redeem/orders/cancel",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn cancel_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiOk> {
    redeem_service::cancel_my_order(&state, &user, b.id).await?;
    Ok(ApiOk("cancelled"))
}
