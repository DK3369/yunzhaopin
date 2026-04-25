//! Multi-account collaboration for companies (aligned with the HR shared-account logic in PHPYun `comtc.model.php`).
//!
//! The primary company (`usertype=2`) generates an invitation code; HR accounts join via that code and become secondary members of the company.
//! Once joined, HR can post jobs and view applications on behalf of the company (downstream modules can consult `company_hrs` to decide).

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_hr::{
    entity::{CompanyHr, InviteCode},
    repo as hr_repo,
};
use uuid::Uuid;

/// Generate a 16-character invitation code (uppercase letters + digits, ambiguous characters removed)
fn gen_code() -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let u = Uuid::now_v7().as_u128();
    let mut buf = String::with_capacity(16);
    for i in 0..16 {
        let idx = ((u >> (i * 5)) & 0x1F) as usize % CHARS.len();
        buf.push(CHARS[idx] as char);
    }
    buf
}

// ---------- Primary company side ----------

pub struct CodeInput<'a> {
    pub note: &'a str,
    pub max_uses: u32,
    pub expires_at: i64,
}

pub async fn create_code(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CodeInput<'_>,
) -> AppResult<InviteCode> {
    user.require_employer()?;
    let now = clock::now_ts();
    let code = gen_code();
    let id = hr_repo::create_code(
        state.db.pool(),
        user.uid,
        &code,
        input.note,
        input.max_uses,
        input.expires_at,
        now,
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company.invite_code.create", audit::Actor::uid(user.uid))
            .target(format!("code:{code}")),
    )
    .await;
    Ok(InviteCode {
        id,
        company_uid: user.uid,
        code,
        note: input.note.to_string(),
        max_uses: input.max_uses,
        used_count: 0,
        expires_at: input.expires_at,
        status: 1,
        created_at: now,
    })
}

pub async fn list_codes(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<InviteCode>> {
    user.require_employer()?;
    Ok(hr_repo::list_codes(state.db.reader(), user.uid).await?)
}

pub async fn revoke_code(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = hr_repo::revoke_code(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn list_hrs(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<CompanyHr>> {
    user.require_employer()?;
    Ok(hr_repo::list_hrs(state.db.reader(), user.uid).await?)
}

pub async fn remove_hr(
    state: &AppState,
    user: &AuthenticatedUser,
    hr_uid: u64,
) -> AppResult<()> {
    user.require_employer()?;
    hr_repo::remove_hr(state.db.pool(), user.uid, hr_uid).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company.hr.remove", audit::Actor::uid(user.uid))
            .target(format!("uid:{hr_uid}")),
    )
    .await;
    Ok(())
}

// ---------- HR side ----------

pub async fn join_by_code(
    state: &AppState,
    user: &AuthenticatedUser,
    code: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    let now = clock::now_ts();
    let c = hr_repo::find_code_active(state.db.reader(), code, now)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("invalid_code".into())))?;
    if c.company_uid == user.uid {
        return Err(AppError::new(InfraError::InvalidParam("cannot_join_self".into())));
    }
    let consumed = hr_repo::consume_code(state.db.pool(), c.id).await?;
    if consumed == 0 {
        return Err(AppError::new(InfraError::InvalidParam("code_exhausted".into())));
    }
    hr_repo::add_hr(state.db.pool(), c.company_uid, user.uid, "hr", now).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company.hr.join", audit::Actor::uid(user.uid))
            .target(format!("company:{}", c.company_uid)),
    )
    .await;
    Ok(c.company_uid)
}

pub async fn my_companies(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<CompanyHr>> {
    Ok(hr_repo::list_companies_for_hr(state.db.reader(), user.uid).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_is_16_chars_no_confusables() {
        let c = gen_code();
        assert_eq!(c.len(), 16);
        for ch in c.chars() {
            assert!(ch.is_ascii_alphanumeric());
            assert!(ch != 'I' && ch != 'O' && ch != '0' && ch != '1');
        }
    }
}
