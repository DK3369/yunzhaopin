//! Strictly aligned with PHPYun's `article.model.php`: main table
//! `phpyun_news_base`, body stored in side table `phpyun_news_content`
//! (linked by `nbid`), category dictionary `phpyun_news_group` (linked by
//! `nid`).
//!
//! Rust field mapping to PHP columns:
//!   - `nid`/`category` <-> phpyun_news_base.nid + JOIN news_group.name
//!   - `summary`        <-> description
//!   - `cover`          <-> newsphoto
//!   - `published_at`   <-> datetime
//!   - `rec`            <-> describe CSV contains "1" (PHP convention)
//!   - `status`         always 1 (PHPYun news_base has no status column)

use super::entity::Article;
use crate::soft_delete;
use sqlx::{MySqlPool, QueryBuilder};

/// Full-field SELECT -- 18 columns of phpyun_news_base + JOIN
/// news_group.name + LEFT JOIN news_content.content.
///
/// `rec` is derived from the describe CSV (PHP uses
/// `in_array("1", explode(",", $describe))`); on the SQL side we use
/// `FIND_IN_SET("1", describe) > 0`; supported on MySQL 5.7+.
const FIELDS: &str = "\
    CAST(n.id AS UNSIGNED) AS id, \
    CAST(COALESCE(n.nid, 0) AS SIGNED) AS nid, \
    COALESCE(g.name, '') AS category, \
    CAST(COALESCE(n.did, 0) AS UNSIGNED) AS did, \
    n.title, \
    n.color, \
    COALESCE(n.keyword, '') AS keyword, \
    COALESCE(n.author, '') AS author, \
    CAST(COALESCE(n.datetime, 0) AS SIGNED) AS published_at, \
    CAST(COALESCE(n.hits, 0) AS SIGNED) AS hits, \
    COALESCE(n.`describe`, '') AS `describe`, \
    COALESCE(n.description, '') AS summary, \
    COALESCE(n.newsphoto, '') AS cover, \
    n.s_thumb, \
    n.source, \
    CAST(COALESCE(n.sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(n.lastupdate, 0) AS SIGNED) AS lastupdate, \
    CAST(COALESCE(n.starttime, 0) AS SIGNED) AS starttime, \
    CAST(COALESCE(n.endtime, 0) AS SIGNED) AS endtime, \
    CAST(IF(FIND_IN_SET('1', COALESCE(n.`describe`, '')) > 0, 1, 0) AS SIGNED) AS rec, \
    CAST(1 AS SIGNED) AS status";

const FROM_LIST: &str = "\
    FROM phpyun_news_base n \
    LEFT JOIN phpyun_news_group g ON g.id = n.nid";

const FROM_DETAIL: &str = "\
    FROM phpyun_news_base n \
    LEFT JOIN phpyun_news_group g   ON g.id  = n.nid \
    LEFT JOIN phpyun_news_content c ON c.nbid = n.id";

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Article>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS}, c.content AS content {FROM_DETAIL} WHERE n.id = ? AND COALESCE(n.deleted,0)=0 LIMIT 1");
    sqlx::query_as::<_, Article>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

#[derive(Debug, Default, Clone)]
pub struct ArticleFilter<'a> {
    /// PHPYun's `nid` (news_group.id). Pass a numeric string.
    pub category: Option<&'a str>,
    pub keyword: Option<&'a str>,
    pub rec_only: bool,
    pub did: u32,
    pub datetime_min: Option<i64>,
    pub author_kw: Option<&'a str>,
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &ArticleFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Article>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT {FIELDS}, NULL AS content {FROM_LIST} WHERE 1=1"
    ));
    push_did_scope(&mut qb, f.did);
    push_filters(&mut qb, f);
    qb.push(" ORDER BY n.sort DESC, n.datetime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Article>().fetch_all(pool).await
}

pub async fn count_public(pool: &MySqlPool, f: &ArticleFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_news_base n WHERE 1=1");
    push_did_scope(&mut qb, f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP convention (`app/controller/wap/article.class.php:29-32`):
/// - main site (did=0): no scope filter — articles from any did show up.
/// - subsite (did>0): match either this subsite's did OR did=-1
///   (which PHP treats as "publish to all sites").
fn push_did_scope<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, did: u32) {
    if did > 0 {
        qb.push(" AND (n.did = ");
        qb.push_bind(did);
        qb.push(" OR n.did = -1)");
    }
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &ArticleFilter<'a>) {
    qb.push(" AND COALESCE(n.deleted,0)=0");
    if let Some(c) = f.category {
        if !c.is_empty() {
            qb.push(" AND n.nid = ");
            qb.push_bind(c.parse::<u32>().unwrap_or(0));
        }
    }
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND n.title LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    if f.rec_only {
        qb.push(" AND FIND_IN_SET('1', COALESCE(n.`describe`, '')) > 0");
    }
    if let Some(ts) = f.datetime_min {
        qb.push(" AND n.datetime >= ");
        qb.push_bind(ts);
    }
    if let Some(kw) = f.author_kw {
        if !kw.is_empty() {
            qb.push(" AND n.author LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
}

pub async fn incr_hits(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_news_base SET hits = hits + 1 WHERE id = ? AND COALESCE(deleted,0)=0")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Read the current hit count without incrementing. Used by `GetHits_action`
/// equivalents that need to render "今日浏览 X 次" widgets.
pub async fn get_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(hits, 0) AS SIGNED) FROM phpyun_news_base WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row
        .map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0))
}

/// Atomically increment + return the new hit count.
pub async fn bump_and_get_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    incr_hits(pool, id).await?;
    get_hits(pool, id).await
}

pub struct ArticleIngest<'a> {
    pub title: &'a str,
    pub nid: i32,
    pub did: i32,
    pub author: &'a str,
    pub description: &'a str,
    pub source: &'a str,
    pub datetime: i64,
    pub hits: i32,
    pub sort: i32,
    pub newsphoto: &'a str,
    pub s_thumb: &'a str,
    pub keyword: &'a str,
    pub content: &'a str,
}

/// Locoy news ingest. Returns `Ok(None)` when the same title+nid already exists.
pub async fn ingest(pool: &MySqlPool, a: ArticleIngest<'_>) -> Result<Option<u64>, sqlx::Error> {
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM phpyun_news_base WHERE title = ? AND nid = ? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(a.title)
    .bind(a.nid)
    .fetch_optional(pool)
    .await?;
    if exists.is_some() {
        return Ok(None);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_news_base
           (title, nid, did, author, description, source, datetime, starttime,
            hits, sort, newsphoto, s_thumb, keyword, lastupdate)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(a.title)
    .bind(a.nid)
    .bind(a.did)
    .bind(a.author)
    .bind(a.description)
    .bind(a.source)
    .bind(a.datetime)
    .bind(a.datetime)
    .bind(a.hits)
    .bind(a.sort)
    .bind(a.newsphoto)
    .bind(a.s_thumb)
    .bind(a.keyword)
    .bind(a.datetime)
    .execute(pool)
    .await?;
    let id = res.last_insert_id();
    sqlx::query("INSERT INTO phpyun_news_content (nbid, content) VALUES (?, ?)")
        .bind(id)
        .bind(a.content)
        .execute(pool)
        .await?;
    Ok(Some(id))
}

pub async fn list_groups(pool: &MySqlPool) -> Result<Vec<super::entity::NewsGroup>, sqlx::Error> {
    sqlx::query_as::<_, super::entity::NewsGroup>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name, '') AS name, \
                CAST(COALESCE(keyid, 0) AS SIGNED) AS keyid \
         FROM phpyun_news_group ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn list_admin(
    pool: &MySqlPool,
    f: &ArticleFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Article>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT {FIELDS}, NULL AS content {FROM_LIST} WHERE 1=1"
    ));
    push_filters(&mut qb, f);
    qb.push(" ORDER BY n.sort DESC, n.datetime DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Article>().fetch_all(pool).await
}

pub async fn count_admin(pool: &MySqlPool, f: &ArticleFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_news_base n WHERE 1=1");
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct ArticleUpsert<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub nid: i32,
    pub content: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub keyword: &'a str,
    pub source: &'a str,
    pub newsphoto: &'a str,
    pub did: i32,
    pub now: i64,
}

pub async fn upsert(pool: &MySqlPool, a: ArticleUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_news_base
               SET title = ?, nid = ?, author = ?, description = ?, keyword = ?,
                   source = ?, newsphoto = ?, lastupdate = ?
               WHERE id = ?"#,
        )
        .bind(a.title)
        .bind(a.nid)
        .bind(a.author)
        .bind(a.description)
        .bind(a.keyword)
        .bind(a.source)
        .bind(a.newsphoto)
        .bind(a.now)
        .bind(id)
        .execute(pool)
        .await?;
        let updated = sqlx::query("UPDATE phpyun_news_content SET content = ? WHERE nbid = ?")
            .bind(a.content)
            .bind(id)
            .execute(pool)
            .await?;
        if updated.rows_affected() == 0 {
            sqlx::query("INSERT INTO phpyun_news_content (nbid, content) VALUES (?, ?)")
                .bind(id)
                .bind(a.content)
                .execute(pool)
                .await?;
        }
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_news_base
           (title, nid, did, author, description, source, datetime, starttime,
            hits, sort, newsphoto, s_thumb, keyword, lastupdate)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, '', ?, ?)"#,
    )
    .bind(a.title)
    .bind(a.nid)
    .bind(a.did)
    .bind(a.author)
    .bind(a.description)
    .bind(a.source)
    .bind(a.now)
    .bind(a.now)
    .bind(a.newsphoto)
    .bind(a.keyword)
    .bind(a.now)
    .execute(pool)
    .await?;
    let id = res.last_insert_id();
    sqlx::query("INSERT INTO phpyun_news_content (nbid, content) VALUES (?, ?)")
        .bind(id)
        .bind(a.content)
        .execute(pool)
        .await?;
    Ok(id)
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_news_base", id).await
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_news_base", ids).await
}

pub async fn set_did_ids(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_news_base SET did = ");
    qb.push_bind(did);
    qb.push(" WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_nid_ids(pool: &MySqlPool, ids: &[u64], nid: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_news_base SET nid = ");
    qb.push_bind(nid);
    qb.push(" WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_describe(pool: &MySqlPool, id: u64, describe: &str) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_news_base SET `describe` = ? WHERE id = ?")
            .bind(describe)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn list_describe(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<(u64, String)>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let mut qb = QueryBuilder::new("SELECT CAST(id AS UNSIGNED), COALESCE(`describe`,'') FROM phpyun_news_base WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_content(pool: &MySqlPool, id: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(content,'') FROM phpyun_news_content WHERE nbid = ? LIMIT 1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|(s,)| s))
}

pub async fn list_groups_admin(
    pool: &MySqlPool,
) -> Result<Vec<super::entity::NewsGroupAdmin>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, \
         CAST(COALESCE(keyid,0) AS SIGNED) AS keyid, CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
         CAST(COALESCE(rec,0) AS SIGNED) AS rec, CAST(COALESCE(rec_news,0) AS SIGNED) AS rec_news, \
         CAST(COALESCE(is_menu,0) AS SIGNED) AS is_menu \
         FROM phpyun_news_group ORDER BY sort ASC, id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn news_count_by_nid(pool: &MySqlPool) -> Result<Vec<(i32, i64)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(COALESCE(nid,0) AS SIGNED), CAST(COUNT(*) AS SIGNED) \
         FROM phpyun_news_base WHERE COALESCE(deleted,0)=0 GROUP BY nid",
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_group(
    pool: &MySqlPool,
    name: &str,
    keyid: i32,
    rec: i32,
) -> Result<u64, sqlx::Error> {
    let res = if keyid == 0 {
        sqlx::query("INSERT INTO phpyun_news_group (name, keyid, rec) VALUES (?, ?, ?)")
            .bind(name)
            .bind(keyid)
            .bind(rec)
            .execute(pool)
            .await?
    } else {
        sqlx::query("INSERT INTO phpyun_news_group (name, keyid) VALUES (?, ?)")
            .bind(name)
            .bind(keyid)
            .execute(pool)
            .await?
    };
    Ok(res.last_insert_id())
}

pub async fn set_group_is_menu(pool: &MySqlPool, id: u64, is_menu: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_news_group SET is_menu = ? WHERE id = ?")
            .bind(is_menu)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_group_keyid(pool: &MySqlPool, ids: &[u64], keyid: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_news_group SET keyid = ");
    qb.push_bind(keyid);
    qb.push(" WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn patch_group(
    pool: &MySqlPool,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
    rec: Option<i32>,
    rec_news: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if let Some(v) = name {
        return Ok(
            sqlx::query("UPDATE phpyun_news_group SET name = ? WHERE id = ?")
                .bind(v)
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected(),
        );
    }
    if let Some(v) = sort {
        return Ok(
            sqlx::query("UPDATE phpyun_news_group SET sort = ? WHERE id = ?")
                .bind(v)
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected(),
        );
    }
    if let Some(v) = rec {
        return Ok(
            sqlx::query("UPDATE phpyun_news_group SET rec = ? WHERE id = ? AND keyid = 0")
                .bind(v)
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected(),
        );
    }
    if let Some(v) = rec_news {
        return Ok(
            sqlx::query("UPDATE phpyun_news_group SET rec_news = ? WHERE id = ? AND keyid = 0")
                .bind(v)
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected(),
        );
    }
    Ok(0)
}

pub async fn delete_groups(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_news_group WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn list_properties(
    pool: &MySqlPool,
    keyword: Option<&str>,
    kw_type: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<super::entity::NewsProperty>, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(value,'') AS value FROM phpyun_property WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if kw_type == 2 {
            qb.push(" AND value LIKE ");
        } else {
            qb.push(" AND name LIKE ");
        }
        qb.push_bind(like);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_properties(pool: &MySqlPool, keyword: Option<&str>, kw_type: i32) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_property WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if kw_type == 2 {
            qb.push(" AND value LIKE ");
        } else {
            qb.push(" AND name LIKE ");
        }
        qb.push_bind(like);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_property(pool: &MySqlPool, id: Option<u64>, name: &str, value: &str) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|i| *i > 0) {
        sqlx::query("UPDATE phpyun_property SET name = ?, value = ? WHERE id = ?")
            .bind(name)
            .bind(value)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(id)
    } else {
        let res = sqlx::query("INSERT INTO phpyun_property (name, value) VALUES (?, ?)")
            .bind(name)
            .bind(value)
            .execute(pool)
            .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_properties(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_property WHERE id IN (");
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
