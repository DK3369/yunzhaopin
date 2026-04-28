//! `phpyun_banner` repository — company home page banner images.
//!
//! PHP schema (truth): `id, uid, pic, status, statusbody, did`.
//! Rust entity has `link, sort, addtime` which PHP doesn't store; those are
//! exposed as empty/0 defaults.

use super::entity::CompanyBanner;
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                             COALESCE(pic, '') AS pic, \
                             '' AS link, \
                             0 AS sort, \
                             0 AS addtime";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<CompanyBanner>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_banner \
         WHERE uid = ? AND status != 2 \
         ORDER BY id DESC"
    );
    sqlx::query_as::<_, CompanyBanner>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_banner WHERE uid = ? AND status != 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    pic: &str,
    _link: Option<&str>,
    _sort: i32,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP table has only (uid, pic, status, statusbody, did). status defaults to 1.
    // link / sort / addtime have no PHP column and are silently dropped.
    let res = sqlx::query(
        "INSERT INTO phpyun_banner (uid, pic) VALUES (?, ?)",
    )
    .bind(uid)
    .bind(pic)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    pic: Option<&str>,
    _link: Option<&str>,
    _sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    // Only `pic` is updatable on the PHP table; link / sort don't exist.
    let Some(p) = pic else {
        return Ok(0);
    };
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_banner SET pic = ");
    qb.push_bind(p);
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

/// Soft delete: bulk UPDATE status=2.
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_banner SET status = 2 WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}
