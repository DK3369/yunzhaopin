//! PHP `phpyun_user_entrust_record` — 顾问/系统推送给企业的简历记录。
//! Distinct from `entrust` (猎头绑定).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, QueryBuilder};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EntrustRecord {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub jobid: u64,
    pub comid: u64,
    pub ctime: i64,
    pub user_name: String,
    pub job_name: String,
}

const FIELDS: &str = "CAST(d.id AS UNSIGNED) AS id, \
    CAST(COALESCE(d.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(d.eid,0) AS UNSIGNED) AS eid, \
    CAST(COALESCE(d.jobid,0) AS UNSIGNED) AS jobid, \
    CAST(COALESCE(d.comid,0) AS UNSIGNED) AS comid, \
    CAST(COALESCE(d.ctime,0) AS SIGNED) AS ctime, \
    COALESCE(r.name,'') AS user_name, \
    COALESCE(j.name,'') AS job_name";

pub async fn list_by_com(
    pool: &MySqlPool,
    comid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<EntrustRecord>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_user_entrust_record d \
         LEFT JOIN phpyun_resume r ON r.uid = d.uid \
         LEFT JOIN phpyun_company_job j ON j.id = d.jobid \
         WHERE d.comid = ? ORDER BY d.id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, EntrustRecord>(&sql)
        .bind(comid)
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

pub async fn count_by_com(pool: &MySqlPool, comid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_user_entrust_record WHERE comid = ?",
    )
    .bind(comid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_by_com(pool: &MySqlPool, comid: u64, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb =
        QueryBuilder::new("DELETE FROM phpyun_user_entrust_record WHERE comid = ");
    qb.push_bind(comid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
