//! Strictly aligned with PHPYun `phpyun_admin_announcement` (announcements).
//! Actual PHP columns: id/title/keyword/description/content/datetime/startime/endtime/did/view_num.
//! **PHP has no `status` or `created_at` column** -- `status` is derived from
//! startime/endtime, and `created_at` falls back to `datetime`.

use super::entity::Announcement;
use crate::soft_delete::{self, PREDICATE};
use sqlx::MySqlPool;

/// Map PHPYun columns to Rust Announcement struct fields via aliases.
const SELECT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(keyword, '') AS keyword, \
    COALESCE(description, '') AS description, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(view_num, 0) AS UNSIGNED) AS view_num, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS datetime, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS startime, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS endtime, \
    CAST(IFNULL(NULLIF(did, ''), '0') AS UNSIGNED) AS did, \
    CAST(1 AS SIGNED) AS status, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS created_at";

/// PHPYun's "published" predicate (no status column): startime<=now<endtime
/// (or endtime=0 meaning permanent).
const PUBLISHED_WHERE: &str = " (startime = 0 OR startime <= UNIX_TIMESTAMP()) \
      AND (endtime = 0 OR endtime > UNIX_TIMESTAMP())";

pub async fn list_published(
    pool: &MySqlPool,
    did: u32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Announcement>, sqlx::Error> {
    let sql = if did > 0 {
        format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
             WHERE {PUBLISHED_WHERE} AND (did = ? OR did = -1) AND {PREDICATE} \
             ORDER BY startime DESC, datetime DESC, id DESC LIMIT ? OFFSET ?"
        )
    } else {
        format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
             WHERE {PUBLISHED_WHERE} AND (did = -1 OR did = 0 OR did = '' OR did IS NULL) AND {PREDICATE} \
             ORDER BY startime DESC, datetime DESC, id DESC LIMIT ? OFFSET ?"
        )
    };
    let q = sqlx::query_as::<_, Announcement>(&sql);
    if did > 0 {
        q.bind(did).bind(limit).bind(offset).fetch_all(pool).await
    } else {
        q.bind(limit).bind(offset).fetch_all(pool).await
    }
}

pub async fn count_published(pool: &MySqlPool, did: u32) -> Result<u64, sqlx::Error> {
    let sql = if did > 0 {
        format!("SELECT COUNT(*) FROM phpyun_admin_announcement WHERE {PUBLISHED_WHERE} AND (did = ? OR did = -1) AND {PREDICATE}")
    } else {
        format!("SELECT COUNT(*) FROM phpyun_admin_announcement WHERE {PUBLISHED_WHERE} AND (did = -1 OR did = 0 OR did = '' OR did IS NULL) AND {PREDICATE}")
    };
    let (n,): (i64,) = if did > 0 {
        sqlx::query_as(&sql).bind(did).fetch_one(pool).await?
    } else {
        sqlx::query_as(&sql).fetch_one(pool).await?
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Announcement>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement WHERE id = ? AND {PUBLISHED_WHERE} AND {PREDICATE}");
    sqlx::query_as::<_, Announcement>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Increment view count by 1.
pub async fn incr_view(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("UPDATE phpyun_admin_announcement SET view_num = view_num + 1 WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
    Ok(res.rows_affected())
}

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Announcement>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
         WHERE {PREDICATE} ORDER BY datetime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Announcement>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM phpyun_admin_announcement WHERE {PREDICATE}"))
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct AnnouncementUpsert<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub keyword: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub startime: i64,
    pub endtime: i64,
    pub did: u64,
    pub now: i64,
}

pub async fn upsert(pool: &MySqlPool, a: AnnouncementUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_admin_announcement
               SET title = ?, keyword = ?, description = ?, content = ?,
                   startime = ?, endtime = ?, did = ?
               WHERE id = ?"#,
        )
        .bind(a.title)
        .bind(a.keyword)
        .bind(a.description)
        .bind(a.content)
        .bind(a.startime)
        .bind(a.endtime)
        .bind(a.did)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_admin_announcement
           (title, keyword, description, content, datetime, startime, endtime, did, view_num)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0)"#,
    )
    .bind(a.title)
    .bind(a.keyword)
    .bind(a.description)
    .bind(a.content)
    .bind(a.now)
    .bind(a.startime)
    .bind(a.endtime)
    .bind(a.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_admin_announcement", id).await
}

pub async fn set_did_ids(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("UPDATE phpyun_admin_announcement SET did = ");
    qb.push_bind(did);
    qb.push(" WHERE id IN (");
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

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct Neighbor {
    pub id: u64,
    pub title: String,
}

pub async fn neighbors(
    pool: &MySqlPool,
    id: u64,
    datetime: i64,
) -> Result<(Option<Neighbor>, Option<Neighbor>), sqlx::Error> {
    let prev_sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title \
         FROM phpyun_admin_announcement \
         WHERE {PREDICATE} AND {PUBLISHED_WHERE} \
           AND (datetime < ? OR (datetime = ? AND id < ?)) \
         ORDER BY datetime DESC, id DESC LIMIT 1"
    );
    let next_sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title \
         FROM phpyun_admin_announcement \
         WHERE {PREDICATE} AND {PUBLISHED_WHERE} \
           AND (datetime > ? OR (datetime = ? AND id > ?)) \
         ORDER BY datetime ASC, id ASC LIMIT 1"
    );
    let prev = sqlx::query_as::<_, Neighbor>(&prev_sql)
        .bind(datetime)
        .bind(datetime)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    let next = sqlx::query_as::<_, Neighbor>(&next_sql)
        .bind(datetime)
        .bind(datetime)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok((prev, next))
}
