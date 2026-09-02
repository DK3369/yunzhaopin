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

pub async fn find_many(
    pool: &MySqlPool,
    keys: &[&str],
) -> Result<std::collections::HashMap<String, String>, sqlx::Error> {
    let mut out = std::collections::HashMap::new();
    if keys.is_empty() {
        return Ok(out);
    }
    let placeholders = vec!["?"; keys.len()].join(",");
    let sql = format!(
        "SELECT COALESCE(name,''), COALESCE(config,'') FROM phpyun_admin_config WHERE name IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, (String, String)>(&sql);
    for k in keys {
        q = q.bind(*k);
    }
    let rows = q.fetch_all(pool).await?;
    for (name, value) in rows {
        out.insert(name, value);
    }
    Ok(out)
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

pub async fn list_reg_config(pool: &MySqlPool) -> Result<Vec<(String, String)>, sqlx::Error> {
    sqlx::query_as::<_, (String, String)>(
        "SELECT COALESCE(name,''), COALESCE(config,'') FROM phpyun_admin_reg_config",
    )
    .fetch_all(pool)
    .await
}

pub async fn upsert_reg_config(pool: &MySqlPool, name: &str, config: &str) -> Result<(), sqlx::Error> {
    let exists: Option<(i64,)> =
        sqlx::query_as::<_, (i64,)>("SELECT 1 FROM phpyun_admin_reg_config WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_optional(pool)
            .await?;
    if exists.is_some() {
        sqlx::query("UPDATE phpyun_admin_reg_config SET config = ? WHERE name = ?")
            .bind(config)
            .bind(name)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO phpyun_admin_reg_config (name, config) VALUES (?, ?)")
            .bind(name)
            .bind(config)
            .execute(pool)
            .await?;
    }
    Ok(())
}
