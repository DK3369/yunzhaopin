//! `phpyun_recycle` repository — admin recycle-bin / soft-delete buffer.
//!
//! PHP schema (truth): `id, uid, username, tablename, body, ctime, ident, uri`.
//!
//! Mapping (Rust entity → PHP column):
//! - `tablename`  → `tablename`
//! - `row_id`     → `ident`  (PHP stores it as text; Rust `u64` cast on read)
//! - `body`       → `body`
//! - `actor_uid`  → `uid`
//! - `note`       → `uri`    (closest free-text field PHP has)
//! - `created_at` → `ctime`

use super::entity::RecycleEntry;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             COALESCE(tablename, '') AS tablename, \
                             CAST(COALESCE(NULLIF(ident, ''), '0') AS UNSIGNED) AS row_id, \
                             COALESCE(body, '') AS body, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS actor_uid, \
                             COALESCE(uri, '') AS note, \
                             COALESCE(ctime, 0) AS created_at";

pub async fn insert(
    pool: &MySqlPool,
    tablename: &str,
    row_id: u64,
    body_json: &str,
    actor_uid: u64,
    note: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_recycle
           (uid, tablename, body, ctime, ident, uri)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(actor_uid)
    .bind(tablename)
    .bind(body_json)
    .bind(now)
    .bind(row_id.to_string())
    .bind(note)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn get(pool: &MySqlPool, id: u64) -> Result<Option<RecycleEntry>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_recycle WHERE id = ?");
    sqlx::query_as::<_, RecycleEntry>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(
    pool: &MySqlPool,
    tablename: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<RecycleEntry>, sqlx::Error> {
    let sql = match tablename {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_recycle \
             WHERE tablename = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_recycle \
             ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, RecycleEntry>(&sql);
    match tablename {
        Some(t) => q.bind(t).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count(pool: &MySqlPool, tablename: Option<&str>) -> Result<u64, sqlx::Error> {
    let n: (i64,) = match tablename {
        Some(t) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_recycle WHERE tablename = ?")
                .bind(t)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_recycle")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.0.max(0) as u64)
}

pub async fn delete_by_id(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_recycle WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Purge entries older than `cutoff` (epoch seconds).
pub async fn purge_older_than(pool: &MySqlPool, cutoff: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_recycle WHERE ctime < ?")
        .bind(cutoff)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
