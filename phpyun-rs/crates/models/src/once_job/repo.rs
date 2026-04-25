use super::entity::OnceJob;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, companyname, linkman, linktel, provinceid, cityid, three_cityid,
    number, `type`, salary, exp, edu, `require`, pic, yyzz, password, login_ip,
    status, ctime, edate, did, hits";

#[derive(Debug, Default, Clone)]
pub struct Filter<'a> {
    pub keyword: Option<&'a str>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub did: u32,
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<OnceJob>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_once_job WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, OnceJob>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &Filter<'_>,
    offset: u64,
    limit: u64,
    now: i64,
) -> Result<Vec<OnceJob>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(
        " FROM phpyun_once_job WHERE status = 1 AND (edate = 0 OR edate > ",
    );
    qb.push_bind(now);
    qb.push(") AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(" ORDER BY ctime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<OnceJob>().fetch_all(pool).await
}

pub async fn count_public(pool: &MySqlPool, f: &Filter<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_once_job WHERE status = 1 AND (edate = 0 OR edate > ",
    );
    qb.push_bind(now);
    qb.push(") AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &Filter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND (companyname LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR `require` LIKE ");
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
    if let Some(v) = f.edu {
        qb.push(" AND edu = ");
        qb.push_bind(v);
    }
}

pub async fn count_today_by_ip(
    pool: &MySqlPool,
    ip: &str,
    since_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_once_job WHERE login_ip = ? AND ctime > ?",
    )
    .bind(ip)
    .bind(since_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_today_total(pool: &MySqlPool, since_ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job WHERE ctime > ?")
            .bind(since_ts)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub struct Create<'a> {
    pub companyname: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub number: i32,
    pub job_type: i32,
    pub salary: i32,
    pub exp: i32,
    pub edu: i32,
    pub require: &'a str,
    pub pic: &'a str,
    pub yyzz: &'a str,
    pub password_md5: &'a str,
    pub login_ip: &'a str,
    pub status: i32,
    pub edate: i64,
    pub did: u32,
    pub now: i64,
}

pub async fn create(pool: &MySqlPool, c: &Create<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_once_job
           (companyname, linkman, linktel, provinceid, cityid, three_cityid, number,
            `type`, salary, exp, edu, `require`, pic, yyzz, password, login_ip,
            status, ctime, edate, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(c.companyname)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.provinceid)
    .bind(c.cityid)
    .bind(c.three_cityid)
    .bind(c.number)
    .bind(c.job_type)
    .bind(c.salary)
    .bind(c.exp)
    .bind(c.edu)
    .bind(c.require)
    .bind(c.pic)
    .bind(c.yyzz)
    .bind(c.password_md5)
    .bind(c.login_ip)
    .bind(c.status)
    .bind(c.now)
    .bind(c.edate)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct Update<'a> {
    pub companyname: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub number: i32,
    pub job_type: i32,
    pub salary: i32,
    pub exp: i32,
    pub edu: i32,
    pub require: &'a str,
}

pub async fn update_with_password_check(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
    u: &Update<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_once_job SET
           companyname = ?, linkman = ?, linktel = ?, provinceid = ?, cityid = ?,
           three_cityid = ?, number = ?, `type` = ?, salary = ?, exp = ?, edu = ?,
           `require` = ?
         WHERE id = ? AND password = ?",
    )
    .bind(u.companyname)
    .bind(u.linkman)
    .bind(u.linktel)
    .bind(u.provinceid)
    .bind(u.cityid)
    .bind(u.three_cityid)
    .bind(u.number)
    .bind(u.job_type)
    .bind(u.salary)
    .bind(u.exp)
    .bind(u.edu)
    .bind(u.require)
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
        "SELECT COUNT(*) FROM phpyun_once_job WHERE id = ? AND password = ?",
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
        "UPDATE phpyun_once_job SET ctime = ? WHERE id = ? AND password = ?",
    )
    .bind(now)
    .bind(id)
    .bind(password_md5)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: status=2 means deleted. The password constraint also
/// serves as the authentication check.
pub async fn delete_with_password(
    pool: &MySqlPool,
    id: u64,
    password_md5: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_once_job SET status = 2 WHERE id = ? AND password = ?",
    )
    .bind(id)
    .bind(password_md5)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn incr_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_once_job SET hits = hits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
