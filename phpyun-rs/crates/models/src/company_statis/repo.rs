//! `phpyun_company_statis` — per-company counter row.
//!
//! Same shape as `phpyun_member_statis` for jobseekers but on the company
//! side. PHP creates the row lazily when an employer activates their account.
//!
//! This is the **single repo** owning every column. `vip::repo` and
//! `special::repo` re-export the integral / rating accessors from here.

use sqlx::MySqlPool;

/// INSERT IGNORE — create the per-company counter row if it doesn't already
/// exist. Idempotent.
pub async fn ensure_row(pool: &MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT IGNORE INTO phpyun_company_statis (uid) VALUES (?)")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Read the integral balance. Stored as VARCHAR in PHP; CAST forces numeric.
/// Returns 0 when the row doesn't exist.
pub async fn read_integral(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(integral, '0') AS SIGNED) FROM phpyun_company_statis \
         WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n).unwrap_or(0))
}

/// Atomic deduction on the integral column. Returns `1` on success, `0` when
/// balance is insufficient.
pub async fn try_deduct_integral(
    pool: &MySqlPool,
    uid: u64,
    points: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_statis \
            SET integral = CAST(integral AS SIGNED) - ? \
          WHERE uid = ? AND CAST(integral AS SIGNED) >= ?",
    )
    .bind(points)
    .bind(uid)
    .bind(points)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Read the company's rating tier (1..n). Returns 0 when the row doesn't
/// exist.
pub async fn read_rating(pool: &MySqlPool, uid: u64) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(rating, 0) AS SIGNED) FROM phpyun_company_statis \
         WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(r,)| r).unwrap_or(0))
}
