//! `phpyun_member` table access. All SQL is centralized here; the service layer never writes SQL directly.
//!
//! Key points for aligning with the PHPYun schema:
//! - `uid / did` are int(11) signed, so sqlx's `u64`/`u32` need a CAST to UNSIGNED
//! - `reg_date / login_date` are int(11) and need CAST(AS SIGNED) for `i64`
//! - `usertype / status` are int(1)/int(4); using `i32` on the Rust side is safer
//! - **OAuth columns are not** google_id/fb_id/apple_sub — PHPYun actually has:
//!     qqid / qqunionid / sinaid / wxid / wxopenid / unionid / wxname / bdopenid
//!   So OAuth binding on the Rust side uses an allowlist mapping provider → real PHP column name.

use super::entity::Member;
use sqlx::MySqlPool;

/// Core SELECT columns (including aliases / CASTs). Reused by joins from other tables.
const FIELDS: &str = "\
    CAST(uid AS UNSIGNED) AS uid, \
    COALESCE(username, '') AS username, \
    COALESCE(password, '') AS password, \
    COALESCE(salt, '') AS salt, \
    email, \
    moblie, \
    CAST(COALESCE(usertype, 0) AS SIGNED) AS usertype, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(did, 0) AS UNSIGNED) AS did, \
    CAST(COALESCE(reg_date, 0) AS SIGNED) AS reg_date, \
    CAST(COALESCE(login_date, 0) AS SIGNED) AS login_date";

/// Maps a provider name (external Rust protocol) to the real `phpyun_member` column in PHPYun.
/// Returns None for unmapped providers to avoid SQL against empty columns.
fn oauth_column_for(provider: &str) -> Option<&'static str> {
    match provider {
        // Mainstream domestic providers (natively supported by PHPYun)
        "qq" => Some("qqid"),
        "weibo" | "sina" => Some("sinaid"),
        "wechat" | "weixin" => Some("unionid"),       // On the Rust side, WeChat accounts are keyed by unionid
        "wechat_mp" | "wxopenid" => Some("wxopenid"), // Official Account / Mini Program openid
        "baidu" => Some("bdopenid"),
        // Overseas providers (PHPYun has no column; we keep the interface but return None, so upstream gets "bind failed")
        "google" | "facebook" | "apple" => None,
        _ => None,
    }
}

// ==================== Queries ====================

pub async fn find_for_login(pool: &MySqlPool, account: &str) -> Result<Option<Member>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_member \
         WHERE username = ? OR moblie = ? OR email = ? \
         LIMIT 1"
    );
    sqlx::query_as::<_, Member>(&sql)
        .bind(account)
        .bind(account)
        .bind(account)
        .fetch_optional(pool)
        .await
}

/// Cheap projection for flows that only need the uid keyed by any account
/// identifier (username / mobile / email). Used by the password-appeal flow.
pub async fn uid_by_account(pool: &MySqlPool, account: &str) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
         WHERE username = ? OR email = ? OR moblie = ? LIMIT 1",
    )
    .bind(account)
    .bind(account)
    .bind(account)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(u,)| u))
}

/// Persist a password-appeal submission. Sets `appeal`, `appealtime`, and
/// flips `appealstate = 1` (pending review). Returns rows-affected so the
/// caller can detect "uid not found" without a second SELECT.
pub async fn submit_appeal(
    pool: &MySqlPool,
    uid: u64,
    appeal_text: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member \
            SET appeal = ?, appealtime = ?, appealstate = 1 \
          WHERE uid = ?",
    )
    .bind(appeal_text)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<Member>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_member WHERE uid = ? LIMIT 1");
    sqlx::query_as::<_, Member>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_mobile(pool: &MySqlPool, mobile: &str) -> Result<Option<Member>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_member WHERE moblie = ? LIMIT 1");
    sqlx::query_as::<_, Member>(&sql)
        .bind(mobile)
        .fetch_optional(pool)
        .await
}

/// Case-insensitive email lookup (PHPYun stores emails verbatim, but users
/// type them in any case — `getInfo({email:..})` collation is `utf8_general_ci`,
/// which is case-insensitive by default).
pub async fn find_by_email_loose(
    pool: &MySqlPool,
    email: &str,
) -> Result<Option<Member>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_member WHERE email = ? LIMIT 1");
    sqlx::query_as::<_, Member>(&sql)
        .bind(email)
        .fetch_optional(pool)
        .await
}

/// Look up by third-party id (accepts an external provider name; unknown providers return None).
pub async fn find_by_oauth_id(
    pool: &MySqlPool,
    provider: &str,
    sub: &str,
) -> Result<Option<Member>, sqlx::Error> {
    let Some(col) = oauth_column_for(provider) else {
        return Ok(None);
    };
    let sql = format!("SELECT {FIELDS} FROM phpyun_member WHERE {col} = ? LIMIT 1");
    sqlx::query_as::<_, Member>(&sql)
        .bind(sub)
        .fetch_optional(pool)
        .await
}

// ==================== Uniqueness checks (used during registration) ====================

pub async fn exists_username(pool: &MySqlPool, username: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE username = ? LIMIT 1",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn exists_mobile(pool: &MySqlPool, mobile: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE moblie = ? LIMIT 1",
    )
    .bind(mobile)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn exists_email(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE email = ? LIMIT 1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Last-login unix timestamp for a member, `0` if never logged in / not found.
/// Used by detail pages to show "HR active N hours ago" hints.
pub async fn login_date(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(login_date, 0) AS SIGNED) FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(t,)| t).unwrap_or(0))
}

// ==================== Writes ====================

/// Creates a new member and returns the uid. Fields are aligned with PHPYun's `userRegSave`.
#[allow(clippy::too_many_arguments)]
pub async fn create_member<'e, E>(
    exec: E,
    username: &str,
    password_hash: &str,
    salt: &str,
    mobile: Option<&str>,
    email: Option<&str>,
    usertype: u8,
    did: u32,
    reg_ip: &str,
    reg_date: i64,
) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    // PHPYun's `phpyun_member.email` and `moblie` are `NOT NULL DEFAULT ''`,
    // so we coalesce missing values to empty strings rather than letting
    // sqlx bind a SQL NULL (which fails with `1048 Column ... cannot be null`).
    let res = sqlx::query(
        "INSERT INTO phpyun_member \
            (username, password, salt, moblie, email, usertype, status, did, reg_date, reg_ip, login_date) \
         VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?, ?, ?)",
    )
    .bind(username)
    .bind(password_hash)
    .bind(salt)
    .bind(mobile.unwrap_or(""))
    .bind(email.unwrap_or(""))
    .bind(usertype as i32)
    .bind(did)
    .bind(reg_date)
    .bind(reg_ip)
    .bind(reg_date)
    .execute(exec)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_password(pool: &MySqlPool, uid: u64, new_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_member SET password = ? WHERE uid = ?")
        .bind(new_hash)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_password_with_salt(
    pool: &MySqlPool,
    uid: u64,
    new_hash: &str,
    salt: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_member SET password = ?, salt = ? WHERE uid = ?")
        .bind(new_hash)
        .bind(salt)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn bind_oauth_id(
    pool: &MySqlPool,
    uid: u64,
    provider: &str,
    sub: &str,
) -> Result<(), sqlx::Error> {
    let Some(col) = oauth_column_for(provider) else {
        return Ok(());
    };
    let sql = format!("UPDATE phpyun_member SET {col} = ? WHERE uid = ?");
    sqlx::query(&sql).bind(sub).bind(uid).execute(pool).await?;
    Ok(())
}

pub async fn unbind_oauth_id(
    pool: &MySqlPool,
    uid: u64,
    provider: &str,
) -> Result<(), sqlx::Error> {
    let Some(col) = oauth_column_for(provider) else {
        return Ok(());
    };
    let sql = format!("UPDATE phpyun_member SET {col} = NULL WHERE uid = ?");
    sqlx::query(&sql).bind(uid).execute(pool).await?;
    Ok(())
}

/// Returns the OAuth binding state for the given member as a list of PHPYun-supported provider names.
pub async fn list_oauth_bindings(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<&'static str>, sqlx::Error> {
    let row: Option<(
        Option<String>, // qqid
        Option<String>, // sinaid
        Option<String>, // unionid (wechat)
        Option<String>, // wxopenid (wechat_mp)
        Option<String>, // bdopenid
    )> = sqlx::query_as(
        "SELECT qqid, sinaid, unionid, wxopenid, bdopenid FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;

    let mut out = Vec::new();
    if let Some((qq, sina, wx, wxmp, bd)) = row {
        if qq.as_deref().is_some_and(|s| !s.is_empty()) {
            out.push("qq");
        }
        if sina.as_deref().is_some_and(|s| !s.is_empty()) {
            out.push("weibo");
        }
        if wx.as_deref().is_some_and(|s| !s.is_empty()) {
            out.push("wechat");
        }
        if wxmp.as_deref().is_some_and(|s| !s.is_empty()) {
            out.push("wechat_mp");
        }
        if bd.as_deref().is_some_and(|s| !s.is_empty()) {
            out.push("baidu");
        }
    }
    Ok(out)
}

pub async fn update_email(pool: &MySqlPool, uid: u64, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_member SET email = ? WHERE uid = ?")
        .bind(email)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_mobile(pool: &MySqlPool, uid: u64, mobile: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_member SET moblie = ? WHERE uid = ?")
        .bind(mobile)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// First-time set of `usertype` (only when it is currently 0). PHPYun
/// `wap/login::setutype_action` flow: an OAuth-registered user picks a role
/// (1=jobseeker / 2=company / 3=campus) before entering the member centre.
/// Returns the affected row count — 0 means usertype was already set.
pub async fn set_usertype_if_unset(
    pool: &MySqlPool,
    uid: u64,
    usertype: u8,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member SET usertype = ? WHERE uid = ? AND usertype = 0")
        .bind(usertype as i32)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Force-set `usertype` regardless of current value. Used by the account-
/// split flow when an old account is being collapsed into the company role.
/// Generic over `Executor` so it can run inside a transaction.
pub async fn set_usertype<'e, E>(
    exec: E,
    uid: u64,
    usertype: i32,
) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    let res = sqlx::query("UPDATE phpyun_member SET usertype = ? WHERE uid = ?")
        .bind(usertype)
        .bind(uid)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}

/// Hard-delete a member row. Used by the account-merge flow once all of the
/// uid's data has been moved over. Generic so it can run inside a tx.
pub async fn delete_member<'e, E>(exec: E, uid: u64) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    let res = sqlx::query("DELETE FROM phpyun_member WHERE uid = ?")
        .bind(uid)
        .execute(exec)
        .await?;
    Ok(res.rows_affected())
}

// ==================== Admin backend ====================

pub struct AdminUserFilter<'a> {
    pub keyword: Option<&'a str>,
    pub usertype: Option<i32>,
    pub status: Option<i32>,
}

pub async fn admin_list(
    pool: &MySqlPool,
    f: &AdminUserFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Member>, sqlx::Error> {
    let mut sql = format!("SELECT {FIELDS} FROM phpyun_member WHERE 1=1");
    if f.keyword.is_some() {
        sql.push_str(" AND (username LIKE ? OR moblie LIKE ? OR email LIKE ?)");
    }
    if f.usertype.is_some() {
        sql.push_str(" AND usertype = ?");
    }
    if f.status.is_some() {
        sql.push_str(" AND status = ?");
    }
    sql.push_str(" ORDER BY uid DESC LIMIT ? OFFSET ?");

    let mut q = sqlx::query_as::<_, Member>(&sql);
    if let Some(kw) = f.keyword {
        let like = format!("%{kw}%");
        q = q.bind(like.clone()).bind(like.clone()).bind(like);
    }
    if let Some(u) = f.usertype {
        q = q.bind(u);
    }
    if let Some(s) = f.status {
        q = q.bind(s);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn admin_count(
    pool: &MySqlPool,
    f: &AdminUserFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from("SELECT COUNT(*) FROM phpyun_member WHERE 1=1");
    if f.keyword.is_some() {
        sql.push_str(" AND (username LIKE ? OR moblie LIKE ? OR email LIKE ?)");
    }
    if f.usertype.is_some() {
        sql.push_str(" AND usertype = ?");
    }
    if f.status.is_some() {
        sql.push_str(" AND status = ?");
    }

    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(kw) = f.keyword {
        let like = format!("%{kw}%");
        q = q.bind(like.clone()).bind(like.clone()).bind(like);
    }
    if let Some(u) = f.usertype {
        q = q.bind(u);
    }
    if let Some(s) = f.status {
        q = q.bind(s);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn admin_set_status(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member SET status = ? WHERE uid = ?")
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ==================== Company claim ====================

/// Returns PHPYun's "company claim code" — corresponds to the `phpyun_member.appeal` field.
/// (PHPYun's claim flow: generate an `appeal` string first, hand it to the user, and only update the
/// username once the user submits it back successfully.)
pub async fn get_claim_code(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT appeal FROM phpyun_member WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.and_then(|(v,)| v))
}

/// Renames username (one-shot; only allowed for users who have never changed it / claim=0).
/// On success → sets claim to 1 to prevent further changes. affected=0 means it's already been changed or the uid does not exist.
pub async fn rename_username_once(
    pool: &MySqlPool,
    uid: u64,
    new_username: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member SET username = ?, claim = 1 \
         WHERE uid = ? AND (claim = 0 OR claim IS NULL)",
    )
    .bind(new_username)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// On successful claim, atomically updates username / salt / password, clears the appeal field, and records the claim time.
pub async fn update_username_and_password(
    pool: &MySqlPool,
    uid: u64,
    username: &str,
    salt: &str,
    password_hash: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member \
         SET username = ?, salt = ?, password = ?, claim = 1, \
             appeal = NULL, appealtime = ?, appealstate = 1 \
         WHERE uid = ?",
    )
    .bind(username)
    .bind(salt)
    .bind(password_hash)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
