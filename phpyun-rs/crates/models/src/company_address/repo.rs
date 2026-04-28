//! `phpyun_company_job_link` repository — company contact addresses.
//!
//! PHP schema has no `status` column — the previous soft-delete logic
//! (`AND status != 2`) hit a missing column and returned 500 on every read.
//! Switched to real DELETE since PHP behaviour is also a real DELETE.

use super::entity::CompanyAddress;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, uid, link_man, link_moblie, link_phone, email, link_address,
    provinceid, cityid, three_cityid, x, y";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyAddress>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_company_job_link \
         WHERE uid = ? \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyAddress>(&sql)
        .bind(uid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job_link WHERE uid = ?",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<Option<CompanyAddress>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_company_job_link \
         WHERE id = ? AND uid = ? LIMIT 1"
    );
    sqlx::query_as::<_, CompanyAddress>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub struct AddressFields<'a> {
    pub link_man: &'a str,
    pub link_moblie: &'a str,
    pub link_phone: &'a str,
    pub email: &'a str,
    pub link_address: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub x: &'a str,
    pub y: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    f: &AddressFields<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_job_link
           (uid, link_man, link_moblie, link_phone, email, link_address,
            provinceid, cityid, three_cityid, x, y)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(f.link_man)
    .bind(f.link_moblie)
    .bind(f.link_phone)
    .bind(f.email)
    .bind(f.link_address)
    .bind(f.provinceid)
    .bind(f.cityid)
    .bind(f.three_cityid)
    .bind(f.x)
    .bind(f.y)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    f: &AddressFields<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_job_link SET
           link_man = ?, link_moblie = ?, link_phone = ?, email = ?, link_address = ?,
           provinceid = ?, cityid = ?, three_cityid = ?, x = ?, y = ?
         WHERE id = ? AND uid = ?",
    )
    .bind(f.link_man)
    .bind(f.link_moblie)
    .bind(f.link_phone)
    .bind(f.email)
    .bind(f.link_address)
    .bind(f.provinceid)
    .bind(f.cityid)
    .bind(f.three_cityid)
    .bind(f.x)
    .bind(f.y)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_company_job_link WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}
