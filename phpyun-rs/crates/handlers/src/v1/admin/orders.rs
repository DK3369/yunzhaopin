//! Admin VIP order panel.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::admin_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", get(list))
        .route("/orders/{order_no}/status", post(set_status))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct OrderListQuery {
    /// 0=pending 1=paid 2=refunded 3=cancelled; omit for all
    pub status: Option<i32>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn order_status_name(s: i32) -> &'static str {
    match s { 0 => "pending", 1 => "paid", 2 => "refunded", 3 => "cancelled", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItem {
    pub id: u64,
    pub order_no: String,
    pub uid: u64,
    pub package_code: String,
    pub amount_cents: i32,
    pub amount_yuan: f64,
    pub channel: String,
    pub status: i32,
    pub status_n: String,
    pub pay_tx_id: Option<String>,
    pub created_at: i64,
    pub created_at_n: String,
    pub paid_at: i64,
    pub paid_at_n: String,
}

impl From<phpyun_models::vip::entity::PayOrder> for OrderItem {
    fn from(o: phpyun_models::vip::entity::PayOrder) -> Self {
        Self {
            id: o.id,
            order_no: o.order_no,
            uid: o.uid,
            package_code: o.package_code,
            amount_yuan: (o.amount_cents as f64) / 100.0,
            amount_cents: o.amount_cents,
            channel: o.channel,
            status_n: order_status_name(o.status).to_string(),
            status: o.status,
            pay_tx_id: o.pay_tx_id,
            created_at_n: fmt_dt(o.created_at),
            created_at: o.created_at,
            paid_at_n: fmt_dt(o.paid_at),
            paid_at: o.paid_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    /// 2=refund 3=cancel
    #[validate(range(min = 2, max = 3))]
    pub status: i32,
}

/// Global order list
#[utoipa::path(
    get,
    path = "/v1/admin/orders",
    tag = "admin",
    security(("bearer" = [])),
    params(OrderListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<OrderListQuery>,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    user.require_admin()?;
    let r = admin_service::list_orders(&state, q.status, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OrderItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Refund / cancel order
#[utoipa::path(
    post,
    path = "/v1/admin/orders/{order_no}/status",
    tag = "admin",
    security(("bearer" = [])),
    params(("order_no" = String, Path)),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(order_no): Path<String>,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    admin_service::set_order_status(&state, &user, &order_no, f.status).await?;
    Ok(ApiOk("ok"))
}
