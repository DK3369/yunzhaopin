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

/// Read the integral balance. Stored as VARCHAR in PHP and validated in Rust.
/// Returns 0 when the row doesn't exist.
pub async fn read_integral(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    row.map(|(raw,)| {
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")
    })
    .transpose()
    .map(|balance| balance.unwrap_or(0))
}

/// Atomic deduction on the integral column. Returns `1` on success, `0` when
/// balance is insufficient.
pub async fn try_deduct_integral(
    pool: &MySqlPool,
    uid: u64,
    points: i64,
) -> Result<u64, sqlx::Error> {
    if points <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: deduction must be positive, got {points}"
        )));
    }
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((raw,)) = row else {
        tx.rollback().await?;
        return Ok(0);
    };
    let balance =
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")?;
    if balance < points {
        tx.rollback().await?;
        return Ok(0);
    }
    let next = balance.checked_sub(points).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: subtraction overflow for {balance} - {points}"
        ))
    })?;
    sqlx::query("UPDATE phpyun_company_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(1)
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
