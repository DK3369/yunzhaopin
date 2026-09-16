//! `phpyun_rs_interview_review` — App multi-dimension interview scores.

use super::entity::InterviewReview;
use sqlx::MySqlPool;

const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
    CAST(yqms_id AS UNSIGNED) AS yqms_id, \
    CAST(rater_uid AS UNSIGNED) AS rater_uid, \
    CAST(ratee_uid AS UNSIGNED) AS ratee_uid, \
    dimensions, \
    CAST(total AS UNSIGNED) AS total, \
    COALESCE(comment, '') AS comment, \
    CAST(created_at AS SIGNED) AS created_at, \
    CAST(updated_at AS SIGNED) AS updated_at";

pub async fn find_mine(
    pool: &MySqlPool,
    rater_uid: u64,
    yqms_id: u64,
) -> Result<Option<InterviewReview>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_rs_interview_review \
         WHERE rater_uid = ? AND yqms_id = ? LIMIT 1"
    );
    let r = sqlx::query_as::<_, InterviewReview>(&sql)
        .bind(rater_uid)
        .bind(yqms_id)
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
    yqms_id: u64,
    rater_uid: u64,
    ratee_uid: u64,
    dimensions: &str,
    total: u32,
    comment: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_interview_review \
            (yqms_id, rater_uid, ratee_uid, dimensions, total, comment, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
         ON DUPLICATE KEY UPDATE \
            ratee_uid = VALUES(ratee_uid), \
            dimensions = VALUES(dimensions), \
            total = VALUES(total), \
            comment = VALUES(comment), \
            updated_at = VALUES(updated_at)",
    )
    .bind(yqms_id)
    .bind(rater_uid)
    .bind(ratee_uid)
    .bind(dimensions)
    .bind(total)
    .bind(comment)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}
