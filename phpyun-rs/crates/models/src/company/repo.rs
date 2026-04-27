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

/// Increment both `hits` and `expoure` by 1. Used by the public detail page —
/// PHP's `$companyM->upInfo($cuid, '', ['hits' => ['+', 1], 'expoure' => ['+', 1]])`.
pub async fn incr_hits_and_expoure(pool: &MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE phpyun_company SET hits = hits + 1, expoure = expoure + 1 WHERE uid = ?",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Count "open" jobs for a company — i.e. jobs that are visible to the public
/// listing. Equivalent to PHP's
/// `jobM->getJobNum(['uid'=>cuid,'state'=>1,'status'=>0,'r_status'=>1])`.
/// `edate=0` means "no expiry set" (treated as active).
pub async fn count_open_jobs(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job \
         WHERE uid = ? AND state = 1 AND status = 0 AND r_status = 1 \
           AND (edate = 0 OR edate > UNIX_TIMESTAMP())",
    )
    .bind(uid as i64)
    .fetch_one(pool)
    .await?;
    Ok(row.0.max(0) as u64)
}

/// One row from the `phpyun_company_show` showcase table — used on the
/// company detail page to render the "公司风采" carousel.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompanyShowRow {
    pub id: u64,
    pub title: Option<String>,
    pub picurl: Option<String>,
    pub body: Option<String>,
    pub sort: i32,
    pub ctime: i64,
}

/// Fetch active showcase items for a company, ordered by `sort` then `id`.
pub async fn list_show_items(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<CompanyShowRow>, sqlx::Error> {
    sqlx::query_as::<_, CompanyShowRow>(
        "SELECT id, title, picurl, body, \
                CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
                CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime \
         FROM phpyun_company_show \
         WHERE uid = ? AND status = 0 \
         ORDER BY sort ASC, id ASC",
    )
    .bind(uid as i64)
    .fetch_all(pool)
    .await
}

// ==================== Hot / featured companies (homepage banner) ====================

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct HotCompany {
    pub uid: u64,
    pub name: String,
    pub shortname: Option<String>,
    pub logo: Option<String>,
    pub hot_pic: Option<String>,
    /// 0 = ordered by paid `sort`, 1 = ordered by job `lastupdate`, 2 = random.
    /// Echoed back so clients can short-cache appropriately.
    pub sort_mode: i32,
}

// ==================== Autocomplete ====================

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct CompanyBrief {
    pub uid: u64,
    pub name: String,
    pub logo: Option<String>,
}

/// Company card row used by listing pages that need to render a uniform
/// "company chip" (logo + dict-localised industry / scale / location) next to
/// each item — e.g. zph participants, special-topic participants. The name is
/// kept as `Option<String>` to mirror the legacy schema where some rows have
/// no `name` set.
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct CompanyCard {
    pub uid: u64,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub provinceid: i32,
    pub cityid: i32,
}

pub async fn list_cards_by_uids(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<CompanyCard>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = std::iter::repeat("?")
        .take(uids.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT \
            CAST(uid AS UNSIGNED) AS uid, \
            name, logo, \
            CAST(COALESCE(hy,0) AS SIGNED) AS hy, \
            CAST(COALESCE(pr,0) AS SIGNED) AS pr, \
            CAST(COALESCE(mun,0) AS SIGNED) AS mun, \
            CAST(COALESCE(provinceid,0) AS SIGNED) AS provinceid, \
            CAST(COALESCE(cityid,0) AS SIGNED) AS cityid \
         FROM phpyun_company WHERE uid IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, CompanyCard>(&sql);
    for u in uids {
        q = q.bind(*u as i64);
    }
    q.fetch_all(pool).await
}

/// Quick autocomplete for company name search — counterpart of PHP
/// `ajax::getComBySearch_action`. Returns up to `limit` rows whose `name`
/// matches `LIKE %keyword%` and have been approved (`r_status = 1`).
/// Designed for typeahead widgets, NOT general search — fewer columns and
/// no expensive joins.
pub async fn search_brief(
    pool: &MySqlPool,
    keyword: &str,
    limit: u64,
) -> Result<Vec<CompanyBrief>, sqlx::Error> {
    let pattern = format!("%{}%", keyword);
    sqlx::query_as::<_, CompanyBrief>(
        "SELECT \
            CAST(uid AS UNSIGNED) AS uid, \
            COALESCE(name, '') AS name, \
            logo \
         FROM phpyun_company \
         WHERE r_status = 1 AND name LIKE ? \
         ORDER BY hits DESC, uid DESC \
         LIMIT ?",
    )
    .bind(pattern)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Featured companies on the homepage — counterpart of PHP
/// `wap/index::getmq_action`. Joins `phpyun_hotjob` (paid promo banners)
/// with `phpyun_company`, filtering by:
///   - `phpyun_company.hottime > now` (company-level paid window)
///   - `phpyun_company.r_status = 1`  (approved)
///   - `phpyun_hotjob.time_start < now AND time_end > now` (active banner)
///
/// `sort_mode`: 0 = `sort` ASC (default), 1 = `lastupdate` DESC, 2 = `RAND()`.
pub async fn list_hot(
    pool: &MySqlPool,
    sort_mode: i32,
    limit: u64,
    now: i64,
) -> Result<Vec<HotCompany>, sqlx::Error> {
    // ORDER BY clause is whitelisted (not user-supplied) — building it from
    // a static match is safe and avoids an extra lookup.
    let order_clause = match sort_mode {
        1 => "h.lastupdate DESC, h.id DESC",
        2 => "RAND()",
        _ => "h.sort ASC, h.id DESC",
    };
    let sql = format!(
        "SELECT \
            CAST(c.uid AS UNSIGNED) AS uid, \
            COALESCE(c.name, '') AS name, \
            c.shortname, \
            c.logo, \
            h.hot_pic, \
            CAST(? AS SIGNED) AS sort_mode \
         FROM phpyun_hotjob h \
         JOIN phpyun_company c ON c.uid = h.uid \
         WHERE c.hottime > ? \
           AND c.r_status = 1 \
           AND h.time_start < ? \
           AND h.time_end > ? \
         ORDER BY {order_clause} \
         LIMIT ?"
    );
    sqlx::query_as::<_, HotCompany>(&sql)
        .bind(sort_mode)
        .bind(now)
        .bind(now)
        .bind(now)
        .bind(limit)
        .fetch_all(pool)
        .await
}
