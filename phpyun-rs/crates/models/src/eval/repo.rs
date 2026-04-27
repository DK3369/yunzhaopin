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

/// Look up a single log by id (with owner check). Used by `gradeshow_action`
/// equivalent. Returns `None` when the log doesn't exist OR doesn't belong
/// to the caller (avoids leaking other users' grades).
pub async fn find_log_for_owner(
    pool: &MySqlPool,
    log_id: u64,
    uid: u64,
) -> Result<Option<EvalLog>, sqlx::Error> {
    let sql = format!(
        "SELECT {LOG_FIELDS} FROM phpyun_evaluate_log \
         WHERE id = ? AND uid = ? LIMIT 1"
    );
    sqlx::query_as::<_, EvalLog>(&sql)
        .bind(log_id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ExamineeBrief {
    pub uid: u64,
    pub last_taken_at: i64,
    /// Number of distinct papers this user has taken (PHP groups by uid).
    pub papers_taken: u64,
}

/// Recent examinees who have taken any paper, grouped by uid. Counterpart
/// of PHP `evaluate.model.php::getEvaluateLogList(groupby:uid, orderby:ctime,desc)`
/// — drives the "他们也参加了测评" sidebar.
pub async fn list_recent_examinees(
    pool: &MySqlPool,
    examid: u32,
    limit: u64,
) -> Result<Vec<ExamineeBrief>, sqlx::Error> {
    sqlx::query_as::<_, ExamineeBrief>(
        "SELECT \
            CAST(uid AS UNSIGNED) AS uid, \
            CAST(MAX(ctime) AS SIGNED) AS last_taken_at, \
            CAST(COUNT(DISTINCT examid) AS UNSIGNED) AS papers_taken \
         FROM phpyun_evaluate_log \
         WHERE uid > 0 AND examid = ? \
         GROUP BY uid \
         ORDER BY last_taken_at DESC \
         LIMIT ?",
    )
    .bind(examid)
    .bind(limit)
    .fetch_all(pool)
    .await
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

// ==================== Eval paper leave-message ====================
//
// Counterpart of PHP `evaluate/exampaper::message_action` writes +
// `evaluate.model.php::getMessageList` reads. Backed by
// `phpyun_evaluate_leave_message`.

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct PaperMessage {
    pub id: u64,
    pub examid: u32,
    pub uid: String,
    pub usertype: Option<i32>,
    pub message: Option<String>,
    pub ctime: i64,
}

const PMSG_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(examid, 0) AS UNSIGNED) AS examid, \
    COALESCE(uid, '') AS uid, \
    CAST(usertype AS SIGNED) AS usertype, \
    message, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime";

pub async fn insert_paper_message(
    pool: &MySqlPool,
    examid: u32,
    uid: u64,
    usertype: i32,
    message: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_evaluate_leave_message (examid, uid, usertype, message, ctime) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(examid)
    .bind(uid.to_string())
    .bind(usertype)
    .bind(message)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_paper_messages(
    pool: &MySqlPool,
    examid: u32,
    offset: u64,
    limit: u64,
) -> Result<Vec<PaperMessage>, sqlx::Error> {
    let sql = format!(
        "SELECT {PMSG_FIELDS} FROM phpyun_evaluate_leave_message \
         WHERE examid = ? \
         ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, PaperMessage>(&sql)
        .bind(examid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_paper_messages(
    pool: &MySqlPool,
    examid: u32,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_evaluate_leave_message WHERE examid = ?",
    )
    .bind(examid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
