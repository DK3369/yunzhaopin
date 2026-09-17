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
use phpyun_core::extractors::{USERTYPE_ADMIN, USERTYPE_EMPLOYER};
use phpyun_core::jwt::{issue_pair_ex, JwtIssued};
use phpyun_core::{
    background, clock, jwt_blacklist,
    metrics::{auth_event, cache_hit, cache_miss},
    rate_limit, ApiError, AppResult, AppState,
};
use phpyun_models::admin_msg::repo as admin_msg_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::user::{entity::Member, repo as user_repo};

use crate::user_session_service::{self, LoginRecord};

fn auth_identity(user: &Member) -> AppResult<(u8, u32)> {
    Ok((
        phpyun_core::numeric::checked_db(user.usertype, "phpyun_member.usertype")?,
        phpyun_core::numeric::checked_db(user.did, "phpyun_member.did")?,
    ))
}

struct LoginIdentity {
    token_uid: u64,
    hr_uid: Option<u64>,
    usertype: u8,
    did: u32,
}

async fn identity_for_login(state: &AppState, user: &Member) -> AppResult<LoginIdentity> {
    let (usertype, did) = auth_identity(user)?;
    if usertype == USERTYPE_EMPLOYER && user.pid > 0 && user.pid != user.uid {
        let parent = user_repo::find_by_uid(state.db.reader(), user.pid)
            .await?
            .ok_or_else(|| ApiError::business("sub_account_not_found"))?;
        if parent.status == 2 {
            return Err(ApiError::locked());
        }
        let (_, parent_did) = auth_identity(&parent)?;
        return Ok(LoginIdentity {
            token_uid: parent.uid,
            hr_uid: Some(user.uid),
            usertype: USERTYPE_EMPLOYER,
            did: parent_did,
        });
    }
    Ok(LoginIdentity {
        token_uid: user.uid,
        hr_uid: None,
        usertype,
        did,
    })
}

async fn issue_login_tokens(
    state: &AppState,
    user: &Member,
    ctx: &LoginContext<'_>,
) -> AppResult<LoginResult> {
    let ident = identity_for_login(state, user).await?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair_ex(
        &state.config,
        ident.token_uid,
        ident.usertype,
        ident.did,
        ident.hr_uid,
    )?;
    let _ = user_session_service::record_login(
        state,
        LoginRecord {
            uid: user.uid,
            usertype: ident.usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: ctx.ip,
            ua: ctx.ua,
        },
    )
    .await;
    record_member_login_log(state, user, ctx, &ident).await;
    Ok(LoginResult {
        access,
        refresh,
        uid: ident.token_uid,
        usertype: ident.usertype,
        access_exp,
        refresh_exp,
    })
}

/// Login context — carried alongside credentials so the service can record
/// device fingerprint + IP into `phpyun_user_session`. Empty strings are OK
/// (e.g. internal admin tooling).
#[derive(Debug, Default, Clone)]
pub struct LoginContext<'a> {
    pub ip: &'a str,
    pub ua: &'a str,
}

fn is_wap_ua(ua: &str) -> bool {
    let l = ua.to_ascii_lowercase();
    l.contains("mobile")
        || l.contains("android")
        || l.contains("iphone")
        || l.contains("ipad")
        || l.contains("ipod")
        || l.contains("harmonyos")
        || l.contains("okhttp")
        || l.contains("micromessenger")
        || l.contains("miniprogram")
        || l.contains("uni-app")
        || l.contains("wap")
}

/// Numbered keys only. Never concatenate Chinese like PHP `端口延续登录`.
fn login_log_content(ua: &str, continued: bool) -> &'static str {
    let wap = is_wap_ua(ua);
    if continued {
        if wap {
            "wap_01557admin_tool_00738"
        } else {
            "admin_tool_00739"
        }
    } else if wap {
        "common_06521"
    } else {
        "common_06522"
    }
}

async fn record_member_login_log(
    state: &AppState,
    user: &Member,
    ctx: &LoginContext<'_>,
    ident: &LoginIdentity,
) {
    if ident.usertype == USERTYPE_ADMIN {
        return;
    }
    let now = clock::now_ts();
    let prev = user.login_date.unwrap_or(0);
    let continued = prev >= clock::start_of_today();
    let content = login_log_content(ctx.ua, continued);
    let usertype = i32::from(ident.usertype);
    if !continued {
        if let Err(e) = crate::integral_grant_service::grant(
            state,
            user.uid,
            usertype,
            "integral_login",
            "wap_00555",
            0,
        )
        .await
        {
            tracing::warn!(?e, uid = user.uid, "login integral grant failed");
        }
    }
    let did = i32::try_from(ident.did).unwrap_or(0);
    let _ = admin_msg_repo::insert_php_login_log(
        state.db.pool(),
        ident.token_uid,
        usertype,
        content,
        ctx.ip,
        now,
        0,
        did,
    )
    .await;
    let _ = user_repo::touch_login(state.db.pool(), user.uid, ctx.ip, now).await;
}

use std::sync::Arc;
use std::time::Duration;

const EMAIL_LOGIN_TTL_SECS: u64 = 600;
const DEV_EMAIL_LOGIN_CODE: &str = "111111";

// ==================== Login ====================

/// PHP `jycheck(..., wap_js_00062)`：`code_web` 勾了「前台登录」必须验码；
/// 账号连续失败 ≥3 次也强制图形码（不依赖后台开关）。
pub async fn password_login_needs_captcha(state: &AppState, account: &str) -> AppResult<bool> {
    let v = crate::site_gate_service::config_str(state, "code_web").await;
    if v.contains("前台登录") || v.contains("wap_js_00062") {
        return Ok(true);
    }
    let fails = rate_limit::login_fail_count(&state.redis, account).await?;
    Ok(rate_limit::login_fail_requires_captcha(fails))
}

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
    // 1. Pre-check the login-failure counter (distributed via Redis).
    //    Peek only — increment happens after a real failed attempt.
    rate_limit::check_login_fail(&state.redis, account).await?;
    rate_limit::check_login_fail_ip(&state.redis, ctx.ip).await?;

    // 2. Look up the user (use the reader pool to offload the writer)
    let user: Member = match user_repo::find_for_login(state.db.reader(), account).await? {
        Some(u) => u,
        None => {
            auth_event("login_fail", Some("not_found"));
            rate_limit::record_login_fail(&state.redis, account).await;
            rate_limit::record_login_fail_ip(&state.redis, ctx.ip).await;
            return Err(ApiError::bad_credentials());
        }
    };

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
        rate_limit::record_login_fail(&state.redis, account).await;
        rate_limit::record_login_fail_ip(&state.redis, ctx.ip).await;
        return Err(ApiError::bad_credentials());
    }

    // 4. Login succeeded: clear the failure counter
    rate_limit::clear_login_fail(&state.redis, account).await;
    rate_limit::clear_login_fail_ip(&state.redis, ctx.ip).await;

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

    let tokens = issue_login_tokens(state, &user, &ctx).await?;

    auth_event("login_success", None);

    // Audit: write to DB + publish to the event bus (failure does not block login)
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid))
            .target(format!("uid:{}", user.uid))
            .success(true),
    )
    .await;

    Ok(tokens)
}

/// Issue a session for an already-identified member (WeChat scan-to-login).
pub async fn login_by_uid(
    state: &AppState,
    uid: u64,
    ctx: LoginContext<'_>,
) -> AppResult<LoginResult> {
    let user: Member = user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(ApiError::bad_credentials)?;
    if user.status == 2 {
        return Err(ApiError::locked());
    }
    Ok(issue_login_tokens(state, &user, &ctx).await?)
}

/// Admin simulate-login: issue a member JWT without password (PHP 模拟登录).
pub async fn impersonate(
    state: &AppState,
    actor_uid: u64,
    target_uid: u64,
    ctx: LoginContext<'_>,
) -> AppResult<LoginResult> {
    if target_uid == actor_uid {
        return Err(ApiError::param_invalid("cannot_impersonate_self"));
    }
    let user: Member = user_repo::find_by_uid(state.db.reader(), target_uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("user_not_found"))?;
    if user.usertype == i32::from(USERTYPE_ADMIN) {
        return Err(ApiError::forbidden());
    }
    if user.status == 2 {
        return Err(ApiError::locked());
    }
    let tokens = issue_login_tokens(state, &user, &ctx).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.user.impersonate", Actor::uid(actor_uid))
            .target(format!("uid:{target_uid}"))
            .success(true),
    )
    .await;
    Ok(tokens)
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
    //    credential-stuffing). Peek only; increment after a real failed attempt.
    rate_limit::check_login_fail(&state.redis, mobile).await?;
    rate_limit::check_login_fail_ip(&state.redis, ctx.ip).await?;

    // 2. Verify the SMS code
    if !verify::verify(&state.redis, VerifyKind::SmsLogin, mobile, sms_code).await? {
        auth_event("login_fail", Some("bad_sms_code"));
        rate_limit::record_login_fail(&state.redis, mobile).await;
        rate_limit::record_login_fail_ip(&state.redis, ctx.ip).await;
        return Err(ApiError::bad_credentials());
    }

    // 3. Look up the user by phone number
    let user = match user_repo::find_by_mobile(state.db.reader(), mobile).await? {
        Some(u) => u,
        None => {
            auth_event("login_fail", Some("not_found"));
            return Err(ApiError::business("need_register"));
        }
    };
    if user.status == 2 {
        auth_event("login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    // 4. Clear the failure counter
    rate_limit::clear_login_fail(&state.redis, mobile).await;
    rate_limit::clear_login_fail_ip(&state.redis, ctx.ip).await;

    let tokens = issue_login_tokens(state, &user, &ctx).await?;
    auth_event("login_success", Some("sms"));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": "sms" })),
    )
    .await;
    Ok(tokens)
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

    // Laptop-only fallback: `DEV_TOKENS=1` skips the MTA and uses a known code.
    // Public `APP_ENV=dev` debug binaries must still send real mail.
    let use_dev_code = state.config.dev_tokens && state.config.env.is_dev_or_test();
    let code = if use_dev_code {
        DEV_EMAIL_LOGIN_CODE.to_string()
    } else {
        verify::gen_digit_code(6)
    };
    if use_dev_code {
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

    rate_limit::check_login_fail_ip(&state.redis, ctx.ip).await?;

    let email = email.trim().to_ascii_lowercase();
    if !verify::verify(&state.redis, VerifyKind::EmailLogin, &email, code).await? {
        auth_event("login_fail", Some("bad_email_code"));
        rate_limit::record_login_fail_ip(&state.redis, ctx.ip).await;
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
            let random_password = uuid::Uuid::now_v7().simple().to_string();
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
    let tokens = issue_login_tokens(state, &user, &ctx).await?;
    auth_event("login_success", Some("email"));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid).with_ip(ctx.ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": "email", "is_new": is_new })),
    )
    .await;

    Ok((tokens, is_new))
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
    if let Some(hr) = user.hr_uid.filter(|n| *n > 0) {
        if jwt_blacklist::is_token_stale(&state.redis, hr, user.iat).await {
            return Err(ApiError::session_expired());
        }
    }

    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair_ex(
        &state.config,
        user.uid,
        user.usertype,
        user.did,
        user.hr_uid,
    )?;

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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct UserProfile {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: u8,
    pub did: u32,
}

impl TryFrom<Member> for UserProfile {
    type Error = ApiError;

    fn try_from(m: Member) -> Result<Self, Self::Error> {
        let (usertype, did) = auth_identity(&m)?;
        Ok(Self {
            uid: m.uid,
            username: m.username,
            email: m.email,
            moblie: m.moblie,
            usertype,
            did,
        })
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
            let profile = UserProfile::try_from(member)?;
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
    if !matches!(usertype, 1..=3) {
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

#[cfg(test)]
mod conversion_tests {
    use super::*;

    fn member(usertype: i32, did: u64) -> Member {
        Member {
            uid: 1,
            username: "tester".to_owned(),
            password: String::new(),
            salt: String::new(),
            email: None,
            moblie: None,
            usertype,
            status: 1,
            did,
            reg_date: 0,
            login_date: None,
            pid: 0,
        }
    }

    #[test]
    fn invalid_database_usertype_becomes_db_500() {
        let error = UserProfile::try_from(member(-1, 0)).unwrap_err();
        assert_eq!(error.tag(), "db");
        assert!(error.to_string().contains("phpyun_member.usertype"));
        assert_eq!(error.http_status().as_u16(), 500);
    }

    #[test]
    fn invalid_database_did_becomes_db_500() {
        let did = u64::from(u32::MAX) + 1;
        let error = UserProfile::try_from(member(1, did)).unwrap_err();
        assert_eq!(error.tag(), "db");
        assert!(error.to_string().contains("phpyun_member.did"));
        assert_eq!(error.http_status().as_u16(), 500);
    }

    #[test]
    fn login_log_content_pc_success() {
        assert_eq!(
            login_log_content("Mozilla/5.0 (Windows NT 10.0; Win64; x64)", false),
            "common_06522"
        );
    }

    #[test]
    fn login_log_content_wap_success() {
        assert_eq!(
            login_log_content("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)", false),
            "common_06521"
        );
    }

    #[test]
    fn login_log_content_pc_continue() {
        assert_eq!(
            login_log_content("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)", true),
            "admin_tool_00739"
        );
    }

    #[test]
    fn login_log_content_wap_continue() {
        assert_eq!(
            login_log_content("okhttp/4.9.3", true),
            "wap_01557admin_tool_00738"
        );
    }
}
