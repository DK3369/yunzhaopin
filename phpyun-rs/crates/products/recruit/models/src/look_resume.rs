//! `phpyun_look_resume` — companies who viewed a jobseeker's resume (PHP `look`).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LookResume {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub com_id: u64,
    #[sqlx(try_from = "i32")]
    pub resume_id: u64,
    pub datetime: i64,
    pub status: i32,
    pub usertype: i32,
    pub com_name: String,
    pub com_job: String,
    pub com_job_num: i64,
}

const FIELDS: &str = "d.id, COALESCE(d.uid,0) AS uid, COALESCE(d.com_id,0) AS com_id, \
    COALESCE(d.resume_id,0) AS resume_id, CAST(COALESCE(d.datetime,0) AS SIGNED) AS datetime, \
    COALESCE(d.status,0) AS status, COALESCE(d.usertype,0) AS usertype, \
    COALESCE(c.name,'') AS com_name, \
    COALESCE((SELECT j.name FROM phpyun_company_job j \
        WHERE j.uid = d.com_id AND j.state = 1 AND j.r_status = 1 AND j.status = 0 \
        ORDER BY j.id DESC LIMIT 1),'') AS com_job, \
    COALESCE((SELECT COUNT(*) FROM phpyun_company_job j \
        WHERE j.uid = d.com_id AND j.state = 1 AND j.r_status = 1 AND j.status = 0),0) AS com_job_num";

pub async fn list_by_resume_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<LookResume>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_look_resume d \
         LEFT JOIN phpyun_company c ON c.uid = d.com_id \
         WHERE d.uid = ? AND COALESCE(d.status,0) = 0 \
         ORDER BY d.datetime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, LookResume>(&sql)
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

pub async fn count_by_resume_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_look_resume WHERE uid = ? AND COALESCE(status,0) = 0",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `lookresume.model.php::delInfo` for usertype=1: `status = 1`.
pub async fn hide_by_uid(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_look_resume SET status = 1 \
         WHERE id = ? AND uid = ? AND COALESCE(status,0) = 0",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}