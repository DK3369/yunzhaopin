use super::entity::CompanyCert;
use sqlx::MySqlPool;

const FIELDS: &str =
    "uid, license_photo, id_photo, status, note, submitted_at, reviewed_at, reviewer_uid, created_at, updated_at";

pub async fn find(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<CompanyCert>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company_cert WHERE uid = ?");
    sqlx::query_as::<_, CompanyCert>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Company submits/updates verification info -- `ON DUPLICATE KEY UPDATE`
/// is idempotent; status automatically transitions to 1 (under review).
pub async fn upsert(
    pool: &MySqlPool,
    uid: u64,
    license_photo: &str,
    id_photo: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_company_cert
           (uid, license_photo, id_photo, status, note, submitted_at, reviewed_at, reviewer_uid, created_at, updated_at)
           VALUES (?, ?, ?, 1, '', ?, 0, 0, ?, ?)
           ON DUPLICATE KEY UPDATE
               license_photo = VALUES(license_photo),
               id_photo      = VALUES(id_photo),
               status        = 1,
               note          = '',
               submitted_at  = VALUES(submitted_at),
               reviewed_at   = 0,
               reviewer_uid  = 0,
               updated_at    = VALUES(updated_at)"#,
    )
    .bind(uid)
    .bind(license_photo)
    .bind(id_photo)
    .bind(now)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    note: &str,
    reviewer_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_cert
           SET status = ?, note = ?, reviewed_at = ?, reviewer_uid = ?, updated_at = ?
           WHERE uid = ? AND status = 1"#,
    )
    .bind(status)
    .bind(note)
    .bind(now)
    .bind(reviewer_uid)
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
        "SELECT {FIELDS} FROM phpyun_company_cert
         WHERE status = 1 ORDER BY submitted_at ASC LIMIT ? OFFSET ?"
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
