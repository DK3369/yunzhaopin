//! PHPYun `phpyun_wxnav` (custom WeChat menu). Read-only list for admin.

use super::entity::WxNav;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(keyid, 0) AS SIGNED) AS keyid, \
    COALESCE(`key`, '') AS `key`, \
    COALESCE(url, '') AS url, \
    COALESCE(`type`, '') AS nav_type, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    COALESCE(appid, '') AS appid, \
    COALESCE(apppage, '') AS apppage";

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<WxNav>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_wxnav ORDER BY keyid ASC, sort ASC, id ASC"
    );
    sqlx::query_as::<_, WxNav>(&sql).fetch_all(pool).await
}
