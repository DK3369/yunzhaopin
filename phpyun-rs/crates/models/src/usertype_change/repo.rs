//! `phpyun_change` repository — user-type switch applications.
//!
//! PHP schema (truth): `id, uid, username, usertype, name, gid, integral,
//! ctime, num, linktel, linkman, body, status, statusbody, express, expnum`.
//!
//! Mapping (Rust entity → PHP column):
//! - `usertype`       → `usertype`        (current type at apply-time)
//! - `applyusertype`  → no PHP column      (always 0; PHP doesn't track target)
//! - `applybody`      → `body`             (free-text reason)
//! - `ctime`          → `ctime`
//! - `status`         → `status`

use super::entity::UsertypeChange;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                             COALESCE(usertype, 0) AS usertype, \
                             0 AS applyusertype, \
                             body AS applybody, \
                             COALESCE(status, 0) AS status, \
                             COALESCE(ctime, 0) AS ctime";

pub async fn find_latest_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<UsertypeChange>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_change WHERE uid = ? ORDER BY id DESC LIMIT 1"
    );
    sqlx::query_as::<_, UsertypeChange>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    usertype: i32,
    _apply_usertype: i32,
    apply_body: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP table has no `applyusertype` column — silently dropped.
    let res = sqlx::query(
        "INSERT INTO phpyun_change (uid, usertype, body, status, ctime)
         VALUES (?, ?, ?, 1, ?)",
    )
    .bind(uid)
    .bind(usertype)
    .bind(apply_body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_status_admin<'e, E>(
    exec: E,
    id: u64,
    new_status: i32,
) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    let res = sqlx::query(
        "UPDATE phpyun_change SET status = ? WHERE id = ? AND status = 1",
    )
    .bind(new_status)
    .bind(id)
    .execute(exec)
    .await?;
    Ok(res.rows_affected())
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<UsertypeChange>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_change WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, UsertypeChange>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}
