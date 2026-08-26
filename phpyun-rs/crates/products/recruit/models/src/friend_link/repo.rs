//! Strictly aligned with PHPYun `phpyun_admin_link` (friendly links).
//!
//! Rust FriendLink field -> PHP column:
//!   - name       <-> link_name
//!   - url        <-> link_url
//!   - logo       <-> pic
//!   - category   <-> link_type
//!   - sort       <-> link_sorting
//!   - status     <-> link_state
//!   - created_at = 0 (PHP `link_time` is varchar, not a timestamp)

use super::entity::FriendLink;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(link_name, '') AS name, \
    COALESCE(link_url, '') AS url, \
    COALESCE(pic, '') AS logo, \
    COALESCE(link_type, '') AS category, \
    CAST(COALESCE(link_sorting, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(link_state, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS created_at";

pub async fn list_active(
    pool: &MySqlPool,
    category: Option<&str>,
) -> Result<Vec<FriendLink>, sqlx::Error> {
    let sql = match category {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_admin_link \
             WHERE link_state = 1 AND link_type = ? \
             ORDER BY link_sorting DESC, id ASC"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_admin_link \
             WHERE link_state = 1 \
             ORDER BY link_sorting DESC, id ASC"
        ),
    };
    let q = sqlx::query_as::<_, FriendLink>(&sql);
    match category {
        Some(c) => q.bind(c).fetch_all(pool).await,
        None => q.fetch_all(pool).await,
    }
}

pub async fn list_all(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<FriendLink>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_link \
         ORDER BY link_sorting DESC, id ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, FriendLink>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_all(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_link")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct FriendLinkUpsert<'a> {
    pub id: Option<u64>,
    pub link_name: &'a str,
    pub link_url: &'a str,
    pub pic: &'a str,
    pub link_type: &'a str,
    pub link_sorting: i32,
    pub link_state: i32,
}

pub async fn upsert(pool: &MySqlPool, a: FriendLinkUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_admin_link
               SET link_name = ?, link_url = ?, pic = ?, link_type = ?,
                   link_sorting = ?, link_state = ?
               WHERE id = ?"#,
        )
        .bind(a.link_name)
        .bind(a.link_url)
        .bind(a.pic)
        .bind(a.link_type)
        .bind(a.link_sorting)
        .bind(a.link_state)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_admin_link
           (link_name, link_url, pic, link_type, link_sorting, link_state)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(a.link_name)
    .bind(a.link_url)
    .bind(a.pic)
    .bind(a.link_type)
    .bind(a.link_sorting)
    .bind(a.link_state)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_admin_link WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
