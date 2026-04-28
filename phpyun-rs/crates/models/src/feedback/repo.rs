//! `phpyun_advice_question` repository — PHPYun's actual feedback queue.
//!
//! Schema (PHP truth): `id, username, infotype, content, mobile, ctime,
//! email, handlecontent, status`. The Rust `Feedback` entity uses
//! `category / contact / client_ip / created_at` shape; this repo maps via
//! SELECT aliases.
//!
//! Caveats:
//! - PHP has no `uid` column → `list_by_user` returns an empty list because
//!   we cannot scope feedback to a specific user from this table alone.
//!   The user-side "my feedback" page therefore reads as empty until the
//!   product wires a `username` filter through.
//! - PHP has no `client_ip` column → exposed as empty string.

use super::entity::Feedback;
use sqlx::MySqlPool;

// SELECT aliases map PHP cols onto the Rust `Feedback` entity field names.
const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             NULL AS uid, \
                             COALESCE(CAST(infotype AS CHAR), '') AS category, \
                             COALESCE(content, '') AS content, \
                             COALESCE(mobile, '') AS contact, \
                             '' AS client_ip, \
                             status, \
                             COALESCE(ctime, 0) AS created_at";

pub struct FeedbackCreate<'a> {
    pub uid: Option<u64>,
    pub category: &'a str,
    pub content: &'a str,
    pub contact: &'a str,
    pub client_ip: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    c: FeedbackCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP `infotype` is an int; if `category` parses as an int we use it,
    // otherwise default to 0.
    let infotype: Option<i32> = c.category.parse().ok();
    let _ = (c.uid, c.client_ip); // PHP has no column for these — silently dropped
    let res = sqlx::query(
        r#"INSERT INTO phpyun_advice_question
           (username, infotype, content, mobile, ctime, status)
           VALUES (?, ?, ?, ?, ?, 0)"#,
    )
    .bind("")              // username — caller doesn't have it; PHP filled in from session
    .bind(infotype.unwrap_or(0))
    .bind(c.content)
    .bind(c.contact)       // contact lands in `mobile` (PHP convention)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// User-side "my feedback" list. PHP table has no `uid` column, so this
/// always returns empty (handler shape preserved).
pub async fn list_by_user(
    _pool: &MySqlPool,
    _uid: u64,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<Feedback>, sqlx::Error> {
    Ok(Vec::new())
}

pub async fn count_by_user(_pool: &MySqlPool, _uid: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}

/// Admin view: paginated list (status=None means all).
pub async fn list_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Feedback>, sqlx::Error> {
    let sql = match status {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_advice_question \
             WHERE status = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_advice_question \
             ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Feedback>(&sql);
    let q = match status {
        Some(s) => q.bind(s).bind(limit).bind(offset),
        None => q.bind(limit).bind(offset),
    };
    q.fetch_all(pool).await
}

pub async fn count_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match status {
        Some(s) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_advice_question WHERE status = ?")
                .bind(s)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_advice_question")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}

pub async fn set_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_advice_question SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
