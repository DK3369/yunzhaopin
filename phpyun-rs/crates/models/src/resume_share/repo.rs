use super::entity::ShareToken;
use sqlx::MySqlPool;

const FIELDS: &str = "token, CAST(uid AS SIGNED) uid, CAST(view_count AS SIGNED) view_count, CAST(expires_at AS SIGNED) expires_at, CAST(revoked_at AS SIGNED) revoked_at, CAST(created_at AS SIGNED) created_at";

pub async fn create(pool: &MySqlPool, token: &str, uid: u64, expires_at: i64, now: i64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO phpyun_rs_resume_share_tokens(token,uid,expires_at,created_at) VALUES(?,?,?,?)")
        .bind(token).bind(uid).bind(expires_at).bind(now).execute(pool).await?;
    Ok(())
}
pub async fn find(pool: &MySqlPool, token: &str) -> Result<Option<ShareToken>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {FIELDS} FROM phpyun_rs_resume_share_tokens WHERE token=?"))
        .bind(token).fetch_optional(pool).await
}
pub async fn incr_view(pool: &MySqlPool, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_rs_resume_share_tokens SET view_count=view_count+1 WHERE token=?")
        .bind(token).execute(pool).await?; Ok(())
}
pub async fn revoke(pool: &MySqlPool, token: &str, uid: u64, now: i64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_rs_resume_share_tokens SET revoked_at=? WHERE token=? AND uid=? AND revoked_at=0")
        .bind(now).bind(token).bind(uid).execute(pool).await?.rows_affected())
}
pub async fn list_by_uid(pool: &MySqlPool, uid: u64, offset: u64, limit: u64) -> Result<Vec<ShareToken>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {FIELDS} FROM phpyun_rs_resume_share_tokens WHERE uid=? ORDER BY created_at DESC LIMIT ? OFFSET ?"))
        .bind(uid).bind(limit).bind(offset).fetch_all(pool).await
}
pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT CAST(COUNT(*) AS SIGNED) FROM phpyun_rs_resume_share_tokens WHERE uid=?")
        .bind(uid).fetch_one(pool).await?; Ok(n.max(0) as u64)
}
pub async fn purge_stale(pool: &MySqlPool, cutoff: i64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_rs_resume_share_tokens WHERE expires_at<? OR (revoked_at>0 AND revoked_at<?)")
        .bind(cutoff).bind(cutoff).execute(pool).await?.rows_affected())
}
