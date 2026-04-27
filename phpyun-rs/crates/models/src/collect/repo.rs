//! `phpyun_fav_job` repository -- aligned with PHPYun `job.model.php` favorites.
//!
//! PHP stores denormalized snapshots (job_name / com_name); `addFavJob` does a plain
//! INSERT with no UNIQUE KEY, so duplicate-prevention happens in the application
//! layer (PHP first calls `getFavJob` to check). We follow the same pattern via the
//! `exists` helper before insert.

use super::entity::Collect;
use sqlx::MySqlPool;

pub struct InsertJob<'a> {
    pub uid: u64,
    pub com_id: u64,
    pub com_name: &'a str,
    pub job_id: u64,
    pub job_name: &'a str,
    /// 1 = normal, 2 = lt (headhunter).
    pub r#type: i32,
    pub datetime: i64,
}

pub async fn insert(pool: &MySqlPool, v: InsertJob<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_fav_job (uid, com_id, com_name, datetime, `type`, job_id, job_name)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(v.uid)
    .bind(v.com_id)
    .bind(v.com_name)
    .bind(v.datetime)
    .bind(v.r#type)
    .bind(v.job_id)
    .bind(v.job_name)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_fav_job WHERE uid = ? AND job_id = ?")
        .bind(uid)
        .bind(job_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn exists(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
) -> Result<bool, sqlx::Error> {
    // 1 literal sidesteps any signed/unsigned int decode mismatch — we only
    // care whether any row matches, not the actual id.
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM phpyun_fav_job WHERE uid = ? AND job_id = ? LIMIT 1",
    )
    .bind(uid)
    .bind(job_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Collect>, sqlx::Error> {
    // PHP schema columns are signed int(11); CAST to UNSIGNED so sqlx can decode
    // them into u64 fields on the entity (sqlx refuses signed→unsigned widening).
    sqlx::query_as::<_, Collect>(
        r#"SELECT
              CAST(id      AS UNSIGNED) AS id,
              CAST(uid     AS UNSIGNED) AS uid,
              CAST(com_id  AS UNSIGNED) AS com_id,
              com_name,
              datetime,
              `type`,
              CAST(job_id  AS UNSIGNED) AS job_id,
              job_name
           FROM phpyun_fav_job
           WHERE uid = ?
           ORDER BY datetime DESC, id DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Pull every favorited job_id for a user — used to warm the Redis cache.
/// Bounded to 5000 to keep cold-warm cost predictable; users with > 5000
/// favorites are an edge case we'd handle by paginating the warm.
pub async fn all_job_ids_by_user(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(job_id AS UNSIGNED) FROM phpyun_fav_job
          WHERE uid = ? AND job_id IS NOT NULL
          ORDER BY id DESC LIMIT 5000",
    )
    .bind(uid)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn count_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_fav_job WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// How many users have collected a given job.
pub async fn count_collectors_of_job(
    pool: &MySqlPool,
    job_id: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_fav_job WHERE job_id = ?")
            .bind(job_id)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// "Fans" — distinct users who have favorited any job belonging to this
/// company (`com_id` == company-side uid). Aligned with PHP
/// `com.class.php::attention_me_action`, which renders a list of users
/// "interested in me" on the company center.
///
/// Returns `(uid, fav_count, last_datetime)` ordered by most recent activity.
pub async fn list_fans_by_com_uid(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<(u64, u64, i64)>, sqlx::Error> {
    let rows: Vec<(u64, i64, i64)> = sqlx::query_as(
        r#"SELECT
              CAST(uid AS UNSIGNED)            AS uid,
              CAST(COUNT(*) AS SIGNED)         AS fav_count,
              CAST(MAX(datetime) AS SIGNED)    AS last_datetime
           FROM phpyun_fav_job
           WHERE com_id = ? AND uid > 0
           GROUP BY uid
           ORDER BY last_datetime DESC, uid DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(com_uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(uid, n, ts)| (uid, n.max(0) as u64, ts))
        .collect())
}

pub async fn count_fans_by_com_uid(
    pool: &MySqlPool,
    com_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT uid) FROM phpyun_fav_job WHERE com_id = ? AND uid > 0",
    )
    .bind(com_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Batch lookup: which of `job_ids` has `uid` favorited.
/// Returns the set of favorited job ids (caller checks containment per row).
/// Empty input → empty result, no DB call.
pub async fn favorited_job_ids_for_user(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
) -> Result<std::collections::HashSet<u64>, sqlx::Error> {
    use std::collections::HashSet;
    if job_ids.is_empty() {
        return Ok(HashSet::new());
    }
    // Build "?,?,?,..." placeholders. SQL injection-safe (numeric IDs only).
    let placeholders = vec!["?"; job_ids.len()].join(",");
    let sql = format!(
        "SELECT CAST(job_id AS UNSIGNED) FROM phpyun_fav_job
          WHERE uid = ? AND job_id IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(uid);
    for id in job_ids {
        q = q.bind(*id);
    }
    let rows: Vec<(u64,)> = q.fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}
