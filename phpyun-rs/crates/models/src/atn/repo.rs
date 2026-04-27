//! `phpyun_atn` repository — directional follow edges.
//!
//! No UNIQUE KEY exists on the PHP table; duplicate-prevention happens in the
//! application layer via `find_one` before insert (matching `atn.model.php`).

use super::entity::Atn;
use sqlx::MySqlPool;

// PHPYun's `phpyun_atn` declares every numeric column as **signed** `int(11)`,
// but our entity uses unsigned `u64`/`u32` for cleaner downstream APIs. Cast
// every column at the SELECT boundary so sqlx can decode without type
// mismatches. The `time` column is intentionally cast to SIGNED because it's
// a Unix timestamp that we model as `i64`.
const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(uid AS UNSIGNED) AS uid, \
    CAST(sc_uid AS UNSIGNED) AS sc_uid, \
    CAST(COALESCE(time, 0) AS SIGNED) AS time, \
    CAST(usertype AS SIGNED) AS usertype, \
    CAST(sc_usertype AS SIGNED) AS sc_usertype, \
    CAST(tid AS SIGNED) AS tid, \
    CAST(conid AS SIGNED) AS conid, \
    CAST(xjhid AS SIGNED) AS xjhid";

pub async fn find_one(
    pool: &MySqlPool,
    uid: u64,
    sc_uid: u64,
    sc_usertype: i32,
) -> Result<Option<Atn>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_atn \
         WHERE uid = ? AND sc_uid = ? AND sc_usertype = ? LIMIT 1"
    );
    sqlx::query_as::<_, Atn>(&sql)
        .bind(uid)
        .bind(sc_uid)
        .bind(sc_usertype)
        .fetch_optional(pool)
        .await
}

pub async fn exists(
    pool: &MySqlPool,
    uid: u64,
    sc_uid: u64,
    sc_usertype: i32,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM phpyun_atn \
         WHERE uid = ? AND sc_uid = ? AND sc_usertype = ? LIMIT 1",
    )
    .bind(uid)
    .bind(sc_uid)
    .bind(sc_usertype)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Same as [`exists`] but without filtering by `sc_usertype` — used by the
/// public company-detail page to render the "已关注" button (we don't know
/// the user-type at that point, follow is unique by (uid, sc_uid) anyway).
pub async fn exists_pair(
    pool: &MySqlPool,
    uid: u64,
    sc_uid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM phpyun_atn WHERE uid = ? AND sc_uid = ? LIMIT 1",
    )
    .bind(uid)
    .bind(sc_uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub struct InsertAtn {
    pub uid: u64,
    pub sc_uid: u64,
    pub usertype: i32,
    pub sc_usertype: i32,
    pub time: i64,
}

pub async fn insert(pool: &MySqlPool, v: InsertAtn) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_atn (uid, sc_uid, usertype, sc_usertype, time) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(v.uid)
    .bind(v.sc_uid)
    .bind(v.usertype)
    .bind(v.sc_usertype)
    .bind(v.time)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_edge(
    pool: &MySqlPool,
    uid: u64,
    sc_uid: u64,
    sc_usertype: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_atn \
         WHERE uid = ? AND sc_uid = ? AND sc_usertype = ?",
    )
    .bind(uid)
    .bind(sc_uid)
    .bind(sc_usertype)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// List the targets a user is currently following, newest first.
pub async fn list_by_follower(
    pool: &MySqlPool,
    uid: u64,
    sc_usertype: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Atn>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_atn \
         WHERE uid = ? AND sc_usertype = ? \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Atn>(&sql)
        .bind(uid)
        .bind(sc_usertype)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_follower(
    pool: &MySqlPool,
    uid: u64,
    sc_usertype: i32,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_atn WHERE uid = ? AND sc_usertype = ?",
    )
    .bind(uid)
    .bind(sc_usertype)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Followers of a target.
pub async fn list_by_followee(
    pool: &MySqlPool,
    sc_uid: u64,
    sc_usertype: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Atn>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_atn \
         WHERE sc_uid = ? AND sc_usertype = ? \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Atn>(&sql)
        .bind(sc_uid)
        .bind(sc_usertype)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_followee(
    pool: &MySqlPool,
    sc_uid: u64,
    sc_usertype: i32,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_atn WHERE sc_uid = ? AND sc_usertype = ?",
    )
    .bind(sc_uid)
    .bind(sc_usertype)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Return the set of `sc_uid` values a given user follows (any usertype).
/// Used by feed pages that need to mark "followed?" against many target uids
/// at once — one query is cheaper than N point-checks.
pub async fn list_followee_uids(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(sc_uid,0) AS SIGNED) FROM phpyun_atn WHERE uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(v,)| v.max(0) as u64).collect())
}

/// Best-effort bump of `phpyun_company.ant_num` (note the historical typo —
/// the column really is `ant_num`, not `atn_num`). Clamps at 0 on the way down.
pub async fn bump_company_ant_num(
    pool: &MySqlPool,
    company_uid: u64,
    delta: i32,
) -> Result<(), sqlx::Error> {
    if delta >= 0 {
        sqlx::query("UPDATE phpyun_company SET ant_num = ant_num + ? WHERE uid = ?")
            .bind(delta)
            .bind(company_uid)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            "UPDATE phpyun_company SET ant_num = GREATEST(ant_num - ?, 0) WHERE uid = ?",
        )
        .bind(-delta)
        .bind(company_uid)
        .execute(pool)
        .await?;
    }
    Ok(())
}
