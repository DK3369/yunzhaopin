use super::entity::{PayOrder, UserVip, VipPackage};
use sqlx::MySqlPool;

// ==================== Packages ====================

pub async fn list_active_packages(
    pool: &MySqlPool,
    usertype: i32,
) -> Result<Vec<VipPackage>, sqlx::Error> {
    sqlx::query_as::<_, VipPackage>(
        r#"SELECT id, code, name, target_usertype, duration_days, price_cents,
                  desc_json, is_active, sort_order, created_at
           FROM phpyun_admin_template
           WHERE is_active = 1 AND (target_usertype = 0 OR target_usertype = ?)
           ORDER BY sort_order ASC, price_cents ASC"#,
    )
    .bind(usertype)
    .fetch_all(pool)
    .await
}

pub async fn find_package_by_code(
    pool: &MySqlPool,
    code: &str,
) -> Result<Option<VipPackage>, sqlx::Error> {
    sqlx::query_as::<_, VipPackage>(
        r#"SELECT id, code, name, target_usertype, duration_days, price_cents,
                  desc_json, is_active, sort_order, created_at
           FROM phpyun_admin_template WHERE code = ? LIMIT 1"#,
    )
    .bind(code)
    .fetch_optional(pool)
    .await
}

// ==================== User VIP state ====================

pub async fn find_user_vip(pool: &MySqlPool, uid: u64) -> Result<Option<UserVip>, sqlx::Error> {
    sqlx::query_as::<_, UserVip>(
        "SELECT uid, package_code, started_at, expires_at, updated_at FROM phpyun_rs_user_vip WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// Activate / renew — if a record already exists and has not expired, accumulate the renewal; otherwise reset `started_at`.
pub async fn upsert_user_vip(
    pool: &MySqlPool,
    uid: u64,
    package_code: &str,
    duration_secs: i64,
    now: i64,
) -> Result<(), sqlx::Error> {
    // ON DUPLICATE KEY: renewal = GREATEST(expires_at, now) + duration
    sqlx::query(
        r#"INSERT INTO phpyun_rs_user_vip (uid, package_code, started_at, expires_at, updated_at)
           VALUES (?, ?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE
             package_code = VALUES(package_code),
             expires_at   = GREATEST(expires_at, VALUES(started_at)) + ?,
             updated_at   = VALUES(updated_at)"#,
    )
    .bind(uid)
    .bind(package_code)
    .bind(now)
    .bind(now + duration_secs)
    .bind(now)
    .bind(duration_secs)
    .execute(pool)
    .await?;
    Ok(())
}

// ==================== Orders ====================

pub async fn create_order(
    pool: &MySqlPool,
    order_no: &str,
    uid: u64,
    package_code: &str,
    amount_cents: i32,
    channel: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_company_order
           (order_no, uid, package_code, amount_cents, channel, status, created_at, paid_at)
           VALUES (?, ?, ?, ?, ?, 0, ?, 0)"#,
    )
    .bind(order_no)
    .bind(uid)
    .bind(package_code)
    .bind(amount_cents)
    .bind(channel)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_order_by_no(
    pool: &MySqlPool,
    order_no: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    sqlx::query_as::<_, PayOrder>(
        r#"SELECT id, order_no, uid, package_code, amount_cents, channel, status,
                  pay_tx_id, created_at, paid_at
           FROM phpyun_company_order WHERE order_no = ? LIMIT 1"#,
    )
    .bind(order_no)
    .fetch_optional(pool)
    .await
}

pub async fn mark_order_paid(
    pool: &MySqlPool,
    order_no: &str,
    pay_tx_id: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_order
           SET status = 1, pay_tx_id = ?, paid_at = ?
           WHERE order_no = ? AND status = 0"#,
    )
    .bind(pay_tx_id)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_user_orders(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<PayOrder>, sqlx::Error> {
    sqlx::query_as::<_, PayOrder>(
        r#"SELECT id, order_no, uid, package_code, amount_cents, channel, status,
                  pay_tx_id, created_at, paid_at
           FROM phpyun_company_order
           WHERE uid = ?
           ORDER BY created_at DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Cancels an unpaid order (status: 0=pending payment → 2=cancelled).
/// Includes a uid constraint to prevent cross-user cancellation. affected=0 → order does not exist / already paid / already cancelled.
pub async fn cancel_order(
    pool: &MySqlPool,
    order_no: &str,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_order
           SET status = 2
           WHERE order_no = ? AND uid = ? AND status = 0"#,
    )
    .bind(order_no)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn count_user_orders(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_order WHERE uid = ?",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

// ---------- Admin backend ----------

pub async fn admin_list_orders(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PayOrder>, sqlx::Error> {
    let sql = match status {
        Some(_) => r#"SELECT id, order_no, uid, package_code, amount_cents, channel, status,
                             pay_tx_id, created_at, paid_at
                      FROM phpyun_company_order
                      WHERE status = ?
                      ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        None => r#"SELECT id, order_no, uid, package_code, amount_cents, channel, status,
                          pay_tx_id, created_at, paid_at
                   FROM phpyun_company_order
                   ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
    };
    let q = sqlx::query_as::<_, PayOrder>(sql);
    match status {
        Some(s) => q.bind(s).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn admin_count_orders(
    pool: &MySqlPool,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match status {
        Some(s) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_order WHERE status = ?")
                .bind(s)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_order")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}

/// Changes an order's status: `2=refunded / 3=cancelled`. `status=0` (pending payment) cannot be transitioned to paid from here (use `mark_order_paid` instead).
pub async fn admin_set_order_status(
    pool: &MySqlPool,
    order_no: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_order SET status = ? WHERE order_no = ?",
    )
    .bind(status)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
