use super::entity::View;
use sqlx::MySqlPool;

/// Inserts one visit record. The same viewer+target pair may be recorded multiple times.
pub async fn record(
    pool: &MySqlPool,
    viewer_uid: u64,
    kind: i32,
    target_id: u64,
    datetime: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_rs_views (viewer_uid, kind, target_id, datetime)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(viewer_uid)
    .bind(kind)
    .bind(target_id)
    .bind(datetime)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

// ==================== "Viewed by me" (viewer perspective) ====================

pub async fn list_by_viewer(
    pool: &MySqlPool,
    viewer_uid: u64,
    kind: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<View>, sqlx::Error> {
    sqlx::query_as::<_, View>(
        r#"SELECT id, viewer_uid, kind, target_id, datetime
           FROM phpyun_rs_views
           WHERE viewer_uid = ? AND kind = ?
           ORDER BY datetime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(viewer_uid)
    .bind(kind)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_by_viewer(
    pool: &MySqlPool,
    viewer_uid: u64,
    kind: i32,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_rs_views WHERE viewer_uid = ? AND kind = ?",
    )
    .bind(viewer_uid)
    .bind(kind)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

// ==================== "Who viewed me" (target perspective) ====================

pub async fn list_by_target(
    pool: &MySqlPool,
    kind: i32,
    target_id: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<View>, sqlx::Error> {
    sqlx::query_as::<_, View>(
        r#"SELECT id, viewer_uid, kind, target_id, datetime
           FROM phpyun_rs_views
           WHERE kind = ? AND target_id = ?
           ORDER BY datetime DESC
           LIMIT ? OFFSET ?"#,
    )
    .bind(kind)
    .bind(target_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_by_target(
    pool: &MySqlPool,
    kind: i32,
    target_id: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_rs_views WHERE kind = ? AND target_id = ?",
    )
    .bind(kind)
    .bind(target_id)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

// ==================== Recently viewed (used for dedup / rate limiting) ====================

/// Checks whether a given viewer has viewed the same target within the last `since` seconds. Used by the frontend / handler for deduplication.
pub async fn recently_viewed(
    pool: &MySqlPool,
    viewer_uid: u64,
    kind: i32,
    target_id: u64,
    since_ts: i64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        r#"SELECT id FROM phpyun_rs_views
           WHERE viewer_uid = ? AND kind = ? AND target_id = ? AND datetime > ?
           LIMIT 1"#,
    )
    .bind(viewer_uid)
    .bind(kind)
    .bind(target_id)
    .bind(since_ts)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}
