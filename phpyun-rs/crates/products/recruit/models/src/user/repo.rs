//! `phpyun_member` table access. All SQL is centralized here; the service layer never writes SQL directly.
//!
//! Key points for aligning with the PHPYun schema:
//! - `uid / did` are int(11) signed, so sqlx's `u64`/`u32` need a CAST to UNSIGNED
//! - `reg_date / login_date` are int(11) and need CAST(AS SIGNED) for `i64`
//! - `usertype / status` are int(1)/int(4); using `i32` on the Rust side is safer
//! - **OAuth columns are not** google_id/fb_id/apple_sub — PHPYun actually has:
//!   qqid / qqunionid / sinaid / wxid / wxopenid / unionid / wxname / bdopenid
//!   So OAuth binding on the Rust side uses an allowlist mapping provider → real PHP column name.

use super::entity::{AdminAppealListRow, AdminMemberListRow, Member};
use sqlx::{MySqlPool, QueryBuilder};

type OAuthBindingsRow = (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

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
        "wechat" | "weixin" => Some("unionid"), // On the Rust side, WeChat accounts are keyed by unionid
        "wechat_mp" | "wxopenid" => Some("wxopenid"), // Official Account / Mini Program openid
        "baidu" => Some("bdopenid"),
        // Overseas providers (PHPYun has no column; we keep the interface but return None, so upstream gets "bind failed")
        "google" | "facebook" | "apple" => None,
        _ => None,
    }
}

// ==================== Queries ====================

pub async fn find_for_login(
    pool: &MySqlPool,
    account: &str,
) -> Result<Option<Member>, sqlx::Error> {
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
    exists_username_except(pool, username, None).await
}

pub async fn exists_username_except(
    pool: &MySqlPool,
    username: &str,
    except_uid: Option<u64>,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = if let Some(uid) = except_uid {
        sqlx::query_as(
            "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
             WHERE username = ? AND uid <> ? LIMIT 1",
        )
        .bind(username)
        .bind(uid)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE username = ? LIMIT 1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await?
    };
    Ok(row.is_some())
}

pub async fn exists_mobile_or_username(pool: &MySqlPool, value: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
         WHERE moblie = ? OR username = ? LIMIT 1",
    )
    .bind(value)
    .bind(value)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn exists_email_or_username(pool: &MySqlPool, value: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
         WHERE email = ? OR username = ? LIMIT 1",
    )
    .bind(value)
    .bind(value)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn set_address<'e, E>(exec: E, uid: u64, address: &str) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query("UPDATE phpyun_member SET address = ? WHERE uid = ?")
        .bind(address)
        .bind(uid)
        .execute(exec)
        .await?;
    Ok(())
}

pub async fn exists_mobile(pool: &MySqlPool, mobile: &str) -> Result<bool, sqlx::Error> {
    exists_mobile_except(pool, mobile, None).await
}

pub async fn exists_mobile_except(
    pool: &MySqlPool,
    mobile: &str,
    except_uid: Option<u64>,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = if let Some(uid) = except_uid {
        sqlx::query_as(
            "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
             WHERE moblie = ? AND uid <> ? LIMIT 1",
        )
        .bind(mobile)
        .bind(uid)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as("SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE moblie = ? LIMIT 1")
            .bind(mobile)
            .fetch_optional(pool)
            .await?
    };
    Ok(row.is_some())
}

pub async fn exists_email(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    exists_email_except(pool, email, None).await
}

pub async fn exists_email_except(
    pool: &MySqlPool,
    email: &str,
    except_uid: Option<u64>,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = if let Some(uid) = except_uid {
        sqlx::query_as(
            "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member \
             WHERE email = ? AND uid <> ? LIMIT 1",
        )
        .bind(email)
        .bind(uid)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as("SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE email = ? LIMIT 1")
            .bind(email)
            .fetch_optional(pool)
            .await?
    };
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
    .bind(i32::from(usertype))
    .bind(did)
    .bind(reg_date)
    .bind(reg_ip)
    .bind(reg_date)
    .execute(exec)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_password(
    pool: &MySqlPool,
    uid: u64,
    new_hash: &str,
) -> Result<(), sqlx::Error> {
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
    let row: Option<OAuthBindingsRow> = sqlx::query_as(
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
        .bind(i32::from(usertype))
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Force-set `usertype` regardless of current value. Used by the account-
/// split flow when an old account is being collapsed into the company role.
/// Generic over `Executor` so it can run inside a transaction.
pub async fn set_usertype<'e, E>(exec: E, uid: u64, usertype: i32) -> Result<u64, sqlx::Error>
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

pub async fn admin_count(pool: &MySqlPool, f: &AdminUserFilter<'_>) -> Result<u64, sqlx::Error> {
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `msgNum::memNumV1`: total members with `pid = 0` (not child accounts).
pub async fn count_admin_pid0(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_member WHERE pid = 0")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `msgNum::memNumV1`: members with a given `status` (2 = locked).
pub async fn count_admin_status(pool: &MySqlPool, status: i32) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_member WHERE status = ?")
        .bind(status)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

const MEMBER_LIST_FIELDS: &str = "CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
    COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie, \
    CAST(COALESCE(moblie_status,0) AS SIGNED) AS moblie_status, COALESCE(reg_ip,'') AS reg_ip, \
    CAST(COALESCE(reg_date,0) AS SIGNED) AS reg_date, COALESCE(login_ip,'') AS login_ip, \
    CAST(COALESCE(login_date,0) AS SIGNED) AS login_date, CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, \
    CAST(COALESCE(status,0) AS SIGNED) AS status, COALESCE(lock_info,'') AS lock_info, \
    CAST(COALESCE(source,0) AS SIGNED) AS source, CAST(COALESCE(did,0) AS UNSIGNED) AS did, \
    COALESCE(login_address,'') AS login_address, COALESCE(moblie_address,'') AS moblie_address";

pub struct PhpMemberListFilter<'a> {
    pub usertype: Option<i32>,
    pub status: Option<i32>,
    pub source: Option<i32>,
    pub keyword: Option<&'a str>,
    pub kw_type: i32,
    pub time_col: Option<&'a str>,
    pub time_from: Option<i64>,
    pub time_to: Option<i64>,
}

fn push_php_member_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpMemberListFilter<'a>) {
    qb.push(" AND pid = 0");
    if let Some(t) = f.usertype {
        qb.push(" AND usertype = ");
        qb.push_bind(t);
    }
    if let Some(s) = f.status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    if let Some(src) = f.source.filter(|v| *v > 0) {
        qb.push(" AND source = ");
        qb.push_bind(src);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        match f.kw_type {
            2 => {
                qb.push(" AND moblie LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            3 => {
                let uid: u64 = kw.parse().unwrap_or(0);
                qb.push(" AND uid = ");
                qb.push_bind(uid);
            }
            4 => {
                qb.push(" AND (reg_ip LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(" OR login_ip LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(")");
            }
            _ => {
                qb.push(" AND username LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
        }
    }
    if let (Some(col), Some(from), Some(to)) = (f.time_col, f.time_from, f.time_to) {
        let col = match col {
            "login_date" => "login_date",
            _ => "reg_date",
        };
        qb.push(" AND ");
        qb.push(col);
        qb.push(" >= ");
        qb.push_bind(from);
        qb.push(" AND ");
        qb.push(col);
        qb.push(" <= ");
        qb.push_bind(to);
    }
}

pub async fn list_php_members(
    pool: &MySqlPool,
    f: &PhpMemberListFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminMemberListRow>, sqlx::Error> {
    let limit = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let offset = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(MEMBER_LIST_FIELDS);
    qb.push(" FROM phpyun_member WHERE 1=1");
    push_php_member_filters(&mut qb, f);
    qb.push(" ORDER BY uid DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_php_members(
    pool: &MySqlPool,
    f: &PhpMemberListFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_member WHERE 1=1");
    push_php_member_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_php_appeals(
    pool: &MySqlPool,
    keyword: Option<&str>,
    appealstate: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminAppealListRow>, sqlx::Error> {
    let limit = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let offset = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(appeal,'') AS appeal, CAST(COALESCE(appealtime,0) AS SIGNED) AS appealtime, \
         CAST(COALESCE(appealstate,0) AS SIGNED) AS appealstate, COALESCE(moblie,'') AS moblie, \
         COALESCE(email,'') AS email FROM phpyun_member WHERE appeal IS NOT NULL AND appeal <> ''",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND username LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(st) = appealstate {
        qb.push(" AND appealstate = ");
        qb.push_bind(st);
    }
    qb.push(" ORDER BY appealstate ASC, appealtime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_php_appeals(
    pool: &MySqlPool,
    keyword: Option<&str>,
    appealstate: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_member WHERE appeal IS NOT NULL AND appeal <> ''",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND username LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(st) = appealstate {
        qb.push(" AND appealstate = ");
        qb.push_bind(st);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_set_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member SET status = ? WHERE uid = ?")
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminMemberExtras {
    pub username: String,
    pub status: i32,
    pub lock_info: String,
    pub reg_ip: String,
    pub reg_date: i64,
    pub source: i32,
    pub wxid: String,
    pub wxopenid: String,
    pub login_date: i64,
}

pub async fn find_admin_extras(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<AdminMemberExtras>, sqlx::Error> {
    sqlx::query_as(
        "SELECT COALESCE(username,'') AS username, CAST(COALESCE(status,0) AS SIGNED) AS status, \
         COALESCE(lock_info,'') AS lock_info, COALESCE(reg_ip,'') AS reg_ip, \
         CAST(COALESCE(reg_date,0) AS SIGNED) AS reg_date, \
         CAST(COALESCE(source,0) AS SIGNED) AS source, \
         COALESCE(wxid,'') AS wxid, COALESCE(wxopenid,'') AS wxopenid, \
         CAST(COALESCE(login_date,0) AS SIGNED) AS login_date \
         FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

pub async fn update_contact(
    pool: &MySqlPool,
    uid: u64,
    email: &str,
    mobile: &str,
    address: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_member SET email = ?, moblie = ?, address = ? WHERE uid = ?")
        .bind(email)
        .bind(mobile)
        .bind(address)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_admin_account(
    pool: &MySqlPool,
    uid: u64,
    username: &str,
    status: i32,
    lock_info: &str,
    password_hash: Option<(&str, &str)>,
) -> Result<u64, sqlx::Error> {
    let res = if let Some((hash, salt)) = password_hash {
        sqlx::query(
            "UPDATE phpyun_member SET username = ?, status = ?, lock_info = ?, password = ?, salt = ? \
             WHERE uid = ?",
        )
        .bind(username)
        .bind(status)
        .bind(lock_info)
        .bind(hash)
        .bind(salt)
        .bind(uid)
        .execute(pool)
        .await?
    } else {
        sqlx::query(
            "UPDATE phpyun_member SET username = ?, status = ?, lock_info = ? WHERE uid = ?",
        )
        .bind(username)
        .bind(status)
        .bind(lock_info)
        .bind(uid)
        .execute(pool)
        .await?
    };
    Ok(res.rows_affected())
}

// ==================== Company claim ====================

/// Returns PHPYun's "company claim code" — corresponds to the `phpyun_member.appeal` field.
/// (PHPYun's claim flow: generate an `appeal` string first, hand it to the user, and only update the
/// username once the user submits it back successfully.)
pub async fn get_claim_code(pool: &MySqlPool, uid: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT appeal FROM phpyun_member WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.and_then(|(v,)| v))
}

/// PHP `$ComMember.source==6 && claim==0 && email!=''`.
pub async fn claim_eligibility(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<(i32, i32, bool)>, sqlx::Error> {
    let row: Option<(i32, i32, Option<String>)> = sqlx::query_as(
        "SELECT CAST(COALESCE(source,0) AS SIGNED), CAST(COALESCE(claim,0) AS SIGNED), email \
         FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(source, claim, email)| {
        (source, claim, email.as_deref().is_some_and(|s| !s.trim().is_empty()))
    }))
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
         WHERE uid = ? AND (claim = 0 OR claim IS NULL)",
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

/// PHP `admin_appeal::info_action` member row (`getInfo` without field list).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpMemberDetail {
    pub uid: u64,
    pub username: String,
    pub email: String,
    pub moblie: String,
    pub usertype: i32,
    pub status: i32,
    pub did: u64,
    pub reg_date: i64,
    pub login_date: i64,
    pub login_hits: i32,
    pub lock_info: String,
    pub appeal: String,
    pub appealtime: i64,
    pub appealstate: i32,
    pub login_ip: String,
    pub reg_ip: String,
    pub address: String,
}

pub async fn find_php_member_detail(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<PhpMemberDetail>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, \
         CAST(COALESCE(status,0) AS SIGNED) AS status, \
         CAST(COALESCE(did,0) AS UNSIGNED) AS did, \
         CAST(COALESCE(reg_date,0) AS SIGNED) AS reg_date, \
         CAST(COALESCE(login_date,0) AS SIGNED) AS login_date, \
         CAST(COALESCE(login_hits,0) AS SIGNED) AS login_hits, \
         COALESCE(lock_info,'') AS lock_info, COALESCE(appeal,'') AS appeal, \
         CAST(COALESCE(appealtime,0) AS SIGNED) AS appealtime, \
         CAST(COALESCE(appealstate,0) AS SIGNED) AS appealstate, \
         COALESCE(login_ip,'') AS login_ip, COALESCE(reg_ip,'') AS reg_ip, \
         COALESCE(address,'') AS address \
         FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// PHP `userinfo::lock` → `upInfo` status + lock_info.
pub async fn update_lock(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    lock_info: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member SET status = ?, lock_info = ? WHERE uid = ?")
        .bind(status)
        .bind(lock_info)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// PHP `userinfo::commonLock`: resume / company / expect / job / part `r_status`.
pub async fn lock_related_r_status(
    pool: &MySqlPool,
    uid: u64,
    r_status: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_company SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_resume_expect SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_company_job SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_partjob SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub struct PhpMemberEdit<'a> {
    pub username: &'a str,
    pub mobile: &'a str,
    pub email: &'a str,
    pub reg_ip: &'a str,
    pub did: u64,
    pub status: i32,
    pub password: Option<(&'a str, &'a str)>,
}

/// PHP `userinfo::upMemberInfo` member row (admin).
pub async fn update_php_admin_member(
    pool: &MySqlPool,
    uid: u64,
    e: &PhpMemberEdit<'_>,
) -> Result<u64, sqlx::Error> {
    let res = if let Some((hash, salt)) = e.password {
        sqlx::query(
            "UPDATE phpyun_member SET username = ?, moblie = ?, email = ?, reg_ip = ?, did = ?, \
             status = ?, password = ?, salt = ? WHERE uid = ?",
        )
        .bind(e.username)
        .bind(e.mobile)
        .bind(e.email)
        .bind(e.reg_ip)
        .bind(e.did)
        .bind(e.status)
        .bind(hash)
        .bind(salt)
        .bind(uid)
        .execute(pool)
        .await?
    } else {
        sqlx::query(
            "UPDATE phpyun_member SET username = ?, moblie = ?, email = ?, reg_ip = ?, did = ?, \
             status = ? WHERE uid = ?",
        )
        .bind(e.username)
        .bind(e.mobile)
        .bind(e.email)
        .bind(e.reg_ip)
        .bind(e.did)
        .bind(e.status)
        .bind(uid)
        .execute(pool)
        .await?
    };
    Ok(res.rows_affected())
}

/// PHP `upMemberInfo` also writes resume.telphone/email and company.linktel/linkmail.
pub async fn sync_php_profile_contact(
    pool: &MySqlPool,
    uid: u64,
    mobile: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume SET telphone = ?, email = ? WHERE uid = ?")
        .bind(mobile)
        .bind(email)
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_company SET linktel = ?, linkmail = ? WHERE uid = ?")
        .bind(mobile)
        .bind(email)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// PHP `userinfo::delMember` core: drop member + related profile/job rows.
pub async fn delete_php_members(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    for sql in [
        "DELETE FROM phpyun_resume_expect WHERE uid IN (",
        "DELETE FROM phpyun_company_job WHERE uid IN (",
        "DELETE FROM phpyun_partjob WHERE uid IN (",
        "DELETE FROM phpyun_resume WHERE uid IN (",
        "DELETE FROM phpyun_company WHERE uid IN (",
    ] {
        let mut qb = QueryBuilder::new(sql);
        let mut sep = qb.separated(", ");
        for uid in uids {
            sep.push_bind(*uid);
        }
        qb.push(")");
        qb.build().execute(pool).await?;
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_member WHERE uid IN (");
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn update_appeal_state(
    pool: &MySqlPool,
    uid: u64,
    appealstate: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member SET appealstate = ? WHERE uid = ?")
        .bind(appealstate)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// PHP appeal `del_action`: clear appeal text, keep the member row.
pub async fn clear_appeals(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(
        "UPDATE phpyun_member SET appeal = '', appealtime = 0, appealstate = 1 WHERE uid IN (",
    );
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

/// PHP `logout::status` member anonymize (skip mail/SMS).
pub async fn anonymize_logout_member(
    pool: &MySqlPool,
    uid: u64,
    username: &str,
    mobile: &str,
    email: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member SET username = ?, moblie = ?, email = ?, status = 2, \
         lock_info = 'common_06533', pwuid = 0, pw_repeat = 0, \
         qqid = '', qqunionid = '', sinaid = '', wxid = '', wxopenid = '', unionid = '', \
         wxname = '', wxbindtime = 0, clientid = '', deviceToken = '', maguid = 0, qfyuid = 0 \
         WHERE uid = ?",
    )
    .bind(username)
    .bind(mobile)
    .bind(email)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
