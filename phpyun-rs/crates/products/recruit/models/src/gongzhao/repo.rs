//! Strictly aligned with PHPYun `phpyun_gongzhao` (public recruitment).
//!
//! PHP columns: id/title/keyword/description/content/datetime/did/startime/endtime/pic/rec
//! Rust Gongzhao field -> PHP column:
//!   - cover      <-> pic
//!   - body       <-> content
//!   - tag        <-> keyword
//!   - status     = 1 (no status column in PHP)
//!   - view_count = 0 (no view-count column in PHP; Rust stub)
//!   - start_at   <-> startime
//!   - end_at     <-> endtime
//!   - created_at <-> datetime

use super::entity::Gongzhao;
use crate::soft_delete::{self, PREDICATE};
use sqlx::{FromRow, MySqlPool, QueryBuilder};
use serde::{Deserialize, Serialize};

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(description, '') AS description, \
    COALESCE(pic, '') AS cover, \
    COALESCE(content, '') AS body, \
    COALESCE(keyword, '') AS tag, \
    CAST(1 AS SIGNED) AS status, \
    CAST(0 AS UNSIGNED) AS view_count, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS created_at";

pub async fn list(
    pool: &MySqlPool,
    tag: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Gongzhao>, sqlx::Error> {
    let sql = match tag {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao WHERE keyword = ? AND {PREDICATE} \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao WHERE {PREDICATE} \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Gongzhao>(&sql);
    match tag {
        Some(t) => q.bind(t).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count(pool: &MySqlPool, tag: Option<&str>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match tag {
        Some(t) => {
            sqlx::query_as(&format!(
                "SELECT COUNT(*) FROM phpyun_gongzhao WHERE keyword = ? AND {PREDICATE}"
            ))
                .bind(t)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as(&format!("SELECT COUNT(*) FROM phpyun_gongzhao WHERE {PREDICATE}"))
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Gongzhao>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_gongzhao WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Gongzhao>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_view(_pool: &MySqlPool, _id: u64) -> Result<(), sqlx::Error> {
    // PHPYun phpyun_gongzhao has no view-count column; this op is a no-op.
    Ok(())
}

pub struct GongzhaoUpsert<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub keyword: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub pic: &'a str,
    pub startime: i64,
    pub endtime: i64,
    pub did: i32,
    pub now: i64,
}

pub async fn upsert(pool: &MySqlPool, a: GongzhaoUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_gongzhao
               SET title = ?, keyword = ?, description = ?, content = ?, pic = ?,
                   startime = ?, endtime = ?, did = ?
               WHERE id = ?"#,
        )
        .bind(a.title)
        .bind(a.keyword)
        .bind(a.description)
        .bind(a.content)
        .bind(a.pic)
        .bind(a.startime)
        .bind(a.endtime)
        .bind(a.did)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_gongzhao
           (title, keyword, description, content, datetime, did, startime, endtime, pic, rec)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)"#,
    )
    .bind(a.title)
    .bind(a.keyword)
    .bind(a.description)
    .bind(a.content)
    .bind(a.now)
    .bind(a.did)
    .bind(a.startime)
    .bind(a.endtime)
    .bind(a.pic)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_gongzhao", id).await
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GongzhaoAdminRow {
    pub id: u64,
    pub title: String,
    pub keyword: String,
    pub description: String,
    pub content: String,
    pub pic: String,
    pub datetime: i64,
    pub startime: i64,
    pub endtime: i64,
    pub did: i32,
    pub rec: i32,
}

pub struct GongzhaoAdminFilter<'a> {
    pub keyword: Option<&'a str>,
    pub datetime_min: Option<i64>,
    pub order_col: &'a str,
    pub order_dir: &'a str,
}

fn gongzhao_order_sql(col: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match col {
        "datetime" => {
            if desc {
                "datetime DESC, id DESC"
            } else {
                "datetime ASC, id ASC"
            }
        }
        "startime" => {
            if desc {
                "startime DESC, id DESC"
            } else {
                "startime ASC, id ASC"
            }
        }
        "endtime" => {
            if desc {
                "endtime DESC, id DESC"
            } else {
                "endtime ASC, id ASC"
            }
        }
        "title" => {
            if desc {
                "title DESC, id DESC"
            } else {
                "title ASC, id ASC"
            }
        }
        _ => {
            if desc {
                "id DESC"
            } else {
                "id ASC"
            }
        }
    }
}

const ADMIN_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(keyword, '') AS keyword, \
    COALESCE(description, '') AS description, \
    COALESCE(content, '') AS content, \
    COALESCE(pic, '') AS pic, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS datetime, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS startime, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS endtime, \
    CAST(COALESCE(did, 0) AS SIGNED) AS did, \
    CAST(COALESCE(rec, 0) AS SIGNED) AS rec";

fn push_gongzhao_admin_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &GongzhaoAdminFilter<'_>) {
    qb.push(format!(" FROM phpyun_gongzhao WHERE {PREDICATE}"));
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(min) = f.datetime_min.filter(|n| *n > 0) {
        qb.push(" AND datetime >= ");
        qb.push_bind(min);
    }
}

pub async fn list_admin(
    pool: &MySqlPool,
    f: &GongzhaoAdminFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<GongzhaoAdminRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!("SELECT {ADMIN_FIELDS}"));
    push_gongzhao_admin_where(&mut qb, f);
    qb.push(" ORDER BY ");
    qb.push(gongzhao_order_sql(f.order_col, f.order_dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_admin(pool: &MySqlPool, f: &GongzhaoAdminFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_gongzhao_admin_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_gongzhao", ids).await
}

pub async fn set_rec(pool: &MySqlPool, id: u64, rec: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_gongzhao SET rec = ? WHERE id = ?")
            .bind(rec)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_did_ids(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_gongzhao SET did = ");
    qb.push_bind(did);
    qb.push(" WHERE id IN (");
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
