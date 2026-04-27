//! User ratings repo — backed by Rust-only `phpyun_rs_rating` (detail) +
//! `phpyun_rs_rating_aggregate` (cached count/avg) tables.
//!
//! PHPYun's `phpyun_company_rating` is a VIP-package config table and is
//! NOT used here. Migration `20260426000002_user_ratings.sql` provisions
//! the two Rust-side tables.

use super::entity::{Rating, RatingAggregate};
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, rater_uid, target_uid, target_kind, stars, comment, status, created_at, updated_at";

pub async fn find_mine(
    pool: &MySqlPool,
    rater_uid: u64,
    target_uid: u64,
    target_kind: i32,
) -> Result<Option<Rating>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_rs_rating
         WHERE rater_uid = ? AND target_uid = ? AND target_kind = ?"
    );
    sqlx::query_as::<_, Rating>(&sql)
        .bind(rater_uid)
        .bind(target_uid)
        .bind(target_kind)
        .fetch_optional(pool)
        .await
}

/// Upsert: only one row per (rater, target); a repeat rating overwrites.
/// Returns (new_id_if_inserted, prev_stars_if_existed).
pub async fn upsert(
    pool: &MySqlPool,
    rater_uid: u64,
    target_uid: u64,
    target_kind: i32,
    stars: i32,
    comment: &str,
    now: i64,
) -> Result<(Option<u64>, Option<i32>), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let prev = sqlx::query_as::<_, (i32,)>(
        "SELECT stars FROM phpyun_rs_rating
         WHERE rater_uid = ? AND target_uid = ? AND target_kind = ? FOR UPDATE",
    )
    .bind(rater_uid)
    .bind(target_uid)
    .bind(target_kind)
    .fetch_optional(&mut *tx)
    .await?
    .map(|(s,)| s);

    let res = sqlx::query(
        r#"INSERT INTO phpyun_rs_rating
           (rater_uid, target_uid, target_kind, stars, comment, status, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, 1, ?, ?)
           ON DUPLICATE KEY UPDATE
               stars      = VALUES(stars),
               comment    = VALUES(comment),
               status     = 1,
               updated_at = VALUES(updated_at)"#,
    )
    .bind(rater_uid)
    .bind(target_uid)
    .bind(target_kind)
    .bind(stars)
    .bind(comment)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Update aggregate.
    let delta_count: i32 = if prev.is_some() { 0 } else { 1 };
    let delta_sum: i32 = stars - prev.unwrap_or(0);
    sqlx::query(
        r#"INSERT INTO phpyun_rs_rating_aggregate
           (target_uid, target_kind, count, sum_stars, avg_x100, updated_at)
           VALUES (?, ?, ?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE
               count      = GREATEST(CAST(count AS SIGNED) + VALUES(count), 0),
               sum_stars  = GREATEST(CAST(sum_stars AS SIGNED) + VALUES(sum_stars), 0),
               avg_x100   = IF((count + VALUES(count)) = 0, 0,
                               ROUND(((sum_stars + VALUES(sum_stars)) * 100) / (count + VALUES(count)))),
               updated_at = VALUES(updated_at)"#,
    )
    .bind(target_uid)
    .bind(target_kind)
    .bind(delta_count)
    .bind(delta_sum)
    .bind(if prev.is_some() { 0 } else { (stars as u32) * 100 })
    .bind(now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((
        if prev.is_none() { Some(res.last_insert_id()) } else { None },
        prev,
    ))
}

pub async fn delete(
    pool: &MySqlPool,
    rater_uid: u64,
    target_uid: u64,
    target_kind: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let prev = sqlx::query_as::<_, (i32,)>(
        "SELECT stars FROM phpyun_rs_rating
         WHERE rater_uid = ? AND target_uid = ? AND target_kind = ? FOR UPDATE",
    )
    .bind(rater_uid)
    .bind(target_uid)
    .bind(target_kind)
    .fetch_optional(&mut *tx)
    .await?
    .map(|(s,)| s);

    let affected = sqlx::query(
        "DELETE FROM phpyun_rs_rating
         WHERE rater_uid = ? AND target_uid = ? AND target_kind = ?",
    )
    .bind(rater_uid)
    .bind(target_uid)
    .bind(target_kind)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    if let Some(stars) = prev {
        sqlx::query(
            r#"UPDATE phpyun_rs_rating_aggregate
               SET count = GREATEST(CAST(count AS SIGNED) - 1, 0),
                   sum_stars = GREATEST(CAST(sum_stars AS SIGNED) - ?, 0),
                   avg_x100 = IF(count <= 1, 0, ROUND(((sum_stars - ?) * 100) / (count - 1))),
                   updated_at = ?
               WHERE target_uid = ? AND target_kind = ?"#,
        )
        .bind(stars)
        .bind(stars)
        .bind(now)
        .bind(target_uid)
        .bind(target_kind)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(affected)
}

pub async fn aggregate(
    pool: &MySqlPool,
    target_uid: u64,
    target_kind: i32,
) -> Result<Option<RatingAggregate>, sqlx::Error> {
    sqlx::query_as::<_, RatingAggregate>(
        "SELECT target_uid, target_kind, count, sum_stars, avg_x100, updated_at
         FROM phpyun_rs_rating_aggregate WHERE target_uid = ? AND target_kind = ?",
    )
    .bind(target_uid)
    .bind(target_kind)
    .fetch_optional(pool)
    .await
}

pub async fn list_for_target(
    pool: &MySqlPool,
    target_uid: u64,
    target_kind: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<Rating>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_rs_rating
         WHERE target_uid = ? AND target_kind = ? AND status = 1
         ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Rating>(&sql)
        .bind(target_uid)
        .bind(target_kind)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_for_target(
    pool: &MySqlPool,
    target_uid: u64,
    target_kind: i32,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_rs_rating
         WHERE target_uid = ? AND target_kind = ? AND status = 1",
    )
    .bind(target_uid)
    .bind(target_kind)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}
