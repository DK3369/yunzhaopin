//! Strictly aligned with PHPYun `phpyun_gongzhao` (public recruitment).
//!
//! PHP columns: id/title/keyword/description/content/datetime/did/startime/endtime/pic/rec
//! Rust Gongzhao field -> PHP column:
//!   - cover      <-> pic
//!   - body       <-> content
//!   - tag        <-> keyword
//!   - status     = 1 (no status column in PHP)
//!   - view_count = 0 (no view-count column in PHP; Rust stub)
//!   - start_at   <-> startime
//!   - end_at     <-> endtime
//!   - created_at <-> datetime

use super::entity::Gongzhao;
use crate::soft_delete::{self, PREDICATE};
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(description, '') AS description, \
    COALESCE(pic, '') AS cover, \
    COALESCE(content, '') AS body, \
    COALESCE(keyword, '') AS tag, \
    CAST(1 AS SIGNED) AS status, \
    CAST(0 AS UNSIGNED) AS view_count, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS created_at";

pub async fn list(
    pool: &MySqlPool,
    tag: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Gongzhao>, sqlx::Error> {
    let sql = match tag {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao WHERE keyword = ? AND {PREDICATE} \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao WHERE {PREDICATE} \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Gongzhao>(&sql);
    match tag {
        Some(t) => q.bind(t).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count(pool: &MySqlPool, tag: Option<&str>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match tag {
        Some(t) => {
            sqlx::query_as(&format!(
                "SELECT COUNT(*) FROM phpyun_gongzhao WHERE keyword = ? AND {PREDICATE}"
            ))
                .bind(t)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as(&format!("SELECT COUNT(*) FROM phpyun_gongzhao WHERE {PREDICATE}"))
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Gongzhao>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_gongzhao WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Gongzhao>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_view(_pool: &MySqlPool, _id: u64) -> Result<(), sqlx::Error> {
    // PHPYun phpyun_gongzhao has no view-count column; this op is a no-op.
    Ok(())
}

pub struct GongzhaoUpsert<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub keyword: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub pic: &'a str,
    pub startime: i64,
    pub endtime: i64,
    pub did: i32,
    pub now: i64,
}

pub async fn upsert(pool: &MySqlPool, a: GongzhaoUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_gongzhao
               SET title = ?, keyword = ?, description = ?, content = ?, pic = ?,
                   startime = ?, endtime = ?
               WHERE id = ?"#,
        )
        .bind(a.title)
        .bind(a.keyword)
        .bind(a.description)
        .bind(a.content)
        .bind(a.pic)
        .bind(a.startime)
        .bind(a.endtime)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_gongzhao
           (title, keyword, description, content, datetime, did, startime, endtime, pic, rec)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)"#,
    )
    .bind(a.title)
    .bind(a.keyword)
    .bind(a.description)
    .bind(a.content)
    .bind(a.now)
    .bind(a.did)
    .bind(a.startime)
    .bind(a.endtime)
    .bind(a.pic)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_gongzhao", id).await
}
