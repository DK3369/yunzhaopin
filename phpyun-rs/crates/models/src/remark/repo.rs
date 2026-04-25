use super::entity::Remark;
use sqlx::MySqlPool;

pub async fn get(
    pool: &MySqlPool,
    uid: u64,
    target_uid: u64,
    target_kind: i32,
) -> Result<Option<Remark>, sqlx::Error> {
    sqlx::query_as::<_, Remark>(
        "SELECT uid, target_uid, target_kind, note, updated_at
         FROM phpyun_resume_remark WHERE uid = ? AND target_uid = ? AND target_kind = ?",
    )
    .bind(uid)
    .bind(target_uid)
    .bind(target_kind)
    .fetch_optional(pool)
    .await
}

pub async fn upsert(
    pool: &MySqlPool,
    uid: u64,
    target_uid: u64,
    target_kind: i32,
    note: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_resume_remark (uid, target_uid, target_kind, note, updated_at)
           VALUES (?, ?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE note = VALUES(note), updated_at = VALUES(updated_at)"#,
    )
    .bind(uid)
    .bind(target_uid)
    .bind(target_kind)
    .bind(note)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(
    pool: &MySqlPool,
    uid: u64,
    target_uid: u64,
    target_kind: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM phpyun_resume_remark WHERE uid = ? AND target_uid = ? AND target_kind = ?",
    )
    .bind(uid)
    .bind(target_uid)
    .bind(target_kind)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    kind: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Remark>, sqlx::Error> {
    let sql = match kind {
        Some(_) => "SELECT uid, target_uid, target_kind, note, updated_at
                    FROM phpyun_resume_remark
                    WHERE uid = ? AND target_kind = ?
                    ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        None => "SELECT uid, target_uid, target_kind, note, updated_at
                 FROM phpyun_resume_remark
                 WHERE uid = ?
                 ORDER BY updated_at DESC LIMIT ? OFFSET ?",
    };
    let q = sqlx::query_as::<_, Remark>(sql);
    match kind {
        Some(k) => q.bind(uid).bind(k).bind(limit).bind(offset).fetch_all(pool).await,
        None => q.bind(uid).bind(limit).bind(offset).fetch_all(pool).await,
    }
}
