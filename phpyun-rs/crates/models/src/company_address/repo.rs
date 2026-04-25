use super::entity::CompanyAddress;
use sqlx::{MySqlPool, QueryBuilder};

// Soft-delete convention: status=2 means deleted. All queries include
// `AND status != 2`.

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
         WHERE uid = ? AND status != 2 \
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
        "SELECT COUNT(*) FROM phpyun_company_job_link WHERE uid = ? AND status != 2",
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
         WHERE id = ? AND uid = ? AND status != 2 LIMIT 1"
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
            provinceid, cityid, three_cityid, x, y, status)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
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
         WHERE id = ? AND uid = ? AND status != 2",
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

/// Soft delete: bulk UPDATE status=2.
pub async fn delete_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_company_job_link SET status = 2 WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}
