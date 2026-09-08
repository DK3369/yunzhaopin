//! `phpyun_user_entrust` — list / count / delete / status.

use super::entity::{TrustStat, UserEntrustRow};
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "CAST(e.id AS UNSIGNED) AS id, \
    CAST(COALESCE(e.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(e.eid,0) AS UNSIGNED) AS eid, \
    CAST(COALESCE(e.price,0) AS CHAR) AS price, \
    CAST(COALESCE(e.status,0) AS SIGNED) AS status, \
    CAST(COALESCE(e.add_time,0) AS SIGNED) AS add_time, \
    COALESCE(r.name,'') AS uname, \
    COALESCE(ex.name,'') AS name";

const FROM: &str = " FROM phpyun_user_entrust e \
    LEFT JOIN (SELECT uid, MAX(name) AS name FROM phpyun_resume GROUP BY uid) r ON r.uid = e.uid \
    LEFT JOIN phpyun_resume_expect ex ON ex.id = e.eid \
    LEFT JOIN phpyun_member m ON m.uid = e.uid";

pub struct Filter<'a> {
    pub status: Option<i32>,
    pub keyword: Option<&'a str>,
    /// 1 = 姓名 (`resume.name` / `member.username`), 2 = 期望职位.
    pub name_kind: i32,
    pub since: Option<i64>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn apply_filter(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &Filter<'_>) {
    if let Some(s) = f.status {
        qb.push(" AND e.status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if f.name_kind == 2 {
            qb.push(" AND ex.name LIKE ");
            qb.push_bind(like);
        } else {
            qb.push(" AND (r.name LIKE ");
            qb.push_bind(like.clone());
            qb.push(" OR m.username LIKE ");
            qb.push_bind(like);
            qb.push(")");
        }
    }
    if let Some(since) = f.since {
        qb.push(" AND e.add_time >= ");
        qb.push_bind(since);
    }
}

fn order_clause(sort: &str, dir: &str) -> (&'static str, &'static str) {
    let col = match sort {
        "uid" => "uid",
        "add_time" => "add_time",
        "price" => "price",
        "status" => "status",
        _ => "id",
    };
    let dir = if dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };
    (col, dir)
}

pub async fn list(
    pool: &MySqlPool,
    f: &Filter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<UserEntrustRow>, sqlx::Error> {
    let (l, o) = (
        phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?,
        phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?,
    );
    let mut qb = QueryBuilder::new(format!("SELECT {FIELDS}{FROM} WHERE 1=1"));
    apply_filter(&mut qb, f);
    let (col, dir) = order_clause(f.sort, f.dir);
    qb.push(" ORDER BY e.");
    qb.push(col);
    qb.push(" ");
    qb.push(dir);
    qb.push(" LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count(pool: &MySqlPool, f: &Filter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!("SELECT COUNT(*){FROM} WHERE 1=1"));
    apply_filter(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn stat(pool: &MySqlPool) -> Result<TrustStat, sqlx::Error> {
    async fn n(pool: &MySqlPool, sql: &str) -> Result<u64, sqlx::Error> {
        let (c,): (i64,) = sqlx::query_as(sql).fetch_one(pool).await?;
        Ok(phpyun_core::numeric::nonnegative_count(c))
    }
    Ok(TrustStat {
        resume_all_num: n(pool, "SELECT COUNT(*) FROM phpyun_user_entrust").await?,
        resume_status_num1: n(
            pool,
            "SELECT COUNT(*) FROM phpyun_user_entrust WHERE status=0",
        )
        .await?,
        resume_status_num2: n(
            pool,
            "SELECT COUNT(*) FROM phpyun_user_entrust WHERE status=2",
        )
        .await?,
    })
}

pub async fn delete_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_user_entrust WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    auid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_user_entrust SET status=?, audit_time=?, auid=? WHERE id=?",
    )
    .bind(status)
    .bind(now)
    .bind(auid)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
