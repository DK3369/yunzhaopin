use super::entity::Feedback;
use sqlx::MySqlPool;

const FIELDS: &str = "id, uid, category, content, contact, client_ip, status, created_at";

pub struct FeedbackCreate<'a> {
    pub uid: Option<u64>,
    pub category: &'a str,
    pub content: &'a str,
    pub contact: &'a str,
    pub client_ip: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    c: FeedbackCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_advice_question
           (uid, category, content, contact, client_ip, status, created_at)
           VALUES (?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.uid)
    .bind(c.category)
    .bind(c.content)
    .bind(c.contact)
    .bind(c.client_ip)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Feedback>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_advice_question
         WHERE uid = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Feedback>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_advice_question WHERE uid = ?",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
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
            "SELECT {FIELDS} FROM phpyun_advice_question
             WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_advice_question
             ORDER BY created_at DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Feedback>(&sql);
    let q = match status {
        Some(s) => q.bind(s).bind(limit).bind(offset),
        None => q.bind(limit).bind(offset),
    };
    q.fetch_all(pool).await
}

pub async fn count_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
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
    Ok(n.max(0) as u64)
}

pub async fn set_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_advice_question SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
