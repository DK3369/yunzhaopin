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
    welfare, \
    COALESCE(did, 0) AS did";

// ==================== Public search ====================

/// Public-company list filter. Field set tracks PHPYun's `comlist` Smarty
/// plugin (`smarty_internal_compile_comlist.php`). Welfare / linkman /
/// linktel / linkmail / logo / uptime are deliberately omitted — they
/// either require a secondary SELECT (`welfare` does FIND_IN_SET into a
/// uid set first) or are rarely used in practice; add them on demand.
#[derive(Debug, Default, Clone)]
pub struct CompanyFilter<'a> {
    /// Matched against `name` AND `shortname` via LIKE (PHP `comlist`).
    pub keyword: Option<&'a str>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    /// Industry dict id (`hy`).
    pub hy: Option<i32>,
    /// Company-type dict id — 国企/外资/民营/… (`pr`).
    pub pr: Option<i32>,
    /// Staff-count dict id — 50人以下/50-200/… (`mun`).
    pub mun: Option<i32>,
    /// Welfare dict id (`FIND_IN_SET` against CSV `welfare`).
    pub welfare: Option<i32>,
    /// `cert=true` keeps only companies whose business license has been
    /// verified (`yyzz_status = 1`).
    pub cert: bool,
    /// `rec=true` keeps only sticky/promoted companies. PHP composite:
    /// `rec=1 AND hotstart <= now AND hottime > now`.
    pub rec: bool,
    pub did: u32,
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &CompanyFilter<'_>,
    offset: u64,
    limit: u64,
    now: i64,
) -> Result<Vec<Company>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company WHERE r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    qb.push(" ORDER BY rec DESC, hits DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Company>().fetch_all(pool).await
}

pub async fn count_public(
    pool: &MySqlPool,
    f: &CompanyFilter<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company WHERE r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &CompanyFilter<'a>, now: i64) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            // PHP `comlist`: `(name LIKE OR shortname LIKE)`.
            qb.push(" AND (name LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR shortname LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
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
    if let Some(v) = f.three_city_id {
        qb.push(" AND three_cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.hy {
        qb.push(" AND hy = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.pr {
        qb.push(" AND pr = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.mun {
        qb.push(" AND mun = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.welfare {
        qb.push(" AND FIND_IN_SET(");
        qb.push_bind(v);
        qb.push(", welfare)");
    }
    if f.cert {
        qb.push(" AND yyzz_status = 1");
    }
    if f.rec {
        // PHP composite: hotstart <= now < hottime, AND rec=1.
        qb.push(" AND rec = 1 AND hotstart <= ");
        qb.push_bind(now);
        qb.push(" AND hottime > ");
        qb.push_bind(now);
    }
}

/// Cheap existence check — `SELECT 1`. Used by transfer/merge preconditions
/// where the full entity is unnecessary.
pub async fn exists_by_uid(pool: &MySqlPool, uid: u64) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM phpyun_company WHERE uid = ? LIMIT 1")
        .bind(uid)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<Company>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company WHERE uid = ? LIMIT 1");
    sqlx::query_as::<_, Company>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Load full company rows for the given uids, preserving input order.
pub async fn list_by_uids(pool: &MySqlPool, uids: &[u64]) -> Result<Vec<Company>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = std::iter::repeat_n("?", uids.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!("SELECT {FIELDS} FROM phpyun_company WHERE uid IN ({placeholders})");
    let signed_uids = uids
        .iter()
        .copied()
        .map(|uid| phpyun_core::numeric::checked_db_i64(uid, "company.uid"))
        .collect::<Result<Vec<i64>, _>>()?;
    let mut q = sqlx::query_as::<_, Company>(&sql);
    for uid in signed_uids {
        q = q.bind(uid);
    }
    let rows = q.fetch_all(pool).await?;
    let mut map: std::collections::HashMap<u64, Company> =
        rows.into_iter().map(|c| (c.uid, c)).collect();
    Ok(uids.iter().filter_map(|uid| map.remove(uid)).collect())
}

pub async fn touch_jobtime(pool: &MySqlPool, uid: u64, now: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_company SET jobtime = ?, lastupdate = ? WHERE uid = ?")
        .bind(now)
        .bind(now.to_string())
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_uid_by_name(pool: &MySqlPool, name: &str) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_company WHERE name = ? LIMIT 1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
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

pub struct AdminCompanyInsert<'a> {
    pub uid: u64,
    pub name: &'a str,
    pub shortname: &'a str,
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: &'a str,
    pub x: &'a str,
    pub y: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub linkphone: &'a str,
    pub linkmail: &'a str,
    pub content: &'a str,
    pub lastupdate: &'a str,
    pub rating: i32,
    pub rating_name: &'a str,
    pub vipstime: i64,
    pub vipetime: i64,
}

/// PHP `userinfo::addInfo` company insert (`r_status=1`).
pub async fn insert_admin_created<'e, E>(
    exec: E,
    row: AdminCompanyInsert<'_>,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query(
        "INSERT INTO phpyun_company (\
            uid, name, shortname, hy, pr, mun, provinceid, cityid, three_cityid, \
            address, x, y, linkman, linktel, linkphone, linkmail, content, lastupdate, \
            r_status, rating, rating_name, vipstime, vipetime, busstops, welfare\
         ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,1,?,?,?,?, '', '')",
    )
    .bind(row.uid)
    .bind(row.name)
    .bind(row.shortname)
    .bind(row.hy)
    .bind(row.pr)
    .bind(row.mun)
    .bind(row.provinceid)
    .bind(row.cityid)
    .bind(row.three_cityid)
    .bind(row.address)
    .bind(row.x)
    .bind(row.y)
    .bind(row.linkman)
    .bind(row.linktel)
    .bind(row.linkphone)
    .bind(row.linkmail)
    .bind(row.content)
    .bind(row.lastupdate)
    .bind(row.rating)
    .bind(row.rating_name)
    .bind(row.vipstime)
    .bind(row.vipetime)
    .execute(exec)
    .await?;
    Ok(())
}

pub async fn delete_by_uid<'e, E>(exec: E, uid: u64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query("DELETE FROM phpyun_company WHERE uid = ?")
        .bind(uid)
        .execute(exec)
        .await?;
    Ok(())
}

/// Bare INSERT IGNORE — only sets `uid`. Counterpart of
/// [`crate::resume::repo::ensure_uid_only`] for the company side; called by
/// `seed_role_rows` when an employer's usertype is set after registration.
pub async fn ensure_uid_only(pool: &sqlx::MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT IGNORE INTO phpyun_company (uid) VALUES (?)")
        .bind(uid)
        .execute(pool)
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

pub struct AdminCompanyProfile<'a> {
    pub name: &'a str,
    pub shortname: &'a str,
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub linkphone: &'a str,
    pub linkmail: &'a str,
    pub address: &'a str,
    pub moneytype: i32,
    pub money: i32,
    pub linkqq: &'a str,
    pub website: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub content: &'a str,
    pub busstops: &'a str,
    pub welfare: &'a str,
    pub lastupdate: &'a str,
    pub x: &'a str,
    pub y: &'a str,
    pub r_status: Option<i32>,
    pub infostatus: Option<i32>,
    pub sdate: Option<&'a str>,
    pub linkjob: Option<&'a str>,
}

pub async fn update_admin_profile(
    pool: &MySqlPool,
    uid: u64,
    p: AdminCompanyProfile<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company SET \
            name=?, shortname=?, hy=?, pr=?, mun=?, linkman=?, linktel=?, linkphone=?, linkmail=?, \
            address=?, moneytype=?, money=?, linkqq=?, website=?, provinceid=?, cityid=?, three_cityid=?, \
            content=?, busstops=?, welfare=?, lastupdate=?, x=?, y=?, \
            r_status=COALESCE(?, r_status), infostatus=COALESCE(?, infostatus), \
            sdate=COALESCE(?, sdate), linkjob=COALESCE(?, linkjob) \
         WHERE uid=?",
    )
    .bind(p.name)
    .bind(p.shortname)
    .bind(p.hy)
    .bind(p.pr)
    .bind(p.mun)
    .bind(p.linkman)
    .bind(p.linktel)
    .bind(p.linkphone)
    .bind(p.linkmail)
    .bind(p.address)
    .bind(p.moneytype)
    .bind(p.money)
    .bind(p.linkqq)
    .bind(p.website)
    .bind(p.provinceid)
    .bind(p.cityid)
    .bind(p.three_cityid)
    .bind(p.content)
    .bind(p.busstops)
    .bind(p.welfare)
    .bind(p.lastupdate)
    .bind(p.x)
    .bind(p.y)
    .bind(p.r_status)
    .bind(p.infostatus)
    .bind(p.sdate)
    .bind(p.linkjob)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
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
    sqlx::query("UPDATE phpyun_company SET hits = hits + 1, expoure = expoure + 1 WHERE uid = ?")
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
    .bind(phpyun_core::numeric::checked_db_i64(uid, "company.uid")?)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

/// PHP `$invite_resume`: interview invitations this company has sent.
pub async fn count_interview_invites(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_userid_msg WHERE fid = ?")
        .bind(phpyun_core::numeric::checked_db_i64(uid, "company.uid")?)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(row.0))
}

/// Open-job counts for a batch of company uids (one round-trip).
pub async fn count_open_jobs_by_uids(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<(u64, u64)>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; uids.len()].join(",");
    let sql = format!(
        "SELECT CAST(uid AS SIGNED), CAST(COUNT(*) AS SIGNED) FROM phpyun_company_job \
         WHERE uid IN ({placeholders}) AND state = 1 AND status = 0 AND r_status = 1 \
           AND (edate = 0 OR edate > UNIX_TIMESTAMP()) \
         GROUP BY uid"
    );
    let mut q = sqlx::query_as::<_, (i64, i64)>(&sql);
    for uid in uids {
        q = q.bind(phpyun_core::numeric::checked_db_i64(*uid, "company.uid")?);
    }
    let rows = q.fetch_all(pool).await?;
    Ok(rows
        .into_iter()
        .map(|(uid, n)| {
            (
                phpyun_core::numeric::nonnegative_count(uid),
                phpyun_core::numeric::nonnegative_count(n),
            )
        })
        .collect())
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
         WHERE uid = ? AND status = 0 AND COALESCE(deleted,0)=0 \
         ORDER BY sort ASC, id ASC",
    )
    .bind(phpyun_core::numeric::checked_db_i64(uid, "company.uid")?)
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
    pub yyzz_status: i32,
    pub fact_status: i32,
}

pub async fn list_cards_by_uids(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<CompanyCard>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = std::iter::repeat_n("?", uids.len())
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
            CAST(COALESCE(cityid,0) AS SIGNED) AS cityid, \
            CAST(COALESCE(yyzz_status,0) AS SIGNED) AS yyzz_status, \
            CAST(COALESCE(fact_status,0) AS SIGNED) AS fact_status \
         FROM phpyun_company WHERE uid IN ({placeholders})"
    );
    let signed_uids = uids
        .iter()
        .copied()
        .map(|uid| phpyun_core::numeric::checked_db_i64(uid, "company.uid"))
        .collect::<Result<Vec<i64>, _>>()?;
    let mut q = sqlx::query_as::<_, CompanyCard>(&sql);
    for uid in signed_uids {
        q = q.bind(uid);
    }
    q.fetch_all(pool).await
}

/// PHP `company::getKhList` — name LIKE, with CRM admin display name.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KhCompany {
    pub name: String,
    pub crm_uid: i32,
    pub crm_name: String,
}

pub async fn list_kh_by_name(
    pool: &MySqlPool,
    keyword: &str,
    limit: u64,
) -> Result<Vec<KhCompany>, sqlx::Error> {
    let pattern = format!("%{keyword}%");
    sqlx::query_as::<_, KhCompany>(
        "SELECT \
            COALESCE(c.name, '') AS name, \
            CAST(COALESCE(c.crm_uid, 0) AS SIGNED) AS crm_uid, \
            COALESCE(NULLIF(a.name, ''), a.username, '') AS crm_name \
         FROM phpyun_company c \
         LEFT JOIN phpyun_admin_user a ON a.uid = c.crm_uid \
         WHERE c.name LIKE ? \
         ORDER BY c.uid DESC \
         LIMIT ?",
    )
    .bind(pattern)
    .bind(limit)
    .fetch_all(pool)
    .await
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
/// `{yun:}hotjob{/yun}` / `wap/index::getmq_action`. Joins `phpyun_hotjob`
/// with `phpyun_company`, filtering by:
///   - `phpyun_company.r_status = 1` (approved)
///   - `phpyun_hotjob.time_start < now AND time_end > now` (active banner)
///
/// PHP does **not** require `company.hottime` for this widget (that column is
/// used for sticky/rec companies). `sort_mode`: 0 = `sort` DESC (admin order),
/// 1 = `lastupdate` DESC, 2 = `RAND()`.
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
        _ => "h.sort DESC, h.id DESC",
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
         WHERE c.r_status = 1 \
           AND h.time_start < ? \
           AND h.time_end > ? \
           AND COALESCE(h.deleted,0)=0 \
         ORDER BY {order_clause} \
         LIMIT ?"
    );
    sqlx::query_as::<_, HotCompany>(&sql)
        .bind(sort_mode)
        .bind(now)
        .bind(now)
        .bind(limit)
        .fetch_all(pool)
        .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct HotJobRow {
    pub id: u64,
    pub uid: u64,
    pub username: String,
    pub hot_pic: String,
    pub time_start: i64,
    pub time_end: i64,
    pub sort: i32,
    pub beizhu: String,
    pub rating_id: i32,
}

pub async fn hotjob_list(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<HotJobRow>, sqlx::Error> {
    sqlx::query_as::<_, HotJobRow>(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
                  COALESCE(username, '') AS username,
                  COALESCE(hot_pic, '') AS hot_pic,
                  CAST(COALESCE(time_start, 0) AS SIGNED) AS time_start,
                  CAST(COALESCE(time_end, 0) AS SIGNED) AS time_end,
                  CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
                  COALESCE(beizhu, '') AS beizhu,
                  CAST(COALESCE(rating_id, 0) AS SIGNED) AS rating_id
           FROM phpyun_hotjob
           WHERE COALESCE(deleted,0)=0
           ORDER BY sort DESC, id DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn hotjob_count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_hotjob WHERE COALESCE(deleted,0)=0")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct HotJobUpsert<'a> {
    pub id: Option<u64>,
    pub uid: u64,
    pub username: &'a str,
    pub hot_pic: &'a str,
    pub time_start: i64,
    pub time_end: i64,
    pub sort: i32,
    pub beizhu: &'a str,
    pub rating_id: i32,
    pub now: i64,
}

pub async fn hotjob_upsert(pool: &MySqlPool, a: HotJobUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            r#"UPDATE phpyun_hotjob
               SET uid = ?, username = ?, hot_pic = ?, time_start = ?, time_end = ?,
                   sort = ?, beizhu = ?, rating_id = ?, lastupdate = ?
               WHERE id = ?"#,
        )
        .bind(a.uid)
        .bind(a.username)
        .bind(a.hot_pic)
        .bind(a.time_start)
        .bind(a.time_end)
        .bind(a.sort)
        .bind(a.beizhu)
        .bind(a.rating_id)
        .bind(a.now)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        r#"INSERT INTO phpyun_hotjob
           (uid, username, hot_pic, time_start, time_end, sort, beizhu, rating_id, lastupdate)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(a.uid)
    .bind(a.username)
    .bind(a.hot_pic)
    .bind(a.time_start)
    .bind(a.time_end)
    .bind(a.sort)
    .bind(a.beizhu)
    .bind(a.rating_id)
    .bind(a.now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn hotjob_delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_hotjob SET deleted=1 WHERE id = ? AND COALESCE(deleted,0)=0")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn hotjob_find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<HotJobRow>, sqlx::Error> {
    sqlx::query_as::<_, HotJobRow>(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
                  COALESCE(username, '') AS username,
                  COALESCE(hot_pic, '') AS hot_pic,
                  CAST(COALESCE(time_start, 0) AS SIGNED) AS time_start,
                  CAST(COALESCE(time_end, 0) AS SIGNED) AS time_end,
                  CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
                  COALESCE(beizhu, '') AS beizhu,
                  CAST(COALESCE(rating_id, 0) AS SIGNED) AS rating_id
           FROM phpyun_hotjob
           WHERE uid = ? AND COALESCE(deleted,0)=0
           ORDER BY id DESC LIMIT 1"#,
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

pub async fn hotjob_find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<HotJobRow>, sqlx::Error> {
    sqlx::query_as::<_, HotJobRow>(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
                  COALESCE(username, '') AS username,
                  COALESCE(hot_pic, '') AS hot_pic,
                  CAST(COALESCE(time_start, 0) AS SIGNED) AS time_start,
                  CAST(COALESCE(time_end, 0) AS SIGNED) AS time_end,
                  CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
                  COALESCE(beizhu, '') AS beizhu,
                  CAST(COALESCE(rating_id, 0) AS SIGNED) AS rating_id
           FROM phpyun_hotjob
           WHERE id = ? AND COALESCE(deleted,0)=0
           LIMIT 1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct CompanyExpireRow {
    pub uid: u64,
    pub name: String,
    pub rating: i32,
    pub rating_name: String,
    pub vip_stime: i64,
    pub vip_etime: i64,
}

pub async fn list_expire(
    pool: &MySqlPool,
    expired_only: bool,
    now: i64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyExpireRow>, sqlx::Error> {
    let sql = if expired_only {
        r#"SELECT CAST(c.uid AS UNSIGNED) AS uid,
                  COALESCE(c.name, '') AS name,
                  CAST(COALESCE(s.rating, 0) AS SIGNED) AS rating,
                  COALESCE(s.rating_name, '') AS rating_name,
                  CAST(COALESCE(s.vip_stime, 0) AS SIGNED) AS vip_stime,
                  CAST(COALESCE(s.vip_etime, 0) AS SIGNED) AS vip_etime
           FROM phpyun_company c
           INNER JOIN phpyun_company_statis s ON s.uid = c.uid
           WHERE s.vip_etime > 0 AND s.vip_etime < ?
           ORDER BY s.vip_etime ASC
           LIMIT ? OFFSET ?"#
    } else {
        r#"SELECT CAST(c.uid AS UNSIGNED) AS uid,
                  COALESCE(c.name, '') AS name,
                  CAST(COALESCE(s.rating, 0) AS SIGNED) AS rating,
                  COALESCE(s.rating_name, '') AS rating_name,
                  CAST(COALESCE(s.vip_stime, 0) AS SIGNED) AS vip_stime,
                  CAST(COALESCE(s.vip_etime, 0) AS SIGNED) AS vip_etime
           FROM phpyun_company c
           INNER JOIN phpyun_company_statis s ON s.uid = c.uid
           WHERE s.vip_etime > 0
           ORDER BY s.vip_etime ASC
           LIMIT ? OFFSET ?"#
    };
    let q = sqlx::query_as::<_, CompanyExpireRow>(sql);
    if expired_only {
        q.bind(now).bind(limit).bind(offset).fetch_all(pool).await
    } else {
        q.bind(limit).bind(offset).fetch_all(pool).await
    }
}

pub async fn count_expire(
    pool: &MySqlPool,
    expired_only: bool,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = if expired_only {
        sqlx::query_as(
            "SELECT COUNT(*) FROM phpyun_company_statis s WHERE s.vip_etime > 0 AND s.vip_etime < ?",
        )
        .bind(now)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_statis s WHERE s.vip_etime > 0")
            .fetch_one(pool)
            .await?
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminCompanyRow {
    pub uid: u64,
    pub name: String,
    pub r_status: i32,
    pub hy: i32,
    pub cityid: i32,
    pub hits: i32,
    pub rating: i32,
    pub rating_name: String,
    pub yyzz_status: i32,
    pub login_date: i64,
    pub lastupdate: i64,
    pub logo: String,
}

pub async fn list_admin(
    pool: &MySqlPool,
    r_status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminCompanyRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        r#"SELECT CAST(uid AS UNSIGNED) AS uid,
                  COALESCE(name, '') AS name,
                  CAST(COALESCE(r_status, 0) AS SIGNED) AS r_status,
                  CAST(COALESCE(hy, 0) AS SIGNED) AS hy,
                  CAST(COALESCE(cityid, 0) AS SIGNED) AS cityid,
                  CAST(COALESCE(hits, 0) AS SIGNED) AS hits,
                  CAST(COALESCE(rating, 0) AS SIGNED) AS rating,
                  COALESCE(rating_name, '') AS rating_name,
                  CAST(COALESCE(yyzz_status, 0) AS SIGNED) AS yyzz_status,
                  CAST(COALESCE(login_date, 0) AS SIGNED) AS login_date,
                  CAST(COALESCE(lastupdate, 0) AS SIGNED) AS lastupdate,
                  COALESCE(logo, '') AS logo
           FROM phpyun_company WHERE 1=1"#,
    );
    push_admin_company_filters(&mut qb, r_status, keyword);
    qb.push(" ORDER BY uid DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<AdminCompanyRow>().fetch_all(pool).await
}

pub async fn count_admin(
    pool: &MySqlPool,
    r_status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company WHERE 1=1");
    push_admin_company_filters(&mut qb, r_status, keyword);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_admin_company_filters<'a>(
    qb: &mut QueryBuilder<'a, sqlx::MySql>,
    r_status: Option<i32>,
    keyword: Option<&'a str>,
) {
    if let Some(st) = r_status {
        qb.push(" AND r_status = ");
        qb.push_bind(st);
    }
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct CompanyRatingOpt {
    pub id: i32,
    pub name: String,
}

pub async fn list_rating_options(pool: &MySqlPool) -> Result<Vec<CompanyRatingOpt>, sqlx::Error> {
    sqlx::query_as::<_, CompanyRatingOpt>(
        r#"SELECT CAST(id AS SIGNED) AS id, COALESCE(name, '') AS name
           FROM phpyun_company_rating
           WHERE category = 1 AND COALESCE(deleted,0)=0
           ORDER BY sort DESC, id ASC"#,
    )
    .fetch_all(pool)
    .await
}

pub async fn set_rating(
    pool: &MySqlPool,
    uid: u64,
    rating: i32,
    rating_name: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company SET rating = ?, rating_name = ? WHERE uid = ?")
        .bind(rating)
        .bind(rating_name)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn set_r_status(pool: &MySqlPool, uid: u64, r_status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn count_r_status_except(
    pool: &MySqlPool,
    r_status: i32,
    except_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company WHERE r_status = ? AND uid <> ?",
    )
    .bind(r_status)
    .bind(except_uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_yyzz(
    pool: &MySqlPool,
    uid: u64,
    yyzz_status: i32,
    name: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let res = if let Some(n) = name.filter(|s| !s.is_empty()) {
        sqlx::query("UPDATE phpyun_company SET yyzz_status = ?, name = ? WHERE uid = ?")
            .bind(yyzz_status)
            .bind(n)
            .bind(uid)
            .execute(pool)
            .await?
    } else {
        sqlx::query("UPDATE phpyun_company SET yyzz_status = ? WHERE uid = ?")
            .bind(yyzz_status)
            .bind(uid)
            .execute(pool)
            .await?
    };
    Ok(res.rows_affected())
}

pub async fn set_email_lock(
    pool: &MySqlPool,
    uid: u64,
    email: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company SET linkmail = ?, email_status = ? WHERE uid = ?",
    )
    .bind(email)
    .bind(status)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn set_mobile_lock(
    pool: &MySqlPool,
    uid: u64,
    mobile: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company SET linktel = ?, moblie_status = ? WHERE uid = ?",
    )
    .bind(mobile)
    .bind(status)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn set_vip_times(
    pool: &MySqlPool,
    uid: u64,
    stime: i64,
    etime: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_company SET vipstime = ?, vipetime = ? WHERE uid = ?")
        .bind(stime)
        .bind(etime)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// PHP `company::index_action` + `company.model::getDataList` admin row.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpCompanyListRow {
    pub uid: u64,
    pub name: String,
    pub shortname: String,
    pub r_status: i32,
    pub rating: i32,
    pub rating_name: String,
    pub vipetime: i64,
    pub yyzz_status: i32,
    pub logo: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub crm_uid: i32,
    pub crm_name: String,
    pub fact_status: i32,
    pub moblie_status: i32,
    pub email_status: i32,
    pub username: String,
    pub usertype: i32,
    pub wxid: String,
    pub wxopenid: String,
    pub unionid: String,
    pub lock_info: String,
    pub source: i32,
    pub login_ip: String,
    pub login_address: String,
    pub moblie_address: String,
    pub login_date: i64,
    pub reg_date: i64,
    pub jobnum: i32,
    pub zz_jobnum: i32,
}

pub struct PhpCompanyListFilter<'a> {
    pub keyword: Option<&'a str>,
    pub kw_type: i32,
    pub r_status: Option<i32>,
    pub rating: Option<i32>,
    pub rec: Option<i32>,
    pub source: Option<i32>,
    pub crm_uid: Option<i32>,
    pub has_job: Option<i32>,
    pub fact_status: Option<i32>,
    pub map_status: Option<i32>,
    pub city_class: Option<&'a str>,
    pub time_col: Option<&'a str>,
    pub time_from: Option<i64>,
    pub time_to: Option<i64>,
    pub order_t: &'a str,
    pub order_dir: &'a str,
}

fn push_php_company_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpCompanyListFilter<'a>) {
    if let Some(st) = f.r_status {
        qb.push(" AND c.r_status = ");
        qb.push_bind(st);
    }
    if let Some(r) = f.rating.filter(|v| *v > 0) {
        qb.push(" AND c.rating = ");
        qb.push_bind(r);
    }
    if let Some(rec) = f.rec.filter(|v| *v > 0) {
        qb.push(" AND c.rec = ");
        qb.push_bind(rec);
    }
    if let Some(src) = f.source.filter(|v| *v > 0) {
        qb.push(" AND m.source = ");
        qb.push_bind(src);
    }
    if let Some(gw) = f.crm_uid.filter(|v| *v > 0) {
        qb.push(" AND c.crm_uid = ");
        qb.push_bind(gw);
    }
    match f.has_job {
        Some(1) => {
            qb.push(" AND COALESCE(c.jobtime,0) > 0");
        }
        Some(2) => {
            qb.push(" AND COALESCE(c.jobtime,0) = 0");
        }
        _ => {}
    }
    match f.fact_status {
        Some(1) => {
            qb.push(" AND COALESCE(c.fact_status,0) = 1");
        }
        Some(2) => {
            qb.push(" AND COALESCE(c.fact_status,0) = 0");
        }
        _ => {}
    }
    match f.map_status {
        Some(1) => {
            qb.push(" AND COALESCE(c.x,'') <> '' AND COALESCE(c.y,'') <> ''");
        }
        Some(2) => {
            qb.push(" AND (COALESCE(c.x,'') = '' OR COALESCE(c.y,'') = '')");
        }
        _ => {}
    }
    if let Some(city) = f.city_class.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (FIND_IN_SET(c.provinceid, ");
        qb.push_bind(city);
        qb.push(") OR FIND_IN_SET(c.cityid, ");
        qb.push_bind(city);
        qb.push(") OR FIND_IN_SET(c.three_cityid, ");
        qb.push_bind(city);
        qb.push("))");
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        match f.kw_type {
            2 => {
                qb.push(" AND m.username LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            3 => {
                qb.push(" AND c.linkman LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            4 => {
                qb.push(" AND c.linktel LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            5 => {
                qb.push(" AND c.linkmail LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            6 => {
                let uid: u64 = kw.parse().unwrap_or(0);
                qb.push(" AND c.uid = ");
                qb.push_bind(uid);
            }
            7 => {
                qb.push(" AND m.login_ip LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            8 => {
                qb.push(" AND c.address LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            _ => {
                qb.push(" AND (c.name LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(" OR c.shortname LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(")");
            }
        }
    }
    if let (Some(col), Some(from), Some(to)) = (f.time_col, f.time_from, f.time_to) {
        let col = match col {
            "login_date" => "COALESCE(m.login_date, c.login_date)",
            _ => "m.reg_date",
        };
        qb.push(" AND ");
        qb.push(col);
        qb.push(" >= ");
        qb.push_bind(from);
        qb.push(" AND ");
        qb.push(col);
        qb.push(" <= ");
        qb.push_bind(to);
    }
}

fn push_php_company_order(qb: &mut QueryBuilder<'_, sqlx::MySql>, t: &str, order: &str) {
    let asc = order.eq_ignore_ascii_case("asc");
    qb.push(" ORDER BY ");
    qb.push(match (t, asc) {
        ("uid", true) => "c.uid ASC",
        ("login_date", true) => "COALESCE(m.login_date, c.login_date) ASC",
        ("login_date", false) => "COALESCE(m.login_date, c.login_date) DESC",
        ("vipetime", true) => "c.vipetime ASC",
        ("vipetime", false) => "c.vipetime DESC",
        _ => "c.uid DESC",
    });
}

const PHP_COMPANY_LIST_FIELDS: &str = "CAST(c.uid AS UNSIGNED) AS uid, COALESCE(c.name,'') AS name, \
    COALESCE(c.shortname,'') AS shortname, CAST(COALESCE(c.r_status,0) AS SIGNED) AS r_status, \
    CAST(COALESCE(c.rating,0) AS SIGNED) AS rating, COALESCE(c.rating_name,'') AS rating_name, \
    CAST(COALESCE(c.vipetime,0) AS SIGNED) AS vipetime, \
    CAST(COALESCE(c.yyzz_status,0) AS SIGNED) AS yyzz_status, COALESCE(c.logo,'') AS logo, \
    COALESCE(c.linktel,'') AS linktel, COALESCE(c.linkphone,'') AS linkphone, \
    COALESCE(c.linkmail,'') AS linkmail, CAST(COALESCE(c.crm_uid,0) AS SIGNED) AS crm_uid, \
    COALESCE(NULLIF(a.name,''), a.username, '') AS crm_name, \
    CAST(COALESCE(c.fact_status,0) AS SIGNED) AS fact_status, \
    CAST(COALESCE(c.moblie_status,0) AS SIGNED) AS moblie_status, \
    CAST(COALESCE(c.email_status,0) AS SIGNED) AS email_status, \
    COALESCE(m.username,'') AS username, CAST(COALESCE(m.usertype,2) AS SIGNED) AS usertype, \
    COALESCE(m.wxid,'') AS wxid, COALESCE(m.wxopenid,'') AS wxopenid, \
    COALESCE(m.unionid,'') AS unionid, COALESCE(m.lock_info,'') AS lock_info, \
    CAST(COALESCE(m.source,0) AS SIGNED) AS source, COALESCE(m.login_ip,'') AS login_ip, \
    COALESCE(m.login_address,'') AS login_address, COALESCE(m.moblie_address,'') AS moblie_address, \
    CAST(COALESCE(m.login_date, c.login_date, 0) AS SIGNED) AS login_date, \
    CAST(COALESCE(m.reg_date,0) AS SIGNED) AS reg_date, \
    CAST((SELECT COUNT(*) FROM phpyun_company_job j WHERE j.uid = c.uid) AS SIGNED) AS jobnum, \
    CAST((SELECT COUNT(*) FROM phpyun_company_job j WHERE j.uid = c.uid AND COALESCE(j.status,0) = 0 AND COALESCE(j.state,0) = 1) AS SIGNED) AS zz_jobnum";

pub async fn list_php_companies(
    pool: &MySqlPool,
    f: &PhpCompanyListFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpCompanyListRow>, sqlx::Error> {
    let limit = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let offset = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(PHP_COMPANY_LIST_FIELDS);
    qb.push(
        " FROM phpyun_company c \
         LEFT JOIN phpyun_member m ON m.uid = c.uid \
         LEFT JOIN phpyun_admin_user a ON a.uid = c.crm_uid \
         WHERE 1=1",
    );
    push_php_company_filters(&mut qb, f);
    push_php_company_order(&mut qb, f.order_t, f.order_dir);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_php_companies(
    pool: &MySqlPool,
    f: &PhpCompanyListFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_company c \
         LEFT JOIN phpyun_member m ON m.uid = c.uid WHERE 1=1",
    );
    push_php_company_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}
