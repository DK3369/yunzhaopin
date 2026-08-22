//! Cross-table uid reassignment helpers.
//!
//! Used by the account-split / account-merge flows where dozens of legacy
//! tables key on `uid` and must move atomically. The service layer holds
//! the static whitelist of tables; this module provides the SQL shape.
//!
//! `table` and `c_uid_col` are interpolated into the SQL string — callers
//! MUST pass values from a static whitelist, never user input.

use sqlx::{Executor, MySql};

/// `UPDATE {table} SET uid = ? WHERE uid = ?`. Returns rows-affected.
pub async fn reassign_uid<'e, E>(
    exec: E,
    table: &str,
    new_uid: u64,
    old_uid: u64,
) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = MySql>,
{
    let sql = format!("UPDATE {table} SET uid = ? WHERE uid = ?");
    let res = sqlx::query(&sql)
        .bind(new_uid)
        .bind(old_uid)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}

/// `UPDATE {table} SET uid = ? WHERE uid = ? AND usertype = ?`.
pub async fn reassign_uid_with_usertype<'e, E>(
    exec: E,
    table: &str,
    new_uid: u64,
    old_uid: u64,
    usertype: i32,
) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = MySql>,
{
    let sql = format!("UPDATE {table} SET uid = ? WHERE uid = ? AND usertype = ?");
    let res = sqlx::query(&sql)
        .bind(new_uid)
        .bind(old_uid)
        .bind(usertype)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}

/// `UPDATE {table} SET {col_uid} = ? WHERE {col_uid} = ? AND usertype = ?`.
/// For tables whose owner-uid is named something other than `uid`
/// (e.g. `phpyun_blacklist.c_uid`, `phpyun_report.p_uid`).
pub async fn reassign_named_uid_with_usertype<'e, E>(
    exec: E,
    table: &str,
    col: &str,
    new_uid: u64,
    old_uid: u64,
    usertype: i32,
) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = MySql>,
{
    let sql = format!("UPDATE {table} SET {col} = ? WHERE {col} = ? AND usertype = ?");
    let res = sqlx::query(&sql)
        .bind(new_uid)
        .bind(old_uid)
        .bind(usertype)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}
