use super::entity::SearchHistory;
use sqlx::MySqlPool;

const FIELDS: &str = "id, uid, scope, keyword, created_at";

pub async fn insert(
    pool: &MySqlPool,
    uid: u64,
    scope: &str,
    keyword: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_keyword_log (uid, scope, keyword, created_at)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(scope)
    .bind(keyword)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list(
    pool: &MySqlPool,
    uid: u64,
    scope: Option<&str>,
    limit: u64,
) -> Result<Vec<SearchHistory>, sqlx::Error> {
    let sql = match scope {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_keyword_log
             WHERE uid = ? AND scope = ?
             ORDER BY created_at DESC LIMIT ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_keyword_log
             WHERE uid = ?
             ORDER BY created_at DESC LIMIT ?"
        ),
    };
    let q = sqlx::query_as::<_, SearchHistory>(&sql);
    match scope {
        Some(s) => q.bind(uid).bind(s).bind(limit).fetch_all(pool).await,
        None => q.bind(uid).bind(limit).fetch_all(pool).await,
    }
}

pub async fn delete_one(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_keyword_log WHERE id = ? AND uid = ?",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn clear(
    pool: &MySqlPool,
    uid: u64,
    scope: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let res = match scope {
        Some(s) => {
            sqlx::query("DELETE FROM phpyun_keyword_log WHERE uid = ? AND scope = ?")
                .bind(uid)
                .bind(s)
                .execute(pool)
                .await?
        }
        None => {
            sqlx::query("DELETE FROM phpyun_keyword_log WHERE uid = ?")
                .bind(uid)
                .execute(pool)
                .await?
        }
    };
    Ok(res.rows_affected())
}

/// When there are more than `keep` entries, delete the older ones until only `keep` remain.
pub async fn trim(
    pool: &MySqlPool,
    uid: u64,
    scope: &str,
    keep: u64,
) -> Result<u64, sqlx::Error> {
    // MySQL does not support DELETE with LIMIT+OFFSET directly; use a subquery instead.
    let res = sqlx::query(
        r#"DELETE FROM phpyun_keyword_log
           WHERE uid = ? AND scope = ? AND id NOT IN (
               SELECT * FROM (
                   SELECT id FROM phpyun_keyword_log
                   WHERE uid = ? AND scope = ?
                   ORDER BY created_at DESC LIMIT ?
               ) AS keep_ids
           )"#,
    )
    .bind(uid)
    .bind(scope)
    .bind(uid)
    .bind(scope)
    .bind(keep)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
