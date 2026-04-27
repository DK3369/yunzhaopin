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

use super::entity::{Answer, Question};
use sqlx::MySqlPool;

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
    let mut sql = format!("SELECT {Q_FIELDS} FROM phpyun_question WHERE state = 1");
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

pub async fn count_questions(
    pool: &MySqlPool,
    f: &QuestionFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from("SELECT COUNT(*) FROM phpyun_question WHERE state = 1");
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
    Ok(n.max(0) as u64)
}

pub async fn find_question(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<Question>, sqlx::Error> {
    let sql = format!("SELECT {Q_FIELDS} FROM phpyun_question WHERE id = ?");
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
    let res = sqlx::query("DELETE FROM phpyun_question WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
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
         WHERE uid = ? ORDER BY add_time DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Question>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_questions_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_question WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
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
    Ok(n.max(0) as u64)
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
    Ok(n.max(0) as u64)
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
// PHPYun `phpyun_attention` columns: id, uid, qid, add_time
// (different versions may use question_id/add_time; we conservatively
// stick with the most common "uid/qid/add_time").

pub async fn toggle_attention(
    pool: &MySqlPool,
    uid: u64,
    question_id: u64,
    now: i64,
) -> Result<bool, sqlx::Error> {
    let existed = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM phpyun_attention WHERE uid = ? AND qid = ?",
    )
    .bind(uid)
    .bind(question_id)
    .fetch_optional(pool)
    .await?
    .is_some();
    if existed {
        sqlx::query("DELETE FROM phpyun_attention WHERE uid = ? AND qid = ?")
            .bind(uid)
            .bind(question_id)
            .execute(pool)
            .await?;
        Ok(false)
    } else {
        sqlx::query("INSERT IGNORE INTO phpyun_attention (uid, qid, add_time) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(question_id)
            .bind(now)
            .execute(pool)
            .await?;
        Ok(true)
    }
}

pub async fn list_attended_questions(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Question>, sqlx::Error> {
    let sql = format!(
        "SELECT {Q_FIELDS} \
         FROM phpyun_question q \
         INNER JOIN phpyun_attention a ON a.qid = q.id AND a.uid = ? \
         WHERE q.state = 1 \
         ORDER BY a.add_time DESC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Question>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_attended_questions(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_attention WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
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

use super::entity::{AnswerReview, QClass};

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
        .bind(aid as i64)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_reviews_by_answer(pool: &MySqlPool, aid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_answer_review WHERE aid = ? AND status = 1")
            .bind(aid as i64)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
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
    .bind(c.aid as i64)
    .bind(c.qid as i64)
    .bind(c.uid as i64)
    .bind(c.usertype)
    .bind(c.content)
    .bind(c.status)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    let new_id = res.last_insert_id();
    if c.status == 1 {
        sqlx::query("UPDATE phpyun_answer SET comment = comment + 1 WHERE id = ?")
            .bind(c.aid as i64)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(new_id)
}

/// Delete a comment (only the author can; phpyun_answer.comment is decremented).
pub async fn delete_review(
    pool: &MySqlPool,
    review_id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let row: Option<(i64, i32)> =
        sqlx::query_as("SELECT aid, status FROM phpyun_answer_review WHERE id = ? AND uid = ?")
            .bind(review_id as i64)
            .bind(uid as i64)
            .fetch_optional(pool)
            .await?;
    let Some((aid, status)) = row else {
        return Ok(0);
    };
    let mut tx = pool.begin().await?;
    let res = sqlx::query("DELETE FROM phpyun_answer_review WHERE id = ? AND uid = ?")
        .bind(review_id as i64)
        .bind(uid as i64)
        .execute(&mut *tx)
        .await?;
    if res.rows_affected() > 0 && status == 1 {
        sqlx::query(
            "UPDATE phpyun_answer SET comment = GREATEST(comment - 1, 0) WHERE id = ?",
        )
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
    let sql =
        format!("SELECT {QC_FIELDS} FROM phpyun_q_class ORDER BY pid ASC, sort DESC, id ASC");
    sqlx::query_as::<_, QClass>(&sql).fetch_all(pool).await
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
         WHERE state IN (0,1) AND add_time >= ? \
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
