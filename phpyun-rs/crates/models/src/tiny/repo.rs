//! General worker resume repo.
//!
//! Table: `phpyun_resume_tiny`. All dynamic WHERE clauses go through `QueryBuilder::push_bind`.

use super::entity::TinyResume;
use sqlx::{MySqlPool, QueryBuilder};

// `phpyun_resume_tiny` declares lastupdate / did / hits as nullable int;
// `TinyResume` reads them as plain `i64 / u32 / i64`. COALESCE so a NULL row
// can't surprise sqlx.
const FIELDS: &str = "id, username, sex, exp, job, mobile, password, provinceid, cityid, three_cityid, \
    production, status, login_ip, time, \
    COALESCE(lastupdate, 0) AS lastupdate, \
    COALESCE(did, 0) AS did, \
    COALESCE(hits, 0) AS hits";

#[derive(Debug, Default, Clone)]
pub struct TinyFilter<'a> {
    pub keyword: Option<&'a str>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub sex: Option<i32>,
    pub did: u32,
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<TinyResume>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_tiny WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, TinyResume>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &TinyFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<TinyResume>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_resume_tiny WHERE status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<TinyResume>().fetch_all(pool).await
}

pub async fn count_public(pool: &MySqlPool, f: &TinyFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_resume_tiny WHERE status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &TinyFilter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND (job LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR production LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
        }
    }
    if let Some(v) = f.province_id {
        qb.push(" AND provinceid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.city_id {
        qb.push(" AND cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.three_city_id {
        qb.push(" AND three_cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.sex {
        qb.push(" AND sex = ");
        qb.push_bind(v);
    }
}

pub async fn count_today_by_ip(
    pool: &MySqlPool,
    ip: &str,
    since_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resume_tiny WHERE login_ip = ? AND time > ?",
    )
    .bind(ip)
    .bind(since_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_today_total(pool: &MySqlPool, since_ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume_tiny WHERE time > ?")
        .bind(since_ts)
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
}

pub struct CreateTiny<'a> {
    pub username: &'a str,
    pub sex: i32,
    pub exp: i32,
    pub job: &'a str,
    pub mobile: &'a str,
    pub password_md5: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub production: &'a str,
    pub status: i32,
    pub login_ip: &'a str,
    pub now: i64,
    pub did: u32,
}

pub async fn create(pool: &MySqlPool, c: &CreateTiny<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_resume_tiny
           (username, sex, exp, job, mobile, password, provinceid, cityid, three_cityid,
            production, status, login_ip, time, lastupdate, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(c.username)
    .bind(c.sex)
    .bind(c.exp)
    .bind(c.job)
    .bind(c.mobile)
    .bind(c.password_md5)
    .bind(c.provinceid)
    .bind(c.cityid)
    .bind(c.three_cityid)
    .bind(c.production)
    .bind(c.status)
    .bind(c.login_ip)
    .bind(c.now)
    .bind(c.now)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct UpdateTiny<'a> {
    pub username: &'a str,
    pub sex: i32,
    pub exp: i32,
    pub job: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub production: &'a str,
    pub status: i32,
    pub now: i64,
}

pub async fn update_with_password_check(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
    u: &UpdateTiny<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_tiny SET
           username = ?, sex = ?, exp = ?, job = ?, provinceid = ?, cityid = ?, three_cityid = ?,
           production = ?, status = ?, lastupdate = ?
         WHERE id = ? AND password = ?",
    )
    .bind(u.username)
    .bind(u.sex)
    .bind(u.exp)
    .bind(u.job)
    .bind(u.provinceid)
    .bind(u.cityid)
    .bind(u.three_cityid)
    .bind(u.production)
    .bind(u.status)
    .bind(u.now)
    .bind(id)
    .bind(password_md5)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn verify_password(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resume_tiny WHERE id = ? AND password = ?",
    )
    .bind(id)
    .bind(password_md5)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

pub async fn refresh_with_password(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_tiny SET lastupdate = ? WHERE id = ? AND password = ?",
    )
    .bind(now)
    .bind(id)
    .bind(password_md5)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: `status = 2` indicates deleted. The password constraint also serves as authentication.
pub async fn delete_with_password(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_tiny SET status = 2 WHERE id = ? AND password = ?",
    )
    .bind(id)
    .bind(password_md5)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn incr_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_resume_tiny SET hits = hits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
