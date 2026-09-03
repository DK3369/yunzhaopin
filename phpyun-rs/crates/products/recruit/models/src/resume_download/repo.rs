use super::entity::ResumeDownload;
use sqlx::{MySqlPool, QueryBuilder};
use std::collections::HashSet;

pub async fn record(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
    eid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_down_resume (comid, uid, eid, downtime)
           VALUES (?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE downtime = VALUES(downtime)"#,
    )
    .bind(com_id)
    .bind(uid)
    .bind(eid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn already_downloaded(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> =
        sqlx::query_as("SELECT id FROM phpyun_down_resume WHERE comid = ? AND uid = ? LIMIT 1")
            .bind(com_id)
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

/// PHP `freedown_resume` — free-quota unlock still counts as `m_status=1`.
pub async fn already_freedown(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> =
        sqlx::query_as("SELECT id FROM phpyun_freedown_resume WHERE comid = ? AND uid = ? LIMIT 1")
            .bind(com_id)
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

/// Company viewing the resumes they have downloaded
pub async fn list_for_company(
    pool: &MySqlPool,
    com_id: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ResumeDownload>, sqlx::Error> {
    sqlx::query_as::<_, ResumeDownload>(
        r#"SELECT id, COALESCE(comid, 0) AS com_id, COALESCE(uid, 0) AS uid,
                  COALESCE(eid, 0) AS eid, downtime AS datetime
           FROM phpyun_down_resume
           WHERE comid = ?
           ORDER BY downtime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(com_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_for_company(pool: &MySqlPool, com_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_down_resume WHERE comid = ?")
        .bind(com_id)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Job seeker viewing who has downloaded their resume
pub async fn list_for_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ResumeDownload>, sqlx::Error> {
    sqlx::query_as::<_, ResumeDownload>(
        r#"SELECT id, COALESCE(comid, 0) AS com_id, COALESCE(uid, 0) AS uid,
                  COALESCE(eid, 0) AS eid, downtime AS datetime
           FROM phpyun_down_resume
           WHERE uid = ?
           ORDER BY downtime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_for_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_down_resume WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Uids among `uids` this company has downloaded or free-downloaded.
pub async fn unlocked_uids(
    pool: &MySqlPool,
    com_id: u64,
    uids: &[u64],
) -> Result<HashSet<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(HashSet::new());
    }
    let mut down: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_down_resume WHERE comid = ",
    );
    down.push_bind(com_id);
    down.push(" AND uid IN (");
    {
        let mut sep = down.separated(", ");
        for id in uids {
            sep.push_bind(*id);
        }
    }
    down.push(")");
    let mut set: HashSet<u64> = down
        .build_query_as::<(u64,)>()
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();
    let mut free: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_freedown_resume WHERE comid = ",
    );
    free.push_bind(com_id);
    free.push(" AND uid IN (");
    {
        let mut sep = free.separated(", ");
        for id in uids {
            sep.push_bind(*id);
        }
    }
    free.push(")");
    for (id,) in free.build_query_as::<(u64,)>().fetch_all(pool).await? {
        set.insert(id);
    }
    Ok(set)
}

pub async fn record_freedown(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
    eid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_freedown_resume
           (comid, uid, eid, downtime, type, usertype, status)
           VALUES (?, ?, ?, ?, 0, 2, 0)"#,
    )
    .bind(com_id)
    .bind(uid)
    .bind(eid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn count_today_freedown(
    pool: &MySqlPool,
    com_id: u64,
    today_start: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_freedown_resume \
         WHERE comid = ? AND usertype = 2 AND downtime >= ?",
    )
    .bind(com_id)
    .bind(today_start)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_today_down(
    pool: &MySqlPool,
    com_id: u64,
    today_start: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_down_resume WHERE comid = ? AND downtime >= ?",
    )
    .bind(com_id)
    .bind(today_start)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}
