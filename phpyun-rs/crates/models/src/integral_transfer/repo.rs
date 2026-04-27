//! Integral / points transfer — aligned with PHPYun `integral_model::company_invtal`.
//!
//! PHP truth:
//! - **Balance** lives in `phpyun_member_statis.integral` (jobseeker, usertype=1)
//!   or `phpyun_company_statis.integral` (employer, usertype=2). It's stored
//!   as VARCHAR(10) but treated as a numeric in app logic — we cast on read.
//! - **Ledger** rows go to `phpyun_company_pay` (despite the name, it
//!   handles both employer and jobseeker rows; `com_id` = affected uid).
//!
//! `transfer(from, to, points, note)` is a between-users transfer:
//!   1. UPDATE phpyun_member_statis SET integral = integral - points WHERE uid = from AND integral >= points
//!   2. UPDATE phpyun_member_statis SET integral = integral + points WHERE uid = to
//!   3. INSERT phpyun_company_pay (one row for from = -points, one row for to = +points)
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
    let mut tx = pool.begin().await?;

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

    // 1. Deduct from sender. PHP stores `integral` as VARCHAR; CAST for arithmetic.
    let deduct = sqlx::query(
        "UPDATE phpyun_member_statis
            SET integral = CAST(integral AS SIGNED) - ?
          WHERE uid = ? AND CAST(integral AS SIGNED) >= ?",
    )
    .bind(points as i64)
    .bind(from_uid)
    .bind(points as i64)
    .execute(&mut *tx)
    .await?;
    if deduct.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(None);
    }

    // 2. Credit recipient (UPSERT — insert a new statis row if missing).
    sqlx::query(
        "INSERT INTO phpyun_member_statis
              (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num)
           VALUES (?, ?, 0, 0, 0, 0, 0)
           ON DUPLICATE KEY UPDATE
              integral = CAST(IFNULL(integral, '0') AS SIGNED) + VALUES(integral)",
    )
    .bind(to_uid)
    .bind(points.to_string())
    .execute(&mut *tx)
    .await?;

    // 3. Two ledger rows: one debit (-points), one credit (+points).
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
    .bind(-(points as i64))
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
    .bind(points as i64)
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
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_pay WHERE com_id = ? AND pay_type = ?",
    )
    .bind(uid)
    .bind(PAY_TYPE_USER_TRANSFER)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
