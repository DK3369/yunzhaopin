use super::entity::IntegralTransfer;
use sqlx::MySqlPool;

/// Atomic transfer: deduct from `from`, credit `to`, write log -- all in
/// a single transaction. On failure (insufficient balance / DB issue) no
/// user loses points. Returns the transfer record id.
pub async fn execute(
    pool: &MySqlPool,
    from_uid: u64,
    to_uid: u64,
    points: u32,
    note: &str,
    now: i64,
) -> Result<Option<u64>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Deduct from `from` (must have balance >= points to affect 1 row).
    let deduct = sqlx::query(
        r#"UPDATE phpyun_member_log_detail
           SET balance = balance - ?, updated_at = ?
           WHERE uid = ? AND balance >= ?"#,
    )
    .bind(points)
    .bind(now)
    .bind(from_uid)
    .bind(points)
    .execute(&mut *tx)
    .await?;
    if deduct.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(None);
    }

    // Credit `to` (UPSERT).
    sqlx::query(
        r#"INSERT INTO phpyun_member_log_detail (uid, balance, updated_at)
           VALUES (?, ?, ?)
           ON DUPLICATE KEY UPDATE balance = balance + VALUES(balance), updated_at = VALUES(updated_at)"#,
    )
    .bind(to_uid)
    .bind(points as i64)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Log.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_member_log
           (from_uid, to_uid, points, note, created_at)
           VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(from_uid)
    .bind(to_uid)
    .bind(points)
    .bind(note)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(res.last_insert_id()))
}

const FIELDS: &str = "id, from_uid, to_uid, points, note, created_at";

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<IntegralTransfer>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_member_log
         WHERE from_uid = ? OR to_uid = ?
         ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, IntegralTransfer>(&sql)
        .bind(uid)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_user(
    pool: &MySqlPool,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_member_log
         WHERE from_uid = ? OR to_uid = ?",
    )
    .bind(uid)
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
