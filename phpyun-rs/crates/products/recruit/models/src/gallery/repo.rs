use super::entity::{GalleryItem, GalleryKind};
use sqlx::{MySqlPool, QueryBuilder};

// Soft-delete convention: status=2 means deleted. Both tables
// phpyun_company_show / phpyun_resume_show have a status column.

const FIELDS: &str = "id, uid, title, picurl, sort";

pub async fn list_by_uid(
    pool: &MySqlPool,
    kind: GalleryKind,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<GalleryItem>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM {} \
         WHERE uid = ? AND status != 2 \
         ORDER BY sort DESC, id DESC LIMIT ? OFFSET ?",
        kind.table()
    );
    sqlx::query_as::<_, GalleryItem>(&sql)
        .bind(uid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(
    pool: &MySqlPool,
    kind: GalleryKind,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let sql = format!(
        "SELECT COUNT(*) FROM {} WHERE uid = ? AND status != 2",
        kind.table()
    );
    let (n,): (i64,) = sqlx::query_as(&sql).bind(uid).fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(
    pool: &MySqlPool,
    kind: GalleryKind,
    id: u64,
    uid: u64,
) -> Result<Option<GalleryItem>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM {} WHERE id = ? AND uid = ? AND status != 2 LIMIT 1",
        kind.table()
    );
    sqlx::query_as::<_, GalleryItem>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    kind: GalleryKind,
    uid: u64,
    title: &str,
    picurl: &str,
    sort: i32,
) -> Result<u64, sqlx::Error> {
    // status column defaults to 1 (active).
    let sql = format!(
        "INSERT INTO {} (uid, title, picurl, sort) VALUES (?, ?, ?, ?)",
        kind.table()
    );
    let res = sqlx::query(&sql)
        .bind(uid)
        .bind(title)
        .bind(picurl)
        .bind(sort)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    kind: GalleryKind,
    id: u64,
    uid: u64,
    title: Option<&str>,
    picurl: Option<&str>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE ");
    qb.push(kind.table());
    qb.push(" SET ");
    let mut any = false;
    if let Some(t) = title {
        qb.push("title = ");
        qb.push_bind(t);
        any = true;
    }
    if let Some(p) = picurl {
        if any {
            qb.push(", ");
        }
        qb.push("picurl = ");
        qb.push_bind(p);
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
    kind: GalleryKind,
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
