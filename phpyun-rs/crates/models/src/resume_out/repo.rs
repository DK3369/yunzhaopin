use super::entity::ResumeOut;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, uid, resume, email, comname, jobname, resumename, addtime";

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
        "INSERT INTO phpyun_resumeout (uid, resume, email, comname, jobname, resumename, addtime)
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

// Soft-delete convention: status=2 means deleted. All queries always include `AND status != 2`.

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ResumeOut>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resumeout \
         WHERE uid = ? AND status != 2 \
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
        "SELECT COUNT(*) FROM phpyun_resumeout WHERE uid = ? AND status != 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Soft delete: batch UPDATE status=2.
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_resumeout SET status = 2 WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

// ==================== Rate-limit helpers (aligned with the PHP `recommend` table) ====================

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
