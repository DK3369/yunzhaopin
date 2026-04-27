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

// ==================== Pay flow (phpyun_company_order type=25) ====================

const ONCE_ORDER_TYPE: i32 = 25;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct OnceOrder {
    pub id: u64,
    pub order_id: String,
    pub order_type: String,
    pub order_price: f64,
    pub order_time: i64,
    pub order_state: i32,
    pub order_remark: String,
    pub did: Option<i32>,
    pub once_id: Option<i32>,
    pub fast: Option<String>,
}

const ORDER_FIELDS: &str = "id, order_id, order_type, order_price, order_time, order_state, \
                            order_remark, did, once_id, fast";

/// Look up a price gear by id; returns `(days, price_yuan)`.
pub async fn find_price_gear(
    pool: &MySqlPool,
    gear_id: i32,
) -> Result<Option<(i32, f64)>, sqlx::Error> {
    let row: Option<(i32, f64)> = sqlx::query_as(
        "SELECT CAST(COALESCE(days, 0) AS SIGNED), CAST(COALESCE(price, 0) AS DECIMAL(18,2)) \
         FROM phpyun_once_price_gear WHERE id = ? LIMIT 1",
    )
    .bind(gear_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Get the once_job's current pay state — `0` means unpaid, `2` means paid.
pub async fn get_pay_state(pool: &MySqlPool, id: u64) -> Result<Option<i32>, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(pay, 0) AS SIGNED) FROM phpyun_once_job WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(p,)| p))
}

/// Mark a once_job as paid (called after a successful order, or immediately
/// for free gears). PHP `payOnce` writes `pay = 2` here.
pub async fn mark_once_paid(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_once_job SET pay = 2 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub struct OrderInsert<'a> {
    pub uid: u64,
    pub order_id: &'a str,
    pub pay_type: &'a str,
    pub price: f64,
    pub now: i64,
    /// 1 = pending, 2 = paid (used when the gear price is 0).
    pub state: i32,
    pub did: i32,
    pub once_id: u64,
    pub fast: &'a str,
}

pub async fn insert_once_order(
    pool: &MySqlPool,
    o: OrderInsert<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_order \
            (uid, order_id, order_type, order_price, order_time, order_state, \
             order_remark, type, once_id, fast, did, port, usertype, status) \
         VALUES (?, ?, ?, ?, ?, ?, '店铺招聘收费', ?, ?, ?, ?, 2, 2, 2)",
    )
    .bind(o.uid)
    .bind(o.order_id)
    .bind(o.pay_type)
    .bind(o.price)
    .bind(o.now)
    .bind(o.state)
    .bind(ONCE_ORDER_TYPE)
    .bind(o.once_id)
    .bind(o.fast)
    .bind(o.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Drop any pre-existing pending orders for the same once_job — matches PHP
/// `payOnce` which clears stale orders before creating a new one.
pub async fn delete_pending_orders_for_once(
    pool: &MySqlPool,
    once_id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_company_order \
         WHERE once_id = ? AND type = ? AND order_state = 1",
    )
    .bind(once_id)
    .bind(ONCE_ORDER_TYPE)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// List the user's pending one-shot orders (type=25, order_state=1).
pub async fn list_pending_once_orders(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<OnceOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_FIELDS} FROM phpyun_company_order \
         WHERE uid = ? AND type = ? AND order_state = 1 \
         ORDER BY order_time DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, OnceOrder>(&sql)
        .bind(uid)
        .bind(ONCE_ORDER_TYPE)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_pending_once_orders(
    pool: &MySqlPool,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_order \
         WHERE uid = ? AND type = ? AND order_state = 1",
    )
    .bind(uid)
    .bind(ONCE_ORDER_TYPE)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Cancel a pending order (mark `order_state=3`, matching PHP `del`).
pub async fn cancel_pending_once_order(
    pool: &MySqlPool,
    uid: u64,
    id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_order \
            SET order_state = 3 \
          WHERE id = ? AND uid = ? AND type = ? AND order_state = 1",
    )
    .bind(id)
    .bind(uid)
    .bind(ONCE_ORDER_TYPE)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
