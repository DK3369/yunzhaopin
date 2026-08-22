//! Strictly aligned with PHPYun `phpyun_gongzhao` (public recruitment).
//!
//! PHP columns: id/title/keyword/description/content/datetime/did/startime/endtime/pic/rec
//! Rust Gongzhao field -> PHP column:
//!   - cover      <-> pic
//!   - body       <-> content
//!   - tag        <-> keyword
//!   - status     = 1 (no status column in PHP)
//!   - view_count = 0 (no view-count column in PHP; Rust stub)
//!   - start_at   <-> startime
//!   - end_at     <-> endtime
//!   - created_at <-> datetime

use super::entity::Gongzhao;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(description, '') AS description, \
    COALESCE(pic, '') AS cover, \
    COALESCE(content, '') AS body, \
    COALESCE(keyword, '') AS tag, \
    CAST(1 AS SIGNED) AS status, \
    CAST(0 AS UNSIGNED) AS view_count, \
    CAST(COALESCE(startime, 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(endtime, 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(datetime, 0) AS SIGNED) AS created_at";

pub async fn list(
    pool: &MySqlPool,
    tag: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Gongzhao>, sqlx::Error> {
    let sql = match tag {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao WHERE keyword = ? \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_gongzhao \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Gongzhao>(&sql);
    match tag {
        Some(t) => q.bind(t).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count(pool: &MySqlPool, tag: Option<&str>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match tag {
        Some(t) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_gongzhao WHERE keyword = ?")
                .bind(t)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_gongzhao")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Gongzhao>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_gongzhao WHERE id = ?");
    sqlx::query_as::<_, Gongzhao>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_view(_pool: &MySqlPool, _id: u64) -> Result<(), sqlx::Error> {
    // PHPYun phpyun_gongzhao has no view-count column; this op is a no-op.
    Ok(())
}
