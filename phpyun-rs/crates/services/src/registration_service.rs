//! Registration flow (aligned with PHPYun `userinfo.model.php::userRegSave`).
//!
//! Steps:
//! 1. Image captcha captcha_cid + captcha_input -> `verify::verify(ImageCaptcha, cid, input)`
//! 2. SMS code mobile + sms_code -> `verify::verify(SmsRegister, mobile, code)`
//! 3. Uniqueness: username / mobile / email (all three are checked)
//! 4. Generate salt (16 random bytes) + argon2-hash the password
//! 5. INSERT phpyun_member
//! 6. Audit event `user.register`
//! 7. Auto-login, issue access+refresh token pair

use phpyun_auth::argon2_hash_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt::{issue_pair, JwtIssued};
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{
    clock,
    metrics::auth_event,
    AppError, AppResult, AppState, InfraError,
};
use phpyun_models::company::repo as company_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::user::repo as user_repo;
use uuid::Uuid;

pub struct RegisterInput<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub mobile: &'a str,
    pub email: Option<&'a str>,
    pub captcha_cid: &'a str,
    pub captcha_input: &'a str,
    pub sms_code: &'a str,
    pub usertype: u8,
    /// PHPYun `regway`: 1=username / 2=mobile / 3=email
    /// (currently only audited; uniqueness is fully checked)
    pub regway: u8,
    pub did: u32,
    pub client_ip: &'a str,
    pub user_agent: &'a str,
    /// Inviter uid (aligned with PHPYun reward); 0 means no inviter
    pub referrer_uid: u64,
}

pub struct RegisterResult {
    pub uid: u64,
    pub access: String,
    pub refresh: String,
    pub access_exp: i64,
    pub refresh_exp: i64,
}

pub async fn register(state: &AppState, input: RegisterInput<'_>) -> AppResult<RegisterResult> {
    // 1. Image captcha (case-insensitive: stored uppercase, input is upper-cased before compare)
    let captcha_input_upper = input.captcha_input.to_uppercase();
    if !verify::verify(
        &state.redis,
        VerifyKind::ImageCaptcha,
        input.captcha_cid,
        &captcha_input_upper,
    )
    .await?
    {
        auth_event("register_fail", Some("bad_captcha"));
        return Err(AppError::captcha());
    }

    // 2. SMS code
    if !verify::verify(
        &state.redis,
        VerifyKind::SmsRegister,
        input.mobile,
        input.sms_code,
    )
    .await?
    {
        auth_event("register_fail", Some("bad_sms_code"));
        return Err(InfraError::InvalidParam("sms_code".into()).into());
    }

    // 3. Uniqueness check (writer guarantees real-time consistency)
    let writer = state.db.pool();
    if user_repo::exists_username(writer, input.username).await? {
        return Err(InfraError::InvalidParam("username_taken".into()).into());
    }
    if user_repo::exists_mobile(writer, input.mobile).await? {
        return Err(InfraError::InvalidParam("mobile_taken".into()).into());
    }
    if let Some(email) = input.email {
        if !email.is_empty() && user_repo::exists_email(writer, email).await? {
            return Err(InfraError::InvalidParam("email_taken".into()).into());
        }
    }

    // 4. Generate salt + argon2 hash (spawn_blocking)
    let salt = gen_salt();
    let salted = format!("{}{}", input.password, salt);
    let password_hash = argon2_hash_async(salted.clone()).await?;

    // 5. Persist (member + insert resume or company depending on role)
    //
    // Use a transaction for atomicity: if member is inserted but resume/company fails,
    // roll back as a whole.
    // PHPYun's `userRegSave` is sequential INSERTs without a transaction; if it crashes
    // it leaves an orphan member -- we fix that here.
    let now = clock::now_ts();
    let username = input.username.to_string();
    let password_hash_c = password_hash.clone();
    let salt_c = salt.clone();
    let mobile_c = input.mobile.to_string();
    let email_c = input.email.filter(|e| !e.is_empty()).map(str::to_string);
    let ip_c = input.client_ip.to_string();
    let usertype = input.usertype;
    let did = input.did;

    let uid = state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                let uid = user_repo::create_member(
                    &mut **tx,
                    &username,
                    &password_hash_c,
                    &salt_c,
                    Some(&mobile_c),
                    email_c.as_deref(),
                    usertype,
                    did,
                    &ip_c,
                    now,
                )
                .await?;
                match usertype {
                    1 => resume_repo::ensure_row(&mut **tx, uid, did, now).await?,
                    2 => company_repo::ensure_row(&mut **tx, uid, did).await?,
                    _ => {} // usertype=3 campus; auxiliary table is not linked yet
                }
                Ok::<u64, AppError>(uid)
            })
        })
        .await?;

    auth_event("register_success", None);

    // 6. Audit
    let _ = audit::emit(
        state,
        AuditEvent::new("user.register", Actor::uid(uid).with_ip(input.client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({
                "usertype": input.usertype,
                "regway": input.regway,
                "did": input.did,
            })),
    )
    .await;

    // 7. Auto-login
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, uid, input.usertype, input.did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid,
            usertype: input.usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: input.client_ip,
            ua: input.user_agent,
        },
    )
    .await;

    // Referral reward: fire-and-forget; failure does not roll back registration
    if input.referrer_uid > 0 {
        crate::referral_service::record_on_signup(state, input.referrer_uid, uid).await;
    }

    Ok(RegisterResult {
        uid,
        access,
        refresh,
        access_exp,
        refresh_exp,
    })
}

/// 16-character salt (PHPYun's salt is 6 chars; we bump to 16; argon2 accepts any length)
fn gen_salt() -> String {
    let u = Uuid::now_v7();
    // The first 16 hex chars are enough
    u.simple().to_string().chars().take(16).collect()
}
