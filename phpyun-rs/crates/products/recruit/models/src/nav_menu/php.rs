//! PHP-shaped `phpyun_navigation` / `phpyun_navigation_type` queries.

use serde::Serialize;
use sqlx::{FromRow, MySqlPool, QueryBuilder};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpNavRow {
    pub id: u64,
    pub nid: i32,
    pub name: String,
    pub url: String,
    pub furl: String,
    pub sort: i32,
    pub display: i32,
    pub eject: i32,
    pub r#type: i32,
    pub color: String,
    pub model: String,
    pub bold: i32,
    pub pic: String,
    pub desc: String,
    pub news: String,
    pub config: String,
}

const NAV_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(nid,0) AS SIGNED) AS nid, \
    COALESCE(name,'') AS name, \
    COALESCE(url,'') AS url, \
    COALESCE(furl,'') AS furl, \
    CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(display,0) AS SIGNED) AS display, \
    CAST(COALESCE(eject,0) AS SIGNED) AS eject, \
    CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
    COALESCE(color,'') AS color, \
    COALESCE(model,'') AS model, \
    CAST(COALESCE(bold,0) AS SIGNED) AS bold, \
    COALESCE(pic,'') AS pic, \
    COALESCE(`desc`,'') AS `desc`, \
    COALESCE(news,'') AS news, \
    COALESCE(config,'') AS config";

#[derive(Debug, Default, Clone)]
pub struct PhpNavFilter {
    pub type_eq: Option<i32>,
    pub eject: Option<i32>,
    pub display: Option<i32>,
    pub nid: Option<i32>,
    pub keyword: Option<String>,
}

fn push_nav_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpNavFilter) {
    qb.push(" FROM phpyun_navigation WHERE 1=1");
    if let Some(t) = f.type_eq {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(v) = f.eject {
        qb.push(" AND eject = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.display {
        qb.push(" AND display = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.nid {
        qb.push(" AND nid = ");
        qb.push_bind(v);
    }
    if let Some(kw) = f.keyword.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
}

pub async fn php_list_nav(
    pool: &MySqlPool,
    f: &PhpNavFilter,
    offset: u64,
    limit: u64,
    order_col: &str,
    order_dir: &str,
) -> Result<Vec<PhpNavRow>, sqlx::Error> {
    let col = match order_col {
        "sort" => "sort",
        "name" => "name",
        "nid" => "nid",
        _ => "id",
    };
    let dir = if order_dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };
    let mut qb = QueryBuilder::new("SELECT ");
    qb.push(NAV_FIELDS);
    push_nav_where(&mut qb, f);
    qb.push(format!(" ORDER BY {col} {dir}, id DESC LIMIT "));
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_nav(pool: &MySqlPool, f: &PhpNavFilter) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_nav_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_get_nav(pool: &MySqlPool, id: u64) -> Result<Option<PhpNavRow>, sqlx::Error> {
    let sql = format!("SELECT {NAV_FIELDS} FROM phpyun_navigation WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PhpNavRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn php_get_nav_by_config(
    pool: &MySqlPool,
    config: &str,
) -> Result<Option<PhpNavRow>, sqlx::Error> {
    let sql = format!("SELECT {NAV_FIELDS} FROM phpyun_navigation WHERE config = ? LIMIT 1");
    sqlx::query_as::<_, PhpNavRow>(&sql)
        .bind(config)
        .fetch_optional(pool)
        .await
}

pub async fn php_get_nav_by_news(pool: &MySqlPool, news_id: i64) -> Result<Option<PhpNavRow>, sqlx::Error> {
    let sql = format!("SELECT {NAV_FIELDS} FROM phpyun_navigation WHERE news = ? LIMIT 1");
    sqlx::query_as::<_, PhpNavRow>(&sql)
        .bind(news_id)
        .fetch_optional(pool)
        .await
}

pub async fn php_set_nav_news(pool: &MySqlPool, id: u64, news: i64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_navigation SET news = ? WHERE id = ?")
        .bind(news)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_nav_name_taken(pool: &MySqlPool, name: &str, nid: i32) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_navigation WHERE name = ? AND nid = ?",
    )
    .bind(name)
    .bind(nid)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

pub struct PhpNavSave<'a> {
    pub id: Option<u64>,
    pub nid: i32,
    pub eject: i32,
    pub display: i32,
    pub name: &'a str,
    pub url: &'a str,
    pub furl: &'a str,
    pub sort: i32,
    pub color: &'a str,
    pub model: &'a str,
    pub bold: i32,
    pub r#type: i32,
    pub pic: Option<&'a str>,
    pub config: &'a str,
}

pub async fn php_save_nav(pool: &MySqlPool, s: PhpNavSave<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = s.id.filter(|n| *n > 0) {
        if let Some(pic) = s.pic.filter(|p| !p.is_empty()) {
            sqlx::query(
                "UPDATE phpyun_navigation SET nid=?, eject=?, display=?, name=?, url=?, furl=?, sort=?, \
                 color=?, model=?, bold=?, `type`=?, pic=?, config=? WHERE id=?",
            )
            .bind(s.nid)
            .bind(s.eject)
            .bind(s.display)
            .bind(s.name)
            .bind(s.url)
            .bind(s.furl)
            .bind(s.sort)
            .bind(s.color)
            .bind(s.model)
            .bind(s.bold)
            .bind(s.r#type)
            .bind(pic)
            .bind(s.config)
            .bind(id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "UPDATE phpyun_navigation SET nid=?, eject=?, display=?, name=?, url=?, furl=?, sort=?, \
                 color=?, model=?, bold=?, `type`=?, config=? WHERE id=?",
            )
            .bind(s.nid)
            .bind(s.eject)
            .bind(s.display)
            .bind(s.name)
            .bind(s.url)
            .bind(s.furl)
            .bind(s.sort)
            .bind(s.color)
            .bind(s.model)
            .bind(s.bold)
            .bind(s.r#type)
            .bind(s.config)
            .bind(id)
            .execute(pool)
            .await?;
        }
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_navigation (nid, eject, display, name, url, furl, sort, color, model, bold, `type`, pic, config) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(s.nid)
    .bind(s.eject)
    .bind(s.display)
    .bind(s.name)
    .bind(s.url)
    .bind(s.furl)
    .bind(s.sort)
    .bind(s.color)
    .bind(s.model)
    .bind(s.bold)
    .bind(s.r#type)
    .bind(s.pic.unwrap_or(""))
    .bind(s.config)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn php_set_nav_field(
    pool: &MySqlPool,
    id: u64,
    field: &str,
    rec: i32,
) -> Result<u64, sqlx::Error> {
    let col = match field {
        "display" => "display",
        "eject" => "eject",
        "sort" => "sort",
        "bold" => "bold",
        _ => return Ok(0),
    };
    let sql = format!("UPDATE phpyun_navigation SET `{col}` = ? WHERE id = ?");
    Ok(sqlx::query(&sql)
        .bind(rec)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_set_nav_display_by_config(
    pool: &MySqlPool,
    config: &str,
    display: i32,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_navigation SET display = ? WHERE config = ?")
            .bind(display)
            .bind(config)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn php_navs_with_links(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<PhpNavRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new("SELECT ");
    qb.push(NAV_FIELDS);
    qb.push(" FROM phpyun_navigation WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") AND (`desc` <> '' OR news <> '')");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_navs_by_nid_with_links(
    pool: &MySqlPool,
    nid: i32,
) -> Result<Vec<PhpNavRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {NAV_FIELDS} FROM phpyun_navigation WHERE nid = ? AND (`desc` <> '' OR news <> '')"
    );
    sqlx::query_as::<_, PhpNavRow>(&sql)
        .bind(nid)
        .fetch_all(pool)
        .await
}

pub async fn php_delete_navs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_navigation WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_delete_navs_by_nid(pool: &MySqlPool, nid: i32) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_navigation WHERE nid = ?")
        .bind(nid)
        .execute(pool)
        .await?
        .rows_affected())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpNavTypeRow {
    pub id: u64,
    pub typename: String,
}

pub async fn php_list_nav_types(pool: &MySqlPool) -> Result<Vec<PhpNavTypeRow>, sqlx::Error> {
    sqlx::query_as::<_, PhpNavTypeRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(typename,'') AS typename \
         FROM phpyun_navigation_type ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn php_nav_type_taken(pool: &MySqlPool, name: &str) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_navigation_type WHERE typename = ?",
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

pub async fn php_add_nav_type(pool: &MySqlPool, name: &str) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("INSERT INTO phpyun_navigation_type (typename) VALUES (?)")
            .bind(name)
            .execute(pool)
            .await?
            .last_insert_id(),
    )
}

pub async fn php_up_nav_type(pool: &MySqlPool, id: u64, name: &str) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_navigation_type SET typename = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn php_delete_nav_type(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_navigation_type WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_clear_desc_menu(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_description SET is_menu = 0 WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
