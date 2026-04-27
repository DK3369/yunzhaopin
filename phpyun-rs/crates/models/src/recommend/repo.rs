//! `phpyun_recommend` repository.
//!
//! Quota helpers (`count_today_by_user`, `last_addtime_by_user`) drive the
//! same throttles PHP enforces in `resumeshare::index_action`:
//!   - per-day cap (`sy_recommend_day_num`)
//!   - min interval between sends (`sy_recommend_interval`)

use super::entity::RecommendLog;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(uid AS UNSIGNED) AS uid, \
    CAST(COALESCE(rec_type, 0) AS SIGNED) AS rec_type, \
    CAST(COALESCE(rec_id, 0) AS UNSIGNED) AS rec_id, \
    COALESCE(email, '') AS email, \
    CAST(COALESCE(addtime, 0) AS SIGNED) AS addtime";

pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    rec_type: i32,
    rec_id: u64,
    email: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_recommend (uid, rec_type, rec_id, email, addtime) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(rec_type)
    .bind(rec_id)
    .bind(email)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn count_today_by_user(
    pool: &MySqlPool,
    uid: u64,
    day_start_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_recommend WHERE uid = ? AND addtime >= ?",
    )
    .bind(uid)
    .bind(day_start_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn last_addtime_by_user(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(addtime, 0) AS SIGNED) FROM phpyun_recommend \
         WHERE uid = ? ORDER BY addtime DESC, id DESC LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(t,)| t))
}

#[allow(dead_code)]
pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<RecommendLog>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_recommend \
         WHERE uid = ? ORDER BY addtime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, RecommendLog>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}
