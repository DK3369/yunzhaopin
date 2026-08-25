use super::entity::Remark;
use sqlx::MySqlPool;

/// PHP `phpyun_resume_remark` columns: id, uid (seeker), eid, comid (company),
/// ctime, status, remark. The API still uses `target_uid` / `note` / `updated_at`.
const SELECT: &str = r#"SELECT
    CAST(comid AS UNSIGNED) AS uid,
    CAST(uid AS UNSIGNED) AS target_uid,
    1 AS target_kind,
    COALESCE(remark, '') AS note,
    CAST(ctime AS SIGNED) AS updated_at
FROM phpyun_resume_remark"#;

pub async fn get(
    pool: &MySqlPool,
    company_uid: u64,
    seeker_uid: u64,
    target_kind: i32,
) -> Result<Option<Remark>, sqlx::Error> {
    if target_kind != 0 && target_kind != super::entity::REMARK_RESUME {
        return Ok(None);
    }
    sqlx::query_as::<_, Remark>(&format!(
        "{SELECT} WHERE comid = ? AND uid = ? LIMIT 1"
    ))
    .bind(company_uid)
    .bind(seeker_uid)
    .fetch_optional(pool)
    .await
}

pub async fn upsert(
    pool: &MySqlPool,
    company_uid: u64,
    seeker_uid: u64,
    _target_kind: i32,
    note: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    let existing: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_resume_remark WHERE comid = ? AND uid = ? LIMIT 1",
    )
    .bind(company_uid)
    .bind(seeker_uid)
    .fetch_optional(pool)
    .await?;
    if let Some((id,)) = existing {
        sqlx::query("UPDATE phpyun_resume_remark SET remark = ?, ctime = ? WHERE id = ?")
            .bind(note)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            r#"INSERT INTO phpyun_resume_remark (uid, eid, comid, ctime, status, remark)
               VALUES (?, ?, ?, ?, 0, ?)"#,
        )
        .bind(seeker_uid)
        .bind(seeker_uid)
        .bind(company_uid)
        .bind(now)
        .bind(note)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn delete(
    pool: &MySqlPool,
    company_uid: u64,
    seeker_uid: u64,
    _target_kind: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_remark WHERE comid = ? AND uid = ?")
        .bind(company_uid)
        .bind(seeker_uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn count_by_user(
    pool: &MySqlPool,
    company_uid: u64,
    kind: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if matches!(kind, Some(k) if k != super::entity::REMARK_RESUME) {
        return Ok(0);
    }
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume_remark WHERE comid = ?")
            .bind(company_uid)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_by_user(
    pool: &MySqlPool,
    company_uid: u64,
    kind: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Remark>, sqlx::Error> {
    if matches!(kind, Some(k) if k != super::entity::REMARK_RESUME) {
        return Ok(Vec::new());
    }
    sqlx::query_as::<_, Remark>(&format!(
        "{SELECT} WHERE comid = ? ORDER BY ctime DESC LIMIT ? OFFSET ?"
    ))
    .bind(company_uid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
