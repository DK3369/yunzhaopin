use sqlx::MySqlPool;

pub async fn count_active_jobs(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1",
    )
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_active_companies(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_company WHERE r_status = 1")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_active_resumes(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume WHERE r_status = 1")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_jobs_since(pool: &MySqlPool, ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job
         WHERE state = 1 AND status = 0 AND r_status = 1 AND lastupdate >= ?",
    )
    .bind(ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_resumes_since(pool: &MySqlPool, ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resume
         WHERE r_status = 1 AND lastupdate >= ?",
    )
    .bind(ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
