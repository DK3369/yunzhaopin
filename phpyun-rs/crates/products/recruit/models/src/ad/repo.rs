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
use sqlx::{MySqlPool, QueryBuilder};

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
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_ad WHERE is_open IS NULL OR is_open != 2")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn insert_click(
    pool: &MySqlPool,
    aid: u64,
    uid: u64,
    ip: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO phpyun_adclick (aid, uid, ip, addtime) VALUES (?, ?, ?, ?)")
        .bind(aid)
        .bind(uid)
        .bind(ip)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AdClassRow {
    pub id: u64,
    pub class_name: String,
    pub place: i32,
}

pub async fn list_classes(pool: &MySqlPool) -> Result<Vec<AdClassRow>, sqlx::Error> {
    sqlx::query_as::<_, AdClassRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, \
                COALESCE(class_name, '') AS class_name, \
                CAST(COALESCE(place, 0) AS SIGNED) AS place \
         FROM phpyun_ad_class ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_target(pool: &MySqlPool, id: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(pic_src, '') FROM phpyun_ad WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|(s,)| s))
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AdAdminRow {
    pub id: u64,
    pub ad_name: String,
    pub class_id: i32,
    pub class_name: String,
    pub hits: i32,
    pub ad_type: String,
    pub pic_url: String,
    pub pic_src: String,
    pub word_url: String,
    pub word_info: String,
    pub time_start: String,
    pub time_end: String,
    pub did: i32,
    pub sort: i32,
    pub is_open: i32,
    pub is_check: i32,
    pub target: i32,
    pub pic_width: String,
    pub pic_height: String,
    pub pic_content: String,
    pub remark: String,
    pub flash_url: String,
    pub flash_src: String,
    pub flash_width: String,
    pub flash_height: String,
    pub lianmeng_url: String,
}

const ADMIN_AD_FIELDS: &str = "\
    CAST(a.id AS UNSIGNED) AS id, \
    COALESCE(a.ad_name,'') AS ad_name, \
    CAST(COALESCE(a.class_id,0) AS SIGNED) AS class_id, \
    COALESCE(c.class_name,'') AS class_name, \
    CAST(COALESCE(a.hits,0) AS SIGNED) AS hits, \
    COALESCE(a.ad_type,'') AS ad_type, \
    COALESCE(a.pic_url,'') AS pic_url, \
    COALESCE(a.pic_src,'') AS pic_src, \
    COALESCE(a.word_url,'') AS word_url, \
    COALESCE(a.word_info,'') AS word_info, \
    COALESCE(a.time_start,'') AS time_start, \
    COALESCE(a.time_end,'') AS time_end, \
    CAST(COALESCE(a.did,0) AS SIGNED) AS did, \
    CAST(COALESCE(a.sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(a.is_open,0) AS SIGNED) AS is_open, \
    CAST(COALESCE(a.is_check,0) AS SIGNED) AS is_check, \
    CAST(COALESCE(a.target,0) AS SIGNED) AS target, \
    COALESCE(a.pic_width,'') AS pic_width, \
    COALESCE(a.pic_height,'') AS pic_height, \
    COALESCE(a.pic_content,'') AS pic_content, \
    COALESCE(a.remark,'') AS remark, \
    COALESCE(a.flash_url,'') AS flash_url, \
    COALESCE(a.flash_src,'') AS flash_src, \
    COALESCE(a.flash_width,'') AS flash_width, \
    COALESCE(a.flash_height,'') AS flash_height, \
    COALESCE(a.lianmeng_url,'') AS lianmeng_url";

pub struct AdAdminFilter<'a> {
    pub class_id: Option<i32>,
    pub is_check: Option<i32>,
    pub expired: bool,
    pub name: Option<&'a str>,
    pub ad_type: Option<&'a str>,
}

fn push_ad_admin_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &AdAdminFilter<'_>, now_date: &str) {
    qb.push(" FROM phpyun_ad a LEFT JOIN phpyun_ad_class c ON c.id = a.class_id \
             WHERE (a.is_open IS NULL OR a.is_open != 2)");
    if let Some(cid) = f.class_id.filter(|n| *n > 0) {
        qb.push(" AND a.class_id = ");
        qb.push_bind(cid);
    }
    if f.expired {
        qb.push(" AND a.time_end <> '' AND a.time_end <= ");
        qb.push_bind(now_date.to_string());
    } else if let Some(chk) = f.is_check {
        qb.push(" AND a.is_check = ");
        qb.push_bind(chk);
        qb.push(" AND (a.time_end = '' OR a.time_end > ");
        qb.push_bind(now_date.to_string());
        qb.push(")");
    }
    if let Some(name) = f.name.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND a.ad_name LIKE ");
        qb.push_bind(format!("%{name}%"));
    }
    if let Some(ty) = f.ad_type.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND a.ad_type = ");
        qb.push_bind(ty.to_string());
    }
}

pub async fn list_admin_php(
    pool: &MySqlPool,
    f: &AdAdminFilter<'_>,
    now_date: &str,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdAdminRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!("SELECT {ADMIN_AD_FIELDS}"));
    push_ad_admin_where(&mut qb, f, now_date);
    qb.push(" ORDER BY a.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_admin_php(
    pool: &MySqlPool,
    f: &AdAdminFilter<'_>,
    now_date: &str,
) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_ad_admin_where(&mut qb, f, now_date);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_admin(pool: &MySqlPool, id: u64) -> Result<Option<AdAdminRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {ADMIN_AD_FIELDS} FROM phpyun_ad a \
         LEFT JOIN phpyun_ad_class c ON c.id = a.class_id WHERE a.id = ? LIMIT 1"
    );
    sqlx::query_as::<_, AdAdminRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct AdPhpWrite<'a> {
    pub id: Option<u64>,
    pub ad_name: &'a str,
    pub target: i32,
    pub time_start: &'a str,
    pub time_end: &'a str,
    pub ad_type: &'a str,
    pub class_id: i32,
    pub is_check: i32,
    pub did: i32,
    pub is_open: i32,
    pub sort: i32,
    pub remark: &'a str,
    pub pic_url: &'a str,
    pub pic_src: &'a str,
    pub pic_content: &'a str,
    pub word_info: &'a str,
    pub word_url: &'a str,
    pub pic_width: &'a str,
    pub pic_height: &'a str,
    pub flash_url: &'a str,
    pub lianmeng_url: &'a str,
}

pub async fn upsert_php(pool: &MySqlPool, a: AdPhpWrite<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|n| *n > 0) {
        sqlx::query(
            "UPDATE phpyun_ad SET ad_name=?, target=?, time_start=?, time_end=?, ad_type=?, \
             class_id=?, is_check=?, did=?, is_open=?, sort=?, remark=?, \
             pic_url=IF(?='', pic_url, ?), pic_src=?, pic_content=?, word_info=?, word_url=?, \
             pic_width=?, pic_height=?, flash_url=IF(?='', flash_url, ?), lianmeng_url=? WHERE id=?",
        )
        .bind(a.ad_name)
        .bind(a.target)
        .bind(a.time_start)
        .bind(a.time_end)
        .bind(a.ad_type)
        .bind(a.class_id)
        .bind(a.is_check)
        .bind(a.did)
        .bind(a.is_open)
        .bind(a.sort)
        .bind(a.remark)
        .bind(a.pic_url)
        .bind(a.pic_url)
        .bind(a.pic_src)
        .bind(a.pic_content)
        .bind(a.word_info)
        .bind(a.word_url)
        .bind(a.pic_width)
        .bind(a.pic_height)
        .bind(a.flash_url)
        .bind(a.flash_url)
        .bind(a.lianmeng_url)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_ad (ad_name, target, time_start, time_end, ad_type, class_id, is_check, did, \
         is_open, sort, remark, pic_url, pic_src, pic_content, word_info, word_url, pic_width, pic_height, \
         flash_url, lianmeng_url) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(a.ad_name)
    .bind(a.target)
    .bind(a.time_start)
    .bind(a.time_end)
    .bind(a.ad_type)
    .bind(a.class_id)
    .bind(a.is_check)
    .bind(a.did)
    .bind(a.is_open)
    .bind(a.sort)
    .bind(a.remark)
    .bind(a.pic_url)
    .bind(a.pic_src)
    .bind(a.pic_content)
    .bind(a.word_info)
    .bind(a.word_url)
    .bind(a.pic_width)
    .bind(a.pic_height)
    .bind(a.flash_url)
    .bind(a.lianmeng_url)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_ad SET is_open = 2 WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_check(pool: &MySqlPool, id: u64, is_check: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_ad SET is_check = ? WHERE id = ?")
            .bind(is_check)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_ad SET sort = ? WHERE id = ?")
            .bind(sort)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn extend_end_days(pool: &MySqlPool, ids: &[u64], days: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() || days < 1 {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_ad SET time_end = DATE_ADD(time_end, INTERVAL ");
    qb.push_bind(days);
    qb.push(" DAY) WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AdClassAdminRow {
    pub id: u64,
    pub class_name: String,
    pub place: i32,
    pub orders: i32,
    pub r#type: i32,
    pub href: String,
    pub integral_buy: String,
    pub btype: i32,
    pub x: String,
    pub y: String,
    pub remark: String,
}

pub async fn list_classes_admin(
    pool: &MySqlPool,
    keyword: Option<&str>,
    kw_type: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdClassAdminRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(class_name,'') AS class_name, \
         CAST(COALESCE(place,0) AS SIGNED) AS place, CAST(COALESCE(orders,0) AS SIGNED) AS orders, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, COALESCE(href,'') AS href, \
         COALESCE(integral_buy,'') AS integral_buy, CAST(COALESCE(btype,0) AS SIGNED) AS btype, \
         COALESCE(x,'') AS x, COALESCE(y,'') AS y, COALESCE(remark,'') AS remark \
         FROM phpyun_ad_class WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if kw_type == 1 {
            if let Ok(id) = kw.parse::<u64>() {
                qb.push(" AND id = ");
                qb.push_bind(id);
            }
        } else {
            qb.push(" AND class_name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_classes_admin(
    pool: &MySqlPool,
    keyword: Option<&str>,
    kw_type: i32,
) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_ad_class WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if kw_type == 1 {
            if let Ok(id) = kw.parse::<u64>() {
                qb.push(" AND id = ");
                qb.push_bind(id);
            }
        } else {
            qb.push(" AND class_name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_class(pool: &MySqlPool, id: u64) -> Result<Option<AdClassAdminRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(class_name,'') AS class_name, \
         CAST(COALESCE(place,0) AS SIGNED) AS place, CAST(COALESCE(orders,0) AS SIGNED) AS orders, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, COALESCE(href,'') AS href, \
         COALESCE(integral_buy,'') AS integral_buy, CAST(COALESCE(btype,0) AS SIGNED) AS btype, \
         COALESCE(x,'') AS x, COALESCE(y,'') AS y, COALESCE(remark,'') AS remark \
         FROM phpyun_ad_class WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub struct AdClassWrite<'a> {
    pub id: Option<u64>,
    pub class_name: &'a str,
    pub orders: i32,
    pub place: i32,
    pub r#type: i32,
    pub btype: &'a str,
    pub integral_buy: &'a str,
    pub href: &'a str,
    pub x: &'a str,
    pub y: &'a str,
    pub remark: &'a str,
}

pub async fn upsert_class(pool: &MySqlPool, a: AdClassWrite<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|n| *n > 0) {
        sqlx::query(
            "UPDATE phpyun_ad_class SET class_name=?, orders=?, place=?, `type`=?, btype=?, \
             integral_buy=?, href=IF(?='', href, ?), x=?, y=?, remark=? WHERE id=?",
        )
        .bind(a.class_name)
        .bind(a.orders)
        .bind(a.place)
        .bind(a.r#type)
        .bind(a.btype)
        .bind(a.integral_buy)
        .bind(a.href)
        .bind(a.href)
        .bind(a.x)
        .bind(a.y)
        .bind(a.remark)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_ad_class (class_name, orders, place, `type`, btype, integral_buy, href, x, y, remark) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(a.class_name)
    .bind(a.orders)
    .bind(a.place)
    .bind(a.r#type)
    .bind(a.btype)
    .bind(a.integral_buy)
    .bind(a.href)
    .bind(a.x)
    .bind(a.y)
    .bind(a.remark)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn count_ads_in_class(pool: &MySqlPool, class_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_ad WHERE class_id = ? AND (is_open IS NULL OR is_open != 2)",
    )
    .bind(class_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_classes(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_ad_class WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_class_orders(pool: &MySqlPool, id: u64, orders: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_ad_class SET orders = ? WHERE id = ?")
            .bind(orders)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn clear_class_buy(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query(
            "UPDATE phpyun_ad_class SET integral_buy='', href='', `type`=2, btype='', x='', y='', remark='' WHERE id=?",
        )
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected(),
    )
}
