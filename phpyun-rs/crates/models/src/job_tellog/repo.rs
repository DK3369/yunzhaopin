use super::entity::JobTelLog;
use sqlx::MySqlPool;

// `phpyun_job_tellog` declares jobid/comid/uid/source/ctime as nullable int
// and ip as nullable varchar(20); `JobTelLog` deserializes them as plain
// `u64 / i32 / String`. COALESCE so a NULL row never reaches sqlx.
const FIELDS: &str = "id, \
    COALESCE(jobid, 0) AS jobid, \
    COALESCE(comid, 0) AS comid, \
    COALESCE(uid, 0) AS uid, \
    COALESCE(source, 0) AS source, \
    COALESCE(ip, '') AS ip, \
    COALESCE(ctime, 0) AS ctime";

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
    sqlx::query_as::<_, JobTelLog>(&format!(
        "SELECT {FIELDS} \
         FROM phpyun_job_tellog WHERE comid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?"
    ))
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
