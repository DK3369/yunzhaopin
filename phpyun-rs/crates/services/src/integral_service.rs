//! Points-mall service (mirrors PHPYun `wap/redeem`).
//!
//! Redemption flow (atomic):
//!   1) Deduct stock (atomic UPDATE WHERE stock>0) — failure means sold out
//!   2) Deduct user points (atomic UPDATE WHERE balance>=cost) — failure means insufficient balance
//!   3) Write the exchange record
//!   4) If any of the above fails, roll back what was already deducted (using a reverse UPDATE)
//!
//! No database transaction is used here (transactions across the writer pool are expensive);
//! we use compensation (transaction-script pattern) instead. The PHPYun original only does naive
//! checks at the PHP layer — not atomic, so it has oversell / double-deduct issues. The migrated
//! version fixes that along the way.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::integral::{
    entity::{IntegralExchange, IntegralItem, UserIntegral},
    repo as integral_repo,
};

pub async fn list_items(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<IntegralItem>> {
    let db = state.db.reader();
    let list = integral_repo::list_items(db, page.offset, page.limit).await?;
    let total = integral_repo::count_items(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_item(state: &AppState, id: u64) -> AppResult<IntegralItem> {
    integral_repo::find_item(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("item_not_found".into())))
}

pub async fn balance(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<UserIntegral> {
    Ok(integral_repo::get_balance(state.db.reader(), user.uid).await?)
}

pub async fn exchange(
    state: &AppState,
    user: &AuthenticatedUser,
    item_id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    let item = integral_repo::find_item(state.db.reader(), item_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("item_not_found".into())))?;
    if item.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("item_unavailable".into())));
    }

    let pool = state.db.pool();
    let now = clock::now_ts();

    // 1) Deduct stock
    let stock_affected = integral_repo::try_consume_stock(pool, item_id).await?;
    if stock_affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam("item_sold_out".into())));
    }

    // 2) Deduct points
    let deduct_affected = integral_repo::try_deduct(pool, user.uid, item.cost, now).await?;
    if deduct_affected == 0 {
        // Roll back stock
        integral_repo::rollback_stock(pool, item_id).await?;
        return Err(AppError::new(InfraError::InvalidParam("insufficient_balance".into())));
    }

    // 3) Write the exchange record
    let exchange_id =
        match integral_repo::create_exchange(pool, user.uid, item_id, item.cost, now).await {
            Ok(id) => id,
            Err(e) => {
                // Roll back points + stock
                let _ = integral_repo::add_balance(pool, user.uid, item.cost as i32, now).await;
                let _ = integral_repo::rollback_stock(pool, item_id).await;
                return Err(e.into());
            }
        };

    let _ = audit::emit(
        state,
        audit::AuditEvent::new(
            "integral.exchange",
            audit::Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("item:{item_id}"))
        .meta(&serde_json::json!({ "cost": item.cost, "exchange_id": exchange_id })),
    )
    .await;

    Ok(exchange_id)
}

pub async fn list_history(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<IntegralExchange>> {
    let db = state.db.reader();
    let list =
        integral_repo::list_exchanges_by_user(db, user.uid, page.offset, page.limit).await?;
    let total = integral_repo::count_exchanges_by_user(db, user.uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

// ---------- Points transfer ----------

use phpyun_models::integral_transfer::{entity::IntegralTransfer, repo as transfer_repo};

pub async fn transfer(
    state: &AppState,
    user: &AuthenticatedUser,
    to_uid: u64,
    points: u32,
    note: &str,
) -> AppResult<u64> {
    if to_uid == user.uid {
        return Err(AppError::new(InfraError::InvalidParam("cannot_transfer_to_self".into())));
    }
    if points == 0 {
        return Err(AppError::new(InfraError::InvalidParam("bad_points".into())));
    }
    let now = clock::now_ts();
    let id = transfer_repo::execute(state.db.pool(), user.uid, to_uid, points, note, now)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("insufficient_balance".into())))?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("integral.transfer", audit::Actor::uid(user.uid))
            .target(format!("to:{to_uid}"))
            .meta(&serde_json::json!({ "points": points, "transfer_id": id })),
    )
    .await;
    Ok(id)
}

pub async fn list_transfers(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<IntegralTransfer>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        transfer_repo::list_by_user(db, user.uid, page.offset, page.limit),
        transfer_repo::count_by_user(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
