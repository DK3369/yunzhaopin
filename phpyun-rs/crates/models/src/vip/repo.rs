//! VIP packages, user-VIP state, and pay orders.
//!
//! ## Schema reality check
//!
//! - `phpyun_company_rating` (PHP) is the **VIP-package config table** (43 cols
//!   including `service_price`, `integral_buy`, `time_start/end`, `sons_num`,
//!   etc.). NOT the user-rating table.
//! - `phpyun_company_order` (PHP) is the actual order ledger with 28 cols:
//!   `id, uid, order_id (varchar18), order_type (varchar25), order_price (double),
//!   order_time, order_state, order_remark (text), order_bank, bank_time, type,
//!   rating, integral, is_invoice, coupon, did, sid, order_pic, order_info,
//!   rewardid, crm_uid, once_id, fast, usertype, port, is_crm, status, order_dkjf`.
//!
//! Rust DTO ↔ PHP column mapping for orders:
//!
//! | Rust DTO field | PHP column        | Notes                                    |
//! |----------------|-------------------|------------------------------------------|
//! | `order_no`     | `order_id`        | varchar(18) business order id            |
//! | `amount_cents` | `order_price`     | PHP stores yuan as DOUBLE — convert ÷100 |
//! | `channel`      | `order_type`      | varchar(25) e.g. "wechat" / "alipay"     |
//! | `pay_tx_id`    | `order_bank`      | varchar(150) txid from payment gateway   |
//! | `created_at`   | `order_time`      | unix seconds                             |
//! | `paid_at`      | `bank_time`       | unix seconds                             |
//! | `status`       | `order_state`     | 0=pending, 1=paid, 2=cancelled, 3=refund |
//! | `package_code` | `order_remark`    | text — also stored in `order_info` JSON  |
//! | `uid`          | `uid`             | direct                                   |
//!
//! `phpyun_admin_template` is **NOT** the VIP package table either — it's the
//! admin theme/template config. The Rust `phpyun_company_rating` SELECT below
//! correctly targets the VIP-package config.

use super::entity::{PayOrder, UserVip, VipPackage};
use sqlx::MySqlPool;

// ==================== Packages (phpyun_company_rating = VIP tier config) ====================

/// VIP package list (active = `time_end == 0` OR `time_end > now`; `display=1`).
/// PHP `phpyun_company_rating` has no `code` field — we synthesize one from
/// `id` (`pkg_<id>`) so the API layer's `package_code` indirection still works.
pub async fn list_active_packages(
    pool: &MySqlPool,
    usertype: i32,
) -> Result<Vec<VipPackage>, sqlx::Error> {
    let now = phpyun_core::clock::now_ts();
    sqlx::query_as::<_, VipPackage>(
        r#"SELECT
              CAST(id AS UNSIGNED) AS id,
              CONCAT('pkg_', id) AS code,
              COALESCE(name, '') AS name,
              COALESCE(`type`, 0) AS target_usertype,
              COALESCE(service_time, 0) AS duration_days,
              CAST(COALESCE(service_price, 0) * 100 AS SIGNED) AS price_cents,
              NULL AS desc_json,
              COALESCE(display, 1) AS is_active,
              COALESCE(sort, 0) AS sort_order,
              COALESCE(time_start, 0) AS created_at
           FROM phpyun_company_rating
           WHERE COALESCE(display, 1) = 1
             AND (`type` = 0 OR `type` = ?)
             AND (COALESCE(time_end, 0) = 0 OR time_end > ?)
           ORDER BY sort ASC, service_price ASC"#,
    )
    .bind(usertype)
    .bind(now)
    .fetch_all(pool)
    .await
}

// ==================== Pricing quote (PHPYun `getVipPrice` semantics) ====================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackagePricing {
    pub id: u64,
    pub name: String,
    pub service_price: f64,
    /// Discounted price when the row's `time_start < now < time_end`. Decoded
    /// as `0` when no promo window is configured.
    pub yh_price: f64,
    pub time_start: i64,
    pub time_end: i64,
}

/// Raw `phpyun_company_rating` price columns for the price-quote endpoint.
/// PHPYun stores prices as VARCHAR; we read them as strings and parse to f64
/// (MySQL doesn't accept `CAST AS DOUBLE` and the bigdecimal feature would
/// pull in extra deps).
pub async fn find_package_pricing(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<PackagePricing>, sqlx::Error> {
    let row: Option<(u64, String, String, String, i64, i64)> = sqlx::query_as(
        "SELECT \
            CAST(id AS UNSIGNED), \
            COALESCE(name, ''), \
            COALESCE(service_price, '0'), \
            COALESCE(yh_price, '0'), \
            CAST(COALESCE(time_start, 0) AS SIGNED), \
            CAST(COALESCE(time_end, 0) AS SIGNED) \
         FROM phpyun_company_rating WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(id, name, sp, yh, ts, te)| PackagePricing {
        id,
        name,
        service_price: sp.parse::<f64>().unwrap_or(0.0),
        yh_price: yh.parse::<f64>().unwrap_or(0.0),
        time_start: ts,
        time_end: te,
    }))
}

/// Read company integral balance from `phpyun_company_statis.integral`.
/// Re-exported from the canonical `company_statis::repo`.
pub async fn read_company_integral(
    pool: &MySqlPool,
    uid: u64,
) -> Result<i64, sqlx::Error> {
    crate::company_statis::repo::read_integral(pool, uid).await
}

/// Read the company's rating-tier discount (`service_discount`) — applied to
/// `getPackPrice_action`'s computation. Returns `100` (= no discount) when
/// the user has no rating row.
pub async fn read_company_rating_discount(
    pool: &MySqlPool,
    uid: u64,
) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(r.service_discount, 100) AS SIGNED) \
         FROM phpyun_company_statis cs \
         LEFT JOIN phpyun_company_rating r ON r.id = cs.rating \
         WHERE cs.uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(d,)| if d <= 0 { 100 } else { d }).unwrap_or(100))
}

pub async fn find_package_by_code(
    pool: &MySqlPool,
    code: &str,
) -> Result<Option<VipPackage>, sqlx::Error> {
    // Synthesized code is "pkg_<id>". Strip the prefix to recover the id.
    let id: u64 = match code.strip_prefix("pkg_").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return Ok(None),
    };
    sqlx::query_as::<_, VipPackage>(
        r#"SELECT
              CAST(id AS UNSIGNED) AS id,
              CONCAT('pkg_', id) AS code,
              COALESCE(name, '') AS name,
              COALESCE(`type`, 0) AS target_usertype,
              COALESCE(service_time, 0) AS duration_days,
              CAST(COALESCE(service_price, 0) * 100 AS SIGNED) AS price_cents,
              NULL AS desc_json,
              COALESCE(display, 1) AS is_active,
              COALESCE(sort, 0) AS sort_order,
              COALESCE(time_start, 0) AS created_at
           FROM phpyun_company_rating WHERE id = ? LIMIT 1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// ==================== User VIP state ====================
// Rust-only auxiliary table (`phpyun_rs_user_vip`). PHP keeps VIP state on
// `phpyun_company.vipstime/vipetime/rating` — but those are per-employer
// only and don't generalize to jobseeker VIPs. Keeping a Rust-side table
// is the simpler portable choice; if you want strict PHP parity later,
// migrate this to read from `phpyun_company`.

pub async fn find_user_vip(pool: &MySqlPool, uid: u64) -> Result<Option<UserVip>, sqlx::Error> {
    // `phpyun_rs_user_vip` is Rust-port-only — when not provisioned, return
    // Ok(None) so the handler reports "no active VIP" instead of 5xx.
    let r = sqlx::query_as::<_, UserVip>(
        "SELECT uid, package_code, started_at, expires_at, updated_at FROM phpyun_rs_user_vip WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn upsert_user_vip(
    pool: &MySqlPool,
    uid: u64,
    package_code: &str,
    duration_secs: i64,
    now: i64,
) -> Result<(), sqlx::Error> {
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

// ==================== Orders (phpyun_company_order = real PHP ledger) ====================

const ORDER_SELECT: &str = "
    CAST(id AS UNSIGNED) AS id,
    COALESCE(order_id, '') AS order_no,
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
    COALESCE(order_remark, '') AS package_code,
    CAST(COALESCE(order_price, 0) * 100 AS SIGNED) AS amount_cents,
    COALESCE(order_type, '') AS channel,
    COALESCE(order_state, 0) AS status,
    order_bank AS pay_tx_id,
    COALESCE(order_time, 0) AS created_at,
    COALESCE(bank_time, 0) AS paid_at";

pub async fn create_order(
    pool: &MySqlPool,
    order_no: &str,
    uid: u64,
    package_code: &str,
    amount_cents: i32,
    channel: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP `order_price` is DOUBLE in yuan; convert from cents.
    let price_yuan = (amount_cents as f64) / 100.0;
    // PHP `rating` is the FK to phpyun_company_rating.id; recover from "pkg_<id>" code.
    let rating: i32 = package_code
        .strip_prefix("pkg_")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let res = sqlx::query(
        r#"INSERT INTO phpyun_company_order
              (order_id, uid, order_type, order_price, order_time, order_state,
               order_remark, `type`, rating, did, sid, usertype, status,
               order_dkjf, integral, is_invoice, coupon, crm_uid, once_id,
               port, is_crm)
           VALUES (?, ?, ?, ?, ?, 0,
                   ?, 1, ?, 0, 0, 0, 1,
                   0, 0, 0, 0, 0, 0,
                   1, 0)"#,
    )
    .bind(order_no)
    .bind(uid)
    .bind(channel)
    .bind(price_yuan)
    .bind(now)
    .bind(package_code)
    .bind(rating)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_order_by_no(
    pool: &MySqlPool,
    order_no: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_SELECT}
           FROM phpyun_company_order WHERE order_id = ? LIMIT 1"
    );
    sqlx::query_as::<_, PayOrder>(&sql)
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
           SET order_state = 1, order_bank = ?, bank_time = ?
           WHERE order_id = ? AND order_state = 0"#,
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
    let sql = format!(
        "SELECT {ORDER_SELECT}
           FROM phpyun_company_order
           WHERE uid = ?
           ORDER BY order_time DESC, id DESC
           LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, PayOrder>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn cancel_order(
    pool: &MySqlPool,
    order_no: &str,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_order
           SET order_state = 2
           WHERE order_id = ? AND uid = ? AND order_state = 0"#,
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
    let (sql, with_status) = match status {
        Some(_) => (
            format!(
                "SELECT {ORDER_SELECT}
                   FROM phpyun_company_order
                   WHERE order_state = ?
                   ORDER BY order_time DESC LIMIT ? OFFSET ?"
            ),
            true,
        ),
        None => (
            format!(
                "SELECT {ORDER_SELECT}
                   FROM phpyun_company_order
                   ORDER BY order_time DESC LIMIT ? OFFSET ?"
            ),
            false,
        ),
    };
    let q = sqlx::query_as::<_, PayOrder>(&sql);
    if with_status {
        q.bind(status.unwrap()).bind(limit).bind(offset).fetch_all(pool).await
    } else {
        q.bind(limit).bind(offset).fetch_all(pool).await
    }
}

pub async fn admin_count_orders(
    pool: &MySqlPool,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match status {
        Some(s) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_order WHERE order_state = ?")
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

pub async fn admin_set_order_status(
    pool: &MySqlPool,
    order_no: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_order SET order_state = ? WHERE order_id = ?",
    )
    .bind(status)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
