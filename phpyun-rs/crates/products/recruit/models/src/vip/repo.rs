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
use sqlx::{MySqlPool, QueryBuilder};

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
             AND COALESCE(deleted,0)=0
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
         FROM phpyun_company_rating WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    let Some((id, name, sp, yh, time_start, time_end)) = row else {
        return Ok(None);
    };
    let service_price = parse_finite_price(&sp, "phpyun_company_rating.service_price")?;
    let yh_price = parse_finite_price(&yh, "phpyun_company_rating.yh_price")?;
    Ok(Some(PackagePricing {
        id,
        name,
        service_price,
        yh_price,
        time_start,
        time_end,
    }))
}

fn parse_finite_price(raw: &str, context: &'static str) -> Result<f64, sqlx::Error> {
    let value = raw
        .trim()
        .parse::<f64>()
        .map_err(|error| phpyun_core::numeric::db_conversion_error::<f64>(context, raw, error))?;
    phpyun_core::numeric::finite_to_f64_db(value, context)
}

/// Read company integral balance from `phpyun_company_statis.integral`.
/// Re-exported from the canonical `company_statis::repo`.
pub async fn read_company_integral(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    crate::company_statis::repo::read_integral(pool, uid).await
}

/// Read the company's rating-tier discount (`service_discount`) — applied to
/// `getPackPrice_action`'s computation. Returns `100` (= no discount) when
/// the user has no rating row.
pub async fn read_company_rating_discount(pool: &MySqlPool, uid: u64) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(r.service_discount, 100) AS SIGNED) \
         FROM phpyun_company_statis cs \
         LEFT JOIN phpyun_company_rating r ON r.id = cs.rating AND COALESCE(r.deleted,0)=0 \
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
           FROM phpyun_company_rating WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1"#,
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
    let price_yuan = f64::from(amount_cents) / 100.0;
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

pub async fn cancel_order(pool: &MySqlPool, order_no: &str, uid: u64) -> Result<u64, sqlx::Error> {
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
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_order WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
        q.bind(status.unwrap())
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    } else {
        q.bind(limit).bind(offset).fetch_all(pool).await
    }
}

pub async fn admin_count_orders(pool: &MySqlPool, status: Option<i32>) -> Result<u64, sqlx::Error> {
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_set_order_status(
    pool: &MySqlPool,
    order_no: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_order SET order_state = ? WHERE order_id = ?")
        .bind(status)
        .bind(order_no)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpOrderRow {
    pub id: u64,
    pub uid: u64,
    pub order_id: String,
    pub order_price: String,
    pub r#type: i32,
    pub rating: i32,
    pub order_state: i32,
    pub order_type: String,
    pub order_time: i64,
    pub once_id: i32,
    pub crm_uid: i32,
    pub usertype: i32,
    pub integral: i32,
    pub order_remark: String,
    pub username: String,
    pub comname: String,
    pub rating_name: String,
    pub crm_name: String,
    pub bank_name: String,
    pub bank_id: String,
}

pub struct PhpOrderFilter<'a> {
    pub uid: Option<u64>,
    pub usertype: Option<i32>,
    pub order_type: Option<&'a str>,
    pub order_kind: Option<i32>,
    pub rating: Option<i32>,
    pub order_state: Option<i32>,
    pub order_id_kw: Option<&'a str>,
    pub uid_in: Option<&'a [u64]>,
    pub time_min: Option<i64>,
    pub time_max: Option<i64>,
    pub ids: Option<&'a [u64]>,
}

const PHP_ORDER_FIELDS: &str = "\
    CAST(o.id AS UNSIGNED) AS id, CAST(COALESCE(o.uid,0) AS UNSIGNED) AS uid, \
    COALESCE(o.order_id,'') AS order_id, CAST(COALESCE(o.order_price,0) AS CHAR) AS order_price, \
    CAST(COALESCE(o.`type`,0) AS SIGNED) AS `type`, CAST(COALESCE(o.rating,0) AS SIGNED) AS rating, \
    CAST(COALESCE(o.order_state,0) AS SIGNED) AS order_state, COALESCE(o.order_type,'') AS order_type, \
    CAST(COALESCE(o.order_time,0) AS SIGNED) AS order_time, CAST(COALESCE(o.once_id,0) AS SIGNED) AS once_id, \
    CAST(COALESCE(o.crm_uid,0) AS SIGNED) AS crm_uid, CAST(COALESCE(o.usertype,0) AS SIGNED) AS usertype, \
    CAST(COALESCE(o.integral,0) AS SIGNED) AS integral, COALESCE(o.order_remark,'') AS order_remark, \
    COALESCE(m.username,'') AS username, COALESCE(c.name,'') AS comname, \
    COALESCE(r.name,'') AS rating_name, COALESCE(au.name,'') AS crm_name, \
    COALESCE(o.order_bank,'') AS bank_name, COALESCE(o.order_info,'') AS bank_id";

fn push_php_order_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpOrderFilter<'_>) {
    qb.push(
        " FROM phpyun_company_order o \
         LEFT JOIN phpyun_member m ON m.uid = o.uid \
         LEFT JOIN phpyun_company c ON c.uid = o.uid \
         LEFT JOIN phpyun_company_rating r ON r.id = o.rating \
         LEFT JOIN phpyun_admin_user au ON au.uid = o.crm_uid \
         WHERE 1=1",
    );
    if let Some(uid) = f.uid.filter(|n| *n > 0) {
        qb.push(" AND o.uid = ");
        qb.push_bind(uid);
    }
    if let Some(ut) = f.usertype.filter(|n| *n > 0) {
        qb.push(" AND o.usertype = ");
        qb.push_bind(ut);
    }
    if let Some(ot) = f.order_type.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND o.order_type = ");
        qb.push_bind(ot.to_string());
    }
    if let Some(k) = f.order_kind.filter(|n| *n > 0) {
        qb.push(" AND o.`type` = ");
        qb.push_bind(k);
    }
    if let Some(r) = f.rating.filter(|n| *n > 0) {
        qb.push(" AND o.rating = ");
        qb.push_bind(r);
    }
    if let Some(st) = f.order_state {
        qb.push(" AND o.order_state = ");
        qb.push_bind(st);
    }
    if let Some(kw) = f.order_id_kw.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND o.order_id LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(ids) = f.uid_in.filter(|s| !s.is_empty()) {
        qb.push(" AND o.uid IN (");
        let mut first = true;
        for id in ids {
            if !first {
                qb.push(",");
            }
            qb.push_bind(*id);
            first = false;
        }
        qb.push(")");
    }
    if let Some(ids) = f.ids.filter(|s| !s.is_empty()) {
        qb.push(" AND o.id IN (");
        let mut first = true;
        for id in ids {
            if !first {
                qb.push(",");
            }
            qb.push_bind(*id);
            first = false;
        }
        qb.push(")");
    }
    if let Some(t) = f.time_min.filter(|n| *n > 0) {
        qb.push(" AND o.order_time >= ");
        qb.push_bind(t);
    }
    if let Some(t) = f.time_max.filter(|n| *n > 0) {
        qb.push(" AND o.order_time < ");
        qb.push_bind(t);
    }
}

pub async fn php_list_orders(
    pool: &MySqlPool,
    f: &PhpOrderFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpOrderRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!("SELECT {PHP_ORDER_FIELDS}"));
    push_php_order_where(&mut qb, f);
    qb.push(" ORDER BY o.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_orders(pool: &MySqlPool, f: &PhpOrderFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_php_order_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpOrderSum {
    pub all_price: String,
    pub payed: String,
    pub paying: String,
    pub wait_pay: String,
}

pub async fn php_sum_orders(pool: &MySqlPool, f: &PhpOrderFilter<'_>) -> Result<PhpOrderSum, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(COALESCE(SUM(o.order_price),0) AS CHAR) AS all_price, \
         CAST(COALESCE(SUM(CASE WHEN o.order_state=2 THEN o.order_price ELSE 0 END),0) AS CHAR) AS payed, \
         CAST(COALESCE(SUM(CASE WHEN o.order_state=3 THEN o.order_price ELSE 0 END),0) AS CHAR) AS paying, \
         CAST(COALESCE(SUM(CASE WHEN o.order_state=1 THEN o.order_price ELSE 0 END),0) AS CHAR) AS wait_pay",
    );
    push_php_order_where(&mut qb, f);
    qb.build_query_as().fetch_one(pool).await
}

pub async fn php_find_order(pool: &MySqlPool, id: u64) -> Result<Option<PhpOrderRow>, sqlx::Error> {
    let sql = format!("SELECT {PHP_ORDER_FIELDS} FROM phpyun_company_order o \
         LEFT JOIN phpyun_member m ON m.uid = o.uid \
         LEFT JOIN phpyun_company c ON c.uid = o.uid \
         LEFT JOIN phpyun_company_rating r ON r.id = o.rating \
         LEFT JOIN phpyun_admin_user au ON au.uid = o.crm_uid \
         WHERE o.id = ? LIMIT 1");
    sqlx::query_as::<_, PhpOrderRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn php_update_order(
    pool: &MySqlPool,
    id: u64,
    price: &str,
    remark: &str,
    new_order_id: Option<&str>,
) -> Result<u64, sqlx::Error> {
    if let Some(oid) = new_order_id {
        return Ok(
            sqlx::query(
                "UPDATE phpyun_company_order SET order_price=?, order_remark=?, order_id=? WHERE id=?",
            )
            .bind(price)
            .bind(remark)
            .bind(oid)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
        );
    }
    Ok(
        sqlx::query("UPDATE phpyun_company_order SET order_price=?, order_remark=? WHERE id=?")
            .bind(price)
            .bind(remark)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn php_set_order_state(pool: &MySqlPool, id: u64, state: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company_order SET order_state=? WHERE id=?")
            .bind(state)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn php_delete_orders(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_company_order WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub struct PhpOrderInsert<'a> {
    pub order_id: &'a str,
    pub uid: u64,
    pub order_type: &'a str,
    pub order_price: &'a str,
    pub order_time: i64,
    pub order_state: i32,
    pub order_remark: &'a str,
    pub r#type: i32,
    pub rating: i32,
    pub integral: i32,
    pub usertype: i32,
}

pub async fn php_insert_order(pool: &MySqlPool, a: PhpOrderInsert<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_order \
         (uid, order_id, order_type, order_price, order_time, order_state, order_remark, \
          order_bank, `type`, rating, integral, order_pic, order_info, usertype, is_crm, status) \
         VALUES (?, ?, ?, ?, ?, ?, ?, '', ?, ?, ?, '', '', ?, 0, 2)",
    )
    .bind(a.uid)
    .bind(a.order_id)
    .bind(a.order_type)
    .bind(a.order_price)
    .bind(a.order_time)
    .bind(a.order_state)
    .bind(a.order_remark)
    .bind(a.r#type)
    .bind(a.rating)
    .bind(a.integral)
    .bind(a.usertype)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn search_member_companies(
    pool: &MySqlPool,
    username_like: Option<&str>,
    comname_like: Option<&str>,
) -> Result<Vec<(u64, String, String, String, i64)>, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(m.uid AS UNSIGNED), COALESCE(m.username,''), COALESCE(c.name,''), \
         COALESCE(c.rating_name,''), CAST(COALESCE(c.vipetime,0) AS SIGNED) \
         FROM phpyun_member m LEFT JOIN phpyun_company c ON c.uid = m.uid \
         WHERE m.usertype = 2",
    );
    if let Some(kw) = username_like.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND m.username LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(kw) = comname_like.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY m.uid DESC LIMIT 10");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_member_uids_like(
    pool: &MySqlPool,
    username_like: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE username LIKE ? LIMIT 200",
    )
    .bind(format!("%{username_like}%"))
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn find_company_uids_like(
    pool: &MySqlPool,
    name_like: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_company WHERE name LIKE ? LIMIT 200",
    )
    .bind(format!("%{name_like}%"))
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

#[cfg(test)]
mod tests {
    use super::parse_finite_price;

    #[test]
    fn package_prices_reject_invalid_and_non_finite_database_values() {
        assert_eq!(parse_finite_price("12.5", "price").unwrap(), 12.5);
        for raw in ["", "not-a-price", "NaN", "inf", "-inf"] {
            let error = parse_finite_price(raw, "package.price").unwrap_err();
            assert!(matches!(error, sqlx::Error::Decode(_)));
            assert!(error.to_string().contains("package.price"));
        }
    }
}
