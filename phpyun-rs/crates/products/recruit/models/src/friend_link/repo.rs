//! Strictly aligned with PHPYun `phpyun_admin_link` (friendly links).
//!
//! Rust FriendLink field -> PHP column:
//!   - name       <-> link_name
//!   - url        <-> link_url
//!   - logo       <-> pic
//!   - category   <-> link_type
//!   - sort       <-> link_sorting
//!   - status     <-> link_state
//!   - created_at = 0 (PHP `link_time` is varchar, not a timestamp)

use super::entity::{FriendLink, FriendLinkPhpRow};
use crate::soft_delete::{self, PREDICATE};
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(link_name, '') AS name, \
    COALESCE(link_url, '') AS url, \
    COALESCE(pic, '') AS logo, \
    COALESCE(link_type, '') AS category, \
    CAST(COALESCE(link_sorting, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(link_state, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS created_at";

pub async fn list_active(
    pool: &MySqlPool,
    category: Option<&str>,
) -> Result<Vec<FriendLink>, sqlx::Error> {
    let sql = match category {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_admin_link \
             WHERE link_state = 1 AND link_type = ? AND {PREDICATE} \
             ORDER BY link_sorting DESC, id ASC"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_admin_link \
             WHERE link_state = 1 AND {PREDICATE} \
             ORDER BY link_sorting DESC, id ASC"
        ),
    };
    let q = sqlx::query_as::<_, FriendLink>(&sql);
    match category {
        Some(c) => q.bind(c).fetch_all(pool).await,
        None => q.fetch_all(pool).await,
    }
}

pub async fn list_all(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<FriendLink>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_link \
         WHERE {PREDICATE} ORDER BY link_sorting DESC, id ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, FriendLink>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_all(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM phpyun_admin_link WHERE {PREDICATE}"))
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct FriendLinkUpsert<'a> {
    pub id: Option<u64>,
    pub link_name: &'a str,
    pub link_url: &'a str,
    pub pic: &'a str,
    pub link_type: &'a str,
    pub link_sorting: i32,
    pub link_state: i32,
}

pub async fn upsert(pool: &MySqlPool, a: FriendLinkUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_admin_link
               SET link_name = ?, link_url = ?, pic = ?, link_type = ?,
                   link_sorting = ?, link_state = ?
               WHERE id = ?"#,
        )
        .bind(a.link_name)
        .bind(a.link_url)
        .bind(a.pic)
        .bind(a.link_type)
        .bind(a.link_sorting)
        .bind(a.link_state)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_admin_link
           (link_name, link_url, pic, link_type, link_sorting, link_state)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(a.link_name)
    .bind(a.link_url)
    .bind(a.pic)
    .bind(a.link_type)
    .bind(a.link_sorting)
    .bind(a.link_state)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_admin_link", id).await
}

const PHP_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(link_name,'') AS link_name, \
    COALESCE(link_url,'') AS link_url, \
    COALESCE(pic,'') AS pic, \
    COALESCE(link_type,'') AS link_type, \
    CAST(COALESCE(link_sorting,0) AS SIGNED) AS link_sorting, \
    CAST(COALESCE(did,0) AS SIGNED) AS did, \
    CAST(COALESCE(tem_type,0) AS SIGNED) AS tem_type, \
    CAST(COALESCE(img_type,0) AS SIGNED) AS img_type, \
    CAST(COALESCE(link_state,0) AS SIGNED) AS link_state, \
    COALESCE(link_time,'') AS link_time, \
    COALESCE(statusbody,'') AS statusbody";

pub async fn php_find(pool: &MySqlPool, id: u64) -> Result<Option<FriendLinkPhpRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {PHP_FIELDS} FROM phpyun_admin_link WHERE id = ? AND {PREDICATE} LIMIT 1"
    );
    sqlx::query_as(&sql).bind(id).fetch_optional(pool).await
}

pub async fn set_did(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_admin_link SET did = ");
    qb.push_bind(did);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// PHP `set_friendlink::index_action` filters.
pub struct PhpLinkFilter<'a> {
    pub name_kw: Option<&'a str>,
    pub link_type: Option<&'a str>,
    pub did: Option<i32>,
    /// `Some(1)` approved; `Some(2)` pending/rejected (`link_state` in 0,2).
    pub state: Option<i32>,
    pub time_min: Option<i64>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_link_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpLinkFilter<'_>) {
    qb.push(format!(" FROM phpyun_admin_link WHERE {PREDICATE}"));
    if let Some(kw) = f.name_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND link_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(t) = f.link_type.filter(|s| !s.is_empty()) {
        qb.push(" AND link_type = ");
        qb.push_bind(t.to_string());
    }
    if let Some(d) = f.did.filter(|n| *n > 0) {
        qb.push(" AND did = ");
        qb.push_bind(d);
    }
    match f.state {
        Some(1) => {
            qb.push(" AND COALESCE(link_state,0) = 1");
        }
        Some(2) => {
            qb.push(" AND COALESCE(link_state,0) IN (0, 2)");
        }
        _ => {}
    }
    if let Some(t) = f.time_min {
        qb.push(" AND CAST(COALESCE(link_time,0) AS SIGNED) >= ");
        qb.push_bind(t);
    }
}

fn link_order(sort: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match sort {
        "link_time" | "ctime_n" => {
            if desc {
                " ORDER BY CAST(link_time AS SIGNED) DESC, id DESC"
            } else {
                " ORDER BY CAST(link_time AS SIGNED) ASC, id ASC"
            }
        }
        "link_sorting" => {
            if desc {
                " ORDER BY link_sorting DESC, id DESC"
            } else {
                " ORDER BY link_sorting ASC, id ASC"
            }
        }
        _ => {
            if desc {
                " ORDER BY id DESC"
            } else {
                " ORDER BY id ASC"
            }
        }
    }
}

pub async fn php_list(
    pool: &MySqlPool,
    f: &PhpLinkFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<FriendLinkPhpRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!("SELECT {PHP_FIELDS}"));
    push_link_where(&mut qb, f);
    qb.push(link_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count(pool: &MySqlPool, f: &PhpLinkFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_link_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct PhpLinkSave<'a> {
    pub id: Option<u64>,
    pub link_name: &'a str,
    pub link_url: &'a str,
    pub pic: Option<&'a str>,
    pub link_type: &'a str,
    pub link_sorting: i32,
    pub did: i32,
    pub tem_type: i32,
    pub img_type: i32,
    pub now: i64,
}

pub async fn php_save(pool: &MySqlPool, a: &PhpLinkSave<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
            "UPDATE phpyun_admin_link SET link_name = ",
        );
        qb.push_bind(a.link_name);
        qb.push(", link_url = ");
        qb.push_bind(a.link_url);
        qb.push(", link_type = ");
        qb.push_bind(a.link_type);
        qb.push(", link_sorting = ");
        qb.push_bind(a.link_sorting);
        qb.push(", did = ");
        qb.push_bind(a.did);
        qb.push(", tem_type = ");
        qb.push_bind(a.tem_type);
        qb.push(", img_type = ");
        qb.push_bind(a.img_type);
        qb.push(", link_state = 1");
        if let Some(pic) = a.pic {
            qb.push(", pic = ");
            qb.push_bind(pic);
        }
        qb.push(" WHERE id = ");
        qb.push_bind(id);
        qb.build().execute(pool).await?;
        Ok(id)
    } else {
        let pic = a.pic.unwrap_or("");
        let res = sqlx::query(
            "INSERT INTO phpyun_admin_link \
             (link_name, link_url, pic, link_type, link_sorting, link_state, link_time, did, tem_type, img_type) \
             VALUES (?, ?, ?, ?, ?, 1, ?, ?, ?, ?)",
        )
        .bind(a.link_name)
        .bind(a.link_url)
        .bind(pic)
        .bind(a.link_type)
        .bind(a.link_sorting)
        .bind(a.now.to_string())
        .bind(a.did)
        .bind(a.tem_type)
        .bind(a.img_type)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn php_set_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_admin_link SET link_state = ?, statusbody = ? WHERE id = ?",
    )
    .bind(status)
    .bind(statusbody)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn php_delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_admin_link WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
