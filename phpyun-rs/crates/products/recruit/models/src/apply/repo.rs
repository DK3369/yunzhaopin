//! Job application repo.
//!
//! The `phpyun_userid_job` table has no unique index in PHP, but we check
//! "same uid+job_id already applied" at the business layer to prevent
//! duplicate applications. For strict consistency, a migration adding
//! UNIQUE(uid, job_id) could be considered later.

use super::entity::Apply;
use sqlx::{MySqlPool, QueryBuilder};
use std::collections::{HashMap, HashSet};

// PHP `phpyun_userid_job.invited / invite_time` are nullable int; entity
// uses plain i32/i64. COALESCE so a NULL row can't trip sqlx.
const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
     CAST(uid AS UNSIGNED) AS uid, \
     CAST(job_id AS UNSIGNED) AS job_id, \
     CAST(com_id AS UNSIGNED) AS com_id, \
     CAST(eid AS UNSIGNED) AS eid, \
     COALESCE(job_name, '') AS job_name, \
     COALESCE(com_name, '') AS com_name, \
     CAST(datetime AS SIGNED) AS datetime, is_browse, \
     COALESCE(invited, 0) AS invited, \
     COALESCE(invite_time, 0) AS invite_time, \
     isdel, quxiao";

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Apply>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_userid_job WHERE id = ? AND isdel = 9 LIMIT 1");
    sqlx::query_as::<_, Apply>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// PHP `setCompanyLink` com_login_link=5: applied if userid_job (is_browse<>6) or userid_msg.
pub async fn count_active_by_uid_job(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_job \
          WHERE uid = ? AND job_id = ? AND isdel = 9 AND COALESCE(is_browse, 0) <> 6",
    )
    .bind(uid)
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_userid_msg_by_uid_job(
    pool: &MySqlPool,
    uid: u64,
    job_id: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
          WHERE uid = ? AND jobid = ? AND isdel = 9",
    )
    .bind(uid)
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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

pub struct ApplyCreate<'a> {
    pub uid: u64,
    pub job_id: u64,
    pub job_name: &'a str,
    pub com_id: u64,
    pub com_name: &'a str,
    pub eid: u64,
    pub now: i64,
}

pub async fn create(pool: &MySqlPool, c: ApplyCreate<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_userid_job
           (uid, job_id, job_name, com_id, com_name, eid, datetime, is_browse, invited, invite_time, isdel, quxiao)
           VALUES (?, ?, ?, ?, ?, ?, ?, 1, 0, 0, 9, 0)"#,
    )
    .bind(c.uid)
    .bind(c.job_id)
    .bind(c.job_name)
    .bind(c.com_id)
    .bind(c.com_name)
    .bind(c.eid)
    .bind(c.now)
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `getSqJobInfo(['com_id'=>$uid,'eid'=>$eid,'isdel'=>9])`.
pub async fn exists_by_com_eid(
    pool: &MySqlPool,
    com_id: u64,
    eid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM phpyun_userid_job WHERE com_id = ? AND eid = ? AND isdel = 9 LIMIT 1",
    )
    .bind(com_id)
    .bind(eid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Count active (`isdel = 9`) applications by a jobseeker to a specific
/// company. Used by the company-detail page to show "you've applied N times".
pub async fn count_by_uid_to_company(
    pool: &MySqlPool,
    uid: u64,
    com_id: u64,
) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_job \
         WHERE uid = ? AND com_id = ? AND isdel = 9",
    )
    .bind(uid)
    .bind(com_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

/// PHP `getSqJobNum` for one job (`isdel=9`).
pub async fn count_by_job(pool: &MySqlPool, job_id: u64) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_job WHERE job_id = ? AND isdel = 9",
    )
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

/// PHP `is_browse > 1` treated as replied / processed.
pub async fn count_replied_by_job(pool: &MySqlPool, job_id: u64) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_job \
         WHERE job_id = ? AND isdel = 9 AND COALESCE(is_browse, 0) > 1",
    )
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
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
    let res = sqlx::query("UPDATE phpyun_userid_job SET is_browse = ? WHERE id = ? AND com_id = ?")
        .bind(state)
        .bind(id)
        .bind(com_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Job seeker withdraws application (soft delete + set quxiao=1).
pub async fn withdraw(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("UPDATE phpyun_userid_job SET quxiao = 1, isdel = 0 WHERE id = ? AND uid = ?")
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

/// Resume display names for application list rows (`phpyun_resume.uid`).
pub async fn resume_names_by_uids(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<HashMap<u64, String>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(HashMap::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(name, '') AS name FROM phpyun_resume WHERE uid IN (",
    );
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    sep.push_unseparated(")");
    let rows: Vec<(u64, String)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().collect())
}

pub async fn count_by_com(
    pool: &MySqlPool,
    com_id: u64,
    f: ApplyFilter,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_userid_job WHERE com_id = ");
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
pub async fn invite(pool: &MySqlPool, id: u64, com_id: u64, now: i64) -> Result<u64, sqlx::Error> {
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

/// PHP `addYqms`: mark any existing applications from this seeker as invited.
pub async fn mark_invited_by_seeker(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_job SET invited = 1, invite_time = ?, is_browse = 2 \
         WHERE uid = ? AND com_id = ? AND isdel = 9",
    )
    .bind(now)
    .bind(uid)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Job ids in `job_ids` that this jobseeker has already applied to (`isdel=9`).
pub async fn applied_job_ids(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
) -> Result<HashSet<u64>, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(HashSet::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(job_id AS UNSIGNED) FROM phpyun_userid_job WHERE uid = ",
    );
    qb.push_bind(uid);
    qb.push(" AND isdel = 9 AND job_id IN (");
    let mut sep = qb.separated(", ");
    for id in job_ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Seeker uids this company has already invited (`phpyun_userid_msg`).
pub async fn invited_seeker_uids(
    pool: &MySqlPool,
    com_uid: u64,
    uids: &[u64],
) -> Result<HashSet<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(HashSet::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_userid_msg WHERE fid = ",
    );
    qb.push_bind(com_uid);
    qb.push(" AND isdel = 9 AND uid IN (");
    let mut sep = qb.separated(", ");
    for id in uids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Seekers among `uids` who have applied to this company (`phpyun_userid_job`).
pub async fn applied_seeker_uids(
    pool: &MySqlPool,
    com_uid: u64,
    uids: &[u64],
) -> Result<HashSet<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(HashSet::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_userid_job WHERE com_id = ",
    );
    qb.push_bind(com_uid);
    qb.push(" AND isdel = 9 AND uid IN (");
    let mut sep = qb.separated(", ");
    for id in uids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Unread applications (`is_browse=1`) used for company reply-rate `pre`.
pub async fn count_unread_by_company(pool: &MySqlPool, com_id: u64) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_job \
         WHERE com_id = ? AND isdel = 9 AND is_browse = 1",
    )
    .bind(com_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

pub async fn count_userid_msg_by_fid_uid(
    pool: &MySqlPool,
    fid: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
         WHERE fid = ? AND uid = ? AND isdel = 9",
    )
    .bind(fid)
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

pub async fn count_userid_msg_today(
    pool: &MySqlPool,
    fid: u64,
    today_start: i64,
) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
         WHERE fid = ? AND isdel = 9 AND datetime >= ?",
    )
    .bind(fid)
    .bind(today_start)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}
