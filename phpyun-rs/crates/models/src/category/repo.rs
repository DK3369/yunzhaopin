//! Aligns with PHPYun's multiple category tables: a `kind` string on the
//! Rust side selects which real table to query.
//!
//! | Rust kind          | PHPYun real table       | parent id col |
//! |--------------------|-------------------------|---------------|
//! | `job`              | `phpyun_job_class`      | `keyid`       |
//! | `company` / `industry` / `com` | `phpyun_comclass` | `keyid` |
//! | `city`             | `phpyun_city_class`     | `keyid`       |
//! | `part`             | `phpyun_partclass`      | `keyid`       |
//! | `question` / `q` / `qa` | `phpyun_q_class`   | `pid`         |
//!
//! Unknown kinds return an empty list (no longer trying to query a
//! non-existent "unified table").

use super::entity::Category;
use sqlx::MySqlPool;

/// Resolve `kind` to (real PHPYun table, parent id column).
fn resolve(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind {
        "job" => Some(("phpyun_job_class", "keyid")),
        "company" | "industry" | "com" | "comclass" => Some(("phpyun_comclass", "keyid")),
        "city" => Some(("phpyun_city_class", "keyid")),
        "part" | "partclass" => Some(("phpyun_partclass", "keyid")),
        "question" | "qa" | "q" | "q_class" => Some(("phpyun_q_class", "pid")),
        _ => None,
    }
}

fn select_sql(table: &str, parent_col: &str, kind: &str) -> String {
    // CAST coerces PHPYun INT columns into the BIGINT UNSIGNED / SIGNED
    // types expected by the Rust entity.
    format!(
        "SELECT \
           CAST(id AS UNSIGNED) AS id, \
           CAST(COALESCE({parent_col}, 0) AS UNSIGNED) AS parent_id, \
           '{kind}' AS `kind`, \
           COALESCE(name, '') AS name, \
           COALESCE(sort, 0) AS sort, \
           CAST(1 AS SIGNED) AS status, \
           CAST(0 AS SIGNED) AS updated_at \
         FROM {table}"
    )
}

pub async fn list_all(pool: &MySqlPool, kind: &str) -> Result<Vec<Category>, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    let sql = format!(
        "{} ORDER BY {pc} ASC, sort DESC, id ASC",
        select_sql(table, pc, kind)
    );
    sqlx::query_as::<_, Category>(&sql).fetch_all(pool).await
}

pub async fn list_children(
    pool: &MySqlPool,
    kind: &str,
    parent_id: u64,
) -> Result<Vec<Category>, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    let sql = format!(
        "{} WHERE {pc} = ? ORDER BY sort DESC, id ASC",
        select_sql(table, pc, kind)
    );
    sqlx::query_as::<_, Category>(&sql)
        .bind(parent_id)
        .fetch_all(pool)
        .await
}

/// Recommended categories — counterpart of PHP `category::getHotJobClass(rec=1)`.
/// Filters by the `rec = 1` flag (only `phpyun_job_class` and `phpyun_comclass`
/// have this column; other kinds get an empty list rather than an error).
pub async fn list_recommended(
    pool: &MySqlPool,
    kind: &str,
    limit: u64,
) -> Result<Vec<Category>, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    // Only `phpyun_job_class` ships a `rec` column; other tables (comclass,
    // city_class, partclass, q_class) don't, so fall back to "top-N by sort"
    // — same UX as PHP's default "热门类别" widget.
    let has_rec = table == "phpyun_job_class";
    let sql = if has_rec {
        format!(
            "{} WHERE COALESCE(rec, 0) = 1 ORDER BY sort DESC, id ASC LIMIT ?",
            select_sql(table, pc, kind)
        )
    } else {
        format!(
            "{} ORDER BY sort DESC, id ASC LIMIT ?",
            select_sql(table, pc, kind)
        )
    };
    sqlx::query_as::<_, Category>(&sql)
        .bind(limit)
        .fetch_all(pool)
        .await
}

// ---------- admin CRUD ----------

pub async fn admin_list_by_kind(
    pool: &MySqlPool,
    kind: &str,
) -> Result<Vec<Category>, sqlx::Error> {
    // Same as public listing: PHPYun category tables have no status
    // column, so just list everything.
    list_all(pool, kind).await
}

pub struct CatCreate<'a> {
    pub parent_id: u64,
    pub kind: &'a str,
    pub name: &'a str,
    pub sort: i32,
}

pub async fn create(
    pool: &MySqlPool,
    c: CatCreate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let Some((table, pc)) = resolve(c.kind) else {
        return Err(sqlx::Error::Protocol(format!(
            "unknown category kind: {}",
            c.kind
        )));
    };
    let sql = format!("INSERT INTO {table} ({pc}, name, sort) VALUES (?, ?, ?)");
    let res = sqlx::query(&sql)
        .bind(c.parent_id)
        .bind(c.name)
        .bind(c.sort)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub struct CatUpdate<'a> {
    pub parent_id: Option<u64>,
    pub name: Option<&'a str>,
    pub sort: Option<i32>,
    /// PHPYun category tables have no status column; this field is ignored.
    pub status: Option<i32>,
}

/// PHPYun category tables have no `kind` column -- the kind must be supplied
/// from context to locate the right table. Existing callers don't pass kind
/// yet; to keep the old signature compiling, default to `job`.
pub async fn update(
    pool: &MySqlPool,
    id: u64,
    u: CatUpdate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    update_kind(pool, id, "job", u, _now).await
}

pub async fn update_kind(
    pool: &MySqlPool,
    id: u64,
    kind: &str,
    u: CatUpdate<'_>,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = u.status; // PHPYun tables have no status column
    let Some((table, pc)) = resolve(kind) else {
        return Err(sqlx::Error::Protocol(format!("unknown category kind: {kind}")));
    };
    let sql = format!(
        "UPDATE {table} SET \
            {pc}  = COALESCE(?, {pc}), \
            name  = COALESCE(?, name), \
            sort  = COALESCE(?, sort) \
         WHERE id = ?"
    );
    let res = sqlx::query(&sql)
        .bind(u.parent_id)
        .bind(u.name)
        .bind(u.sort)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Delete category (and child nodes). Falls back to `job` when kind unknown.
pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    delete_kind(pool, id, "job").await
}

pub async fn delete_kind(pool: &MySqlPool, id: u64, kind: &str) -> Result<u64, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Err(sqlx::Error::Protocol(format!("unknown category kind: {kind}")));
    };
    let sql = format!("DELETE FROM {table} WHERE id = ? OR {pc} = ?");
    let res = sqlx::query(&sql)
        .bind(id)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
