//! `phpyun_member_log` — user activity audit trail.
//!
//! Schema (PHP): `uid, opera, type, usertype, content, ip, ctime, did`.
//! `opera` is the activity-bucket id (5 = collection, 7 = applies, …) and
//! `type` is the verb (1 = add, 3 = delete). Both are PHP-side enumerations,
//! so callers pass i32 values from named constants in the service layer.

use sqlx::MySqlPool;

/// Append a member-log row. Best-effort — fire-and-forget paths simply ignore
/// the error; only callers that need confirmation should propagate it.
pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    opera: i32,
    type_: i32,
    usertype: i32,
    content: &str,
    ip: &str,
    ctime: i64,
    did: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_member_log
              (uid, opera, type, usertype, content, ip, ctime, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(opera)
    .bind(type_)
    .bind(usertype)
    .bind(content)
    .bind(ip)
    .bind(ctime)
    .bind(did)
    .execute(pool)
    .await?;
    Ok(())
}
