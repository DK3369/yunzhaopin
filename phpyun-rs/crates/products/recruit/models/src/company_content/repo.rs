use super::entity::{CompanyContent, ContentKind};
use sqlx::{MySqlPool, QueryBuilder};

/// Live `phpyun_company_news` has no `file` / `usertype`; products store the
/// image in `pic`. SELECT aliases keep the entity's `file` / `usertype` fields.
fn select_fields(kind: ContentKind) -> &'static str {
    match kind {
        ContentKind::News => {
            "CAST(id AS UNSIGNED) AS id, \
             CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
             COALESCE(title,'') AS title, \
             body, \
             CAST('' AS CHAR) AS file, \
             CAST(COALESCE(status,0) AS SIGNED) AS status, \
             statusbody, \
             CAST(COALESCE(ctime,0) AS SIGNED) AS ctime, \
             CAST(COALESCE(did,0) AS UNSIGNED) AS did, \
             CAST(2 AS SIGNED) AS usertype"
        }
        ContentKind::Product => {
            "CAST(id AS UNSIGNED) AS id, \
             CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
             COALESCE(title,'') AS title, \
             body, \
             pic AS file, \
             CAST(COALESCE(status,0) AS SIGNED) AS status, \
             statusbody, \
             CAST(COALESCE(ctime,0) AS SIGNED) AS ctime, \
             CAST(COALESCE(did,0) AS UNSIGNED) AS did, \
             CAST(2 AS SIGNED) AS usertype"
        }
    }
}

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
    qb.push(select_fields(kind));
    qb.push(" FROM ");
    qb.push(kind.table());
    qb.push(" WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND status != 2");
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            qb.push(" AND title LIKE ");
            crate::sql::push_contains(&mut qb, kw);
        }
    }
    qb.push(" ORDER BY ctime DESC LIMIT ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(
        limit,
        "pagination.limit",
    )?);
    qb.push(" OFFSET ");
    qb.push_bind(phpyun_core::numeric::checked_db_i64(
        offset,
        "pagination.offset",
    )?);
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
            crate::sql::push_contains(&mut qb, kw);
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_by_id(
    pool: &MySqlPool,
    kind: ContentKind,
    id: u64,
    uid: u64,
) -> Result<Option<CompanyContent>, sqlx::Error> {
    let fields = select_fields(kind);
    let sql = format!(
        "SELECT {fields} FROM {} WHERE id = ? AND uid = ? AND status != 2 LIMIT 1",
        kind.table()
    );
    sqlx::query_as::<_, CompanyContent>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub struct CreateInput<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub body: &'a str,
    pub file: Option<&'a str>,
    pub usertype: i32,
    pub did: u32,
    pub now: i64,
}

pub async fn create(
    pool: &MySqlPool,
    kind: ContentKind,
    input: CreateInput<'_>,
) -> Result<u64, sqlx::Error> {
    let _ = input.usertype;
    let res = match kind {
        ContentKind::News => {
            sqlx::query(
                "INSERT INTO phpyun_company_news (uid, title, body, status, ctime, did)
                 VALUES (?, ?, ?, 0, ?, ?)",
            )
            .bind(input.uid)
            .bind(input.title)
            .bind(input.body)
            .bind(input.now)
            .bind(input.did)
            .execute(pool)
            .await?
        }
        ContentKind::Product => {
            sqlx::query(
                "INSERT INTO phpyun_company_product (uid, title, body, pic, status, ctime, did)
                 VALUES (?, ?, ?, ?, 0, ?, ?)",
            )
            .bind(input.uid)
            .bind(input.title)
            .bind(input.body)
            .bind(input.file.unwrap_or(""))
            .bind(input.now)
            .bind(input.did)
            .execute(pool)
            .await?
        }
    };
    Ok(res.last_insert_id())
}

pub struct UpdateInput<'a> {
    pub id: u64,
    pub uid: u64,
    pub title: &'a str,
    pub body: &'a str,
    pub file: Option<&'a str>,
    pub now: i64,
}

pub async fn update(
    pool: &MySqlPool,
    kind: ContentKind,
    input: UpdateInput<'_>,
) -> Result<u64, sqlx::Error> {
    // After update, reset status = 0 to re-submit for review (matching PHP behavior).
    let res = match (kind, input.file) {
        (ContentKind::Product, Some(pic)) => {
            sqlx::query(
                "UPDATE phpyun_company_product
                 SET title = ?, body = ?, pic = ?, status = 0, ctime = ?
                 WHERE id = ? AND uid = ?",
            )
            .bind(input.title)
            .bind(input.body)
            .bind(pic)
            .bind(input.now)
            .bind(input.id)
            .bind(input.uid)
            .execute(pool)
            .await?
        }
        (ContentKind::Product, None) => {
            sqlx::query(
                "UPDATE phpyun_company_product
                 SET title = ?, body = ?, status = 0, ctime = ?
                 WHERE id = ? AND uid = ?",
            )
            .bind(input.title)
            .bind(input.body)
            .bind(input.now)
            .bind(input.id)
            .bind(input.uid)
            .execute(pool)
            .await?
        }
        (ContentKind::News, _) => {
            sqlx::query(
                "UPDATE phpyun_company_news
                 SET title = ?, body = ?, status = 0, ctime = ?
                 WHERE id = ? AND uid = ?",
            )
            .bind(input.title)
            .bind(input.body)
            .bind(input.now)
            .bind(input.id)
            .bind(input.uid)
            .execute(pool)
            .await?
        }
    };
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
