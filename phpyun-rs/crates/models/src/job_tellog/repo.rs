use super::entity::JobTelLog;
use sqlx::MySqlPool;

pub async fn insert(
    pool: &MySqlPool,
    jobid: u64,
    comid: u64,
    uid: u64,
    source: i32,
    ip: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_job_tellog (jobid, comid, uid, source, ip, ctime) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(jobid)
    .bind(comid)
    .bind(uid)
    .bind(source)
    .bind(ip)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn count_by_job(pool: &MySqlPool, jobid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_job_tellog WHERE jobid = ?")
            .bind(jobid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn list_by_com(
    pool: &MySqlPool,
    comid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<JobTelLog>, sqlx::Error> {
    sqlx::query_as::<_, JobTelLog>(
        "SELECT id, jobid, comid, uid, source, ip, ctime \
         FROM phpyun_job_tellog WHERE comid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?",
    )
    .bind(comid)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn count_by_com(pool: &MySqlPool, comid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_job_tellog WHERE comid = ?")
            .bind(comid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}
