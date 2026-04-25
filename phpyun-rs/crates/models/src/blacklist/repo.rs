use super::entity::BlacklistEntry;
use sqlx::MySqlPool;

// Soft-delete convention: status=2 means deleted. All queries filter out
// deleted rows via `AND status != 2`.

pub async fn add(
    pool: &MySqlPool,
    uid: u64,
    blocked_uid: u64,
    reason: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // If the same (uid, blocked_uid) pair was previously soft-deleted,
    // ON DUPLICATE KEY UPDATE resets status back to 0.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_blacklist
           (uid, blocked_uid, reason, created_at, status)
           VALUES (?, ?, ?, ?, 0)
           ON DUPLICATE KEY UPDATE reason = VALUES(reason), status = 0, created_at = VALUES(created_at)"#,
    )
    .bind(uid)
    .bind(blocked_uid)
    .bind(reason)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: status=2, no physical DELETE.
pub async fn remove(pool: &MySqlPool, uid: u64, blocked_uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_blacklist SET status = 2 WHERE uid = ? AND blocked_uid = ? AND status != 2",
    )
    .bind(uid)
    .bind(blocked_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Clear all (soft-delete every non-deleted entry under uid).
pub async fn remove_all(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_blacklist SET status = 2 WHERE uid = ? AND status != 2")
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
         WHERE uid = ? AND blocked_uid = ? AND status != 2 LIMIT 1",
    )
    .bind(uid)
    .bind(blocked_uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<BlacklistEntry>, sqlx::Error> {
    sqlx::query_as::<_, BlacklistEntry>(
        r#"SELECT id, uid, blocked_uid, reason, created_at
           FROM phpyun_blacklist
           WHERE uid = ? AND status != 2
           ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_blacklist WHERE uid = ? AND status != 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
