//! PHPYun `phpyun_wxnav` (custom WeChat menu). Read-only list for admin.

use super::entity::WxNav;
use crate::soft_delete::{self, PREDICATE};
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

pub async fn upsert_php(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    keyid: i32,
    key: &str,
    url: &str,
    nav_type: &str,
    sort: i32,
    appid: &str,
    apppage: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|i| *i > 0) {
        sqlx::query(
            "UPDATE phpyun_wxnav SET name=?, keyid=?, `key`=?, url=?, `type`=?, sort=?, \
             appid=?, apppage=? WHERE id=?",
        )
        .bind(name)
        .bind(keyid)
        .bind(key)
        .bind(url)
        .bind(nav_type)
        .bind(sort)
        .bind(appid)
        .bind(apppage)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_wxnav (name, keyid, `key`, url, `type`, sort, appid, apppage) \
             VALUES (?,?,?,?,?,?,?,?)",
        )
        .bind(name)
        .bind(keyid)
        .bind(key)
        .bind(url)
        .bind(nav_type)
        .bind(sort)
        .bind(appid)
        .bind(apppage)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn count_dup_name(
    pool: &MySqlPool,
    name: &str,
    keyid: i32,
    except_id: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_wxnav WHERE name = ? AND keyid = ? \
         AND COALESCE(deleted,0)=0 AND id <> ?",
    )
    .bind(name)
    .bind(keyid)
    .bind(except_id)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn patch_field(pool: &MySqlPool, id: u64, sort: Option<i32>, name: Option<&str>) -> Result<u64, sqlx::Error> {
    if let Some(s) = sort {
        let res = sqlx::query("UPDATE phpyun_wxnav SET sort = ? WHERE id = ?")
            .bind(s)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(res.rows_affected());
    }
    if let Some(n) = name {
        let res = sqlx::query("UPDATE phpyun_wxnav SET name = ? WHERE id = ?")
            .bind(n)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(res.rows_affected());
    }
    Ok(0)
}

pub async fn delete_with_children(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut n = 0u64;
    for id in ids {
        n += soft_delete::mark_id(pool, "phpyun_wxnav", *id).await?;
        let res = sqlx::query(
            "UPDATE phpyun_wxnav SET deleted=1 WHERE COALESCE(deleted,0)=0 AND keyid = ?",
        )
        .bind(*id as i64)
        .execute(pool)
        .await?;
        n += res.rows_affected();
    }
    Ok(n)
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_wxnav", id).await
}

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<WxNav>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_wxnav WHERE {PREDICATE} ORDER BY keyid ASC, sort ASC, id ASC"
    );
    sqlx::query_as::<_, WxNav>(&sql).fetch_all(pool).await
}
