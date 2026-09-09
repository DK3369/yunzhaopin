//! PHP `tongji.model.php` time-series / top-ten / 分站对比 SQL.
//! Table and column names are allow-listed before interpolation.

use std::collections::HashMap;

use chrono::{Duration, TimeZone};
use phpyun_core::clock;
use sqlx::{FromRow, MySqlPool, QueryBuilder, Row};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TjFmt {
    Hour,
    Md,
    Month,
}

#[derive(Debug, Clone)]
pub struct TjBucket {
    pub onday: String,
    pub tjtime: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct TjWindow {
    pub sdate: i64,
    pub edate: i64,
    pub fmt: TjFmt,
    pub month_suffix: bool,
    pub buckets: Vec<TjBucket>,
}

#[derive(Debug, Clone)]
pub enum Extra {
    Eq(&'static str, i32),
    In(&'static str, Vec<i32>),
}

#[derive(Debug, Clone)]
pub struct TjPoint {
    pub tjtime: String,
    pub date: String,
    pub count: f64,
}

#[derive(Debug, Clone)]
pub struct TjSeries {
    pub allnum: f64,
    pub list: Vec<TjPoint>,
}

#[derive(Debug, Clone)]
pub struct TopPair {
    pub id: u64,
    pub count: f64,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopJobRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopComRow {
    pub uid: u64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopExpectRow {
    pub id: u64,
    pub uid: u64,
    pub uname: String,
    pub job_classid: String,
    pub city_classid: String,
    pub exp: i32,
    pub edu: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopResumeRow {
    pub uid: u64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopOrderRow {
    pub uid: u64,
    pub username: String,
    pub usertype: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct TopAdRow {
    pub id: u64,
    pub ad_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct JobSlice {
    pub edu: i32,
    pub exp: i32,
    pub job1: i32,
    pub provinceid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ExpectSlice {
    pub id: u64,
    pub sex: i32,
    pub edu: i32,
    pub exp: i32,
    pub job_classid: String,
    pub maxsalary: i32,
    pub minsalary: i32,
    pub source: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ExpectCity {
    pub eid: u64,
    pub provinceid: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderSlice {
    pub order_type: String,
    #[sqlx(rename = "type")]
    pub kind: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ComSlice {
    pub uid: u64,
    pub hy: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ComRating {
    pub uid: u64,
    pub rating: i32,
    pub rating_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct SourceCount {
    pub source: i32,
    pub count: i64,
}

fn table_sql(name: &str) -> Option<&'static str> {
    Some(match name {
        "member" => "phpyun_member",
        "resume_expect" => "phpyun_resume_expect",
        "userid_job" => "phpyun_userid_job",
        "resume_refresh_log" => "phpyun_resume_refresh_log",
        "company_job" => "phpyun_company_job",
        "down_resume" => "phpyun_down_resume",
        "job_refresh_log" => "phpyun_job_refresh_log",
        "userid_msg" => "phpyun_userid_msg",
        "login_log" => "phpyun_login_log",
        "look_job" => "phpyun_look_job",
        "look_resume" => "phpyun_look_resume",
        "freedown_resume" => "phpyun_freedown_resume",
        "company_order" => "phpyun_company_order",
        "adclick" => "phpyun_adclick",
        "chat_log" => "phpyun_chat_log",
        "ad" => "phpyun_ad",
        "company" => "phpyun_company",
        "company_statis" => "phpyun_company_statis",
        "resume" => "phpyun_resume",
        _ => return None,
    })
}

fn field_sql(name: &str) -> Option<&'static str> {
    Some(match name {
        "reg_date" => "reg_date",
        "ctime" => "ctime",
        "datetime" => "datetime",
        "r_time" => "r_time",
        "sdate" => "sdate",
        "downtime" => "downtime",
        "sendTime" => "sendTime",
        "order_time" => "order_time",
        "addtime" => "addtime",
        "usertype" => "usertype",
        "status" => "status",
        "order_state" => "order_state",
        "type" => "type",
        "r_status" => "r_status",
        "jobid" => "jobid",
        "com_id" => "com_id",
        "resume_id" => "resume_id",
        "comid" => "comid",
        "uid" => "uid",
        "eid" => "eid",
        "fid" => "fid",
        "aid" => "aid",
        "id" => "id",
        _ => return None,
    })
}

fn expr(field: &str) -> Option<String> {
    let col = field_sql(field)?;
    if col == "sendTime" {
        Some(format!("(`{col}`/1000)"))
    } else {
        Some(format!("`{col}`"))
    }
}

fn fmt_sql(fmt: TjFmt) -> &'static str {
    match fmt {
        TjFmt::Hour => "%k",
        TjFmt::Md => "%m%d",
        TjFmt::Month => "%m",
    }
}

fn dt(ts: i64) -> chrono::DateTime<chrono::FixedOffset> {
    clock::tz()
        .timestamp_opt(ts.max(0), 0)
        .single()
        .or_else(|| clock::tz().timestamp_opt(clock::now_ts(), 0).single())
        .unwrap_or_else(|| {
            chrono::DateTime::from_timestamp(0, 0)
                .unwrap_or_else(|| chrono::DateTime::from_timestamp(1, 0).expect("epoch"))
                .with_timezone(&clock::tz())
        })
}

fn ms_to_sec(v: i64) -> i64 {
    if v > 10_000_000_000 {
        v / 1000
    } else {
        v
    }
}

/// PHP `TimeDate`: `days` -1/1/2/3/4/5 or `time:[ms,ms]`.
pub fn tj_window(days: i32, time: Option<(i64, i64)>, now: i64) -> TjWindow {
    if let Some((a, b)) = time {
        if a > 0 && b > 0 {
            let s = clock::start_of_day(ms_to_sec(a));
            let e = clock::start_of_day(ms_to_sec(b)) + 86_399;
            let n = ((e - s).max(0) / 86_400).max(1);
            let mut buckets = Vec::new();
            for i in 0..n {
                let t = e - i * 86_400;
                let d = dt(t);
                buckets.push(TjBucket {
                    onday: d.format("%m%d").to_string(),
                    tjtime: d.format("%m-%d").to_string(),
                    date: d.format("%m-%d").to_string(),
                });
            }
            return TjWindow {
                sdate: s,
                edate: e,
                fmt: TjFmt::Md,
                month_suffix: false,
                buckets,
            };
        }
    }
    let day = if days == 0 { 1 } else { days };
    if day == -1 {
        let today = clock::start_of_today();
        let sdate = today - 86_400;
        let buckets: Vec<TjBucket> = (0..=24)
            .map(|i| TjBucket {
                onday: i.to_string(),
                tjtime: i.to_string(),
                date: format!("{i}:00"),
            })
            .collect();
        return TjWindow {
            sdate,
            edate: today,
            fmt: TjFmt::Hour,
            month_suffix: false,
            buckets,
        };
    }
    if day == 1 {
        let buckets: Vec<TjBucket> = (0..=24)
            .map(|i| TjBucket {
                onday: i.to_string(),
                tjtime: i.to_string(),
                date: format!("{i}:00"),
            })
            .collect();
        return TjWindow {
            sdate: clock::start_of_today(),
            edate: now,
            fmt: TjFmt::Hour,
            month_suffix: false,
            buckets,
        };
    }
    if day > 1 && day < 4 {
        let span = if day == 2 { 7 } else { 30 };
        let sdate = now - i64::from(span) * 86_400;
        let mut buckets = Vec::new();
        for i in 0..=span {
            let t = now - i64::from(i) * 86_400;
            let d = dt(t);
            buckets.push(TjBucket {
                onday: d.format("%m%d").to_string(),
                tjtime: d.format("%m-%d").to_string(),
                date: d.format("%m-%d").to_string(),
            });
        }
        return TjWindow {
            sdate,
            edate: now,
            fmt: TjFmt::Md,
            month_suffix: false,
            buckets,
        };
    }
    if day > 3 && day < 6 {
        let months = if day == 4 { 6 } else { 12 };
        let sdate = dt(now)
            .checked_sub_signed(Duration::days(i64::from(months) * 30))
            .map(|d| d.timestamp())
            .unwrap_or(now - i64::from(months) * 30 * 86_400);
        let mut buckets = Vec::new();
        let now_y = dt(now).format("%y").to_string();
        for i in 0..=months {
            let t = dt(now)
                .checked_sub_signed(Duration::days(i64::from(i) * 30))
                .map(|d| d.timestamp())
                .unwrap_or(now);
            let d = dt(t);
            let y = d.format("%y").to_string();
            let m = d.format("%m").to_string();
            let mut date = m.clone();
            if y != now_y {
                date = m.clone();
            }
            buckets.push(TjBucket {
                onday: m.clone(),
                tjtime: m.clone(),
                date,
            });
        }
        return TjWindow {
            sdate,
            edate: now,
            fmt: TjFmt::Month,
            month_suffix: true,
            buckets,
        };
    }
    tj_window(1, None, now)
}

fn push_extra(qb: &mut QueryBuilder<'_, sqlx::MySql>, extra: &[Extra]) {
    for e in extra {
        match e {
            Extra::Eq(col, val) => {
                let Some(c) = field_sql(col) else { continue };
                qb.push(format!(" AND `{c}` = "));
                qb.push_bind(*val);
            }
            Extra::In(col, vals) => {
                let Some(c) = field_sql(col) else { continue };
                if vals.is_empty() {
                    qb.push(" AND 1=0");
                    continue;
                }
                qb.push(format!(" AND `{c}` IN ("));
                let mut sep = qb.separated(", ");
                for v in vals {
                    sep.push_bind(*v);
                }
                qb.push(")");
            }
        }
    }
}

pub async fn get_tj(
    pool: &MySqlPool,
    table: &str,
    field: &str,
    win: &TjWindow,
    extra: &[Extra],
    sum_price: bool,
) -> Result<TjSeries, sqlx::Error> {
    let Some(tbl) = table_sql(table) else {
        return Ok(empty_series(win));
    };
    let Some(fexp) = expr(field) else {
        return Ok(empty_series(win));
    };
    let fmt = fmt_sql(win.fmt);
    let agg = if sum_price {
        "IFNULL(SUM(COALESCE(`order_price`,0)),0) + 0.0"
    } else {
        "COUNT(*) + 0.0"
    };
    let mut qb = QueryBuilder::new(format!(
        "SELECT TRIM(FROM_UNIXTIME({fexp}, '{fmt}')) AS tjtime, {agg} AS cnt FROM `{tbl}` WHERE {fexp} >= "
    ));
    qb.push_bind(win.sdate);
    qb.push(" AND ");
    qb.push(fexp.clone());
    qb.push(" <= ");
    qb.push_bind(win.edate);
    push_extra(&mut qb, extra);
    qb.push(" GROUP BY tjtime");
    let rows = match qb.build().fetch_all(pool).await {
        Ok(rows) => rows,
        Err(e) if is_missing_table(&e) => return Ok(empty_series(win)),
        Err(e) => return Err(e),
    };
    let mut map: HashMap<String, f64> = HashMap::new();
    let mut allnum = 0.0;
    for row in rows {
        let k: String = row
            .try_get::<String, _>("tjtime")
            .unwrap_or_default()
            .trim()
            .to_string();
        let n = row
            .try_get::<f64, _>("cnt")
            .or_else(|_| row.try_get::<i64, _>("cnt").map(|n| n as f64))
            .unwrap_or(0.0);
        allnum += n;
        map.insert(k.trim().to_string(), n);
    }
    let mut list: Vec<TjPoint> = win
        .buckets
        .iter()
        .map(|b| {
            let count = map.get(&b.onday).copied().unwrap_or(0.0);
            let date = if win.month_suffix {
                format!("{}月", b.date.trim_start_matches('0'))
            } else {
                b.date.clone()
            };
            TjPoint {
                tjtime: b.tjtime.clone(),
                date,
                count,
            }
        })
        .collect();
    list.sort_by(|a, b| b.tjtime.cmp(&a.tjtime));
    Ok(TjSeries { allnum, list })
}

fn is_missing_table(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db) => {
            db.code().as_deref() == Some("42S02") || db.message().contains("doesn't exist")
        }
        _ => false,
    }
}

fn empty_series(win: &TjWindow) -> TjSeries {
    TjSeries {
        allnum: 0.0,
        list: win
            .buckets
            .iter()
            .map(|b| TjPoint {
                tjtime: b.tjtime.clone(),
                date: b.date.clone(),
                count: 0.0,
            })
            .collect(),
    }
}

pub async fn group_top(
    pool: &MySqlPool,
    table: &str,
    time_field: &str,
    group_field: &str,
    win: &TjWindow,
    extra: &[Extra],
    limit: u64,
    sum_price: bool,
) -> Result<Vec<TopPair>, sqlx::Error> {
    let Some(tbl) = table_sql(table) else {
        return Ok(Vec::new());
    };
    let Some(fexp) = expr(time_field) else {
        return Ok(Vec::new());
    };
    let Some(gcol) = field_sql(group_field) else {
        return Ok(Vec::new());
    };
    let agg = if sum_price {
        "IFNULL(SUM(COALESCE(`order_price`,0)),0) + 0.0"
    } else {
        "COUNT(*) + 0.0"
    };
    let cap = limit.clamp(1, 50);
    let mut qb = QueryBuilder::new(format!(
        "SELECT CAST(`{gcol}` AS UNSIGNED) AS id, {agg} AS cnt FROM `{tbl}` WHERE {fexp} >= "
    ));
    qb.push_bind(win.sdate);
    qb.push(" AND ");
    qb.push(fexp);
    qb.push(" <= ");
    qb.push_bind(win.edate);
    push_extra(&mut qb, extra);
    qb.push(format!(" GROUP BY `{gcol}` ORDER BY cnt DESC LIMIT {cap}"));
    let rows = match qb.build().fetch_all(pool).await {
        Ok(rows) => rows,
        Err(e) if is_missing_table(&e) => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    Ok(rows
        .into_iter()
        .map(|r| {
            let id: u64 = r.try_get("id").unwrap_or(0);
            let count = r
                .try_get::<f64, _>("cnt")
                .or_else(|_| r.try_get::<i64, _>("cnt").map(|n| n as f64))
                .unwrap_or(0.0);
            TopPair { id, count }
        })
        .filter(|p| p.id > 0)
        .collect())
}

async fn fetch_in<T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin>(
    pool: &MySqlPool,
    sql_head: &str,
    ids: &[u64],
) -> Result<Vec<T>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for chunk in ids.chunks(400) {
        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(sql_head);
        let mut sep = qb.separated(", ");
        for id in chunk {
            sep.push_bind(*id);
        }
        qb.push(")");
        out.extend(qb.build_query_as().fetch_all(pool).await?);
    }
    Ok(out)
}

pub async fn jobs_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopJobRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, COALESCE(`name`,'') AS `name` \
         FROM phpyun_company_job WHERE id IN (",
        ids,
    )
    .await
}

pub async fn companies_by_uids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopComRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(`name`,'') AS `name` FROM phpyun_company WHERE uid IN (",
        ids,
    )
    .await
}

pub async fn expects_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopExpectRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, COALESCE(uname,'') AS uname, \
         COALESCE(job_classid,'') AS job_classid, COALESCE(city_classid,'') AS city_classid, \
         CAST(COALESCE(exp,0) AS SIGNED) AS exp, CAST(COALESCE(edu,0) AS SIGNED) AS edu \
         FROM phpyun_resume_expect WHERE id IN (",
        ids,
    )
    .await
}

pub async fn resumes_by_uids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopResumeRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(`name`,'') AS `name` FROM phpyun_resume WHERE uid IN (",
        ids,
    )
    .await
}

pub async fn default_eids(pool: &MySqlPool, uids: &[u64]) -> Result<HashMap<u64, u64>, sqlx::Error> {
    let rows: Vec<(u64, u64)> = fetch_in(
        pool,
        "SELECT CAST(uid AS UNSIGNED), CAST(id AS UNSIGNED) FROM phpyun_resume_expect \
         WHERE defaults = 1 AND uid IN (",
        uids,
    )
    .await?;
    Ok(rows.into_iter().collect())
}

pub async fn members_by_uids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopOrderRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype FROM phpyun_member WHERE uid IN (",
        ids,
    )
    .await
}

pub async fn ads_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<TopAdRow>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(ad_name,'') AS ad_name FROM phpyun_ad WHERE id IN (",
        ids,
    )
    .await
}

pub async fn list_ids(
    pool: &MySqlPool,
    table: &str,
    id_field: &str,
    time_field: &str,
    win: &TjWindow,
    extra: &[Extra],
) -> Result<Vec<u64>, sqlx::Error> {
    let Some(tbl) = table_sql(table) else {
        return Ok(Vec::new());
    };
    let Some(fexp) = expr(time_field) else {
        return Ok(Vec::new());
    };
    let Some(icol) = field_sql(id_field) else {
        return Ok(Vec::new());
    };
    let mut qb = QueryBuilder::new(format!(
        "SELECT CAST(`{icol}` AS UNSIGNED) AS id FROM `{tbl}` WHERE {fexp} >= "
    ));
    qb.push_bind(win.sdate);
    qb.push(" AND ");
    qb.push(fexp);
    qb.push(" <= ");
    qb.push_bind(win.edate);
    push_extra(&mut qb, extra);
    qb.push(" LIMIT 20000");
    let rows = match qb.build().fetch_all(pool).await {
        Ok(rows) => rows,
        Err(e) if is_missing_table(&e) => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    Ok(rows
        .into_iter()
        .filter_map(|r| r.try_get::<u64, _>("id").ok())
        .filter(|n| *n > 0)
        .collect())
}

pub async fn job_slices(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<JobSlice>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(COALESCE(edu,0) AS SIGNED) AS edu, CAST(COALESCE(exp,0) AS SIGNED) AS exp, \
         CAST(COALESCE(job1,0) AS SIGNED) AS job1, CAST(COALESCE(provinceid,0) AS SIGNED) AS provinceid, \
         CAST(COALESCE(minsalary,0) AS SIGNED) AS minsalary, CAST(COALESCE(maxsalary,0) AS SIGNED) AS maxsalary \
         FROM phpyun_company_job WHERE id IN (",
        ids,
    )
    .await
}

pub async fn expect_slices(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<ExpectSlice>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(sex,0) AS SIGNED) AS sex, \
         CAST(COALESCE(edu,0) AS SIGNED) AS edu, CAST(COALESCE(exp,0) AS SIGNED) AS exp, \
         COALESCE(job_classid,'') AS job_classid, CAST(COALESCE(maxsalary,0) AS SIGNED) AS maxsalary, \
         CAST(COALESCE(minsalary,0) AS SIGNED) AS minsalary, CAST(COALESCE(source,0) AS SIGNED) AS source \
         FROM phpyun_resume_expect WHERE id IN (",
        ids,
    )
    .await
}

pub async fn expect_cities(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<ExpectCity>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(eid AS UNSIGNED) AS eid, CAST(COALESCE(provinceid,0) AS SIGNED) AS provinceid \
         FROM phpyun_resume_cityclass WHERE eid IN (",
        ids,
    )
    .await
}

pub async fn order_slices(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<OrderSlice>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT COALESCE(order_type,'') AS order_type, CAST(COALESCE(`type`,0) AS SIGNED) AS `type` \
         FROM phpyun_company_order WHERE id IN (",
        ids,
    )
    .await
}

pub async fn com_slices(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<ComSlice>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(uid AS UNSIGNED) AS uid, CAST(COALESCE(hy,0) AS SIGNED) AS hy FROM phpyun_company WHERE uid IN (",
        ids,
    )
    .await
}

pub async fn com_ratings(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<ComRating>, sqlx::Error> {
    fetch_in(
        pool,
        "SELECT CAST(s.uid AS UNSIGNED) AS uid, CAST(COALESCE(s.rating,0) AS SIGNED) AS rating, \
         COALESCE(r.name,'') AS rating_name \
         FROM phpyun_company_statis s LEFT JOIN phpyun_company_rating r ON r.id = s.rating \
         WHERE s.uid IN (",
        ids,
    )
    .await
}

pub async fn source_counts(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<SourceCount>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(COALESCE(source,0) AS SIGNED) AS source, COUNT(*) AS count \
         FROM phpyun_member WHERE uid IN (",
    );
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") GROUP BY source");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn job_parents(pool: &MySqlPool) -> Result<HashMap<i32, i32>, sqlx::Error> {
    let rows: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), CAST(COALESCE(keyid,0) AS SIGNED) FROM phpyun_job_class",
    )
    .fetch_all(pool)
    .await?;
    let map: HashMap<i32, i32> = rows.into_iter().collect();
    let mut root = HashMap::new();
    for &id in map.keys() {
        let mut cur = id;
        let mut guard = 0;
        while guard < 8 {
            guard += 1;
            let p = map.get(&cur).copied().unwrap_or(0);
            if p <= 0 {
                root.insert(id, cur);
                break;
            }
            cur = p;
        }
    }
    Ok(root)
}

/// Monthly counts for 分站对比. Keys match PHP `fenxiabiaoClient` after rename.
pub async fn fenxiabiao_month(
    pool: &MySqlPool,
    sdate: i64,
    edate: i64,
) -> Result<HashMap<String, HashMap<String, i64>>, sqlx::Error> {
    let specs: [(&str, &str, &str, Option<i32>); 11] = [
        ("member", "reg_date", "member", Some(1)),
        ("login_log", "ctime", "login_log", Some(1)),
        ("resume_expect", "ctime", "resume_expect", None),
        ("member", "reg_date", "company", Some(2)),
        ("login_log", "ctime", "company_login_log", Some(2)),
        ("company_job", "sdate", "company_job", None),
        ("userid_job", "datetime", "userid_job", None),
        ("chat_log", "sendTime", "chat_log", None),
        ("userid_msg", "datetime", "userid_msg", None),
        ("down_resume", "downtime", "down_resume", None),
        ("freedown_resume", "downtime", "freedown_resume", None),
    ];
    let mut out = HashMap::new();
    for (table, field, key, usertype) in specs {
        let extra = match usertype {
            Some(u) => vec![Extra::Eq("usertype", u)],
            None => Vec::new(),
        };
        let Some(tbl) = table_sql(table) else { continue };
        let Some(fexp) = expr(field) else { continue };
        let mut qb = QueryBuilder::new(format!(
            "SELECT FROM_UNIXTIME({fexp}, '%Y-%m') AS tjtime, COUNT(*) AS cnt FROM `{tbl}` WHERE {fexp} >= "
        ));
        qb.push_bind(sdate);
        qb.push(" AND ");
        qb.push(fexp);
        qb.push(" <= ");
        qb.push_bind(edate);
        push_extra(&mut qb, &extra);
        qb.push(" GROUP BY tjtime");
        let rows = match qb.build().fetch_all(pool).await {
            Ok(rows) => rows,
            Err(e) if is_missing_table(&e) => continue,
            Err(e) => return Err(e),
        };
        let mut map: HashMap<String, i64> = HashMap::new();
        let mut sum = 0i64;
        for row in rows {
            let k: String = row.try_get::<String, _>("tjtime").unwrap_or_default();
            let n: i64 = row
                .try_get::<i64, _>("cnt")
                .or_else(|_| row.try_get::<f64, _>("cnt").map(|n| n as i64))
                .unwrap_or(0);
            sum += n;
            map.insert(k, n);
        }
        map.insert("sum".into(), sum);
        out.insert(key.to_string(), map);
    }
    Ok(out)
}

fn local_midnight(y: i32, m: u32, d: u32) -> Option<i64> {
    let naive = chrono::NaiveDate::from_ymd_opt(y, m, d)?.and_hms_opt(0, 0, 0)?;
    clock::tz().from_local_datetime(&naive).single().map(|dt| dt.timestamp())
}

/// PHP `setFenxiabiaoData`: type 1 year (`YYYY`), type 2 month (`YYYY-MM`).
pub fn fenxiabiao_range(fx_type: i32, time: &str) -> Option<(i64, i64, i32)> {
    let t = time.trim();
    if t.is_empty() {
        return None;
    }
    if fx_type == 1 {
        let y: i32 = t.parse().ok()?;
        let sdate = local_midnight(y, 1, 1)?;
        let edate = local_midnight(y + 1, 1, 1)?;
        Some((sdate, edate, 12))
    } else {
        let mut p = t.split('-');
        let y: i32 = p.next()?.parse().ok()?;
        let m: u32 = p.next()?.parse().ok()?;
        if !(1..=12).contains(&m) {
            return None;
        }
        let sdate = local_midnight(y, m, 1)?;
        let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        let edate = local_midnight(ny, nm, 1)?;
        Some((sdate, edate, 1))
    }
}

pub fn ym_of(ts: i64) -> String {
    dt(ts).format("%Y-%m").to_string()
}

pub fn add_months(ts: i64, n: i32) -> i64 {
    use chrono::Months;
    let d = dt(ts);
    if n >= 0 {
        d.checked_add_months(Months::new(n as u32))
            .map(|x| x.timestamp())
            .unwrap_or(ts)
    } else {
        d.checked_sub_months(Months::new((-n) as u32))
            .map(|x| x.timestamp())
            .unwrap_or(ts)
    }
}
