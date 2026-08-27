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

pub async fn upsert(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    keyid: i32,
    key: &str,
    url: &str,
    nav_type: &str,
    sort: i32,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id {
        sqlx::query(
            "UPDATE phpyun_wxnav SET name=?, keyid=?, `key`=?, url=?, `type`=?, sort=? WHERE id=?",
        )
        .bind(name)
        .bind(keyid)
        .bind(key)
        .bind(url)
        .bind(nav_type)
        .bind(sort)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_wxnav (name, keyid, `key`, url, `type`, sort) VALUES (?,?,?,?,?,?)",
        )
        .bind(name)
        .bind(keyid)
        .bind(key)
        .bind(url)
        .bind(nav_type)
        .bind(sort)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_wxnav WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<WxNav>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_wxnav ORDER BY keyid ASC, sort ASC, id ASC"
    );
    sqlx::query_as::<_, WxNav>(&sql).fetch_all(pool).await
}
