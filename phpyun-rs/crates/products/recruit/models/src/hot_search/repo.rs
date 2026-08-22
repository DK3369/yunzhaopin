//! Strictly aligned with PHPYun `phpyun_hot_key` (trending search keywords).
//!
//! Actual PHP columns: id / key_name / num / type / size / check / color /
//! bold / tuijian / wxtime / ...
//! Rust field -> PHP column mapping:
//!   - `scope`       <-> `type` (int enum; CAST(type AS CHAR) to string here
//!     so Rust can handle uniformly)
//!   - `keyword`     <-> `key_name`
//!   - `hits`        <-> `num`
//!   - `last_hit_at` <-> `wxtime` (in PHPYun this is "WeChat push time";
//!     repurposed here as an approximate "last hit time")

use super::entity::HotSearch;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(`type`, 0) AS CHAR) AS scope, \
    COALESCE(key_name, '') AS keyword, \
    CAST(COALESCE(num, 0) AS SIGNED) AS hits, \
    CAST(COALESCE(wxtime, 0) AS SIGNED) AS last_hit_at";

/// UPSERT -- for the same scope+keyword, increment num and refresh wxtime.
///
/// PHPYun `phpyun_hot_key` typically has a unique index only on key_name,
/// not (type, key_name); to be safe, SELECT first then UPDATE/INSERT
/// (non-atomic, but matches the original PHP behavior).
pub async fn bump(
    pool: &MySqlPool,
    scope: &str,
    keyword: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    // Parse scope string to int ("0" -> 0); fall back to 0 for non-numeric.
    let scope_int: i32 = scope.parse().unwrap_or(0);
    let updated = sqlx::query(
        "UPDATE phpyun_hot_key SET num = num + 1, wxtime = ? \
         WHERE key_name = ? AND `type` = ?",
    )
    .bind(now)
    .bind(keyword)
    .bind(scope_int)
    .execute(pool)
    .await?;
    if updated.rows_affected() == 0 {
        sqlx::query(
            "INSERT INTO phpyun_hot_key (key_name, num, `type`, wxtime) VALUES (?, 1, ?, ?)",
        )
        .bind(keyword)
        .bind(scope_int)
        .bind(now)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Top N trending search keywords (ordered by num DESC).
pub async fn top(pool: &MySqlPool, scope: &str, limit: u64) -> Result<Vec<HotSearch>, sqlx::Error> {
    let scope_int: i32 = scope.parse().unwrap_or(0);
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_hot_key \
         WHERE `type` = ? ORDER BY num DESC LIMIT ?"
    );
    sqlx::query_as::<_, HotSearch>(&sql)
        .bind(scope_int)
        .bind(limit)
        .fetch_all(pool)
        .await
}
