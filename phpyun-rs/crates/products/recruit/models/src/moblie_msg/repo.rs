//! PHPYun `phpyun_moblie_msg` (SMS send log).

use super::entity::MoblieMsg;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS SIGNED) AS uid, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(cuid, 0) AS SIGNED) AS cuid, \
    COALESCE(cname, '') AS cname, \
    COALESCE(moblie, '') AS moblie, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    CAST(COALESCE(state, 0) AS SIGNED) AS state, \
    COALESCE(ip, '') AS ip, \
    CAST(COALESCE(del, 0) AS SIGNED) AS del, \
    CAST(COALESCE(msgtype, 0) AS SIGNED) AS msgtype, \
    CAST(COALESCE(port, 0) AS SIGNED) AS port, \
    COALESCE(location, '') AS location";

/// PHP `messagelog::index_action`. Default `port <> 6`.
pub struct PhpSmsFilter<'a> {
    pub moblie_kw: Option<&'a str>,
    pub content_kw: Option<&'a str>,
    pub cuid_zero: bool,
    pub uid_zero: bool,
    pub cuid_in: Option<&'a [u64]>,
    pub uid_in: Option<&'a [u64]>,
    pub time_min: Option<i64>,
    pub time_max: Option<i64>,
    /// `Some(0)` success, `Some(other)` failed (`state <> 0`).
    pub state: Option<i32>,
    /// `None` keeps PHP default `port <> 6`.
    pub port: Option<i32>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_sms_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpSmsFilter<'_>) {
    qb.push(" FROM phpyun_moblie_msg WHERE COALESCE(del,0) <> 1");
    if let Some(kw) = f.moblie_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND moblie LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(kw) = f.content_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND content LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if f.cuid_zero {
        qb.push(" AND COALESCE(cuid,0) = 0");
    } else if let Some(ids) = f.cuid_in {
        push_uid_in(qb, "cuid", ids);
    }
    if f.uid_zero {
        qb.push(" AND COALESCE(uid,0) = 0");
    } else if let Some(ids) = f.uid_in {
        push_uid_in(qb, "uid", ids);
    }
    if let Some(t) = f.time_min {
        qb.push(" AND ctime >= ");
        qb.push_bind(t);
    }
    if let Some(t) = f.time_max {
        qb.push(" AND ctime <= ");
        qb.push_bind(t);
    }
    match f.state {
        Some(0) => {
            qb.push(" AND COALESCE(state,0) = 0");
        }
        Some(_) => {
            qb.push(" AND COALESCE(state,0) <> 0");
        }
        None => {}
    }
    match f.port {
        Some(p) => {
            qb.push(" AND port = ");
            qb.push_bind(p);
        }
        None => {
            qb.push(" AND COALESCE(port,0) <> 6");
        }
    }
}

fn push_uid_in(qb: &mut QueryBuilder<'_, sqlx::MySql>, col: &str, ids: &[u64]) {
    if ids.is_empty() {
        qb.push(" AND 1=0");
        return;
    }
    qb.push(" AND ");
    qb.push(col);
    qb.push(" IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") AND COALESCE(");
    qb.push(col);
    qb.push(",0) <> 0");
}

fn sms_order(sort: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match sort {
        "ctime" => {
            if desc {
                " ORDER BY ctime DESC, id DESC"
            } else {
                " ORDER BY ctime ASC, id ASC"
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

fn empty_filter<'a>() -> PhpSmsFilter<'a> {
    PhpSmsFilter {
        moblie_kw: None,
        content_kw: None,
        cuid_zero: false,
        uid_zero: false,
        cuid_in: None,
        uid_in: None,
        time_min: None,
        time_max: None,
        state: None,
        port: None,
        sort: "id",
        dir: "desc",
    }
}

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<MoblieMsg>, sqlx::Error> {
    php_list(pool, &empty_filter(), offset, limit).await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    php_count(pool, &empty_filter()).await
}

pub async fn php_list(
    pool: &MySqlPool,
    f: &PhpSmsFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<MoblieMsg>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!("SELECT {FIELDS}"));
    push_sms_where(&mut qb, f);
    qb.push(sms_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count(pool: &MySqlPool, f: &PhpSmsFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_sms_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn get_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<MoblieMsg>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {FIELDS} FROM phpyun_moblie_msg WHERE id IN ("));
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn set_state(pool: &MySqlPool, ids: &[u64], state: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_moblie_msg SET state = ");
    qb.push_bind(state);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_moblie_msg WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
