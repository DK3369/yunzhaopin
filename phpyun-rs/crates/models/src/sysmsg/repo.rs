use super::entity::SysMsg;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, fa_uid, usertype, content, remind_status, ctime";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    usertype: i32,
    unread_only: bool,
    offset: u64,
    limit: u64,
) -> Result<Vec<SysMsg>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_sysmsg WHERE fa_uid = ");
    qb.push_bind(uid);
    qb.push(" AND usertype = ");
    qb.push_bind(usertype);
    if unread_only {
        qb.push(" AND remind_status = 1");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as::<SysMsg>().fetch_all(pool).await
}

pub async fn count_by_uid(
    pool: &MySqlPool,
    uid: u64,
    usertype: i32,
    unread_only: bool,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_sysmsg WHERE fa_uid = ");
    qb.push_bind(uid);
    qb.push(" AND usertype = ");
    qb.push_bind(usertype);
    if unread_only {
        qb.push(" AND remind_status = 1");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn mark_read(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_sysmsg SET remind_status = 0 WHERE id = ? AND fa_uid = ? AND remind_status = 1",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn mark_all_read(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_sysmsg SET remind_status = 0 WHERE fa_uid = ? AND remind_status = 1",
    )
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
        QueryBuilder::new("DELETE FROM phpyun_sysmsg WHERE fa_uid = ");
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

pub async fn insert(
    pool: &MySqlPool,
    fa_uid: u64,
    usertype: i32,
    content: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_sysmsg (fa_uid, usertype, content, remind_status, ctime) VALUES (?, ?, ?, 1, ?)",
    )
    .bind(fa_uid)
    .bind(usertype)
    .bind(content)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}
