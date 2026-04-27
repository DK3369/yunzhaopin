//! `phpyun_domain` repository.

use super::entity::DomainSite;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(domain, '') AS domain, \
    CAST(province AS SIGNED) AS province, \
    CAST(cityid AS SIGNED) AS city_id, \
    CAST(three_cityid AS SIGNED) AS three_city_id, \
    CAST(COALESCE(fz_type, 0) AS SIGNED) AS fz_type, \
    CAST(hy AS SIGNED) AS hy, \
    style, tpl, \
    webtitle AS web_title, \
    webkeyword AS web_keyword, \
    webmeta AS web_meta, \
    weblogo AS web_logo, \
    CAST(COALESCE(mode, 0) AS SIGNED) AS mode, \
    indexdir";

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<DomainSite>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_domain ORDER BY id ASC");
    sqlx::query_as::<_, DomainSite>(&sql)
        .fetch_all(pool)
        .await
}

pub async fn list_by_fz_type(
    pool: &MySqlPool,
    fz_type: i32,
) -> Result<Vec<DomainSite>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_domain WHERE fz_type = ? ORDER BY id ASC"
    );
    sqlx::query_as::<_, DomainSite>(&sql)
        .bind(fz_type)
        .fetch_all(pool)
        .await
}

/// Find the sub-site that matches a (provinceid, cityid, three_cityid) triplet.
/// Tries the most specific match first (district → city → province), mirroring
/// PHP `getCityDomain` precedence.
pub async fn find_for_city(
    pool: &MySqlPool,
    province_id: i32,
    city_id: i32,
    three_city_id: i32,
) -> Result<Option<DomainSite>, sqlx::Error> {
    // 1) three-level match
    if three_city_id > 0 {
        let sql = format!(
            "SELECT {FIELDS} FROM phpyun_domain \
             WHERE fz_type = 1 AND three_cityid = ? LIMIT 1"
        );
        if let Some(r) = sqlx::query_as::<_, DomainSite>(&sql)
            .bind(three_city_id)
            .fetch_optional(pool)
            .await?
        {
            return Ok(Some(r));
        }
    }
    // 2) city
    if city_id > 0 {
        let sql = format!(
            "SELECT {FIELDS} FROM phpyun_domain \
             WHERE fz_type = 1 AND cityid = ? \
               AND (three_cityid IS NULL OR three_cityid = 0) \
             LIMIT 1"
        );
        if let Some(r) = sqlx::query_as::<_, DomainSite>(&sql)
            .bind(city_id)
            .fetch_optional(pool)
            .await?
        {
            return Ok(Some(r));
        }
    }
    // 3) province
    if province_id > 0 {
        let sql = format!(
            "SELECT {FIELDS} FROM phpyun_domain \
             WHERE fz_type = 1 AND province = ? \
               AND (cityid IS NULL OR cityid = 0) \
             LIMIT 1"
        );
        if let Some(r) = sqlx::query_as::<_, DomainSite>(&sql)
            .bind(province_id)
            .fetch_optional(pool)
            .await?
        {
            return Ok(Some(r));
        }
    }
    Ok(None)
}
