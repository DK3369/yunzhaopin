//! Admin CRUD for `phpyun_evaluate_group` / `phpyun_evaluate` / messages / logs.

use sqlx::{MySqlPool, QueryBuilder};

use super::php_ser;
use crate::soft_delete::{self, PREDICATE};

const PAPER_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(keyid, 0) AS SIGNED) AS keyid, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    COALESCE(description, '') AS description, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    COALESCE(fromscore, '') AS fromscore, \
    COALESCE(toscore, '') AS toscore, \
    COALESCE(comment, '') AS comment, \
    CAST(COALESCE(visits, 0) AS UNSIGNED) AS visits, \
    COALESCE(pic, '') AS pic, \
    CAST(COALESCE(recommend, 0) AS SIGNED) AS recommend, \
    CAST(COALESCE(top, 0) AS SIGNED) AS top, \
    CAST(COALESCE(hot, 0) AS SIGNED) AS hot, \
    CAST(COALESCE(num, 0) AS SIGNED) AS num, \
    CAST(COALESCE(score, 0) AS SIGNED) AS score";

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AdminEvalPaper {
    pub id: u64,
    pub keyid: i32,
    pub name: String,
    pub sort: i32,
    pub description: String,
    pub ctime: i64,
    pub fromscore: String,
    pub toscore: String,
    pub comment: String,
    pub visits: u32,
    pub pic: String,
    pub recommend: i32,
    pub top: i32,
    pub hot: i32,
    pub num: i32,
    pub score: i32,
    #[sqlx(skip)]
    #[serde(default)]
    pub ctime_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub url: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub keyid_n: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AdminEvalQuestion {
    pub id: u64,
    pub paper_id: u64,
    pub question: String,
    pub option: String,
    pub score: String,
    pub sort: i32,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AdminEvalMessage {
    pub id: u64,
    pub examid: u32,
    pub uid: String,
    pub usertype: Option<i32>,
    pub message: Option<String>,
    pub ctime: i64,
    #[serde(default)]
    pub name: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub ctime_n: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AdminEvalLog {
    pub id: u64,
    pub uid: u64,
    pub examid: u64,
    pub grade: i32,
    pub ctime: i64,
    pub usedsecond: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub title: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub ctime_n: String,
}

pub struct PaperWrite<'a> {
    pub name: &'a str,
    pub keyid: i32,
    pub sort: i32,
    pub top: i32,
    pub hot: i32,
    pub recommend: i32,
    pub description: &'a str,
    pub pic: Option<&'a str>,
    pub fromscore: &'a [String],
    pub toscore: &'a [String],
    pub comment: &'a [String],
}

pub struct QuestionWrite<'a> {
    pub id: Option<u64>,
    pub question: &'a str,
    pub option: &'a [String],
    pub score: &'a [String],
}

fn bind_limit(limit: u64, offset: u64) -> Result<(i64, i64), sqlx::Error> {
    Ok((
        phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?,
        phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?,
    ))
}

pub async fn list_papers(
    pool: &MySqlPool,
    keyid: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminEvalPaper>, sqlx::Error> {
    let (lim, off) = bind_limit(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(PAPER_FIELDS);
    qb.push(" FROM phpyun_evaluate_group WHERE keyid <> 0 AND ");
    qb.push(PREDICATE);
    if let Some(k) = keyid.filter(|v| *v > 0) {
        qb.push(" AND keyid = ");
        qb.push_bind(k);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(lim);
    qb.push(" OFFSET ");
    qb.push_bind(off);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_papers(
    pool: &MySqlPool,
    keyid: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!(
            "SELECT COUNT(*) FROM phpyun_evaluate_group WHERE keyid <> 0 AND {PREDICATE}"
        ));
    if let Some(k) = keyid.filter(|v| *v > 0) {
        qb.push(" AND keyid = ");
        qb.push_bind(k);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_paper(pool: &MySqlPool, id: u64) -> Result<Option<AdminEvalPaper>, sqlx::Error> {
    let sql = format!("SELECT {PAPER_FIELDS} FROM phpyun_evaluate_group WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, AdminEvalPaper>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_groups(pool: &MySqlPool) -> Result<Vec<AdminEvalPaper>, sqlx::Error> {
    let sql = format!(
        "SELECT {PAPER_FIELDS} FROM phpyun_evaluate_group WHERE keyid = 0 AND {PREDICATE} ORDER BY sort DESC, id ASC"
    );
    sqlx::query_as::<_, AdminEvalPaper>(&sql)
        .fetch_all(pool)
        .await
}

pub async fn count_papers_in_group(pool: &MySqlPool, keyid: i32) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as(&format!(
            "SELECT COUNT(*) FROM phpyun_evaluate_group WHERE keyid = ? AND {PREDICATE}"
        ))
            .bind(keyid)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn insert_paper(pool: &MySqlPool, w: PaperWrite<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let from = php_ser::serialize_strings(w.fromscore);
    let to = php_ser::serialize_strings(w.toscore);
    let comment = php_ser::serialize_strings(w.comment);
    let pic = w.pic.unwrap_or("");
    let res = sqlx::query(
        "INSERT INTO phpyun_evaluate_group \
         (keyid, name, sort, description, ctime, fromscore, toscore, comment, pic, recommend, top, hot) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(w.keyid)
    .bind(w.name)
    .bind(w.sort)
    .bind(w.description)
    .bind(now)
    .bind(from)
    .bind(to)
    .bind(comment)
    .bind(pic)
    .bind(w.recommend)
    .bind(w.top)
    .bind(w.hot)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_paper(pool: &MySqlPool, id: u64, w: PaperWrite<'_>) -> Result<u64, sqlx::Error> {
    let from = php_ser::serialize_strings(w.fromscore);
    let to = php_ser::serialize_strings(w.toscore);
    let comment = php_ser::serialize_strings(w.comment);
    let n = if let Some(pic) = w.pic {
        sqlx::query(
            "UPDATE phpyun_evaluate_group SET keyid=?, name=?, sort=?, description=?, \
             fromscore=?, toscore=?, comment=?, pic=?, recommend=?, top=?, hot=? WHERE id=?",
        )
        .bind(w.keyid)
        .bind(w.name)
        .bind(w.sort)
        .bind(w.description)
        .bind(from)
        .bind(to)
        .bind(comment)
        .bind(pic)
        .bind(w.recommend)
        .bind(w.top)
        .bind(w.hot)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected()
    } else {
        sqlx::query(
            "UPDATE phpyun_evaluate_group SET keyid=?, name=?, sort=?, description=?, \
             fromscore=?, toscore=?, comment=?, recommend=?, top=?, hot=? WHERE id=?",
        )
        .bind(w.keyid)
        .bind(w.name)
        .bind(w.sort)
        .bind(w.description)
        .bind(from)
        .bind(to)
        .bind(comment)
        .bind(w.recommend)
        .bind(w.top)
        .bind(w.hot)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected()
    };
    Ok(n)
}

pub async fn insert_group(pool: &MySqlPool, name: &str) -> Result<u64, sqlx::Error> {
    let exists: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM phpyun_evaluate_group WHERE name = ? AND keyid = 0 AND COALESCE(deleted,0)=0 LIMIT 1")
            .bind(name)
            .fetch_optional(pool)
            .await?;
    if exists.is_some() {
        return Ok(0);
    }
    let res = sqlx::query("INSERT INTO phpyun_evaluate_group (name, keyid, sort) VALUES (?, 0, 0)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn patch_group(
    pool: &MySqlPool,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_evaluate_group SET ");
    let mut any = false;
    if let Some(n) = name.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push("name = ");
        qb.push_bind(n);
        any = true;
    }
    if let Some(s) = sort {
        if any {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(s);
        any = true;
    }
    if !any {
        return Ok(0);
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND keyid = 0");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn paper_ids_in_group(pool: &MySqlPool, keyid: u64) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> =
        sqlx::query_as(&format!(
            "SELECT CAST(id AS UNSIGNED) FROM phpyun_evaluate_group WHERE keyid = ? AND {PREDICATE}"
        ))
            .bind(keyid)
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn delete_papers(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    soft_delete::mark_col_in(pool, "phpyun_evaluate", "gid", ids).await?;
    soft_delete::mark_col_in(pool, "phpyun_evaluate_leave_message", "examid", ids).await?;
    soft_delete::mark_col_in(pool, "phpyun_evaluate_log", "examid", ids).await?;
    soft_delete::mark_ids(pool, "phpyun_evaluate_group", ids).await
}

pub async fn list_questions(
    pool: &MySqlPool,
    paper_id: u64,
) -> Result<Vec<AdminEvalQuestion>, sqlx::Error> {
    sqlx::query_as::<_, AdminEvalQuestion>(
        "SELECT CAST(id AS UNSIGNED) AS id, \
                CAST(COALESCE(gid, 0) AS UNSIGNED) AS paper_id, \
                COALESCE(question, '') AS question, \
                COALESCE(`option`, '') AS `option`, \
                COALESCE(score, '') AS score, \
                CAST(COALESCE(sort, 0) AS SIGNED) AS sort \
         FROM phpyun_evaluate WHERE gid = ? AND COALESCE(deleted,0)=0 ORDER BY id ASC",
    )
    .bind(paper_id)
    .fetch_all(pool)
    .await
}

pub async fn upsert_question(
    pool: &MySqlPool,
    paper_id: u64,
    w: QuestionWrite<'_>,
) -> Result<u64, sqlx::Error> {
    let option = php_ser::serialize_strings(w.option);
    let score = php_ser::serialize_strings(w.score);
    if let Some(id) = w.id.filter(|v| *v > 0) {
        sqlx::query("UPDATE phpyun_evaluate SET question=?, `option`=?, score=? WHERE id=? AND gid=?")
            .bind(w.question)
            .bind(option)
            .bind(score)
            .bind(id)
            .bind(paper_id)
            .execute(pool)
            .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_evaluate (gid, question, `option`, score, sort) VALUES (?, ?, ?, ?, 0)",
    )
    .bind(paper_id)
    .bind(w.question)
    .bind(option)
    .bind(score)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_questions_notin(
    pool: &MySqlPool,
    paper_id: u64,
    keep: &[u64],
) -> Result<u64, sqlx::Error> {
    if keep.is_empty() {
        let res = sqlx::query(
            "UPDATE phpyun_evaluate SET deleted=1 WHERE gid = ? AND COALESCE(deleted,0)=0",
        )
        .bind(paper_id)
        .execute(pool)
        .await?;
        return Ok(res.rows_affected());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "UPDATE phpyun_evaluate SET deleted=1 WHERE gid = ",
    );
    qb.push_bind(paper_id);
    qb.push(" AND COALESCE(deleted,0)=0 AND id NOT IN (");
    let mut sep = qb.separated(", ");
    for id in keep {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_question(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_evaluate", id).await
}

pub async fn list_messages(
    pool: &MySqlPool,
    keyword: Option<&str>,
    by_uid: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminEvalMessage>, sqlx::Error> {
    let (lim, off) = bind_limit(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(m.id AS UNSIGNED) AS id, \
                CAST(COALESCE(m.examid, 0) AS UNSIGNED) AS examid, \
                COALESCE(m.uid, '') AS uid, \
                CAST(m.usertype AS SIGNED) AS usertype, \
                m.message, \
                CAST(COALESCE(m.ctime, 0) AS SIGNED) AS ctime, \
                COALESCE(mem.username, '') AS name \
         FROM phpyun_evaluate_leave_message m \
         LEFT JOIN phpyun_member mem ON mem.uid = CAST(m.uid AS UNSIGNED) \
         WHERE COALESCE(m.deleted,0)=0",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if by_uid {
            qb.push(" AND m.uid LIKE ");
        } else {
            qb.push(" AND m.message LIKE ");
        }
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY m.id DESC LIMIT ");
    qb.push_bind(lim);
    qb.push(" OFFSET ");
    qb.push_bind(off);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_messages(
    pool: &MySqlPool,
    keyword: Option<&str>,
    by_uid: bool,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!(
            "SELECT COUNT(*) FROM phpyun_evaluate_leave_message m WHERE {PREDICATE}"
        ));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if by_uid {
            qb.push(" AND m.uid LIKE ");
        } else {
            qb.push(" AND m.message LIKE ");
        }
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_messages(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_evaluate_leave_message", ids).await
}

pub async fn list_logs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    by_paper: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminEvalLog>, sqlx::Error> {
    let (lim, off) = bind_limit(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(l.id AS UNSIGNED) AS id, \
                CAST(COALESCE(l.uid, 0) AS UNSIGNED) AS uid, \
                CAST(COALESCE(l.examid, 0) AS UNSIGNED) AS examid, \
                CAST(COALESCE(l.grade, 0) AS SIGNED) AS grade, \
                CAST(COALESCE(l.ctime, 0) AS SIGNED) AS ctime, \
                CAST(COALESCE(l.usedsecond, 0) AS SIGNED) AS usedsecond, \
                COALESCE(mem.username, '') AS name, \
                COALESCE(g.name, '') AS title \
         FROM phpyun_evaluate_log l \
         LEFT JOIN phpyun_member mem ON mem.uid = l.uid \
         LEFT JOIN phpyun_evaluate_group g ON g.id = l.examid AND COALESCE(g.deleted,0)=0 \
         WHERE COALESCE(l.deleted,0)=0",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if by_paper {
            qb.push(
                " AND l.examid IN (SELECT id FROM phpyun_evaluate_group WHERE COALESCE(deleted,0)=0 AND name LIKE ",
            );
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
        } else if let Ok(uid) = kw.parse::<u64>() {
            qb.push(" AND l.uid = ");
            qb.push_bind(uid);
        }
    }
    qb.push(" ORDER BY l.id DESC LIMIT ");
    qb.push_bind(lim);
    qb.push(" OFFSET ");
    qb.push_bind(off);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_logs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    by_paper: bool,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!(
            "SELECT COUNT(*) FROM phpyun_evaluate_log l WHERE {PREDICATE}"
        ));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if by_paper {
            qb.push(
                " AND l.examid IN (SELECT id FROM phpyun_evaluate_group WHERE COALESCE(deleted,0)=0 AND name LIKE ",
            );
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
        } else if let Ok(uid) = kw.parse::<u64>() {
            qb.push(" AND l.uid = ");
            qb.push_bind(uid);
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_logs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_evaluate_log", ids).await
}
