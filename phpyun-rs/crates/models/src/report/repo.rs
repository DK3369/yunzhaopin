//! `phpyun_report` repository — PHPYun's actual report queue.
//!
//! Schema (PHP truth): `id, p_uid, c_uid, eid, usertype, c_usertype,
//! inputtime, username, r_name, status, r_reason, type, r_type, did,
//! result, rtime, admin, datafh`.
//!
//! Mapping (Rust entity → PHP column):
//! - `reporter_uid` → `c_uid`     (the complainer; PHP convention)
//! - `target_kind`  → `r_type`    (1=job / 2=company / 3=resume / ...)
//! - `target_id`    → `eid`
//! - `reason_code`  → `r_reason`
//! - `detail`       → `result`    (varchar 255 free-text)
//! - `status`       → `status`
//! - `created_at`   → `inputtime`

use super::entity::Report;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS reporter_uid, \
                             COALESCE(r_type, 0) AS target_kind, \
                             CAST(COALESCE(eid, 0) AS UNSIGNED) AS target_id, \
                             COALESCE(r_reason, '') AS reason_code, \
                             result AS detail, \
                             COALESCE(status, 0) AS status, \
                             COALESCE(inputtime, 0) AS created_at";

pub struct ReportCreate<'a> {
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason_code: &'a str,
    pub detail: Option<&'a str>,
}

pub async fn create(pool: &MySqlPool, c: ReportCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_report
           (c_uid, r_type, eid, r_reason, result, status, inputtime)
           VALUES (?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.reporter_uid)
    .bind(c.target_kind)
    .bind(c.target_id)
    .bind(c.reason_code)
    .bind(c.detail)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_by_reporter(
    pool: &MySqlPool,
    reporter_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Report>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_report \
         WHERE c_uid = ? ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Report>(&sql)
        .bind(reporter_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_reporter(
    pool: &MySqlPool,
    reporter_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_report WHERE c_uid = ?",
    )
    .bind(reporter_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Admin: filter by status (`None` means no filter).
pub async fn list_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Report>, sqlx::Error> {
    let sql = match status {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_report \
             WHERE status = ? ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_report \
             ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Report>(&sql);
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
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_report WHERE status = ?")
                .bind(s)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_report")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}

pub async fn set_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_report SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
