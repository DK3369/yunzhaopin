use super::entity::{PosterKind, PosterTemplate};
use sqlx::MySqlPool;

const FIELDS: &str = "id, title, pic, `type`, isopen, sort, num, config_pos";

pub async fn list_by_kind(
    pool: &MySqlPool,
    kind: PosterKind,
) -> Result<Vec<PosterTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_jobwhb \
         WHERE `type` = ? AND isopen = 1 ORDER BY num DESC, sort DESC, id DESC"
    );
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(kind.as_i8())
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<PosterTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_jobwhb WHERE id = ? LIMIT 1"
    );
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Fetch the first (hottest) enabled template for a given kind; used as
/// the default template when the `hb` parameter is omitted.
pub async fn default_for_kind(
    pool: &MySqlPool,
    kind: PosterKind,
) -> Result<Option<PosterTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_jobwhb \
         WHERE `type` = ? AND isopen = 1 ORDER BY sort DESC, num DESC, id DESC LIMIT 1"
    );
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(kind.as_i8())
        .fetch_optional(pool)
        .await
}

/// Increment usage count.
pub async fn incr_num(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_admin_jobwhb SET num = num + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
