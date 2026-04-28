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

use super::entity::{IntegralExchange, IntegralItem};
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
// The actual SQL lives in `crate::member_statis::repo` (single repo per
// table); these are kept as re-exports so the legacy call sites
// `integral_repo::get_balance / try_deduct / add_balance` keep working.

pub use crate::member_statis::repo::{add_balance, get_balance, try_deduct};
