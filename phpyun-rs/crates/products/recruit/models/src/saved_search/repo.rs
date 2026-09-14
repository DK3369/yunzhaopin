//! `phpyun_subscribe` repository — saved search alerts.
//!
//! PHP schema (truth): `id, uid, email, job1, job1_son, job_post, provinceid,
//! cityid, three_cityid, salary, type, ctime, status, code, cycle_time, time,
//! minsalary, maxsalary, jobclass_id, stime`. PHPYun's design pre-dates
//! generic JSON columns: each filter parameter has its own physical column
//! and the only feature kind is "job alert" (no `kind` discriminator).
//!
//! Mapping (Rust entity → PHP column):
//! - `name`              → `email`              (PHP uses email as the alert label)
//! - `kind`              → hardcoded `"job"`     (PHP only has job-search alerts)
//! - `params`            → JSON_OBJECT of the PHP per-filter columns
//! - `notify`            → `status`              (1=on / 0=off)
//! - `last_notified_at`  → `stime`
//! - `created_at`        → `ctime`
//! - `updated_at`        → `ctime`               (PHP has no separate updated_at)

use super::entity::SavedSearch;
use serde_json::Value;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                             COALESCE(email, '') AS name, \
                             'job' AS kind, \
                             JSON_OBJECT( \
                                 'job1', COALESCE(job1, 0), \
                                 'job1_son', COALESCE(job1_son, 0), \
                                 'job_post', COALESCE(job_post, 0), \
                                 'provinceid', COALESCE(provinceid, 0), \
                                 'cityid', COALESCE(cityid, 0), \
                                 'three_cityid', COALESCE(three_cityid, 0), \
                                 'salary', COALESCE(salary, 0), \
                                 'minsalary', COALESCE(minsalary, 0), \
                                 'maxsalary', COALESCE(maxsalary, 0) \
                             ) AS params, \
                             COALESCE(status, 0) AS notify, \
                             COALESCE(stime, 0) AS last_notified_at, \
                             COALESCE(ctime, 0) AS created_at, \
                             COALESCE(ctime, 0) AS updated_at";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<SavedSearch>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_subscribe \
         WHERE uid = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, SavedSearch>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_subscribe WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct JobAlertInsert<'a> {
    pub uid: u64,
    pub email: &'a str,
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub salary: i32,
    pub r#type: i32,
    pub status: i8,
    pub code: &'a str,
    pub cycle_time: i64,
    pub time: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub jobclass_id: &'a str,
    pub stime: i64,
    pub ctime: i64,
}

pub async fn create_job_alert(pool: &MySqlPool, row: JobAlertInsert<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_subscribe
           (uid, email, job1, job1_son, job_post, provinceid, cityid, three_cityid,
            salary, type, ctime, status, code, cycle_time, time, minsalary, maxsalary,
            jobclass_id, stime)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(row.uid)
    .bind(row.email)
    .bind(row.job1)
    .bind(row.job1_son)
    .bind(row.job_post)
    .bind(row.provinceid)
    .bind(row.cityid)
    .bind(row.three_cityid)
    .bind(row.salary)
    .bind(row.r#type)
    .bind(row.ctime)
    .bind(row.status)
    .bind(row.code)
    .bind(row.cycle_time)
    .bind(row.time)
    .bind(row.minsalary)
    .bind(row.maxsalary)
    .bind(row.jobclass_id)
    .bind(row.stime)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

fn json_i32(v: &Value, keys: &[&str]) -> i32 {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(n) = x.as_i64() {
                return i32::try_from(n).unwrap_or(0);
            }
            if let Some(s) = x.as_str() {
                if let Ok(n) = s.trim().parse::<i32>() {
                    return n;
                }
            }
        }
    }
    0
}

fn jobclass_csv(job1: i32, job1_son: i32, job_post: i32) -> String {
    [job1, job1_son, job_post]
        .into_iter()
        .filter(|n| *n > 0)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    _kind: &str,
    params: &Value,
    notify: bool,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let job1 = json_i32(params, &["job1"]);
    let job1_son = json_i32(params, &["job1_son"]);
    let job_post = json_i32(params, &["job_post"]);
    let provinceid = json_i32(params, &["provinceid", "province_id"]);
    let cityid = json_i32(params, &["cityid", "city_id"]);
    let three_cityid = json_i32(params, &["three_cityid", "three_city_id"]);
    let minsalary = json_i32(params, &["minsalary", "min_salary"]);
    let maxsalary = json_i32(params, &["maxsalary", "max_salary"]);
    let salary = json_i32(params, &["salary"]);
    let time = json_i32(params, &["time"]);
    let r#type = {
        let n = json_i32(params, &["type"]);
        if n > 0 {
            n
        } else {
            1
        }
    };
    let jobclass_id = jobclass_csv(job1, job1_son, job_post);
    let cycle_time = if time > 0 {
        now + i64::from(time) * 86_400
    } else {
        0
    };
    create_job_alert(
        pool,
        JobAlertInsert {
            uid,
            email: name,
            job1,
            job1_son,
            job_post,
            provinceid,
            cityid,
            three_cityid,
            salary,
            r#type,
            status: if notify { 1 } else { 0 },
            code: "",
            cycle_time,
            time,
            minsalary,
            maxsalary,
            jobclass_id: &jobclass_id,
            stime: 0,
            ctime: now,
        },
    )
    .await
}

pub async fn set_notify(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    notify: bool,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP table has no `updated_at` — only flip status.
    let res = sqlx::query("UPDATE phpyun_subscribe SET status = ? WHERE id = ? AND uid = ?")
        .bind(if notify { 1i8 } else { 0 })
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_subscribe WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
