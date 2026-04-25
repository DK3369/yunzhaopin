use super::entity::Company;
use sqlx::{MySqlPool, QueryBuilder};

// Relies on the entity's #[sqlx(try_from = "i32")] for u64 <- INT conversion.
// Covers the subset of PHPYun's 60-column source table used by the WAP detail page.
const FIELDS: &str = "\
    uid, name, shortname, \
    COALESCE(hy, 0) AS hy, \
    COALESCE(pr, 0) AS pr, \
    COALESCE(mun, 0) AS mun, \
    sdate, \
    COALESCE(money, 0) AS money, \
    COALESCE(moneytype, 0) AS moneytype, \
    COALESCE(provinceid, 0) AS provinceid, \
    COALESCE(cityid, 0) AS cityid, \
    COALESCE(three_cityid, 0) AS three_cityid, \
    address, zip, x, y, \
    linkman, linkjob, linkqq, linkphone, linktel, linkmail, website, \
    logo, COALESCE(logo_status, 0) AS logo_status, firmpic, comqcode, \
    content, \
    COALESCE(r_status, 0) AS r_status, \
    COALESCE(rec, 0) AS rec, \
    COALESCE(hits, 0) AS hits, \
    COALESCE(expoure, 0) AS expoure, \
    COALESCE(moblie_status, 0) AS moblie_status, \
    COALESCE(email_status, 0) AS email_status, \
    COALESCE(yyzz_status, 0) AS yyzz_status, \
    COALESCE(rating, 0) AS rating, \
    rating_name, \
    COALESCE(vipstime, 0) AS vipstime, \
    COALESCE(vipetime, 0) AS vipetime, \
    COALESCE(payd, 0) AS payd, \
    COALESCE(integral, 0) AS integral, \
    lastupdate, \
    COALESCE(addtime, 0) AS addtime, \
    COALESCE(login_date, 0) AS login_date, \
    COALESCE(fact_status, 0) AS fact_status, \
    COALESCE(did, 0) AS did";

// ==================== Public search ====================

#[derive(Debug, Default, Clone)]
pub struct CompanyFilter<'a> {
    pub keyword: Option<&'a str>, // matched against name via LIKE
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub hy: Option<i32>, // industry
    pub did: u32,
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &CompanyFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Company>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company WHERE r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(" ORDER BY rec DESC, hits DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Company>().fetch_all(pool).await
}

pub async fn count_public(
    pool: &MySqlPool,
    f: &CompanyFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company WHERE r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &CompanyFilter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    if let Some(v) = f.province_id {
        qb.push(" AND provinceid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.city_id {
        qb.push(" AND cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.hy {
        qb.push(" AND hy = ");
        qb.push_bind(v);
    }
}

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<Company>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company WHERE uid = ? LIMIT 1");
    sqlx::query_as::<_, Company>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn ensure_row<'e, E>(exec: E, uid: u64, did: u32) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query(
        r#"INSERT IGNORE INTO phpyun_company
           (uid, did, r_status, rec, hits, hy, provinceid, cityid, three_cityid, logo_status)
           VALUES (?, ?, 0, 0, 0, 0, 0, 0, 0, 0)"#,
    )
    .bind(uid)
    .bind(did)
    .execute(exec)
    .await?;
    Ok(())
}

pub struct CompanyUpdate<'a> {
    pub name: Option<&'a str>,
    pub shortname: Option<&'a str>,
    pub hy: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    pub logo: Option<&'a str>,
    pub content: Option<&'a str>,
    pub linkman: Option<&'a str>,
    pub linkjob: Option<&'a str>,
    pub linkphone: Option<&'a str>,
    pub linkmail: Option<&'a str>,
}

pub async fn update(pool: &MySqlPool, uid: u64, u: CompanyUpdate<'_>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_company SET
            name         = COALESCE(?, name),
            shortname    = COALESCE(?, shortname),
            hy           = COALESCE(?, hy),
            provinceid   = COALESCE(?, provinceid),
            cityid       = COALESCE(?, cityid),
            three_cityid = COALESCE(?, three_cityid),
            logo         = COALESCE(?, logo),
            content      = COALESCE(?, content),
            linkman      = COALESCE(?, linkman),
            linkjob      = COALESCE(?, linkjob),
            linkphone    = COALESCE(?, linkphone),
            linkmail     = COALESCE(?, linkmail)
           WHERE uid = ?"#,
    )
    .bind(u.name)
    .bind(u.shortname)
    .bind(u.hy)
    .bind(u.provinceid)
    .bind(u.cityid)
    .bind(u.three_cityid)
    .bind(u.logo)
    .bind(u.content)
    .bind(u.linkman)
    .bind(u.linkjob)
    .bind(u.linkphone)
    .bind(u.linkmail)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Increment hit count by 1 (one detail-page view). Uses the writer pool
/// (UPDATE). Not wrapped in a transaction; failures do not block.
pub async fn incr_hits(pool: &MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_company SET hits = hits + 1 WHERE uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
