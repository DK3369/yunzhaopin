use super::entity::CompanyTpl;
use sqlx::MySqlPool;

// Actual PHPYun phpyun_company_tpl columns:
// id/name/url/pic/type/price(varchar)/status/service_uid
// No `sort` column -- order by `id DESC` by default.
// `price` is varchar so requires CAST; `id` is INT so requires
// CAST UNSIGNED to satisfy sqlx u64 decoding.
// `phpyun_company_tpl.name` and `.url` are nullable varchar; entity uses
// plain String. COALESCE empty so a NULL row can't trip sqlx.
const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(url, '') AS url, \
    pic, \
    CAST(COALESCE(`type`, 0) AS SIGNED) AS `type`, \
    CAST(COALESCE(NULLIF(price, ''), '0') AS SIGNED) AS price, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(0 AS SIGNED) AS sort";

pub async fn list_public(pool: &MySqlPool) -> Result<Vec<CompanyTpl>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company_tpl WHERE status = 1 ORDER BY id DESC");
    sqlx::query_as::<_, CompanyTpl>(&sql).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<CompanyTpl>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company_tpl WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, CompanyTpl>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

// ==================== Apply state: company_statis ====================
//
// PHP `statis.model` picks the table from `usertype`: 1 -> `member_statis`,
// 2 -> `company_statis`, and keys on `uid` alone. Company skins are usertype 2,
// so they live in `phpyun_company_statis`, which is the only one of the two
// tables that has `comtpl` / `comtpl_all` at all — `phpyun_member_statis` has
// neither, nor a `usertype` column.

/// Read the company's currently purchased templates
/// (`company_statis.comtpl_all`, a comma-separated url list).
pub async fn fetch_purchased_urls(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT comtpl_all FROM phpyun_company_statis WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.and_then(|(s,)| s))
}

pub async fn fetch_applied_tpl(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(comtpl, '') FROM phpyun_company_statis WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).unwrap_or_default())
}

pub async fn set_applied_tpl(pool: &MySqlPool, uid: u64, url: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_statis SET comtpl = ? WHERE uid = ?")
        .bind(url)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Append a purchased url to comtpl_all (de-duplicated).
/// If the uid has no company_statis row, upsert one.
pub async fn append_purchased_url(
    pool: &MySqlPool,
    uid: u64,
    url: &str,
) -> Result<(), sqlx::Error> {
    let current = fetch_purchased_urls(pool, uid).await?;
    let new_value = match current {
        Some(s) if !s.is_empty() => {
            let already = s.split(',').map(|x| x.trim()).any(|x| x == url);
            if already {
                return Ok(());
            }
            format!("{s},{url}")
        }
        _ => url.to_string(),
    };
    let affected = sqlx::query("UPDATE phpyun_company_statis SET comtpl_all = ? WHERE uid = ?")
        .bind(&new_value)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected();

    // Upsert if no row exists. `sq_job` / `fav_job` / `all_pay` / `consum_pay`
    // are NOT NULL with no default, so they have to be named explicitly under
    // `STRICT_TRANS_TABLES`.
    if affected == 0 {
        let _ = sqlx::query(
            "INSERT INTO phpyun_company_statis \
             (uid, comtpl_all, sq_job, fav_job, all_pay, consum_pay) \
             VALUES (?, ?, 0, 0, 0, 0)",
        )
        .bind(uid)
        .bind(&new_value)
        .execute(pool)
        .await?;
    }
    Ok(())
}
