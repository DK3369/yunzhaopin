//! Strictly aligned with PHPYun:
//!   phpyun_company_product columns: id/uid/title/pic/body/ctime/status/statusbody/did
//!   phpyun_company_news    columns: id/uid/title/ctime/body/status/statusbody/did
//!
//! Rust field -> PHP column mapping:
//!   CompanyProduct.cover <-> pic; sort = 0; created_at/updated_at <-> ctime
//!   CompanyNews.summary  = first 200 chars of body (PHP has no summary column);
//!                          hits = 0; time <-> ctime

use super::entity::{CompanyNews, CompanyProduct};
use sqlx::MySqlPool;

// ---------- Products ----------

const P_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(title, '') AS title, \
    COALESCE(pic, '') AS cover, \
    COALESCE(body, '') AS body, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS sort, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS updated_at";

pub async fn list_products_public(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyProduct>, sqlx::Error> {
    let sql = format!(
        "SELECT {P_FIELDS} FROM phpyun_company_product \
         WHERE uid = ? AND status = 1 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyProduct>(&sql)
        .bind(com_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_products_public(
    pool: &MySqlPool,
    com_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_product WHERE uid = ? AND status = 1",
    )
    .bind(com_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_product_public(
    pool: &MySqlPool,
    com_uid: u64,
    id: u64,
) -> Result<Option<CompanyProduct>, sqlx::Error> {
    let sql = format!(
        "SELECT {P_FIELDS} FROM phpyun_company_product \
         WHERE id = ? AND uid = ? AND status = 1"
    );
    sqlx::query_as::<_, CompanyProduct>(&sql)
        .bind(id)
        .bind(com_uid)
        .fetch_optional(pool)
        .await
}

pub async fn list_products_own(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyProduct>, sqlx::Error> {
    let sql = format!(
        "SELECT {P_FIELDS} FROM phpyun_company_product \
         WHERE uid = ? AND status != 2 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyProduct>(&sql)
        .bind(com_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_products_own(
    pool: &MySqlPool,
    com_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_product WHERE uid = ? AND status != 2",
    )
    .bind(com_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub struct ProductCreate<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub cover: &'a str,
    pub body: &'a str,
    pub sort: i32,
}

pub async fn create_product(
    pool: &MySqlPool,
    c: ProductCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = c.sort; // PHPYun phpyun_company_product has no sort column
    let res = sqlx::query(
        "INSERT INTO phpyun_company_product (uid, title, pic, body, status, ctime) \
         VALUES (?, ?, ?, ?, 1, ?)",
    )
    .bind(c.uid)
    .bind(c.title)
    .bind(c.cover)
    .bind(c.body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct ProductUpdate<'a> {
    pub title: Option<&'a str>,
    pub cover: Option<&'a str>,
    pub body: Option<&'a str>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn update_product(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    u: ProductUpdate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = u.sort;
    let res = sqlx::query(
        "UPDATE phpyun_company_product SET \
             title  = COALESCE(?, title), \
             pic    = COALESCE(?, pic), \
             body   = COALESCE(?, body), \
             status = COALESCE(?, status) \
         WHERE id = ? AND uid = ? AND status != 2",
    )
    .bind(u.title)
    .bind(u.cover)
    .bind(u.body)
    .bind(u.status)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: UPDATE status=2, no physical DELETE.
pub async fn delete_product(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_product SET status = 2 \
         WHERE id = ? AND uid = ? AND status != 2",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

// ---------- News ----------

const N_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(title, '') AS title, \
    COALESCE(LEFT(body, 200), '') AS summary, \
    COALESCE(body, '') AS body, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(0 AS UNSIGNED) AS hits, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS updated_at";

pub async fn list_news_public(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyNews>, sqlx::Error> {
    let sql = format!(
        "SELECT {N_FIELDS} FROM phpyun_company_news \
         WHERE uid = ? AND status = 1 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyNews>(&sql)
        .bind(com_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_news_public(
    pool: &MySqlPool,
    com_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_news WHERE uid = ? AND status = 1",
    )
    .bind(com_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_news_public(
    pool: &MySqlPool,
    com_uid: u64,
    id: u64,
) -> Result<Option<CompanyNews>, sqlx::Error> {
    let sql = format!(
        "SELECT {N_FIELDS} FROM phpyun_company_news \
         WHERE id = ? AND uid = ? AND status = 1"
    );
    sqlx::query_as::<_, CompanyNews>(&sql)
        .bind(id)
        .bind(com_uid)
        .fetch_optional(pool)
        .await
}

pub async fn incr_news_hit(_pool: &MySqlPool, _id: u64) -> Result<(), sqlx::Error> {
    // PHPYun phpyun_company_news has no hits column, no-op.
    Ok(())
}

pub async fn list_news_own(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyNews>, sqlx::Error> {
    let sql = format!(
        "SELECT {N_FIELDS} FROM phpyun_company_news \
         WHERE uid = ? AND status != 2 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyNews>(&sql)
        .bind(com_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_news_own(pool: &MySqlPool, com_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_news WHERE uid = ? AND status != 2",
    )
    .bind(com_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub struct NewsCreate<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
}

pub async fn create_news(
    pool: &MySqlPool,
    c: NewsCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHPYun phpyun_company_news has no summary column; the supplied summary is dropped.
    let _ = c.summary;
    let res = sqlx::query(
        "INSERT INTO phpyun_company_news (uid, title, body, status, ctime) \
         VALUES (?, ?, ?, 1, ?)",
    )
    .bind(c.uid)
    .bind(c.title)
    .bind(c.body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct NewsUpdate<'a> {
    pub title: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub body: Option<&'a str>,
    pub status: Option<i32>,
}

pub async fn update_news(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    u: NewsUpdate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = u.summary;
    let res = sqlx::query(
        "UPDATE phpyun_company_news SET \
             title  = COALESCE(?, title), \
             body   = COALESCE(?, body), \
             status = COALESCE(?, status) \
         WHERE id = ? AND uid = ? AND status != 2",
    )
    .bind(u.title)
    .bind(u.body)
    .bind(u.status)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: UPDATE status=2, no physical DELETE.
pub async fn delete_news(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_news SET status = 2 \
         WHERE id = ? AND uid = ? AND status != 2",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
