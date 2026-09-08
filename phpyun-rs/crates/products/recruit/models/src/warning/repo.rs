//! `phpyun_warning` repository — PHPYun's actual warning table.
//!
//! Schema (PHP truth): `id, uid, type, status, ctime, content, usertype`.
//! - `uid`      = recipient (Rust `target_uid`)
//! - `type`     = warning kind (Rust `target_kind`)
//! - `content`  = warning text (Rust `reason`)
//! - `ctime`    = unix seconds (Rust `created_at`)
//! - `status`   = 1 = unread (default), 2 = marked read by user
//! - `usertype` = recipient role (1 jobseeker / 2 employer)
//!
//! PHP has no `target_id` or `issuer_uid` columns; they are emitted as 0 in
//! response payloads to keep the Rust DTO shape stable. If they are needed
//! for product features, add them via a real ALTER TABLE migration first.

use super::entity::{PhpWarningRow, Warning};
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(uid AS UNSIGNED) AS target_uid, \
                             COALESCE(`type`, 0) AS target_kind, \
                             0 AS target_id, \
                             COALESCE(content, '') AS reason, \
                             IF(status = 2, 1, 0) AS is_read, \
                             0 AS issuer_uid, \
                             COALESCE(ctime, 0) AS created_at";

pub struct WarnCreate<'a> {
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason: &'a str,
    pub issuer_uid: u64,
}

pub async fn create(pool: &MySqlPool, c: WarnCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    // PHP `phpyun_warning` has no target_id / issuer_uid columns; we drop
    // them silently. usertype defaults to 0 (unknown); callers that care
    // should populate target_uid's row in phpyun_member separately.
    let _ = (c.target_id, c.issuer_uid);
    let res = sqlx::query(
        r#"INSERT INTO phpyun_warning
           (uid, `type`, content, status, ctime, usertype)
           VALUES (?, ?, ?, 1, ?, 0)"#,
    )
    .bind(c.target_uid)
    .bind(c.target_kind)
    .bind(c.reason)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_mine(
    pool: &MySqlPool,
    target_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Warning>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_warning
         WHERE uid = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Warning>(&sql)
        .bind(target_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_mine(pool: &MySqlPool, target_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning WHERE uid = ?")
        .bind(target_uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_unread(pool: &MySqlPool, target_uid: u64) -> Result<u64, sqlx::Error> {
    // status=1 (or NULL) means unread; status=2 means user marked it read.
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_warning WHERE uid = ? AND (status IS NULL OR status <> 2)",
    )
    .bind(target_uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn mark_read(pool: &MySqlPool, id: u64, target_uid: u64) -> Result<u64, sqlx::Error> {
    // Set status=2 to mark this row as read by the user.
    let res = sqlx::query(
        "UPDATE phpyun_warning SET status = 2 WHERE id = ? AND uid = ? AND (status IS NULL OR status <> 2)",
    )
    .bind(id)
    .bind(target_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn admin_list(
    pool: &MySqlPool,
    kind: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Warning>, sqlx::Error> {
    let sql = match kind {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_warning
             WHERE `type` = ? ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_warning
             ORDER BY ctime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Warning>(&sql);
    match kind {
        Some(k) => q.bind(k).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn admin_count(pool: &MySqlPool, kind: Option<i32>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match kind {
        Some(k) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning WHERE `type` = ?")
                .bind(k)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_warning")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

const PHP_FIELDS: &str = "CAST(w.id AS UNSIGNED) AS id, \
    CAST(COALESCE(w.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(w.`type`,0) AS SIGNED) AS warn_type, \
    CAST(COALESCE(w.status,0) AS SIGNED) AS status, \
    CAST(COALESCE(w.ctime,0) AS SIGNED) AS ctime, \
    COALESCE(w.content,'') AS content, \
    CAST(COALESCE(w.usertype,0) AS SIGNED) AS usertype, \
    COALESCE(m.username,'') AS username, \
    CASE WHEN w.usertype=1 THEN COALESCE(r.name,'') \
         WHEN w.usertype=2 THEN COALESCE(c.name,'') \
         ELSE '' END AS name_n";

pub struct PhpWarningFilter {
    pub status: Option<i32>,
    pub time_min: Option<i64>,
    pub time_max: Option<i64>,
}

fn push_php_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpWarningFilter) {
    qb.push(
        " FROM phpyun_warning w \
         LEFT JOIN phpyun_member m ON m.uid = w.uid \
         LEFT JOIN phpyun_resume r ON r.uid = w.uid \
         LEFT JOIN phpyun_company c ON c.uid = w.uid WHERE 1=1",
    );
    if let Some(s) = f.status.filter(|n| *n > 0) {
        qb.push(" AND w.status = ");
        qb.push_bind(s);
    }
    if let Some(t) = f.time_min {
        qb.push(" AND w.ctime >= ");
        qb.push_bind(t);
    }
    if let Some(t) = f.time_max {
        qb.push(" AND w.ctime <= ");
        qb.push_bind(t);
    }
}

pub async fn php_list(
    pool: &MySqlPool,
    f: &PhpWarningFilter,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpWarningRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!("SELECT {PHP_FIELDS}"));
    push_php_where(&mut qb, f);
    qb.push(" ORDER BY w.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count(pool: &MySqlPool, f: &PhpWarningFilter) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_php_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `getWarningList(..., utype=admin)` marks listed rows as status=1.
pub async fn mark_admin_seen(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_warning SET status = 1 WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_warning WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
