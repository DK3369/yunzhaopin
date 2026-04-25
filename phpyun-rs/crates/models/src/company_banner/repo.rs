use super::entity::CompanyBanner;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, uid, pic, link, sort, addtime";

// Soft-delete convention: status=2 means deleted; the table already has a
// status column (default 1).

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<CompanyBanner>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_banner \
         WHERE uid = ? AND status != 2 \
         ORDER BY sort DESC, id DESC"
    );
    sqlx::query_as::<_, CompanyBanner>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_banner WHERE uid = ? AND status != 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    pic: &str,
    link: Option<&str>,
    sort: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_banner (uid, pic, link, sort, addtime) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(pic)
    .bind(link.unwrap_or(""))
    .bind(sort)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    pic: Option<&str>,
    link: Option<&str>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_banner SET ");
    let mut any = false;
    if let Some(p) = pic {
        qb.push("pic = ");
        qb.push_bind(p);
        any = true;
    }
    if let Some(l) = link {
        if any {
            qb.push(", ");
        }
        qb.push("link = ");
        qb.push_bind(l);
        any = true;
    }
    if let Some(s) = sort {
        if any {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(s);
        any = true;
    }
    if !any {
        return Ok(0);
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2");
    let res = qb.build().execute(pool).await?;
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
        QueryBuilder::new("UPDATE phpyun_banner SET status = 2 WHERE uid = ");
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
