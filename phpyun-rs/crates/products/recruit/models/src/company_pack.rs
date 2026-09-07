//! Member-side 加量包：`phpyun_company_service` + `phpyun_company_service_detail`
//! + `phpyun_company_order.type = 5`.

use crate::admin_gap::entity::{RatingServiceDetailRow, RatingServiceRow};
use crate::admin_gap::extra as gap;
use crate::soft_delete::PREDICATE;
use crate::vip::entity::PayOrder;
use sqlx::MySqlPool;

pub async fn list_visible_services(pool: &MySqlPool) -> Result<Vec<RatingServiceRow>, sqlx::Error> {
    let all = gap::list_rating_services(pool).await?;
    Ok(all.into_iter().filter(|s| s.display == 1).collect())
}

pub async fn list_details(
    pool: &MySqlPool,
    type_id: u64,
) -> Result<Vec<RatingServiceDetailRow>, sqlx::Error> {
    gap::list_rating_details(pool, type_id).await
}

pub async fn find_detail(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<RatingServiceDetailRow>, sqlx::Error> {
    gap::find_rating_detail(pool, id).await
}

pub async fn find_service_name(pool: &MySqlPool, id: u64) -> Result<String, sqlx::Error> {
    let sql = format!(
        "SELECT COALESCE(name,'') FROM phpyun_company_service WHERE id=? AND {PREDICATE} LIMIT 1"
    );
    let row: Option<(String,)> = sqlx::query_as(&sql).bind(id).fetch_optional(pool).await?;
    Ok(row.map(|(n,)| n).unwrap_or_default())
}

const ORDER_SELECT: &str = "
    CAST(id AS UNSIGNED) AS id,
    COALESCE(order_id, '') AS order_no,
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
    COALESCE(order_remark, '') AS package_code,
    CAST(COALESCE(order_price, 0) * 100 AS SIGNED) AS amount_cents,
    COALESCE(order_type, '') AS channel,
    CASE COALESCE(order_state, 0)
        WHEN 1 THEN 0
        WHEN 2 THEN 1
        ELSE COALESCE(order_state, 0)
    END AS status,
    order_bank AS pay_tx_id,
    COALESCE(order_time, 0) AS created_at,
    COALESCE(bank_time, 0) AS paid_at";

/// PHP `buyPackOrder`: `type=5`, `order_state=1` (待付), `rating` = detail id.
pub async fn create_order(
    pool: &MySqlPool,
    order_no: &str,
    uid: u64,
    usertype: i32,
    detail_id: u64,
    remark: &str,
    amount_cents: i32,
    channel: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let price_yuan = f64::from(amount_cents) / 100.0;
    let rating = i32::try_from(detail_id).unwrap_or(0);
    let res = sqlx::query(
        r#"INSERT INTO phpyun_company_order
              (order_id, uid, order_type, order_price, order_time, order_state,
               order_remark, `type`, rating, did, sid, usertype, status,
               order_dkjf, integral, is_invoice, coupon, crm_uid, once_id,
               port, is_crm)
           VALUES (?, ?, ?, ?, ?, 1,
                   ?, 5, ?, 0, 0, ?, 1,
                   0, 0, 0, 0, 0, 0,
                   1, 0)"#,
    )
    .bind(order_no)
    .bind(uid)
    .bind(channel)
    .bind(price_yuan)
    .bind(now)
    .bind(remark)
    .bind(rating)
    .bind(usertype)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_order_by_no(
    pool: &MySqlPool,
    order_no: &str,
) -> Result<Option<PayOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_SELECT} FROM phpyun_company_order WHERE order_id = ? AND type = 5 LIMIT 1"
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
           SET order_state = 2, order_bank = ?, bank_time = ?
           WHERE order_id = ? AND type = 5 AND order_state IN (0, 1)"#,
    )
    .bind(pay_tx_id)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn order_detail_id(pool: &MySqlPool, order_no: &str) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(rating, 0) AS UNSIGNED) FROM phpyun_company_order \
         WHERE order_id = ? AND type = 5 LIMIT 1",
    )
    .bind(order_no)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n).filter(|n| *n > 0))
}
