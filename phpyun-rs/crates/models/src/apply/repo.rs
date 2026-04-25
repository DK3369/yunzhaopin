//! Job application repo.
//!
//! The `phpyun_userid_job` table has no unique index in PHP, but we check
//! "same uid+job_id already applied" at the business layer to prevent
//! duplicate applications. For strict consistency, a migration adding
//! UNIQUE(uid, job_id) could be considered later.

use super::entity::Apply;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str =
    "id, uid, job_id, com_id, eid, datetime, is_browse, invited, invite_time, isdel, quxiao";

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Apply>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_userid_job WHERE id = ? AND isdel = 9 LIMIT 1"
    );
    sqlx::query_as::<_, Apply>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_uid_job(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
) -> Result<Option<Apply>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_userid_job WHERE uid = ? AND job_id = ? AND isdel = 9 LIMIT 1"
    );
    sqlx::query_as::<_, Apply>(&sql)
        .bind(uid)
        .bind(job_id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
    com_id: u64,
    eid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_userid_job
           (uid, job_id, com_id, eid, datetime, is_browse, invited, invite_time, isdel, quxiao)
           VALUES (?, ?, ?, ?, ?, 1, 0, 0, 9, 0)"#,
    )
    .bind(uid)
    .bind(job_id)
    .bind(com_id)
    .bind(eid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

// ==================== Job seeker view ====================

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    state_filter: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Apply>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_userid_job WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND isdel = 9");
    if let Some(s) = state_filter {
        qb.push(" AND is_browse = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY datetime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Apply>().fetch_all(pool).await
}

pub async fn count_by_uid(
    pool: &MySqlPool,
    uid: u64,
    state_filter: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_userid_job WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND isdel = 9");
    if let Some(s) = state_filter {
        qb.push(" AND is_browse = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

/// Company side: transition application to any is_browse enum value
/// (1=unread / 0=viewed / 3=interviewed / 4=unsuitable / 7=hired).
/// Constrained by com_id so only the job owner may change it.
pub async fn set_browse_state(
    pool: &MySqlPool,
    id: u64,
    com_id: u64,
    state: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_job SET is_browse = ? WHERE id = ? AND com_id = ?",
    )
    .bind(state)
    .bind(id)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Job seeker withdraws application (soft delete + set quxiao=1).
pub async fn withdraw(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_job SET quxiao = 1, isdel = 0 WHERE id = ? AND uid = ?",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

// ==================== Company view ====================

#[derive(Debug, Default, Clone, Copy)]
pub struct ApplyFilter {
    /// None = all; true = unread only; false = viewed only.
    pub unread_only: Option<bool>,
    pub invited_only: Option<bool>,
}

pub async fn list_by_com(
    pool: &MySqlPool,
    com_id: u64,
    f: ApplyFilter,
    offset: u64,
    limit: u64,
) -> Result<Vec<Apply>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_userid_job WHERE com_id = ");
    qb.push_bind(com_id);
    qb.push(" AND isdel = 9 AND quxiao = 0");
    if let Some(unread) = f.unread_only {
        qb.push(" AND is_browse = ");
        qb.push_bind(if unread { 1 } else { 0 });
    }
    if let Some(inv) = f.invited_only {
        qb.push(" AND invited = ");
        qb.push_bind(if inv { 1 } else { 0 });
    }
    qb.push(" ORDER BY datetime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Apply>().fetch_all(pool).await
}

pub async fn count_by_com(
    pool: &MySqlPool,
    com_id: u64,
    f: ApplyFilter,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_userid_job WHERE com_id = ",
    );
    qb.push_bind(com_id);
    qb.push(" AND isdel = 9 AND quxiao = 0");
    if let Some(unread) = f.unread_only {
        qb.push(" AND is_browse = ");
        qb.push_bind(if unread { 1 } else { 0 });
    }
    if let Some(inv) = f.invited_only {
        qb.push(" AND invited = ");
        qb.push_bind(if inv { 1 } else { 0 });
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

/// Company marks as viewed (is_browse: 1 -> 0).
pub async fn mark_browsed(pool: &MySqlPool, id: u64, com_id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_job SET is_browse = 0 WHERE id = ? AND com_id = ? AND is_browse = 1",
    )
    .bind(id)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Company invites for interview.
pub async fn invite(
    pool: &MySqlPool,
    id: u64,
    com_id: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_job SET invited = 1, invite_time = ? WHERE id = ? AND com_id = ?",
    )
    .bind(now)
    .bind(id)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
