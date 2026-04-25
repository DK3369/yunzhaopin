//! Part-time job repo.
//!
//! Three PHPYun source tables: `phpyun_partjob` / `phpyun_part_apply` /
//! `phpyun_part_collect`. All dynamic WHERE clauses go through
//! `sqlx::QueryBuilder::push_bind`; user input is never string-concatenated.

use super::entity::{PartApply, PartCollect, PartJob};
use sqlx::{MySqlPool, QueryBuilder};

// ==================== Filter ====================

#[derive(Debug, Default, Clone)]
pub struct PartFilter<'a> {
    pub keyword: Option<&'a str>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    /// Part-time category id
    pub part_type: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub salary_type: Option<i32>,
    pub billing_cycle: Option<i32>,
    pub did: u32,
}

const FIELDS: &str = "id, uid, name, com_name, `type`, provinceid, cityid, three_cityid, \
    address, number, sex, salary, salary_type, billing_cycle, worktime, sdate, edate, \
    content, linkman, linktel, state, status, r_status, rec_time, lastupdate, addtime, \
    COALESCE(did, 0) AS did, x, y, COALESCE(hits, 0) AS hits, \
    COALESCE(deadline, 0) AS deadline, \
    COALESCE(upstatus_time, 0) AS upstatus_time, \
    COALESCE(upstatus_count, 0) AS upstatus_count";

// ==================== partjob queries ====================

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<PartJob>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_partjob WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PartJob>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Public part-time job list -- only state=1 (approved) / status=0
/// (published) / r_status=1 (company in good standing) / not expired.
pub async fn list_public(
    pool: &MySqlPool,
    f: &PartFilter<'_>,
    offset: u64,
    limit: u64,
    now: i64,
) -> Result<Vec<PartJob>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_partjob WHERE state = 1 AND status = 0 AND r_status = 1");
    // edate=0 means recruiting indefinitely (PHPYun semantics).
    qb.push(" AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(")");
    qb.push(" AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(" ORDER BY rec_time DESC, lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<PartJob>().fetch_all(pool).await
}

pub async fn count_public(
    pool: &MySqlPool,
    f: &PartFilter<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_partjob WHERE state = 1 AND status = 0 AND r_status = 1",
    );
    qb.push(" AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(")");
    qb.push(" AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PartFilter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    if let Some(v) = f.province_id {
        qb.push(" AND provinceid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.city_id {
        qb.push(" AND cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.three_city_id {
        qb.push(" AND three_cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.part_type {
        qb.push(" AND `type` = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.salary_type {
        qb.push(" AND salary_type = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.billing_cycle {
        qb.push(" AND billing_cycle = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.min_salary {
        qb.push(" AND salary >= ");
        qb.push_bind(v);
    }
    if let Some(v) = f.max_salary {
        qb.push(" AND salary <= ");
        qb.push_bind(v);
    }
}

/// Increment hits.
pub async fn incr_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_partjob SET hits = hits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Company: list its own part-time postings (all states).
pub async fn list_by_com(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<PartJob>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_partjob WHERE uid = ? \
         ORDER BY rec_time DESC, lastupdate DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, PartJob>(&sql)
        .bind(com_uid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
}

pub async fn count_by_com(pool: &MySqlPool, com_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_partjob WHERE uid = ?")
            .bind(com_uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// Delete part-time jobs (a company can only delete its own; admin
/// bypasses the uid filter via the outer caller).
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    com_uid_opt: Option<u64>,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_partjob WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    if let Some(uid) = com_uid_opt {
        qb.push(" AND uid = ");
        qb.push_bind(uid);
    }
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

/// Cascade-delete part_collect / part_apply (used together with delete_by_ids).
pub async fn cascade_delete_children(
    pool: &MySqlPool,
    job_ids: &[u64],
) -> Result<(), sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(());
    }
    {
        let mut qb: QueryBuilder<sqlx::MySql> =
            QueryBuilder::new("DELETE FROM phpyun_part_collect WHERE jobid IN (");
        let mut sep = qb.separated(", ");
        for id in job_ids {
            sep.push_bind(*id);
        }
        qb.push(")");
        qb.build().execute(pool).await?;
    }
    {
        let mut qb: QueryBuilder<sqlx::MySql> =
            QueryBuilder::new("DELETE FROM phpyun_part_apply WHERE jobid IN (");
        let mut sep = qb.separated(", ");
        for id in job_ids {
            sep.push_bind(*id);
        }
        qb.push(")");
        qb.build().execute(pool).await?;
    }
    Ok(())
}

// ==================== part_apply ====================

pub async fn find_apply(
    pool: &MySqlPool,
    uid: u64,
    jobid: u64,
) -> Result<Option<PartApply>, sqlx::Error> {
    sqlx::query_as::<_, PartApply>(
        "SELECT id, uid, jobid, comid, ctime, COALESCE(status, 0) AS status \
         FROM phpyun_part_apply WHERE uid = ? AND jobid = ? LIMIT 1",
    )
    .bind(uid)
    .bind(jobid)
    .fetch_optional(pool)
    .await
}

pub async fn create_apply(
    pool: &MySqlPool,
    uid: u64,
    jobid: u64,
    comid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_part_apply (uid, jobid, comid, ctime, status) VALUES (?, ?, ?, ?, 1)",
    )
    .bind(uid)
    .bind(jobid)
    .bind(comid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_applies_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<PartApply>, sqlx::Error> {
    sqlx::query_as::<_, PartApply>(
        "SELECT id, uid, jobid, comid, ctime, COALESCE(status, 0) AS status \
         FROM phpyun_part_apply WHERE uid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?",
    )
    .bind(uid)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn count_applies_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_part_apply WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn list_applies_by_com(
    pool: &MySqlPool,
    com_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<PartApply>, sqlx::Error> {
    sqlx::query_as::<_, PartApply>(
        "SELECT id, uid, jobid, comid, ctime, COALESCE(status, 0) AS status \
         FROM phpyun_part_apply WHERE comid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?",
    )
    .bind(com_uid)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn count_applies_by_com(pool: &MySqlPool, com_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_part_apply WHERE comid = ?")
            .bind(com_uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn update_apply_status(
    pool: &MySqlPool,
    id: u64,
    com_uid: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_part_apply SET status = ? WHERE id = ? AND comid = ?",
    )
    .bind(status)
    .bind(id)
    .bind(com_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_applies(
    pool: &MySqlPool,
    ids: &[u64],
    uid_filter: Option<u64>,
    com_filter: Option<u64>,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_part_apply WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    if let Some(u) = uid_filter {
        qb.push(" AND uid = ");
        qb.push_bind(u);
    }
    if let Some(c) = com_filter {
        qb.push(" AND comid = ");
        qb.push_bind(c);
    }
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

// ==================== part_collect ====================

pub async fn find_collect(
    pool: &MySqlPool,
    uid: u64,
    jobid: u64,
) -> Result<Option<PartCollect>, sqlx::Error> {
    sqlx::query_as::<_, PartCollect>(
        "SELECT id, uid, jobid, comid, ctime FROM phpyun_part_collect \
         WHERE uid = ? AND jobid = ? LIMIT 1",
    )
    .bind(uid)
    .bind(jobid)
    .fetch_optional(pool)
    .await
}

pub async fn create_collect(
    pool: &MySqlPool,
    uid: u64,
    jobid: u64,
    comid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_part_collect (uid, jobid, comid, ctime) VALUES (?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(jobid)
    .bind(comid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_collects_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<PartCollect>, sqlx::Error> {
    sqlx::query_as::<_, PartCollect>(
        "SELECT id, uid, jobid, comid, ctime FROM phpyun_part_collect \
         WHERE uid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?",
    )
    .bind(uid)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn count_collects_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_part_collect WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn delete_collects(
    pool: &MySqlPool,
    ids: &[u64],
    uid_filter: Option<u64>,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_part_collect WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    if let Some(u) = uid_filter {
        qb.push(" AND uid = ");
        qb.push_bind(u);
    }
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}
