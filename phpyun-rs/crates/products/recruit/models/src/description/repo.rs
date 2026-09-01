//! Strictly aligned with PHPYun `phpyun_description` + `phpyun_desc_class`.
//!
//! Column mapping:
//!   - Description.class_id <-> nid
//!   - Description.link_url <-> url
//!   - Description.status   = 1 (no such column in PHP)
//!   - Description.created_at <-> ctime
//!   - Description.updated_at <-> ctime (PHP does not maintain updated)
//!   - DescClass.created_at = 0 (PHP `phpyun_desc_class` has no ctime column)

use super::entity::{DescClass, Description};
use crate::soft_delete::{self, PREDICATE};
use sqlx::MySqlPool;

const CLASS_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(0 AS SIGNED) AS created_at";

const DESC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(nid, 0) AS UNSIGNED) AS class_id, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(is_type, 0) AS SIGNED) AS is_type, \
    COALESCE(url, '') AS link_url, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(1 AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS updated_at";

// ---------- classes ----------

pub async fn list_classes(pool: &MySqlPool) -> Result<Vec<DescClass>, sqlx::Error> {
    let sql = format!("SELECT {CLASS_FIELDS} FROM phpyun_desc_class WHERE {PREDICATE} ORDER BY sort ASC, id ASC");
    sqlx::query_as::<_, DescClass>(&sql).fetch_all(pool).await
}

pub async fn insert_class(
    pool: &MySqlPool,
    name: &str,
    sort: i32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO phpyun_desc_class (name, sort) VALUES (?, ?)")
        .bind(name)
        .bind(sort)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn update_class_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    php_update_class(pool, id, None, Some(sort)).await
}

/// PHP `singleclass::index`: `orderby=sort,desc`, optional page window.
pub async fn php_list_classes(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<DescClass>, sqlx::Error> {
    let sql = format!(
        "SELECT {CLASS_FIELDS} FROM phpyun_desc_class WHERE {PREDICATE} \
         ORDER BY sort DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, DescClass>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn php_list_all_classes(pool: &MySqlPool) -> Result<Vec<DescClass>, sqlx::Error> {
    let sql = format!(
        "SELECT {CLASS_FIELDS} FROM phpyun_desc_class WHERE {PREDICATE} ORDER BY sort DESC, id DESC"
    );
    sqlx::query_as::<_, DescClass>(&sql).fetch_all(pool).await
}

pub async fn php_count_classes(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_desc_class WHERE {PREDICATE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `addDesClass`: if any of the submitted names already exist, refuse the whole batch.
pub async fn php_class_names_exist(pool: &MySqlPool, names: &[String]) -> Result<bool, sqlx::Error> {
    if names.is_empty() {
        return Ok(false);
    }
    let mut qb = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM phpyun_desc_class WHERE ");
    qb.push(PREDICATE);
    qb.push(" AND name IN (");
    {
        let mut sep = qb.separated(", ");
        for n in names {
            sep.push_bind(n);
        }
    }
    qb.push(")");
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n > 0)
}

/// PHP `upDesClass`: name and/or sort. Vue ajax sends only the changed field.
pub async fn php_update_class(
    pool: &MySqlPool,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if name.is_none() && sort.is_none() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("UPDATE phpyun_desc_class SET ");
    let mut first = true;
    if let Some(n) = name {
        qb.push("name = ");
        qb.push_bind(n);
        first = false;
    }
    if let Some(s) = sort {
        if !first {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(s);
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND ");
    qb.push(PREDICATE);
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_class(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    php_delete_class_ids(pool, &[id]).await
}

pub async fn php_delete_class_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_desc_class", ids).await
}

// ---------- descriptions ----------

pub async fn list(
    pool: &MySqlPool,
    class_id: Option<u64>,
    _only_visible: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Description>, sqlx::Error> {
    // PHPYun has no status column; only_visible has no effect (PHP itself doesn't filter).
    let mut sql = format!("SELECT {DESC_FIELDS} FROM phpyun_description WHERE {PREDICATE}");
    if class_id.is_some() {
        sql.push_str(" AND nid = ?");
    }
    sql.push_str(" ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?");
    let mut q = sqlx::query_as::<_, Description>(&sql);
    if let Some(c) = class_id {
        q = q.bind(c);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count(
    pool: &MySqlPool,
    class_id: Option<u64>,
    _only_visible: bool,
) -> Result<u64, sqlx::Error> {
    let mut sql = format!("SELECT COUNT(*) FROM phpyun_description WHERE {PREDICATE}");
    if class_id.is_some() {
        sql.push_str(" AND nid = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(c) = class_id {
        q = q.bind(c);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn get(pool: &MySqlPool, id: u64) -> Result<Option<Description>, sqlx::Error> {
    let sql = format!("SELECT {DESC_FIELDS} FROM phpyun_description WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Description>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Find the first description whose `name` column matches `name` (e.g.
/// "关于我们" / "联系我们" / "隐私政策" / "注册协议"). Mirrors PHP
/// `description::getDes(array('name' => ...))` which back-queries the
/// `phpyun_description.name` slug column directly.
pub async fn find_by_name(
    pool: &MySqlPool,
    name: &str,
) -> Result<Option<Description>, sqlx::Error> {
    let sql = format!(
        "SELECT {DESC_FIELDS} FROM phpyun_description \
         WHERE name = ? AND {PREDICATE} \
         ORDER BY sort ASC, id DESC LIMIT 1"
    );
    sqlx::query_as::<_, Description>(&sql)
        .bind(name)
        .fetch_optional(pool)
        .await
}

pub struct UpsertDesc<'a> {
    pub id: Option<u64>,
    pub class_id: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub is_type: i32,
    pub link_url: &'a str,
    pub sort: i32,
    pub status: i32,
}

pub async fn upsert(pool: &MySqlPool, d: &UpsertDesc<'_>, now: i64) -> Result<u64, sqlx::Error> {
    // PHPYun phpyun_description columns: nid/name/url/title/keyword/descs/top_tpl/...
    // Only write the columns that map to Rust-side fields.
    if let Some(id) = d.id {
        sqlx::query(
            "UPDATE phpyun_description \
             SET nid = ?, title = ?, content = ?, is_type = ?, url = ?, sort = ? \
             WHERE id = ?",
        )
        .bind(d.class_id)
        .bind(d.title)
        .bind(d.content)
        .bind(d.is_type)
        .bind(d.link_url)
        .bind(d.sort)
        .bind(id)
        .execute(pool)
        .await?;
        let _ = (d.status, now);
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_description \
             (nid, title, content, is_type, url, sort, ctime) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(d.class_id)
        .bind(d.title)
        .bind(d.content)
        .bind(d.is_type)
        .bind(d.link_url)
        .bind(d.sort)
        .bind(now)
        .execute(pool)
        .await?;
        let _ = d.status;
        Ok(res.last_insert_id())
    }
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_description", id).await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct PhpDescRow {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub content: String,
    pub is_type: i32,
    pub is_nav: i32,
    pub sort: i32,
    pub url: String,
    pub ctime: i64,
    pub nid: u64,
    pub keyword: String,
    pub descs: String,
    pub top_tpl: i32,
    pub top_tpl_dir: String,
    pub footer_tpl: i32,
    pub footer_tpl_dir: String,
    pub is_menu: i32,
}

const PHP_DESC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(is_type, 0) AS SIGNED) AS is_type, \
    CAST(COALESCE(is_nav, 0) AS SIGNED) AS is_nav, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    COALESCE(url, '') AS url, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    CAST(COALESCE(nid, 0) AS UNSIGNED) AS nid, \
    COALESCE(keyword, '') AS keyword, \
    COALESCE(descs, '') AS descs, \
    CAST(COALESCE(top_tpl, 1) AS SIGNED) AS top_tpl, \
    COALESCE(top_tpl_dir, '') AS top_tpl_dir, \
    CAST(COALESCE(footer_tpl, 1) AS SIGNED) AS footer_tpl, \
    COALESCE(footer_tpl_dir, '') AS footer_tpl_dir, \
    CAST(COALESCE(is_menu, 0) AS SIGNED) AS is_menu";

pub async fn php_list(
    pool: &MySqlPool,
    keyword: Option<&str>,
    is_type: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpDescRow>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new("SELECT ");
    qb.push(PHP_DESC_FIELDS);
    qb.push(" FROM phpyun_description WHERE ");
    qb.push(PREDICATE);
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(t) = is_type {
        qb.push(" AND is_type = ");
        qb.push_bind(t);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<PhpDescRow>().fetch_all(pool).await
}

pub async fn php_count(
    pool: &MySqlPool,
    keyword: Option<&str>,
    is_type: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM phpyun_description WHERE ");
    qb.push(PREDICATE);
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(t) = is_type {
        qb.push(" AND is_type = ");
        qb.push_bind(t);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_get(pool: &MySqlPool, id: u64) -> Result<Option<PhpDescRow>, sqlx::Error> {
    let sql = format!("SELECT {PHP_DESC_FIELDS} FROM phpyun_description WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, PhpDescRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct PhpDescSave<'a> {
    pub name: &'a str,
    pub nid: u64,
    pub url: &'a str,
    pub title: &'a str,
    pub keyword: &'a str,
    pub descs: &'a str,
    pub content: &'a str,
    pub sort: i32,
    pub is_nav: i32,
    pub is_type: i32,
    pub top_tpl: i32,
    pub top_tpl_dir: &'a str,
    pub footer_tpl: i32,
    pub footer_tpl_dir: &'a str,
}

pub async fn php_upsert(pool: &MySqlPool, id: u64, s: &PhpDescSave<'_>, now: i64) -> Result<u64, sqlx::Error> {
    if id > 0 {
        sqlx::query(
            "UPDATE phpyun_description SET name=?, nid=?, url=?, title=?, keyword=?, descs=?, \
             content=?, sort=?, is_nav=?, is_type=?, top_tpl=?, top_tpl_dir=?, \
             footer_tpl=?, footer_tpl_dir=?, ctime=? WHERE id=?",
        )
        .bind(s.name)
        .bind(s.nid)
        .bind(s.url)
        .bind(s.title)
        .bind(s.keyword)
        .bind(s.descs)
        .bind(s.content)
        .bind(s.sort)
        .bind(s.is_nav)
        .bind(s.is_type)
        .bind(s.top_tpl)
        .bind(s.top_tpl_dir)
        .bind(s.footer_tpl)
        .bind(s.footer_tpl_dir)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_description \
             (name, nid, url, title, keyword, descs, content, sort, is_nav, is_type, \
              top_tpl, top_tpl_dir, footer_tpl, footer_tpl_dir, ctime) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(s.name)
        .bind(s.nid)
        .bind(s.url)
        .bind(s.title)
        .bind(s.keyword)
        .bind(s.descs)
        .bind(s.content)
        .bind(s.sort)
        .bind(s.is_nav)
        .bind(s.is_type)
        .bind(s.top_tpl)
        .bind(s.top_tpl_dir)
        .bind(s.footer_tpl)
        .bind(s.footer_tpl_dir)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn php_set_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_description SET sort = ? WHERE id = ?")
        .bind(sort)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn php_delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_description", ids).await
}
