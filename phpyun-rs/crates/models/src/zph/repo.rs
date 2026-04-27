//! Strictly aligned with PHPYun's job-fair tables:
//!
//! - `phpyun_zhaopinhui`      — the job-fair record itself
//! - `phpyun_zhaopinhui_com`  — participating companies (includes a jobid field; locally acts as the "reservation" concept)
//! - `phpyun_zhaopinhui_space`— booth/stall definitions (id/name/sort/keyid/pic/content/price); this is **not**
//!                              the "company reservation" table — on the Rust side, ZphReservation has been
//!                              repurposed to map to phpyun_zhaopinhui_com.
//!
//! Zph field mapping (Rust → PHP):
//!   - body       ↔ body (same name in PHP)
//!   - banner     ↔ banner (same name in PHP)
//!   - city_id    ↔ cityid
//!   - start_at   ↔ UNIX_TIMESTAMP(starttime) (PHP stores a varchar date)
//!   - end_at     ↔ UNIX_TIMESTAMP(endtime)
//!   - status     ↔ is_open (1=open / 0=closed; PHP's `status` column is a workflow state, while is_open is the listing flag)
//!   - created_at ↔ ctime

use super::entity::{Zph, ZphCompany, ZphReservation};
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

pub async fn list(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Zph>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZPH_FIELDS} FROM phpyun_zhaopinhui \
         WHERE is_open = 1 ORDER BY UNIX_TIMESTAMP(starttime) DESC, id DESC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Zph>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_zhaopinhui WHERE is_open = 1")
        .fetch_one(pool)
        .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Zph>, sqlx::Error> {
    let sql = format!("SELECT {ZPH_FIELDS} FROM phpyun_zhaopinhui WHERE id = ?");
    sqlx::query_as::<_, Zph>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

// ---------- companies ----------

const ZC_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(zid, 0) AS UNSIGNED) AS zid, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

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
pub async fn jobid_csvs_for_zph(
    pool: &MySqlPool,
    zid: u64,
) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT COALESCE(jobid, '') FROM phpyun_zhaopinhui_com \
           WHERE zid = ? AND status = 1",
    )
    .bind(zid)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(s,)| s).filter(|s| !s.is_empty()).collect())
}

pub async fn count_companies(pool: &MySqlPool, zid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_zhaopinhui_com WHERE zid = ? AND status = 1",
    )
    .bind(zid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
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
    pub mobile: &'a str,
}

pub async fn upsert_reservation(
    pool: &MySqlPool,
    r: ReservationCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = r.mobile; // phpyun_zhaopinhui_com has no mobile column; ignore
    // Try UPDATE first; if 0 rows, INSERT (phpyun_zhaopinhui_com typically has no (zid,uid) unique key)
    let updated = sqlx::query(
        "UPDATE phpyun_zhaopinhui_com SET jobid = ?, com_name = ?, ctime = ? \
         WHERE zid = ? AND uid = ?",
    )
    .bind(r.job_ids)
    .bind(r.name)
    .bind(now)
    .bind(r.zid)
    .bind(r.uid)
    .execute(pool)
    .await?;
    if updated.rows_affected() == 0 {
        let res = sqlx::query(
            "INSERT INTO phpyun_zhaopinhui_com (zid, uid, jobid, com_name, ctime, status) \
             VALUES (?, ?, ?, ?, ?, 0)",
        )
        .bind(r.zid)
        .bind(r.uid)
        .bind(r.job_ids)
        .bind(r.name)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    } else {
        let row: (i64,) = sqlx::query_as(
            "SELECT id FROM phpyun_zhaopinhui_com WHERE zid = ? AND uid = ? LIMIT 1",
        )
        .bind(r.zid)
        .bind(r.uid)
        .fetch_one(pool)
        .await?;
        Ok(row.0 as u64)
    }
}

pub async fn find_my_reservation(
    pool: &MySqlPool,
    zid: u64,
    uid: u64,
) -> Result<Option<ZphReservation>, sqlx::Error> {
    let sql = format!(
        "SELECT {ZR_FIELDS} FROM phpyun_zhaopinhui_com WHERE zid = ? AND uid = ? LIMIT 1"
    );
    sqlx::query_as::<_, ZphReservation>(&sql)
        .bind(zid)
        .bind(uid)
        .fetch_optional(pool)
        .await
}
