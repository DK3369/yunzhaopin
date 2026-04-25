use super::entity::{CompanyHr, InviteCode};
use sqlx::MySqlPool;

const CODE_FIELDS: &str =
    "id, company_uid, code, note, max_uses, used_count, expires_at, status, created_at";

// ---------- invite codes ----------

pub async fn create_code(
    pool: &MySqlPool,
    company_uid: u64,
    code: &str,
    note: &str,
    max_uses: u32,
    expires_at: i64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_rs_company_invite_codes
           (company_uid, code, note, max_uses, used_count, expires_at, status, created_at)
           VALUES (?, ?, ?, ?, 0, ?, 1, ?)"#,
    )
    .bind(company_uid)
    .bind(code)
    .bind(note)
    .bind(max_uses)
    .bind(expires_at)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_code_active(
    pool: &MySqlPool,
    code: &str,
    now: i64,
) -> Result<Option<InviteCode>, sqlx::Error> {
    let sql = format!(
        "SELECT {CODE_FIELDS} FROM phpyun_rs_company_invite_codes
         WHERE code = ? AND status = 1
           AND (expires_at = 0 OR expires_at > ?)"
    );
    sqlx::query_as::<_, InviteCode>(&sql)
        .bind(code)
        .bind(now)
        .fetch_optional(pool)
        .await
}

pub async fn consume_code(
    pool: &MySqlPool,
    id: u64,
) -> Result<u64, sqlx::Error> {
    // Atomic +1, validating max_uses at the same time (0 = unlimited).
    let res = sqlx::query(
        r#"UPDATE phpyun_rs_company_invite_codes
           SET used_count = used_count + 1
           WHERE id = ? AND status = 1
             AND (max_uses = 0 OR used_count < max_uses)"#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_codes(
    pool: &MySqlPool,
    company_uid: u64,
) -> Result<Vec<InviteCode>, sqlx::Error> {
    let sql = format!(
        "SELECT {CODE_FIELDS} FROM phpyun_rs_company_invite_codes
         WHERE company_uid = ? ORDER BY id DESC"
    );
    sqlx::query_as::<_, InviteCode>(&sql)
        .bind(company_uid)
        .fetch_all(pool)
        .await
}

pub async fn revoke_code(
    pool: &MySqlPool,
    id: u64,
    company_uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_company_invite_codes SET status = 0
         WHERE id = ? AND company_uid = ?",
    )
    .bind(id)
    .bind(company_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

// ---------- hrs ----------

pub async fn add_hr(
    pool: &MySqlPool,
    company_uid: u64,
    hr_uid: u64,
    role: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_company_consultant
           (company_uid, hr_uid, role, joined_at, status)
           VALUES (?, ?, ?, ?, 1)
           ON DUPLICATE KEY UPDATE role = VALUES(role), status = 1, joined_at = VALUES(joined_at)"#,
    )
    .bind(company_uid)
    .bind(hr_uid)
    .bind(role)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_hr(
    pool: &MySqlPool,
    company_uid: u64,
    hr_uid: u64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_consultant SET status = 0
         WHERE company_uid = ? AND hr_uid = ?",
    )
    .bind(company_uid)
    .bind(hr_uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_hrs(
    pool: &MySqlPool,
    company_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    sqlx::query_as::<_, CompanyHr>(
        "SELECT company_uid, hr_uid, role, joined_at, status
         FROM phpyun_company_consultant WHERE company_uid = ? AND status = 1
         ORDER BY joined_at DESC",
    )
    .bind(company_uid)
    .fetch_all(pool)
    .await
}

pub async fn list_companies_for_hr(
    pool: &MySqlPool,
    hr_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    sqlx::query_as::<_, CompanyHr>(
        "SELECT company_uid, hr_uid, role, joined_at, status
         FROM phpyun_company_consultant WHERE hr_uid = ? AND status = 1",
    )
    .bind(hr_uid)
    .fetch_all(pool)
    .await
}
