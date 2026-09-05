//! `phpyun_finder` — PHP member searcher (`c=finder`).
//!
//! `para` is `key=value##key=value` (not JSON). Mail subscribe stays on
//! `phpyun_subscribe` / saved-searches.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Finder {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub usertype: i32,
    pub name: String,
    pub para: String,
    pub addtime: i64,
}

const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
    COALESCE(usertype,0) AS usertype, COALESCE(name,'') AS name, COALESCE(para,'') AS para, \
    CAST(COALESCE(addtime,0) AS SIGNED) AS addtime";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Finder>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_finder WHERE uid = ? ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Finder>(&sql)
        .bind(uid)
        .bind(phpyun_core::numeric::checked_db_i64(
            limit,
            "pagination.limit",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            offset,
            "pagination.offset",
        )?)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_finder WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    usertype: i32,
    name: &str,
    para: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_finder (uid, usertype, name, para, addtime) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(usertype)
    .bind(name)
    .bind(para)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_by_uid(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_finder WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
