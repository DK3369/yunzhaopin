use super::entity::CompanyClaim;
use sqlx::MySqlPool;

pub async fn record(
    pool: &MySqlPool,
    uid: u64,
    claimer_uid: u64,
    client_ip: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_company_fact
           (uid, claimer_uid, client_ip, created_at)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(claimer_uid)
    .bind(client_ip)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<CompanyClaim>, sqlx::Error> {
    sqlx::query_as::<_, CompanyClaim>(
        r#"SELECT id, uid, claimer_uid, client_ip, created_at
           FROM phpyun_company_fact WHERE uid = ?"#,
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}
