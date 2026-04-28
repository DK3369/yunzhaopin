use super::entity::MemberLogout;
use sqlx::MySqlPool;

// `phpyun_member_logout` marks uid/status/ctime nullable; `MemberLogout`
// uses plain `u64 / i32 / i64`. COALESCE NULLs to defaults.
const FIELDS: &str = "id, \
    COALESCE(uid, 0) AS uid, \
    username, tel, \
    COALESCE(status, 0) AS status, \
    COALESCE(ctime, 0) AS ctime";

pub async fn find_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<MemberLogout>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_member_logout WHERE uid = ? ORDER BY id DESC LIMIT 1"
    );
    sqlx::query_as::<_, MemberLogout>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    username: &str,
    tel: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_member_logout (uid, username, tel, status, ctime) VALUES (?, ?, ?, 1, ?)",
    )
    .bind(uid)
    .bind(username)
    .bind(tel.unwrap_or(""))
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Admin action: approve deletion (status=2 = completed).
pub async fn approve(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member_logout SET status = 2 WHERE id = ? AND status = 1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Admin action: reject deletion (status=3).
pub async fn reject(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member_logout SET status = 3 WHERE id = ? AND status = 1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
