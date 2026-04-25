use super::entity::ResumeDownload;
use sqlx::MySqlPool;

pub async fn record(
    pool: &MySqlPool,
    com_id: u64,
    uid: u64,
    eid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_down_resume (com_id, uid, eid, datetime)
           VALUES (?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE datetime = VALUES(datetime)"#,
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
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT id FROM phpyun_down_resume WHERE com_id = ? AND uid = ? LIMIT 1",
    )
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
        r#"SELECT id, com_id, uid, eid, datetime
           FROM phpyun_down_resume
           WHERE com_id = ?
           ORDER BY datetime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(com_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_for_company(pool: &MySqlPool, com_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_down_resume WHERE com_id = ?",
    )
    .bind(com_id)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Job seeker viewing who has downloaded their resume
pub async fn list_for_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ResumeDownload>, sqlx::Error> {
    sqlx::query_as::<_, ResumeDownload>(
        r#"SELECT id, com_id, uid, eid, datetime
           FROM phpyun_down_resume
           WHERE uid = ?
           ORDER BY datetime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_for_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_down_resume WHERE uid = ?",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
