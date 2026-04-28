//! `phpyun_resumeout` repository — outbound resume sends.
//!
//! Schema (PHP truth): `id, uid, comname, jobname, recipient, email, resume, datetime`.
//! Rust entity uses `resumename` (← `recipient`) and `addtime` (← `datetime`).
//! There is no `status` column in PHP, so deletes are real DELETEs (the
//! previous soft-delete-via-`status=2` was a Rust invention that 500'd on
//! every run).

use super::entity::ResumeOut;
use sqlx::{MySqlPool, QueryBuilder};

// SELECT aliases map PHP columns onto Rust entity field names.
const SELECT_FIELDS: &str = "id, uid, resume, email, comname, jobname, \
                             recipient AS resumename, datetime AS addtime";

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    resume: u64,
    email: &str,
    comname: &str,
    jobname: &str,
    resumename: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_resumeout (uid, resume, email, comname, jobname, recipient, datetime)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(resume)
    .bind(email)
    .bind(comname)
    .bind(jobname)
    .bind(resumename.unwrap_or(""))
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ResumeOut>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_resumeout \
         WHERE uid = ? \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, ResumeOut>(&sql)
        .bind(uid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resumeout WHERE uid = ?",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Hard delete (PHP table has no `status` column for soft-delete).
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_resumeout WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

// ==================== Rate-limit helpers (PHP `phpyun_recommend` table) ====================

/// Number of outbound sends so far today (corresponds to the PHP `recommend` table with rec_type=3)
pub async fn count_today_for_uid(
    pool: &MySqlPool,
    uid: u64,
    today_begin_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_recommend \
         WHERE uid = ? AND rec_type = 3 AND addtime > ?",
    )
    .bind(uid)
    .bind(today_begin_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Timestamp of the last outbound send
pub async fn last_send_ts(pool: &MySqlPool, uid: u64) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT addtime FROM phpyun_recommend \
         WHERE uid = ? AND rec_type = 3 ORDER BY addtime DESC LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(t,)| t))
}

pub async fn insert_recommend_mark(
    pool: &MySqlPool,
    uid: u64,
    now: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        "INSERT INTO phpyun_recommend (uid, rec_type, addtime) VALUES (?, 3, ?)",
    )
    .bind(uid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
