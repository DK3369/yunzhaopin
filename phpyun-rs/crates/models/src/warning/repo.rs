use super::entity::Warning;
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, target_uid, target_kind, target_id, reason, is_read, issuer_uid, created_at";

pub struct WarnCreate<'a> {
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason: &'a str,
    pub issuer_uid: u64,
}

pub async fn create(pool: &MySqlPool, c: WarnCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_warning
           (target_uid, target_kind, target_id, reason, is_read, issuer_uid, created_at)
           VALUES (?, ?, ?, ?, 0, ?, ?)"#,
    )
    .bind(c.target_uid)
    .bind(c.target_kind)
    .bind(c.target_id)
    .bind(c.reason)
    .bind(c.issuer_uid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_mine(
    pool: &MySqlPool,
    target_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Warning>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_warning
         WHERE target_uid = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Warning>(&sql)
        .bind(target_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_mine(pool: &MySqlPool, target_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning WHERE target_uid = ?")
            .bind(target_uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn count_unread(pool: &MySqlPool, target_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_warning WHERE target_uid = ? AND is_read = 0",
    )
    .bind(target_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn mark_read(pool: &MySqlPool, id: u64, target_uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_warning SET is_read = 1 WHERE id = ? AND target_uid = ? AND is_read = 0",
    )
    .bind(id)
    .bind(target_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn admin_list(
    pool: &MySqlPool,
    kind: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Warning>, sqlx::Error> {
    let sql = match kind {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_warning
             WHERE target_kind = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_warning
             ORDER BY created_at DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Warning>(&sql);
    match kind {
        Some(k) => q.bind(k).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn admin_count(pool: &MySqlPool, kind: Option<i32>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match kind {
        Some(k) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning WHERE target_kind = ?")
                .bind(k)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}
