//! `phpyun_seo` — PHP `seo.model` / admin `set_seo`.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SeoRow {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    pub seoname: String,
    pub seomodel: String,
    pub ident: String,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub time: i64,
    pub did: i32,
    pub php_url: String,
    pub rewrite_url: String,
    pub php_wap_url: String,
    pub rewrite_wap_url: String,
}

const FIELDS: &str = "\
    id, COALESCE(seoname,'') AS seoname, COALESCE(seomodel,'') AS seomodel, \
    COALESCE(ident,'') AS ident, COALESCE(title,'') AS title, \
    COALESCE(keywords,'') AS keywords, COALESCE(description,'') AS description, \
    COALESCE(time,0) AS time, COALESCE(did,0) AS did, \
    COALESCE(php_url,'') AS php_url, COALESCE(rewrite_url,'') AS rewrite_url, \
    COALESCE(php_wap_url,'') AS php_wap_url, COALESCE(rewrite_wap_url,'') AS rewrite_wap_url";

pub async fn list_by_model(pool: &MySqlPool, seomodel: &str) -> Result<Vec<SeoRow>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_seo WHERE seomodel = ? ORDER BY id ASC");
    sqlx::query_as::<_, SeoRow>(&sql)
        .bind(seomodel)
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<SeoRow>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_seo WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, SeoRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert(
    pool: &MySqlPool,
    id: u64,
    seoname: &str,
    ident: &str,
    seomodel: &str,
    title: &str,
    keywords: &str,
    php_url: &str,
    rewrite_url: &str,
    php_wap_url: &str,
    rewrite_wap_url: &str,
    description: &str,
    did: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if id > 0 {
        sqlx::query(
            "UPDATE phpyun_seo SET seoname=?, ident=?, seomodel=?, title=?, keywords=?, \
             php_url=?, rewrite_url=?, php_wap_url=?, rewrite_wap_url=?, description=?, did=?, time=? \
             WHERE id=?",
        )
        .bind(seoname)
        .bind(ident)
        .bind(seomodel)
        .bind(title)
        .bind(keywords)
        .bind(php_url)
        .bind(rewrite_url)
        .bind(php_wap_url)
        .bind(rewrite_wap_url)
        .bind(description)
        .bind(did)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_seo (seoname, ident, seomodel, title, keywords, php_url, rewrite_url, \
         php_wap_url, rewrite_wap_url, description, did, time) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(seoname)
    .bind(ident)
    .bind(seomodel)
    .bind(title)
    .bind(keywords)
    .bind(php_url)
    .bind(rewrite_url)
    .bind(php_wap_url)
    .bind(rewrite_wap_url)
    .bind(description)
    .bind(did)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_seo WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
