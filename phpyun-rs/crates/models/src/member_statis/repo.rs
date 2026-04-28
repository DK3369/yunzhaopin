//! `phpyun_member_statis` — per-user counters + integral balance.
//!
//! Schema (PHP): `uid (PK), integral, fav_jobnum, resume_num, sq_jobnum,
//! message_num, down_num`. Each counter is `int(10) NOT NULL`. There is no
//! UNIQUE other than the implicit PK on `uid`, so writes use an UPSERT pattern.
//!
//! This module is the **single repo** owning every column on the table.
//! Functions on the `integral` column are also re-exported from
//! `crate::integral::repo` for backward-compat with older call sites.

use crate::integral::entity::UserIntegral;
use sqlx::MySqlPool;

// ============================================================================
// `integral` column — user point balance.
// PHPYun stores the balance as `varchar(10)` so reads/writes CAST as SIGNED.
// ============================================================================

pub async fn get_balance(pool: &MySqlPool, uid: u64) -> Result<UserIntegral, sqlx::Error> {
    // `integral` is varchar(10) DEFAULT '' — `CAST('' AS SIGNED)` errors
    // under strict sql_mode, so coerce the empty default to '0' first.
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(NULLIF(integral, ''), '0') AS SIGNED) \
         FROM phpyun_member_statis WHERE uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(UserIntegral {
        uid,
        balance: row.map(|(b,)| b.max(0) as i32).unwrap_or(0),
        updated_at: 0,
    })
}

/// Atomic-deduct: returns rows-affected. 0 means insufficient balance.
pub async fn try_deduct(
    pool: &MySqlPool,
    uid: u64,
    delta: u32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member_statis \
         SET integral = CAST(COALESCE(NULLIF(integral, ''), '0') AS SIGNED) - ? \
         WHERE uid = ? \
           AND CAST(COALESCE(NULLIF(integral, ''), '0') AS SIGNED) >= ?",
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
    // `phpyun_member_statis.integral` is `varchar(10) NOT NULL DEFAULT ''`
    // (a PHP-era schema oddity). `CAST('' AS SIGNED)` errors under strict
    // sql_mode with "Truncated incorrect INTEGER value: ''", so we route the
    // empty default to '0' before casting.
    sqlx::query(
        "UPDATE phpyun_member_statis \
         SET integral = GREATEST(CAST(COALESCE(NULLIF(integral, ''), '0') AS SIGNED) + ?, 0) \
         WHERE uid = ?",
    )
    .bind(delta)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

// ============================================================================
// Counter columns + ensure-row.
// ============================================================================

/// INSERT IGNORE — create the per-user counter row with zeros if it doesn't
/// already exist. Idempotent; safe to call from multiple registration / role-
/// upgrade paths.
pub async fn ensure_row(pool: &MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT IGNORE INTO phpyun_member_statis \
            (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num) \
         VALUES (?, '', 0, 0, 0, 0, 0)",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Bump or decrement `fav_jobnum` for a user.
///
/// - `delta >= 0`: UPSERT — insert with the delta or add to existing.
/// - `delta < 0`: UPDATE only, clamped at 0 (no-op when row missing).
///
/// Best-effort: callers swallow the result because counters are denormalised
/// signals, not authoritative data.
pub async fn bump_fav_jobnum(
    pool: &MySqlPool,
    uid: u64,
    delta: i32,
) -> Result<(), sqlx::Error> {
    if delta >= 0 {
        sqlx::query(
            r#"INSERT INTO phpyun_member_statis (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num)
               VALUES (?, '', ?, 0, 0, 0, 0)
               ON DUPLICATE KEY UPDATE fav_jobnum = fav_jobnum + ?"#,
        )
        .bind(uid)
        .bind(delta)
        .bind(delta)
        .execute(pool)
        .await?;
    } else {
        let dec = -delta;
        sqlx::query(
            "UPDATE phpyun_member_statis \
                SET fav_jobnum = GREATEST(fav_jobnum - ?, 0) \
              WHERE uid = ?",
        )
        .bind(dec)
        .bind(uid)
        .execute(pool)
        .await?;
    }
    Ok(())
}
