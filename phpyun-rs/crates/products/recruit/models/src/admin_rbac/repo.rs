//! PHP `phpyun_admin_user` / `phpyun_admin_user_group`.
//! JWT 仍用 `phpyun_member.usertype=3` 进后台；这里只读 PHP 角色表，不解析 `group_power` 序列化。

use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

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
