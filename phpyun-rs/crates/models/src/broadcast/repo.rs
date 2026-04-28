//! `phpyun_admin_announcement` repository — site-wide admin announcements.
//!
//! PHP schema (truth): `id, title, keyword, description, content, datetime,
//! startime, endtime, did, view_num`.
//!
//! Mapping (Rust `Broadcast` entity → PHP column):
//! - `title`           → `title`
//! - `body`            → `content`
//! - `target_usertype` → no PHP column     (always 0 = visible to all)
//! - `status`          → no PHP column     (always 1 = active; PHP relies on
//!   startime/endtime windowing instead)
//! - `issuer_uid`      → no PHP column     (always 0)
//! - `created_at`      → `datetime`
//!
//! Per-user read-receipts live in `phpyun_rs_broadcast_reads` (added in
//! migration `20260428000001` because PHP has no concept of read receipts
//! for site-wide announcements).

use super::entity::Broadcast;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             COALESCE(title, '') AS title, \
                             COALESCE(content, '') AS body, \
                             CAST(0 AS SIGNED) AS target_usertype, \
                             CAST(1 AS SIGNED) AS status, \
                             CAST(0 AS UNSIGNED) AS issuer_uid, \
                             COALESCE(datetime, 0) AS created_at";

pub async fn create(
    pool: &MySqlPool,
    title: &str,
    body: &str,
    _target_usertype: i32,
    _issuer_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP `phpyun_admin_announcement` requires `keyword` and `description`
    // (NOT NULL); fill with empty strings. startime/endtime keep the row
    // visible immediately and indefinitely (0 sentinel).
    let res = sqlx::query(
        r#"INSERT INTO phpyun_admin_announcement
           (title, keyword, description, content, datetime, startime, endtime)
           VALUES (?, '', '', ?, ?, ?, 0)"#,
    )
    .bind(title)
    .bind(body)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_admin_announcement WHERE id = ?")
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
        "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
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
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_announcement")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// Fetch active announcements visible during `(startime, endtime)`. The
/// `target_usertype` filter is dropped because PHP has no equivalent column.
pub async fn list_for_user(
    pool: &MySqlPool,
    _usertype: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Broadcast>, sqlx::Error> {
    let now = phpyun_core::clock::now_ts();
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
         WHERE (startime = 0 OR startime <= ?) \
           AND (endtime = 0 OR endtime > ?) \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Broadcast>(&sql)
        .bind(now)
        .bind(now)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_user(pool: &MySqlPool, _usertype: i32) -> Result<u64, sqlx::Error> {
    let now = phpyun_core::clock::now_ts();
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_admin_announcement \
         WHERE (startime = 0 OR startime <= ?) \
           AND (endtime = 0 OR endtime > ?)",
    )
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Count of unread announcements (active − already read).
///
/// When `phpyun_rs_broadcast_reads` is missing (host PHP install lacks the
/// Rust-port read-receipt table) we treat every active announcement as
/// "unread" — the same behaviour as a brand-new user — instead of 5xx'ing.
pub async fn count_unread(pool: &MySqlPool, uid: u64, _usertype: i32) -> Result<u64, sqlx::Error> {
    let now = phpyun_core::clock::now_ts();
    let res: Result<(i64,), _> = sqlx::query_as(
        r#"SELECT COUNT(*) FROM phpyun_admin_announcement b
           WHERE (b.startime = 0 OR b.startime <= ?)
             AND (b.endtime = 0 OR b.endtime > ?)
             AND NOT EXISTS (
               SELECT 1 FROM phpyun_rs_broadcast_reads r
               WHERE r.uid = ? AND r.broadcast_id = b.id
             )"#,
    )
    .bind(now)
    .bind(now)
    .bind(uid)
    .fetch_one(pool)
    .await;
    match res {
        Ok((n,)) => Ok(n.max(0) as u64),
        Err(e) if phpyun_core::db::is_missing_table(&e) => {
            // Fall back to total active count: every visible announcement is
            // considered unread when read receipts can't be tracked.
            count_for_user(pool, 0).await
        }
        Err(e) => Err(e),
    }
}

pub async fn mark_read(
    pool: &MySqlPool,
    uid: u64,
    broadcast_id: u64,
    now: i64,
) -> Result<(), sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT IGNORE INTO phpyun_rs_broadcast_reads (uid, broadcast_id, read_at)
           VALUES (?, ?, ?)"#,
    )
    .bind(uid)
    .bind(broadcast_id)
    .bind(now)
    .execute(pool)
    .await;
    match res {
        Ok(_) => Ok(()),
        // Read receipts are best-effort when the host install lacks the table.
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(()),
        Err(e) => Err(e),
    }
}
