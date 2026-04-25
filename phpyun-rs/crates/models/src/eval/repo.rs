//! Strictly aligned with PHPYun's evaluation feature.
//!
//! Note that PHPYun naming is **inverted** relative to Rust:
//!   - `phpyun_evaluate_group` = paper / quiz (Rust `EvalPaper`)
//!   - `phpyun_evaluate`       = question (Rust `EvalQuestion`)
//!   - `phpyun_evaluate_log`   = answer record (Rust `EvalLog`)

use super::entity::{EvalLog, EvalPaper, EvalQuestion};
use sqlx::MySqlPool;

const PAPER_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(description, '') AS description, \
    COALESCE(pic, '') AS cover, \
    CAST(COALESCE(visits, 0) AS UNSIGNED) AS visits, \
    CAST(1 AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

const Q_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(gid, 0) AS UNSIGNED) AS paper_id, \
    COALESCE(question, '') AS content, \
    JSON_ARRAY(COALESCE(`option`, '')) AS options, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort";

const LOG_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(examid, 0) AS UNSIGNED) AS paper_id, \
    CAST(COALESCE(grade, 0) AS SIGNED) AS score, \
    CAST(JSON_ARRAY() AS JSON) AS answers, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

pub async fn list_papers(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<EvalPaper>, sqlx::Error> {
    let sql = format!(
        "SELECT {PAPER_FIELDS} FROM phpyun_evaluate_group \
         ORDER BY sort DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, EvalPaper>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_papers(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_evaluate_group")
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_paper(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<EvalPaper>, sqlx::Error> {
    let sql = format!("SELECT {PAPER_FIELDS} FROM phpyun_evaluate_group WHERE id = ?");
    sqlx::query_as::<_, EvalPaper>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_paper_visits(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_evaluate_group SET visits = visits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_questions(
    pool: &MySqlPool,
    paper_id: u64,
) -> Result<Vec<EvalQuestion>, sqlx::Error> {
    let sql = format!(
        "SELECT {Q_FIELDS} FROM phpyun_evaluate WHERE gid = ? ORDER BY sort ASC, id ASC"
    );
    sqlx::query_as::<_, EvalQuestion>(&sql)
        .bind(paper_id)
        .fetch_all(pool)
        .await
}

pub async fn create_log(
    pool: &MySqlPool,
    uid: u64,
    paper_id: u64,
    score: i32,
    _answers: &serde_json::Value,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHPYun does not store per-question answers; only uid/examid/grade/ctime/usedsecond.
    let res = sqlx::query(
        "INSERT INTO phpyun_evaluate_log (uid, examid, grade, ctime, usedsecond) \
         VALUES (?, ?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(paper_id)
    .bind(score)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_logs_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<EvalLog>, sqlx::Error> {
    let sql = format!(
        "SELECT {LOG_FIELDS} FROM phpyun_evaluate_log \
         WHERE uid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, EvalLog>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_logs_by_user(
    pool: &MySqlPool,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_evaluate_log WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}
