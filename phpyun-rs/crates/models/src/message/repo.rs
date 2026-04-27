//! `phpyun_sysmsg` repository — message center.
//!
//! Field mapping for the `Message` entity:
//!   fa_uid       → uid
//!   content      → body
//!   remind_status → remind_status (1=unread, 0=read)
//!   ctime        → created_at
//!
//! `category` / `ref_kind` / `ref_id` parameters in earlier signatures are
//! kept as no-ops in the wrapping service so the API shape doesn't change,
//! but PHP's table has no such columns so we ignore them at the SQL layer.

use super::entity::Message;
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(fa_uid AS UNSIGNED) AS uid, \
                             COALESCE(usertype, 0) AS usertype, \
                             COALESCE(content, '') AS body, \
                             COALESCE(remind_status, 0) AS remind_status, \
                             COALESCE(ctime, 0) AS created_at, \
                             username";

pub struct MessageCreate<'a> {
    /// Recipient uid (PHP `fa_uid`).
    pub uid: u64,
    /// Recipient role: 1=jobseeker, 2=employer (PHP `usertype`).
    pub recipient_usertype: u8,
    /// Title prefix — merged into `content` as `"{title}：{body}"`.
    pub title: &'a str,
    /// Body — None becomes empty string. Merged into `content`.
    pub body: Option<&'a str>,
    // Legacy fields — accepted for source-compat but ignored at SQL layer
    // (PHP `phpyun_sysmsg` has no such columns).
    pub category: &'a str,
    pub ref_kind: i32,
    pub ref_id: u64,
}

pub async fn create(
    pool: &MySqlPool,
    c: MessageCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let body = c.body.unwrap_or("");
    let merged: String = if c.title.is_empty() {
        body.to_owned()
    } else if body.is_empty() {
        c.title.to_owned()
    } else {
        format!("{}：{}", c.title, body)
    };
    let _ = (c.category, c.ref_kind, c.ref_id); // intentionally unused
    let res = sqlx::query(
        r#"INSERT INTO phpyun_sysmsg
              (fa_uid, usertype, content, remind_status, ctime)
           VALUES (?, ?, ?, 1, ?)"#,
    )
    .bind(c.uid)
    .bind(c.recipient_usertype as i32)
    .bind(&merged)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list(
    pool: &MySqlPool,
    uid: u64,
    _category: Option<&str>, // PHP table has no category column; ignored
    unread_only: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Message>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(SELECT_FIELDS);
    qb.push(" FROM phpyun_sysmsg WHERE fa_uid = ");
    qb.push_bind(uid);
    if unread_only {
        qb.push(" AND remind_status = 1");
    }
    qb.push(" ORDER BY ctime DESC, id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Message>().fetch_all(pool).await
}

pub async fn count(
    pool: &MySqlPool,
    uid: u64,
    _category: Option<&str>,
    unread_only: bool,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_sysmsg WHERE fa_uid = ");
    qb.push_bind(uid);
    if unread_only {
        qb.push(" AND remind_status = 1");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

/// Mark a single message read. PHP `remind_status` flips 1→0.
pub async fn mark_read(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_sysmsg SET remind_status = 0
          WHERE id = ? AND fa_uid = ? AND remind_status = 1",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn mark_all_read(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_sysmsg SET remind_status = 0
          WHERE fa_uid = ? AND remind_status = 1",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_sysmsg WHERE id = ? AND fa_uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
