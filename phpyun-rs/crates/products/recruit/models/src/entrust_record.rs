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
    CAST(COALESCE(r.name,'') AS CHAR) AS user_name, \
    CAST(COALESCE(j.name,'') AS CHAR) AS job_name";

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
    let rows = sqlx::query_as::<_, EntrustRecord>(&sql)
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
        .await;
    phpyun_core::db::ok_default_if_object_missing(rows)
}

pub async fn count_by_com(pool: &MySqlPool, comid: u64) -> Result<u64, sqlx::Error> {
    let row = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_user_entrust_record WHERE comid = ?",
    )
    .bind(comid)
    .fetch_one(pool)
    .await;
    let (n,): (i64,) = phpyun_core::db::ok_default_if_object_missing(row)?;
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
    let r = qb.build().execute(pool).await.map(|x| x.rows_affected());
    phpyun_core::db::ok_default_if_object_missing(r)
}

pub async fn exists_record(
    pool: &MySqlPool,
    eid: u64,
    jobid: u64,
    comid: u64,
) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = match sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_user_entrust_record WHERE eid = ? AND jobid = ? AND comid = ?",
    )
    .bind(eid)
    .bind(jobid)
    .bind(comid)
    .fetch_one(pool)
    .await
    {
        Ok(row) => row,
        Err(e) if phpyun_core::db::is_missing_table(&e) => return Ok(false),
        Err(e) => return Err(e),
    };
    Ok(n > 0)
}

pub async fn insert_record(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    jobid: u64,
    comid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        "INSERT INTO phpyun_user_entrust_record (uid, eid, jobid, comid, ctime) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(eid)
    .bind(jobid)
    .bind(comid)
    .bind(now)
    .execute(pool)
    .await
    {
        Ok(r) => Ok(r.last_insert_id()),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(0),
        Err(e) => Err(e),
    }
}
