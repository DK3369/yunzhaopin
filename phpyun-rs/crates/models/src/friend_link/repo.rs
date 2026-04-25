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
