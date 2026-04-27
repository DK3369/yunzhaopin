//! One-off shop posting orders — companion to `wap/once`.
//!
//! Counterpart of PHP `wap/once::paylog_action` (list pending) +
//! `delpaylog_action` (cancel). PHP keys these by the `fast` cookie; the
//! Rust port assumes an authenticated employer and keys on `uid`, which is
//! both safer and matches how the rest of the member centre works.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_services::once_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs/orders", get(list_pending))
        .route("/once-jobs/orders/{id}/cancel", post(cancel))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItem {
    pub id: u64,
    pub order_id: String,
    pub order_type: String,
    pub order_price: f64,
    pub order_time: i64,
    pub order_state: i32,
    pub order_remark: String,
    pub did: Option<i32>,
    pub once_id: Option<i32>,
    pub fast: Option<String>,
}

impl From<phpyun_models::once_job::repo::OnceOrder> for OrderItem {
    fn from(o: phpyun_models::once_job::repo::OnceOrder) -> Self {
        Self {
            id: o.id,
            order_id: o.order_id,
            order_type: o.order_type,
            order_price: o.order_price,
            order_time: o.order_time,
            order_state: o.order_state,
            order_remark: o.order_remark,
            did: o.did,
            once_id: o.once_id,
            fast: o.fast,
        }
    }
}

/// My pending one-off-posting orders (`type=25`, `order_state=1`).
#[utoipa::path(
    get,
    path = "/v1/mcenter/once-jobs/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_pending(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    let r = once_service::list_my_pending_orders(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OrderItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Cancel a pending one-off-posting order (sets `order_state = 3`).
#[utoipa::path(
    post,
    path = "/v1/mcenter/once-jobs/orders/{id}/cancel",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Order not cancellable (already paid / not yours / wrong type)"),
    )
)]
pub async fn cancel(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    once_service::cancel_pending_order(&state, &user, id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
