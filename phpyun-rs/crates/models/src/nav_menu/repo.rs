//! Strictly aligned with PHPYun `phpyun_navigation` (navigation menu).
//!
//! PHP columns: id/nid/name/url/sort/display/eject/type/furl/color/...
//! Rust field -> PHP column mapping:
//!   - position   <-> `type` (PHP stores int; Rust API conventionally
//!     passes "top"/"bottom"/"1"/"2" strings, each side CASTs)
//!   - label      <-> name
//!   - icon       <-> pic
//!   - parent_id  <-> nid
//!   - status     <-> display
//!   - updated_at = 0 (not maintained in PHP)

use super::entity::NavMenu;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(`type`, 0) AS CHAR) AS position, \
    COALESCE(name, '') AS label, \
    COALESCE(url, '') AS url, \
    COALESCE(pic, '') AS icon, \
    CAST(COALESCE(nid, 0) AS UNSIGNED) AS parent_id, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(display, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS updated_at";

/// position is a string in the URL ("1"/"2"/"top"); the PHP column is int.
/// Falls back to 0 when parsing fails.
fn position_to_int(p: &str) -> i32 {
    p.parse::<i32>().unwrap_or(0)
}

pub async fn list_public(
    pool: &MySqlPool,
    position: &str,
) -> Result<Vec<NavMenu>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_navigation \
         WHERE `type` = ? AND display = 1 \
         ORDER BY nid ASC, sort DESC, id ASC"
    );
    sqlx::query_as::<_, NavMenu>(&sql)
        .bind(position_to_int(position))
        .fetch_all(pool)
        .await
}

pub async fn admin_list(
    pool: &MySqlPool,
    position: Option<&str>,
) -> Result<Vec<NavMenu>, sqlx::Error> {
    let sql = match position {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_navigation \
             WHERE `type` = ? ORDER BY nid ASC, sort DESC, id ASC"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_navigation \
             ORDER BY `type` ASC, nid ASC, sort DESC, id ASC"
        ),
    };
    let q = sqlx::query_as::<_, NavMenu>(&sql);
    match position {
        Some(p) => q.bind(position_to_int(p)).fetch_all(pool).await,
        None => q.fetch_all(pool).await,
    }
}

pub struct NavCreate<'a> {
    pub position: &'a str,
    pub label: &'a str,
    pub url: &'a str,
    pub icon: &'a str,
    pub parent_id: u64,
    pub sort: i32,
}

pub async fn create(
    pool: &MySqlPool,
    c: NavCreate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_navigation (`type`, name, url, pic, nid, sort, display) \
         VALUES (?, ?, ?, ?, ?, ?, 1)",
    )
    .bind(position_to_int(c.position))
    .bind(c.label)
    .bind(c.url)
    .bind(c.icon)
    .bind(c.parent_id)
    .bind(c.sort)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct NavUpdate<'a> {
    pub label: Option<&'a str>,
    pub url: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub parent_id: Option<u64>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    u: NavUpdate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_navigation SET \
            name     = COALESCE(?, name), \
            url      = COALESCE(?, url), \
            pic      = COALESCE(?, pic), \
            nid      = COALESCE(?, nid), \
            sort     = COALESCE(?, sort), \
            display  = COALESCE(?, display) \
         WHERE id = ?",
    )
    .bind(u.label)
    .bind(u.url)
    .bind(u.icon)
    .bind(u.parent_id)
    .bind(u.sort)
    .bind(u.status)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: `status=2` means deleted; no physical DELETE.
pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_navigation SET status = 2 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
