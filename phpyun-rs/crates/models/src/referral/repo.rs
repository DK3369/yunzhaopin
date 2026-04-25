use super::entity::Referral;
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, inviter_uid, invitee_uid, points, status, created_at";

/// Record a referral. The unique key on invitee_uid guarantees each
/// invitee can be triggered at most once. affected=0 means duplicate;
/// callers should treat that as a no-op rather than an error.
pub async fn record(
    pool: &MySqlPool,
    inviter_uid: u64,
    invitee_uid: u64,
    points: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT IGNORE INTO phpyun_finder
           (inviter_uid, invitee_uid, points, status, created_at)
           VALUES (?, ?, ?, 1, ?)"#,
    )
    .bind(inviter_uid)
    .bind(invitee_uid)
    .bind(points)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_by_inviter(
    pool: &MySqlPool,
    inviter_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Referral>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_finder
         WHERE inviter_uid = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Referral>(&sql)
        .bind(inviter_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_inviter(
    pool: &MySqlPool,
    inviter_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_finder WHERE inviter_uid = ?",
    )
    .bind(inviter_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Total points the inviter has earned cumulatively.
pub async fn total_points_earned(
    pool: &MySqlPool,
    inviter_uid: u64,
) -> Result<i64, sqlx::Error> {
    let (n,): (Option<i64>,) = sqlx::query_as(
        "SELECT SUM(points) FROM phpyun_finder
         WHERE inviter_uid = ? AND status = 1",
    )
    .bind(inviter_uid)
    .fetch_one(pool)
    .await?;
    Ok(n.unwrap_or(0).max(0))
}
