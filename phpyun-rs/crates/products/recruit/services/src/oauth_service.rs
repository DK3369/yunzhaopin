//! Third-party login (Google / Facebook / Apple).
//!
//! Flow:
//! 1. The client SDK obtains an `id_token` -> sends `POST /v1/wap/oauth/{provider}/login`
//! 2. The server calls `state.oauth.verify(kind, id_token)` to extract sub / email / name
//! 3. Look up the member by sub:
//!    - **Exists**: issue tokens, log the user in directly, and return success
//!    - **Does not exist**: stash `{provider, sub}` in Redis under a short-lived
//!      ticket and return `need_bind` so the client can bind an existing account
//!      or run fast-reg (PHP `wap/qqconnect` bind page).

use phpyun_auth::argon2_hash_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt::{issue_pair, JwtIssued};
use phpyun_core::metrics::auth_event;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{clock, ApiError, AppResult, AppState, ProviderKind};
use phpyun_models::company::repo as company_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::user::{entity::Member, repo as user_repo};
use uuid::Uuid;

const PENDING_PREFIX: &str = "oauth:pending:";
const PENDING_TTL_SECS: u64 = 600;

fn auth_identity(user: &Member) -> AppResult<(u8, u32)> {
    Ok((
        phpyun_core::numeric::checked_db(user.usertype, "phpyun_member.usertype")?,
        phpyun_core::numeric::checked_db(user.did, "phpyun_member.did")?,
    ))
}

pub struct OAuthLoginResult {
    pub uid: u64,
    pub usertype: u8,
    pub access: String,
    pub refresh: String,
    pub access_exp: i64,
    pub refresh_exp: i64,
    /// `sub` extracted from the token; echoed back to the client on business failure so the next
    /// step (binding) is easy.
    pub provider_sub: String,
    pub email_from_provider: Option<String>,
    pub name_from_provider: Option<String>,
    /// When true the client should send the user to `/oauth-bind?ticket=`.
    pub need_bind: bool,
    pub ticket: String,
    pub provider: String,
}

async fn pending_not_bound(
    state: &AppState,
    provider: &str,
    sub: &str,
) -> AppResult<OAuthLoginResult> {
    auth_event("oauth_not_bound", None);
    let ticket = Uuid::now_v7().simple().to_string();
    let payload = format!("{provider}\t{sub}");
    state
        .redis
        .set_ex(
            &format!("{PENDING_PREFIX}{ticket}"),
            &payload,
            PENDING_TTL_SECS,
        )
        .await?;
    Ok(OAuthLoginResult {
        uid: 0,
        usertype: 0,
        access: String::new(),
        refresh: String::new(),
        access_exp: 0,
        refresh_exp: 0,
        provider_sub: sub.to_string(),
        email_from_provider: None,
        name_from_provider: None,
        need_bind: true,
        ticket,
        provider: provider.to_string(),
    })
}

async fn load_pending(state: &AppState, ticket: &str) -> AppResult<(String, String)> {
    let raw = state
        .redis
        .get_str(&format!("{PENDING_PREFIX}{ticket}"))
        .await?
        .ok_or_else(|| ApiError::param_invalid("oauth_ticket_expired"))?;
    let mut parts = raw.splitn(2, '\t');
    let provider = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiError::param_invalid("oauth_ticket_expired"))?;
    let sub = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiError::param_invalid("oauth_ticket_expired"))?;
    Ok((provider.to_string(), sub.to_string()))
}

async fn consume_pending(state: &AppState, ticket: &str) {
    let _ = state.redis.del(&format!("{PENDING_PREFIX}{ticket}")).await;
}

/// AppId / secret / redirect for the PHP-style code OAuth providers.
/// Admin `工具 → 登陆` writes `sy_*` into `phpyun_admin_config`; env is fallback only.
pub struct OauthCodeApp {
    pub appid: String,
    pub appsecret: String,
    pub redirect: String,
}

async fn setting_val(state: &AppState, key: &str) -> String {
    match setting_repo::find(state.db.reader(), key).await {
        Ok(Some(s)) => s.value,
        _ => String::new(),
    }
}

fn switch_on(raw: &str) -> bool {
    matches!(raw.trim(), "1" | "true" | "yes" | "on")
}

fn first_nonempty(db: &str, env: Option<&str>) -> String {
    let db = db.trim();
    if !db.is_empty() {
        return db.to_string();
    }
    env.unwrap_or("").trim().to_string()
}

async fn login_page_redirect(state: &AppState, env_redirect: Option<&str>) -> String {
    let env = env_redirect.unwrap_or("").trim();
    if !env.is_empty() {
        return env.to_string();
    }
    let env_base = state.config.web_base_url.as_deref().unwrap_or("").trim();
    let base = if !env_base.is_empty() {
        env_base.to_string()
    } else {
        setting_val(state, "sy_weburl").await
    };
    let base = base.trim().trim_end_matches('/');
    if base.is_empty() {
        String::new()
    } else {
        format!("{base}/login")
    }
}

async fn load_code_app(
    state: &AppState,
    enable_key: &str,
    closed_key: &'static str,
    db_id: &str,
    db_secret: &str,
    env_id: Option<&str>,
    env_secret: Option<&str>,
    env_redirect: Option<&str>,
    missing_id: &'static str,
    missing_secret: &'static str,
    missing_redirect: &'static str,
) -> AppResult<OauthCodeApp> {
    if !switch_on(&setting_val(state, enable_key).await) {
        return Err(ApiError::param_invalid(closed_key));
    }
    let appid = first_nonempty(&setting_val(state, db_id).await, env_id);
    let appsecret = first_nonempty(&setting_val(state, db_secret).await, env_secret);
    let redirect = login_page_redirect(state, env_redirect).await;
    if appid.is_empty() {
        return Err(ApiError::param_invalid(missing_id));
    }
    if appsecret.is_empty() {
        return Err(ApiError::param_invalid(missing_secret));
    }
    if redirect.is_empty() {
        return Err(ApiError::param_invalid(missing_redirect));
    }
    Ok(OauthCodeApp {
        appid,
        appsecret,
        redirect,
    })
}

pub async fn qq_code_app(state: &AppState) -> AppResult<OauthCodeApp> {
    load_code_app(
        state,
        "sy_qqlogin",
        "qq_login_closed",
        "sy_qqappid",
        "sy_qqappkey",
        state.config.qq_appid.as_deref(),
        state.config.qq_appsecret.as_deref(),
        state.config.qq_oauth_redirect.as_deref(),
        "qq_appid_missing",
        "qq_appsecret_missing",
        "qq_oauth_redirect_missing",
    )
    .await
}

pub async fn weibo_code_app(state: &AppState) -> AppResult<OauthCodeApp> {
    load_code_app(
        state,
        "sy_sinalogin",
        "weibo_login_closed",
        "sy_sinaappid",
        "sy_sinaappkey",
        state.config.weibo_appid.as_deref(),
        state.config.weibo_appsecret.as_deref(),
        state.config.weibo_oauth_redirect.as_deref(),
        "weibo_appid_missing",
        "weibo_appsecret_missing",
        "weibo_oauth_redirect_missing",
    )
    .await
}

pub async fn google_code_app(state: &AppState) -> AppResult<OauthCodeApp> {
    load_code_app(
        state,
        "sy_googlelogin",
        "google_login_closed",
        "sy_googleappid",
        "sy_googleappkey",
        None,
        None,
        None,
        "google_appid_missing",
        "google_appsecret_missing",
        "google_oauth_redirect_missing",
    )
    .await
}

pub async fn facebook_code_app(state: &AppState) -> AppResult<OauthCodeApp> {
    load_code_app(
        state,
        "sy_facebooklogin",
        "facebook_login_closed",
        "sy_facebookappid",
        "sy_facebookappkey",
        None,
        None,
        None,
        "facebook_appid_missing",
        "facebook_appsecret_missing",
        "facebook_oauth_redirect_missing",
    )
    .await
}

async fn login_bound_oauth_member(
    state: &AppState,
    user: Member,
    provider: &'static str,
    sub: String,
    email_from_provider: Option<String>,
    name_from_provider: Option<String>,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }
    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_success", Some(provider));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": provider })),
    )
    .await;

    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: sub,
        email_from_provider,
        name_from_provider,
        need_bind: false,
        ticket: String::new(),
        provider: provider.to_string(),
    })
}

pub async fn login_with_oauth(
    state: &AppState,
    provider: ProviderKind,
    id_token: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    // 1. Verify the id_token
    let identity = state.oauth.verify(provider, id_token).await?;

    // 2. Look up the user
    let member =
        user_repo::find_by_oauth_id(state.db.reader(), provider.member_column(), &identity.sub)
            .await?;

    let Some(user) = member else {
        return pending_not_bound(state, provider.as_str(), &identity.sub).await;
    };

    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    // 3. Issue tokens
    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_success", Some(provider.as_str()));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": provider.as_str() })),
    )
    .await;

    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: identity.sub,
        email_from_provider: identity.email,
        name_from_provider: identity.name,
        need_bind: false,
        ticket: String::new(),
        provider: provider.as_str().to_string(),
    })
}

// ==================== WeChat snsapi_base login (code -> openid) ====================

/// Call `https://api.weixin.qq.com/sns/oauth2/access_token` to exchange the code for an openid.
/// Mirrors PHPYun `wxoauth_controller::index_action`.
pub async fn login_with_wechat_code(
    state: &AppState,
    code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let appid = state
        .config
        .wechat_appid
        .as_deref()
        .ok_or_else(|| ApiError::param_invalid("wechat_appid_missing"))?;
    let appsecret = state
        .config
        .wechat_appsecret
        .as_deref()
        .ok_or_else(|| ApiError::param_invalid("wechat_appsecret_missing"))?;

    // 1) Get the openid
    let url = format!(
        "https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
        urlencoding_minimal(appid),
        urlencoding_minimal(appsecret),
        urlencoding_minimal(code),
    );

    #[derive(serde::Deserialize)]
    struct WxTokenResp {
        #[serde(default)]
        openid: Option<String>,
        #[serde(default)]
        unionid: Option<String>,
        #[serde(default)]
        errcode: Option<i64>,
        #[serde(default)]
        errmsg: Option<String>,
    }

    let resp: WxTokenResp = state.http.get_json(&url).await?;
    if let Some(code) = resp.errcode {
        if code != 0 {
            let msg = resp.errmsg.unwrap_or_default();
            return Err(ApiError::upstream(format!(
                "wechat errcode={code} errmsg={msg}"
            )));
        }
    }
    let Some(openid) = resp.openid else {
        return Err(ApiError::upstream("wechat oauth returned no openid"));
    };

    // 2) Look up the member by openid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "wxid", &openid).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "wechat", &openid).await;
    };

    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_success", Some("wechat"));
    let _ = audit::emit(
        state,
        AuditEvent::new("user.login", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "via": "wechat", "openid": openid })),
    )
    .await;

    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: openid,
        email_from_provider: resp.unionid, // reuse the email field to pass back the unionid for the client to retain
        name_from_provider: None,
        need_bind: false,
        ticket: String::new(),
        provider: "wechat".to_string(),
    })
}

/// Build the snsapi_base authorization URL (for the front end to redirect to).
///
/// `redirect_uri` must be a callback URL on a domain configured in the WeChat Official Account backend.
pub fn wechat_authorize_url(appid: &str, redirect_uri: &str, state_val: &str) -> String {
    format!(
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid={appid}&redirect_uri={redir}&response_type=code&scope=snsapi_base&state={state}#wechat_redirect",
        appid = urlencoding_minimal(appid),
        redir = urlencoding_minimal(redirect_uri),
        state = urlencoding_minimal(state_val),
    )
}

// ==================== QQ Connect (code → access_token → openid) ====================

/// Mirrors PHPYun `wap/qqconnect::qqlogin_action` — exchange `code` for an
/// `access_token`, then call `/oauth2.0/me` to fetch the openid; finally look
/// up the bound member by `qqid`. Identical control flow to `login_with_wechat_code`.
pub async fn login_with_qq_code(
    state: &AppState,
    code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let app = qq_code_app(state).await?;
    let appid = app.appid.as_str();
    let appsecret = app.appsecret.as_str();
    let redirect = app.redirect.as_str();

    // 1) /oauth2.0/token returns text in url-encoded form: access_token=xxx&expires_in=7776000&refresh_token=yyy
    let token_url = format!(
        "https://graph.qq.com/oauth2.0/token?grant_type=authorization_code&client_id={appid}&client_secret={secret}&code={code}&redirect_uri={redir}&fmt=json",
        appid = urlencoding_minimal(appid),
        secret = urlencoding_minimal(appsecret),
        code = urlencoding_minimal(code),
        redir = urlencoding_minimal(redirect),
    );

    #[derive(serde::Deserialize)]
    struct QqTokenResp {
        #[serde(default)]
        access_token: Option<String>,
        #[serde(default)]
        error: Option<i64>,
        #[serde(default)]
        error_description: Option<String>,
    }

    let resp: QqTokenResp = state.http.get_json(&token_url).await?;
    if let Some(err) = resp.error {
        let msg = resp.error_description.unwrap_or_default();
        return Err(ApiError::upstream(format!(
            "qq token error={err} msg={msg}"
        )));
    }
    let Some(access_token) = resp.access_token else {
        return Err(ApiError::upstream("qq oauth returned no access_token"));
    };

    // 2) /oauth2.0/me with fmt=json returns {"client_id": "...", "openid": "..."}
    let me_url = format!(
        "https://graph.qq.com/oauth2.0/me?access_token={tok}&fmt=json",
        tok = urlencoding_minimal(&access_token),
    );
    #[derive(serde::Deserialize)]
    struct QqMeResp {
        #[serde(default)]
        openid: Option<String>,
        #[serde(default)]
        unionid: Option<String>,
        #[serde(default)]
        error: Option<i64>,
        #[serde(default)]
        error_description: Option<String>,
    }
    let me: QqMeResp = state.http.get_json(&me_url).await?;
    if let Some(err) = me.error {
        let msg = me.error_description.unwrap_or_default();
        return Err(ApiError::upstream(format!("qq /me error={err} msg={msg}")));
    }
    let Some(openid) = me.openid else {
        return Err(ApiError::upstream("qq oauth returned no openid"));
    };

    // 3) Look up bound member by qqid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "qqid", &openid).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "qq", &openid).await;
    };
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_success", Some("qq"));
    let _ = audit::emit(
        state,
        AuditEvent::new("oauth.login", Actor::uid(user.uid))
            .meta(&serde_json::json!({ "via": "qq", "openid": openid })),
    )
    .await;

    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: openid,
        email_from_provider: me.unionid,
        name_from_provider: None,
        need_bind: false,
        ticket: String::new(),
        provider: "qq".to_string(),
    })
}

pub fn qq_authorize_url(appid: &str, redirect_uri: &str, state_val: &str) -> String {
    format!(
        "https://graph.qq.com/oauth2.0/authorize?response_type=code&client_id={appid}&redirect_uri={redir}&state={state}",
        appid = urlencoding_minimal(appid),
        redir = urlencoding_minimal(redirect_uri),
        state = urlencoding_minimal(state_val),
    )
}

// ==================== Weibo (Sina) (code → access_token + uid) ====================

/// Mirrors PHPYun `wap/sinaconnect` — Weibo uses `oauth2/access_token` (POST)
/// to exchange the code for `(access_token, uid)`. Then we look up the bound
/// member by `sinaid`.
pub async fn login_with_weibo_code(
    state: &AppState,
    code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let app = weibo_code_app(state).await?;
    let appid = app.appid.as_str();
    let appsecret = app.appsecret.as_str();
    let redirect = app.redirect.as_str();

    // Weibo expects POST application/x-www-form-urlencoded.
    let body = format!(
        "client_id={cid}&client_secret={sec}&grant_type=authorization_code&code={code}&redirect_uri={redir}",
        cid = urlencoding_minimal(appid),
        sec = urlencoding_minimal(appsecret),
        code = urlencoding_minimal(code),
        redir = urlencoding_minimal(redirect),
    );

    #[derive(serde::Deserialize)]
    struct WeiboTokenResp {
        #[serde(default)]
        access_token: Option<String>,
        /// Weibo returns `uid` as a stringified integer.
        #[serde(default)]
        uid: Option<String>,
        #[serde(default)]
        error: Option<String>,
        #[serde(default)]
        error_code: Option<i64>,
        #[serde(default)]
        error_description: Option<String>,
    }

    let resp: WeiboTokenResp = state
        .http
        .post_form_to_json("https://api.weibo.com/oauth2/access_token", &body)
        .await?;
    if let Some(err) = resp.error {
        if !err.is_empty() {
            let msg = resp
                .error_description
                .or_else(|| resp.error_code.map(|c| c.to_string()))
                .unwrap_or_default();
            return Err(ApiError::upstream(format!("weibo error={err} msg={msg}")));
        }
    }
    let Some(uid_str) = resp.uid else {
        return Err(ApiError::upstream("weibo oauth returned no uid"));
    };

    // Look up the bound member by sinaid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "sinaid", &uid_str).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "weibo", &uid_str).await;
    };
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_success", Some("weibo"));
    let _ = audit::emit(
        state,
        AuditEvent::new("oauth.login", Actor::uid(user.uid))
            .meta(&serde_json::json!({ "via": "weibo", "uid": uid_str })),
    )
    .await;

    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: uid_str,
        email_from_provider: resp.access_token,
        name_from_provider: None,
        need_bind: false,
        ticket: String::new(),
        provider: "weibo".to_string(),
    })
}

pub fn weibo_authorize_url(appid: &str, redirect_uri: &str, state_val: &str) -> String {
    format!(
        "https://api.weibo.com/oauth2/authorize?client_id={appid}&redirect_uri={redir}&response_type=code&state={state}",
        appid = urlencoding_minimal(appid),
        redir = urlencoding_minimal(redirect_uri),
        state = urlencoding_minimal(state_val),
    )
}

pub fn google_authorize_url(appid: &str, redirect_uri: &str, state_val: &str) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={appid}&redirect_uri={redir}&response_type=code&scope={scope}&state={state}&access_type=online&prompt=select_account",
        appid = urlencoding_minimal(appid),
        redir = urlencoding_minimal(redirect_uri),
        scope = urlencoding_minimal("openid email profile"),
        state = urlencoding_minimal(state_val),
    )
}

pub fn facebook_authorize_url(appid: &str, redirect_uri: &str, state_val: &str) -> String {
    format!(
        "https://www.facebook.com/v21.0/dialog/oauth?client_id={appid}&redirect_uri={redir}&state={state}&scope={scope}",
        appid = urlencoding_minimal(appid),
        redir = urlencoding_minimal(redirect_uri),
        state = urlencoding_minimal(state_val),
        scope = urlencoding_minimal("email,public_profile"),
    )
}

pub async fn login_with_google_code(
    state: &AppState,
    code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let app = google_code_app(state).await?;
    let body = format!(
        "code={code}&client_id={cid}&client_secret={sec}&redirect_uri={redir}&grant_type=authorization_code",
        code = urlencoding_minimal(code),
        cid = urlencoding_minimal(&app.appid),
        sec = urlencoding_minimal(&app.appsecret),
        redir = urlencoding_minimal(&app.redirect),
    );
    #[derive(serde::Deserialize)]
    struct GoogleTokenResp {
        #[serde(default)]
        access_token: Option<String>,
        #[serde(default)]
        error: Option<String>,
        #[serde(default)]
        error_description: Option<String>,
    }
    let token: GoogleTokenResp = state
        .http
        .post_form_to_json("https://oauth2.googleapis.com/token", &body)
        .await?;
    if let Some(err) = token.error.filter(|s| !s.is_empty()) {
        let msg = token.error_description.unwrap_or_default();
        return Err(ApiError::upstream(format!("google token error={err} msg={msg}")));
    }
    let Some(access_token) = token.access_token.filter(|s| !s.is_empty()) else {
        return Err(ApiError::upstream("google oauth returned no access_token"));
    };
    #[derive(serde::Deserialize)]
    struct GoogleUser {
        #[serde(default)]
        sub: Option<String>,
        #[serde(default)]
        email: Option<String>,
        #[serde(default)]
        name: Option<String>,
    }
    let me_url = format!(
        "https://www.googleapis.com/oauth2/v3/userinfo?access_token={tok}",
        tok = urlencoding_minimal(&access_token),
    );
    let me: GoogleUser = state.http.get_json(&me_url).await?;
    let Some(sub) = me.sub.filter(|s| !s.is_empty()) else {
        return Err(ApiError::upstream("google oauth returned no sub"));
    };
    let member = user_repo::find_by_oauth_id(state.db.reader(), "google", &sub).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "google", &sub).await;
    };
    login_bound_oauth_member(
        state,
        user,
        "google",
        sub,
        me.email,
        me.name,
        client_ip,
        user_agent,
    )
    .await
}

pub async fn login_with_facebook_code(
    state: &AppState,
    code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let app = facebook_code_app(state).await?;
    let token_url = format!(
        "https://graph.facebook.com/v21.0/oauth/access_token?client_id={cid}&redirect_uri={redir}&client_secret={sec}&code={code}",
        cid = urlencoding_minimal(&app.appid),
        redir = urlencoding_minimal(&app.redirect),
        sec = urlencoding_minimal(&app.appsecret),
        code = urlencoding_minimal(code),
    );
    #[derive(serde::Deserialize)]
    struct FbTokenResp {
        #[serde(default)]
        access_token: Option<String>,
        #[serde(default)]
        error: Option<serde_json::Value>,
    }
    let token: FbTokenResp = state.http.get_json(&token_url).await?;
    if token.error.is_some() {
        return Err(ApiError::upstream("facebook token error"));
    }
    let Some(access_token) = token.access_token.filter(|s| !s.is_empty()) else {
        return Err(ApiError::upstream("facebook oauth returned no access_token"));
    };
    #[derive(serde::Deserialize)]
    struct FbUser {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        email: Option<String>,
        #[serde(default)]
        name: Option<String>,
    }
    let me_url = format!(
        "https://graph.facebook.com/me?fields=id,name,email&access_token={tok}",
        tok = urlencoding_minimal(&access_token),
    );
    let me: FbUser = state.http.get_json(&me_url).await?;
    let Some(sub) = me.id.filter(|s| !s.is_empty()) else {
        return Err(ApiError::upstream("facebook oauth returned no id"));
    };
    let member = user_repo::find_by_oauth_id(state.db.reader(), "facebook", &sub).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "facebook", &sub).await;
    };
    login_bound_oauth_member(
        state,
        user,
        "facebook",
        sub,
        me.email,
        me.name,
        client_ip,
        user_agent,
    )
    .await
}

/// Minimal URL encoding — only escapes characters that would break the WeChat URL syntax,
/// avoiding the need to pull in a new `urlencoding` / `percent-encoding` crate.
fn urlencoding_minimal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(b));
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{b:02X}"));
            }
        }
    }
    out
}

/// Bind a third-party identity to a logged-in account (the client must complete a normal login first,
/// then call this endpoint).
pub async fn bind_oauth(
    state: &AppState,
    uid: u64,
    provider: ProviderKind,
    id_token: &str,
    client_ip: &str,
) -> AppResult<()> {
    let identity = state.oauth.verify(provider, id_token).await?;

    // This sub must not already be bound to a different user
    if let Some(other) =
        user_repo::find_by_oauth_id(state.db.reader(), provider.as_str(), &identity.sub)
            .await?
    {
        if other.uid != uid {
            return Err(ApiError::param_invalid("oauth_sub_bound_elsewhere"));
        }
    }

    user_repo::bind_oauth_id(
        state.db.pool(),
        uid,
        provider.as_str(),
        &identity.sub,
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("user.oauth_bind", Actor::uid(uid).with_ip(client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({ "provider": provider.as_str() })),
    )
    .await;

    Ok(())
}

/// Bind a pending third-party identity (from Redis ticket) to the logged-in uid.
pub async fn bind_pending(
    state: &AppState,
    uid: u64,
    ticket: &str,
    client_ip: &str,
) -> AppResult<()> {
    let (provider, sub) = load_pending(state, ticket).await?;
    if let Some(other) = user_repo::find_by_oauth_id(state.db.reader(), &provider, &sub).await? {
        if other.uid != uid {
            return Err(ApiError::param_invalid("oauth_sub_bound_elsewhere"));
        }
    }
    user_repo::bind_oauth_id(state.db.pool(), uid, &provider, &sub).await?;
    consume_pending(state, ticket).await;
    let _ = audit::emit(
        state,
        AuditEvent::new("user.oauth_bind", Actor::uid(uid).with_ip(client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({ "provider": provider, "via": "pending" })),
    )
    .await;
    Ok(())
}

/// Fast-register from a pending OAuth ticket + verified mobile (PHP `fastReg`).
pub async fn fast_reg(
    state: &AppState,
    ticket: &str,
    mobile: &str,
    sms_code: &str,
    password: &str,
    usertype: u8,
    did: u32,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    crate::site_gate_service::ensure_registration_open(state).await?;
    let (provider, sub) = load_pending(state, ticket).await?;
    if !verify::verify(&state.redis, VerifyKind::SmsRegister, mobile, sms_code).await? {
        return Err(ApiError::param_invalid("sms_code"));
    }
    let writer = state.db.pool();
    if user_repo::exists_mobile(writer, mobile).await? {
        return Err(ApiError::param_invalid("mobile_taken"));
    }
    let mut username = mobile.to_string();
    if user_repo::exists_username(writer, &username).await? {
        username = format!("u{}{}", mobile, Uuid::now_v7().simple());
        username.truncate(20);
    }
    let salt: String = Uuid::now_v7().simple().to_string().chars().take(16).collect();
    let password_hash = argon2_hash_async(format!("{password}{salt}")).await?;
    let now = clock::now_ts();
    let username_c = username.clone();
    let hash_c = password_hash.clone();
    let salt_c = salt.clone();
    let mobile_c = mobile.to_string();
    let ip_c = client_ip.to_string();
    let uid = state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                let uid = user_repo::create_member(
                    &mut **tx,
                    &username_c,
                    &hash_c,
                    &salt_c,
                    Some(&mobile_c),
                    None,
                    usertype,
                    did,
                    &ip_c,
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
    if usertype == 2 {
        let _ = crate::registration_service::apply_default_company_rating(state, uid).await;
    }
    user_repo::bind_oauth_id(writer, uid, &provider, &sub).await?;
    consume_pending(state, ticket).await;

    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, uid, usertype, did)?;
    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;
    auth_event("oauth_fast_reg", None);
    Ok(OAuthLoginResult {
        uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: sub,
        email_from_provider: None,
        name_from_provider: None,
        need_bind: false,
        ticket: String::new(),
        provider,
    })
}
/// Additive endpoint; OA `code-login` is unchanged.
pub async fn login_with_wechat_js_code(
    state: &AppState,
    js_code: &str,
    client_ip: &str,
    user_agent: &str,
) -> AppResult<OAuthLoginResult> {
    let appid = state
        .config
        .wechat_mini_appid
        .as_deref()
        .or(state.config.wechat_appid.as_deref())
        .ok_or_else(|| ApiError::param_invalid("wechat_mini_appid_missing"))?;
    let appsecret = state
        .config
        .wechat_mini_secret
        .as_deref()
        .or(state.config.wechat_appsecret.as_deref())
        .ok_or_else(|| ApiError::param_invalid("wechat_mini_secret_missing"))?;

    let url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
        urlencoding_minimal(appid),
        urlencoding_minimal(appsecret),
        urlencoding_minimal(js_code),
    );

    #[derive(serde::Deserialize)]
    struct JsCodeResp {
        #[serde(default)]
        openid: Option<String>,
        #[serde(default)]
        errcode: Option<i64>,
        #[serde(default)]
        errmsg: Option<String>,
    }

    let resp: JsCodeResp = state.http.get_json(&url).await?;
    if let Some(code) = resp.errcode {
        if code != 0 {
            let msg = resp.errmsg.unwrap_or_default();
            return Err(ApiError::upstream(format!(
                "wechat jscode2session errcode={code} errmsg={msg}"
            )));
        }
    }
    let Some(openid) = resp.openid else {
        return Err(ApiError::upstream("wechat jscode2session returned no openid"));
    };

    let member = user_repo::find_by_oauth_id(state.db.reader(), "wxid", &openid).await?;
    let Some(user) = member else {
        return pending_not_bound(state, "wechat", &openid).await;
    };
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(ApiError::locked());
    }

    let (usertype, did) = auth_identity(&user)?;
    let JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, usertype, did)?;

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: client_ip,
            ua: user_agent,
        },
    )
    .await;

    auth_event("oauth_login_ok", Some("wechat_mini"));
    Ok(OAuthLoginResult {
        uid: user.uid,
        usertype,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: openid,
        email_from_provider: None,
        name_from_provider: None,
        need_bind: false,
        ticket: String::new(),
        provider: "wechat".to_string(),
    })
}

#[cfg(test)]
mod wechat_tests {
    use super::*;

    #[test]
    fn minimal_urlencoding_preserves_unreserved() {
        assert_eq!(urlencoding_minimal("abcXYZ0123-_.~"), "abcXYZ0123-_.~");
    }

    #[test]
    fn minimal_urlencoding_escapes_special() {
        assert_eq!(urlencoding_minimal("a b&c=d?e/f"), "a%20b%26c%3Dd%3Fe%2Ff");
    }

    #[test]
    fn authorize_url_contains_required_params() {
        let u = wechat_authorize_url("wx123", "https://ex.com/cb", "STATE!");
        assert!(u.contains("appid=wx123"));
        assert!(u.contains("redirect_uri=https%3A%2F%2Fex.com%2Fcb"));
        assert!(u.contains("state=STATE%21"));
        assert!(u.contains("scope=snsapi_base"));
        assert!(u.ends_with("#wechat_redirect"));
    }
}
