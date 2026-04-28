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
//!
//! Caveat: when creating a saved search the JSON `params` blob is **not**
//! decomposed into per-filter columns; the row is inserted with default
//! filter values. Wiring the JSON → discrete-cols mapper is a follow-up
//! task once the product confirms which filter keys to support.

use super::entity::SavedSearch;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(uid AS UNSIGNED) AS uid, \
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
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_subscribe WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    _kind: &str,
    _params: &serde_json::Value,
    notify: bool,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP `phpyun_subscribe` has discrete columns for each filter rather
    // than a JSON blob. We persist what fits — uid, email (used as label),
    // ctime, status=notify-flag — and leave per-filter columns at their
    // table defaults. A follow-up task can decompose the JSON `params` into
    // job1 / cityid / salary / etc.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_subscribe
           (uid, email, ctime, status)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(name)
    .bind(now)
    .bind(if notify { 1i8 } else { 0 })
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_notify(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    notify: bool,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP table has no `updated_at` — only flip status.
    let res = sqlx::query(
        "UPDATE phpyun_subscribe SET status = ? WHERE id = ? AND uid = ?",
    )
    .bind(if notify { 1i8 } else { 0 })
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("DELETE FROM phpyun_subscribe WHERE id = ? AND uid = ?")
            .bind(id)
            .bind(uid)
            .execute(pool)
            .await?;
    Ok(res.rows_affected())
}
