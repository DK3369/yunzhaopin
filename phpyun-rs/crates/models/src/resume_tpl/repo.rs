use super::entity::ResumeTpl;
use sqlx::MySqlPool;

// PHPYun phpyun_resumetpl actual columns: id/name/url/pic/type/price(varchar)/status/service_uid
// Rust ResumeTpl.sort has no corresponding column; use `type` as the sort weight and SELECT it aliased as sort.
const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    name, \
    pic, \
    CAST(COALESCE(NULLIF(price, ''), '0') AS SIGNED) AS price, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(`type`, 0) AS SIGNED) AS sort";

pub async fn list_public(pool: &MySqlPool) -> Result<Vec<ResumeTpl>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resumetpl WHERE status = 1 ORDER BY id DESC"
    );
    sqlx::query_as::<_, ResumeTpl>(&sql).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<ResumeTpl>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resumetpl WHERE id = ? LIMIT 1"
    );
    sqlx::query_as::<_, ResumeTpl>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Read the list of template ids purchased by the jobseeker (comma-separated in `member_statis.paytpls`).
pub async fn fetch_purchased_ids(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT paytpls FROM phpyun_member_statis WHERE uid = ? AND usertype = 1 LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(s,)| s))
}

/// Append a purchased template id; upsert if the member_statis row does not exist.
pub async fn append_purchased_id(
    pool: &MySqlPool,
    uid: u64,
    tpl_id: u64,
) -> Result<(), sqlx::Error> {
    let current = fetch_purchased_ids(pool, uid).await?;
    let new_value = match current {
        Some(s) if !s.is_empty() => {
            if s.split(',').any(|x| x.trim() == tpl_id.to_string()) {
                return Ok(());
            }
            format!("{s},{tpl_id}")
        }
        _ => tpl_id.to_string(),
    };
    let affected = sqlx::query(
        "UPDATE phpyun_member_statis SET paytpls = ? WHERE uid = ? AND usertype = 1",
    )
    .bind(&new_value)
    .bind(uid)
    .execute(pool)
    .await?
    .rows_affected();
    if affected == 0 {
        let _ = sqlx::query(
            "INSERT INTO phpyun_member_statis (uid, usertype, paytpls) VALUES (?, 1, ?)",
        )
        .bind(uid)
        .bind(&new_value)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn set_applied_tpl(
    pool: &MySqlPool,
    uid: u64,
    tpl_id: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_member_statis SET tpl = ? WHERE uid = ? AND usertype = 1",
    )
    .bind(tpl_id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
