//! Integral / points transfer — aligned with PHPYun `integral_model::company_invtal`.
//!
//! PHP truth:
//! - **Balance** lives in `phpyun_member_statis.integral` (jobseeker, usertype=1)
//!   or `phpyun_company_statis.integral` (employer, usertype=2). It's stored
//!   as VARCHAR(10) but treated as a numeric in app logic — we validate and
//!   calculate it under row locks before writing.
//! - **Ledger** rows go to `phpyun_company_pay` (despite the name, it
//!   handles both employer and jobseeker rows; `com_id` = affected uid).
//!
//! `transfer(from, to, points, note)` is a between-users transfer:
//!   1. Lock both balance rows in a stable UID order and decode their raw values.
//!   2. Check the debit and credit in Rust, then write both resulting balances.
//!   3. Insert `phpyun_company_pay` rows for the debit and credit sides.
//!
//! All three in a single transaction. Failure mid-flight rolls back.

use super::entity::IntegralTransfer;
use sqlx::MySqlPool;

const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                      COALESCE(order_id, '') AS order_id, \
                      COALESCE(order_price, 0) AS order_price, \
                      COALESCE(pay_time, 0) AS pay_time, \
                      COALESCE(pay_state, 0) AS pay_state, \
                      CAST(COALESCE(com_id, 0) AS UNSIGNED) AS com_id, \
                      COALESCE(pay_remark, '') AS pay_remark, \
                      COALESCE(`type`, 0) AS `type`, \
                      COALESCE(pay_type, 0) AS pay_type, \
                      COALESCE(did, 0) AS did, \
                      CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
                      COALESCE(usertype, 0) AS usertype, \
                      CAST(COALESCE(coupon_id, 0) AS UNSIGNED) AS coupon_id";

/// pay_type for between-user transfers (PHPYun integral.model.php docblock):
/// 27 = "积分抵扣" is closest. We use a vendor-extension code 99 ("user→user
/// transfer") to keep these distinguishable in admin reports without
/// colliding with any PHP-known code in the 1..28 range.
const PAY_TYPE_USER_TRANSFER: i32 = 99;

fn gen_order_id(now: i64) -> String {
    // Mirror PHP's `time().rand(10000,99999)`. Avoid pulling in `rand` for one
    // string suffix — use a process-local atomic counter mod 90000 + 10000.
    use std::sync::atomic::{AtomicU32, Ordering};
    static SEQ: AtomicU32 = AtomicU32::new(10000);
    let r = SEQ.fetch_add(1, Ordering::Relaxed) % 90_000 + 10_000;
    format!("{now}{r}")
}

fn transfer_balances(
    from_balance: i64,
    to_balance: i64,
    points: u32,
) -> Result<Option<(i64, i64)>, sqlx::Error> {
    let Some(from_next) = crate::member_statis::repo::balance_after_deduction(from_balance, points)
    else {
        return Ok(None);
    };
    let to_next = crate::member_statis::repo::balance_after_delta(
        to_balance,
        i64::from(points),
        "phpyun_member_statis.integral recipient",
    )?;
    Ok(Some((from_next, to_next)))
}

/// Atomic between-user integral transfer.
/// Returns:
/// - `Ok(Some(ledger_id))` on success — the credit-side `phpyun_company_pay.id`
/// - `Ok(None)` on insufficient balance (no rows changed; user-facing 4xx)
pub async fn execute(
    pool: &MySqlPool,
    from_uid: u64,
    to_uid: u64,
    points: u32,
    note: &str,
    now: i64,
) -> Result<Option<u64>, sqlx::Error> {
    if from_uid == to_uid {
        return Err(sqlx::Error::Protocol(
            "integral transfer source and recipient must differ".to_owned(),
        ));
    }
    if points == 0 {
        return Err(sqlx::Error::Protocol(
            "integral transfer points must be positive".to_owned(),
        ));
    }
    // Ensure the recipient row exists before opening the transfer transaction.
    // This idempotent shell avoids taking an out-of-order unique-key lock inside
    // the two-row transaction below.
    sqlx::query(
        "INSERT IGNORE INTO phpyun_member_statis \
            (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num) \
         VALUES (?, '', 0, 0, 0, 0, 0)",
    )
    .bind(to_uid)
    .execute(pool)
    .await?;
    let mut tx = pool.begin().await?;

    // Lock both rows in stable UID order so concurrent opposite-direction
    // transfers cannot deadlock each other.
    let (first_uid, second_uid) = if from_uid < to_uid {
        (from_uid, to_uid)
    } else {
        (to_uid, from_uid)
    };
    let raw_balances: Vec<(i64, String)> = sqlx::query_as(
        "SELECT uid, COALESCE(integral, '') FROM phpyun_member_statis \
         WHERE uid IN (?, ?) ORDER BY uid FOR UPDATE",
    )
    .bind(first_uid)
    .bind(second_uid)
    .fetch_all(&mut *tx)
    .await?;
    let balances = raw_balances
        .into_iter()
        .map(|(uid, raw)| -> Result<(u64, String), sqlx::Error> {
            Ok((
                phpyun_core::numeric::checked_db(uid, "phpyun_member_statis.uid")?,
                raw,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let from_raw = balances
        .iter()
        .find(|(uid, _)| *uid == from_uid)
        .map(|(_, raw)| raw.as_str());
    let Some(from_raw) = from_raw else {
        tx.rollback().await?;
        return Ok(None);
    };
    let to_raw = balances
        .iter()
        .find(|(uid, _)| *uid == to_uid)
        .map(|(_, raw)| raw.as_str())
        .ok_or_else(|| {
            sqlx::Error::Protocol(format!(
                "recipient balance row disappeared after INSERT IGNORE: uid={to_uid}"
            ))
        })?;
    let from_balance = crate::member_statis::repo::parse_stored_balance(
        from_raw,
        "phpyun_member_statis.integral sender",
    )?;
    let to_balance = crate::member_statis::repo::parse_stored_balance(
        to_raw,
        "phpyun_member_statis.integral recipient",
    )?;
    let Some((from_next, to_next)) = transfer_balances(from_balance, to_balance, points)? else {
        tx.rollback().await?;
        return Ok(None);
    };

    // Get from-side `did` for ledger denormalization.
    let from_did: i32 = sqlx::query_as::<_, (i32,)>(
        "SELECT COALESCE(did, 0) FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(from_uid)
    .fetch_optional(&mut *tx)
    .await?
    .map(|(d,)| d)
    .unwrap_or(0);
    let to_did: i32 = sqlx::query_as::<_, (i32,)>(
        "SELECT COALESCE(did, 0) FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(to_uid)
    .fetch_optional(&mut *tx)
    .await?
    .map(|(d,)| d)
    .unwrap_or(0);

    // Write the two already-checked values while both rows remain locked.
    sqlx::query("UPDATE phpyun_member_statis SET integral = ? WHERE uid = ?")
        .bind(from_next.to_string())
        .bind(from_uid)
        .execute(&mut *tx)
        .await?;
    sqlx::query("UPDATE phpyun_member_statis SET integral = ? WHERE uid = ?")
        .bind(to_next.to_string())
        .bind(to_uid)
        .execute(&mut *tx)
        .await?;

    // Write two ledger rows: one debit (-points), one credit (+points).
    let order_id_debit = gen_order_id(now);
    let order_id_credit = gen_order_id(now);
    let remark = if note.is_empty() {
        "用户积分转账".to_string()
    } else {
        format!("积分转账：{note}")
    };

    sqlx::query(
        "INSERT INTO phpyun_company_pay
              (order_id, order_price, pay_time, pay_state, com_id, pay_remark,
               `type`, pay_type, did, eid, usertype, coupon_id)
           VALUES (?, ?, ?, 2, ?, ?, 1, ?, ?, 0, 1, 0)",
    )
    .bind(&order_id_debit)
    .bind(-i64::from(points))
    .bind(now)
    .bind(from_uid)
    .bind(&remark)
    .bind(PAY_TYPE_USER_TRANSFER)
    .bind(from_did)
    .execute(&mut *tx)
    .await?;

    let credit = sqlx::query(
        "INSERT INTO phpyun_company_pay
              (order_id, order_price, pay_time, pay_state, com_id, pay_remark,
               `type`, pay_type, did, eid, usertype, coupon_id)
           VALUES (?, ?, ?, 2, ?, ?, 1, ?, ?, 0, 1, 0)",
    )
    .bind(&order_id_credit)
    .bind(i64::from(points))
    .bind(now)
    .bind(to_uid)
    .bind(&remark)
    .bind(PAY_TYPE_USER_TRANSFER)
    .bind(to_did)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(credit.last_insert_id()))
}

/// List a user's transfer rows (both debit and credit sides).
pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<IntegralTransfer>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_company_pay
          WHERE com_id = ? AND pay_type = ?
          ORDER BY pay_time DESC, id DESC
          LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, IntegralTransfer>(&sql)
        .bind(uid)
        .bind(PAY_TYPE_USER_TRANSFER)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_pay WHERE com_id = ? AND pay_type = ?")
            .bind(uid)
            .bind(PAY_TYPE_USER_TRANSFER)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::member_statis::repo::MAX_STORED_BALANCE;

    #[test]
    fn transfer_balances_are_checked_on_both_sides() {
        assert_eq!(transfer_balances(100, 20, 30).unwrap(), Some((70, 50)));
        assert_eq!(transfer_balances(29, 20, 30).unwrap(), None);
        assert!(transfer_balances(100, MAX_STORED_BALANCE, 1).is_err());
    }
}
