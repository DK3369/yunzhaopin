//! `phpyun_dict_i18n` — translation rows for category dictionaries.
//!
//! Schema: `(kind, item_id, lang, text)`. Translations for the default
//! language (`zh-CN`) are stored in the per-kind primary tables
//! (`phpyun_industry`, `phpyun_city_class`, ...); this table only stores
//! non-default translations.

use sqlx::MySqlPool;

/// Load primary-language `(id, name)` rows from a single dict table. The
/// table name MUST come from a static whitelist in the caller — the loader
/// passes it through `format!`.
pub async fn list_default(
    pool: &MySqlPool,
    table: &str,
) -> Result<Vec<(i32, Option<String>)>, sqlx::Error> {
    let sql = format!("SELECT id, name FROM {table}");
    sqlx::query_as(&sql).fetch_all(pool).await
}

/// Load every translation row across all kinds. Used at startup to seed the
/// in-memory cache.
pub async fn list_all(
    pool: &MySqlPool,
) -> Result<Vec<(String, i32, String, String)>, sqlx::Error> {
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
