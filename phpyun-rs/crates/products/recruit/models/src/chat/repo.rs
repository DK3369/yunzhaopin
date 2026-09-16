//! `phpyun_rs_chat` — Rust-only peer-to-peer messages (not PHP `chat_log`).

use super::entity::{ChatMessage, ConversationPreview, MemberNameRow};
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
     CAST(sender_uid AS UNSIGNED) AS sender_uid, \
     CAST(receiver_uid AS UNSIGNED) AS receiver_uid, \
     conv_key, \
     body, \
     CAST(is_read AS SIGNED) AS is_read, \
     CAST(created_at AS SIGNED) AS created_at";

/// Canonical conversation key shared by both directions.
pub fn conv_key(a: u64, b: u64) -> String {
    if a <= b {
        format!("{a}-{b}")
    } else {
        format!("{b}-{a}")
    }
}

pub async fn insert(
    pool: &MySqlPool,
    sender_uid: u64,
    receiver_uid: u64,
    body: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let key = conv_key(sender_uid, receiver_uid);
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_chat \
            (sender_uid, receiver_uid, conv_key, body, is_read, created_at) \
         VALUES (?, ?, ?, ?, 0, ?)",
    )
    .bind(sender_uid)
    .bind(receiver_uid)
    .bind(&key)
    .bind(body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Newest-first; caller reverses for chronological UI. `before_id=0` means no cursor.
pub async fn list_with(
    pool: &MySqlPool,
    key: &str,
    before_id: u64,
    limit: u64,
) -> Result<Vec<ChatMessage>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_rs_chat \
         WHERE conv_key = ? AND (? = 0 OR id < ?) \
         ORDER BY id DESC LIMIT ?"
    );
    let r = sqlx::query_as::<_, ChatMessage>(&sql)
        .bind(key)
        .bind(before_id)
        .bind(before_id)
        .bind(limit)
        .fetch_all(pool)
        .await;
    phpyun_core::db::ok_default_if_object_missing(r)
}

pub async fn mark_read_from_peer(
    pool: &MySqlPool,
    me: u64,
    peer: u64,
) -> Result<u64, sqlx::Error> {
    let key = conv_key(me, peer);
    let res = sqlx::query(
        "UPDATE phpyun_rs_chat SET is_read = 1 \
         WHERE conv_key = ? AND receiver_uid = ? AND sender_uid = ? AND is_read = 0",
    )
    .bind(&key)
    .bind(me)
    .bind(peer)
    .execute(pool)
    .await;
    match res {
        Ok(r) => Ok(r.rows_affected()),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(0),
        Err(e) => Err(e),
    }
}

pub async fn count_unread(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let r = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM phpyun_rs_chat WHERE receiver_uid = ? AND is_read = 0",
    )
    .bind(uid)
    .fetch_one(pool)
    .await;
    match r {
        Ok((n,)) => Ok(phpyun_core::numeric::nonnegative_count(n)),
        Err(e) => phpyun_core::db::ok_default_if_object_missing(Err(e)),
    }
}

pub async fn list_conversations(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ConversationPreview>, sqlx::Error> {
    let sql = "SELECT \
            CAST(c.id AS UNSIGNED) AS last_id, \
            CAST(IF(c.sender_uid = ?, c.receiver_uid, c.sender_uid) AS UNSIGNED) AS peer_uid, \
            CAST(c.sender_uid AS UNSIGNED) AS last_sender_uid, \
            c.body AS last_body, \
            CAST(c.created_at AS SIGNED) AS last_at, \
            CAST(( \
                SELECT COUNT(*) FROM phpyun_rs_chat u \
                WHERE u.conv_key = c.conv_key AND u.receiver_uid = ? AND u.is_read = 0 \
            ) AS UNSIGNED) AS unread \
         FROM phpyun_rs_chat c \
         INNER JOIN ( \
            SELECT conv_key, MAX(id) AS max_id \
            FROM phpyun_rs_chat \
            WHERE sender_uid = ? OR receiver_uid = ? \
            GROUP BY conv_key \
         ) t ON t.max_id = c.id \
         ORDER BY c.id DESC LIMIT ? OFFSET ?";
    let r = sqlx::query_as::<_, ConversationPreview>(sql)
        .bind(uid)
        .bind(uid)
        .bind(uid)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;
    phpyun_core::db::ok_default_if_object_missing(r)
}

pub async fn count_conversations(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let r = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(DISTINCT conv_key) FROM phpyun_rs_chat \
         WHERE sender_uid = ? OR receiver_uid = ?",
    )
    .bind(uid)
    .bind(uid)
    .fetch_one(pool)
    .await;
    match r {
        Ok((n,)) => Ok(phpyun_core::numeric::nonnegative_count(n)),
        Err(e) => phpyun_core::db::ok_default_if_object_missing(Err(e)),
    }
}

pub async fn list_member_names(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<MemberNameRow>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, \
                COALESCE(username, '') AS username, \
                CAST(COALESCE(usertype, 0) AS SIGNED) AS usertype \
         FROM phpyun_member WHERE uid IN (",
    );
    let mut sep = qb.separated(',');
    for id in uids.iter().take(200) {
        sep.push_bind(*id);
    }
    sep.push_unseparated(") LIMIT 200");
    qb.build_query_as::<MemberNameRow>().fetch_all(pool).await
}

#[cfg(test)]
mod tests {
    use super::conv_key;

    #[test]
    fn conv_key_orders_pair() {
        assert_eq!(conv_key(99, 1), "1-99");
        assert_eq!(conv_key(1, 99), "1-99");
        assert_eq!(conv_key(5, 5), "5-5");
    }
}
