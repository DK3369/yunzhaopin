//! `phpyun_rs_third_data` — crawler source URLs. SQL stays here.

use super::entity::ThirdData;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(url, '') AS url, \
    COALESCE(api_url, '') AS api_url, \
    COALESCE(provider, '') AS provider, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(enabled, 0) AS SIGNED) AS enabled, \
    CAST(COALESCE(created_at, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(updated_at, 0) AS SIGNED) AS updated_at";

pub async fn list(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<ThirdData>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_rs_third_data \
         ORDER BY sort ASC, id ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, ThirdData>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_all(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_rs_third_data")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Enabled rows in crawl order (`sort` then `id`).
pub async fn list_enabled_ordered(pool: &MySqlPool) -> Result<Vec<ThirdData>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_rs_third_data \
         WHERE enabled = 1 ORDER BY sort ASC, id ASC"
    );
    sqlx::query_as::<_, ThirdData>(&sql).fetch_all(pool).await
}

pub async fn find_by_url(pool: &MySqlPool, url: &str) -> Result<Option<ThirdData>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_rs_third_data WHERE url = ? LIMIT 1");
    sqlx::query_as::<_, ThirdData>(&sql)
        .bind(url)
        .fetch_optional(pool)
        .await
}

pub async fn max_sort(pool: &MySqlPool) -> Result<i32, sqlx::Error> {
    let (n,): (Option<i32>,) = sqlx::query_as("SELECT MAX(sort) FROM phpyun_rs_third_data")
        .fetch_one(pool)
        .await?;
    Ok(n.unwrap_or(0))
}

pub struct ThirdDataUpsert<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub url: &'a str,
    pub api_url: &'a str,
    pub provider: &'a str,
    pub sort: i32,
    pub enabled: i32,
    pub now: i64,
}

pub async fn upsert(pool: &MySqlPool, a: ThirdDataUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            "UPDATE phpyun_rs_third_data \
             SET name = ?, url = ?, api_url = ?, provider = ?, sort = ?, enabled = ?, updated_at = ? \
             WHERE id = ?",
        )
        .bind(a.name)
        .bind(a.url)
        .bind(a.api_url)
        .bind(a.provider)
        .bind(a.sort)
        .bind(a.enabled)
        .bind(a.now)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_third_data \
         (name, url, api_url, provider, sort, enabled, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(a.name)
    .bind(a.url)
    .bind(a.api_url)
    .bind(a.provider)
    .bind(a.sort)
    .bind(a.enabled)
    .bind(a.now)
    .bind(a.now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_rs_third_data WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
