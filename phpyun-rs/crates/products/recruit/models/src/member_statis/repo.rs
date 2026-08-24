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
// PHPYun stores the balance as `varchar(10)`. Parse and validate in Rust so
// malformed legacy values and arithmetic overflow cannot be hidden by MySQL.
// ============================================================================

pub(crate) const MAX_STORED_BALANCE: i64 = 9_999_999_999;

pub(crate) fn parse_stored_balance(raw: &str, context: &'static str) -> Result<i64, sqlx::Error> {
    let value = raw.trim();
    if value.is_empty() {
        return Ok(0);
    }
    let balance = value
        .parse::<i64>()
        .map_err(|error| phpyun_core::numeric::db_conversion_error::<i64>(context, raw, error))?;
    if balance < 0 {
        return Err(phpyun_core::numeric::db_conversion_error::<i64>(
            context,
            raw,
            "balance must not be negative",
        ));
    }
    if balance > MAX_STORED_BALANCE {
        return Err(phpyun_core::numeric::db_conversion_error::<i64>(
            context,
            raw,
            "balance exceeds the VARCHAR(10) storage limit",
        ));
    }
    Ok(balance)
}

pub(crate) fn balance_after_delta(
    balance: i64,
    delta: i64,
    context: &'static str,
) -> Result<i64, sqlx::Error> {
    let next = if delta >= 0 {
        balance.checked_add(delta).ok_or_else(|| {
            phpyun_core::numeric::db_conversion_error::<i64>(
                context,
                format!("{balance} + {delta}"),
                "balance addition overflow",
            )
        })?
    } else {
        let available = u64::try_from(balance).map_err(|error| {
            phpyun_core::numeric::db_conversion_error::<u64>(context, balance, error)
        })?;
        let remaining = available.saturating_sub(delta.unsigned_abs());
        i64::try_from(remaining).map_err(|error| {
            phpyun_core::numeric::db_conversion_error::<i64>(context, remaining, error)
        })?
    };
    if next > MAX_STORED_BALANCE {
        return Err(phpyun_core::numeric::db_conversion_error::<i64>(
            context,
            next,
            "balance exceeds the VARCHAR(10) storage limit",
        ));
    }
    Ok(next)
}

pub(crate) fn balance_after_deduction(balance: i64, delta: u32) -> Option<i64> {
    let delta = i64::from(delta);
    if balance < delta {
        return None;
    }
    balance.checked_sub(delta)
}

pub async fn get_balance(pool: &MySqlPool, uid: u64) -> Result<UserIntegral, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(integral, '') FROM phpyun_member_statis WHERE uid = ?")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    let balance = row
        .map(|(raw,)| parse_stored_balance(&raw, "phpyun_member_statis.integral"))
        .transpose()?
        .unwrap_or(0);
    Ok(UserIntegral {
        uid,
        balance,
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
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_member_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((raw,)) = row else {
        tx.rollback().await?;
        return Ok(0);
    };
    let balance = parse_stored_balance(&raw, "phpyun_member_statis.integral")?;
    let Some(next) = balance_after_deduction(balance, delta) else {
        tx.rollback().await?;
        return Ok(0);
    };
    sqlx::query("UPDATE phpyun_member_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(1)
}

pub async fn add_balance(
    pool: &MySqlPool,
    uid: u64,
    delta: i64,
    _now: i64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT IGNORE INTO phpyun_member_statis \
            (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num) \
         VALUES (?, '', 0, 0, 0, 0, 0)",
    )
    .bind(uid)
    .execute(&mut *tx)
    .await?;
    let (raw,): (String,) = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_member_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_one(&mut *tx)
    .await?;
    let balance = parse_stored_balance(&raw, "phpyun_member_statis.integral")?;
    let next = balance_after_delta(balance, delta, "phpyun_member_statis.integral")?;
    sqlx::query("UPDATE phpyun_member_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
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
pub async fn bump_fav_jobnum(pool: &MySqlPool, uid: u64, delta: i32) -> Result<(), sqlx::Error> {
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

#[cfg(test)]
mod balance_tests {
    use super::*;

    #[test]
    fn stored_balance_accepts_empty_and_ten_digit_limit() {
        assert_eq!(parse_stored_balance("", "balance").unwrap(), 0);
        assert_eq!(
            parse_stored_balance("9999999999", "balance").unwrap(),
            MAX_STORED_BALANCE
        );
    }

    #[test]
    fn stored_balance_rejects_invalid_negative_and_limit_plus_one() {
        for raw in ["points", "-1", "10000000000"] {
            let error = parse_stored_balance(raw, "phpyun_member_statis.integral").unwrap_err();
            assert!(matches!(error, sqlx::Error::Decode(_)));
            assert!(error.to_string().contains("phpyun_member_statis.integral"));
        }
    }

    #[test]
    fn balance_addition_and_refund_do_not_wrap() {
        assert_eq!(balance_after_delta(20, -50, "balance").unwrap(), 0);
        assert_eq!(
            balance_after_delta(1, i64::from(u32::MAX), "balance").unwrap(),
            4_294_967_296
        );
        for error in [
            balance_after_delta(MAX_STORED_BALANCE, 1, "balance").unwrap_err(),
            balance_after_delta(1, i64::MAX, "balance").unwrap_err(),
        ] {
            assert!(matches!(error, sqlx::Error::Decode(_)));
        }
    }

    #[test]
    fn deductions_are_checked_and_never_negative() {
        assert_eq!(balance_after_deduction(10, 10), Some(0));
        assert_eq!(balance_after_deduction(10, 11), None);
        assert_eq!(balance_after_deduction(10, u32::MAX), None);
    }
}
