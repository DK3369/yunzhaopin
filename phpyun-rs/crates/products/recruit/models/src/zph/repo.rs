//! Strictly aligned with PHPYun's job-fair tables:
//!
//! - `phpyun_zhaopinhui`      — the job-fair record itself
//! - `phpyun_zhaopinhui_com`  — participating companies (includes a jobid field; locally acts as the "reservation" concept)
//! - `phpyun_zhaopinhui_space`— booth/stall definitions (id/name/sort/keyid/pic/content/price); this is **not**
//!   the "company reservation" table — on the Rust side, ZphReservation has been
//!   repurposed to map to phpyun_zhaopinhui_com.
//!
//! Zph field mapping (Rust → PHP):
//!   - body       ↔ body (same name in PHP)
//!   - banner     ↔ banner (same name in PHP)
//!   - city_id    ↔ cityid
//!   - start_at   ↔ UNIX_TIMESTAMP(starttime) (PHP stores a varchar date)
//!   - end_at     ↔ UNIX_TIMESTAMP(endtime)
//!   - status     ↔ is_open (1=open / 0=closed; PHP's `status` column is a workflow state, while is_open is the listing flag)
//!   - created_at ↔ ctime

use super::entity::{Zph, ZphCompany, ZphReservation, ZphSpace};
use crate::soft_delete::{self, PREDICATE};
use sqlx::MySqlPool;

const ZPH_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(body, '') AS body, \
    COALESCE(banner, '') AS banner, \
    CAST(COALESCE(cityid, 0) AS SIGNED) AS city_id, \
    COALESCE(address, '') AS address, \
    CAST(COALESCE(UNIX_TIMESTAMP(starttime), 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(UNIX_TIMESTAMP(endtime), 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(is_open, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(sid, 0) AS SIGNED) AS sid, \
    COALESCE(pic, '') AS pic, \
    CAST(COALESCE(provinceid, 0) AS SIGNED) AS province_id, \
    COALESCE(traffic, '') AS traffic, \
    COALESCE(phone, '') AS phone, \
    COALESCE(organizers, '') AS organizers, \
    COALESCE(user, '') AS user, \
    COALESCE(weburl, '') AS weburl, \
    COALESCE(media, '') AS media, \
    COALESCE(packages, '') AS packages, \
    COALESCE(booth, '') AS booth, \
    COALESCE(participate, '') AS participate, \
    COALESCE(zwpic, '') AS zwpic, \
    COALESCE(reserved, '') AS reserved, \
    COALESCE(is_themb_wap, '') AS is_themb_wap, \
    COALESCE(banner_wap, '') AS banner_wap, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(is_open, 0) AS SIGNED) AS is_open";

pub async fn list(pool: &MySqlPool, offset: u64, limit: u64) -> Result<Vec<Zph>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZPH_FIELDS} FROM phpyun_zhaopinhui \
         WHERE is_open = 1 AND {PREDICATE} ORDER BY UNIX_TIMESTAMP(starttime) DESC, id DESC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Zph>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_zhaopinhui WHERE is_open = 1 AND {PREDICATE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Zph>, sqlx::Error> {
    let sql = format!("SELECT {ZPH_FIELDS} FROM phpyun_zhaopinhui WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Zph>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Zph>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZPH_FIELDS} FROM phpyun_zhaopinhui \
         WHERE {PREDICATE} ORDER BY UNIX_TIMESTAMP(starttime) DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Zph>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_zhaopinhui WHERE {PREDICATE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_open(pool: &MySqlPool, id: u64, is_open: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_zhaopinhui SET is_open = ? WHERE id = ?")
        .bind(is_open)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ---------- companies ----------

const ZC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(zid, 0) AS UNSIGNED) AS zid, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    CAST(COALESCE(sid, 0) AS SIGNED) AS sid, \
    CAST(COALESCE(cid, 0) AS SIGNED) AS cid, \
    CAST(COALESCE(bid, 0) AS SIGNED) AS bid";

pub async fn list_companies(
    pool: &MySqlPool,
    zid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<ZphCompany>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZC_FIELDS} FROM phpyun_zhaopinhui_com \
         WHERE zid = ? AND status = 1 \
         ORDER BY sort DESC, ctime ASC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, ZphCompany>(&sql)
        .bind(zid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// Pull every job-id signed up to a recruitment fair. The PHP schema stores
/// these as a CSV string per `phpyun_zhaopinhui_com.jobid`, so this just
/// loads the raw CSVs and lets the caller flatten + dedupe them.
pub async fn jobid_csvs_for_zph(pool: &MySqlPool, zid: u64) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT COALESCE(jobid, '') FROM phpyun_zhaopinhui_com \
           WHERE zid = ? AND status = 1",
    )
    .bind(zid)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(s,)| s)
        .filter(|s| !s.is_empty())
        .collect())
}

pub async fn count_companies(pool: &MySqlPool, zid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_zhaopinhui_com WHERE zid = ? AND status = 1")
            .bind(zid)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- reservations ----------
//
// In PHPYun, "company sign-ups for the job fair" are stored in phpyun_zhaopinhui_com.jobid (varchar);
// PHP has no dedicated "reservation" table. On the Rust side we treat it as a reservation table for read/write.

const ZR_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(zid, 0) AS UNSIGNED) AS zid, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(jobid, '') AS job_ids, \
    COALESCE(com_name, '') AS name, \
    '' AS mobile, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

pub struct ReservationCreate<'a> {
    pub zid: u64,
    pub uid: u64,
    pub job_ids: &'a str,
    pub name: &'a str,
    pub sid: i32,
    pub cid: i32,
    pub bid: i32,
}

pub async fn upsert_reservation(
    pool: &MySqlPool,
    r: ReservationCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // PHP reserveZph always INSERTs a pending row (status=0); never overwrites.
    let res = sqlx::query(
        "INSERT INTO phpyun_zhaopinhui_com \
         (zid, uid, jobid, com_name, ctime, status, sid, cid, bid) \
         VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?)",
    )
    .bind(r.zid)
    .bind(r.uid)
    .bind(r.job_ids)
    .bind(r.name)
    .bind(now)
    .bind(r.sid)
    .bind(r.cid)
    .bind(r.bid)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

const ZS_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(keyid, 0) AS SIGNED) AS keyid, \
    COALESCE(pic, '') AS pic, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(price, 0) AS SIGNED) AS price";

pub async fn list_spaces(
    pool: &MySqlPool,
    keyid: Option<i64>,
    keyword: Option<&str>,
) -> Result<Vec<ZphSpace>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZS_FIELDS} FROM phpyun_zhaopinhui_space WHERE {PREDICATE} \
         {key} {kw} ORDER BY sort ASC, id ASC",
        key = if keyid.is_some() { "AND keyid = ?" } else { "AND keyid = 0" },
        kw = if keyword.map(|s| !s.is_empty()).unwrap_or(false) {
            "AND name LIKE ?"
        } else {
            ""
        }
    );
    let mut q = sqlx::query_as::<_, ZphSpace>(&sql);
    if let Some(k) = keyid {
        q = q.bind(k);
    }
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            q = q.bind(format!("%{kw}%"));
        }
    }
    q.fetch_all(pool).await
}

pub struct SpaceUpsert<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub sort: i32,
    pub keyid: i64,
    pub pic: &'a str,
    pub content: &'a str,
    pub price: i32,
}

pub async fn upsert_space(pool: &MySqlPool, s: SpaceUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = s.id {
        sqlx::query(
            "UPDATE phpyun_zhaopinhui_space SET name=?, sort=?, keyid=?, pic=?, content=?, price=? WHERE id=?",
        )
        .bind(s.name)
        .bind(s.sort)
        .bind(s.keyid)
        .bind(s.pic)
        .bind(s.content)
        .bind(s.price)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_zhaopinhui_space (name, sort, keyid, pic, content, price) VALUES (?,?,?,?,?,?)",
        )
        .bind(s.name)
        .bind(s.sort)
        .bind(s.keyid)
        .bind(s.pic)
        .bind(s.content)
        .bind(s.price)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_space(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_zhaopinhui_space", id).await
}

pub async fn find_my_reservation(
    pool: &MySqlPool,
    zid: u64,
    uid: u64,
) -> Result<Option<ZphReservation>, sqlx::Error> {
    let sql =
        format!("SELECT {ZR_FIELDS} FROM phpyun_zhaopinhui_com WHERE zid = ? AND uid = ? LIMIT 1");
    sqlx::query_as::<_, ZphReservation>(&sql)
        .bind(zid)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

// ---------- admin PHP shapes ----------

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminZphListRow {
    pub id: u64,
    pub title: String,
    pub address: String,
    pub starttime: String,
    pub endtime: String,
    pub did: i32,
    pub is_open: i32,
    pub sid: i32,
    pub reserved: String,
    pub comnum: i64,
    pub booking: i64,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminZphFormRow {
    pub id: u64,
    pub title: String,
    pub sid: i32,
    pub address: String,
    pub traffic: String,
    pub phone: String,
    pub organizers: String,
    pub user: String,
    pub starttime: String,
    pub endtime: String,
    pub body: String,
    pub media: String,
    pub packages: String,
    pub booth: String,
    pub participate: String,
    pub did: i32,
    pub reserved: String,
    pub is_open: i32,
    pub is_themb: String,
    pub banner: String,
    pub is_themb_wap: String,
    pub banner_wap: String,
    pub pic: String,
    pub weburl: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminZphComRow {
    pub id: u64,
    pub uid: u64,
    pub zid: u64,
    pub jobid: String,
    pub ctime: i64,
    pub status: i32,
    pub statusbody: String,
    pub sid: i32,
    pub cid: i32,
    pub bid: i32,
    pub price: i32,
    pub com_name: String,
    pub sort: i32,
}

pub struct AdminZphListFilter<'a> {
    pub keyword: Option<&'a str>,
    pub keyword_type: i32,
    pub status: i32,
}

fn push_zph_admin_filters<'a>(
    qb: &mut sqlx::QueryBuilder<'a, sqlx::MySql>,
    f: &AdminZphListFilter<'a>,
    now: i64,
) {
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if f.keyword_type == 2 {
            qb.push(" AND address LIKE ");
            qb.push_bind(like);
        } else {
            qb.push(" AND title LIKE ");
            qb.push_bind(like);
        }
    }
    match f.status {
        3 => {
            qb.push(" AND UNIX_TIMESTAMP(starttime) > ");
            qb.push_bind(now);
        }
        1 => {
            qb.push(" AND UNIX_TIMESTAMP(starttime) < ");
            qb.push_bind(now);
            qb.push(" AND UNIX_TIMESTAMP(endtime) > ");
            qb.push_bind(now);
        }
        2 => {
            qb.push(" AND UNIX_TIMESTAMP(endtime) < ");
            qb.push_bind(now);
        }
        _ => {}
    }
}

const ADMIN_ZPH_LIST: &str = "\
    CAST(z.id AS UNSIGNED) AS id, \
    COALESCE(z.title, '') AS title, \
    COALESCE(z.address, '') AS address, \
    COALESCE(z.starttime, '') AS starttime, \
    COALESCE(z.endtime, '') AS endtime, \
    CAST(COALESCE(z.did, 0) AS SIGNED) AS did, \
    CAST(COALESCE(z.is_open, 0) AS SIGNED) AS is_open, \
    CAST(COALESCE(z.sid, 0) AS SIGNED) AS sid, \
    COALESCE(z.reserved, '') AS reserved, \
    CAST((SELECT COUNT(*) FROM phpyun_zhaopinhui_com c WHERE c.zid = z.id AND c.status = 1) AS SIGNED) AS comnum, \
    CAST((SELECT COUNT(*) FROM phpyun_zhaopinhui_com c WHERE c.zid = z.id AND c.status = 0) AS SIGNED) AS booking";

pub async fn admin_list_filtered(
    pool: &MySqlPool,
    f: &AdminZphListFilter<'_>,
    now: i64,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminZphListRow>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(format!(
        "SELECT {ADMIN_ZPH_LIST} FROM phpyun_zhaopinhui z WHERE COALESCE(z.deleted,0)=0"
    ));
    push_zph_admin_filters(&mut qb, f, now);
    qb.push(" ORDER BY z.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn admin_count_filtered(
    pool: &MySqlPool,
    f: &AdminZphListFilter<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_zhaopinhui z WHERE COALESCE(z.deleted,0)=0",
    );
    push_zph_admin_filters(&mut qb, f, now);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_admin_form(pool: &MySqlPool, id: u64) -> Result<Option<AdminZphFormRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminZphFormRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, CAST(COALESCE(sid,0) AS SIGNED) AS sid, \
         COALESCE(address,'') AS address, COALESCE(traffic,'') AS traffic, COALESCE(phone,'') AS phone, \
         COALESCE(organizers,'') AS organizers, COALESCE(`user`,'') AS user, \
         COALESCE(starttime,'') AS starttime, COALESCE(endtime,'') AS endtime, \
         COALESCE(body,'') AS body, COALESCE(media,'') AS media, COALESCE(packages,'') AS packages, \
         COALESCE(booth,'') AS booth, COALESCE(participate,'') AS participate, \
         CAST(COALESCE(did,0) AS SIGNED) AS did, COALESCE(reserved,'') AS reserved, \
         CAST(COALESCE(is_open,0) AS SIGNED) AS is_open, COALESCE(is_themb,'') AS is_themb, \
         COALESCE(banner,'') AS banner, COALESCE(is_themb_wap,'') AS is_themb_wap, \
         COALESCE(banner_wap,'') AS banner_wap, COALESCE(pic,'') AS pic, COALESCE(weburl,'') AS weburl \
         FROM phpyun_zhaopinhui WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub struct ZphInfoWrite<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub sid: i32,
    pub address: &'a str,
    pub traffic: &'a str,
    pub phone: &'a str,
    pub organizers: &'a str,
    pub user: &'a str,
    pub starttime: &'a str,
    pub endtime: &'a str,
    pub body: &'a str,
    pub media: &'a str,
    pub packages: &'a str,
    pub booth: &'a str,
    pub participate: &'a str,
    pub did: i32,
    pub reserved: &'a str,
    pub is_open: i32,
    pub is_themb: &'a str,
    pub banner: &'a str,
    pub is_themb_wap: &'a str,
    pub banner_wap: &'a str,
    pub now: i64,
}

pub async fn upsert_info(pool: &MySqlPool, a: ZphInfoWrite<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            "UPDATE phpyun_zhaopinhui SET title=?, sid=?, address=?, traffic=?, phone=?, organizers=?, \
             `user`=?, starttime=?, endtime=?, body=?, media=?, packages=?, booth=?, participate=?, \
             did=?, reserved=?, is_open=?, is_themb=?, banner=?, is_themb_wap=?, banner_wap=? WHERE id=?",
        )
        .bind(a.title)
        .bind(a.sid)
        .bind(a.address)
        .bind(a.traffic)
        .bind(a.phone)
        .bind(a.organizers)
        .bind(a.user)
        .bind(a.starttime)
        .bind(a.endtime)
        .bind(a.body)
        .bind(a.media)
        .bind(a.packages)
        .bind(a.booth)
        .bind(a.participate)
        .bind(a.did)
        .bind(a.reserved)
        .bind(a.is_open)
        .bind(a.is_themb)
        .bind(a.banner)
        .bind(a.is_themb_wap)
        .bind(a.banner_wap)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_zhaopinhui (title, sid, address, traffic, phone, organizers, `user`, \
             starttime, endtime, body, media, packages, booth, participate, ctime, status, did, reserved, \
             is_open, is_themb, banner, is_themb_wap, banner_wap) \
             VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,?,?,?,?,?,?,?)",
        )
        .bind(a.title)
        .bind(a.sid)
        .bind(a.address)
        .bind(a.traffic)
        .bind(a.phone)
        .bind(a.organizers)
        .bind(a.user)
        .bind(a.starttime)
        .bind(a.endtime)
        .bind(a.body)
        .bind(a.media)
        .bind(a.packages)
        .bind(a.booth)
        .bind(a.participate)
        .bind(a.now)
        .bind(a.did)
        .bind(a.reserved)
        .bind(a.is_open)
        .bind(a.is_themb)
        .bind(a.banner)
        .bind(a.is_themb_wap)
        .bind(a.banner_wap)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_zph_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_zhaopinhui", ids).await
}

pub async fn set_did_ids(pool: &MySqlPool, ids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("UPDATE phpyun_zhaopinhui SET did = ");
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
    let n = qb.build().execute(pool).await?.rows_affected();
    let mut qb2 = sqlx::QueryBuilder::new("UPDATE phpyun_zhaopinhui_com SET did = ");
    qb2.push_bind(did);
    qb2.push(" WHERE zid IN (");
    first = true;
    for id in ids {
        if !first {
            qb2.push(",");
        }
        qb2.push_bind(*id);
        first = false;
    }
    qb2.push(")");
    let _ = qb2.build().execute(pool).await?;
    Ok(n)
}

pub struct AdminZphComFilter<'a> {
    pub zid: Option<u64>,
    pub status: Option<i32>,
    pub keyword: Option<&'a str>,
    pub keyword_type: i32,
}

pub async fn admin_list_coms(
    pool: &MySqlPool,
    f: &AdminZphComFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminZphComRow>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT CAST(c.id AS UNSIGNED) AS id, CAST(COALESCE(c.uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(c.zid,0) AS UNSIGNED) AS zid, COALESCE(c.jobid,'') AS jobid, \
         CAST(COALESCE(c.ctime,0) AS SIGNED) AS ctime, CAST(COALESCE(c.status,0) AS SIGNED) AS status, \
         COALESCE(c.statusbody,'') AS statusbody, CAST(COALESCE(c.sid,0) AS SIGNED) AS sid, \
         CAST(COALESCE(c.cid,0) AS SIGNED) AS cid, CAST(COALESCE(c.bid,0) AS SIGNED) AS bid, \
         CAST(COALESCE(c.price,0) AS SIGNED) AS price, COALESCE(c.com_name, co.name, '') AS com_name, \
         CAST(COALESCE(c.sort,0) AS SIGNED) AS sort \
         FROM phpyun_zhaopinhui_com c LEFT JOIN phpyun_company co ON co.uid = c.uid WHERE 1=1",
    );
    if let Some(zid) = f.zid.filter(|z| *z > 0) {
        qb.push(" AND c.zid = ");
        qb.push_bind(zid);
    }
    if let Some(st) = f.status {
        qb.push(" AND c.status = ");
        qb.push_bind(st);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if f.keyword_type == 1 {
            qb.push(" AND c.zid IN (SELECT id FROM phpyun_zhaopinhui WHERE title LIKE ");
            qb.push_bind(like);
            qb.push(" AND COALESCE(deleted,0)=0)");
        } else if f.keyword_type == 2 {
            qb.push(" AND (co.name LIKE ");
            qb.push_bind(like.clone());
            qb.push(" OR c.com_name LIKE ");
            qb.push_bind(like);
            qb.push(")");
        }
    }
    qb.push(" ORDER BY c.status ASC, c.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn admin_count_coms(pool: &MySqlPool, f: &AdminZphComFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_zhaopinhui_com c LEFT JOIN phpyun_company co ON co.uid = c.uid WHERE 1=1",
    );
    if let Some(zid) = f.zid.filter(|z| *z > 0) {
        qb.push(" AND c.zid = ");
        qb.push_bind(zid);
    }
    if let Some(st) = f.status {
        qb.push(" AND c.status = ");
        qb.push_bind(st);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        if f.keyword_type == 1 {
            qb.push(" AND c.zid IN (SELECT id FROM phpyun_zhaopinhui WHERE title LIKE ");
            qb.push_bind(like);
            qb.push(" AND COALESCE(deleted,0)=0)");
        } else if f.keyword_type == 2 {
            qb.push(" AND (co.name LIKE ");
            qb.push_bind(like.clone());
            qb.push(" OR c.com_name LIKE ");
            qb.push_bind(like);
            qb.push(")");
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_com_admin(pool: &MySqlPool, id: u64) -> Result<Option<AdminZphComRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminZphComRow>(
        "SELECT CAST(c.id AS UNSIGNED) AS id, CAST(COALESCE(c.uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(c.zid,0) AS UNSIGNED) AS zid, COALESCE(c.jobid,'') AS jobid, \
         CAST(COALESCE(c.ctime,0) AS SIGNED) AS ctime, CAST(COALESCE(c.status,0) AS SIGNED) AS status, \
         COALESCE(c.statusbody,'') AS statusbody, CAST(COALESCE(c.sid,0) AS SIGNED) AS sid, \
         CAST(COALESCE(c.cid,0) AS SIGNED) AS cid, CAST(COALESCE(c.bid,0) AS SIGNED) AS bid, \
         CAST(COALESCE(c.price,0) AS SIGNED) AS price, COALESCE(c.com_name, co.name, '') AS com_name, \
         CAST(COALESCE(c.sort,0) AS SIGNED) AS sort \
         FROM phpyun_zhaopinhui_com c LEFT JOIN phpyun_company co ON co.uid = c.uid WHERE c.id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update_coms_status(
    pool: &MySqlPool,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("UPDATE phpyun_zhaopinhui_com SET status = ");
    qb.push_bind(status);
    qb.push(", statusbody = ");
    qb.push_bind(statusbody);
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

pub async fn update_com_fields(
    pool: &MySqlPool,
    id: u64,
    jobid: Option<&str>,
    cid: Option<i32>,
    bid: Option<i32>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut sets: Vec<&str> = Vec::new();
    if jobid.is_some() {
        sets.push("jobid = ?");
    }
    if cid.is_some() {
        sets.push("cid = ?");
    }
    if bid.is_some() {
        sets.push("bid = ?");
    }
    if sort.is_some() {
        sets.push("sort = ?");
    }
    if sets.is_empty() {
        return Ok(0);
    }
    let sql = format!("UPDATE phpyun_zhaopinhui_com SET {} WHERE id = ?", sets.join(", "));
    let mut q = sqlx::query(&sql);
    if let Some(v) = jobid {
        q = q.bind(v);
    }
    if let Some(v) = cid {
        q = q.bind(v);
    }
    if let Some(v) = bid {
        q = q.bind(v);
    }
    if let Some(v) = sort {
        q = q.bind(v);
    }
    Ok(q.bind(id).execute(pool).await?.rows_affected())
}

pub async fn insert_zph_com(
    pool: &MySqlPool,
    zid: u64,
    uid: u64,
    sid: i32,
    cid: i32,
    bid: i32,
    jobid: &str,
    com_name: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_zhaopinhui_com (uid, zid, jobid, ctime, status, sid, cid, bid, com_name) \
         VALUES (?, ?, ?, ?, 1, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(zid)
    .bind(jobid)
    .bind(now)
    .bind(sid)
    .bind(cid)
    .bind(bid)
    .bind(com_name)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_coms(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = sqlx::QueryBuilder::new("DELETE FROM phpyun_zhaopinhui_com WHERE id IN (");
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

pub async fn search_company_labels(
    pool: &MySqlPool,
    name: &str,
    limit: u64,
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED), COALESCE(name,'') FROM phpyun_company \
         WHERE name LIKE ? ORDER BY uid DESC LIMIT ?",
    )
    .bind(format!("%{name}%"))
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn job_labels_for_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<(u64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED), COALESCE(name,'') FROM phpyun_company_job \
         WHERE uid = ? AND state = 1 AND r_status <> 2 AND status <> 1 ORDER BY lastupdate DESC LIMIT 200",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn space_name_map(pool: &MySqlPool) -> Result<Vec<(u64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED), COALESCE(name,'') FROM phpyun_zhaopinhui_space WHERE COALESCE(deleted,0)=0",
    )
    .fetch_all(pool)
    .await
}

pub async fn space_children(
    pool: &MySqlPool,
    keyid: i64,
) -> Result<Vec<crate::zph::entity::ZphSpace>, sqlx::Error> {
    list_spaces(pool, Some(keyid), None).await
}

pub async fn reserved_parent_pairs(
    pool: &MySqlPool,
    reserved_ids: &[u64],
) -> Result<Vec<(u64, i64)>, sqlx::Error> {
    if reserved_ids.is_empty() {
        return Ok(vec![]);
    }
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED), CAST(COALESCE(keyid,0) AS SIGNED) FROM phpyun_zhaopinhui_space WHERE id IN (",
    );
    let mut first = true;
    for id in reserved_ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn list_spaces_by_ids(
    pool: &MySqlPool,
    ids: &[i32],
) -> Result<Vec<ZphSpace>, sqlx::Error> {
    let ids: Vec<i32> = ids.iter().copied().filter(|id| *id > 0).collect();
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = sqlx::QueryBuilder::new(format!(
        "SELECT {ZS_FIELDS} FROM phpyun_zhaopinhui_space WHERE id IN ("
    ));
    let mut first = true;
    for id in &ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_space_by_id(pool: &MySqlPool, id: i32) -> Result<Option<ZphSpace>, sqlx::Error> {
    let sql = format!("SELECT {ZS_FIELDS} FROM phpyun_zhaopinhui_space WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, ZphSpace>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_com_by_bid(
    pool: &MySqlPool,
    zid: u64,
    bid: i32,
) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_zhaopinhui_com WHERE zid = ? AND bid = ? LIMIT 1",
    )
    .bind(zid)
    .bind(bid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn taken_bids(pool: &MySqlPool, zid: u64) -> Result<Vec<i32>, sqlx::Error> {
    let rows: Vec<(i32,)> =
        sqlx::query_as("SELECT CAST(COALESCE(bid,0) AS SIGNED) FROM phpyun_zhaopinhui_com WHERE zid = ?")
            .bind(zid)
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().map(|(b,)| b).filter(|b| *b > 0).collect())
}
