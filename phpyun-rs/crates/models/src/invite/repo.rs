use super::entity::Invite;
use sqlx::MySqlPool;

pub struct InviteCreate<'a> {
    pub inviter_uid: u64,
    pub email: &'a str,
    pub subject: &'a str,
    pub content: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    c: InviteCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (inviter_uid, email, subject, content, status, created_at)
           VALUES (?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.inviter_uid)
    .bind(c.email)
    .bind(c.subject)
    .bind(c.content)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Number of invitations sent today (by inviter_uid).
pub async fn count_today_by_user(
    pool: &MySqlPool,
    uid: u64,
    today_start_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_yqmb WHERE inviter_uid = ? AND created_at >= ?",
    )
    .bind(uid)
    .bind(today_start_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Invite>, sqlx::Error> {
    sqlx::query_as::<_, Invite>(
        r#"SELECT id, inviter_uid, email, subject, content, status, created_at
           FROM phpyun_yqmb
           WHERE inviter_uid = ?
           ORDER BY created_at DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_yqmb WHERE inviter_uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}
