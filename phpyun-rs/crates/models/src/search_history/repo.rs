//! `phpyun_keyword_log` repository — per-user keyword search history.
//!
//! PHP schema (truth): `id, uid, usertype, keyword, ctime`. There is no
//! `scope` (string) column — PHP keys by `usertype` (1=jobseeker /
//! 2=employer) instead. We expose Rust's string `scope` field by mapping
//! the integer usertype to a label ("user" / "company") on read; on write
//! the Rust scope string is ignored.

use super::entity::SearchHistory;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                             CASE COALESCE(usertype, 0) \
                                 WHEN 1 THEN 'user' \
                                 WHEN 2 THEN 'company' \
                                 ELSE '' \
                             END AS scope, \
                             COALESCE(keyword, '') AS keyword, \
                             COALESCE(ctime, 0) AS created_at";

pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    scope: &str,
    keyword: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // Map Rust scope string to PHP `usertype` int.
    let usertype: i32 = match scope {
        "user" | "jobseeker" | "1" => 1,
        "company" | "employer" | "2" => 2,
        _ => 0,
    };
    let res = sqlx::query(
        r#"INSERT INTO phpyun_keyword_log (uid, usertype, keyword, ctime)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(usertype)
    .bind(keyword)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list(
    pool: &MySqlPool,
    uid: u64,
    scope: Option<&str>,
    limit: u64,
) -> Result<Vec<SearchHistory>, sqlx::Error> {
    let usertype = scope.and_then(|s| match s {
        "user" | "jobseeker" | "1" => Some(1i32),
        "company" | "employer" | "2" => Some(2i32),
        _ => None,
    });
    let sql = match usertype {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_keyword_log \
             WHERE uid = ? AND usertype = ? \
             ORDER BY ctime DESC, id DESC LIMIT ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_keyword_log \
             WHERE uid = ? \
             ORDER BY ctime DESC, id DESC LIMIT ?"
        ),
    };
    let q = sqlx::query_as::<_, SearchHistory>(&sql);
    match usertype {
        Some(u) => q.bind(uid).bind(u).bind(limit).fetch_all(pool).await,
        None => q.bind(uid).bind(limit).fetch_all(pool).await,
    }
}

pub async fn delete_one(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_keyword_log WHERE id = ? AND uid = ?",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn clear(
    pool: &MySqlPool,
    uid: u64,
    scope: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let usertype = scope.and_then(|s| match s {
        "user" | "jobseeker" | "1" => Some(1i32),
        "company" | "employer" | "2" => Some(2i32),
        _ => None,
    });
    let res = match usertype {
        Some(u) => {
            sqlx::query("DELETE FROM phpyun_keyword_log WHERE uid = ? AND usertype = ?")
                .bind(uid)
                .bind(u)
                .execute(pool)
                .await?
        }
        None => {
            sqlx::query("DELETE FROM phpyun_keyword_log WHERE uid = ?")
                .bind(uid)
                .execute(pool)
                .await?
        }
    };
    Ok(res.rows_affected())
}

/// When there are more than `keep` entries, delete the older ones until only `keep` remain.
pub async fn trim(
    pool: &MySqlPool,
    uid: u64,
    scope: &str,
    keep: u64,
) -> Result<u64, sqlx::Error> {
    let usertype: i32 = match scope {
        "user" | "jobseeker" | "1" => 1,
        "company" | "employer" | "2" => 2,
        _ => 0,
    };
    // MySQL does not support DELETE with LIMIT+OFFSET directly; use a subquery instead.
    let res = sqlx::query(
        r#"DELETE FROM phpyun_keyword_log
           WHERE uid = ? AND usertype = ? AND id NOT IN (
               SELECT * FROM (
                   SELECT id FROM phpyun_keyword_log
                   WHERE uid = ? AND usertype = ?
                   ORDER BY ctime DESC LIMIT ?
               ) AS keep_ids
           )"#,
    )
    .bind(uid)
    .bind(usertype)
    .bind(uid)
    .bind(usertype)
    .bind(keep)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
