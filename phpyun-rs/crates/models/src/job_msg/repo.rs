//! `phpyun_msg` repository — public job-page Q&A.
//!
//! See [`super::entity::JobMsg`] for column meanings. All numeric columns
//! are signed `INT(11)` in PHP; cast at SELECT for `u64` decode parity.

use super::entity::{JobMsg, MSG_TYPE_PUBLIC_QA};
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(uid AS UNSIGNED) AS uid, \
    username, \
    CAST(jobid AS UNSIGNED) AS jobid, \
    CAST(job_uid AS UNSIGNED) AS job_uid, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS datetime, \
    content, \
    reply, \
    CAST(COALESCE(reply_time, 0) AS SIGNED) AS reply_time, \
    com_name, \
    job_name, \
    CAST(COALESCE(del_status, 0) AS SIGNED) AS del_status, \
    CAST(COALESCE(type, 1) AS SIGNED) AS `type`, \
    CAST(COALESCE(user_remind_status, 1) AS SIGNED) AS user_remind_status, \
    CAST(COALESCE(com_remind_status, 0) AS SIGNED) AS com_remind_status, \
    CAST(COALESCE(status, 1) AS SIGNED) AS status";

pub struct InsertMsg<'a> {
    pub uid: u64,
    pub username: &'a str,
    pub jobid: u64,
    pub job_uid: u64,
    pub content: &'a str,
    pub com_name: &'a str,
    pub job_name: &'a str,
    pub now: i64,
}

pub async fn insert(pool: &MySqlPool, m: InsertMsg<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_msg \
            (uid, username, jobid, job_uid, content, com_name, job_name, \
             datetime, type, user_remind_status, com_remind_status, status, del_status, issys) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0, 1, 0, 1)",
    )
    .bind(m.uid)
    .bind(m.username)
    .bind(m.jobid)
    .bind(m.job_uid)
    .bind(m.content)
    .bind(m.com_name)
    .bind(m.job_name)
    .bind(m.now)
    .bind(MSG_TYPE_PUBLIC_QA)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<JobMsg>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_msg WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, JobMsg>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Public list: only show approved + visible + has-reply rows. PHP's
/// job-detail page filters the same way (`status=1 AND reply<>'' AND del_status=0`).
pub async fn list_public_for_job(
    pool: &MySqlPool,
    jobid: u64,
    job_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<JobMsg>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_msg \
         WHERE jobid = ? AND job_uid = ? AND status = 1 \
           AND reply IS NOT NULL AND reply != '' AND del_status = 0 \
           AND `type` = ? \
         ORDER BY datetime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, JobMsg>(&sql)
        .bind(jobid)
        .bind(job_uid)
        .bind(MSG_TYPE_PUBLIC_QA)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_public_for_job(
    pool: &MySqlPool,
    jobid: u64,
    job_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_msg \
         WHERE jobid = ? AND job_uid = ? AND status = 1 \
           AND reply IS NOT NULL AND reply != '' AND del_status = 0 \
           AND `type` = ?",
    )
    .bind(jobid)
    .bind(job_uid)
    .bind(MSG_TYPE_PUBLIC_QA)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Employer-side list: include unanswered messages too.
pub async fn list_for_employer(
    pool: &MySqlPool,
    job_uid: u64,
    only_unanswered: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<JobMsg>, sqlx::Error> {
    let mut sql = format!(
        "SELECT {FIELDS} FROM phpyun_msg \
         WHERE job_uid = ? AND `type` = ? AND del_status = 0"
    );
    if only_unanswered {
        sql.push_str(" AND (reply IS NULL OR reply = '')");
    }
    sql.push_str(" ORDER BY datetime DESC LIMIT ? OFFSET ?");
    sqlx::query_as::<_, JobMsg>(&sql)
        .bind(job_uid)
        .bind(MSG_TYPE_PUBLIC_QA)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_employer(
    pool: &MySqlPool,
    job_uid: u64,
    only_unanswered: bool,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from(
        "SELECT COUNT(*) FROM phpyun_msg \
         WHERE job_uid = ? AND `type` = ? AND del_status = 0",
    );
    if only_unanswered {
        sql.push_str(" AND (reply IS NULL OR reply = '')");
    }
    let (n,): (i64,) = sqlx::query_as(&sql)
        .bind(job_uid)
        .bind(MSG_TYPE_PUBLIC_QA)
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
}

pub async fn employer_reply(
    pool: &MySqlPool,
    id: u64,
    job_uid: u64,
    reply: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_msg \
            SET reply = ?, reply_time = ?, user_remind_status = 0 \
          WHERE id = ? AND job_uid = ?",
    )
    .bind(reply)
    .bind(now)
    .bind(id)
    .bind(job_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft-hide the message — only the original author or job owner is allowed
/// to call this (enforced in the service layer).
pub async fn soft_hide(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_msg SET del_status = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
