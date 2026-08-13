use super::entity::{CompanyHr, InviteCode};
use sqlx::MySqlPool;

const CODE_FIELDS: &str = "CAST(id AS SIGNED) id, CAST(company_uid AS SIGNED) company_uid, code, note, CAST(max_uses AS SIGNED) max_uses, CAST(used_count AS SIGNED) used_count, CAST(expires_at AS SIGNED) expires_at, CAST(status AS SIGNED) status, CAST(created_at AS SIGNED) created_at";
const HR_FIELDS: &str = "CAST(company_uid AS SIGNED) company_uid, CAST(hr_uid AS SIGNED) hr_uid, role, CAST(joined_at AS SIGNED) joined_at, CAST(status AS SIGNED) status";

pub async fn create_code(
    pool: &MySqlPool,
    company_uid: u64,
    code: &str,
    note: &str,
    max_uses: u32,
    expires_at: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let r=sqlx::query("INSERT INTO phpyun_rs_company_invite_codes(company_uid,code,note,max_uses,expires_at,created_at) VALUES(?,?,?,?,?,?)")
        .bind(company_uid).bind(code).bind(note).bind(max_uses).bind(expires_at).bind(now).execute(pool).await?;
    Ok(r.last_insert_id())
}
pub async fn find_code_active(
    pool: &MySqlPool,
    code: &str,
    now: i64,
) -> Result<Option<InviteCode>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {CODE_FIELDS} FROM phpyun_rs_company_invite_codes WHERE code=? AND status=1 AND used_count<max_uses AND (expires_at=0 OR expires_at>?)"))
        .bind(code).bind(now).fetch_optional(pool).await
}
pub async fn consume_code(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_rs_company_invite_codes SET used_count=used_count+1, status=IF(used_count+1>=max_uses,0,status) WHERE id=? AND status=1 AND used_count<max_uses")
        .bind(id).execute(pool).await?.rows_affected())
}
pub async fn list_codes(
    pool: &MySqlPool,
    company_uid: u64,
) -> Result<Vec<InviteCode>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {CODE_FIELDS} FROM phpyun_rs_company_invite_codes WHERE company_uid=? ORDER BY id DESC"))
        .bind(company_uid).fetch_all(pool).await
}
pub async fn revoke_code(pool: &MySqlPool, id: u64, company_uid: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_rs_company_invite_codes SET status=0 WHERE id=? AND company_uid=? AND status=1")
        .bind(id).bind(company_uid).execute(pool).await?.rows_affected())
}
pub async fn add_hr(
    pool: &MySqlPool,
    company_uid: u64,
    hr_uid: u64,
    role: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO phpyun_rs_company_hrs(company_uid,hr_uid,role,joined_at,status) VALUES(?,?,?,?,1) ON DUPLICATE KEY UPDATE role=VALUES(role),status=1")
        .bind(company_uid).bind(hr_uid).bind(role).bind(now).execute(pool).await?;
    Ok(())
}
pub async fn remove_hr(
    pool: &MySqlPool,
    company_uid: u64,
    hr_uid: u64,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query(
        "UPDATE phpyun_rs_company_hrs SET status=0 WHERE company_uid=? AND hr_uid=? AND status=1",
    )
    .bind(company_uid)
    .bind(hr_uid)
    .execute(pool)
    .await?
    .rows_affected())
}
pub async fn list_hrs(pool: &MySqlPool, company_uid: u64) -> Result<Vec<CompanyHr>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {HR_FIELDS} FROM phpyun_rs_company_hrs WHERE company_uid=? AND status=1 ORDER BY joined_at"))
        .bind(company_uid).fetch_all(pool).await
}
pub async fn list_companies_for_hr(
    pool: &MySqlPool,
    hr_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {HR_FIELDS} FROM phpyun_rs_company_hrs WHERE hr_uid=? AND status=1 ORDER BY joined_at"))
        .bind(hr_uid).fetch_all(pool).await
}
