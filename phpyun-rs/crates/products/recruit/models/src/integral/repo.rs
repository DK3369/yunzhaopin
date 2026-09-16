//! PHPYun's points system does not match the Rust design:
//!
//! - `phpyun_admin_integralclass` has only 4 columns
//!   (`id/integral/discount/state`) and records "points-to-discount
//!   conversion rules" (e.g. 100 points = 1 yuan), **not** the
//!   "redeemable items" the Rust side originally envisaged.
//! - The user points balance lives in `phpyun_member.integral` in PHPYun,
//!   not in a separate `member_log_detail.balance`;
//!   `phpyun_member_log_detail` is the points log, not a balance table.
//! - PHPYun records redemption history via redeem orders in
//!   `phpyun_company_order`, not `phpyun_member_withdraw`.
//!
//! Under the "strictly follow PHP + don't change DB" constraint, most of
//! this repo is stubbed: reads return empty, writes are no-ops;
//! `get_balance`/`add_balance`/`try_deduct` are routed to
//! `phpyun_member.integral`.

use super::entity::{IntegralClass, IntegralExchange, IntegralItem, PrepaidCard};
use sqlx::MySqlPool;

pub async fn list_items(
    _pool: &MySqlPool,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<IntegralItem>, sqlx::Error> {
    Ok(vec![])
}

pub async fn count_items(_pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn find_item(_pool: &MySqlPool, _id: u64) -> Result<Option<IntegralItem>, sqlx::Error> {
    Ok(None)
}

pub async fn try_consume_stock(_pool: &MySqlPool, _id: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn rollback_stock(_pool: &MySqlPool, _id: u64) -> Result<(), sqlx::Error> {
    Ok(())
}

pub async fn create_exchange(
    _pool: &MySqlPool,
    _uid: u64,
    _item_id: u64,
    _cost: u32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn list_exchanges_by_user(
    _pool: &MySqlPool,
    _uid: u64,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<IntegralExchange>, sqlx::Error> {
    Ok(vec![])
}

pub async fn count_exchanges_by_user(_pool: &MySqlPool, _uid: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}

// ---------- User points ----------
// PHPYun stores user points in `phpyun_member_statis.integral` (varchar(10)).
// The actual SQL lives in `crate::member_statis::repo` (single repo per
// table); these are kept as re-exports so the legacy call sites
// `integral_repo::get_balance / try_deduct / add_balance` keep working.

pub use crate::member_statis::repo::{add_balance, get_balance, try_deduct};

pub async fn list_active_classes(pool: &MySqlPool) -> Result<Vec<IntegralClass>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(integral,0) AS SIGNED) AS integral, \
         CAST(COALESCE(discount,0) AS SIGNED) AS discount, CAST(COALESCE(state,0) AS SIGNED) AS state \
         FROM phpyun_admin_integralclass WHERE COALESCE(state,0) = 1 \
         ORDER BY integral ASC, id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_class(pool: &MySqlPool, id: u64) -> Result<Option<IntegralClass>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(integral,0) AS SIGNED) AS integral, \
         CAST(COALESCE(discount,0) AS SIGNED) AS discount, CAST(COALESCE(state,0) AS SIGNED) AS state \
         FROM phpyun_admin_integralclass WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_prepaid_card(
    pool: &MySqlPool,
    card: &str,
) -> Result<Option<PrepaidCard>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(password,'') AS password, \
         CAST(COALESCE(quota,0) AS SIGNED) AS quota, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid \
         FROM phpyun_company_card WHERE card = ? LIMIT 1",
    )
    .bind(card)
    .fetch_optional(pool)
    .await
}

pub async fn claim_prepaid_card(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    username: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_card SET uid = ?, username = ?, utime = ? \
         WHERE id = ? AND COALESCE(uid, 0) = 0",
    )
    .bind(uid)
    .bind(username)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
