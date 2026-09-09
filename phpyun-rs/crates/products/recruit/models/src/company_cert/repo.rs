//! `phpyun_company_cert` — qualification (`type=3`) and claim-code (`type=6`).
//!
//! PHP schema: `id, uid, usertype, type, status, step, check, check2,
//! social_credit, owner_cert, wt_cert, other_cert, ctime, statusbody, did`.
//!
//! `uid` is not UNIQUE (one uid can have type 3 and type 6). Upsert is
//! find-by-type then UPDATE / INSERT. Never `ON DUPLICATE KEY`.

use super::entity::{CompanyCert, TYPE_LICENSE};
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(COALESCE(id, 0) AS UNSIGNED) AS id, \
                             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                             CAST(COALESCE(usertype, 0) AS SIGNED) AS usertype, \
                             CAST(COALESCE(type, 0) AS SIGNED) AS cert_type, \
                             CAST(COALESCE(status, 0) AS SIGNED) AS status, \
                             CAST(COALESCE(step, 0) AS SIGNED) AS step, \
                             COALESCE(`check`, '') AS `check`, \
                             COALESCE(check2, '') AS check2, \
                             COALESCE(social_credit, '') AS social_credit, \
                             COALESCE(owner_cert, '') AS owner_cert, \
                             COALESCE(wt_cert, '') AS wt_cert, \
                             COALESCE(other_cert, '') AS other_cert, \
                             COALESCE(ctime, 0) AS ctime, \
                             COALESCE(statusbody, '') AS statusbody, \
                             CAST(COALESCE(did, 0) AS UNSIGNED) AS did";

/// Latest type=3 (企业资质) row for this uid.
pub async fn find(pool: &MySqlPool, uid: u64) -> Result<Option<CompanyCert>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_company_cert \
         WHERE uid = ? AND type = {TYPE_LICENSE} ORDER BY id DESC LIMIT 1"
    );
    sqlx::query_as::<_, CompanyCert>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub struct CertWrite<'a> {
    pub uid: u64,
    pub did: u32,
    pub status: i32,
    pub social_credit: &'a str,
    pub check: Option<&'a str>,
    pub owner_cert: Option<&'a str>,
    pub wt_cert: Option<&'a str>,
    pub other_cert: Option<&'a str>,
    pub now: i64,
}

/// Insert or update the type=3 row. Optional pics only overwrite when `Some`.
pub async fn upsert_type3(pool: &MySqlPool, w: &CertWrite<'_>) -> Result<(), sqlx::Error> {
    if let Some(row) = find(pool, w.uid).await? {
        let mut qb = QueryBuilder::new("UPDATE phpyun_company_cert SET status = ");
        qb.push_bind(w.status);
        qb.push(", statusbody = ''");
        qb.push(", ctime = ");
        qb.push_bind(w.now);
        qb.push(", social_credit = ");
        qb.push_bind(w.social_credit);
        if let Some(v) = w.check {
            qb.push(", `check` = ");
            qb.push_bind(v);
        }
        if let Some(v) = w.owner_cert {
            qb.push(", owner_cert = ");
            qb.push_bind(v);
        }
        if let Some(v) = w.wt_cert {
            qb.push(", wt_cert = ");
            qb.push_bind(v);
        }
        if let Some(v) = w.other_cert {
            qb.push(", other_cert = ");
            qb.push_bind(v);
        }
        qb.push(" WHERE id = ");
        qb.push_bind(row.id);
        qb.build().execute(pool).await?;
    } else {
        sqlx::query(
            r#"INSERT INTO phpyun_company_cert
               (uid, usertype, type, status, step, `check`, check2, social_credit,
                owner_cert, wt_cert, other_cert, ctime, statusbody, did)
               VALUES (?, 2, '3', ?, 1, ?, '0', ?, ?, ?, ?, ?, '', ?)"#,
        )
        .bind(w.uid)
        .bind(w.status)
        .bind(w.check.unwrap_or(""))
        .bind(w.social_credit)
        .bind(w.owner_cert.unwrap_or(""))
        .bind(w.wt_cert.unwrap_or(""))
        .bind(w.other_cert.unwrap_or(""))
        .bind(w.now)
        .bind(w.did)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Admin REST: only rows still waiting (`status=0`).
pub async fn review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    note: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_cert
           SET status = ?, statusbody = ?
           WHERE uid = ? AND type = 3 AND status = 0"#,
    )
    .bind(status)
    .bind(note)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_pending(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyCert>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_company_cert \
         WHERE type = {TYPE_LICENSE} AND status = 0 \
         ORDER BY ctime ASC, id ASC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CompanyCert>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn find_type3_note(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(statusbody,'') FROM phpyun_company_cert WHERE uid = ? AND type = 3 LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).unwrap_or_default())
}

pub async fn update_admin_type3(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    note: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_cert SET status = ?, statusbody = ? WHERE uid = ? AND type = 3",
    )
    .bind(status)
    .bind(note)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn count_pending(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_cert WHERE type = 3 AND status = 0",
    )
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP claim code: `company_cert.check2` where `type=6`.
pub async fn find_claim_code(pool: &MySqlPool, uid: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT check2 FROM phpyun_company_cert WHERE uid = ? AND type = 6 LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(v,)| v.filter(|s| !s.trim().is_empty())))
}
