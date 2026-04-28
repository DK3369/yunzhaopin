//! VIP packages / orders / status.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::vip_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt, pay_order_status_name as order_status_name};

pub fn routes() -> Router<AppState> {
    let r = Router::new()
        .route("/vip/packages", post(list_packages))
        .route("/vip/current", post(get_current))
        .route("/vip/orders", post(create_order))
        .route("/vip/orders/list", post(list_orders))
        .route("/vip/orders/cancel", post(cancel_order))
        .route("/vip/quote", post(quote_price));

    // mock-paid is only mounted in debug builds; the release binary does not include this route.
    #[cfg(debug_assertions)]
    let r = r.route("/vip/orders/mock-paid", post(mock_paid));

    r
}


/// VIP package item — all 10 columns of phpyun_vip_package + derived price_yuan (yuan unit).
#[derive(Debug, Serialize, ToSchema)]
pub struct PackageItem {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub target_usertype: i32,
    pub duration_days: i32,
    pub price_cents: i32,
    /// price_cents / 100 (yuan, for direct rendering as ¥99.00)
    pub price_yuan: f64,
    pub desc: Option<json::Value>,
    pub is_active: i32,
    pub sort_order: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::vip::entity::VipPackage> for PackageItem {
    fn from(p: phpyun_models::vip::entity::VipPackage) -> Self {
        Self {
            id: p.id,
            code: p.code,
            name: p.name,
            target_usertype: p.target_usertype,
            duration_days: p.duration_days,
            price_yuan: (p.price_cents as f64) / 100.0,
            price_cents: p.price_cents,
            desc: p.desc_json,
            is_active: p.is_active,
            sort_order: p.sort_order,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
        }
    }
}

/// List of purchasable packages (filtered by current user's usertype)
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/packages",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_packages(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<PackageItem>>> {
    let list = vip_service::list_packages(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(PackageItem::from).collect()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentVip {
    pub active: bool,
    pub package_code: Option<String>,
    pub started_at: Option<i64>,
    pub expires_at: Option<i64>,
}

/// My current VIP status
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/current",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CurrentVip))
)]
pub async fn get_current(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<CurrentVip>> {
    let v = vip_service::get_current_vip(&state, &user).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(match v {
        Some(v) => CurrentVip {
            active: v.expires_at > now,
            package_code: Some(v.package_code),
            started_at: Some(v.started_at),
            expires_at: Some(v.expires_at),
        },
        None => CurrentVip {
            active: false,
            package_code: None,
            started_at: None,
            expires_at: None,
        },
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderForm {
    #[validate(length(min = 1, max = 32))]
    pub package_code: String,
    /// alipay / wechat / stripe / stub
    #[validate(length(min = 1, max = 16))]
    pub channel: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderCreated {
    pub order_no: String,
}

/// Create an order (returns order_no, hand it to the frontend to call the payment gateway)
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateOrderForm,
    responses((status = 200, description = "ok", body = OrderCreated))
)]
pub async fn create_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CreateOrderForm>,
) -> AppResult<ApiJson<OrderCreated>> {
    let order_no =
        vip_service::create_order(&state, &user, &f.package_code, &f.channel, &ip).await?;
    Ok(ApiJson(OrderCreated { order_no }))
}

/// Pay order item — all 10 columns of phpyun_pay_order + yuan-unit amount + time formatting.
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItem {
    pub id: u64,
    pub order_no: String,
    pub uid: u64,
    pub package_code: String,
    pub amount_cents: i32,
    pub amount_yuan: f64,
    pub channel: String,
    /// 0 pending / 1 paid / 2 refunded / 3 cancelled
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

/// My orders list
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<OrderItem>>> {
    let r = vip_service::list_orders(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Cancel an unpaid order (orders with status=0). Cannot cancel paid / cancelled orders.
#[utoipa::path(post,
    path = "/v1/mcenter/vip/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CancelOrderBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Order not found / does not belong to you / already paid / cancelled"),
    )
)]
pub async fn cancel_order(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<CancelOrderBody>) -> AppResult<ApiJson<json::Value>> {
    let order_no = b.order_no;
    phpyun_core::validators::ensure_path_token(&order_no)?;
    vip_service::cancel_order(&state, &user, &order_no).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

/// **Dev only**: simulates a payment callback (in production, signature verification of the third-party payment gateway is used).
/// Only compiled in debug builds — this function does not exist in the release binary.
#[cfg(debug_assertions)]
#[utoipa::path(post,
    path = "/v1/mcenter/vip/orders/mock-paid",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = MockPaidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mock_paid(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<MockPaidBody>) -> AppResult<ApiJson<json::Value>> {
    let order_no = b.order_no;
    phpyun_core::validators::ensure_path_token(&order_no)?;
    // Defensive check: the order must belong to the currently logged-in user, to avoid marking someone else's order as paid.
    use phpyun_core::error::InfraError;
    let order = phpyun_models::vip::repo::find_order_by_no(state.db.reader(), &order_no)
        .await?
        .ok_or_else(|| -> phpyun_core::AppError {
            InfraError::InvalidParam("order_not_found".into()).into()
        })?;
    if order.uid != user.uid {
        return Err(InfraError::InvalidParam("order_not_owned".into()).into());
    }

    let fake_tx = format!("MOCK-{}", uuid::Uuid::now_v7().simple());
    vip_service::mark_paid(&state, &order_no, &fake_tx).await?;
    Ok(ApiJson(json::json!({ "ok": true, "pay_tx_id": fake_tx })))
}

// ==================== Price quote ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct PriceQuoteView {
    pub id: u64,
    pub name: String,
    pub service_price: f64,
    pub yh_price: f64,
    pub price: f64,
    /// 1=cash, 2=integral-pay, 3=insufficient-integral fallback to cash.
    pub style: i32,
    pub promo_active: bool,
    pub user_integral: i64,
}

impl From<phpyun_services::vip_service::PriceQuote> for PriceQuoteView {
    fn from(p: phpyun_services::vip_service::PriceQuote) -> Self {
        Self {
            id: p.id,
            name: p.name,
            service_price: p.service_price,
            yh_price: p.yh_price,
            price: p.price,
            style: p.style,
            promo_active: p.promo_active,
            user_integral: p.user_integral,
        }
    }
}

/// Compute the effective price the current user pays for a package.
///
/// Counterpart of PHP `ajax::getPackPrice_action` (`kind=pack`) and
/// `ajax::getVipPrice_action` (`kind=vip`). Combines:
///   - rating-tier discount (`service_discount`)
///   - active promo window (`time_start < now < time_end`)
///   - integral-payment availability (`com_integral_online == 3` and `kind`
///     not in `sy_only_price`)
///   - the user's integral balance
///
/// Returns 403 for non-employers; 400 when the package id is unknown or
/// `kind` is not `pack` / `vip`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QuotePriceBody {
    /// `pack` or `vip` — selects which price table to read.
    #[validate(length(min = 1, max = 32), custom(function = "phpyun_core::validators::path_token"))]
    pub kind: String,
    /// VIP package id (`phpyun_member_pricing.id`).
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/quote",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = QuotePriceBody,
    responses(
        (status = 200, description = "ok", body = PriceQuoteView),
        (status = 400, description = "Invalid kind / package not found"),
        (status = 403, description = "Not an employer"),
    )
)]
pub async fn quote_price(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<QuotePriceBody>,
) -> AppResult<ApiJson<PriceQuoteView>> {
    let q = vip_service::quote_package_price(&state, &user, b.id, &b.kind).await?;
    Ok(ApiJson(PriceQuoteView::from(q)))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct CancelOrderBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub order_no: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct MockPaidBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub order_no: String,
}
