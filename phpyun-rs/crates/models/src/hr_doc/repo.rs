//! HR toolbox documents -- corresponds to PHPYun table `phpyun_toolbox_doc`.
//!
//! Column mapping (Rust HrDoc field <- PHP column):
//!   - `hits`       <- `downnum` (PHPYun uses "download count" as a popularity proxy)
//!   - `body`       = '' (this PHPYun table has no body column; the toolbox
//!     usually just redirects via url)
//!   - `created_at` <- `add_time` (UNIX seconds)
//!   - `updated_at` <- `add_time` (PHPYun doesn't maintain updated time;
//!     falls back to publish time)

use super::entity::HrDoc;
use sqlx::MySqlPool;

const FIELDS: &str = "id, \
    COALESCE(cid, 0) AS cid, \
    COALESCE(name, '') AS name, \
    COALESCE(url, '') AS url, \
    '' AS body, \
    COALESCE(downnum, 0) AS hits, \
    COALESCE(is_show, 0) AS is_show, \
    COALESCE(add_time, 0) AS created_at, \
    COALESCE(add_time, 0) AS updated_at";

pub async fn list_public(
    pool: &MySqlPool,
    cid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<HrDoc>, sqlx::Error> {
    let sql = match cid {
        Some(_) => format!(
            "SELECT {FIELDS} FROM phpyun_toolbox_doc \
             WHERE is_show = 1 AND cid = ? \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {FIELDS} FROM phpyun_toolbox_doc \
             WHERE is_show = 1 \
             ORDER BY id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, HrDoc>(&sql);
    match cid {
        Some(c) => q.bind(c).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(limit).bind(offset).fetch_all(pool).await,
    }
}

pub async fn count_public(
    pool: &MySqlPool,
    cid: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match cid {
        Some(c) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_toolbox_doc WHERE is_show = 1 AND cid = ?")
                .bind(c)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_toolbox_doc WHERE is_show = 1")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(n.max(0) as u64)
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<HrDoc>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_toolbox_doc WHERE id = ? AND is_show = 1");
    sqlx::query_as::<_, HrDoc>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_hit(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_toolbox_doc SET downnum = downnum + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
