use super::entity::{CompanyContent, ContentKind};
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str = "id, uid, title, body, file, status, statusbody, ctime, did, usertype";

// Soft-delete convention: status=2 means deleted. list/count/find always
// filter with `AND status != 2`.
// Note: `update()` itself resets status to 0 to re-submit for review,
// which is the existing business semantics; not in conflict with soft delete.

pub async fn list(
    pool: &MySqlPool,
    kind: ContentKind,
    uid: u64,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyContent>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM ");
    qb.push(kind.table()); // enum-whitelisted, not injectable
    qb.push(" WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2");
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            qb.push(" AND title LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    qb.push(" ORDER BY ctime DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as::<CompanyContent>().fetch_all(pool).await
}

pub async fn count(
    pool: &MySqlPool,
    kind: ContentKind,
    uid: u64,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*) FROM ");
    qb.push(kind.table());
    qb.push(" WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2");
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            qb.push(" AND title LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(
    pool: &MySqlPool,
    kind: ContentKind,
    id: u64,
    uid: u64,
) -> Result<Option<CompanyContent>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM {} WHERE id = ? AND uid = ? AND status != 2 LIMIT 1",
        kind.table()
    );
    sqlx::query_as::<_, CompanyContent>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    kind: ContentKind,
    uid: u64,
    title: &str,
    body: &str,
    file: Option<&str>,
    usertype: i32,
    did: u32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let sql = format!(
        "INSERT INTO {} (uid, title, body, file, status, ctime, did, usertype)
         VALUES (?, ?, ?, ?, 0, ?, ?, ?)",
        kind.table()
    );
    let res = sqlx::query(&sql)
        .bind(uid)
        .bind(title)
        .bind(body)
        .bind(file.unwrap_or(""))
        .bind(now)
        .bind(did)
        .bind(usertype)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    kind: ContentKind,
    id: u64,
    uid: u64,
    title: &str,
    body: &str,
    file: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // After update, reset status = 0 to re-submit for review (matching PHP behavior).
    let sql = if file.is_some() {
        format!(
            "UPDATE {} SET title = ?, body = ?, file = ?, status = 0, ctime = ? WHERE id = ? AND uid = ?",
            kind.table()
        )
    } else {
        format!(
            "UPDATE {} SET title = ?, body = ?, status = 0, ctime = ? WHERE id = ? AND uid = ?",
            kind.table()
        )
    };

    let q = sqlx::query(&sql).bind(title).bind(body);
    let q = if let Some(f) = file {
        q.bind(f)
    } else {
        q
    };
    let res = q.bind(now).bind(id).bind(uid).execute(pool).await?;
    Ok(res.rows_affected())
}

/// Soft delete: bulk UPDATE status=2.
pub async fn delete_by_ids(
    pool: &MySqlPool,
    kind: ContentKind,
    ids: &[u64],
    uid: u64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE ");
    qb.push(kind.table());
    qb.push(" SET status = 2 WHERE uid = ");
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
