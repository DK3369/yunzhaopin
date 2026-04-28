//! `phpyun_entrust` repository — jobseeker → headhunter bindings.
//!
//! Schema (5 cols): id, uid, lt_uid, datetime, remind_status.
//! Casts on read for `int(11)` → `u64` decode safety.

use super::entity::Entrust;
use sqlx::MySqlPool;

// PHP `phpyun_entrust.uid` is nullable int — was reading `CAST(uid AS
// UNSIGNED)` which yields NULL → sqlx panics. Wrap in COALESCE.
const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                      CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                      CAST(COALESCE(lt_uid, 0) AS UNSIGNED) AS lt_uid, \
                      COALESCE(datetime, 0) AS datetime, \
                      COALESCE(remind_status, 0) AS remind_status";

/// Look up a binding by (uid, lt_uid). Used for duplicate-check before INSERT.
pub async fn find_binding(
    pool: &MySqlPool,
    uid: u64,
    lt_uid: u64,
) -> Result<Option<Entrust>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_entrust
          WHERE uid = ? AND lt_uid = ? LIMIT 1"
    );
    sqlx::query_as::<_, Entrust>(&sql)
        .bind(uid)
        .bind(lt_uid)
        .fetch_optional(pool)
        .await
}

/// Paginated bindings for one jobseeker, newest first.
pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Entrust>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_entrust
          WHERE uid = ?
          ORDER BY datetime DESC, id DESC
          LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Entrust>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_entrust WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

/// Headhunter side: paginated list of jobseekers who have entrusted this
/// `lt_uid`, newest first.
pub async fn list_by_lt_uid(
    pool: &MySqlPool,
    lt_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Entrust>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_entrust
          WHERE lt_uid = ?
          ORDER BY datetime DESC, id DESC
          LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Entrust>(&sql)
        .bind(lt_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_lt_uid(
    pool: &MySqlPool,
    lt_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_entrust WHERE lt_uid = ?")
            .bind(lt_uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    lt_uid: u64,
    datetime: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_entrust (uid, lt_uid, datetime, remind_status)
              VALUES (?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(lt_uid)
    .bind(datetime)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Unbind by (uid, lt_uid). Idempotent.
pub async fn delete(
    pool: &MySqlPool,
    uid: u64,
    lt_uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_entrust WHERE uid = ? AND lt_uid = ?")
        .bind(uid)
        .bind(lt_uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Unbind by row id (uid scoped — caller cannot delete someone else's row).
pub async fn delete_by_id(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_entrust WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
