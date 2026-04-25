//! Company claim (aligned with PHPYun `wap/claim`).
//!
//! Business: a company account (imported by the admin, currently unowned) -> admin sends the claim code `check2` to the real company contact ->
//! the user provides `uid + code + new_username + new_password` to complete the claim:
//!   - Update the uid's username / password (marking the account as having a new owner).
//!   - Write a claims table row (unique index prevents duplicate claims).
//!   - Audit.
//!
//! Claim code source: the `check2` field of PHPYun `phpyun_cert` rows where `type=6`.

use phpyun_auth::argon2_hash_async;
use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState};
use phpyun_models::company_claim::repo as claim_repo;
use phpyun_models::user::repo as user_repo;
use uuid::Uuid;

fn gen_salt() -> String {
    Uuid::now_v7().simple().to_string().chars().take(16).collect()
}

pub struct ClaimInput<'a> {
    pub uid: u64,
    pub code: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub client_ip: &'a str,
}

pub async fn claim(state: &AppState, input: ClaimInput<'_>) -> AppResult<()> {
    let db = state.db.pool();
    let reader = state.db.reader();

    // 1) Verify the claim code
    let code = user_repo::get_claim_code(reader, input.uid)
        .await?
        .unwrap_or_default();
    if code.is_empty() || code != input.code {
        return Err(AppError::new(InfraError::InvalidParam("invalid_claim_code".into())));
    }

    // 2) Prevent duplicate claims
    if claim_repo::find_by_uid(db, input.uid).await?.is_some() {
        return Err(AppError::new(InfraError::InvalidParam("already_claimed".into())));
    }

    // 3) Username must not be taken
    if user_repo::exists_username(reader, input.username).await? {
        return Err(AppError::new(InfraError::InvalidParam("username_taken".into())));
    }

    // 4) Update username + password (argon2 hash; salt stored separately)
    let salt = gen_salt();
    let salted = format!("{}{}", input.password, salt);
    let hash = argon2_hash_async(salted).await?;
    let now = clock::now_ts();
    let affected =
        user_repo::update_username_and_password(db, input.uid, input.username, &salt, &hash, now)
            .await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam("member_not_found".into())));
    }

    // 5) Write the claim record + audit log
    claim_repo::record(db, input.uid, input.uid, input.client_ip, now).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company.claim", audit::Actor::uid(input.uid).with_ip(input.client_ip))
            .target(format!("uid:{}", input.uid)),
    )
    .await;

    Ok(())
}
