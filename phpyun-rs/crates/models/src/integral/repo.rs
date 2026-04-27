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

use super::entity::{IntegralExchange, IntegralItem, UserIntegral};
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

pub async fn find_item(
    _pool: &MySqlPool,
    _id: u64,
) -> Result<Option<IntegralItem>, sqlx::Error> {
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
// `phpyun_member` itself does NOT have an `integral` column — earlier code
// queried the wrong table and 1054'd at runtime.

pub async fn get_balance(pool: &MySqlPool, uid: u64) -> Result<UserIntegral, sqlx::Error> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT CAST(COALESCE(integral, 0) AS SIGNED) FROM phpyun_member_statis WHERE uid = ?")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(UserIntegral {
        uid,
        balance: row.map(|(b,)| b.max(0) as i32).unwrap_or(0),
        updated_at: 0,
    })
}

pub async fn try_deduct(
    pool: &MySqlPool,
    uid: u64,
    delta: u32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    // integral is varchar(10) in PHPYun; CAST forces numeric comparison.
    let res = sqlx::query(
        "UPDATE phpyun_member_statis \
         SET integral = CAST(integral AS SIGNED) - ? \
         WHERE uid = ? AND CAST(integral AS SIGNED) >= ?",
    )
    .bind(delta)
    .bind(uid)
    .bind(delta)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn add_balance(
    pool: &MySqlPool,
    uid: u64,
    delta: i32,
    _now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE phpyun_member_statis \
         SET integral = GREATEST(CAST(integral AS SIGNED) + ?, 0) \
         WHERE uid = ?",
    )
    .bind(delta)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
