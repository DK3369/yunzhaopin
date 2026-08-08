//! User authentication service.
//!
//! Concurrency model:
//! - argon2 runs on `spawn_blocking` (non-blocking for tokio, encapsulated in phpyun_auth)
//! - Login-failure counter / reset / JWT blacklist all live in Redis via `core::kv`
//!   for distributed coordination
//! - Legacy md5 passwords are upgraded in the background via `background::spawn_best_effort`
//!   (the user does not wait)
//! - `/me` flows through a three-tier cache: L1 moka + L2 Redis + DB
//!
//! Third-party framework discipline: this file does not directly `use redis::*` /
//! `use serde_json::*` / `use chrono::*` / `use metrics::*` / `tokio::spawn` —
//! everything goes through the `phpyun_core` facade.

use phpyun_auth::{argon2_hash_async, verify_password_async};
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::json;
use phpyun_core::jwt::{issue_pair, JwtIssued};
use phpyun_core::{
    background, jwt_blacklist,
    metrics::{auth_event, cache_hit, cache_miss},
    rate_limit, ApiError, AppResult, AppState,
};
use phpyun_models::company::repo as company_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::user::{entity::Member, repo as user_repo};

use crate::user_session_service::{self, LoginRecord};

/// Login context — carried alongside credentials so the service can record
/// device fingerprint + IP into `phpyun_user_session`. Empty strings are OK
/// (e.g. internal admin tooling).
#[derive(Debug, Default, Clone)]
pub struct LoginContext<'a> {
    pub ip: &'a str,
    pub ua: &'a str,
}

use std::sync::Arc;
use std::time::Duration;

const EMAIL_LOGIN_TTL_SECS: u64 = 600;
const DEV_EMAIL_LOGIN_CODE: &str = "111111";

// ==================== Login ====================

pub struct LoginResult {
    pub access: String,
    pub refresh: String,
    pub uid: u64,
    pub usertype: u8,
    pub access_exp: i64,
    pub refresh_exp: i64,
}

pub async fn login(
    state: &AppState,
    account: &str,
    password: &str,
    ctx: LoginContext<'_>,
) -> AppResult<LoginResult> {
    // 1. Pre-check the login-failure counter (distributed via Redis)
    let account_key = format!("rl:login:fail:{account}");
    if rate_limit::check_and_incr(
        &state.redis,
        &account_key,
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(900),
        },
    )
    .await
    .is_err()
    {
        auth_event("login_blocked", Some("too_many_fails"));
        return Err(ApiError::rate_limit());
    }

    // 2. Look up the user (use the reader pool to offload the writer)
    let user: Member = user_repo::find_for_login(state.db.reader(), account)
        .await?
        .ok_or_else(|| {
            auth_event("login_fail", Some("not_found"));
            ApiError::bad_credentials()
        })?;

    if user.status == 2 {
        auth_event("login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    // 3. Password verification (CPU-intensive, runs on spawn_blocking)
    let valid = verify_password_async(
        password.to_string(),
        user.password.clone(),
        user.salt.clone(),
    )
    .await;
    if !valid {
        auth_event("login_fail", Some("bad_password"));
        return Err(ApiError::bad_credentials());
    }

    // 4. Login succeeded: clear the failure counter
    rate_limit::clear_login_fail(&state.redis, account).await;

    // 5. Asynchronously upgrade legacy md5 hashes (writes the DB; the user does not wait)
    if !user.password.starts_with("$argon2") {
        let writer = state.db.pool().clone();
        let uid = user.uid;
        let pw = password.to_string();
        background::spawn_best_effort("password_upgrade", async move {
            if let Ok(new_hash) = argon2_hash_async(pw).await {
                let _ = user_repo::update_password(&writer, uid, &new_hash).await;
                auth_event("password_upgraded", None);
            }
        });
    }

    // 6. Issue the access + refresh token pair
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(
        &state.config,
        user.uid,
        user.usertype as u8,
        user.did as u32,
    )?;

    // Record the login as a session row so the user can list / kick devices.
    // Best-effort: never blocks login on session-table failure.
    let _ = user_session_service::record_login(
        state,
        LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: ctx.ip,
            ua: ctx.ua,
        },
    )
    .await;

    auth_event("login_success", None);

    // Audit: write to DB + publish to the event bus (failure does not block login)
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid))
            .target(format!("uid:{}", user.uid))
            .success(true),
    )
    .await;

    Ok(LoginResult {
        access,
        refresh,
        uid: user.uid,
        usertype: user.usertype as u8,
        access_exp,
        refresh_exp,
    })
}

// ==================== SMS one-time code login ====================
//
// Aligned with the `act_login=1` branch of PHPYun's `mlogin_action`. Password-less,
// using an SMS code to prove ownership of the phone number.
// Prerequisite: `sms_service::send_sms_code(mobile, SmsScene::Login)` must have sent a
// code beforehand.

pub async fn login_with_sms_code(
    state: &AppState,
    mobile: &str,
    sms_code: &str,
    ctx: LoginContext<'_>,
) -> AppResult<LoginResult> {
    use phpyun_core::verify::{self, VerifyKind};

    // 1. Rate limit (shares the same account key with password login to defend against
    //    credential-stuffing)
    let rl_key = format!("rl:login:fail:{mobile}");
    if rate_limit::check_and_incr(
        &state.redis,
        &rl_key,
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(900),
        },
    )
    .await
    .is_err()
    {
        auth_event("login_blocked", Some("too_many_fails"));
        return Err(ApiError::rate_limit());
    }

    // 2. Verify the SMS code
    if !verify::verify(&state.redis, VerifyKind::SmsLogin, mobile, sms_code).await? {
        auth_event("login_fail", Some("bad_sms_code"));
        return Err(ApiError::bad_credentials());
    }

    // 3. Look up the user by phone number
    let user = user_repo::find_by_mobile(state.db.reader(), mobile)
        .await?
        .ok_or_else(|| {
            auth_event("login_fail", Some("not_found"));
            ApiError::bad_credentials()
        })?;
    if user.status == 2 {
        auth_event("login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    // 4. Clear the failure counter
    rate_limit::clear_login_fail(&state.redis, mobile).await;

    // 5. Issue the access + refresh token pair
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(
        &state.config,
        user.uid,
        user.usertype as u8,
        user.did as u32,
    )?;

    let _ = user_session_service::record_login(
        state,
        LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: ctx.ip,
            ua: ctx.ua,
        },
    )
    .await;

    auth_event("login_success", Some("sms"));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": "sms" })),
    )
    .await;

    Ok(LoginResult {
        access,
        refresh,
        uid: user.uid,
        usertype: user.usertype as u8,
        access_exp,
        refresh_exp,
    })
}

/// Issue a one-time code for the merged email login/registration flow.
pub async fn send_email_login_code(state: &AppState, email: &str) -> AppResult<()> {
    use phpyun_core::verify::{self, VerifyKind};

    let email = email.trim().to_ascii_lowercase();
    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:email:login:hour:{email}"),
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(3600),
        },
    )
    .await?;
    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:email:login:min:{email}"),
        rate_limit::LimitRule {
            max: 1,
            window: Duration::from_secs(60),
        },
    )
    .await?;

    // Development-only fallback: local environments often do not have an MTA
    // configured. Keep production fail-closed so a mail delivery failure never
    // turns into a known login code.
    let code = if state.config.env == "dev" {
        DEV_EMAIL_LOGIN_CODE.to_string()
    } else {
        verify::gen_digit_code(6)
    };
    if state.config.env == "dev" {
        // Development must not wait for a host MTA. A missing or misconfigured
        // sendmail can block forever, preventing the endpoint from returning
        // even though the fixed development code is already known.
        tracing::info!(email = %email, code = %code, "development email login code issued");
    } else {
        crate::mail_service::send_text(
            state,
            &email,
            "Your login verification code",
            &format!(
                "Your verification code is {code}. It expires in {} minutes. If you did not request it, ignore this email.",
                EMAIL_LOGIN_TTL_SECS / 60
            ),
        )
        .await?;
    }
    verify::issue(
        &state.redis,
        VerifyKind::EmailLogin,
        &email,
        &code,
        Duration::from_secs(EMAIL_LOGIN_TTL_SECS),
    )
    .await?;
    auth_event("email_login_code_sent", None);
    Ok(())
}

/// Verify an email code, then log in an existing account or atomically create a new one.
pub async fn login_or_register_with_email_code(
    state: &AppState,
    email: &str,
    code: &str,
    usertype: u8,
    did: u32,
    ctx: LoginContext<'_>,
) -> AppResult<(LoginResult, bool)> {
    use phpyun_core::verify::{self, VerifyKind};

    let email = email.trim().to_ascii_lowercase();
    if !verify::verify(&state.redis, VerifyKind::EmailLogin, &email, code).await? {
        auth_event("login_fail", Some("bad_email_code"));
        return Err(ApiError::param_invalid("email_code"));
    }

    let (user, is_new) = match user_repo::find_by_email_loose(state.db.pool(), &email).await? {
        Some(user) => (user, false),
        None => {
            let now = phpyun_core::clock::now_ts();
            let salt = uuid::Uuid::now_v7()
                .simple()
                .to_string()
                .chars()
                .take(16)
                .collect::<String>();
            let random_password = uuid::Uuid::new_v4().simple().to_string();
            let hash = argon2_hash_async(format!("{random_password}{salt}")).await?;
            let email_c = email.clone();
            let ip = ctx.ip.to_string();
            let uid = state
                .db
                .with_tx(|tx| {
                    Box::pin(async move {
                        let uid = user_repo::create_member(
                            &mut **tx,
                            &email_c,
                            &hash,
                            &salt,
                            None,
                            Some(&email_c),
                            usertype,
                            did,
                            &ip,
                            now,
                        )
                        .await?;
                        match usertype {
                            1 => resume_repo::ensure_row_in_tx(&mut **tx, uid, did, now).await?,
                            2 => company_repo::ensure_row(&mut **tx, uid, did).await?,
                            _ => {}
                        }
                        Ok::<u64, ApiError>(uid)
                    })
                })
                .await?;
            let user = user_repo::find_by_uid(state.db.pool(), uid)
                .await?
                .ok_or_else(|| {
                    ApiError::internal(std::io::Error::other("email registration lookup failed"))
                })?;
            auth_event("register_success", Some("email"));
            let _ = audit::emit(
                state,
                AuditEvent::new("user.register", Actor::uid(uid).with_ip(ctx.ip))
                    .target(format!("uid:{uid}"))
                    .meta(&serde_json::json!({ "regway": 3, "usertype": usertype, "did": did })),
            )
            .await;
            (user, true)
        }
    };

    if user.status == 2 {
        return Err(ApiError::locked());
    }
    rate_limit::clear_login_fail(&state.redis, &email).await;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(
        &state.config,
        user.uid,
        user.usertype as u8,
        user.did as u32,
    )?;
    let _ = user_session_service::record_login(
        state,
        LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: ctx.ip,
            ua: ctx.ua,
        },
    )
    .await;
    auth_event("login_success", Some("email"));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid).with_ip(ctx.ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": "email", "is_new": is_new })),
    )
    .await;

    Ok((
        LoginResult {
            access,
            refresh,
            uid: user.uid,
            usertype: user.usertype as u8,
            access_exp,
            refresh_exp,
        },
        is_new,
    ))
}

// ==================== Logout ====================

pub async fn logout(state: &AppState, access_jti: &str, access_exp: i64) -> AppResult<()> {
    jwt_blacklist::revoke(&state.redis, access_jti, access_exp).await?;
    let _ = user_session_service::revoke_current(state, access_jti).await;
    auth_event("logout", None);
    Ok(())
}

// ==================== Access-token rolling refresh ====================
//
// Sliding-session model: the client passes its current valid access_token via
// the Authorization header (validated by `AuthenticatedUser`), the server
// issues a fresh access_token with extended exp, and the old jti is added to
// the blacklist. Trade-off vs. the previous separate-refresh-token model: a
// leaked access_token can be rotated indefinitely until the session is
// kicked manually, but the client gets a single-token UX and never has to
// store a second long-lived secret.

pub async fn refresh_access(
    state: &AppState,
    user: &phpyun_core::AuthenticatedUser,
) -> AppResult<LoginResult> {
    // Defence-in-depth: the extractor already rejected revoked tokens, but
    // re-check after the bearer was issued (in case it was kicked between
    // request reception and this point).
    if jwt_blacklist::is_revoked(&state.redis, &user.jti).await {
        return Err(ApiError::session_expired());
    }
    if jwt_blacklist::is_token_stale(&state.redis, user.uid, user.iat).await {
        return Err(ApiError::session_expired());
    }

    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, user.usertype, user.did)?;

    // Match the session row by the OLD access jti (the client passed an
    // access_token, not a refresh_token). If the row is gone or revoked,
    // `rotate_on_access_refresh` returns `session_expired` and we refuse
    // to mint a new token — server-side session is the source of truth.
    user_session_service::rotate_on_access_refresh(
        state,
        &user.jti,
        &jti_access,
        &jti_refresh,
        access_exp,
        refresh_exp,
    )
    .await?;

    // Revoke the old access jti immediately (replay protection).
    let _ = jwt_blacklist::revoke(&state.redis, &user.jti, user.exp).await;

    auth_event("access_refreshed", None);

    Ok(LoginResult {
        access,
        refresh,
        uid: user.uid,
        usertype: user.usertype,
        access_exp,
        refresh_exp,
    })
}

// ==================== /me with L1+L2 caching ====================

/// Public-facing user summary exposed to the frontend (sensitive fields stripped).
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserProfile {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: u8,
    pub did: u32,
}

impl From<Member> for UserProfile {
    fn from(m: Member) -> Self {
        Self {
            uid: m.uid,
            username: m.username,
            email: m.email,
            moblie: m.moblie,
            usertype: m.usertype as u8,
            did: m.did as u32,
        }
    }
}

const PROFILE_SCOPE: &str = "user.profile";
const PROFILE_TTL_SECS: u64 = 60;

fn profile_key(uid: u64) -> String {
    format!("user:profile:{uid}")
}

/// L1 (moka) -> L2 (Redis) -> DB, with **singleflight** (N concurrent requests for the
/// same uid result in only one DB lookup).
pub async fn get_profile(state: &AppState, uid: u64) -> AppResult<Arc<UserProfile>> {
    // L1 fast path: hit returns a Value, deserialize and return immediately
    if let Some(hit) = state.cache.user.get(&uid).await {
        cache_hit("l1", PROFILE_SCOPE);
        return json::from_value::<UserProfile>((*hit).clone()).map(Arc::new);
    }

    // L1 miss: use moka try_get_with for singleflight semantics
    let kv = state.redis.clone();
    let reader = state.db.reader().clone();
    let key = profile_key(uid);
    let shared_val: Arc<json::Value> = state
        .cache
        .user
        .try_get_with(uid, async move {
            // L2
            if let Some(p) = kv.get_json::<UserProfile>(&key).await? {
                cache_hit("l2", PROFILE_SCOPE);
                return json::to_value(&p).map(Arc::new);
            }
            // Source of truth (via reader pool)
            cache_miss(PROFILE_SCOPE);
            let member = user_repo::find_by_uid(&reader, uid)
                .await?
                .ok_or(ApiError::business("user_not_found"))?;
            let profile: UserProfile = member.into();
            // Backfill L2 in the background (with backpressure; lossy writes do not block)
            kv.spawn_set_json_ex(key, &profile, PROFILE_TTL_SECS);
            json::to_value(&profile).map(Arc::new)
        })
        .await
        .map_err(ApiError::from_arc)?;

    json::from_value::<UserProfile>((*shared_val).clone()).map(Arc::new)
}

/// Invalidate the cache from the write path
pub async fn invalidate_profile(state: &AppState, uid: u64) {
    state.cache.user.invalidate(&uid).await;
    let _ = state.redis.del(&profile_key(uid)).await;
}

// ==================== First-time usertype selection ====================

/// One-shot user-type assignment for accounts that registered via OAuth and
/// haven't picked a role yet. Mirrors PHP `wap/login::setutype_action`:
/// allowed only when `usertype = 0`; sets it to 1/2/3 and seeds the
/// per-role satellite row (`phpyun_member_statis` for jobseeker,
/// `phpyun_company_statis` + `phpyun_company` shell for employer).
///
/// Idempotent on satellite rows (UPSERT). Returns `usertype_already_set` on conflict.
/// when a usertype is already chosen — caller should surface that as 409.
pub async fn set_usertype(state: &AppState, uid: u64, usertype: u8) -> AppResult<()> {
    if !matches!(usertype, 1 | 2 | 3) {
        return Err(ApiError::param_invalid("usertype"));
    }
    let pool = state.db.pool();
    let updated = user_repo::set_usertype_if_unset(pool, uid, usertype).await?;
    if updated == 0 {
        return Err(ApiError::business("usertype_already_set"));
    }
    seed_role_rows(state, uid, usertype).await;
    invalidate_profile(state, uid).await;
    Ok(())
}

async fn seed_role_rows(state: &AppState, uid: u64, usertype: u8) {
    let pool = state.db.pool();
    match usertype {
        1 => {
            let _ = phpyun_models::member_statis::repo::ensure_row(pool, uid).await;
            let _ = phpyun_models::resume::repo::ensure_uid_only(pool, uid).await;
        }
        2 | 3 => {
            let _ = phpyun_models::company_statis::repo::ensure_row(pool, uid).await;
            let _ = phpyun_models::company::repo::ensure_uid_only(pool, uid).await;
        }
        _ => {}
    }
}
