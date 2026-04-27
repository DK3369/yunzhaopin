//! Third-party login (Google / Facebook / Apple).
//!
//! Flow:
//! 1. The client SDK obtains an `id_token` -> sends `POST /v1/wap/oauth/{provider}/login`
//! 2. The server calls `state.oauth.verify(kind, id_token)` to extract sub / email / name
//! 3. Look up the member by sub:
//!    - **Exists**: issue tokens, log the user in directly, and return success
//!    - **Does not exist**: current policy returns an `oauth_not_bound` error so the client can
//!      take the "bind to an existing account / quick register" path; auto-registration (configured
//!      via `auto_register: true`) can be added later.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt::{issue_pair, JwtIssued};
use phpyun_core::metrics::auth_event;
use phpyun_core::{AppError, AppResult, AppState, InfraError, ProviderKind};
use phpyun_models::user::repo as user_repo;

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
    let member = user_repo::find_by_oauth_id(
        state.db.reader(),
        provider.member_column(),
        &identity.sub,
    )
    .await?;

    let Some(user) = member else {
        auth_event("oauth_not_bound", Some(provider.as_str()));
        return Err(AppError::new(InfraError::InvalidParam(format!(
            "oauth_not_bound:{}:{}",
            provider.as_str(),
            identity.sub
        ))));
    };

    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(AppError::locked());
    }

    // 3. Issue tokens
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

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
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
        usertype: user.usertype as u8,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: identity.sub,
        email_from_provider: identity.email,
        name_from_provider: identity.name,
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
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("wechat_appid_missing".into())))?;
    let appsecret = state.config.wechat_appsecret.as_deref().ok_or_else(|| {
        AppError::new(InfraError::InvalidParam("wechat_appsecret_missing".into()))
    })?;

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
            return Err(AppError::new(InfraError::Upstream(format!(
                "wechat errcode={code} errmsg={msg}"
            ))));
        }
    }
    let Some(openid) = resp.openid else {
        return Err(AppError::new(InfraError::Upstream(
            "wechat oauth returned no openid".into(),
        )));
    };

    // 2) Look up the member by openid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "wxid", &openid).await?;
    let Some(user) = member else {
        auth_event("oauth_not_bound", Some("wechat"));
        return Err(AppError::new(InfraError::InvalidParam(format!(
            "oauth_not_bound:wechat:{openid}"
        ))));
    };

    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(AppError::locked());
    }

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

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
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
        usertype: user.usertype as u8,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: openid,
        email_from_provider: resp.unionid, // reuse the email field to pass back the unionid for the client to retain
        name_from_provider: None,
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
    let appid = state
        .config
        .qq_appid
        .as_deref()
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("qq_appid_missing".into())))?;
    let appsecret = state
        .config
        .qq_appsecret
        .as_deref()
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("qq_appsecret_missing".into())))?;
    let redirect = state.config.qq_oauth_redirect.as_deref().ok_or_else(|| {
        AppError::new(InfraError::InvalidParam(
            "qq_oauth_redirect_missing".into(),
        ))
    })?;

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
        return Err(AppError::new(InfraError::Upstream(format!(
            "qq token error={err} msg={msg}"
        ))));
    }
    let Some(access_token) = resp.access_token else {
        return Err(AppError::new(InfraError::Upstream(
            "qq oauth returned no access_token".into(),
        )));
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
        return Err(AppError::new(InfraError::Upstream(format!(
            "qq /me error={err} msg={msg}"
        ))));
    }
    let Some(openid) = me.openid else {
        return Err(AppError::new(InfraError::Upstream(
            "qq oauth returned no openid".into(),
        )));
    };

    // 3) Look up bound member by qqid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "qqid", &openid).await?;
    let Some(user) = member else {
        auth_event("oauth_not_bound", Some("qq"));
        return Err(AppError::new(InfraError::InvalidParam(format!(
            "oauth_not_bound:qq:{openid}"
        ))));
    };
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(AppError::locked());
    }

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

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
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
        usertype: user.usertype as u8,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: openid,
        email_from_provider: me.unionid,
        name_from_provider: None,
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
    let appid = state
        .config
        .weibo_appid
        .as_deref()
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("weibo_appid_missing".into())))?;
    let appsecret = state.config.weibo_appsecret.as_deref().ok_or_else(|| {
        AppError::new(InfraError::InvalidParam("weibo_appsecret_missing".into()))
    })?;
    let redirect = state.config.weibo_oauth_redirect.as_deref().ok_or_else(|| {
        AppError::new(InfraError::InvalidParam(
            "weibo_oauth_redirect_missing".into(),
        ))
    })?;

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
            return Err(AppError::new(InfraError::Upstream(format!(
                "weibo error={err} msg={msg}"
            ))));
        }
    }
    let Some(uid_str) = resp.uid else {
        return Err(AppError::new(InfraError::Upstream(
            "weibo oauth returned no uid".into(),
        )));
    };

    // Look up the bound member by sinaid
    let member = user_repo::find_by_oauth_id(state.db.reader(), "sinaid", &uid_str).await?;
    let Some(user) = member else {
        auth_event("oauth_not_bound", Some("weibo"));
        return Err(AppError::new(InfraError::InvalidParam(format!(
            "oauth_not_bound:weibo:{uid_str}"
        ))));
    };
    if user.status == 2 {
        auth_event("oauth_login_fail", Some("locked"));
        return Err(AppError::locked());
    }

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

    let _ = crate::user_session_service::record_login(
        state,
        crate::user_session_service::LoginRecord {
            uid: user.uid,
            usertype: user.usertype as u8,
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
        usertype: user.usertype as u8,
        access,
        refresh,
        access_exp,
        refresh_exp,
        provider_sub: uid_str,
        email_from_provider: resp.access_token,
        name_from_provider: None,
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

/// Minimal URL encoding — only escapes characters that would break the WeChat URL syntax,
/// avoiding the need to pull in a new `urlencoding` / `percent-encoding` crate.
fn urlencoding_minimal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{b:02X}"));
            }
        }
    }
    out
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
        assert_eq!(
            urlencoding_minimal("a b&c=d?e/f"),
            "a%20b%26c%3Dd%3Fe%2Ff"
        );
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
    if let Some(other) = user_repo::find_by_oauth_id(
        state.db.reader(),
        provider.member_column(),
        &identity.sub,
    )
    .await?
    {
        if other.uid != uid {
            return Err(InfraError::InvalidParam("oauth_sub_bound_elsewhere".into()).into());
        }
    }

    user_repo::bind_oauth_id(
        state.db.pool(),
        uid,
        provider.member_column(),
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
