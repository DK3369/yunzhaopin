//! PHPYun `phpyun_moblie_msg` (SMS send log). Admin read-only.

use super::entity::MoblieMsg;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS SIGNED) AS uid, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(cuid, 0) AS SIGNED) AS cuid, \
    COALESCE(cname, '') AS cname, \
    COALESCE(moblie, '') AS moblie, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime, \
    CAST(COALESCE(state, 0) AS SIGNED) AS state, \
    COALESCE(ip, '') AS ip, \
    CAST(COALESCE(del, 0) AS SIGNED) AS del, \
    CAST(COALESCE(msgtype, 0) AS SIGNED) AS msgtype";

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<MoblieMsg>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_moblie_msg WHERE del = 0 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, MoblieMsg>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_moblie_msg WHERE del = 0")
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}
