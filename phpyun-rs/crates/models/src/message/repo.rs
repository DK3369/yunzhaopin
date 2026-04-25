use super::entity::Message;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str =
    "id, uid, category, title, body, ref_kind, ref_id, is_read, created_at";

pub struct MessageCreate<'a> {
    pub uid: u64,
    pub category: &'a str,
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub ref_kind: i32,
    pub ref_id: u64,
}

pub async fn create(
    pool: &MySqlPool,
    c: MessageCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_msg
           (uid, category, title, body, ref_kind, ref_id, is_read, created_at)
           VALUES (?, ?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.uid)
    .bind(c.category)
    .bind(c.title)
    .bind(c.body)
    .bind(c.ref_kind)
    .bind(c.ref_id)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list(
    pool: &MySqlPool,
    uid: u64,
    category: Option<&str>,
    unread_only: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Message>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_msg WHERE uid = ");
    qb.push_bind(uid);
    if let Some(c) = category {
        qb.push(" AND category = ");
        qb.push_bind(c);
    }
    if unread_only {
        qb.push(" AND is_read = 0");
    }
    qb.push(" ORDER BY created_at DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Message>().fetch_all(pool).await
}

pub async fn count(
    pool: &MySqlPool,
    uid: u64,
    category: Option<&str>,
    unread_only: bool,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_msg WHERE uid = ");
    qb.push_bind(uid);
    if let Some(c) = category {
        qb.push(" AND category = ");
        qb.push_bind(c);
    }
    if unread_only {
        qb.push(" AND is_read = 0");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn mark_read(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_msg SET is_read = 1 WHERE id = ? AND uid = ? AND is_read = 0",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn mark_all_read(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_msg SET is_read = 1 WHERE uid = ? AND is_read = 0",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_msg WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
