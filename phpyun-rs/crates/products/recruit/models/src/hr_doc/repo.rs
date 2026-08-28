//! HR toolbox documents -- corresponds to PHPYun table `phpyun_toolbox_doc`.
//!
//! Column mapping (Rust HrDoc field <- PHP column):
//!   - `hits`       <- `downnum` (PHPYun uses "download count" as a popularity proxy)
//!   - `body`       = '' (this PHPYun table has no body column; the toolbox
//!     usually just redirects via url)
//!   - `created_at` <- `add_time` (UNIX seconds)
//!   - `updated_at` <- `add_time` (PHPYun doesn't maintain updated time;
//!     falls back to publish time)

use super::entity::{HrDoc, ToolboxClass};
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, \
    COALESCE(cid, 0) AS cid, \
    COALESCE(name, '') AS name, \
    COALESCE(url, '') AS url, \
    '' AS body, \
    COALESCE(downnum, 0) AS hits, \
    COALESCE(is_show, 0) AS is_show, \
    COALESCE(add_time, 0) AS created_at, \
    COALESCE(add_time, 0) AS updated_at";

pub async fn list_public(
    pool: &MySqlPool,
    cid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<HrDoc>, sqlx::Error> {
    let sql = match cid {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_toolbox_doc \
             WHERE is_show = 1 AND cid = ? \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_toolbox_doc \
             WHERE is_show = 1 \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, HrDoc>(&sql);
    match cid {
        Some(c) => q.bind(c).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count_public(pool: &MySqlPool, cid: Option<u64>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match cid {
        Some(c) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_toolbox_doc WHERE is_show = 1 AND cid = ?")
                .bind(c)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_toolbox_doc WHERE is_show = 1")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<HrDoc>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_toolbox_doc WHERE id = ? AND is_show = 1");
    sqlx::query_as::<_, HrDoc>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_hit(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_toolbox_doc SET downnum = downnum + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

const ADMIN_DOC_FIELDS: &str = "id, \
    COALESCE(cid, 0) AS cid, \
    COALESCE(name, '') AS name, \
    COALESCE(url, '') AS url, \
    '' AS body, \
    COALESCE(downnum, 0) AS hits, \
    COALESCE(is_show, 0) AS is_show, \
    COALESCE(add_time, 0) AS created_at, \
    COALESCE(add_time, 0) AS updated_at";

pub async fn list_admin(
    pool: &MySqlPool,
    cid: Option<u64>,
    keyword: Option<&str>,
    is_show: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<HrDoc>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(ADMIN_DOC_FIELDS);
    qb.push(" FROM phpyun_toolbox_doc WHERE 1=1");
    if let Some(c) = cid.filter(|v| *v > 0) {
        qb.push(" AND cid = ");
        qb.push_bind(c);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(show) = is_show {
        qb.push(" AND is_show = ");
        qb.push_bind(show);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?);
    qb.push(" OFFSET ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(
        offset,
        "pagination.offset",
    )?);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_admin(
    pool: &MySqlPool,
    cid: Option<u64>,
    keyword: Option<&str>,
    is_show: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_toolbox_doc WHERE 1=1");
    if let Some(c) = cid.filter(|v| *v > 0) {
        qb.push(" AND cid = ");
        qb.push_bind(c);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(show) = is_show {
        qb.push(" AND is_show = ");
        qb.push_bind(show);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_admin(pool: &MySqlPool, id: u64) -> Result<Option<HrDoc>, sqlx::Error> {
    let sql = format!("SELECT {ADMIN_DOC_FIELDS} FROM phpyun_toolbox_doc WHERE id = ?");
    sqlx::query_as::<_, HrDoc>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_doc(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    cid: u64,
    url: &str,
    is_show: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        if url.is_empty() {
            sqlx::query("UPDATE phpyun_toolbox_doc SET name=?, cid=?, is_show=? WHERE id=?")
                .bind(name)
                .bind(cid)
                .bind(is_show)
                .bind(id)
                .execute(pool)
                .await?;
        } else {
            sqlx::query(
                "UPDATE phpyun_toolbox_doc SET name=?, cid=?, url=?, is_show=? WHERE id=?",
            )
            .bind(name)
            .bind(cid)
            .bind(url)
            .bind(is_show)
            .bind(id)
            .execute(pool)
            .await?;
        }
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_toolbox_doc (cid, name, url, is_show, add_time, downnum) \
         VALUES (?, ?, ?, ?, ?, 0)",
    )
    .bind(cid)
    .bind(name)
    .bind(url)
    .bind(is_show)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_doc_show(pool: &MySqlPool, id: u64, is_show: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_toolbox_doc SET is_show = ? WHERE id = ?")
        .bind(is_show)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_docs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_toolbox_doc WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn list_classes(pool: &MySqlPool) -> Result<Vec<ToolboxClass>, sqlx::Error> {
    sqlx::query_as::<_, ToolboxClass>(
        "SELECT CAST(id AS UNSIGNED) AS id, \
                COALESCE(name, '') AS name, \
                COALESCE(content, '') AS content, \
                COALESCE(pic, '') AS pic \
         FROM phpyun_toolbox_class ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn upsert_class(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    content: &str,
    pic: Option<&str>,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        if let Some(p) = pic {
            sqlx::query("UPDATE phpyun_toolbox_class SET name=?, content=?, pic=? WHERE id=?")
                .bind(name)
                .bind(content)
                .bind(p)
                .bind(id)
                .execute(pool)
                .await?;
        } else {
            sqlx::query("UPDATE phpyun_toolbox_class SET name=?, content=? WHERE id=?")
                .bind(name)
                .bind(content)
                .bind(id)
                .execute(pool)
                .await?;
        }
        return Ok(id);
    }
    let res = sqlx::query("INSERT INTO phpyun_toolbox_class (name, content, pic) VALUES (?, ?, ?)")
        .bind(name)
        .bind(content)
        .bind(pic.unwrap_or(""))
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_classes(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_toolbox_doc WHERE cid IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    qb.build().execute(pool).await?;
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_toolbox_class WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
