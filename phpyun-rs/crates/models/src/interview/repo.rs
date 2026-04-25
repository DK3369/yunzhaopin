use super::entity::Interview;
use sqlx::MySqlPool;

const FIELDS: &str = "id, apply_id, com_id, uid, job_id, inter_time, address,
                     linkman, linktel, remark, status, created_at";

pub struct InterviewCreate<'a> {
    pub apply_id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub inter_time: i64,
    pub address: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub remark: Option<&'a str>,
}

pub async fn create(
    pool: &MySqlPool,
    c: InterviewCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (apply_id, com_id, uid, job_id, inter_time, address, linkman, linktel, remark, status, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.apply_id)
    .bind(c.com_id)
    .bind(c.uid)
    .bind(c.job_id)
    .bind(c.inter_time)
    .bind(c.address)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.remark)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Interview>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_yqmb WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Interview>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Job seeker views their own received interview invitations.
pub async fn list_for_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Interview>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_yqmb
         WHERE uid = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Interview>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_yqmb WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
}

/// Company views invitations it has sent.
pub async fn list_for_company(
    pool: &MySqlPool,
    com_id: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Interview>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_yqmb
         WHERE com_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Interview>(&sql)
        .bind(com_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_company(pool: &MySqlPool, com_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_yqmb WHERE com_id = ?",
    )
    .bind(com_id)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Job seeker responds to interview -- 1 = accept / 2 = decline.
pub async fn respond_by_user(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_yqmb SET status = ? WHERE id = ? AND uid = ? AND status = 0",
    )
    .bind(status)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Company cancels the interview.
pub async fn cancel_by_company(
    pool: &MySqlPool,
    id: u64,
    com_id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_yqmb SET status = 3 WHERE id = ? AND com_id = ? AND status IN (0, 1)",
    )
    .bind(id)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
