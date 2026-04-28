//! Invite-friend referral repository — **degraded stub**.
//!
//! PHP `phpyun_finder` is a "saved finder filter" table (`uid, usertype,
//! name, para, addtime`), unrelated to invite-friend rewards. PHPYun has
//! no built-in inviter→invitee→points table, so this Rust feature has no
//! persistent backend on the legacy schema. To avoid 500s, every function
//! is a safe no-op:
//!
//! - `record` reports 0 rows affected (caller treats as duplicate-no-op)
//! - `list_*` returns empty
//! - `count_*` / `total_points_earned` returns 0
//!
//! If the feature is needed for production a proper migration adding a
//! Rust-side table can be authored — but that's a product decision, not a
//! bug fix.

use super::entity::Referral;
use sqlx::MySqlPool;

pub async fn record(
    _pool: &MySqlPool,
    _inviter_uid: u64,
    _invitee_uid: u64,
    _points: i32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn list_by_inviter(
    _pool: &MySqlPool,
    _inviter_uid: u64,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<Referral>, sqlx::Error> {
    Ok(Vec::new())
}

pub async fn count_by_inviter(
    _pool: &MySqlPool,
    _inviter_uid: u64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn total_points_earned(
    _pool: &MySqlPool,
    _inviter_uid: u64,
) -> Result<i64, sqlx::Error> {
    Ok(0)
}
