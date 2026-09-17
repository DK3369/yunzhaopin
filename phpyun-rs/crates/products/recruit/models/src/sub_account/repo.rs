//! Classic company sub-accounts: `phpyun_member.pid` = parent company uid.

use super::entity::SubAccountRow;
use sqlx::MySqlPool;

pub async fn list_by_parent(pool: &MySqlPool, parent_uid: u64) -> Result<Vec<SubAccountRow>, sqlx::Error> {
    sqlx::query_as::<_, SubAccountRow>(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         CAST(COALESCE(status,0) AS SIGNED) AS status, \
         CAST(COALESCE(login_date,0) AS SIGNED) AS login_date \
         FROM phpyun_member WHERE pid = ? AND usertype = 2 ORDER BY uid DESC",
    )
    .bind(parent_uid)
    .fetch_all(pool)
    .await
}

pub async fn find_owned(
    pool: &MySqlPool,
    parent_uid: u64,
    uid: u64,
) -> Result<Option<SubAccountRow>, sqlx::Error> {
    sqlx::query_as::<_, SubAccountRow>(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         CAST(COALESCE(status,0) AS SIGNED) AS status, \
         CAST(COALESCE(login_date,0) AS SIGNED) AS login_date \
         FROM phpyun_member WHERE uid = ? AND pid = ? AND usertype = 2 LIMIT 1",
    )
    .bind(uid)
    .bind(parent_uid)
    .fetch_optional(pool)
    .await
}

pub async fn insert(
    pool: &MySqlPool,
    parent_uid: u64,
    parent_did: u32,
    username: &str,
    password_hash: &str,
    salt: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_member \
            (username, password, salt, moblie, email, usertype, status, did, pid, reg_date, reg_ip, login_date) \
         VALUES (?, ?, ?, '', '', 2, 1, ?, ?, ?, '', ?)",
    )
    .bind(username)
    .bind(password_hash)
    .bind(salt)
    .bind(parent_did)
    .bind(parent_uid)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_status(
    pool: &MySqlPool,
    parent_uid: u64,
    uid: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member SET status = ? WHERE uid = ? AND pid = ? AND usertype = 2",
    )
    .bind(status)
    .bind(uid)
    .bind(parent_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_owned(pool: &MySqlPool, parent_uid: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_member WHERE uid = ? AND pid = ? AND usertype = 2")
        .bind(uid)
        .bind(parent_uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
