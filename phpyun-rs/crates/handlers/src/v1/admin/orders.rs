//! Admin VIP order panel.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::StatusFilterBody, ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::admin_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", post(list))
        .route("/orders/status", post(set_status))
}


// Reuse mcenter's `vip::OrderItem` — same shape and `From<PayOrder>` impl.
pub type OrderItem = crate::v1::mcenter::vip::OrderItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(length(min = 1, max = 64))]
    pub order_no: String,
    /// 2=refund 3=cancel
    #[validate(range(min = 2, max = 3))]
    pub status: i32,
}

/// Global order list
#[utoipa::path(
    post,
    path = "/v1/admin/orders",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    user.require_admin()?;
    let r = admin_service::list_orders(&state, q.status, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Refund / cancel order
#[utoipa::path(post,
    path = "/v1/admin/orders/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>) -> AppResult<ApiOk> {
    let order_no = f.order_no;
    phpyun_core::validators::ensure_path_token(&order_no)?;
    user.require_admin()?;
    admin_service::set_order_status(&state, &user, &order_no, f.status).await?;
    Ok(ApiOk("ok"))
}
