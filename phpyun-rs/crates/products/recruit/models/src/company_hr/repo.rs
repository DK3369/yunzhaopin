use super::entity::{CompanyHr, InviteCode};
use sqlx::MySqlPool;

const CODE_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, CAST(company_uid AS UNSIGNED) AS company_uid, COALESCE(code,'') AS code, COALESCE(note,'') AS note, max_uses, used_count, CAST(expires_at AS SIGNED) AS expires_at, CAST(status AS SIGNED) AS status, CAST(created_at AS SIGNED) AS created_at";
const HR_FIELDS: &str = "CAST(h.company_uid AS UNSIGNED) AS company_uid, CAST(h.hr_uid AS UNSIGNED) AS hr_uid, COALESCE(h.role,'') AS role, CAST(h.joined_at AS SIGNED) AS joined_at, CAST(h.status AS SIGNED) AS status, COALESCE(m.username,'') AS hr_name, COALESCE(c.name,'') AS company_name";

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
    sqlx::query_as(&format!("SELECT {CODE_FIELDS} FROM phpyun_rs_company_invite_codes WHERE code=? AND status=1 AND (max_uses=0 OR used_count<max_uses) AND (expires_at=0 OR expires_at>?)"))
        .bind(code).bind(now).fetch_optional(pool).await
}
pub async fn consume_code(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_rs_company_invite_codes SET used_count=used_count+1, status=IF(max_uses>0 AND used_count+1>=max_uses,0,status) WHERE id=? AND status=1 AND (max_uses=0 OR used_count<max_uses)")
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
    sqlx::query_as(&format!("SELECT {HR_FIELDS} FROM phpyun_rs_company_hrs h LEFT JOIN phpyun_member m ON m.uid=h.hr_uid LEFT JOIN phpyun_company c ON c.uid=h.company_uid WHERE h.company_uid=? AND h.status=1 ORDER BY h.joined_at"))
        .bind(company_uid).fetch_all(pool).await
}
pub async fn list_companies_for_hr(
    pool: &MySqlPool,
    hr_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    sqlx::query_as(&format!("SELECT {HR_FIELDS} FROM phpyun_rs_company_hrs h LEFT JOIN phpyun_member m ON m.uid=h.hr_uid LEFT JOIN phpyun_company c ON c.uid=h.company_uid WHERE h.hr_uid=? AND h.status=1 ORDER BY h.joined_at"))
        .bind(hr_uid).fetch_all(pool).await
}
