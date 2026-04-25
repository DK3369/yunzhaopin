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
