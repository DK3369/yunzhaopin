//! Third-party login handlers -- Google / Facebook / Apple (extensible).
//!
//! - POST /v1/wap/oauth/{provider}/login  -- id_token -> login
//! - POST /v1/wap/oauth/{provider}/bind   -- logged-in user binds third-party account

use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use phpyun_core::{
    ApiJson, AppError, AppResult, AppState, AuthenticatedUser, ClientIp, ProviderKind, ValidatedJson,
};
use phpyun_services::oauth_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/oauth/{provider}/login", post(oauth_login))
        .route("/oauth/{provider}/bind", post(oauth_bind))
        // WeChat Official Account snsapi_base only (code flow, not id_token)
        .route(
            "/oauth/wechat/authorize-url",
            axum::routing::get(wechat_authorize_url),
        )
        .route("/oauth/wechat/code-login", post(wechat_code_login))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OAuthLoginForm {
    /// id_token (JWT) returned from the provider SDK
    #[validate(length(min = 40, max = 8192))]
    pub id_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthLoginData {
    pub uid: u64,
    pub usertype: u8,
    pub access_token: String,
    pub access_exp: i64,
    pub refresh_token: String,
    pub refresh_exp: i64,
}

/// Third-party login: exchange id_token for access+refresh tokens
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/{provider}/login",
    tag = "auth",
    params(("provider" = String, Path, description = "google / facebook / apple")),
    request_body = OAuthLoginForm,
    responses(
        (status = 200, description = "Login successful", body = OAuthLoginData),
        (status = 400, description = "Invalid provider / failed to parse id_token"),
        (status = 401, description = "Account not bound to this provider — client should guide to bind / quick register"),
    )
)]
pub async fn oauth_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    Path(provider): Path<String>,
    ValidatedJson(f): ValidatedJson<OAuthLoginForm>,
) -> AppResult<ApiJson<OAuthLoginData>> {
    let kind = ProviderKind::parse(&provider)
        .ok_or_else(|| AppError::param_invalid(format!("provider: {provider}")))?;

    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = oauth_service::login_with_oauth(&state, kind, &f.id_token, &ip, &ua).await?;

    Ok(ApiJson(OAuthLoginData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        access_exp: r.access_exp,
        refresh_token: r.refresh,
        refresh_exp: r.refresh_exp,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthBindData {
    pub ok: bool,
}

/// Logged-in user binds a third-party account to the current uid
#[utoipa::path(
    post,
    path = "/v1/wap/oauth/{provider}/bind",
    tag = "auth",
    security(("bearer" = [])),
    params(("provider" = String, Path)),
    request_body = OAuthLoginForm,
    responses(
        (status = 200, description = "Bind successful", body = OAuthBindData),
        (status = 400, description = "Invalid provider / this third-party already bound to another user"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn oauth_bind(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(provider): Path<String>,
    ValidatedJson(f): ValidatedJson<OAuthLoginForm>,
) -> AppResult<ApiJson<OAuthBindData>> {
    let kind = ProviderKind::parse(&provider)
        .ok_or_else(|| AppError::param_invalid(format!("provider: {provider}")))?;
    oauth_service::bind_oauth(&state, user.uid, kind, &f.id_token, &ip).await?;
    Ok(ApiJson(OAuthBindData { ok: true }))
}

// ==================== WeChat snsapi_base ====================

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct WechatAuthorizeQuery {
    /// Required: full redirect URL after authorization (must be whitelisted in the Official Account backend)
    pub redirect_uri: String,
    /// Recommended: CSRF random string, returned as-is in the callback for client-side verification
    pub state: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WechatAuthorizeData {
    pub authorize_url: String,
    /// CSRF-protection state; the client must pass it back as-is to `/oauth/wechat/code-login`
    pub state: String,
}

const WECHAT_STATE_PREFIX: &str = "oauth:wechat:state:";
const WECHAT_STATE_TTL_SECS: u64 = 600; // 10 minutes

/// Generate the WeChat snsapi_base authorization redirect URL
#[utoipa::path(
    get,
    path = "/v1/wap/oauth/wechat/authorize-url",
    tag = "auth",
    params(WechatAuthorizeQuery),
    responses(
        (status = 200, description = "ok", body = WechatAuthorizeData),
        (status = 400, description = "wechat_appid not configured"),
    )
)]
pub async fn wechat_authorize_url(
    State(state): State<AppState>,
    axum::extract::Query(q): axum::extract::Query<WechatAuthorizeQuery>,
) -> AppResult<ApiJson<WechatAuthorizeData>> {
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
    Ok(ApiJson(WechatAuthorizeData {
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
        (status = 200, description = "Login successful", body = OAuthLoginData),
        (status = 400, description = "wechat not configured / invalid code / invalid state"),
        (status = 401, description = "openid not bound to member — client should guide to bind / quick register"),
    )
)]
pub async fn wechat_code_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<WechatCodeLoginForm>,
) -> AppResult<ApiJson<OAuthLoginData>> {
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
    Ok(ApiJson(OAuthLoginData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        access_exp: r.access_exp,
        refresh_token: r.refresh,
        refresh_exp: r.refresh_exp,
    }))
}
