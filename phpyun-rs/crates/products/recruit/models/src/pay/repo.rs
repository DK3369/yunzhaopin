//! `phpyun_rs_pay_*` — gateway merchants, methods, orders.

use super::entity::{
    MerchantWrite, MethodWrite, OrderInsert, OrderListQuery, PayMerchant, PayMethod, PayOrder,
    PayOrderListRow,
};
use sqlx::MySqlPool;

const MERCHANT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(code,'') AS code, \
    COALESCE(name,'') AS name, \
    COALESCE(api_key,'') AS api_key, \
    COALESCE(api_secret,'') AS api_secret, \
    COALESCE(notify_url,'') AS notify_url, \
    COALESCE(return_url,'') AS return_url, \
    COALESCE(allow_ips,'') AS allow_ips, \
    COALESCE(status,'') AS status, \
    CAST(ctime AS SIGNED) AS ctime, \
    CAST(updated_at AS SIGNED) AS updated_at";

const METHOD_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(merchant_id AS UNSIGNED) AS merchant_id, \
    COALESCE(code,'') AS code, \
    COALESCE(name,'') AS name, \
    COALESCE(status,'') AS status, \
    COALESCE(config_json,'') AS config_json, \
    CAST(sort AS SIGNED) AS sort, \
    CAST(ctime AS SIGNED) AS ctime, \
    CAST(updated_at AS SIGNED) AS updated_at";

const ORDER_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(pay_no,'') AS pay_no, \
    CAST(merchant_id AS UNSIGNED) AS merchant_id, \
    COALESCE(merchant_order_no,'') AS merchant_order_no, \
    COALESCE(method_code,'') AS method_code, \
    CAST(amount_cents AS SIGNED) AS amount_cents, \
    COALESCE(currency,'') AS currency, \
    COALESCE(status,'') AS status, \
    COALESCE(channel_ref,'') AS channel_ref, \
    COALESCE(pay_url,'') AS pay_url, \
    COALESCE(subject,'') AS subject, \
    CAST(paid_at AS SIGNED) AS paid_at, \
    CAST(ctime AS SIGNED) AS ctime, \
    CAST(updated_at AS SIGNED) AS updated_at, \
    COALESCE(extra_json,'') AS extra_json";

pub async fn list_merchants(pool: &MySqlPool) -> Result<Vec<PayMerchant>, sqlx::Error> {
    let sql = format!(
        "SELECT {MERCHANT_FIELDS} FROM phpyun_rs_pay_merchant ORDER BY id ASC"
    );
    sqlx::query_as::<_, PayMerchant>(&sql).fetch_all(pool).await
}

pub async fn find_merchant_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<PayMerchant>, sqlx::Error> {
    let sql = format!("SELECT {MERCHANT_FIELDS} FROM phpyun_rs_pay_merchant WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PayMerchant>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_merchant_by_code(
    pool: &MySqlPool,
    code: &str,
) -> Result<Option<PayMerchant>, sqlx::Error> {
    let sql =
        format!("SELECT {MERCHANT_FIELDS} FROM phpyun_rs_pay_merchant WHERE code = ? LIMIT 1");
    sqlx::query_as::<_, PayMerchant>(&sql)
        .bind(code)
        .fetch_optional(pool)
        .await
}

pub async fn find_merchant_by_api_key(
    pool: &MySqlPool,
    api_key: &str,
) -> Result<Option<PayMerchant>, sqlx::Error> {
    let sql =
        format!("SELECT {MERCHANT_FIELDS} FROM phpyun_rs_pay_merchant WHERE api_key = ? LIMIT 1");
    sqlx::query_as::<_, PayMerchant>(&sql)
        .bind(api_key)
        .fetch_optional(pool)
        .await
}

pub async fn insert_merchant(
    pool: &MySqlPool,
    w: MerchantWrite<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_pay_merchant \
         (code, name, api_key, api_secret, notify_url, return_url, allow_ips, status, ctime, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(w.code)
    .bind(w.name)
    .bind(w.api_key)
    .bind(w.api_secret)
    .bind(w.notify_url)
    .bind(w.return_url)
    .bind(w.allow_ips)
    .bind(w.status)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_merchant(
    pool: &MySqlPool,
    id: u64,
    name: &str,
    notify_url: &str,
    return_url: &str,
    allow_ips: &str,
    status: &str,
    api_secret: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(secret) = api_secret {
        let res = sqlx::query(
            "UPDATE phpyun_rs_pay_merchant SET name=?, notify_url=?, return_url=?, allow_ips=?, status=?, \
             api_secret=?, updated_at=? WHERE id=?",
        )
        .bind(name)
        .bind(notify_url)
        .bind(return_url)
        .bind(allow_ips)
        .bind(status)
        .bind(secret)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(res.rows_affected());
    }
    let res = sqlx::query(
        "UPDATE phpyun_rs_pay_merchant SET name=?, notify_url=?, return_url=?, allow_ips=?, status=?, \
         updated_at=? WHERE id=?",
    )
    .bind(name)
    .bind(notify_url)
    .bind(return_url)
    .bind(allow_ips)
    .bind(status)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn set_merchant_status(
    pool: &MySqlPool,
    id: u64,
    status: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_pay_merchant SET status=?, updated_at=? WHERE id=?",
    )
    .bind(status)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_methods(
    pool: &MySqlPool,
    merchant_id: Option<u64>,
) -> Result<Vec<PayMethod>, sqlx::Error> {
    match merchant_id {
        Some(mid) => {
            let sql = format!(
                "SELECT {METHOD_FIELDS} FROM phpyun_rs_pay_method WHERE merchant_id = ? \
                 ORDER BY sort ASC, id ASC"
            );
            sqlx::query_as::<_, PayMethod>(&sql)
                .bind(mid)
                .fetch_all(pool)
                .await
        }
        None => {
            let sql = format!(
                "SELECT {METHOD_FIELDS} FROM phpyun_rs_pay_method ORDER BY merchant_id ASC, sort ASC, id ASC"
            );
            sqlx::query_as::<_, PayMethod>(&sql).fetch_all(pool).await
        }
    }
}

pub async fn list_active_methods(
    pool: &MySqlPool,
    merchant_id: u64,
) -> Result<Vec<PayMethod>, sqlx::Error> {
    let sql = format!(
        "SELECT {METHOD_FIELDS} FROM phpyun_rs_pay_method \
         WHERE merchant_id = ? AND status = 'active' ORDER BY sort ASC, id ASC"
    );
    sqlx::query_as::<_, PayMethod>(&sql)
        .bind(merchant_id)
        .fetch_all(pool)
        .await
}

pub async fn find_method_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<PayMethod>, sqlx::Error> {
    let sql = format!("SELECT {METHOD_FIELDS} FROM phpyun_rs_pay_method WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PayMethod>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_method(
    pool: &MySqlPool,
    merchant_id: u64,
    code: &str,
) -> Result<Option<PayMethod>, sqlx::Error> {
    let sql = format!(
        "SELECT {METHOD_FIELDS} FROM phpyun_rs_pay_method \
         WHERE merchant_id = ? AND code = ? LIMIT 1"
    );
    sqlx::query_as::<_, PayMethod>(&sql)
        .bind(merchant_id)
        .bind(code)
        .fetch_optional(pool)
        .await
}

pub async fn method_config_by_merchant_code(
    pool: &MySqlPool,
    merchant_code: &str,
    method_code: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_as::<_, (String,)>(
        "SELECT COALESCE(x.config_json,'') FROM phpyun_rs_pay_method x \
         INNER JOIN phpyun_rs_pay_merchant m ON m.id = x.merchant_id \
         WHERE m.code = ? AND x.code = ? LIMIT 1",
    )
    .bind(merchant_code)
    .bind(method_code)
    .fetch_optional(pool)
    .await
    .map(|r| r.map(|x| x.0))
}

pub async fn insert_method(
    pool: &MySqlPool,
    w: MethodWrite<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_pay_method \
         (merchant_id, code, name, status, config_json, sort, ctime, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(w.merchant_id)
    .bind(w.code)
    .bind(w.name)
    .bind(w.status)
    .bind(w.config_json)
    .bind(w.sort)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_method(
    pool: &MySqlPool,
    id: u64,
    name: &str,
    status: &str,
    config_json: &str,
    sort: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_pay_method SET name=?, status=?, config_json=?, sort=?, updated_at=? \
         WHERE id=?",
    )
    .bind(name)
    .bind(status)
    .bind(config_json)
    .bind(sort)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn set_method_status(
    pool: &MySqlPool,
    id: u64,
    status: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_rs_pay_method SET status=?, updated_at=? WHERE id=?")
        .bind(status)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn set_method_config_json(
    pool: &MySqlPool,
    id: u64,
    config_json: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_rs_pay_method SET config_json=?, updated_at=? WHERE id=?")
        .bind(config_json)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_method(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_rs_pay_method WHERE id=?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn count_pending_for_method(
    pool: &MySqlPool,
    merchant_id: u64,
    method_code: &str,
) -> Result<i64, sqlx::Error> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_rs_pay_order \
         WHERE merchant_id = ? AND method_code = ? AND status = 'pending'",
    )
    .bind(merchant_id)
    .bind(method_code)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

pub async fn find_order_by_pay_no(
    pool: &MySqlPool,
    pay_no: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    let sql = format!("SELECT {ORDER_FIELDS} FROM phpyun_rs_pay_order WHERE pay_no = ? LIMIT 1");
    sqlx::query_as::<_, PayOrder>(&sql)
        .bind(pay_no)
        .fetch_optional(pool)
        .await
}

pub async fn find_order_by_channel_ref(
    pool: &MySqlPool,
    channel_ref: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    if channel_ref.trim().is_empty() {
        return Ok(None);
    }
    let sql =
        format!("SELECT {ORDER_FIELDS} FROM phpyun_rs_pay_order WHERE channel_ref = ? LIMIT 1");
    sqlx::query_as::<_, PayOrder>(&sql)
        .bind(channel_ref)
        .fetch_optional(pool)
        .await
}

pub async fn find_order_by_merchant_ref(
    pool: &MySqlPool,
    merchant_id: u64,
    merchant_order_no: &str,
    method_code: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_FIELDS} FROM phpyun_rs_pay_order \
         WHERE merchant_id = ? AND merchant_order_no = ? AND method_code = ? LIMIT 1"
    );
    sqlx::query_as::<_, PayOrder>(&sql)
        .bind(merchant_id)
        .bind(merchant_order_no)
        .bind(method_code)
        .fetch_optional(pool)
        .await
}

pub async fn insert_order(
    pool: &MySqlPool,
    w: OrderInsert<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_pay_order \
         (pay_no, merchant_id, merchant_order_no, method_code, amount_cents, currency, \
          status, channel_ref, pay_url, subject, paid_at, ctime, updated_at, extra_json) \
         VALUES (?, ?, ?, ?, ?, ?, 'pending', NULL, '', ?, 0, ?, ?, '{}')",
    )
    .bind(w.pay_no)
    .bind(w.merchant_id)
    .bind(w.merchant_order_no)
    .bind(w.method_code)
    .bind(w.amount_cents)
    .bind(w.currency)
    .bind(w.subject)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_checkout(
    pool: &MySqlPool,
    pay_no: &str,
    pay_url: &str,
    channel_ref: &str,
    extra_json: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE phpyun_rs_pay_order SET pay_url=?, channel_ref=?, extra_json=?, updated_at=? \
         WHERE pay_no=?",
    )
    .bind(pay_url)
    .bind(empty_to_null(channel_ref))
    .bind(extra_json)
    .bind(now)
    .bind(pay_no)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_paid(
    pool: &MySqlPool,
    pay_no: &str,
    channel_ref: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_pay_order SET status='paid', paid_at=?, channel_ref=COALESCE(?, channel_ref), \
         updated_at=? WHERE pay_no=? AND status='pending'",
    )
    .bind(now)
    .bind(empty_to_null(channel_ref))
    .bind(now)
    .bind(pay_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn mark_status(
    pool: &MySqlPool,
    pay_no: &str,
    status: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_pay_order SET status=?, updated_at=? WHERE pay_no=? AND status='pending'",
    )
    .bind(status)
    .bind(now)
    .bind(pay_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

fn push_order_filters(sql: &mut String, q: &OrderListQuery<'_>) {
    if q.merchant_code.is_some() {
        sql.push_str(" AND m.code = ?");
    }
    if q.method_code.is_some() {
        sql.push_str(" AND o.method_code = ?");
    }
    if q.status.is_some() {
        sql.push_str(" AND o.status = ?");
    }
    if q.pay_no.is_some() {
        sql.push_str(" AND o.pay_no = ?");
    }
    if q.merchant_order_no.is_some() {
        sql.push_str(" AND o.merchant_order_no = ?");
    }
    if q.channel_ref.is_some() {
        sql.push_str(" AND o.channel_ref = ?");
    }
    if q.ctime_from.is_some() {
        sql.push_str(" AND o.ctime >= ?");
    }
    if q.ctime_to.is_some() {
        sql.push_str(" AND o.ctime <= ?");
    }
}

pub async fn list_orders(
    pool: &MySqlPool,
    q: &OrderListQuery<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PayOrderListRow>, sqlx::Error> {
    let mut sql = String::from(
        "SELECT CAST(o.id AS UNSIGNED) AS id, COALESCE(o.pay_no,'') AS pay_no, \
         CAST(o.merchant_id AS UNSIGNED) AS merchant_id, COALESCE(m.code,'') AS merchant_code, \
         COALESCE(m.name,'') AS merchant_name, COALESCE(o.merchant_order_no,'') AS merchant_order_no, \
         COALESCE(o.method_code,'') AS method_code, CAST(o.amount_cents AS SIGNED) AS amount_cents, \
         COALESCE(o.currency,'') AS currency, COALESCE(o.status,'') AS status, \
         COALESCE(o.channel_ref,'') AS channel_ref, COALESCE(o.pay_url,'') AS pay_url, \
         COALESCE(o.subject,'') AS subject, \
         CAST(o.paid_at AS SIGNED) AS paid_at, CAST(o.ctime AS SIGNED) AS ctime \
         FROM phpyun_rs_pay_order o \
         INNER JOIN phpyun_rs_pay_merchant m ON m.id = o.merchant_id WHERE 1=1",
    );
    push_order_filters(&mut sql, q);
    sql.push_str(" ORDER BY o.id DESC LIMIT ? OFFSET ?");
    let mut qb = sqlx::query_as::<_, PayOrderListRow>(&sql);
    if let Some(c) = q.merchant_code {
        qb = qb.bind(c);
    }
    if let Some(c) = q.method_code {
        qb = qb.bind(c);
    }
    if let Some(s) = q.status {
        qb = qb.bind(s);
    }
    if let Some(v) = q.pay_no {
        qb = qb.bind(v);
    }
    if let Some(v) = q.merchant_order_no {
        qb = qb.bind(v);
    }
    if let Some(v) = q.channel_ref {
        qb = qb.bind(v);
    }
    if let Some(v) = q.ctime_from {
        qb = qb.bind(v);
    }
    if let Some(v) = q.ctime_to {
        qb = qb.bind(v);
    }
    qb.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count_orders(pool: &MySqlPool, q: &OrderListQuery<'_>) -> Result<u64, sqlx::Error> {
    let mut sql = String::from(
        "SELECT COUNT(*) FROM phpyun_rs_pay_order o \
         INNER JOIN phpyun_rs_pay_merchant m ON m.id = o.merchant_id WHERE 1=1",
    );
    push_order_filters(&mut sql, q);
    let mut qb = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(c) = q.merchant_code {
        qb = qb.bind(c);
    }
    if let Some(c) = q.method_code {
        qb = qb.bind(c);
    }
    if let Some(s) = q.status {
        qb = qb.bind(s);
    }
    if let Some(v) = q.pay_no {
        qb = qb.bind(v);
    }
    if let Some(v) = q.merchant_order_no {
        qb = qb.bind(v);
    }
    if let Some(v) = q.channel_ref {
        qb = qb.bind(v);
    }
    if let Some(v) = q.ctime_from {
        qb = qb.bind(v);
    }
    if let Some(v) = q.ctime_to {
        qb = qb.bind(v);
    }
    let n = qb.fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}

fn empty_to_null(s: &str) -> Option<&str> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}
