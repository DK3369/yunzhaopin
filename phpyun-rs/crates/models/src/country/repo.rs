//! Country data access — sourced from `phpyun_city_class` (the canonical
//! PHPYun region tree; countries are the `keyid = 0` rows, ids 4001..=4250).
//! ISO alpha-2 code + continent are pulled from `phpyun_region` via the
//! offset relationship `phpyun_region.id = phpyun_city_class.id - 4000`
//! (verified to match exactly across all 250 countries).
//!
//! Field mapping:
//!   - `id`            ← cc.id (the PHPYun city_class id; matches what every
//!                       other PHPYun endpoint expects)
//!   - `code` (ISO α2) ← r.country_code  (LEFT JOIN; '' if region missing)
//!   - `name_en`       ← cc.e_name
//!   - `name_zh`       ← cc.name
//!   - `continent`     ← r.continent
//!   - `phone_code`    ← cc.code (stored as INT, cast to CHAR)
//!   - `sort`          ← cc.sort
//!   - `status`        ← cc.display (1 = visible)
//!
//! Source rows lack `code3`, `numeric_code`, `currency`, `flag`, `created_at`,
//! `updated_at` — synthesised as empty / 0.
//!
//! Reads go through `country_service`'s in-process cache.

use super::entity::Country;
use sqlx::MySqlPool;

const PROJECTION: &str = "\
    CAST(cc.id AS UNSIGNED) AS id, \
    COALESCE(r.country_code, '') AS code, \
    '' AS code3, \
    0 AS numeric_code, \
    cc.e_name AS name_en, \
    cc.name AS name_zh, \
    COALESCE(r.continent, '') AS continent, \
    CAST(cc.code AS CHAR) AS phone_code, \
    '' AS currency, \
    '' AS flag, \
    cc.sort, \
    CAST(cc.display AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS created_at, \
    CAST(0 AS SIGNED) AS updated_at";

const FROM_JOIN: &str = "\
    FROM phpyun_city_class cc \
    LEFT JOIN phpyun_region r ON r.id = cc.id - 4000";

// ==================== Reads ====================

/// All visible countries (city_class.keyid = 0 AND display = 1) in
/// `(sort ASC, id ASC)` order. Loaded by the cache.
pub async fn list_active(pool: &MySqlPool) -> Result<Vec<Country>, sqlx::Error> {
    let sql = format!(
        "SELECT {PROJECTION} {FROM_JOIN} \
         WHERE cc.keyid = 0 AND cc.display = 1 \
         ORDER BY cc.sort ASC, cc.id ASC"
    );
    phpyun_core::db::ok_default_if_object_missing(
        sqlx::query_as::<_, Country>(&sql).fetch_all(pool).await,
    )
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Country>, sqlx::Error> {
    let sql = format!(
        "SELECT {PROJECTION} {FROM_JOIN} \
         WHERE cc.id = ? AND cc.keyid = 0 AND cc.display = 1 LIMIT 1"
    );
    let r = sqlx::query_as::<_, Country>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn find_by_code(pool: &MySqlPool, code: &str) -> Result<Option<Country>, sqlx::Error> {
    // ISO α2 lookup goes through the LEFT-joined region row; if region is
    // missing we can't resolve by code, return None.
    let sql = format!(
        "SELECT {PROJECTION} {FROM_JOIN} \
         WHERE r.country_code = ? AND cc.keyid = 0 AND cc.display = 1 LIMIT 1"
    );
    let r = sqlx::query_as::<_, Country>(&sql)
        .bind(code)
        .fetch_optional(pool)
        .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

// ==================== Writes (admin only) ====================

pub struct CountryCreate<'a> {
    pub code: &'a str,
    pub code3: &'a str,
    pub numeric_code: u16,
    pub name_en: &'a str,
    pub name_zh: &'a str,
    pub continent: &'a str,
    pub phone_code: &'a str,
    pub currency: &'a str,
    pub flag: &'a str,
    pub sort: i32,
}

pub async fn create(
    pool: &MySqlPool,
    c: CountryCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_country \
         (code, code3, numeric_code, name_en, name_zh, continent, phone_code, currency, flag, sort, status, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(c.code)
    .bind(c.code3)
    .bind(c.numeric_code)
    .bind(c.name_en)
    .bind(c.name_zh)
    .bind(c.continent)
    .bind(c.phone_code)
    .bind(c.currency)
    .bind(c.flag)
    .bind(c.sort)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

#[derive(Default)]
pub struct CountryPatch<'a> {
    pub name_en: Option<&'a str>,
    pub name_zh: Option<&'a str>,
    pub continent: Option<&'a str>,
    pub phone_code: Option<&'a str>,
    pub currency: Option<&'a str>,
    pub flag: Option<&'a str>,
    pub sort: Option<i32>,
}

/// Partial update — only fields explicitly set on the patch are touched.
pub async fn update(
    pool: &MySqlPool,
    id: u64,
    p: CountryPatch<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut sets: Vec<&str> = Vec::new();
    if p.name_en.is_some()    { sets.push("name_en = ?"); }
    if p.name_zh.is_some()    { sets.push("name_zh = ?"); }
    if p.continent.is_some()  { sets.push("continent = ?"); }
    if p.phone_code.is_some() { sets.push("phone_code = ?"); }
    if p.currency.is_some()   { sets.push("currency = ?"); }
    if p.flag.is_some()       { sets.push("flag = ?"); }
    if p.sort.is_some()       { sets.push("sort = ?"); }
    if sets.is_empty() {
        return Ok(0);
    }
    sets.push("updated_at = ?");
    let sql = format!(
        "UPDATE phpyun_country SET {} WHERE id = ? AND status != 2",
        sets.join(", ")
    );
    let mut q = sqlx::query(&sql);
    if let Some(v) = p.name_en    { q = q.bind(v); }
    if let Some(v) = p.name_zh    { q = q.bind(v); }
    if let Some(v) = p.continent  { q = q.bind(v); }
    if let Some(v) = p.phone_code { q = q.bind(v); }
    if let Some(v) = p.currency   { q = q.bind(v); }
    if let Some(v) = p.flag       { q = q.bind(v); }
    if let Some(v) = p.sort       { q = q.bind(v); }
    q = q.bind(now).bind(id);
    Ok(q.execute(pool).await?.rows_affected())
}

/// Soft delete (`status = 2`). Cache filters by `status != 2` so deleted
/// rows disappear on the next reload.
pub async fn soft_delete(
    pool: &MySqlPool,
    id: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_country SET status = 2, updated_at = ? WHERE id = ? AND status != 2",
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
