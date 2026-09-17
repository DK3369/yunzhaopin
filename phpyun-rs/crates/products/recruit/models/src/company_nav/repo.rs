//! `phpyun_company_nav.nav_info` JSON (this fork does not interchange PHP serialize).

use sqlx::MySqlPool;

pub async fn find_nav_info(pool: &MySqlPool, uid: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(nav_info, '') FROM phpyun_company_nav WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(s,)| s).filter(|s| !s.is_empty()))
}

pub async fn upsert_nav_info(pool: &MySqlPool, uid: u64, nav_info: &str) -> Result<(), sqlx::Error> {
    let n = sqlx::query("UPDATE phpyun_company_nav SET nav_info = ? WHERE uid = ?")
        .bind(nav_info)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected();
    if n == 0 {
        sqlx::query("INSERT INTO phpyun_company_nav (uid, nav_info) VALUES (?, ?)")
            .bind(uid)
            .bind(nav_info)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn delete_nav(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_company_nav WHERE uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
