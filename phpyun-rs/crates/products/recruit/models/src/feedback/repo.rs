//! `phpyun_advice_question` repository — PHPYun's actual feedback queue.
//!
//! Schema (PHP truth): `id, username, infotype, content, mobile, ctime,
//! email, handlecontent, status`. The Rust `Feedback` entity uses
//! `category / contact / client_ip / created_at` shape; this repo maps via
//! SELECT aliases.
//!
//! Caveats:
//! - PHP has no `uid` column → `list_by_user` returns an empty list because
//!   we cannot scope feedback to a specific user from this table alone.
//!   The user-side "my feedback" page therefore reads as empty until the
//!   product wires a `username` filter through.
//! - PHP has no `client_ip` column → exposed as empty string.

use super::entity::Feedback;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, QueryBuilder};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AdviceAdminRow {
    pub id: u64,
    pub username: String,
    pub infotype: i32,
    pub content: String,
    pub mobile: String,
    pub email: String,
    pub handlecontent: String,
    pub status: i32,
    pub ctime: i64,
}

const PHP_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
    COALESCE(username,'') AS username, \
    COALESCE(infotype,0) AS infotype, \
    COALESCE(content,'') AS content, \
    COALESCE(mobile,'') AS mobile, \
    COALESCE(email,'') AS email, \
    COALESCE(handlecontent,'') AS handlecontent, \
    COALESCE(status,1) AS status, \
    CAST(COALESCE(ctime,0) AS SIGNED) AS ctime";

// SELECT aliases map PHP cols onto the Rust `Feedback` entity field names.
const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             NULL AS uid, \
                             COALESCE(CAST(infotype AS CHAR), '') AS category, \
                             COALESCE(content, '') AS content, \
                             COALESCE(mobile, '') AS contact, \
                             '' AS client_ip, \
                             status, \
                             COALESCE(ctime, 0) AS created_at";

pub struct FeedbackCreate<'a> {
    pub uid: Option<u64>,
    pub username: &'a str,
    pub category: &'a str,
    pub content: &'a str,
    pub contact: &'a str,
    pub client_ip: &'a str,
}

fn parse_infotype(raw: &str) -> i32 {
    let t = raw.trim();
    if let Ok(n) = t.parse::<i32>() {
        if (1..=4).contains(&n) {
            return n;
        }
    }
    match t.to_ascii_lowercase().as_str() {
        "advice" | "suggest" | "suggestion" => 1,
        "job" | "bug" | "complaint" => 2,
        "resume" => 3,
        "other" => 4,
        _ => 1,
    }
}

pub async fn create(pool: &MySqlPool, c: FeedbackCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let _ = (c.uid, c.client_ip);
    let infotype = parse_infotype(c.category);
    let res = sqlx::query(
        r#"INSERT INTO phpyun_advice_question
           (username, infotype, content, mobile, ctime, status)
           VALUES (?, ?, ?, ?, ?, 1)"#,
    )
    .bind(c.username)
    .bind(infotype)
    .bind(c.content)
    .bind(c.contact)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// User-side "my feedback" list. PHP table has no `uid` column, so this
/// always returns empty (handler shape preserved).
pub async fn list_by_user(
    _pool: &MySqlPool,
    _uid: u64,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<Feedback>, sqlx::Error> {
    Ok(Vec::new())
}

pub async fn count_by_user(_pool: &MySqlPool, _uid: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}

/// Admin view: paginated list (status=None means all).
pub async fn list_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Feedback>, sqlx::Error> {
    let sql = match status {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_advice_question \
             WHERE status = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_advice_question \
             ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Feedback>(&sql);
    let q = match status {
        Some(s) => q.bind(s).bind(limit).bind(offset),
        None => q.bind(limit).bind(offset),
    };
    q.fetch_all(pool).await
}

pub async fn count_by_status(pool: &MySqlPool, status: Option<i32>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match status {
        Some(s) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_advice_question WHERE status = ?")
                .bind(s)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_advice_question")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_advice_question SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Default, Clone)]
pub struct AdviceAdminFilter<'a> {
    pub keyword: &'a str,
    /// PHP `type`: 1=username, otherwise content.
    pub keyword_type: &'a str,
    pub infotype: Option<i32>,
    pub status: Option<i32>,
    pub ctime_gte: Option<i64>,
    pub order_col: &'a str,
    pub order_dir: &'a str,
}

fn order_clause(col: &str, dir: &str) -> &'static str {
    let asc = dir.eq_ignore_ascii_case("asc");
    match col {
        "id" => {
            if asc {
                "id ASC"
            } else {
                "id DESC"
            }
        }
        "ctime" => {
            if asc {
                "ctime ASC, id ASC"
            } else {
                "ctime DESC, id DESC"
            }
        }
        "infotype" => {
            if asc {
                "infotype ASC, id DESC"
            } else {
                "infotype DESC, id DESC"
            }
        }
        "status" => {
            if asc {
                "status ASC, id DESC"
            } else {
                "status DESC, id DESC"
            }
        }
        _ => "id DESC",
    }
}

fn push_advice_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &AdviceAdminFilter<'_>) {
    qb.push(" WHERE 1=1");
    if !f.keyword.is_empty() {
        let like = format!("%{}%", f.keyword);
        if f.keyword_type == "1" || f.keyword_type.is_empty() {
            qb.push(" AND username LIKE ").push_bind(like);
        } else {
            qb.push(" AND content LIKE ").push_bind(like);
        }
    }
    if let Some(t) = f.infotype {
        qb.push(" AND infotype = ").push_bind(t);
    }
    if let Some(s) = f.status {
        if s == 1 {
            qb.push(" AND status IN (0, 1)");
        } else {
            qb.push(" AND status = ").push_bind(s);
        }
    }
    if let Some(ts) = f.ctime_gte {
        qb.push(" AND ctime >= ").push_bind(ts);
    }
}

pub async fn admin_php_list(
    pool: &MySqlPool,
    f: &AdviceAdminFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdviceAdminRow>, sqlx::Error> {
    let order = order_clause(f.order_col, f.order_dir);
    let mut qb = QueryBuilder::new(format!("SELECT {PHP_FIELDS} FROM phpyun_advice_question"));
    push_advice_where(&mut qb, f);
    qb.push(" ORDER BY ");
    qb.push(order);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<AdviceAdminRow>().fetch_all(pool).await
}

pub async fn admin_php_count(pool: &MySqlPool, f: &AdviceAdminFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_advice_question");
    push_advice_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_status_handle(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    handlecontent: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_advice_question SET status = ?, handlecontent = ? WHERE id = ?",
    )
    .bind(status)
    .bind(handlecontent)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_advice_question WHERE id IN (");
    {
        let mut sep = qb.separated(",");
        for id in ids {
            sep.push_bind(*id);
        }
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
