//! Strictly aligned with PHPYun `phpyun_admin_announcement` (announcements).
//! Actual PHP columns: id/title/keyword/description/content/datetime/startime/endtime/did/view_num.
//! **PHP has no `status` or `created_at` column** -- `status` is derived from
//! startime/endtime, and `created_at` falls back to `datetime`.

use super::entity::Announcement;
use sqlx::MySqlPool;

/// Map PHPYun columns to Rust Announcement struct fields via aliases.
const SELECT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(description, '') AS description, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(view_num, 0) AS UNSIGNED) AS view_num, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS datetime, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS startime, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS endtime, \
    CAST(IFNULL(NULLIF(did, ''), '0') AS UNSIGNED) AS did, \
    CAST(1 AS SIGNED) AS status, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS created_at";

/// PHPYun's "published" predicate (no status column): startime<=now<endtime
/// (or endtime=0 meaning permanent).
const PUBLISHED_WHERE: &str =
    " (startime = 0 OR startime <= UNIX_TIMESTAMP()) \
      AND (endtime = 0 OR endtime > UNIX_TIMESTAMP())";

pub async fn list_published(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Announcement>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement \
         WHERE {PUBLISHED_WHERE} ORDER BY datetime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Announcement>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_published(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_admin_announcement WHERE {PUBLISHED_WHERE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<Announcement>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_admin_announcement WHERE id = ?"
    );
    sqlx::query_as::<_, Announcement>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Increment view count by 1.
pub async fn incr_view(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res =
        sqlx::query("UPDATE phpyun_admin_announcement SET view_num = view_num + 1 WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
    Ok(res.rows_affected())
}
