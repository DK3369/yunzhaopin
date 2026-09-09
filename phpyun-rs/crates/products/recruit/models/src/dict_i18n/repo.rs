//! `phpyun_dict_i18n` — translation rows for category dictionaries.
//!
//! Schema: `(kind, item_id, lang, text)`. Translations for the default
//! language (`zh-CN`) are stored in the per-kind primary tables
//! (`phpyun_industry`, `phpyun_city_class`, ...); this table only stores
//! non-default translations.

use sqlx::MySqlPool;

fn soft_delete_where(table: &str) -> &'static str {
    match table {
        "phpyun_job_class"
        | "phpyun_city_class"
        | "phpyun_partclass"
        | "phpyun_q_class"
        | "phpyun_comclass"
        | "phpyun_userclass" => " WHERE COALESCE(deleted,0)=0",
        _ => "",
    }
}

/// Load primary-language `(id, name)` rows from a single dict table. The
/// table name MUST come from a static whitelist in the caller — the loader
/// passes it through `format!`.
pub async fn list_default(
    pool: &MySqlPool,
    table: &str,
) -> Result<Vec<(i32, Option<String>)>, sqlx::Error> {
    let sql = format!("SELECT id, name FROM {table}{}", soft_delete_where(table));
    sqlx::query_as(&sql).fetch_all(pool).await
}

/// Legacy PHP `e_name` column (English display name) on job / city class tables.
/// Empty strings are omitted. Missing column → empty list so older schemas still load.
pub async fn list_ename(
    pool: &MySqlPool,
    table: &str,
) -> Result<Vec<(i32, String)>, sqlx::Error> {
    match table {
        "phpyun_job_class" | "phpyun_city_class" => {}
        _ => return Ok(Vec::new()),
    }
    let sql = format!(
        "SELECT id, COALESCE(e_name,'') FROM {table}{}",
        soft_delete_where(table)
    );
    match sqlx::query_as::<_, (i32, String)>(&sql)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => Ok(rows
            .into_iter()
            .filter_map(|(id, name)| {
                let name = name.trim().to_string();
                (!name.is_empty()).then_some((id, name))
            })
            .collect()),
        Err(e) if phpyun_core::db::is_missing_column(&e) => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

/// Load `(id, name, keyid, variable)` for `phpyun_comclass` / `phpyun_userclass`.
/// PHP groups children by parent `keyid` (`job_edu` / `user_edu` / …).
pub async fn list_class_rows(
    pool: &MySqlPool,
    table: &str,
) -> Result<Vec<(i32, Option<String>, i32, Option<String>)>, sqlx::Error> {
    match table {
        "phpyun_comclass" | "phpyun_userclass" => {}
        _ => return Ok(Vec::new()),
    }
    let sql = format!(
        "SELECT id, name, COALESCE(keyid, 0) AS keyid, variable FROM {table} WHERE COALESCE(deleted,0)=0"
    );
    sqlx::query_as(&sql).fetch_all(pool).await
}

/// Load every translation row across all kinds. Used at startup to seed the
/// in-memory cache.
pub async fn list_all(pool: &MySqlPool) -> Result<Vec<(String, i32, String, String)>, sqlx::Error> {
    sqlx::query_as("SELECT kind, item_id, lang, text FROM phpyun_dict_i18n")
        .fetch_all(pool)
        .await
}

/// Load translation rows for a single `kind` (e.g. "region"). Used by
/// services that don't need every translation in memory.
pub async fn list_by_kind(
    pool: &MySqlPool,
    kind: &str,
) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
    sqlx::query_as("SELECT item_id, lang, text FROM phpyun_dict_i18n WHERE kind = ?")
        .bind(kind)
        .fetch_all(pool)
        .await
}
