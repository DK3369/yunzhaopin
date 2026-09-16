//! `phpyun_rs_article_channel_sub` — per-user news_group subscriptions.

use super::entity::ArticleChannelSub;
use sqlx::MySqlPool;

pub async fn find(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<ArticleChannelSub>, sqlx::Error> {
    let r = sqlx::query_as::<_, ArticleChannelSub>(
        "SELECT CAST(uid AS UNSIGNED) AS uid, group_ids, \
                CAST(updated_at AS SIGNED) AS updated_at \
         FROM phpyun_rs_article_channel_sub WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn upsert(
    pool: &MySqlPool,
    uid: u64,
    group_ids: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO phpyun_rs_article_channel_sub (uid, group_ids, updated_at) \
         VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE group_ids = VALUES(group_ids), updated_at = VALUES(updated_at)",
    )
    .bind(uid)
    .bind(group_ids)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
