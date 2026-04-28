//! `phpyun_user_session` repository.
//!
//! All SELECTs `CAST(...) AS UNSIGNED` for ID columns to keep sqlx's strict
//! signed/unsigned decoder happy across mixed-schema legacy + new tables.

use super::entity::UserSession;
use sqlx::MySqlPool;

pub struct InsertSession<'a> {
    pub uid: u64,
    pub usertype: u8,
    pub jti_access: &'a str,
    pub jti_refresh: &'a str,
    pub device: &'a str,
    pub device_raw: &'a str,
    pub ip: &'a str,
    pub login_at: i64,
    pub access_exp: i64,
    pub refresh_exp: i64,
}

pub async fn insert(pool: &MySqlPool, v: InsertSession<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_user_session
              (uid, usertype, jti_access, jti_refresh, device, device_raw, ip,
               login_at, last_seen_at, access_exp, refresh_exp, revoked_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)"#,
    )
    .bind(v.uid)
    .bind(v.usertype as i32)
    .bind(v.jti_access)
    .bind(v.jti_refresh)
    .bind(v.device)
    .bind(v.device_raw)
    .bind(v.ip)
    .bind(v.login_at)
    .bind(v.login_at) // last_seen_at = login_at
    .bind(v.access_exp)
    .bind(v.refresh_exp)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Rotate jtis on refresh. Looks the row up by the OLD refresh jti, swaps in
/// the new pair + bumped exps + bumped last_seen.
/// Returns rows_affected — 0 means the chain has been broken (kicked from
/// another device); caller should refuse the refresh.
pub async fn rotate_on_refresh(
    pool: &MySqlPool,
    old_refresh_jti: &str,
    new_access_jti: &str,
    new_refresh_jti: &str,
    new_access_exp: i64,
    new_refresh_exp: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_user_session
              SET jti_access = ?,
                  jti_refresh = ?,
                  access_exp = ?,
                  refresh_exp = ?,
                  last_seen_at = ?
            WHERE jti_refresh = ? AND revoked_at = 0"#,
    )
    .bind(new_access_jti)
    .bind(new_refresh_jti)
    .bind(new_access_exp)
    .bind(new_refresh_exp)
    .bind(now)
    .bind(old_refresh_jti)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Sliding-session rotate: client passed its current access_token; look the
/// row up by the OLD access jti and swap in the new pair. The session table
/// is the source of truth — if the row doesn't exist or has been revoked
/// (kicked from another device, manual logout, etc.), `rows_affected = 0`
/// and the caller refuses to mint a new token.
///
/// This guards against the case where the JWT signature is still valid (and
/// the jti hasn't been blacklisted yet) but the server-side session record
/// has been removed.
pub async fn rotate_on_access_refresh(
    pool: &MySqlPool,
    old_access_jti: &str,
    new_access_jti: &str,
    new_refresh_jti: &str,
    new_access_exp: i64,
    new_refresh_exp: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_user_session
              SET jti_access = ?,
                  jti_refresh = ?,
                  access_exp = ?,
                  refresh_exp = ?,
                  last_seen_at = ?
            WHERE jti_access = ? AND revoked_at = 0"#,
    )
    .bind(new_access_jti)
    .bind(new_refresh_jti)
    .bind(new_access_exp)
    .bind(new_refresh_exp)
    .bind(now)
    .bind(old_access_jti)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn touch_last_seen(
    pool: &MySqlPool,
    access_jti: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_user_session SET last_seen_at = ?
          WHERE jti_access = ? AND revoked_at = 0",
    )
    .bind(now)
    .bind(access_jti)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_active_by_uid(
    pool: &MySqlPool,
    uid: u64,
    now: i64,
) -> Result<Vec<UserSession>, sqlx::Error> {
    sqlx::query_as::<_, UserSession>(
        r#"SELECT
              CAST(id AS UNSIGNED) AS id,
              CAST(uid AS UNSIGNED) AS uid,
              usertype, jti_access, jti_refresh, device, device_raw, ip, ip_loc,
              CAST(login_at AS SIGNED) AS login_at, CAST(last_seen_at AS SIGNED) AS last_seen_at, CAST(access_exp AS SIGNED) AS access_exp, CAST(refresh_exp AS SIGNED) AS refresh_exp, CAST(revoked_at AS SIGNED) AS revoked_at
           FROM phpyun_user_session
           WHERE uid = ? AND revoked_at = 0 AND refresh_exp > ?
           ORDER BY last_seen_at DESC, id DESC"#,
    )
    .bind(uid)
    .bind(now)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id_and_uid(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<Option<UserSession>, sqlx::Error> {
    sqlx::query_as::<_, UserSession>(
        r#"SELECT
              CAST(id AS UNSIGNED) AS id,
              CAST(uid AS UNSIGNED) AS uid,
              usertype, jti_access, jti_refresh, device, device_raw, ip, ip_loc,
              CAST(login_at AS SIGNED) AS login_at, CAST(last_seen_at AS SIGNED) AS last_seen_at, CAST(access_exp AS SIGNED) AS access_exp, CAST(refresh_exp AS SIGNED) AS refresh_exp, CAST(revoked_at AS SIGNED) AS revoked_at
           FROM phpyun_user_session
           WHERE id = ? AND uid = ? LIMIT 1"#,
    )
    .bind(id)
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// Mark a single session revoked. Returns the row's (jti_access, access_exp,
/// jti_refresh, refresh_exp) so the caller can push BOTH jtis into the JWT
/// blacklist for instant cutoff.
pub async fn revoke_by_id(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    now: i64,
) -> Result<Option<(String, i64, String, i64)>, sqlx::Error> {
    let row: Option<(String, i64, String, i64)> = sqlx::query_as(
        "SELECT jti_access, CAST(access_exp AS SIGNED) AS access_exp, jti_refresh, CAST(refresh_exp AS SIGNED) AS refresh_exp
           FROM phpyun_user_session
          WHERE id = ? AND uid = ? AND revoked_at = 0 LIMIT 1",
    )
    .bind(id)
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    if row.is_none() {
        return Ok(None);
    }
    sqlx::query(
        "UPDATE phpyun_user_session SET revoked_at = ?
          WHERE id = ? AND uid = ? AND revoked_at = 0",
    )
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(row)
}

/// Revoke all OTHER active sessions for a user (keep the one matching
/// `keep_access_jti`). Returns the (access_jti, access_exp, refresh_jti,
/// refresh_exp) of revoked rows so the caller can blacklist them all.
pub async fn revoke_others(
    pool: &MySqlPool,
    uid: u64,
    keep_access_jti: &str,
    now: i64,
) -> Result<Vec<(String, i64, String, i64)>, sqlx::Error> {
    let rows: Vec<(String, i64, String, i64)> = sqlx::query_as(
        "SELECT jti_access, CAST(access_exp AS SIGNED) AS access_exp, jti_refresh, CAST(refresh_exp AS SIGNED) AS refresh_exp
           FROM phpyun_user_session
          WHERE uid = ? AND jti_access <> ? AND revoked_at = 0",
    )
    .bind(uid)
    .bind(keep_access_jti)
    .fetch_all(pool)
    .await?;
    sqlx::query(
        "UPDATE phpyun_user_session SET revoked_at = ?
          WHERE uid = ? AND jti_access <> ? AND revoked_at = 0",
    )
    .bind(now)
    .bind(uid)
    .bind(keep_access_jti)
    .execute(pool)
    .await?;
    Ok(rows)
}

/// Revoke by current access jti — used by logout.
pub async fn revoke_by_access_jti(
    pool: &MySqlPool,
    access_jti: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_user_session SET revoked_at = ?
          WHERE jti_access = ? AND revoked_at = 0",
    )
    .bind(now)
    .bind(access_jti)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Bulk delete dead rows. Run by the scheduler hourly.
pub async fn purge_dead(pool: &MySqlPool, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_user_session
          WHERE refresh_exp < ?
             OR (revoked_at != 0 AND revoked_at < ?)",
    )
    .bind(now)
    .bind(now - 7 * 24 * 3600) // keep 7 days of revoked rows for audit
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
