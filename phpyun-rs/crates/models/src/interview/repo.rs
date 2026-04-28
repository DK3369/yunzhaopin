//! `phpyun_yqmb` repository — interview invitations.
//!
//! Real PHP columns:
//!   `id, uid, name, linkman, linktel, address, intertime, content, addtime,
//!    did, status, statusbody`
//!
//! Mapping to the Rust `Interview` entity (`apply_id`, `com_id`, `uid`,
//! `job_id`, `inter_time`, `address`, `linkman`, `linktel`, `remark`,
//! `status`, `created_at`):
//!
//! | entity field | PHP column                    |
//! |--------------|-------------------------------|
//! | id           | id                            |
//! | apply_id     | (none) — projected as `0`     |
//! | com_id       | did   — recruiter's uid       |
//! | uid          | uid                           |
//! | job_id       | (none) — projected as `0`     |
//! | inter_time   | intertime                     |
//! | address      | address                       |
//! | linkman      | linkman                       |
//! | linktel      | linktel                       |
//! | remark       | content                       |
//! | status       | status                        |
//! | created_at   | addtime                       |
//!
//! The original `phpyun_yqmb` doesn't track which application or job an
//! invitation came from — that's a Rust-port concept. We accept losing those
//! two fields rather than introducing a side table.

use super::entity::Interview;
use sqlx::MySqlPool;

/// SELECT projection that maps real `phpyun_yqmb` columns onto the Rust
/// `Interview` shape via aliases. Always paired with `FROM phpyun_yqmb`.
const SELECT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(0 AS UNSIGNED) AS apply_id, \
    CAST(COALESCE(did, 0) AS UNSIGNED) AS com_id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    CAST(0 AS UNSIGNED) AS job_id, \
    CAST(COALESCE(intertime, 0) AS SIGNED) AS inter_time, \
    COALESCE(address, '') AS address, \
    COALESCE(linkman, '') AS linkman, \
    COALESCE(linktel, '') AS linktel, \
    content AS remark, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(addtime, 0) AS SIGNED) AS created_at";

pub struct InterviewCreate<'a> {
    /// Persisted only in audit/event metadata; the `phpyun_yqmb` table doesn't
    /// have an `apply_id` column, so this field is dropped on insert.
    pub apply_id: u64,
    pub com_id: u64,
    pub uid: u64,
    /// Same caveat as `apply_id`.
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
    // `name` is NOT NULL in `phpyun_yqmb`; PHP's caller fills it with the job
    // title, but the Rust caller doesn't have it on hand. Empty string keeps
    // the row valid and avoids re-querying the job. `statusbody` and `content`
    // accept NULL/empty.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (uid, did, name, intertime, address, linkman, linktel, content, status, addtime)
           VALUES (?, ?, '', ?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.uid)
    .bind(c.com_id)
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
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_yqmb WHERE id = ? LIMIT 1");
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
        "SELECT {SELECT_FIELDS} FROM phpyun_yqmb \
         WHERE uid = ? ORDER BY addtime DESC, id DESC LIMIT ? OFFSET ?"
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
        "SELECT {SELECT_FIELDS} FROM phpyun_yqmb \
         WHERE did = ? ORDER BY addtime DESC, id DESC LIMIT ? OFFSET ?"
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
        "SELECT COUNT(*) FROM phpyun_yqmb WHERE did = ?",
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
        "UPDATE phpyun_yqmb SET status = 3 WHERE id = ? AND did = ? AND status IN (0, 1)",
    )
    .bind(id)
    .bind(com_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
