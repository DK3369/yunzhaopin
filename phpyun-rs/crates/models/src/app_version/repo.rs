//! Strictly aligned with `phpyun_app_version` (a Rust-side new table that
//! follows PHPYun naming/field conventions).
//!
//! Actual PHP columns: id / platform / version / version_code / is_force /
//! download_url / changelog / status / released_at / ctime
//!
//! Rust `AppVersion` fields map 1:1 to PHP columns (ctime maps to Rust
//! `created_at`).

use super::entity::AppVersion;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(platform, '') AS platform, \
    COALESCE(version, '') AS version, \
    CAST(COALESCE(version_code, 0) AS UNSIGNED) AS version_code, \
    CAST(COALESCE(is_force, 0) AS SIGNED) AS is_force, \
    COALESCE(download_url, '') AS download_url, \
    COALESCE(changelog, '') AS changelog, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(released_at, 0) AS SIGNED) AS released_at, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

pub async fn latest_for_platform(
    pool: &MySqlPool,
    platform: &str,
) -> Result<Option<AppVersion>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_app_version \
         WHERE platform = ? AND status = 1 \
         ORDER BY version_code DESC, id DESC LIMIT 1"
    );
    sqlx::query_as::<_, AppVersion>(&sql)
        .bind(platform)
        .fetch_optional(pool)
        .await
}

pub async fn admin_list(
    pool: &MySqlPool,
    platform: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AppVersion>, sqlx::Error> {
    let sql = match platform {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_app_version \
             WHERE platform = ? ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_app_version \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, AppVersion>(&sql);
    match platform {
        Some(p) => q.bind(p).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub struct VersionCreate<'a> {
    pub platform: &'a str,
    pub version: &'a str,
    pub version_code: u32,
    pub is_force: bool,
    pub download_url: &'a str,
    pub changelog: &'a str,
    pub released_at: i64,
}

pub async fn create(
    pool: &MySqlPool,
    c: VersionCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_app_version \
         (platform, version, version_code, is_force, download_url, changelog, status, released_at, ctime) \
         VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)",
    )
    .bind(c.platform)
    .bind(c.version)
    .bind(c.version_code)
    .bind(if c.is_force { 1i32 } else { 0 })
    .bind(c.download_url)
    .bind(c.changelog)
    .bind(c.released_at)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_app_version WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
