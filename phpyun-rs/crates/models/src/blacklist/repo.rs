//! `phpyun_blacklist` repository.
//!
//! PHP schema: `(id, p_uid, c_uid, inputtime, usertype, com_name, status)` —
//! the original Rust port mistakenly assumed `(uid, blocked_uid, reason,
//! created_at)`, which broke every call site at runtime. We now alias PHP's
//! columns to those names at the SQL boundary so the entity / service /
//! handler can keep their cleaner public names.
//!
//! Mapping:
//!   - `uid`         ↔ `p_uid`     (perpetrator — the one doing the block)
//!   - `blocked_uid` ↔ `c_uid`     (counterparty — the one being blocked)
//!   - `created_at`  ↔ `inputtime`
//!   - `reason`      ↔ `com_name`  (PHP uses this column as a free-text label)
//!
//! Soft-delete column `status` was added by the
//! `20260425000001_add_status_for_soft_delete` migration.

use super::entity::BlacklistEntry;
use sqlx::MySqlPool;

pub async fn add(
    pool: &MySqlPool,
    uid: u64,
    blocked_uid: u64,
    reason: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP has no UNIQUE on (p_uid, c_uid); guard against duplicates by
    // un-deleting an existing row first, then INSERT only if none was found.
    let updated = sqlx::query(
        "UPDATE phpyun_blacklist \
            SET com_name = ?, inputtime = ?, status = 0 \
          WHERE p_uid = ? AND c_uid = ?",
    )
    .bind(reason)
    .bind(now)
    .bind(uid)
    .bind(blocked_uid)
    .execute(pool)
    .await?;
    if updated.rows_affected() > 0 {
        return Ok(updated.rows_affected());
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_blacklist \
            (p_uid, c_uid, inputtime, com_name, status) \
         VALUES (?, ?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(blocked_uid)
    .bind(now)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: status=2, no physical DELETE.
pub async fn remove(pool: &MySqlPool, uid: u64, blocked_uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_blacklist SET status = 2 \
         WHERE p_uid = ? AND c_uid = ? AND status != 2",
    )
    .bind(uid)
    .bind(blocked_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn remove_all(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_blacklist SET status = 2 WHERE p_uid = ? AND status != 2",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn is_blocked(
    pool: &MySqlPool,
    uid: u64,
    blocked_uid: u64,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM phpyun_blacklist \
         WHERE p_uid = ? AND c_uid = ? AND status != 2 LIMIT 1",
    )
    .bind(uid)
    .bind(blocked_uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(p_uid AS UNSIGNED) AS uid, \
    CAST(c_uid AS UNSIGNED) AS blocked_uid, \
    COALESCE(com_name, '') AS reason, \
    CAST(COALESCE(inputtime, 0) AS SIGNED) AS created_at";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<BlacklistEntry>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_blacklist \
         WHERE p_uid = ? AND status != 2 \
         ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, BlacklistEntry>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_blacklist WHERE p_uid = ? AND status != 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
