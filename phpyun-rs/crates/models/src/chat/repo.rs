use super::entity::{conv_key_for, Chat};
use sqlx::MySqlPool;

pub async fn send(
    pool: &MySqlPool,
    sender_uid: u64,
    receiver_uid: u64,
    body: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let conv_key = conv_key_for(sender_uid, receiver_uid);
    let res = sqlx::query(
        r#"INSERT INTO phpyun_rs_chat
           (sender_uid, receiver_uid, conv_key, body, is_read, created_at)
           VALUES (?, ?, ?, ?, 0, ?)"#,
    )
    .bind(sender_uid)
    .bind(receiver_uid)
    .bind(&conv_key)
    .bind(body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Fetch the most recent N messages between two users (ordered by id DESC).
pub async fn list_with_peer(
    pool: &MySqlPool,
    self_uid: u64,
    peer_uid: u64,
    before_id: Option<u64>,
    limit: u64,
) -> Result<Vec<Chat>, sqlx::Error> {
    let conv_key = conv_key_for(self_uid, peer_uid);
    let row_limit = limit.clamp(1, 200);
    if let Some(before) = before_id {
        sqlx::query_as::<_, Chat>(
            r#"SELECT id, sender_uid, receiver_uid, conv_key, body, is_read, created_at
               FROM phpyun_rs_chat
               WHERE conv_key = ? AND id < ?
               ORDER BY id DESC LIMIT ?"#,
        )
        .bind(conv_key)
        .bind(before)
        .bind(row_limit)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, Chat>(
            r#"SELECT id, sender_uid, receiver_uid, conv_key, body, is_read, created_at
               FROM phpyun_rs_chat
               WHERE conv_key = ?
               ORDER BY id DESC LIMIT ?"#,
        )
        .bind(conv_key)
        .bind(row_limit)
        .fetch_all(pool)
        .await
    }
}

/// Latest message of each of my conversations (aggregated by conv_key).
pub async fn list_conversations(
    pool: &MySqlPool,
    self_uid: u64,
    limit: u64,
) -> Result<Vec<Chat>, sqlx::Error> {
    let row_limit = limit.clamp(1, 100);
    // For each conv_key, fetch the row with the largest id.
    sqlx::query_as::<_, Chat>(
        r#"SELECT c.id, c.sender_uid, c.receiver_uid, c.conv_key, c.body, c.is_read, c.created_at
           FROM phpyun_rs_chat c
           INNER JOIN (
             SELECT conv_key, MAX(id) AS max_id
             FROM phpyun_rs_chat
             WHERE sender_uid = ? OR receiver_uid = ?
             GROUP BY conv_key
           ) t ON c.id = t.max_id
           ORDER BY c.id DESC LIMIT ?"#,
    )
    .bind(self_uid)
    .bind(self_uid)
    .bind(row_limit)
    .fetch_all(pool)
    .await
}

pub async fn mark_read_from_peer(
    pool: &MySqlPool,
    self_uid: u64,
    peer_uid: u64,
) -> Result<u64, sqlx::Error> {
    let conv_key = conv_key_for(self_uid, peer_uid);
    let res = sqlx::query(
        r#"UPDATE phpyun_rs_chat SET is_read = 1
           WHERE conv_key = ? AND receiver_uid = ? AND is_read = 0"#,
    )
    .bind(conv_key)
    .bind(self_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn count_unread(pool: &MySqlPool, self_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_rs_chat WHERE receiver_uid = ? AND is_read = 0",
    )
    .bind(self_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
