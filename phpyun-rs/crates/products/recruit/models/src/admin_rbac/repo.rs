//! PHP `phpyun_admin_user` / `phpyun_admin_user_group` / `phpyun_admin_navigation`.

use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::php_power;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AdminRbacUser {
    pub uid: u64,
    pub m_id: i32,
    pub username: String,
    pub name: String,
    pub status: i32,
    pub group_name: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AdminRbacGroup {
    pub id: u64,
    pub group_name: String,
}

pub async fn list_users(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminRbacUser>, sqlx::Error> {
    sqlx::query_as::<_, AdminRbacUser>(
        r#"SELECT CAST(u.uid AS UNSIGNED) AS uid,
                  CAST(COALESCE(u.m_id, 0) AS SIGNED) AS m_id,
                  COALESCE(u.username, '') AS username,
                  COALESCE(u.name, '') AS name,
                  CAST(COALESCE(u.status, 0) AS SIGNED) AS status,
                  COALESCE(g.group_name, '') AS group_name
           FROM phpyun_admin_user u
           LEFT JOIN phpyun_admin_user_group g ON g.id = u.m_id
           ORDER BY u.uid ASC
           LIMIT ? OFFSET ?"#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_users(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_user")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_groups(pool: &MySqlPool) -> Result<Vec<AdminRbacGroup>, sqlx::Error> {
    sqlx::query_as::<_, AdminRbacGroup>(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  COALESCE(group_name, '') AS group_name
           FROM phpyun_admin_user_group
           ORDER BY id ASC"#,
    )
    .fetch_all(pool)
    .await
}

pub async fn set_user_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_admin_user SET status = ? WHERE uid = ?")
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct AdminLoginRow {
    pub uid: u64,
    pub m_id: i32,
    pub username: String,
    pub name: String,
    pub password: String,
    pub status: i32,
    pub did: u64,
    pub control_login: String,
}

pub async fn find_login_user(
    pool: &MySqlPool,
    username: &str,
) -> Result<Option<AdminLoginRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminLoginRow>(
        r#"SELECT CAST(uid AS UNSIGNED) AS uid,
                  CAST(COALESCE(m_id, 0) AS SIGNED) AS m_id,
                  COALESCE(username, '') AS username,
                  COALESCE(name, '') AS name,
                  COALESCE(password, '') AS password,
                  CAST(COALESCE(status, 0) AS SIGNED) AS status,
                  CAST(COALESCE(did, 0) AS UNSIGNED) AS did,
                  COALESCE(control_login, '') AS control_login
           FROM phpyun_admin_user
           WHERE username = ? AND status = 1
           LIMIT 1"#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<AdminLoginRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminLoginRow>(
        r#"SELECT CAST(uid AS UNSIGNED) AS uid,
                  CAST(COALESCE(m_id, 0) AS SIGNED) AS m_id,
                  COALESCE(username, '') AS username,
                  COALESCE(name, '') AS name,
                  COALESCE(password, '') AS password,
                  CAST(COALESCE(status, 0) AS SIGNED) AS status,
                  CAST(COALESCE(did, 0) AS UNSIGNED) AS did,
                  COALESCE(control_login, '') AS control_login
           FROM phpyun_admin_user
           WHERE uid = ?
           LIMIT 1"#,
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

pub async fn touch_lasttime(pool: &MySqlPool, uid: u64, ts: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_admin_user SET lasttime = ? WHERE uid = ?")
        .bind(ts)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn group_power_ids(pool: &MySqlPool, m_id: i32) -> Result<Vec<i64>, sqlx::Error> {
    let row: Option<(Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT group_name, group_power FROM phpyun_admin_user_group WHERE id = ? LIMIT 1",
    )
    .bind(m_id)
    .fetch_optional(pool)
    .await?;
    Ok(row
        .and_then(|(_, power)| power)
        .map(|p| php_power::parse_group_power(&p))
        .unwrap_or_default())
}

pub async fn group_name(pool: &MySqlPool, m_id: i32) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(group_name, '') FROM phpyun_admin_user_group WHERE id = ?")
            .bind(m_id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|(n,)| n).unwrap_or_default())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AdminNavRow {
    pub id: i64,
    pub keyid: i64,
    pub name: String,
    pub url: String,
    pub path: String,
    pub classname: String,
    pub menu: i32,
    pub sort: i32,
    pub display: i32,
}

pub async fn list_navigation(pool: &MySqlPool) -> Result<Vec<AdminNavRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminNavRow>(
        r#"SELECT CAST(id AS SIGNED) AS id,
                  CAST(COALESCE(keyid, 0) AS SIGNED) AS keyid,
                  COALESCE(name, '') AS name,
                  COALESCE(url, '') AS url,
                  COALESCE(path, '') AS path,
                  COALESCE(classname, '') AS classname,
                  CAST(COALESCE(menu, 0) AS SIGNED) AS menu,
                  CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
                  CAST(COALESCE(display, 0) AS SIGNED) AS display
           FROM phpyun_admin_navigation
           WHERE display <> 1
           ORDER BY sort ASC, id ASC"#,
    )
    .fetch_all(pool)
    .await
}
