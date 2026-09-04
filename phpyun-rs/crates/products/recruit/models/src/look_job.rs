//! `phpyun_look_job` — job seekers who viewed a company's jobs (PHP `look_job`).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LookJob {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub jobid: u64,
    #[sqlx(try_from = "i32")]
    pub com_id: u64,
    pub datetime: i64,
    pub status: i32,
    pub com_status: i32,
    #[sqlx(try_from = "i32")]
    pub did: u32,
    pub ip: Option<String>,
    pub job_name: String,
    #[sqlx(default)]
    pub com_name: String,
    #[sqlx(default)]
    pub minsalary: i32,
    #[sqlx(default)]
    pub maxsalary: i32,
}

const FIELDS: &str = "d.id, COALESCE(d.uid,0) AS uid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(d.com_id,0) AS com_id, CAST(COALESCE(d.datetime,0) AS SIGNED) AS datetime, \
    COALESCE(d.status,0) AS status, COALESCE(d.com_status,0) AS com_status, \
    COALESCE(d.did,0) AS did, d.ip, COALESCE(j.name,'') AS job_name, \
    COALESCE(j.com_name,'') AS com_name, COALESCE(j.minsalary,0) AS minsalary, \
    COALESCE(j.maxsalary,0) AS maxsalary";

pub async fn list_by_com(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<LookJob>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_look_job d \
         LEFT JOIN phpyun_company_job j ON j.id = d.jobid \
         WHERE d.com_id = ? ORDER BY d.datetime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, LookJob>(&sql)
        .bind(com_uid)
        .bind(phpyun_core::numeric::checked_db_i64(
            limit,
            "pagination.limit",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            offset,
            "pagination.offset",
        )?)
        .fetch_all(pool)
        .await
}

pub async fn count_by_com(pool: &MySqlPool, com_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_look_job WHERE com_id = ?")
            .bind(com_uid)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_by_seeker(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<LookJob>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_look_job d \
         LEFT JOIN phpyun_company_job j ON j.id = d.jobid \
         WHERE d.uid = ? AND COALESCE(d.status,0) = 0 \
         ORDER BY d.datetime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, LookJob>(&sql)
        .bind(uid)
        .bind(phpyun_core::numeric::checked_db_i64(
            limit,
            "pagination.limit",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            offset,
            "pagination.offset",
        )?)
        .fetch_all(pool)
        .await
}

pub async fn count_by_seeker(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_look_job WHERE uid = ? AND COALESCE(status,0) = 0",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `delLookJob` for usertype=1: `status = 1`.
pub async fn hide_by_seeker(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_look_job SET status = 1 \
         WHERE id = ? AND uid = ? AND COALESCE(status,0) = 0",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
