//! Strictly aligned with PHPYun Q&A: `phpyun_question` + `phpyun_answer`
//! + `phpyun_attention`.
//!
//! Question mapping (Rust -> PHP):
//!   - category_id   <-> cid
//!   - hits          <-> visit
//!   - answer_count  <-> answer_num
//!   - support_count <-> atnnum
//!   - status        <-> state
//!   - created_at    <-> add_time
//!
//! Answer mapping:
//!   - question_id   <-> qid
//!   - support_count <-> support
//!   - is_accepted   = 0 (PHP has no "accepted" field; Rust initializes to 0;
//!     mark_accepted uses local semantics)
//!   - created_at    <-> add_time
//!
//! PHPYun **does not have** a "question_supports" table (Rust originally
//! added one for like records). Following the "don't modify DB" rule,
//! likes only maintain counters and don't record "who liked what".
//! `toggle_support` therefore degrades to **idempotent +1** (no undo).

use super::entity::{Answer, AnswerReview, QClass, Question};
use crate::soft_delete::{self, PREDICATE};
use sqlx::{MySqlPool, QueryBuilder};

const Q_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(cid, 0) AS SIGNED) AS category_id, \
    CAST(COALESCE(visit, 0) AS UNSIGNED) AS hits, \
    CAST(COALESCE(answer_num, 0) AS UNSIGNED) AS answer_count, \
    CAST(COALESCE(atnnum, 0) AS UNSIGNED) AS support_count, \
    CAST(COALESCE(state, 0) AS SIGNED) AS status, \
    CAST(COALESCE(add_time, 0) AS SIGNED) AS created_at, \
    nickname, pic, \
    CAST(COALESCE(is_recom, 0) AS SIGNED) AS is_recom, \
    CAST(COALESCE(lastupdate, 0) AS SIGNED) AS lastupdate, \
    ip";

const A_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(qid, 0) AS UNSIGNED) AS question_id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(support, 0) AS UNSIGNED) AS support_count, \
    CAST(0 AS SIGNED) AS is_accepted, \
    CAST(COALESCE(add_time, 0) AS SIGNED) AS created_at, \
    nickname, pic, \
    CAST(COALESCE(usertype, 0) AS SIGNED) AS usertype, \
    CAST(COALESCE(comment, 0) AS UNSIGNED) AS comment_count, \
    CAST(COALESCE(oppose, 0) AS UNSIGNED) AS oppose_count, \
    CAST(COALESCE(cid, 0) AS SIGNED) AS category_id, \
    CAST(COALESCE(status, 1) AS SIGNED) AS status";

// ---------- Questions ----------

pub struct QuestionFilter<'a> {
    pub keyword: Option<&'a str>,
    pub category_id: Option<i32>,
    pub order: QuestionOrder,
}

pub enum QuestionOrder {
    Latest,
    Hot,
}

pub async fn list_questions(
    pool: &MySqlPool,
    f: &QuestionFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    let mut sql = format!("SELECT {Q_FIELDS} FROM phpyun_question WHERE state = 1 AND {PREDICATE}");
    if f.keyword.is_some() {
        sql.push_str(" AND title LIKE ?");
    }
    if f.category_id.is_some() {
        sql.push_str(" AND cid = ?");
    }
    sql.push_str(match f.order {
        QuestionOrder::Latest => " ORDER BY add_time DESC",
        QuestionOrder::Hot => " ORDER BY visit DESC, add_time DESC",
    });
    sql.push_str(" LIMIT ? OFFSET ?");
    let mut q = sqlx::query_as::<_, Question>(&sql);
    if let Some(kw) = f.keyword {
        q = q.bind(format!("%{kw}%"));
    }
    if let Some(c) = f.category_id {
        q = q.bind(c);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count_questions(pool: &MySqlPool, f: &QuestionFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut sql = format!("SELECT COUNT(*) FROM phpyun_question WHERE state = 1 AND {PREDICATE}");
    if f.keyword.is_some() {
        sql.push_str(" AND title LIKE ?");
    }
    if f.category_id.is_some() {
        sql.push_str(" AND cid = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(kw) = f.keyword {
        q = q.bind(format!("%{kw}%"));
    }
    if let Some(c) = f.category_id {
        q = q.bind(c);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_question(pool: &MySqlPool, id: u64) -> Result<Option<Question>, sqlx::Error> {
    let sql = format!("SELECT {Q_FIELDS} FROM phpyun_question WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Question>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct QuestionCreate<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub category_id: i32,
}

pub async fn create_question(
    pool: &MySqlPool,
    c: QuestionCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_question \
         (uid, title, content, cid, visit, answer_num, atnnum, state, add_time, lastupdate) \
         VALUES (?, ?, ?, ?, 0, 0, 0, 1, ?, ?)",
    )
    .bind(c.uid)
    .bind(c.title)
    .bind(c.content)
    .bind(c.category_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_question(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_question SET deleted=1 WHERE id = ? AND uid = ? AND COALESCE(deleted,0)=0")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn admin_delete_question(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_question", id).await
}

pub async fn set_question_state(pool: &MySqlPool, id: u64, state: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_question SET state = ? WHERE id = ?")
        .bind(state)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub struct AdminQuestionFilter<'a> {
    pub keyword: Option<&'a str>,
    /// PHP POST `status` maps to column `state`.
    pub status: Option<i32>,
    pub is_recom: Option<i32>,
}

pub async fn admin_list_questions(
    pool: &MySqlPool,
    f: &AdminQuestionFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    let mut qb: sqlx::QueryBuilder<sqlx::MySql> =
        sqlx::QueryBuilder::new(format!("SELECT {Q_FIELDS} FROM phpyun_question WHERE {PREDICATE}"));
    if let Some(s) = f.status {
        qb.push(" AND state = ");
        qb.push_bind(s);
    }
    if let Some(r) = f.is_recom {
        qb.push(" AND is_recom = ");
        qb.push_bind(r);
    }
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND title LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    qb.push(" ORDER BY add_time DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Question>().fetch_all(pool).await
}

pub async fn admin_count_questions(
    pool: &MySqlPool,
    f: &AdminQuestionFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: sqlx::QueryBuilder<sqlx::MySql> =
        sqlx::QueryBuilder::new(format!("SELECT COUNT(*) FROM phpyun_question WHERE {PREDICATE}"));
    if let Some(s) = f.status {
        qb.push(" AND state = ");
        qb.push_bind(s);
    }
    if let Some(r) = f.is_recom {
        qb.push(" AND is_recom = ");
        qb.push_bind(r);
    }
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND title LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn incr_question_hit(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_question SET visit = visit + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_questions_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    let sql = format!(
        "SELECT {Q_FIELDS} FROM phpyun_question \
         WHERE uid = ? AND {PREDICATE} ORDER BY add_time DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Question>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_questions_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM phpyun_question WHERE uid = ? AND {PREDICATE}"
    ))
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- Answers ----------

pub struct AnswerCreate<'a> {
    pub question_id: u64,
    pub uid: u64,
    pub content: &'a str,
}

pub async fn create_answer(
    pool: &MySqlPool,
    c: AnswerCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let res = sqlx::query(
        "INSERT INTO phpyun_answer (qid, uid, content, support, add_time, status) \
         VALUES (?, ?, ?, 0, ?, 1)",
    )
    .bind(c.question_id)
    .bind(c.uid)
    .bind(c.content)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    sqlx::query("UPDATE phpyun_question SET answer_num = answer_num + 1 WHERE id = ?")
        .bind(c.question_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(res.last_insert_id())
}

pub async fn list_answers(
    pool: &MySqlPool,
    question_id: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Answer>, sqlx::Error> {
    let sql = format!(
        "SELECT {A_FIELDS} FROM phpyun_answer \
         WHERE qid = ? \
         ORDER BY support DESC, add_time ASC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Answer>(&sql)
        .bind(question_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_answers(pool: &MySqlPool, question_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_answer WHERE qid = ?")
        .bind(question_id)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_answers_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Answer>, sqlx::Error> {
    let sql = format!(
        "SELECT {A_FIELDS} FROM phpyun_answer \
         WHERE uid = ? ORDER BY add_time DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Answer>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_answers_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_answer WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Cheap getter for an answer's `(qid, status)` pair. Used by the comment
/// endpoint, which only needs to validate that the parent answer exists and
/// is published before persisting a child review.
pub async fn answer_qid_status(
    pool: &MySqlPool,
    answer_id: u64,
) -> Result<Option<(u64, i32)>, sqlx::Error> {
    let row: Option<(i64, i32)> = sqlx::query_as(
        "SELECT CAST(COALESCE(qid,0) AS SIGNED), CAST(COALESCE(status,1) AS SIGNED) \
         FROM phpyun_answer WHERE id = ?",
    )
    .bind(answer_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(q, s)| (phpyun_core::numeric::nonnegative_count(q), s)))
}

/// PHPYun `phpyun_answer` has no is_accepted column -- this function has
/// no PHP equivalent. The Rust side keeps the API but only updates the
/// answer's status field (1 = accepted); other answers are unaffected.
pub async fn mark_answer_accepted(
    pool: &MySqlPool,
    answer_id: u64,
    _question_id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_answer SET status = 1 WHERE id = ?")
        .bind(answer_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ---------- Attentions (follow) ----------
//
// PHP `phpyun_attention` schema: `id, ids (text CSV), type, uid` —
// per-user-per-type rows where `ids` is a comma-separated list of attended
// question ids. There is no `qid` column. Toggle is implemented by
// rewriting the CSV in place: parse, add/remove, re-pack.
// `type=1` = questions (kept consistent with `list_attended_questions`).

pub async fn is_question_attended(
    pool: &MySqlPool,
    uid: u64,
    question_id: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT ids FROM phpyun_attention WHERE uid = ? AND type = 1 LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.and_then(|(ids,)| ids).is_some_and(|ids| {
        ids.split(',')
            .filter_map(|value| value.trim().parse::<u64>().ok())
            .any(|id| id == question_id)
    }))
}

pub async fn toggle_attention(
    pool: &MySqlPool,
    uid: u64,
    question_id: u64,
    _now: i64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT ids FROM phpyun_attention WHERE uid = ? AND type = 1 LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    let mut ids: Vec<u64> = row
        .as_ref()
        .and_then(|(s,)| s.as_deref())
        .map(|s| {
            s.split(',')
                .filter_map(|p| p.trim().parse::<u64>().ok())
                .filter(|id| *id > 0)
                .collect()
        })
        .unwrap_or_default();
    let was_present = ids.contains(&question_id);
    if was_present {
        ids.retain(|x| *x != question_id);
    } else {
        ids.push(question_id);
    }
    let csv = ids
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");
    if row.is_some() {
        sqlx::query("UPDATE phpyun_attention SET ids = ? WHERE uid = ? AND type = 1")
            .bind(&csv)
            .bind(uid)
            .execute(pool)
            .await?;
    } else if !ids.is_empty() {
        sqlx::query("INSERT INTO phpyun_attention (uid, type, ids) VALUES (?, 1, ?)")
            .bind(uid)
            .bind(&csv)
            .execute(pool)
            .await?;
    }
    Ok(!was_present)
}

pub async fn list_attended_questions(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    // PHP `phpyun_attention` shape: `id, ids (text CSV), type, uid` — there
    // is no `qid` column; "attended question ids" are stored CSV-packed in
    // the `ids` text field, distinguished by `type`. We fetch the CSV row
    // for this user, parse the ids client-side, then look up the question
    // rows in one shot. Empty result if no row / empty CSV.
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT ids FROM phpyun_attention WHERE uid = ? AND type = 1 LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    let csv = match row.and_then(|(ids,)| ids) {
        Some(s) if !s.trim().is_empty() => s,
        _ => return Ok(Vec::new()),
    };
    let mut question_ids: Vec<u64> = csv
        .split(',')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .filter(|id| *id > 0)
        .collect();
    if question_ids.is_empty() {
        return Ok(Vec::new());
    }
    // Apply offset/limit on the parsed list (PHP does the same — no DB-side
    // pagination available because `ids` is a single CSV).
    let off = phpyun_core::numeric::checked_db_usize(offset, "pagination.offset")?;
    let lim = phpyun_core::numeric::checked_db_usize(limit, "pagination.limit")?;
    if off >= question_ids.len() {
        return Ok(Vec::new());
    }
    let take_to = pagination_end(off, lim, question_ids.len());
    question_ids = question_ids[off..take_to].to_vec();

    let placeholders = std::iter::repeat_n("?", question_ids.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT {Q_FIELDS} FROM phpyun_question \
         WHERE state = 1 AND {PREDICATE} AND id IN ({placeholders}) \
         ORDER BY FIELD(id, {placeholders}) DESC"
    );
    let mut q = sqlx::query_as::<_, Question>(&sql);
    for id in &question_ids {
        q = q.bind(*id);
    }
    for id in &question_ids {
        q = q.bind(*id);
    }
    q.fetch_all(pool).await
}

fn pagination_end(offset: usize, limit: usize, len: usize) -> usize {
    offset.saturating_add(limit).min(len)
}

pub async fn count_attended_questions(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    // Count = number of comma-separated ids in `phpyun_attention.ids` for
    // type=1. Empty / missing row → 0.
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT ids FROM phpyun_attention WHERE uid = ? AND type = 1 LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    let n = match row.and_then(|(ids,)| ids) {
        Some(s) if !s.trim().is_empty() => s
            .split(',')
            .filter(|p| p.trim().parse::<u64>().is_ok())
            .count(),
        _ => 0_usize,
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- Support (like) ----------
//
// PHPYun has no "support/oppose" detail table; counters live on
// phpyun_question.atnnum / phpyun_answer.support. The Rust side originally
// tracked "who liked what" in `phpyun_rs_question_supports`, but per the
// "don't modify DB" rule that table doesn't exist -- this degrades to
// **idempotent +1**; callers see "already liked, no undo".

pub async fn toggle_support(
    pool: &MySqlPool,
    _uid: u64,
    target_kind: i32,
    target_id: u64,
    _now: i64,
) -> Result<bool, sqlx::Error> {
    let sql = if target_kind == super::entity::SUPPORT_KIND_QUESTION {
        "UPDATE phpyun_question SET atnnum = atnnum + 1 WHERE id = ?"
    } else {
        "UPDATE phpyun_answer SET support = support + 1 WHERE id = ?"
    };
    sqlx::query(sql).bind(target_id).execute(pool).await?;
    Ok(true)
}

// ---------- Reviews (answer comments: phpyun_answer_review) ----------

// `phpyun_answer_review` itself doesn't store nickname/avatar (PHP JOINs
// at render time).
//
// Here:
//   - nickname = `phpyun_member.username` (job seeker) or
//                `phpyun_company.linkman` (company)
//   - avatar   = `phpyun_resume.photo`    (job seeker) or
//                `phpyun_company.logo`    (company)
//
// The job seeker's avatar lives on the resume table
// (phpyun_member has no pic column), so we LEFT JOIN once more.
const AR_FIELDS: &str = "\
    CAST(r.id AS UNSIGNED) AS id, \
    CAST(COALESCE(r.aid, 0) AS UNSIGNED) AS aid, \
    CAST(COALESCE(r.qid, 0) AS UNSIGNED) AS qid, \
    CAST(COALESCE(r.uid, 0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(r.usertype, 0) AS SIGNED) AS usertype, \
    COALESCE(r.content, '') AS content, \
    CAST(COALESCE(r.support, 0) AS SIGNED) AS support, \
    CAST(COALESCE(r.status, 1) AS SIGNED) AS status, \
    CAST(COALESCE(r.add_time, 0) AS SIGNED) AS add_time, \
    COALESCE(m.username, c.linkman) AS nickname, \
    COALESCE(rs.photo, c.logo) AS pic";

/// List comments under an answer (public read; only returns status=1 approved rows).
pub async fn list_reviews_by_answer(
    pool: &MySqlPool,
    aid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<AnswerReview>, sqlx::Error> {
    let sql = format!(
        "SELECT {AR_FIELDS} FROM phpyun_answer_review r \
         LEFT JOIN phpyun_member  m  ON m.uid  = r.uid AND r.usertype = 1 \
         LEFT JOIN phpyun_resume  rs ON rs.uid = r.uid AND r.usertype = 1 \
         LEFT JOIN phpyun_company c  ON c.uid  = r.uid AND r.usertype = 2 \
         WHERE r.aid = ? AND r.status = 1 \
         ORDER BY r.add_time ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, AnswerReview>(&sql)
        .bind(phpyun_core::numeric::checked_db_i64(
            aid,
            "answer_review.aid",
        )?)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_reviews_by_answer(pool: &MySqlPool, aid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_answer_review WHERE aid = ? AND status = 1")
            .bind(phpyun_core::numeric::checked_db_i64(
                aid,
                "answer_review.aid",
            )?)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct ReviewCreate<'a> {
    pub aid: u64,
    pub qid: u64,
    pub uid: u64,
    pub usertype: i32,
    pub content: &'a str,
    pub status: i32,
}

/// Write a comment; also bumps the parent answer's `comment` counter by 1
/// (PHP only increments when status=1, mirrored here).
pub async fn create_review(
    pool: &MySqlPool,
    c: ReviewCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let res = sqlx::query(
        "INSERT INTO phpyun_answer_review \
         (aid, qid, uid, usertype, content, support, status, add_time) \
         VALUES (?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(phpyun_core::numeric::checked_db_i64(
        c.aid,
        "answer_review.aid",
    )?)
    .bind(phpyun_core::numeric::checked_db_i64(
        c.qid,
        "answer_review.qid",
    )?)
    .bind(phpyun_core::numeric::checked_db_i64(
        c.uid,
        "answer_review.uid",
    )?)
    .bind(c.usertype)
    .bind(c.content)
    .bind(c.status)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    let new_id = res.last_insert_id();
    if c.status == 1 {
        sqlx::query("UPDATE phpyun_answer SET comment = comment + 1 WHERE id = ?")
            .bind(phpyun_core::numeric::checked_db_i64(
                c.aid,
                "answer_review.aid",
            )?)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(new_id)
}

/// Delete a comment (only the author can; phpyun_answer.comment is decremented).
pub async fn delete_review(pool: &MySqlPool, review_id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64, i32)> =
        sqlx::query_as("SELECT aid, status FROM phpyun_answer_review WHERE id = ? AND uid = ?")
            .bind(phpyun_core::numeric::checked_db_i64(
                review_id,
                "answer_review.id",
            )?)
            .bind(phpyun_core::numeric::checked_db_i64(
                uid,
                "answer_review.uid",
            )?)
            .fetch_optional(pool)
            .await?;
    let Some((aid, status)) = row else {
        return Ok(0);
    };
    let mut tx = pool.begin().await?;
    let res = sqlx::query("DELETE FROM phpyun_answer_review WHERE id = ? AND uid = ?")
        .bind(phpyun_core::numeric::checked_db_i64(
            review_id,
            "answer_review.id",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            uid,
            "answer_review.uid",
        )?)
        .execute(&mut *tx)
        .await?;
    if res.rows_affected() > 0 && status == 1 {
        sqlx::query("UPDATE phpyun_answer SET comment = GREATEST(comment - 1, 0) WHERE id = ?")
            .bind(aid)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(res.rows_affected())
}

// ---------- Categories (phpyun_q_class) ----------

const QC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(pid, 0) AS SIGNED) AS pid, \
    pic, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    intro, \
    CAST(COALESCE(add_time, 0) AS SIGNED) AS add_time";

pub async fn list_qclasses(pool: &MySqlPool) -> Result<Vec<QClass>, sqlx::Error> {
    let sql = format!("SELECT {QC_FIELDS} FROM phpyun_q_class WHERE {PREDICATE} ORDER BY pid ASC, sort DESC, id ASC");
    sqlx::query_as::<_, QClass>(&sql).fetch_all(pool).await
}

pub async fn list_qclasses_admin(
    pool: &MySqlPool,
    pid: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<QClass>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(QC_FIELDS);
    qb.push(" FROM phpyun_q_class WHERE ");
    qb.push(PREDICATE);
    qb.push(" AND pid = ");
    qb.push_bind(pid.unwrap_or(0));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY sort DESC, id ASC LIMIT ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?);
    qb.push(" OFFSET ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(
        offset,
        "pagination.offset",
    )?);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_qclasses_admin(
    pool: &MySqlPool,
    pid: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!(
            "SELECT COUNT(*) FROM phpyun_q_class WHERE {PREDICATE} AND pid = "
        ));
    qb.push_bind(pid.unwrap_or(0));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_qclass(pool: &MySqlPool, id: u64) -> Result<Option<QClass>, sqlx::Error> {
    let sql = format!("SELECT {QC_FIELDS} FROM phpyun_q_class WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, QClass>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_qclass(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    pid: i32,
    intro: &str,
    sort: i32,
    pic: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        if let Some(p) = pic {
            sqlx::query(
                "UPDATE phpyun_q_class SET name=?, pid=?, intro=?, sort=?, pic=? WHERE id=?",
            )
            .bind(name)
            .bind(pid)
            .bind(intro)
            .bind(sort)
            .bind(p)
            .bind(id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query("UPDATE phpyun_q_class SET name=?, pid=?, intro=?, sort=? WHERE id=?")
                .bind(name)
                .bind(pid)
                .bind(intro)
                .bind(sort)
                .bind(id)
                .execute(pool)
                .await?;
        }
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_q_class (name, pid, pic, sort, intro, add_time) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(name)
    .bind(pid)
    .bind(pic.unwrap_or(""))
    .bind(sort)
    .bind(intro)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_qclasses(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_col_in(pool, "phpyun_question", "cid", ids).await?;
    soft_delete::mark_col_in(pool, "phpyun_q_class", "pid", ids).await?;
    soft_delete::mark_ids(pool, "phpyun_q_class", ids).await
}

// ---------- Hotweek (this week's hot questions) ----------

/// Return hot questions from the last `since` seconds, ordered by
/// (atnnum + answer_num + visit/3). Aligns with PHPYun
/// `wap/ask::hotweek_action`'s 7-day window.
pub async fn hotweek_questions(
    pool: &MySqlPool,
    since: i64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    let sql = format!(
        "SELECT {Q_FIELDS} FROM phpyun_question \
         WHERE state IN (0,1) AND add_time >= ? AND {PREDICATE} \
         ORDER BY (COALESCE(atnnum,0) + COALESCE(answer_num,0) + COALESCE(visit,0)/3) DESC, id DESC \
         LIMIT ?"
    );
    sqlx::query_as::<_, Question>(&sql)
        .bind(since)
        .bind(limit)
        .fetch_all(pool)
        .await
}

// ==================== Top answerers leaderboard ====================
//
// Counterpart of PHP `ask::getAnswersList(groupby:uid, orderby:num)` used by
// `topic.class.php` and `search.class.php` to render the "热门回答者" sidebar:
// in the last 30 days, group by uid, count answers (`num`), sum support
// votes (`support`), order by `num DESC` then `support DESC`.

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AnswererBrief {
    pub uid: u64,
    pub nickname: Option<String>,
    pub answer_count: u64,
    pub support_total: u64,
}

pub async fn list_top_answerers(
    pool: &MySqlPool,
    since: i64,
    limit: u64,
) -> Result<Vec<AnswererBrief>, sqlx::Error> {
    sqlx::query_as::<_, AnswererBrief>(
        "SELECT \
            CAST(uid AS UNSIGNED) AS uid, \
            MAX(nickname) AS nickname, \
            CAST(COUNT(id) AS UNSIGNED) AS answer_count, \
            CAST(COALESCE(SUM(support), 0) AS UNSIGNED) AS support_total \
         FROM phpyun_answer \
         WHERE add_time >= ? \
         GROUP BY uid \
         ORDER BY answer_count DESC, support_total DESC \
         LIMIT ?",
    )
    .bind(since)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn set_question_recom(pool: &MySqlPool, id: u64, rec: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_question SET is_recom = ? WHERE id = ?")
            .bind(rec)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn upsert_question_admin(
    pool: &MySqlPool,
    id: u64,
    title: &str,
    cid: i32,
    content: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if id > 0 {
        sqlx::query(
            "UPDATE phpyun_question SET title = ?, cid = ?, content = ?, lastupdate = ? WHERE id = ?",
        )
        .bind(title)
        .bind(cid)
        .bind(content)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_question (title, cid, content, uid, nickname, add_time, lastupdate, state) \
             VALUES (?, ?, ?, 0, 'admin', ?, ?, 1)",
        )
        .bind(title)
        .bind(cid)
        .bind(content)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn admin_delete_questions(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_question", ids).await
}

pub async fn list_answers_admin(
    pool: &MySqlPool,
    qid: Option<u64>,
    aid: Option<u64>,
    status: Option<i32>,
) -> Result<Vec<Answer>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!("SELECT {A_FIELDS} FROM phpyun_answer WHERE 1=1"));
    if let Some(id) = aid.filter(|i| *i > 0) {
        qb.push(" AND id = ");
        qb.push_bind(id);
    }
    if let Some(id) = qid.filter(|i| *i > 0) {
        qb.push(" AND qid = ");
        qb.push_bind(id);
    }
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY add_time DESC LIMIT 500");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn set_answer_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_answer SET status = ?, statusbody = ? WHERE id = ?")
            .bind(status)
            .bind(statusbody)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn update_answer_admin(
    pool: &MySqlPool,
    id: u64,
    content: &str,
    support: i32,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_answer SET content = ?, support = ? WHERE id = ?")
            .bind(content)
            .bind(support)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn delete_answers(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_answer WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn decr_answer_num(pool: &MySqlPool, qid: u64, n: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_question SET answer_num = GREATEST(CAST(answer_num AS SIGNED) - ?, 0) WHERE id = ?")
            .bind(n)
            .bind(qid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn list_reviews_admin(
    pool: &MySqlPool,
    aid: Option<u64>,
    id: Option<u64>,
    status: Option<i32>,
) -> Result<Vec<AnswerReview>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!(
        "SELECT {AR_FIELDS} FROM phpyun_answer_review r \
         LEFT JOIN phpyun_member  m  ON m.uid  = r.uid AND r.usertype = 1 \
         LEFT JOIN phpyun_resume  rs ON rs.uid = r.uid AND r.usertype = 1 \
         LEFT JOIN phpyun_company c  ON c.uid  = r.uid AND r.usertype = 2 \
         WHERE 1=1"
    ));
    if let Some(v) = aid.filter(|i| *i > 0) {
        qb.push(" AND r.aid = ");
        qb.push_bind(v);
    }
    if let Some(v) = id.filter(|i| *i > 0) {
        qb.push(" AND r.id = ");
        qb.push_bind(v);
    }
    if let Some(s) = status {
        qb.push(" AND r.status = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY r.id DESC LIMIT 500");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn set_review_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_answer_review SET status = ?, statusbody = ? WHERE id = ?")
            .bind(status)
            .bind(statusbody)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn update_review_content(pool: &MySqlPool, id: u64, content: &str) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_answer_review SET content = ? WHERE id = ?")
            .bind(content)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn delete_reviews(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_answer_review WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::pagination_end;

    #[test]
    fn csv_pagination_end_saturates_before_clamping_to_length() {
        assert_eq!(pagination_end(3, 4, 20), 7);
        assert_eq!(pagination_end(usize::MAX - 2, 10, usize::MAX), usize::MAX);
    }
}
