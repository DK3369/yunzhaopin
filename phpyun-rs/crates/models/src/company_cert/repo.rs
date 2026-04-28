//! `phpyun_company_cert` repository — company verification queue.
//!
//! PHP schema (truth): `id, uid, usertype, type, status, step, check, check2,
//! social_credit, owner_cert, wt_cert, other_cert, ctime, statusbody, did`.
//!
//! Mapping (Rust entity → PHP column):
//! - `license_photo` → `social_credit`  (营业执照照片)
//! - `id_photo`      → `owner_cert`     (法人身份证照片)
//! - `status`        → `status`
//! - `note`          → `statusbody`     (审核备注)
//! - `submitted_at`  → `ctime`
//! - `reviewed_at`   → no PHP column    (always 0)
//! - `reviewer_uid`  → no PHP column    (always 0)
//! - `created_at`    → `ctime`
//! - `updated_at`    → `ctime`
//!
//! Caveats:
//! - PHP table has no UNIQUE on `uid`, so `ON DUPLICATE KEY UPDATE` would
//!   never fire. Implement upsert as `find → UPDATE or INSERT` instead.

use super::entity::CompanyCert;
use sqlx::MySqlPool;

const SELECT_FIELDS: &str = "CAST(uid AS UNSIGNED) AS uid, \
                             COALESCE(social_credit, '') AS license_photo, \
                             COALESCE(owner_cert, '') AS id_photo, \
                             COALESCE(status, 0) AS status, \
                             COALESCE(statusbody, '') AS note, \
                             COALESCE(ctime, 0) AS submitted_at, \
                             0 AS reviewed_at, \
                             CAST(0 AS UNSIGNED) AS reviewer_uid, \
                             COALESCE(ctime, 0) AS created_at, \
                             COALESCE(ctime, 0) AS updated_at";

pub async fn find(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<CompanyCert>, sqlx::Error> {
    let sql = format!("SELECT {SELECT_FIELDS} FROM phpyun_company_cert WHERE uid = ? LIMIT 1");
    sqlx::query_as::<_, CompanyCert>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Company submits/updates verification info. PHP table has no UNIQUE on
/// `uid` so we explicitly check + UPDATE / INSERT.
pub async fn upsert(
    pool: &MySqlPool,
    uid: u64,
    license_photo: &str,
    id_photo: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM phpyun_company_cert WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    if exists.is_some() {
        sqlx::query(
            r#"UPDATE phpyun_company_cert
               SET social_credit = ?, owner_cert = ?, status = 1,
                   statusbody = '', ctime = ?
               WHERE uid = ?"#,
        )
        .bind(license_photo)
        .bind(id_photo)
        .bind(now)
        .bind(uid)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"INSERT INTO phpyun_company_cert
               (uid, social_credit, owner_cert, status, statusbody, ctime)
               VALUES (?, ?, ?, 1, '', ?)"#,
        )
        .bind(uid)
        .bind(license_photo)
        .bind(id_photo)
        .bind(now)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    note: &str,
    _reviewer_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP table has no `reviewed_at` / `reviewer_uid` columns — those are
    // dropped silently. Audit info still goes through `audit::emit()`.
    let res = sqlx::query(
        r#"UPDATE phpyun_company_cert
           SET status = ?, statusbody = ?, ctime = ?
           WHERE uid = ? AND status = 1"#,
    )
    .bind(status)
    .bind(note)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_pending(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyCert>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_company_cert \
         WHERE status = 1 ORDER BY ctime ASC, id ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyCert>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_pending(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_cert WHERE status = 1")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}
