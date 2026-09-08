//! `phpyun_recycle` with PHP's own column names, for the console recycle-bin page.
//!
//! PHP fills this table as a side effect of `delete_all`: every row about to go
//! away is `serialize()`d into `body` together with the table it came from, the
//! admin who triggered it, and an `ident` md5 shared by one operation. Restoring
//! is `INSERT` of that body back into `tablename`.
//!
//! Restore is the only place in the codebase that writes to a table chosen at
//! runtime, so both the table and every column name are checked against
//! `information_schema` before they reach a statement — a tampered `body` can
//! only ever produce a rejected restore, never injected SQL.

use super::entity::PhpRecycleRow;
use crate::php_ser::{self, Scalar};
use sqlx::{MySqlPool, QueryBuilder, Row};

const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                      CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
                      COALESCE(username, '') AS username, \
                      COALESCE(tablename, '') AS tablename, \
                      COALESCE(body, '') AS body, \
                      CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
                      COALESCE(ident, '') AS ident, \
                      COALESCE(uri, '') AS uri";

/// PHP prefixes every table with `phpyun_`; the bare name is what `tablename` holds.
const TABLE_PREFIX: &str = "phpyun_";

/// PHP `dataRecycle::index_action` filters. `keyword` matches the serialized
/// body, which is how the page searches inside deleted rows.
#[derive(Debug, Default, Clone)]
pub struct PhpFilter {
    pub username: Option<String>,
    pub keyword: Option<String>,
    pub table: Option<String>,
    pub ident: Option<String>,
    pub ctime_from: Option<i64>,
    pub ctime_to: Option<i64>,
}

fn push_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpFilter) {
    qb.push(" WHERE 1=1");
    for (col, val) in [
        ("username", &f.username),
        ("body", &f.keyword),
        ("tablename", &f.table),
    ] {
        if let Some(v) = val.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            qb.push(format!(" AND {col} LIKE "));
            qb.push_bind(format!("%{v}%"));
        }
    }
    if let Some(v) = f.ident.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND ident = ");
        qb.push_bind(v.to_owned());
    }
    if let Some(v) = f.ctime_from {
        qb.push(" AND ctime >= ");
        qb.push_bind(v);
    }
    if let Some(v) = f.ctime_to {
        qb.push(" AND ctime <= ");
        qb.push_bind(v);
    }
}

pub async fn count(pool: &MySqlPool, f: &PhpFilter) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_recycle");
    push_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Newest first: PHP's `orderby => 'id'` has no direction segment, and its
/// builder falls through to `DESC` whenever one is missing.
pub async fn list(
    pool: &MySqlPool,
    f: &PhpFilter,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpRecycleRow>, sqlx::Error> {
    let l = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let o = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {FIELDS} FROM phpyun_recycle"));
    push_where(&mut qb, f);
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<PhpRecycleRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {FIELDS} FROM phpyun_recycle WHERE id IN ("));
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") ORDER BY id");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_by_ident(
    pool: &MySqlPool,
    ident: &str,
) -> Result<Vec<PhpRecycleRow>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_recycle WHERE ident = ? ORDER BY id");
    sqlx::query_as(&sql).bind(ident).fetch_all(pool).await
}

pub async fn delete_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_recycle WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// `TRUNCATE`, matching PHP's `tuncateRecycle_action`. Resets the auto-increment
/// too, which a `DELETE` would not.
pub async fn truncate(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("TRUNCATE TABLE phpyun_recycle")
        .execute(pool)
        .await?;
    Ok(())
}

/// Columns of `phpyun_<bare>` in the connected schema, or an empty list when
/// there is no such table. Also the table-exists check: no columns, no restore.
///
/// The bare name is validated the way PHP's `checkTableName` does (letters and
/// underscores, 2..=30) before it is used, even though it is only ever bound as
/// a parameter here.
pub async fn table_columns(pool: &MySqlPool, bare: &str) -> Result<Vec<String>, sqlx::Error> {
    if !is_php_table_name(bare) {
        return Ok(Vec::new());
    }
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT COLUMN_NAME FROM information_schema.COLUMNS \
          WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?",
    )
    .bind(format!("{TABLE_PREFIX}{bare}"))
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(c,)| c).collect())
}

/// PHP `action::checkTableName`: `/^[_a-z]{2,30}$/i`. Digits are not allowed,
/// which is why no PHPYun table has one in its name.
pub fn is_php_table_name(bare: &str) -> bool {
    (2..=30).contains(&bare.len())
        && bare
            .bytes()
            .all(|c| c == b'_' || c.is_ascii_alphabetic())
}

/// Writes one snapshot back into its source table.
///
/// `cols` must already be filtered to names present in the table; anything else
/// is dropped by the caller so a stale snapshot taken before a schema change
/// still restores its surviving columns, exactly like PHP's `insert_into`.
pub async fn insert_row(
    pool: &MySqlPool,
    bare: &str,
    cols: &[(String, Scalar)],
) -> Result<u64, sqlx::Error> {
    let known = table_columns(pool, bare).await?;
    if known.is_empty() {
        return Ok(0);
    }
    let usable: Vec<&(String, Scalar)> = cols
        .iter()
        .filter(|(name, _)| known.iter().any(|k| k == name))
        .collect();
    if usable.is_empty() {
        return Ok(0);
    }
    let mut sql = format!("INSERT INTO `{TABLE_PREFIX}{bare}` (");
    for (i, (name, _)) in usable.iter().enumerate() {
        if i > 0 {
            sql.push_str(", ");
        }
        sql.push('`');
        sql.push_str(name);
        sql.push('`');
    }
    sql.push_str(") VALUES (");
    sql.push_str(&vec!["?"; usable.len()].join(", "));
    sql.push(')');

    let mut q = sqlx::query(&sql);
    for (_, val) in &usable {
        q = q.bind(val.clone());
    }
    Ok(q.execute(pool).await?.rows_affected())
}

/// `phpyun_admin_user.username` — what PHP puts in `$_SESSION['ausername']` and
/// therefore what the recycle grid's operator column and search match against.
pub async fn admin_username(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(username, '') FROM phpyun_admin_user WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).unwrap_or_default())
}

/// Snapshot rows into the bin before they are deleted — PHP `insert_recycle`.
///
/// Returns how many snapshots were stored. A table PHP would refuse, or one
/// this schema does not have, stores nothing and reports 0; the caller deletes
/// either way, as PHP does (it ignores the result too).
///
/// Every column is read back as `CHAR` because PHP snapshots a
/// `mysql_fetch_assoc` array, where an `INT` column is already the string
/// `"12"`. Restoring then hands those strings to MySQL, which coerces them
/// back — so a round trip through the bin does not change a row.
pub async fn archive(
    pool: &MySqlPool,
    bare: &str,
    ids: &[u64],
    actor_uid: u64,
    actor_username: &str,
    ident: &str,
    uri: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let cols = table_columns(pool, bare).await?;
    if cols.is_empty() {
        return Ok(0);
    }

    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    let mut sep = qb.separated(", ");
    for c in &cols {
        sep.push(format!("CAST(`{c}` AS CHAR) AS `{c}`"));
    }
    qb.push(format!(" FROM `{TABLE_PREFIX}{bare}` WHERE id IN ("));
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows = qb.build().fetch_all(pool).await?;
    if rows.is_empty() {
        return Ok(0);
    }

    let now = phpyun_core::clock::now_ts();
    let mut ins: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "INSERT INTO phpyun_recycle (tablename, ctime, uid, username, uri, body, ident) ",
    );
    ins.push_values(&rows, |mut b, row| {
        let body: Vec<(String, Scalar)> = cols
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), cell(row, i)))
            .collect();
        b.push_bind(bare.to_owned())
            .push_bind(now)
            .push_bind(actor_uid)
            .push_bind(actor_username.to_owned())
            .push_bind(uri.to_owned())
            .push_bind(php_ser::serialize_row(&body))
            .push_bind(ident.to_owned());
    });
    Ok(ins.build().execute(pool).await?.rows_affected())
}

/// One `CAST(… AS CHAR)` cell. A column MySQL still hands back as bytes (a
/// `BLOB` cast stays binary) is taken lossily rather than dropped, so the
/// snapshot keeps something a human can read in the detail drawer.
fn cell(row: &sqlx::mysql::MySqlRow, idx: usize) -> Scalar {
    if let Ok(v) = row.try_get::<Option<String>, _>(idx) {
        return v;
    }
    row.try_get::<Option<Vec<u8>>, _>(idx)
        .ok()
        .flatten()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
}

#[cfg(test)]
mod tests {
    use super::is_php_table_name;

    #[test]
    fn accepts_real_table_names_and_rejects_injection() {
        assert!(is_php_table_name("down_resume"));
        assert!(is_php_table_name("recycle"));
        assert!(!is_php_table_name("a"));
        assert!(!is_php_table_name("resume2"));
        assert!(!is_php_table_name("resume`; DROP TABLE x"));
        assert!(!is_php_table_name(""));
        assert!(!is_php_table_name(&"a".repeat(31)));
    }
}
