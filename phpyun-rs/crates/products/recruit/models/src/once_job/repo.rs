use super::entity::OnceJob;
use sqlx::{MySqlPool, QueryBuilder};

// Real `phpyun_once_job` columns (PHP):
//   id, title, mans, require, companyname, phone, hits, linkman,
//   provinceid, cityid, three_cityid, address, ctime, status,
//   password, qq, email, edate, login_ip, did, sxtime, sxnumber,
//   pic, salary (varchar), pay, yyzz
//
// Public entity aliases:
//   linktel ← phone; number ← CAST(mans); type/exp/edu ← 0 (no columns).
const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    companyname, \
    linkman, \
    COALESCE(phone, '') AS linktel, \
    provinceid, cityid, three_cityid, \
    COALESCE(address, '') AS address, \
    COALESCE(mans, '') AS mans, \
    CAST(COALESCE(NULLIF(mans, ''), '0') AS SIGNED) AS number, \
    CAST(0 AS SIGNED) AS `type`, \
    CAST(COALESCE(NULLIF(salary, ''), '0') AS SIGNED) AS salary, \
    COALESCE(salary, '') AS salary_text, \
    CAST(0 AS SIGNED) AS exp, \
    CAST(0 AS SIGNED) AS edu, \
    `require`, pic, yyzz, password, login_ip, \
    status, \
    CAST(COALESCE(pay, 0) AS SIGNED) AS pay, \
    ctime, \
    COALESCE(edate, 0) AS edate, \
    COALESCE(did, 0) AS did, \
    COALESCE(hits, 0) AS hits";

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
    qb.push(" FROM phpyun_once_job WHERE status = 1 AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(") AND (");
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(") ");
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
    qb.push(") AND (");
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(") ");
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &Filter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND (title LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR companyname LIKE ");
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
    // PHP table has no exp/edu columns; keyword/city filters only.
    let _ = (f.exp, f.edu);
}

pub async fn count_today_by_ip(
    pool: &MySqlPool,
    ip: &str,
    since_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job WHERE login_ip = ? AND ctime > ?")
            .bind(ip)
            .bind(since_ts)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_today_total(pool: &MySqlPool, since_ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job WHERE ctime > ?")
        .bind(since_ts)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct Create<'a> {
    pub title: &'a str,
    pub companyname: &'a str,
    pub linkman: &'a str,
    pub phone: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: &'a str,
    pub mans: &'a str,
    pub salary: &'a str,
    pub require: &'a str,
    pub pic: &'a str,
    pub yyzz: &'a str,
    pub password_md5: &'a str,
    pub login_ip: &'a str,
    pub status: i32,
    pub pay: i32,
    pub edate: i64,
    pub did: u32,
    pub now: i64,
}

pub async fn create(pool: &MySqlPool, c: &Create<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_once_job
           (title, companyname, linkman, phone, provinceid, cityid, three_cityid,
            address, mans, salary, `require`, pic, yyzz, password, login_ip,
            status, pay, ctime, edate, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(c.title)
    .bind(c.companyname)
    .bind(c.linkman)
    .bind(c.phone)
    .bind(c.provinceid)
    .bind(c.cityid)
    .bind(c.three_cityid)
    .bind(c.address)
    .bind(c.mans)
    .bind(c.salary)
    .bind(c.require)
    .bind(c.pic)
    .bind(c.yyzz)
    .bind(c.password_md5)
    .bind(c.login_ip)
    .bind(c.status)
    .bind(c.pay)
    .bind(c.now)
    .bind(c.edate)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct Update<'a> {
    pub title: &'a str,
    pub companyname: &'a str,
    pub linkman: &'a str,
    pub phone: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: &'a str,
    pub mans: &'a str,
    pub salary: &'a str,
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
           title = ?, companyname = ?, linkman = ?, phone = ?, provinceid = ?, cityid = ?,
           three_cityid = ?, address = ?, mans = ?, salary = ?, `require` = ?
         WHERE id = ? AND password = ?",
    )
    .bind(u.title)
    .bind(u.companyname)
    .bind(u.linkman)
    .bind(u.phone)
    .bind(u.provinceid)
    .bind(u.cityid)
    .bind(u.three_cityid)
    .bind(u.address)
    .bind(u.mans)
    .bind(u.salary)
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
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job WHERE id = ? AND password = ?")
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
    let res = sqlx::query("UPDATE phpyun_once_job SET ctime = ? WHERE id = ? AND password = ?")
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
    let res = sqlx::query("UPDATE phpyun_once_job SET status = 2 WHERE id = ? AND password = ?")
        .bind(id)
        .bind(password_md5)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn admin_list(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<OnceJob>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_once_job WHERE 1=1");
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY ctime DESC, id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<OnceJob>().fetch_all(pool).await
}

pub async fn admin_count(pool: &MySqlPool, status: Option<i32>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_once_job WHERE 1=1");
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_set_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_once_job SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
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

pub async fn insert_once_order(pool: &MySqlPool, o: OrderInsert<'_>) -> Result<u64, sqlx::Error> {
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

pub async fn find_order_by_order_id(
    pool: &MySqlPool,
    order_id: &str,
) -> Result<Option<OnceOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_FIELDS} FROM phpyun_company_order \
         WHERE order_id = ? AND type = ? LIMIT 1"
    );
    sqlx::query_as::<_, OnceOrder>(&sql)
        .bind(order_id)
        .bind(ONCE_ORDER_TYPE)
        .fetch_optional(pool)
        .await
}

pub async fn mark_order_paid(pool: &MySqlPool, order_id: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_order SET order_state = 2 \
         WHERE order_id = ? AND type = ? AND order_state = 1",
    )
    .bind(order_id)
    .bind(ONCE_ORDER_TYPE)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
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

pub async fn list_pending_once_orders_by_fast(
    pool: &MySqlPool,
    fast: &str,
    offset: u64,
    limit: u64,
) -> Result<Vec<OnceOrder>, sqlx::Error> {
    let sql = format!(
        "SELECT {ORDER_FIELDS} FROM phpyun_company_order \
         WHERE fast = ? AND type = ? AND order_state = 1 \
         ORDER BY order_time DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, OnceOrder>(&sql)
        .bind(fast)
        .bind(ONCE_ORDER_TYPE)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_pending_once_orders_by_fast(
    pool: &MySqlPool,
    fast: &str,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_order \
         WHERE fast = ? AND type = ? AND order_state = 1",
    )
    .bind(fast)
    .bind(ONCE_ORDER_TYPE)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn cancel_pending_once_order_by_fast(
    pool: &MySqlPool,
    fast: &str,
    id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_order \
            SET order_state = 3 \
          WHERE id = ? AND fast = ? AND type = ? AND order_state = 1",
    )
    .bind(id)
    .bind(fast)
    .bind(ONCE_ORDER_TYPE)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn count_pending_once_orders(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_order \
         WHERE uid = ? AND type = ? AND order_state = 1",
    )
    .bind(uid)
    .bind(ONCE_ORDER_TYPE)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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

// ---------- admin php-content ----------

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct PriceGearRow {
    pub id: u64,
    pub days: i32,
    pub price: f64,
}

pub async fn list_price_gears(pool: &MySqlPool) -> Result<Vec<PriceGearRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(days, 0) AS SIGNED) AS days, \
         (COALESCE(price, 0) + 0e0) AS price \
         FROM phpyun_once_price_gear ORDER BY days ASC, id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_price_gear_by_days(
    pool: &MySqlPool,
    days: i32,
    except_id: u64,
) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_once_price_gear \
         WHERE days = ? AND id <> ? LIMIT 1",
    )
    .bind(days)
    .bind(except_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn insert_price_gear(pool: &MySqlPool, days: i32, price: f64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO phpyun_once_price_gear (days, price) VALUES (?, ?)")
        .bind(days)
        .bind(price)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn update_price_gear(
    pool: &MySqlPool,
    id: u64,
    days: Option<i32>,
    price: Option<f64>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_once_price_gear SET \
            days = COALESCE(?, days), \
            price = COALESCE(?, price) \
         WHERE id = ?",
    )
    .bind(days)
    .bind(price)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_price_gears(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_once_price_gear WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminOnceRow {
    pub id: u64,
    pub title: String,
    pub companyname: String,
    pub linkman: String,
    pub phone: String,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: String,
    pub require: String,
    pub salary: String,
    pub password: String,
    pub status: i32,
    pub ctime: i64,
    pub edate: i64,
    pub did: i32,
    pub pic: String,
    pub yyzz: String,
    pub hits: i64,
    pub pay: i32,
}

const ADMIN_ONCE_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(companyname, '') AS companyname, \
    COALESCE(linkman, '') AS linkman, \
    COALESCE(phone, '') AS phone, \
    CAST(COALESCE(provinceid, 0) AS SIGNED) AS provinceid, \
    CAST(COALESCE(cityid, 0) AS SIGNED) AS cityid, \
    CAST(COALESCE(three_cityid, 0) AS SIGNED) AS three_cityid, \
    COALESCE(address, '') AS address, \
    COALESCE(`require`, '') AS `require`, \
    COALESCE(salary, '') AS salary, \
    COALESCE(password, '') AS password, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    CAST(COALESCE(edate, 0) AS SIGNED) AS edate, \
    CAST(COALESCE(did, 0) AS SIGNED) AS did, \
    COALESCE(pic, '') AS pic, \
    COALESCE(yyzz, '') AS yyzz, \
    CAST(COALESCE(hits, 0) AS SIGNED) AS hits, \
    CAST(COALESCE(pay, 0) AS SIGNED) AS pay";

pub async fn find_admin(pool: &MySqlPool, id: u64) -> Result<Option<AdminOnceRow>, sqlx::Error> {
    let sql = format!("SELECT {ADMIN_ONCE_FIELDS} FROM phpyun_once_job WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, AdminOnceRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct AdminOnceSave<'a> {
    pub title: &'a str,
    pub companyname: &'a str,
    pub linkman: &'a str,
    pub phone: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: &'a str,
    pub require: &'a str,
    pub salary: &'a str,
    pub password_md5: Option<&'a str>,
    pub edate: i64,
    pub did: i32,
    pub now: i64,
}

pub async fn admin_save(
    pool: &MySqlPool,
    id: u64,
    s: &AdminOnceSave<'_>,
) -> Result<u64, sqlx::Error> {
    if id > 0 {
        sqlx::query(
            "UPDATE phpyun_once_job SET title=?, companyname=?, linkman=?, phone=?, \
             provinceid=?, cityid=?, three_cityid=?, address=?, `require`=?, salary=?, \
             password=COALESCE(?, password), edate=?, status=1, did=? WHERE id=?",
        )
        .bind(s.title)
        .bind(s.companyname)
        .bind(s.linkman)
        .bind(s.phone)
        .bind(s.provinceid)
        .bind(s.cityid)
        .bind(s.three_cityid)
        .bind(s.address)
        .bind(s.require)
        .bind(s.salary)
        .bind(s.password_md5)
        .bind(s.edate)
        .bind(s.did)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_once_job \
             (title, companyname, linkman, phone, provinceid, cityid, three_cityid, address, \
              `require`, salary, password, status, did, ctime, edate) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?)",
        )
        .bind(s.title)
        .bind(s.companyname)
        .bind(s.linkman)
        .bind(s.phone)
        .bind(s.provinceid)
        .bind(s.cityid)
        .bind(s.three_cityid)
        .bind(s.address)
        .bind(s.require)
        .bind(s.salary)
        .bind(s.password_md5.unwrap_or(""))
        .bind(s.did)
        .bind(s.now)
        .bind(s.edate)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_once_job WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn extend_edate(pool: &MySqlPool, ids: &[u64], days: i32, now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() || days <= 0 {
        return Ok(0);
    }
    let add = i64::from(days) * 86_400;
    let mut n = 0u64;
    for id in ids {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT CAST(COALESCE(edate, 0) AS SIGNED) FROM phpyun_once_job WHERE id = ?")
                .bind(*id)
                .fetch_optional(pool)
                .await?;
        let Some((edate,)) = row else {
            continue;
        };
        let next = if edate < now { now + add } else { edate + add };
        let res = sqlx::query("UPDATE phpyun_once_job SET edate = ? WHERE id = ?")
            .bind(next)
            .bind(*id)
            .execute(pool)
            .await?;
        n += res.rows_affected();
    }
    Ok(n)
}

pub async fn refresh_ctime(pool: &MySqlPool, ids: &[u64], now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_once_job SET ctime = ");
    qb.push_bind(now);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

/// PHP `weipin_once::index_action` filters (keyword type / UI status / ctime).
#[derive(Debug, Default, Clone)]
pub struct AdminOncePhpFilter<'a> {
    pub keyword: Option<&'a str>,
    pub keyword_type: i32,
    pub list_status: Option<i32>,
    pub ctime_min: Option<i64>,
    pub now: i64,
}

fn push_admin_php_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &AdminOncePhpFilter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            let col = match f.keyword_type {
                3 => "phone",
                4 => "linkman",
                5 => "companyname",
                _ => "title",
            };
            qb.push(" AND ");
            qb.push(col);
            qb.push(" LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    match f.list_status {
        Some(1) => {
            qb.push(" AND status = 1 AND edate > ");
            qb.push_bind(f.now);
        }
        Some(3) => {
            qb.push(" AND status = 0 AND edate > ");
            qb.push_bind(f.now);
        }
        Some(2) => {
            qb.push(" AND edate < ");
            qb.push_bind(f.now);
        }
        _ => {}
    }
    if let Some(min) = f.ctime_min {
        qb.push(" AND ctime >= ");
        qb.push_bind(min);
    }
}

fn push_once_php_order(qb: &mut QueryBuilder<'_, sqlx::MySql>, col: &str, dir: &str) {
    let dir = if dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };
    let col = match col {
        "id" | "ctime" | "edate" | "status" | "title" | "phone" | "companyname" | "linkman"
        | "hits" => col,
        _ => "",
    };
    if col.is_empty() {
        qb.push(" ORDER BY ctime DESC, id DESC");
    } else {
        qb.push(" ORDER BY ");
        qb.push(col);
        qb.push(" ");
        qb.push(dir);
        qb.push(", id ");
        qb.push(dir);
    }
}

pub async fn admin_php_list(
    pool: &MySqlPool,
    f: &AdminOncePhpFilter<'_>,
    offset: u64,
    limit: u64,
    order_col: &str,
    order_dir: &str,
) -> Result<Vec<AdminOnceRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(ADMIN_ONCE_FIELDS);
    qb.push(" FROM phpyun_once_job WHERE 1=1");
    push_admin_php_filters(&mut qb, f);
    push_once_php_order(&mut qb, order_col, order_dir);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<AdminOnceRow>().fetch_all(pool).await
}

pub async fn admin_php_count(pool: &MySqlPool, f: &AdminOncePhpFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_once_job WHERE 1=1");
    push_admin_php_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_all(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_pending_unexpired(pool: &MySqlPool, now: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_once_job WHERE status = 0 AND edate > ?",
    )
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_expired(pool: &MySqlPool, now: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_once_job WHERE edate < ?")
        .bind(now)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_pay_eq(pool: &MySqlPool, ids: &[u64], pay: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_once_job WHERE pay = ");
    qb.push_bind(pay);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_set_status_ids(
    pool: &MySqlPool,
    ids: &[u64],
    status: i32,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_once_job SET status = ");
    qb.push_bind(status);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_did_ids(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_once_job SET did = ");
    qb.push_bind(did);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
