//! Third-party login handlers -- Google / Facebook / Apple (extensible).
//!
//! - POST /v1/wap/oauth/{provider}/login  -- id_token -> login
//! - POST /v1/wap/oauth/{provider}/bind   -- logged-in user binds third-party account

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::{AuthTokenData, OAuthAuthorizeData, OkResp}, ApiJson, AppError, AppResult, AppState, AuthenticatedUser, ClientIp, ProviderKind, ValidatedJson};
use phpyun_services::oauth_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/oauth/login", post(oauth_login))
        .route("/oauth/bind", post(oauth_bind))
        // Code-flow providers (no id_token, third-party returns `code`).
        // WeChat Official Account snsapi_base
        .route(
            "/oauth/wechat/authorize-url",
            post(wechat_authorize_url),
        )
        .route("/oauth/wechat/code-login", post(wechat_code_login))
        // QQ Connect
        .route(
            "/oauth/qq/authorize-url",
            post(qq_authorize_url),
        )
        .route("/oauth/qq/code-login", post(qq_code_login))
        // Weibo
        .route(
            "/oauth/weibo/authorize-url",
            post(weibo_authorize_url),
        )
        .route("/oauth/weibo/code-login", post(weibo_code_login))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OAuthLoginForm {
    /// Provider name: google / facebook / apple.
    #[validate(length(min = 1, max = 32))]
    pub provider: String,
    /// id_token (JWT) returned from the provider SDK
    #[validate(length(min = 40, max = 8192))]
    pub id_token: String,
}

/// Third-party login: exchange id_token for access+refresh tokens
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/login",
    tag = "auth",
    request_body = OAuthLoginForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 400, description = "Invalid provider / failed to parse id_token"),
        (status = 401, description = "Account not bound to this provider — client should guide to bind / quick register"),
    )
)]
pub async fn oauth_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<OAuthLoginForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    phpyun_core::validators::ensure_path_token(&f.provider)?;
    let kind = ProviderKind::parse(&f.provider)
        .ok_or_else(|| AppError::param_invalid(format!("provider: {}", f.provider)))?;

    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = oauth_service::login_with_oauth(&state, kind, &f.id_token, &ip, &ua).await?;

    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        }))
}

/// Logged-in user binds a third-party account to the current uid
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/bind",
    tag = "auth",
    security(("bearer" = [])),
    request_body = OAuthLoginForm,
    responses(
        (status = 200, description = "Bind successful", body = OkResp),
        (status = 400, description = "Invalid provider / this third-party already bound to another user"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn oauth_bind(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<OAuthLoginForm>,
) -> AppResult<ApiJson<OkResp>> {
    phpyun_core::validators::ensure_path_token(&f.provider)?;
    let kind = ProviderKind::parse(&f.provider)
        .ok_or_else(|| AppError::param_invalid(format!("provider: {}", f.provider)))?;
    oauth_service::bind_oauth(&state, user.uid, kind, &f.id_token, &ip).await?;
    Ok(ApiJson(OkResp { ok: true }))
}

// ==================== WeChat snsapi_base ====================

#[derive(Debug, Deserialize, Validate, utoipa::IntoParams)]
pub struct WechatAuthorizeQuery {
    /// Required: full redirect URL after authorization (must be whitelisted in the Official Account backend)
    #[validate(length(min = 1, max = 1024))]
    pub redirect_uri: String,
    /// Recommended: CSRF random string, returned as-is in the callback for client-side verification
    #[validate(length(max = 256))]
    pub state: Option<String>,
}

const WECHAT_STATE_PREFIX: &str = "oauth:wechat:state:";
const WECHAT_STATE_TTL_SECS: u64 = 600; // 10 minutes

/// Generate the WeChat snsapi_base authorization redirect URL
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/wechat/authorize-url",
    tag = "auth",
    params(WechatAuthorizeQuery),
    responses(
        (status = 200, description = "ok", body = OAuthAuthorizeData),
        (status = 400, description = "wechat_appid not configured"),
    )
)]
pub async fn wechat_authorize_url(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<WechatAuthorizeQuery>,
) -> AppResult<ApiJson<OAuthAuthorizeData>> {
    let appid = state
        .config
        .wechat_appid
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("wechat_appid_missing"))?;

    // Open-redirect protection: `redirect_uri` must exactly match (or share an
    // origin prefix with) the backend-configured `WECHAT_OAUTH_REDIRECT`.
    // If not configured, refuse to open the WeChat OAuth entrypoint -- otherwise
    // an attacker could point redirect_uri at any malicious domain for phishing.
    let allowed = state
        .config
        .wechat_oauth_redirect
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("wechat_oauth_redirect_not_configured"))?;
    if q.redirect_uri != allowed {
        return Err(AppError::param_invalid("redirect_uri_not_allowed"));
    }

    // Generate / verify state: always have the server produce a random string
    // and write it to Redis; ignore the client-supplied state (frontend claims
    // cannot be trusted).
    let state_val = uuid::Uuid::now_v7().simple().to_string();
    state
        .redis
        .set_ex(
            &format!("{WECHAT_STATE_PREFIX}{state_val}"),
            "1",
            WECHAT_STATE_TTL_SECS,
        )
        .await?;

    let url = oauth_service::wechat_authorize_url(appid, &q.redirect_uri, &state_val);
    Ok(ApiJson(OAuthAuthorizeData {
        authorize_url: url,
        state: state_val,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WechatCodeLoginForm {
    /// Code returned by the WeChat callback
    #[validate(length(min = 1, max = 256))]
    pub code: String,
    /// State returned by the WeChat callback -- must match the state returned by `authorize-url` (OAuth CSRF protection)
    #[validate(length(min = 1, max = 128))]
    pub state: String,
}

/// Exchange the wxoauth code for a JWT (snsapi_base lightweight flow, only fetches openid)
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/wechat/code-login",
    tag = "auth",
    request_body = WechatCodeLoginForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 400, description = "wechat not configured / invalid code / invalid state"),
        (status = 401, description = "openid not bound to member — client should guide to bind / quick register"),
    )
)]
pub async fn wechat_code_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<WechatCodeLoginForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    // state must exist in Redis (written by authorize-url, 10 minute TTL)
    let key = format!("{WECHAT_STATE_PREFIX}{}", f.state);
    if !state.redis.exists(&key).await {
        return Err(AppError::param_invalid("invalid_state"));
    }
    // One-time use: delete immediately after use to prevent replay
    let _ = state.redis.del(&key).await;

    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = oauth_service::login_with_wechat_code(&state, &f.code, &ip, &ua).await?;
    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        }))
}

// ==================== QQ Connect ====================

const QQ_STATE_PREFIX: &str = "oauth:qq:state:";
const QQ_STATE_TTL_SECS: u64 = 600;

/// Generate the QQ Connect authorization redirect URL.
/// Counterpart of PHP `wap/qqconnect::qqlogin_action` step 1.
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/qq/authorize-url",
    tag = "auth",
    responses(
        (status = 200, description = "ok", body = OAuthAuthorizeData),
        (status = 400, description = "qq_appid / qq_oauth_redirect not configured"),
    )
)]
pub async fn qq_authorize_url(
    State(state): State<AppState>,
) -> AppResult<ApiJson<OAuthAuthorizeData>> {
    let appid = state
        .config
        .qq_appid
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("qq_appid_missing"))?;
    let redirect = state
        .config
        .qq_oauth_redirect
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("qq_oauth_redirect_missing"))?;

    let state_val = uuid::Uuid::now_v7().simple().to_string();
    state
        .redis
        .set_ex(
            &format!("{QQ_STATE_PREFIX}{state_val}"),
            "1",
            QQ_STATE_TTL_SECS,
        )
        .await?;

    let url = oauth_service::qq_authorize_url(appid, redirect, &state_val);
    Ok(ApiJson(OAuthAuthorizeData {
        authorize_url: url,
        state: state_val,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CodeLoginForm {
    /// Code returned by the third-party callback.
    #[validate(length(min = 1, max = 256))]
    pub code: String,
    /// State returned by the third-party callback — must match the value
    /// returned by the `authorize-url` endpoint (CSRF protection).
    #[validate(length(min = 1, max = 128))]
    pub state: String,
}

/// Exchange a QQ Connect code for a JWT.
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/qq/code-login",
    tag = "auth",
    request_body = CodeLoginForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 400, description = "qq not configured / invalid code / invalid state"),
        (status = 401, description = "openid not bound to member"),
    )
)]
pub async fn qq_code_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<CodeLoginForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    let key = format!("{QQ_STATE_PREFIX}{}", f.state);
    if !state.redis.exists(&key).await {
        return Err(AppError::param_invalid("invalid_state"));
    }
    let _ = state.redis.del(&key).await;

    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = oauth_service::login_with_qq_code(&state, &f.code, &ip, &ua).await?;
    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        }))
}

// ==================== Weibo (Sina) ====================

const WEIBO_STATE_PREFIX: &str = "oauth:weibo:state:";
const WEIBO_STATE_TTL_SECS: u64 = 600;

/// Generate the Weibo authorization redirect URL.
/// Counterpart of PHP `wap/sinaconnect::index_action` step 1.
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/weibo/authorize-url",
    tag = "auth",
    responses(
        (status = 200, description = "ok", body = OAuthAuthorizeData),
        (status = 400, description = "weibo_appid / weibo_oauth_redirect not configured"),
    )
)]
pub async fn weibo_authorize_url(
    State(state): State<AppState>,
) -> AppResult<ApiJson<OAuthAuthorizeData>> {
    let appid = state
        .config
        .weibo_appid
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("weibo_appid_missing"))?;
    let redirect = state
        .config
        .weibo_oauth_redirect
        .as_deref()
        .ok_or_else(|| AppError::param_invalid("weibo_oauth_redirect_missing"))?;

    let state_val = uuid::Uuid::now_v7().simple().to_string();
    state
        .redis
        .set_ex(
            &format!("{WEIBO_STATE_PREFIX}{state_val}"),
            "1",
            WEIBO_STATE_TTL_SECS,
        )
        .await?;

    let url = oauth_service::weibo_authorize_url(appid, redirect, &state_val);
    Ok(ApiJson(OAuthAuthorizeData {
        authorize_url: url,
        state: state_val,
    }))
}

/// Exchange a Weibo code for a JWT.
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/weibo/code-login",
    tag = "auth",
    request_body = CodeLoginForm,
    responses(
        (status = 200, description = "Login successful", body = AuthTokenData),
        (status = 400, description = "weibo not configured / invalid code / invalid state"),
        (status = 401, description = "uid not bound to member"),
    )
)]
pub async fn weibo_code_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<CodeLoginForm>,
) -> AppResult<ApiJson<AuthTokenData>> {
    let key = format!("{WEIBO_STATE_PREFIX}{}", f.state);
    if !state.redis.exists(&key).await {
        return Err(AppError::param_invalid("invalid_state"));
    }
    let _ = state.redis.del(&key).await;

    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = oauth_service::login_with_weibo_code(&state, &f.code, &ip, &ua).await?;
    Ok(ApiJson(AuthTokenData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        }))
}
