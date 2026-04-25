//! `phpyun_region` data access — global hierarchical region tree.
//!
//! Hot path: `list_all_active` loads every active row at startup into the
//! in-memory cache (`region_service`). Per-request lookups go through that
//! cache, so DB-level reads here are mostly used by:
//! - admin CRUD endpoints (mutation paths)
//! - the cache reload itself
//! - one-off code-based lookups when the cache happens to be empty

use super::entity::Region;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "id, parent_id, country_code, code, level, name, \
    continent, lat, lng, sort, status, created_at, updated_at";

// ==================== Reads ====================

/// Load every active row in `(level ASC, sort ASC, id ASC)` order.
/// Used at startup by the cache loader.
pub async fn list_all_active(pool: &MySqlPool) -> Result<Vec<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region \
         WHERE status != 2 \
         ORDER BY level ASC, sort ASC, id ASC"
    );
    sqlx::query_as::<_, Region>(&sql).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region WHERE id = ? AND status != 2 LIMIT 1"
    );
    sqlx::query_as::<_, Region>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_code(pool: &MySqlPool, code: &str) -> Result<Option<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region WHERE code = ? AND status != 2 LIMIT 1"
    );
    sqlx::query_as::<_, Region>(&sql)
        .bind(code)
        .fetch_optional(pool)
        .await
}

pub async fn list_children(
    pool: &MySqlPool,
    parent_id: u64,
) -> Result<Vec<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region \
         WHERE parent_id = ? AND status != 2 \
         ORDER BY sort ASC, id ASC"
    );
    sqlx::query_as::<_, Region>(&sql)
        .bind(parent_id)
        .fetch_all(pool)
        .await
}

/// Top-level countries (parent_id IS NULL).
pub async fn list_countries(pool: &MySqlPool) -> Result<Vec<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region \
         WHERE parent_id IS NULL AND status != 2 \
         ORDER BY name ASC"
    );
    sqlx::query_as::<_, Region>(&sql).fetch_all(pool).await
}

/// Filter helper used by `/v1/wap/regions?country=CN&level=1`.
pub async fn list_by_country_level(
    pool: &MySqlPool,
    country_code: &str,
    level: i32,
) -> Result<Vec<Region>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_region \
         WHERE country_code = ? AND level = ? AND status != 2 \
         ORDER BY sort ASC, id ASC"
    );
    sqlx::query_as::<_, Region>(&sql)
        .bind(country_code)
        .bind(level)
        .fetch_all(pool)
        .await
}

// ==================== Writes (admin only) ====================

pub struct RegionCreate<'a> {
    pub parent_id: Option<u64>,
    pub country_code: &'a str,
    pub code: &'a str,
    pub level: i32,
    pub name: &'a str,
    pub continent: Option<&'a str>,
    pub sort: i32,
}

pub async fn create(
    pool: &MySqlPool,
    r: RegionCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_region \
         (parent_id, country_code, code, level, name, continent, sort, status, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(r.parent_id)
    .bind(r.country_code)
    .bind(r.code)
    .bind(r.level)
    .bind(r.name)
    .bind(r.continent)
    .bind(r.sort)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct RegionPatch<'a> {
    pub name: Option<&'a str>,
    pub sort: Option<i32>,
    pub continent: Option<&'a str>,
}

/// Partial update. Only fields explicitly set on the patch are touched.
pub async fn update(
    pool: &MySqlPool,
    id: u64,
    p: RegionPatch<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut sets: Vec<&str> = Vec::new();
    if p.name.is_some() {
        sets.push("name = ?");
    }
    if p.sort.is_some() {
        sets.push("sort = ?");
    }
    if p.continent.is_some() {
        sets.push("continent = ?");
    }
    if sets.is_empty() {
        return Ok(0);
    }
    sets.push("updated_at = ?");
    let sql = format!(
        "UPDATE phpyun_region SET {} WHERE id = ? AND status != 2",
        sets.join(", ")
    );
    let mut q = sqlx::query(&sql);
    if let Some(v) = p.name {
        q = q.bind(v);
    }
    if let Some(v) = p.sort {
        q = q.bind(v);
    }
    if let Some(v) = p.continent {
        q = q.bind(v);
    }
    q = q.bind(now).bind(id);
    Ok(q.execute(pool).await?.rows_affected())
}

/// Soft delete (sets `status=2`). Cascades manually: any descendant rows
/// are also flipped because front-end queries filter `status != 2`.
pub async fn soft_delete(
    pool: &MySqlPool,
    id: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_region SET status = 2, updated_at = ? WHERE id = ? AND status != 2",
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
