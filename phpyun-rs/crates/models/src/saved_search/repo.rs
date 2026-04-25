use super::entity::SavedSearch;
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, uid, name, kind, params, notify, last_notified_at, created_at, updated_at";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<SavedSearch>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_subscribe
         WHERE uid = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, SavedSearch>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_subscribe WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    kind: &str,
    params: &serde_json::Value,
    notify: bool,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_subscribe
           (uid, name, kind, params, notify, last_notified_at, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, 0, ?, ?)"#,
    )
    .bind(uid)
    .bind(name)
    .bind(kind)
    .bind(params)
    .bind(if notify { 1i8 } else { 0 })
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_notify(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    notify: bool,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_subscribe SET notify = ?, updated_at = ?
         WHERE id = ? AND uid = ?",
    )
    .bind(if notify { 1i8 } else { 0 })
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("DELETE FROM phpyun_subscribe WHERE id = ? AND uid = ?")
            .bind(id)
            .bind(uid)
            .execute(pool)
            .await?;
    Ok(res.rows_affected())
}
