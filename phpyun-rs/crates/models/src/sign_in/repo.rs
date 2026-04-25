//! Daily sign-in — strictly aligned with PHPYun's implementation:
//! state is stored in `phpyun_member.signday` (consecutive days) and `signdays` (cumulative days).
//! PHPYun's "is today already signed" check goes through the PHP session
//! (`$_SESSION['qiandao']`), with **no** dedicated per-day deduplication table. Rust has no
//! session, so deduplication should be handled at the service layer using the Redis key
//! `sign:uid:YYYYMMDD` (see `sign_service::sign`).
//!
//! Therefore this repo:
//!   - `get_user_sign`    fetches signday/signdays from `phpyun_member` (`last_date_ymd`
//!                        does not exist in PHP, returns 0; the service layer fills it via Redis)
//!   - `try_sign`         no-op, returns 1 (deduplication delegated to Redis at the service layer)
//!   - `find_today`       always returns None (no day-level persisted record)
//!   - `upsert_user_sign` UPDATE phpyun_member

use super::entity::{SignIn, UserSign};
use sqlx::MySqlPool;

/// Day-level sign-in record — PHPYun does not store this; always returns None.
pub async fn find_today(
    _pool: &MySqlPool,
    _uid: u64,
    _date_ymd: u32,
) -> Result<Option<SignIn>, sqlx::Error> {
    Ok(None)
}

/// PHPYun has no "unique (uid,date_ymd)" table — always allows further writes to phpyun_member.
/// Deduplication is delegated to Redis at the service layer.
pub async fn try_sign(
    _pool: &MySqlPool,
    _uid: u64,
    _date_ymd: u32,
    _client_ip: &str,
    _reward: u32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(1)
}

pub async fn get_user_sign(pool: &MySqlPool, uid: u64) -> Result<UserSign, sqlx::Error> {
    let row: Option<(i64, i64)> = sqlx::query_as(
        "SELECT CAST(COALESCE(signday, 0) AS SIGNED), \
                CAST(COALESCE(signdays, 0) AS SIGNED) \
         FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    let (sd, sds) = row.unwrap_or((0, 0));
    Ok(UserSign {
        uid,
        signday: sd.max(0) as u32,
        signdays: sds.max(0) as u32,
        last_date_ymd: 0,
        updated_at: 0,
    })
}

/// Write back consecutive days / cumulative days to `phpyun_member`
/// (equivalent to PHPYun's `userinfoM -> upInfo`).
pub async fn upsert_user_sign(
    pool: &MySqlPool,
    uid: u64,
    signday: u32,
    _date_ymd: u32,
    _now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE phpyun_member SET signday = ?, signdays = signdays + 1 WHERE uid = ?",
    )
    .bind(signday)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
