//! One-off shop posting orders — companion to `wap/once`.
//!
//! Counterpart of PHP `wap/once::paylog_action` (list pending) +
//! `delpaylog_action` (cancel). PHP keys these by the `fast` cookie; the
//! Rust port assumes an authenticated employer and keys on `uid`, which is
//! both safer and matches how the rest of the member centre works.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::json;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::once_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs/orders", post(list_pending))
        .route("/once-jobs/orders/cancel", post(cancel))
        .route("/once-jobs/paylogs", post(list_paylogs))
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
    post,
    path = "/v1/mcenter/once-jobs/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_pending(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<OrderItem>>> {
    let r = once_service::list_my_pending_orders(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

/// Cancel a pending one-off-posting order (sets `order_state = 3`).
#[utoipa::path(
    post,
    path = "/v1/mcenter/once-jobs/orders/cancel",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Order not cancellable (already paid / not yours / wrong type)"),
    )
)]
pub async fn cancel(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    once_service::cancel_pending_order(&state, &user, b.id).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaylogsQuery {
    /// Optional `company_order.order_state` 1/2/3. Omit = 1+2+3.
    #[serde(default)]
    #[validate(range(min = 0, max = 3))]
    pub order_state: Option<i32>,
}

/// `type=25` paylogs. `/once-jobs/orders` still lists pending (`order_state=1`) only.
#[utoipa::path(
    post,
    path = "/v1/mcenter/once-jobs/paylogs",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PaylogsQuery,
    responses((status = 200, description = "ok"))
)]
pub async fn list_paylogs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<PaylogsQuery>,
) -> AppResult<ApiResponse<Paged<OrderItem>>> {
    let st = q.order_state.filter(|s| *s > 0);
    let r = once_service::list_my_paylogs(&state, &user, st, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}
