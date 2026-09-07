//! Country data access — `phpyun_country` is the country table.
//!
//! Countries are not `phpyun_city_class` rows. Subdivisions live in
//! `phpyun_region` (parented by country code) and are loaded through
//! `region_service`, not this repo.

use super::entity::Country;
use sqlx::MySqlPool;

const PROJECTION: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    code, \
    code3, \
    CAST(numeric_code AS SIGNED) AS numeric_code, \
    name_en, \
    name_zh, \
    continent, \
    phone_code, \
    currency, \
    flag, \
    sort, \
    CAST(status AS SIGNED) AS status, \
    created_at, \
    updated_at";

// ==================== Reads ====================

/// All visible countries (`status != 2`) in `(sort ASC, id ASC)` order.
pub async fn list_active(pool: &MySqlPool) -> Result<Vec<Country>, sqlx::Error> {
    let sql = format!(
        "SELECT {PROJECTION} FROM phpyun_country \
         WHERE status != 2 \
         ORDER BY sort ASC, id ASC"
    );
    phpyun_core::db::ok_default_if_object_missing(
        sqlx::query_as::<_, Country>(&sql).fetch_all(pool).await,
    )
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Country>, sqlx::Error> {
    let sql = format!(
        "SELECT {PROJECTION} FROM phpyun_country \
         WHERE id = ? AND status != 2 LIMIT 1"
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
    let sql = format!(
        "SELECT {PROJECTION} FROM phpyun_country \
         WHERE code = ? AND status != 2 LIMIT 1"
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
    pub status: i32,
}

pub async fn create(pool: &MySqlPool, c: CountryCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_country \
         (code, code3, numeric_code, name_en, name_zh, continent, phone_code, currency, flag, sort, status, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    .bind(c.status)
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
    pub status: Option<i32>,
}

/// Partial update — only fields explicitly set on the patch are touched.
pub async fn update(
    pool: &MySqlPool,
    id: u64,
    p: CountryPatch<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut sets: Vec<&str> = Vec::new();
    if p.name_en.is_some() {
        sets.push("name_en = ?");
    }
    if p.name_zh.is_some() {
        sets.push("name_zh = ?");
    }
    if p.continent.is_some() {
        sets.push("continent = ?");
    }
    if p.phone_code.is_some() {
        sets.push("phone_code = ?");
    }
    if p.currency.is_some() {
        sets.push("currency = ?");
    }
    if p.flag.is_some() {
        sets.push("flag = ?");
    }
    if p.sort.is_some() {
        sets.push("sort = ?");
    }
    if p.status.is_some() {
        sets.push("status = ?");
    }
    if sets.is_empty() {
        return Ok(0);
    }
    sets.push("updated_at = ?");
    let sql = format!(
        "UPDATE phpyun_country SET {} WHERE id = ? AND status != 2",
        sets.join(", ")
    );
    let mut q = sqlx::query(&sql);
    if let Some(v) = p.name_en {
        q = q.bind(v);
    }
    if let Some(v) = p.name_zh {
        q = q.bind(v);
    }
    if let Some(v) = p.continent {
        q = q.bind(v);
    }
    if let Some(v) = p.phone_code {
        q = q.bind(v);
    }
    if let Some(v) = p.currency {
        q = q.bind(v);
    }
    if let Some(v) = p.flag {
        q = q.bind(v);
    }
    if let Some(v) = p.sort {
        q = q.bind(v);
    }
    if let Some(v) = p.status {
        q = q.bind(v);
    }
    q = q.bind(now).bind(id);
    Ok(q.execute(pool).await?.rows_affected())
}

/// Soft delete (`status = 2`). Cache filters by `status != 2` so deleted
/// rows disappear on the next reload.
pub async fn soft_delete(pool: &MySqlPool, id: u64, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_country SET status = 2, updated_at = ? WHERE id = ? AND status != 2",
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Mark `ids` as the front-end set (`status = 1`); every other live row
/// becomes `status = 0`. Soft-deleted rows (`status = 2`) are left alone.
pub async fn set_enabled(pool: &MySqlPool, ids: &[u64], now: i64) -> Result<u64, sqlx::Error> {
    sqlx::query("UPDATE phpyun_country SET status = 0, updated_at = ? WHERE status != 2")
        .bind(now)
        .execute(pool)
        .await?;
    if ids.is_empty() {
        return Ok(0);
    }
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "UPDATE phpyun_country SET status = 1, updated_at = ? WHERE status != 2 AND id IN ({placeholders})"
    );
    let mut q = sqlx::query(&sql).bind(now);
    for id in ids {
        q = q.bind(*id);
    }
    Ok(q.execute(pool).await?.rows_affected())
}
