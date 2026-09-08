//! PHPYun `phpyun_email_msg` (email send log).

use super::entity::EmailMsg;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS SIGNED) AS uid, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(cuid, 0) AS SIGNED) AS cuid, \
    COALESCE(cname, '') AS cname, \
    COALESCE(email, '') AS email, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    CAST(COALESCE(state, 0) AS SIGNED) AS state, \
    COALESCE(smtpserver, '') AS smtpserver, \
    CAST(COALESCE(del, 0) AS SIGNED) AS del";

/// PHP `emaillog::index_action` filters. `del <> 1`.
pub struct PhpEmailFilter<'a> {
    pub email_kw: Option<&'a str>,
    pub smtp_kw: Option<&'a str>,
    pub cuid_zero: bool,
    pub uid_zero: bool,
    /// `Some([])` means no matching senders — force empty result.
    pub cuid_in: Option<&'a [u64]>,
    pub uid_in: Option<&'a [u64]>,
    pub time_min: Option<i64>,
    pub time_max: Option<i64>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_email_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpEmailFilter<'_>) {
    qb.push(" FROM phpyun_email_msg WHERE COALESCE(del,0) <> 1");
    if let Some(kw) = f.email_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND email LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(kw) = f.smtp_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND smtpserver LIKE ");
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

fn email_order(sort: &str, dir: &str) -> &'static str {
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

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<EmailMsg>, sqlx::Error> {
    php_list(
        pool,
        &PhpEmailFilter {
            email_kw: None,
            smtp_kw: None,
            cuid_zero: false,
            uid_zero: false,
            cuid_in: None,
            uid_in: None,
            time_min: None,
            time_max: None,
            sort: "id",
            dir: "desc",
        },
        offset,
        limit,
    )
    .await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    php_count(
        pool,
        &PhpEmailFilter {
            email_kw: None,
            smtp_kw: None,
            cuid_zero: false,
            uid_zero: false,
            cuid_in: None,
            uid_in: None,
            time_min: None,
            time_max: None,
            sort: "id",
            dir: "desc",
        },
    )
    .await
}

pub async fn php_list(
    pool: &MySqlPool,
    f: &PhpEmailFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<EmailMsg>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!("SELECT {FIELDS}"));
    push_email_where(&mut qb, f);
    qb.push(email_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count(pool: &MySqlPool, f: &PhpEmailFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_email_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn get_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<EmailMsg>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {FIELDS} FROM phpyun_email_msg WHERE id IN ("));
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
        QueryBuilder::new("UPDATE phpyun_email_msg SET state = ");
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
        QueryBuilder::new("DELETE FROM phpyun_email_msg WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
