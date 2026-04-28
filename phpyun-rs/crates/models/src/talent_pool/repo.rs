//! Company talent pool repo.
//!
//! Table: `phpyun_talent_pool`.

use super::entity::TalentPoolItem;
use sqlx::{MySqlPool, QueryBuilder};

// All numeric columns on `phpyun_talent_pool` are NULL-allowed; entity uses
// plain `u64 / i64`. COALESCE at the projection.
const FIELDS: &str = "id, \
    COALESCE(eid, 0) AS eid, \
    COALESCE(cuid, 0) AS cuid, \
    COALESCE(uid, 0) AS uid, \
    remark, \
    COALESCE(ctime, 0) AS ctime";

// Soft-delete convention: `status = 2` means deleted. All queries include `AND status != 2`.

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<TalentPoolItem>, sqlx::Error> {
    let sql =
        format!("SELECT {FIELDS} FROM phpyun_talent_pool WHERE id = ? AND status != 2 LIMIT 1");
    sqlx::query_as::<_, TalentPoolItem>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_com_and_eid(
    pool: &MySqlPool,
    cuid: u64,
    eid: u64,
) -> Result<Option<TalentPoolItem>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_talent_pool \
         WHERE cuid = ? AND eid = ? AND status != 2 LIMIT 1"
    );
    sqlx::query_as::<_, TalentPoolItem>(&sql)
        .bind(cuid)
        .bind(eid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    cuid: u64,
    uid: u64,
    eid: u64,
    remark: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_talent_pool (eid, cuid, uid, remark, ctime) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(eid)
    .bind(cuid)
    .bind(uid)
    .bind(remark.unwrap_or(""))
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_by_com(
    pool: &MySqlPool,
    cuid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<TalentPoolItem>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_talent_pool \
         WHERE cuid = ? AND status != 2 \
         ORDER BY ctime DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, TalentPoolItem>(&sql)
        .bind(cuid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
}

pub async fn count_by_com(pool: &MySqlPool, cuid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_talent_pool WHERE cuid = ? AND status != 2",
    )
    .bind(cuid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Soft delete: bulk UPDATE status=2; no physical DELETE.
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    cuid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_talent_pool SET status = 2 WHERE cuid = ");
    qb.push_bind(cuid);
    qb.push(" AND status != 2 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn update_remark(
    pool: &MySqlPool,
    id: u64,
    cuid: u64,
    remark: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_talent_pool SET remark = ? \
         WHERE id = ? AND cuid = ? AND status != 2",
    )
    .bind(remark)
    .bind(id)
    .bind(cuid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
