//! Company claim (aligned with PHPYun `wap/claim` + `claim/index`).
//!
//! Claim code source: `phpyun_company_cert.check2` where `type=6`.
//! Duplicate protection: `phpyun_member.claim==1` (and `source==6` eligibility).

use phpyun_auth::argon2_hash_async;
use phpyun_core::{audit, clock, ApiError, AppResult, AppState};
use phpyun_models::company_cert::repo as cert_repo;
use phpyun_models::user::repo as user_repo;
use uuid::Uuid;

fn gen_salt() -> String {
    Uuid::now_v7()
        .simple()
        .to_string()
        .chars()
        .take(16)
        .collect()
}

pub struct ClaimInput<'a> {
    pub uid: u64,
    pub code: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub client_ip: &'a str,
}

pub struct ClaimCheck {
    pub ok: bool,
}

pub async fn check(state: &AppState, uid: u64, code: &str) -> AppResult<ClaimCheck> {
    verify_eligibility_and_code(state, uid, code).await?;
    Ok(ClaimCheck { ok: true })
}

async fn verify_eligibility_and_code(state: &AppState, uid: u64, code: &str) -> AppResult<()> {
    let reader = state.db.reader();
    let Some((source, claim, _has_email)) = user_repo::claim_eligibility(reader, uid).await? else {
        return Err(ApiError::param_invalid("member_not_found"));
    };
    if claim == 1 {
        return Err(ApiError::param_invalid("already_claimed"));
    }
    if source != 6 {
        return Err(ApiError::param_invalid("claim_not_eligible"));
    }
    let stored = cert_repo::find_claim_code(reader, uid)
        .await?
        .unwrap_or_default();
    if stored.is_empty() || stored != code {
        return Err(ApiError::param_invalid("invalid_claim_code"));
    }
    Ok(())
}

pub async fn claim(state: &AppState, input: ClaimInput<'_>) -> AppResult<()> {
    verify_eligibility_and_code(state, input.uid, input.code).await?;

    let db = state.db.pool();
    let reader = state.db.reader();

    if user_repo::exists_username(reader, input.username).await? {
        return Err(ApiError::param_invalid("username_taken"));
    }

    let salt = gen_salt();
    let salted = format!("{}{}", input.password, salt);
    let hash = argon2_hash_async(salted).await?;
    let now = clock::now_ts();
    let affected =
        user_repo::update_username_and_password(db, input.uid, input.username, &salt, &hash, now)
            .await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("already_claimed"));
    }

    let _ = audit::emit(
        state,
        audit::AuditEvent::new(
            "company.claim",
            audit::Actor::uid(input.uid).with_ip(input.client_ip),
        )
        .target(format!("uid:{}", input.uid)),
    )
    .await;

    Ok(())
}
