use super::entity::Broadcast;
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, title, body, target_usertype, status, issuer_uid, created_at";

pub async fn create(
    pool: &MySqlPool,
    title: &str,
    body: &str,
    target_usertype: i32,
    issuer_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_broadcast
           (title, body, target_usertype, status, issuer_uid, created_at)
           VALUES (?, ?, ?, 1, ?, ?)"#,
    )
    .bind(title)
    .bind(body)
    .bind(target_usertype)
    .bind(issuer_uid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_broadcast WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn admin_list(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Broadcast>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_broadcast
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Broadcast>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn admin_count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_broadcast")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// Fetch active broadcasts targeting `usertype` (target_usertype=0 matches everyone).
pub async fn list_for_user(
    pool: &MySqlPool,
    usertype: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Broadcast>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_broadcast
         WHERE status = 1 AND (target_usertype = 0 OR target_usertype = ?)
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Broadcast>(&sql)
        .bind(usertype)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_user(pool: &MySqlPool, usertype: i32) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_broadcast
         WHERE status = 1 AND (target_usertype = 0 OR target_usertype = ?)",
    )
    .bind(usertype)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Count of unread broadcasts (active - already read).
pub async fn count_unread(pool: &MySqlPool, uid: u64, usertype: i32) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM phpyun_broadcast b
           WHERE b.status = 1
             AND (b.target_usertype = 0 OR b.target_usertype = ?)
             AND NOT EXISTS (
               SELECT 1 FROM phpyun_rs_broadcast_reads r
               WHERE r.uid = ? AND r.broadcast_id = b.id
             )"#,
    )
    .bind(usertype)
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn mark_read(
    pool: &MySqlPool,
    uid: u64,
    broadcast_id: u64,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT IGNORE INTO phpyun_rs_broadcast_reads (uid, broadcast_id, read_at)
           VALUES (?, ?, ?)"#,
    )
    .bind(uid)
    .bind(broadcast_id)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
