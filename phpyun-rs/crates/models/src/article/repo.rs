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
    let sql = format!(
        "SELECT {FIELDS}, c.content AS content {FROM_DETAIL} WHERE n.id = ? LIMIT 1"
    );
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
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &ArticleFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Article>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {FIELDS}, NULL AS content {FROM_LIST} WHERE 1=1"));
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
    Ok(n.max(0) as u64)
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
}

pub async fn incr_hits(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_news_base SET hits = hits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Read the current hit count without incrementing. Used by `GetHits_action`
/// equivalents that need to render "今日浏览 X 次" widgets.
pub async fn get_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(hits, 0) AS SIGNED) FROM phpyun_news_base WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n.max(0) as u64).unwrap_or(0))
}

/// Atomically increment + return the new hit count.
pub async fn bump_and_get_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    incr_hits(pool, id).await?;
    get_hits(pool, id).await
}
