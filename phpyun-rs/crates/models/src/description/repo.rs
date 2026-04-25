//! Strictly aligned with PHPYun `phpyun_description` + `phpyun_desc_class`.
//!
//! Column mapping:
//!   - Description.class_id <-> nid
//!   - Description.link_url <-> url
//!   - Description.status   = 1 (no such column in PHP)
//!   - Description.created_at <-> ctime
//!   - Description.updated_at <-> ctime (PHP does not maintain updated)
//!   - DescClass.created_at = 0 (PHP `phpyun_desc_class` has no ctime column)

use super::entity::{DescClass, Description};
use sqlx::MySqlPool;

const CLASS_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(0 AS SIGNED) AS created_at";

const DESC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(nid, 0) AS UNSIGNED) AS class_id, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(is_type, 0) AS SIGNED) AS is_type, \
    COALESCE(url, '') AS link_url, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(1 AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS updated_at";

// ---------- classes ----------

pub async fn list_classes(pool: &MySqlPool) -> Result<Vec<DescClass>, sqlx::Error> {
    let sql = format!("SELECT {CLASS_FIELDS} FROM phpyun_desc_class ORDER BY sort ASC, id ASC");
    sqlx::query_as::<_, DescClass>(&sql).fetch_all(pool).await
}

pub async fn insert_class(
    pool: &MySqlPool,
    name: &str,
    sort: i32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO phpyun_desc_class (name, sort) VALUES (?, ?)")
        .bind(name)
        .bind(sort)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn update_class_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_desc_class SET sort = ? WHERE id = ?")
        .bind(sort)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_class(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_desc_class WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ---------- descriptions ----------

pub async fn list(
    pool: &MySqlPool,
    class_id: Option<u64>,
    _only_visible: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Description>, sqlx::Error> {
    // PHPYun has no status column; only_visible has no effect (PHP itself doesn't filter).
    let mut sql = format!("SELECT {DESC_FIELDS} FROM phpyun_description WHERE 1=1");
    if class_id.is_some() {
        sql.push_str(" AND nid = ?");
    }
    sql.push_str(" ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?");
    let mut q = sqlx::query_as::<_, Description>(&sql);
    if let Some(c) = class_id {
        q = q.bind(c);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count(
    pool: &MySqlPool,
    class_id: Option<u64>,
    _only_visible: bool,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from("SELECT COUNT(*) FROM phpyun_description WHERE 1=1");
    if class_id.is_some() {
        sql.push_str(" AND nid = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(c) = class_id {
        q = q.bind(c);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn get(pool: &MySqlPool, id: u64) -> Result<Option<Description>, sqlx::Error> {
    let sql = format!("SELECT {DESC_FIELDS} FROM phpyun_description WHERE id = ?");
    sqlx::query_as::<_, Description>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct UpsertDesc<'a> {
    pub id: Option<u64>,
    pub class_id: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub is_type: i32,
    pub link_url: &'a str,
    pub sort: i32,
    pub status: i32,
}

pub async fn upsert(pool: &MySqlPool, d: &UpsertDesc<'_>, now: i64) -> Result<u64, sqlx::Error> {
    // PHPYun phpyun_description columns: nid/name/url/title/keyword/descs/top_tpl/...
    // Only write the columns that map to Rust-side fields.
    if let Some(id) = d.id {
        sqlx::query(
            "UPDATE phpyun_description \
             SET nid = ?, title = ?, content = ?, is_type = ?, url = ?, sort = ? \
             WHERE id = ?",
        )
        .bind(d.class_id)
        .bind(d.title)
        .bind(d.content)
        .bind(d.is_type)
        .bind(d.link_url)
        .bind(d.sort)
        .bind(id)
        .execute(pool)
        .await?;
        let _ = (d.status, now);
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_description \
             (nid, title, content, is_type, url, sort, ctime) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(d.class_id)
        .bind(d.title)
        .bind(d.content)
        .bind(d.is_type)
        .bind(d.link_url)
        .bind(d.sort)
        .bind(now)
        .execute(pool)
        .await?;
        let _ = d.status;
        Ok(res.last_insert_id())
    }
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_description WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
