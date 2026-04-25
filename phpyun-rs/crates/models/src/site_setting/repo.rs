//! Strictly aligned with PHPYun `phpyun_admin_config` (site-wide global settings).
//!
//! The PHP table only has two real columns: `name` (key) / `config` (value) — there is
//! no description / is_public / updated_at. Those fields in the Rust struct are filled
//! with default values via aliases.

use super::entity::SiteSetting;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "\
    COALESCE(name, '') AS key_name, \
    COALESCE(config, '') AS value, \
    '' AS description, \
    CAST(1 AS SIGNED) AS is_public, \
    CAST(0 AS SIGNED) AS updated_at";

pub async fn list_public(pool: &MySqlPool) -> Result<Vec<SiteSetting>, sqlx::Error> {
    // PHPYun has no is_public field — PHP treats this table as a publicly readable runtime config
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_admin_config ORDER BY name ASC");
    sqlx::query_as::<_, SiteSetting>(&sql).fetch_all(pool).await
}

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<SiteSetting>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_admin_config ORDER BY name ASC");
    sqlx::query_as::<_, SiteSetting>(&sql).fetch_all(pool).await
}

pub async fn find(pool: &MySqlPool, key: &str) -> Result<Option<SiteSetting>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_admin_config WHERE name = ?");
    sqlx::query_as::<_, SiteSetting>(&sql)
        .bind(key)
        .fetch_optional(pool)
        .await
}

pub async fn upsert(
    pool: &MySqlPool,
    key: &str,
    value: &str,
    _description: &str,
    _is_public: bool,
    _now: i64,
) -> Result<(), sqlx::Error> {
    // PHPYun table has no description/is_public/updated_at columns — ignored
    sqlx::query(
        "INSERT INTO phpyun_admin_config (name, config) VALUES (?, ?) \
         ON DUPLICATE KEY UPDATE config = VALUES(config)",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &MySqlPool, key: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_admin_config WHERE name = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
