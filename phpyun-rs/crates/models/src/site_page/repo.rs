//! Strictly aligned with PHPYun `phpyun_templates` (site pages).
//!
//! PHP columns: id/name/title/content. Rust SitePage.code uses PHP's `name` field
//! (unique identifier); updated_at = 0 (PHP does not maintain it).

use super::entity::SitePage;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    COALESCE(name, '') AS code, \
    COALESCE(title, '') AS title, \
    COALESCE(content, '') AS content, \
    CAST(0 AS SIGNED) AS updated_at";

pub async fn find_by_code(
    pool: &MySqlPool,
    code: &str,
) -> Result<Option<SitePage>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_templates WHERE name = ? LIMIT 1");
    sqlx::query_as::<_, SitePage>(&sql)
        .bind(code)
        .fetch_optional(pool)
        .await
}
