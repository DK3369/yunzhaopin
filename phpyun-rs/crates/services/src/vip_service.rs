//! VIP membership service: list packages / create order / mock payment success / query current VIP status.
//!
//! Payment currently uses `channel=stub` which goes through a "fake payment" flow for dev.
//! In production this needs to integrate with alipay / wechat / stripe.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
use phpyun_models::vip::{entity::{PayOrder, UserVip, VipPackage}, repo as vip_repo};
use uuid::Uuid;

const SECS_PER_DAY: i64 = 86_400;

pub async fn list_packages(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<VipPackage>> {
    Ok(vip_repo::list_active_packages(state.db.reader(), user.usertype as i32).await?)
}

/// Create an order -- returns order_no, the client uses it to call the payment gateway.
pub async fn create_order(
    state: &AppState,
    user: &AuthenticatedUser,
    package_code: &str,
    channel: &str,
    client_ip: &str,
) -> AppResult<String> {
    let pkg = vip_repo::find_package_by_code(state.db.reader(), package_code)
        .await?
        .ok_or_else(|| -> AppError {
            InfraError::InvalidParam(format!("unknown package: {package_code}")).into()
        })?;
    if pkg.is_active != 1 {
        return Err(InfraError::InvalidParam("package_inactive".into()).into());
    }
    if pkg.target_usertype != 0 && pkg.target_usertype != user.usertype as i32 {
        return Err(InfraError::InvalidParam("package_usertype_mismatch".into()).into());
    }

    let order_no = format!("ON{}", Uuid::now_v7().simple());
    let now = clock::now_ts();
    vip_repo::create_order(
        state.db.pool(),
        &order_no,
        user.uid,
        &pkg.code,
        pkg.price_cents,
        channel,
        now,
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "amount": pkg.price_cents,
                "channel": channel,
            })),
    )
    .await;

    Ok(order_no)
}

/// Mark order as paid + activate VIP.
///
/// **Security contract**: this function does **not** verify caller identity
/// (it only checks order-status idempotency). The caller MUST have completed authorization:
///   - production: the payment gateway callback handler **must** verify the signature first
///     (see `pay_callback.rs`)
///   - dev: the `mock_paid` handler **must** first check `order.uid == authenticated_user.uid`
/// Any new entry point that bypasses the above and calls this function directly is a security hole.
pub async fn mark_paid(
    state: &AppState,
    order_no: &str,
    pay_tx_id: &str,
) -> AppResult<()> {
    let order = vip_repo::find_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| -> AppError { InfraError::InvalidParam("order_not_found".into()).into() })?;
    if order.status != 0 {
        return Err(InfraError::InvalidParam("order_not_pending".into()).into());
    }

    let pkg = vip_repo::find_package_by_code(state.db.reader(), &order.package_code)
        .await?
        .ok_or_else(|| -> AppError { AppError::internal(std::io::Error::other("package gone")) })?;

    let now = clock::now_ts();
    // 1. Update order status
    let affected = vip_repo::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if affected == 0 {
        return Err(InfraError::InvalidParam("order_already_processed".into()).into());
    }
    // 2. Activate / renew VIP
    vip_repo::upsert_user_vip(
        state.db.pool(),
        order.uid,
        &order.package_code,
        pkg.duration_days as i64 * SECS_PER_DAY,
        now,
    )
    .await?;

    // 3. Audit + event bus
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_paid", Actor::uid(order.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "amount": pkg.price_cents,
                "pay_tx_id": pay_tx_id,
            })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "vip.activated",
            &serde_json::json!({
                "uid": order.uid,
                "package": pkg.code,
                "duration_days": pkg.duration_days,
            }),
        )
        .await;
    Ok(())
}

pub struct OrderPage {
    pub list: Vec<PayOrder>,
    pub total: u64,
}

pub async fn list_orders(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<OrderPage> {
    let (total, list) = tokio::join!(
        vip_repo::count_user_orders(state.db.reader(), user.uid),
        vip_repo::list_user_orders(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(OrderPage {
        total: total?,
        list: list?,
    })
}

/// Cancel an unpaid order (only callable by the order owner).
pub async fn cancel_order(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
) -> AppResult<()> {
    let affected = vip_repo::cancel_order(state.db.pool(), order_no, user.uid).await?;
    if affected == 0 {
        return Err(InfraError::InvalidParam("order_not_cancellable".into()).into());
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_cancelled", Actor::uid(user.uid))
            .target(format!("order:{order_no}")),
    )
    .await;
    Ok(())
}

pub async fn get_current_vip(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Option<UserVip>> {
    Ok(vip_repo::find_user_vip(state.db.reader(), user.uid).await?)
}

// ==================== Pricing quote (PHPYun `getVipPrice` / `getPackPrice`) ====================
//
// Computes the effective price for a package. Mirrors PHP rules:
// - Active promo window (`time_start < now < time_end`) → use `yh_price`
// - Apply rating-tier discount (`service_discount`, percent value 0..=100)
// - When `com_integral_online == 3` and the package is integral-eligible:
//   integral may substitute for cash, multiplied by `integral_proportion`.
//   Compare the integral-priced amount to the user's integral balance to
//   decide `style`:
//     * 1 = cash-only path (integral mode disabled or excluded)
//     * 2 = integral covers it (user has enough)
//     * 3 = integral mode but balance insufficient — frontend should fall
//           back to cash with the discount still applied.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PriceQuote {
    pub id: u64,
    pub name: String,
    /// Original (un-discounted) price in yuan.
    pub service_price: f64,
    /// Discounted yuan price. Falls back to `service_price` when no discount applies.
    pub yh_price: f64,
    /// Effective price the user will pay. Yuan when `style == 1`, integral
    /// units when `style == 2`, yuan when `style == 3` (frontend should
    /// route to cash payment).
    pub price: f64,
    /// 1=cash, 2=integral-pay, 3=insufficient-integral-fallback-to-cash.
    pub style: i32,
    /// Whether the promo window is currently active.
    pub promo_active: bool,
    /// User integral balance — let the client display "余额 X 不足".
    pub user_integral: i64,
}

/// `kind`:
///   * `pack` — company-package buy (PHP `getPackPrice_action`)
///   * `vip`  — individual VIP buy (PHP `getVipPrice_action`)
pub async fn quote_package_price(
    state: &AppState,
    user: &AuthenticatedUser,
    package_id: u64,
    kind: &str,
) -> AppResult<PriceQuote> {
    user.require_employer()?;
    let reader = state.db.reader();

    let pkg = phpyun_models::vip::repo::find_package_pricing(reader, package_id)
        .await?
        .ok_or_else(|| {
            phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
                "package_not_found".into(),
            ))
        })?;

    // Read site config (`com_integral_online`, `integral_proportion`) and
    // `sy_only_price` (CSV of kinds that opt out of integral payment).
    let online_mode = read_int_setting(state, "com_integral_online").await.unwrap_or(0);
    let proportion = read_int_setting(state, "integral_proportion").await.unwrap_or(0);
    let only_price = read_str_setting(state, "sy_only_price")
        .await
        .unwrap_or_default();
    let only_price_csv: Vec<&str> = only_price.split(',').filter(|s| !s.is_empty()).collect();

    let now = phpyun_core::clock::now_ts();
    let promo_active = pkg.time_start < now && pkg.time_end > now;

    let user_integral =
        phpyun_models::vip::repo::read_company_integral(reader, user.uid).await?;
    let discount =
        phpyun_models::vip::repo::read_company_rating_discount(reader, user.uid).await?;

    // PHP `service_discount` is a percent (e.g. 80 = 80%); divide by 100.
    let discount_factor = (discount as f64) / 100.0;
    let pro = proportion.max(0) as f64; // multiplier between yuan and integral

    let effective_yh = if pkg.yh_price > 0.0 { pkg.yh_price } else { pkg.service_price };

    // PHP separates `pack` and `vip` paths but the math is the same once
    // discount + window are folded in. The only difference: `pack` always
    // applies the rating discount; `vip` only applies the promo-window
    // discount. We stay faithful to that here.
    let (display_yh, display_service) = if kind == "pack" {
        (
            effective_yh * discount_factor,
            pkg.service_price,
        )
    } else if kind == "vip" {
        if promo_active {
            (pkg.yh_price, pkg.service_price)
        } else {
            (pkg.service_price, pkg.service_price)
        }
    } else {
        return Err(phpyun_core::AppError::param_invalid("kind"));
    };

    let integral_excluded = only_price_csv.iter().any(|s| *s == kind);
    let integral_path = online_mode == 3 && !integral_excluded && pro > 0.0;

    let (price, style) = if !integral_path {
        (display_yh, 1)
    } else {
        // When paying with integral, the user pays `display_yh * pro` integral.
        let integral_needed = (display_yh * pro) as i64;
        if integral_needed <= user_integral {
            (display_yh * pro, 2)
        } else {
            (display_yh, 3)
        }
    };

    Ok(PriceQuote {
        id: pkg.id,
        name: pkg.name,
        service_price: display_service,
        yh_price: display_yh,
        price,
        style,
        promo_active,
        user_integral,
    })
}

async fn read_int_setting(state: &AppState, key: &str) -> Option<i64> {
    let row = phpyun_models::site_setting::repo::find(state.db.reader(), key)
        .await
        .ok()
        .flatten()?;
    row.value.parse::<i64>().ok()
}

async fn read_str_setting(state: &AppState, key: &str) -> Option<String> {
    let row = phpyun_models::site_setting::repo::find(state.db.reader(), key)
        .await
        .ok()
        .flatten()?;
    Some(row.value)
}
