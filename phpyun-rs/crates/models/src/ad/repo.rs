//! Strictly aligned with PHPYun `phpyun_ad` (advertisements).
//!
//! PHP columns: id/ad_name/did/time_start/time_end/pic_src/pic_url/word_url/class_id/is_check/is_open/hits/sort/target/pic_width/pic_height/pic_content/...
//!
//! A PHPYun row carries two click payloads (toggled by `ad_type`):
//!   - Image ad: `pic_url` = **image file path**, `pic_src` = **click target URL**
//!   - Text ad:  `word_url` = click target URL (pic_url/pic_src empty in this case)
//!
//! Rust Ad field -> PHP column mapping:
//!   - slot        <-> CAST(class_id AS CHAR)
//!   - title       <-> ad_name
//!   - image       <-> pic_url (image file path)
//!   - link        <-> COALESCE(NULLIF(pic_src,''), word_url) (click URL; pic preferred)
//!   - weight      <-> sort
//!   - start_at    <-> UNIX_TIMESTAMP(time_start)
//!   - end_at      <-> UNIX_TIMESTAMP(time_end)
//!   - status      <-> is_open
//!   - created_at  = 0 (no such column in PHP)

use super::entity::Ad;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(class_id, 0) AS CHAR) AS slot, \
    COALESCE(ad_name, '') AS title, \
    COALESCE(pic_url, '') AS image, \
    COALESCE(NULLIF(pic_src, ''), word_url, '') AS link, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS weight, \
    CAST(COALESCE(UNIX_TIMESTAMP(time_start), 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(UNIX_TIMESTAMP(time_end), 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(is_open, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS created_at, \
    CAST(COALESCE(target, 0) AS SIGNED) AS target, \
    COALESCE(pic_width, '') AS pic_width, \
    COALESCE(pic_height, '') AS pic_height, \
    COALESCE(pic_content, '') AS pic_content";

pub async fn list_active(
    pool: &MySqlPool,
    slot: &str,
    now: i64,
    limit: u64,
) -> Result<Vec<Ad>, sqlx::Error> {
    let slot_int: i32 = slot.parse().unwrap_or(0);
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_ad \
         WHERE class_id = ? AND is_open = 1 \
           AND (time_start IS NULL OR time_start = '' OR UNIX_TIMESTAMP(time_start) <= ?) \
           AND (time_end IS NULL OR time_end = '' OR UNIX_TIMESTAMP(time_end) >= ?) \
         ORDER BY sort DESC, id DESC \
         LIMIT ?"
    );
    sqlx::query_as::<_, Ad>(&sql)
        .bind(slot_int)
        .bind(now)
        .bind(now)
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn list_all(
    pool: &MySqlPool,
    slot: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Ad>, sqlx::Error> {
    // Soft delete: is_open=2 means deleted; filter out from listings.
    match slot {
        Some(s) => {
            let slot_int: i32 = s.parse().unwrap_or(0);
            let sql = format!(
                "SELECT {FIELDS} FROM phpyun_ad \
                 WHERE class_id = ? AND (is_open IS NULL OR is_open != 2) \
                 ORDER BY id DESC LIMIT ? OFFSET ?"
            );
            sqlx::query_as::<_, Ad>(&sql)
                .bind(slot_int)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await
        }
        None => {
            let sql = format!(
                "SELECT {FIELDS} FROM phpyun_ad \
                 WHERE is_open IS NULL OR is_open != 2 \
                 ORDER BY id DESC LIMIT ? OFFSET ?"
            );
            sqlx::query_as::<_, Ad>(&sql)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await
        }
    }
}

pub async fn count_all(pool: &MySqlPool, slot: Option<&str>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match slot {
        Some(s) => {
            let slot_int: i32 = s.parse().unwrap_or(0);
            sqlx::query_as(
                "SELECT COUNT(*) FROM phpyun_ad \
                 WHERE class_id = ? AND (is_open IS NULL OR is_open != 2)",
            )
            .bind(slot_int)
            .fetch_one(pool)
            .await?
        }
        None => {
            sqlx::query_as(
                "SELECT COUNT(*) FROM phpyun_ad WHERE is_open IS NULL OR is_open != 2",
            )
            .fetch_one(pool)
            .await?
        }
    };
    Ok(n.max(0) as u64)
}

pub struct AdCreate<'a> {
    pub slot: &'a str,
    pub title: &'a str,
    pub image: &'a str,
    pub link: &'a str,
    pub weight: i32,
    pub start_at: i64,
    pub end_at: i64,
}

pub async fn create(pool: &MySqlPool, c: AdCreate<'_>, _now: i64) -> Result<u64, sqlx::Error> {
    let slot_int: i32 = c.slot.parse().unwrap_or(0);
    // image -> pic_url (image path); link -> pic_src (image-ad click URL).
    // Text ads should go through the dedicated word endpoint -- the current
    // admin API only creates image ad slots.
    let res = sqlx::query(
        "INSERT INTO phpyun_ad \
         (ad_name, pic_url, pic_src, sort, class_id, time_start, time_end, is_open, ad_type) \
         VALUES (?, ?, ?, ?, ?, FROM_UNIXTIME(?, '%Y-%m-%d'), FROM_UNIXTIME(?, '%Y-%m-%d'), 1, 'pic')",
    )
    .bind(c.title)
    .bind(c.image)
    .bind(c.link)
    .bind(c.weight)
    .bind(slot_int)
    .bind(c.start_at)
    .bind(c.end_at)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct AdUpdate<'a> {
    pub slot: Option<&'a str>,
    pub title: Option<&'a str>,
    pub image: Option<&'a str>,
    pub link: Option<&'a str>,
    pub weight: Option<i32>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    pub status: Option<i32>,
}

pub async fn update(pool: &MySqlPool, id: u64, u: AdUpdate<'_>) -> Result<u64, sqlx::Error> {
    let slot_int: Option<i32> = u.slot.map(|s| s.parse().unwrap_or(0));
    let res = sqlx::query(
        "UPDATE phpyun_ad SET \
            ad_name    = COALESCE(?, ad_name), \
            pic_url    = COALESCE(?, pic_url), \
            pic_src    = COALESCE(?, pic_src), \
            sort       = COALESCE(?, sort), \
            class_id   = COALESCE(?, class_id), \
            time_start = COALESCE(FROM_UNIXTIME(?, '%Y-%m-%d'), time_start), \
            time_end   = COALESCE(FROM_UNIXTIME(?, '%Y-%m-%d'), time_end), \
            is_open    = COALESCE(?, is_open) \
         WHERE id = ?",
    )
    .bind(u.title)
    .bind(u.image)
    .bind(u.link)
    .bind(u.weight)
    .bind(slot_int)
    .bind(u.start_at)
    .bind(u.end_at)
    .bind(u.status)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: the actual column is `is_open` (exposed as `status` via SELECT alias).
/// `is_open=2` means deleted; no physical DELETE is performed.
pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_ad SET is_open = 2 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ==================== Click tracking (phpyun_adclick) ====================
//
// Counterpart of PHP `index/index::clickhits_action` — records the click in
// `phpyun_adclick` and (in PHP) redirects to the ad's target URL. PHP also
// rate-limits per-IP via `sy_adclick` (hours window) so the same IP can't
// inflate the click count.

pub async fn count_clicks_recent(
    pool: &MySqlPool,
    aid: u64,
    ip: &str,
    since: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_adclick \
         WHERE aid = ? AND ip = ? AND addtime > ?",
    )
    .bind(aid)
    .bind(ip)
    .bind(since)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn insert_click(
    pool: &MySqlPool,
    aid: u64,
    uid: u64,
    ip: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_adclick (aid, uid, ip, addtime) VALUES (?, ?, ?, ?)",
    )
    .bind(aid)
    .bind(uid)
    .bind(ip)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_target(pool: &MySqlPool, id: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(pic_src, '') FROM phpyun_ad WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(s,)| s))
}
