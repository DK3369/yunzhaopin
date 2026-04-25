use super::entity::UsertypeChange;
use sqlx::MySqlPool;

const FIELDS: &str = "id, uid, usertype, applyusertype, applybody, status, ctime";

pub async fn find_latest_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<UsertypeChange>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_change WHERE uid = ? ORDER BY id DESC LIMIT 1"
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
    apply_usertype: i32,
    apply_body: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_change (uid, usertype, applyusertype, applybody, status, ctime)
         VALUES (?, ?, ?, ?, 1, ?)",
    )
    .bind(uid)
    .bind(usertype)
    .bind(apply_usertype)
    .bind(apply_body)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_status_admin(
    pool: &MySqlPool,
    id: u64,
    new_status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_change SET status = ? WHERE id = ? AND status = 1",
    )
    .bind(new_status)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<UsertypeChange>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_change WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, UsertypeChange>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}
