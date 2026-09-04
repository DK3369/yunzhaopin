//! Aligns with PHPYun's multiple category tables: a `kind` string on the
//! Rust side selects which real table to query.
//!
//! | Rust kind          | PHPYun real table       | parent id col |
//! |--------------------|-------------------------|---------------|
//! | `job`              | `phpyun_job_class`      | `keyid`       |
//! | `company` / `com` / `comclass` | `phpyun_comclass` | `keyid` |
//! | `industry`         | `phpyun_industry`       | (flat)        |
//! | `city`             | `phpyun_city_class`     | `keyid`       |
//! | `part` / `partclass` | `phpyun_partclass`    | `keyid`       |
//! | `question` / `q` / `qa` | `phpyun_q_class`   | `pid`         |
//! | `introduce`        | `phpyun_introduce_class`| (flat)        |
//!
//! Unknown kinds return an empty list (no longer trying to query a
//! non-existent "unified table").

use super::entity::Category;
use crate::soft_delete;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool, QueryBuilder};

/// Resolve `kind` to (real PHPYun table, parent id column).
fn resolve(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind {
        "job" => Some(("phpyun_job_class", "keyid")),
        "company" | "com" | "comclass" => Some(("phpyun_comclass", "keyid")),
        "industry" => Some(("phpyun_industry", "id")),
        "city" => Some(("phpyun_city_class", "keyid")),
        "part" | "partclass" => Some(("phpyun_partclass", "keyid")),
        "question" | "qa" | "q" | "q_class" => Some(("phpyun_q_class", "pid")),
        "userclass" | "user" => Some(("phpyun_userclass", "keyid")),
        "reason" => Some(("phpyun_reason", "id")),
        "introduce" | "introduce_class" => Some(("phpyun_introduce_class", "id")),
        "schoolclass" | "school" => Some(("phpyun_schoolclass", "keyid")),
        "px_subject" | "subject" => Some(("phpyun_px_subject_class", "keyid")),
        _ => None,
    }
}

fn is_flat(kind: &str) -> bool {
    matches!(
        kind,
        "industry" | "reason" | "introduce" | "introduce_class"
    )
}

fn uses_deleted(kind: &str) -> bool {
    matches!(
        kind,
        "job"
            | "company"
            | "com"
            | "comclass"
            | "city"
            | "part"
            | "partclass"
            | "question"
            | "qa"
            | "q"
            | "q_class"
            | "userclass"
            | "user"
            | "reason"
    )
}

fn deleted_pred(kind: &str) -> &'static str {
    if uses_deleted(kind) {
        "COALESCE(deleted,0)=0"
    } else {
        "1=1"
    }
}

fn select_sql(table: &str, parent_col: &str, kind: &str) -> String {
    let pred = deleted_pred(kind);
    if is_flat(kind) {
        let sort_expr = if kind == "reason" {
            "CAST(0 AS SIGNED)"
        } else {
            "COALESCE(sort, 0)"
        };
        return format!(
            "SELECT \
               CAST(id AS UNSIGNED) AS id, \
               CAST(0 AS UNSIGNED) AS parent_id, \
               '{kind}' AS `kind`, \
               COALESCE(name, '') AS name, \
               {sort_expr} AS sort, \
               CAST(1 AS SIGNED) AS status, \
               CAST(0 AS SIGNED) AS updated_at \
             FROM {table} WHERE {pred}"
        );
    }
    format!(
        "SELECT \
           CAST(id AS UNSIGNED) AS id, \
           CAST(COALESCE({parent_col}, 0) AS UNSIGNED) AS parent_id, \
           '{kind}' AS `kind`, \
           COALESCE(name, '') AS name, \
           COALESCE(sort, 0) AS sort, \
           CAST(1 AS SIGNED) AS status, \
           CAST(0 AS SIGNED) AS updated_at \
         FROM {table} WHERE {pred}"
    )
}

pub async fn list_all(pool: &MySqlPool, kind: &str) -> Result<Vec<Category>, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    let order = if kind == "reason" {
        "ORDER BY id ASC".to_string()
    } else if is_flat(kind) {
        "ORDER BY sort DESC, id ASC".to_string()
    } else {
        format!("ORDER BY {pc} ASC, sort DESC, id ASC")
    };
    let sql = format!("{} {order}", select_sql(table, pc, kind));
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
        "{} AND {pc} = ? ORDER BY sort DESC, id ASC",
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
            "{} AND COALESCE(rec, 0) = 1 ORDER BY sort DESC, id ASC LIMIT ?",
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

pub async fn create(pool: &MySqlPool, c: CatCreate<'_>, _now: i64) -> Result<u64, sqlx::Error> {
    let Some((table, pc)) = resolve(c.kind) else {
        return Err(sqlx::Error::Protocol(format!(
            "unknown category kind: {}",
            c.kind
        )));
    };
    if c.kind == "industry" {
        let res = sqlx::query("INSERT INTO phpyun_industry (name, sort) VALUES (?, ?)")
            .bind(c.name)
            .bind(c.sort)
            .execute(pool)
            .await?;
        return Ok(res.last_insert_id());
    }
    if c.kind == "reason" {
        let res = sqlx::query("INSERT INTO phpyun_reason (name) VALUES (?)")
            .bind(c.name)
            .execute(pool)
            .await?;
        return Ok(res.last_insert_id());
    }
    if c.kind == "introduce" || c.kind == "introduce_class" {
        let res = sqlx::query(
            "INSERT INTO phpyun_introduce_class (name, sort, content) VALUES (?, ?, '')",
        )
        .bind(c.name)
        .bind(c.sort)
        .execute(pool)
        .await?;
        return Ok(res.last_insert_id());
    }
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
        return Err(sqlx::Error::Protocol(format!(
            "unknown category kind: {kind}"
        )));
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
        return Err(sqlx::Error::Protocol(format!(
            "unknown category kind: {kind}"
        )));
    };
    let sql = format!(
        "UPDATE {table} SET deleted=1 WHERE COALESCE(deleted,0)=0 AND (id = ? OR `{pc}` = ?)"
    );
    let res = sqlx::query(&sql).bind(id).bind(id).execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn patch_job_class(
    pool: &MySqlPool,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
    e_name: Option<&str>,
    s_name: Option<&str>,
    rec: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if name.is_none() && sort.is_none() && e_name.is_none() && s_name.is_none() && rec.is_none() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("UPDATE phpyun_job_class SET ");
    let mut first = true;
    if let Some(v) = name {
        if !first {
            qb.push(", ");
        }
        qb.push("name = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = sort {
        if !first {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = e_name {
        if !first {
            qb.push(", ");
        }
        qb.push("e_name = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = s_name {
        if !first {
            qb.push(", ");
        }
        qb.push("s_name = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = rec {
        if !first {
            qb.push(", ");
        }
        qb.push("rec = ");
        qb.push_bind(v);
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

/// PHP admin category row (extra columns Vue lists actually bind).
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct CatPhpRow {
    pub id: u64,
    pub keyid: u64,
    pub name: String,
    pub sort: i32,
    pub variable: String,
    pub e_name: String,
    pub letter: String,
    pub display: i32,
    pub code: String,
    pub content: String,
    pub rec: i32,
}

fn php_row_sql(kind: &str, table: &str) -> String {
    let pred = deleted_pred(kind);
    match kind {
        "city" => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(COALESCE(keyid,0) AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    COALESCE(sort,0) AS sort, \
                    '' AS variable, \
                    COALESCE(e_name,'') AS e_name, \
                    COALESCE(letter,'') AS letter, \
                    CAST(COALESCE(display,1) AS SIGNED) AS display, \
                    CAST(COALESCE(code,0) AS CHAR) AS code, \
                    '' AS content, \
                    CAST(0 AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
        "industry" => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(0 AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    COALESCE(sort,0) AS sort, \
                    '' AS variable, '' AS e_name, '' AS letter, \
                    CAST(1 AS SIGNED) AS display, '' AS code, '' AS content, \
                    CAST(0 AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
        "reason" => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(0 AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    CAST(0 AS SIGNED) AS sort, \
                    '' AS variable, '' AS e_name, '' AS letter, \
                    CAST(1 AS SIGNED) AS display, '' AS code, '' AS content, \
                    CAST(0 AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
        "introduce" | "introduce_class" => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(0 AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    COALESCE(sort,0) AS sort, \
                    '' AS variable, '' AS e_name, '' AS letter, \
                    CAST(1 AS SIGNED) AS display, '' AS code, \
                    COALESCE(content,'') AS content, \
                    CAST(0 AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
        "job" => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(COALESCE(keyid,0) AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    COALESCE(sort,0) AS sort, \
                    '' AS variable, \
                    COALESCE(e_name,'') AS e_name, \
                    '' AS letter, CAST(1 AS SIGNED) AS display, '' AS code, \
                    COALESCE(content,'') AS content, \
                    CAST(COALESCE(rec,0) AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
        _ => format!(
            "SELECT CAST(id AS UNSIGNED) AS id, \
                    CAST(COALESCE(keyid,0) AS UNSIGNED) AS keyid, \
                    COALESCE(name,'') AS name, \
                    COALESCE(sort,0) AS sort, \
                    COALESCE(variable,'') AS variable, \
                    '' AS e_name, '' AS letter, \
                    CAST(1 AS SIGNED) AS display, '' AS code, '' AS content, \
                    CAST(0 AS SIGNED) AS rec \
             FROM {table} WHERE {pred}"
        ),
    }
}

fn php_order(kind: &str) -> &'static str {
    match kind {
        "reason" => "ORDER BY id ASC",
        "industry" | "introduce" | "introduce_class" => "ORDER BY sort DESC, id ASC",
        "city" => "ORDER BY sort ASC, id ASC",
        _ => "ORDER BY id ASC, sort ASC",
    }
}

/// `parent = None` lists all (flat kinds) or roots (`keyid=0`).
pub async fn list_php(
    pool: &MySqlPool,
    kind: &str,
    parent: Option<u64>,
) -> Result<Vec<CatPhpRow>, sqlx::Error> {
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    let base = php_row_sql(kind, table);
    let sql = if is_flat(kind) {
        format!("{} {}", base, php_order(kind))
    } else if parent.is_some() {
        format!("{base} AND {pc} = ? {}", php_order(kind))
    } else {
        format!("{base} AND {pc} = 0 {}", php_order(kind))
    };
    let q = sqlx::query_as::<_, CatPhpRow>(&sql);
    let rows = if !is_flat(kind) {
        if let Some(pid) = parent {
            q.bind(pid).fetch_all(pool).await
        } else {
            q.fetch_all(pool).await
        }
    } else {
        q.fetch_all(pool).await
    };
    phpyun_core::db::ok_default_if_object_missing(rows)
}

pub async fn get_php(
    pool: &MySqlPool,
    kind: &str,
    id: u64,
) -> Result<Option<CatPhpRow>, sqlx::Error> {
    let Some((table, _)) = resolve(kind) else {
        return Ok(None);
    };
    let sql = format!("{} AND id = ? LIMIT 1", php_row_sql(kind, table));
    let r = sqlx::query_as::<_, CatPhpRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) || phpyun_core::db::is_missing_column(&e) => {
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

pub async fn insert_php(
    pool: &MySqlPool,
    kind: &str,
    parent_id: u64,
    name: &str,
    sort: i32,
    variable: &str,
) -> Result<u64, sqlx::Error> {
    let id = create(
        pool,
        CatCreate {
            parent_id,
            kind,
            name,
            sort,
        },
        0,
    )
    .await;
    let id = match id {
        Ok(v) => v,
        Err(e)
            if phpyun_core::db::is_missing_table(&e) || phpyun_core::db::is_missing_column(&e) =>
        {
            return Ok(0);
        }
        Err(e) => return Err(e),
    };
    if id > 0
        && !variable.is_empty()
        && matches!(
            kind,
            "userclass"
                | "user"
                | "comclass"
                | "company"
                | "com"
                | "part"
                | "partclass"
                | "schoolclass"
                | "school"
        )
    {
        let Some((table, _)) = resolve(kind) else {
            return Ok(id);
        };
        let sql = format!("UPDATE {table} SET variable = ? WHERE id = ?");
        let _ = sqlx::query(&sql)
            .bind(variable)
            .bind(id)
            .execute(pool)
            .await;
    }
    Ok(id)
}

/// Insert one city row (PHP `addCityClass`).
pub async fn insert_city(
    pool: &MySqlPool,
    keyid: u64,
    name: &str,
    letter: &str,
    display: i32,
    sort: i32,
    e_name: &str,
    code: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_city_class (keyid, name, letter, display, sort, e_name, code) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(keyid)
    .bind(name)
    .bind(letter)
    .bind(display)
    .bind(sort)
    .bind(e_name)
    .bind(code)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_city(
    pool: &MySqlPool,
    id: u64,
    name: &str,
    letter: &str,
    display: i32,
    sort: i32,
    e_name: &str,
    code: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_city_class SET name=?, letter=?, display=?, sort=?, e_name=?, code=? \
         WHERE id=?",
    )
    .bind(name)
    .bind(letter)
    .bind(display)
    .bind(sort)
    .bind(e_name)
    .bind(code)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn patch_php(
    pool: &MySqlPool,
    kind: &str,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
    e_name: Option<&str>,
    content: Option<&str>,
) -> Result<u64, sqlx::Error> {
    if kind == "job" {
        return patch_job_class(pool, id, name, sort, e_name, None, None).await;
    }
    let Some((table, _)) = resolve(kind) else {
        return Ok(0);
    };
    if name.is_none() && sort.is_none() && e_name.is_none() && content.is_none() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(format!("UPDATE {table} SET "));
    let mut first = true;
    if let Some(v) = name {
        qb.push("name = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = sort {
        if !first {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(v);
        first = false;
    }
    if let Some(v) = e_name {
        if kind == "city" {
            if !first {
                qb.push(", ");
            }
            qb.push("e_name = ");
            qb.push_bind(v);
            first = false;
        }
    }
    if let Some(v) = content {
        if kind == "introduce" || kind == "introduce_class" {
            if !first {
                qb.push(", ");
            }
            qb.push("content = ");
            qb.push_bind(v);
        }
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

async fn child_ids(pool: &MySqlPool, kind: &str, parents: &[u64]) -> Result<Vec<u64>, sqlx::Error> {
    if parents.is_empty() || is_flat(kind) {
        return Ok(vec![]);
    }
    let Some((table, pc)) = resolve(kind) else {
        return Ok(vec![]);
    };
    let mut qb = QueryBuilder::new(format!("SELECT CAST(id AS UNSIGNED) FROM {table} WHERE {pc} IN ("));
    for (i, id) in parents.iter().enumerate() {
        if i > 0 {
            qb.push(", ");
        }
        qb.push_bind(*id);
    }
    qb.push(")");
    match qb.build_query_as().fetch_all(pool).await {
        Ok(rows) => Ok(rows.into_iter().map(|(id,)| id).collect()),
        Err(e)
            if phpyun_core::db::is_missing_table(&e) || phpyun_core::db::is_missing_column(&e) =>
        {
            Ok(vec![])
        }
        Err(e) => Err(e),
    }
}

fn soft_table(kind: &str) -> Option<&'static str> {
    match kind {
        "job" => Some("phpyun_job_class"),
        "city" => Some("phpyun_city_class"),
        "userclass" | "user" => Some("phpyun_userclass"),
        "comclass" | "company" | "com" => Some("phpyun_comclass"),
        "part" | "partclass" => Some("phpyun_partclass"),
        "reason" => Some("phpyun_reason"),
        _ => None,
    }
}

pub async fn delete_php_ids(pool: &MySqlPool, kind: &str, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut all = ids.to_vec();
    let mut frontier = ids.to_vec();
    for _ in 0..8 {
        let kids = child_ids(pool, kind, &frontier).await?;
        let next: Vec<u64> = kids
            .into_iter()
            .filter(|k| !all.contains(k))
            .collect();
        if next.is_empty() {
            break;
        }
        all.extend_from_slice(&next);
        frontier = next;
    }
    if let Some(table) = soft_table(kind) {
        return soft_delete::mark_ids(pool, table, &all).await;
    }
    let Some((table, _)) = resolve(kind) else {
        return Ok(0);
    };
    let mut qb = QueryBuilder::new(format!("DELETE FROM {table} WHERE id IN ("));
    for (i, id) in all.iter().enumerate() {
        if i > 0 {
            qb.push(", ");
        }
        qb.push_bind(*id);
    }
    qb.push(")");
    match qb.build().execute(pool).await {
        Ok(res) => Ok(res.rows_affected()),
        Err(e)
            if phpyun_core::db::is_missing_table(&e) || phpyun_core::db::is_missing_column(&e) =>
        {
            Ok(0)
        }
        Err(e) => Err(e),
    }
}

pub async fn city_clear_pinyin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_city_class SET e_name = '' WHERE COALESCE(deleted,0)=0")
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct CityDupRow {
    pub id: u64,
    pub name: String,
    pub e_name: String,
}

pub async fn city_dup_pinyin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<CityDupRow>, sqlx::Error> {
    sqlx::query_as::<_, CityDupRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(e_name,'') AS e_name \
         FROM phpyun_city_class \
         WHERE COALESCE(deleted,0)=0 AND e_name <> '' \
           AND e_name IN ( \
             SELECT e_name FROM phpyun_city_class \
             WHERE COALESCE(deleted,0)=0 AND e_name <> '' \
             GROUP BY e_name HAVING COUNT(*) > 1 \
           ) \
         ORDER BY e_name ASC, id ASC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn patch_job_class_parent(
    pool: &MySqlPool,
    id: u64,
    keyid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_job_class SET keyid = ? WHERE id = ? AND COALESCE(deleted,0)=0")
        .bind(keyid)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn job_dup_pinyin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<CityDupRow>, sqlx::Error> {
    sqlx::query_as::<_, CityDupRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(e_name,'') AS e_name \
         FROM phpyun_job_class \
         WHERE COALESCE(deleted,0)=0 AND e_name <> '' \
           AND e_name IN ( \
             SELECT e_name FROM phpyun_job_class \
             WHERE COALESCE(deleted,0)=0 AND e_name <> '' \
             GROUP BY e_name HAVING COUNT(*) > 1 \
           ) \
         ORDER BY e_name ASC, id ASC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// `phpyun_city_class.id` → `(id, letter, name)` for sub-site grouping.
pub async fn city_meta_by_ids(
    pool: &MySqlPool,
    ids: &[i32],
) -> Result<Vec<(i32, String, String)>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS SIGNED), COALESCE(letter,''), COALESCE(name,'') \
         FROM phpyun_city_class WHERE id IN (",
    );
    {
        let mut sep = qb.separated(",");
        for id in ids {
            sep.push_bind(*id);
        }
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}
