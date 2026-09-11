//! Dedup map + run log for English job scrape.

use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ScrapeLog {
    #[sqlx(try_from = "i64")]
    pub id: u64,
    pub started_at: i64,
    pub finished_at: i64,
    pub fetched: i32,
    pub inserted: i32,
    pub skipped: i32,
    pub error: String,
}

pub async fn find_job_id_by_url(pool: &MySqlPool, source_url: &str) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(job_id AS UNSIGNED) FROM phpyun_rs_job_scrape_item WHERE source_url = ? LIMIT 1",
    )
    .bind(source_url)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn insert_item(
    pool: &MySqlPool,
    source_url: &str,
    job_id: u64,
    company_uid: u64,
    role: &str,
    company_name: &str,
    created_at: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO phpyun_rs_job_scrape_item \
            (source_url, job_id, company_uid, role, company_name, created_at) \
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(source_url)
    .bind(job_id)
    .bind(company_uid)
    .bind(role)
    .bind(company_name)
    .bind(created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_log(
    pool: &MySqlPool,
    started_at: i64,
    finished_at: i64,
    fetched: i32,
    inserted: i32,
    skipped: i32,
    error: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_job_scrape_log \
            (started_at, finished_at, fetched, inserted, skipped, error) \
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(started_at)
    .bind(finished_at)
    .bind(fetched)
    .bind(inserted)
    .bind(skipped)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_logs(pool: &MySqlPool, limit: i64) -> Result<Vec<ScrapeLog>, sqlx::Error> {
    sqlx::query_as::<_, ScrapeLog>(
        "SELECT CAST(id AS SIGNED) AS id, \
                CAST(started_at AS SIGNED) AS started_at, \
                CAST(finished_at AS SIGNED) AS finished_at, \
                fetched, inserted, skipped, COALESCE(error,'') AS error \
         FROM phpyun_rs_job_scrape_log \
         ORDER BY id DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}
