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

pub async fn set_rating(pool: &MySqlPool, uid: u64, rating: i32) -> Result<u64, sqlx::Error> {
    ensure_row(pool, uid).await?;
    let res = sqlx::query("UPDATE phpyun_company_statis SET rating = ? WHERE uid = ?")
        .bind(rating)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

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

/// PHP `finance_recharge` jifen：在现有 VARCHAR 积分上加正数。
pub async fn add_integral(pool: &MySqlPool, uid: u64, points: i64) -> Result<i64, sqlx::Error> {
    if points <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: add must be positive, got {points}"
        )));
    }
    ensure_row(pool, uid).await?;
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let raw = row.map(|(s,)| s).unwrap_or_default();
    let balance =
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")?;
    let next = balance.checked_add(points).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: add overflow for {balance} + {points}"
        ))
    })?;
    sqlx::query("UPDATE phpyun_company_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(next)
}

/// PHP 开通套餐天数：从 max(now, vip_etime) 起加 `days` 天。
pub async fn extend_vip_days(
    pool: &MySqlPool,
    uid: u64,
    days: i64,
    now: i64,
) -> Result<i64, sqlx::Error> {
    if days <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.vip_etime: days must be positive, got {days}"
        )));
    }
    ensure_row(pool, uid).await?;
    let mut tx = pool.begin().await?;
    let row: Option<(i64, i64)> = sqlx::query_as(
        "SELECT CAST(COALESCE(vip_stime, 0) AS SIGNED), \
                CAST(COALESCE(vip_etime, 0) AS SIGNED) \
         FROM phpyun_company_statis WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let (stime, etime) = row.unwrap_or((0, 0));
    let base = if etime > now { etime } else { now };
    let extra = days.checked_mul(86_400).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.vip_etime: days overflow {days}"
        ))
    })?;
    let next = base.checked_add(extra).ok_or_else(|| {
        sqlx::Error::Protocol("phpyun_company_statis.vip_etime: timestamp overflow".into())
    })?;
    let next_stime = if stime > 0 { stime } else { now };
    sqlx::query("UPDATE phpyun_company_statis SET vip_stime = ?, vip_etime = ? WHERE uid = ?")
        .bind(next_stime)
        .bind(next)
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(next)
}
