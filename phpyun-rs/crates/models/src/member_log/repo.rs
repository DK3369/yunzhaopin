//! `phpyun_member_log` — user activity audit trail.
//!
//! Schema (PHP): `uid, opera, type, usertype, content, ip, ctime, did`.
//! `opera` is the activity-bucket id (5 = collection, 7 = applies, …) and
//! `type` is the verb (1 = add, 3 = delete). Both are PHP-side enumerations,
//! so callers pass i32 values from named constants in the service layer.

use sqlx::MySqlPool;

/// Append a member-log row. Best-effort — fire-and-forget paths simply ignore
/// the error; only callers that need confirmation should propagate it.
pub struct InsertInput<'a> {
    pub uid: u64,
    pub opera: i32,
    pub type_: i32,
    pub usertype: i32,
    pub content: &'a str,
    pub ip: &'a str,
    pub ctime: i64,
    pub did: u32,
}

pub async fn insert(pool: &MySqlPool, input: InsertInput<'_>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_member_log
              (uid, opera, type, usertype, content, ip, ctime, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(input.uid)
    .bind(input.opera)
    .bind(input.type_)
    .bind(input.usertype)
    .bind(input.content)
    .bind(input.ip)
    .bind(input.ctime)
    .bind(input.did)
    .execute(pool)
    .await?;
    Ok(())
}
