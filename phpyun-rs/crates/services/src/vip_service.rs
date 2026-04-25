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
