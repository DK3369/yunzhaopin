//! Common Axum extractors.
//!
//! Usage:
//! ```ignore
//! pub async fn handler(
//!     user: AuthenticatedUser,  // automatic authentication
//!     page: Pagination,          // automatic pagination parameters
//!     ValidatedJson(body): ValidatedJson<MyForm>,  // automatic JSON + validator
//! ) -> AppResult<...> { ... }
//! ```

use crate::error::AppError;
use crate::state::AppState;
use axum::{
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::{header, request::Parts},
    Json,
};
use serde::{de::DeserializeOwned, Deserialize};

// ========== 1. AuthenticatedUser ==========

/// Authenticated user — adding this to the handler signature automatically:
/// 1. Extracts the token from `Authorization: Bearer <jwt>` or `Cookie: token=...`.
/// 2. Verifies the JWT signature + `exp`.
/// 3. Injects `uid` / `usertype`.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub uid: u64,
    pub usertype: u8,
    pub did: u32,
    pub jti: String,
    /// Access-token expiration timestamp (seconds); used to compute the precise
    /// TTL on revocation.
    pub exp: i64,
}

/// PHPYun usertype: 1 = jobseeker, 2 = employer, 3 = admin.
pub const USERTYPE_JOBSEEKER: u8 = 1;
pub const USERTYPE_EMPLOYER: u8 = 2;
pub const USERTYPE_ADMIN: u8 = 3;

impl AuthenticatedUser {
    /// Require the current user to be a jobseeker (`usertype=1`); otherwise
    /// return `role_mismatch` 403.
    pub fn require_jobseeker(&self) -> Result<(), AppError> {
        if self.usertype != USERTYPE_JOBSEEKER {
            return Err(AppError::new(crate::error::InfraError::RoleMismatch));
        }
        Ok(())
    }

    /// Require the current user to be an employer (`usertype=2`).
    pub fn require_employer(&self) -> Result<(), AppError> {
        if self.usertype != USERTYPE_EMPLOYER {
            return Err(AppError::new(crate::error::InfraError::RoleMismatch));
        }
        Ok(())
    }

    /// Require the current user to be an admin (`usertype=3`).
    pub fn require_admin(&self) -> Result<(), AppError> {
        if self.usertype != USERTYPE_ADMIN {
            return Err(AppError::new(crate::error::InfraError::RoleMismatch));
        }
        Ok(())
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Try `Authorization: Bearer ...`.
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .or_else(|| {
                // 2. Fall back to the cookie (compatibility with the legacy
                //    PHPYun migration window).
                parts
                    .headers
                    .get(header::COOKIE)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|cookies| {
                        cookies
                            .split(';')
                            .map(|c| c.trim())
                            .find_map(|c| c.strip_prefix("token="))
                    })
            })
            .ok_or_else(AppError::unauth)?;

        // 3. Verify (only access tokens are accepted; refresh tokens are used
        //    solely to mint new access tokens).
        let claims = crate::jwt::verify_access(&state.config.jwt_secret, token)?;

        // 4. Blacklist: after logout / explicit revocation, the jti goes into
        //    the Redis blacklist.
        //    If Redis is unreachable we treat the token as not revoked
        //    (`is_revoked` returns `false` internally) so authentication
        //    doesn't fall over wholesale; truly sensitive operations should
        //    re-check at the service layer.
        if crate::jwt_blacklist::is_revoked(&state.redis, &claims.jti).await {
            return Err(AppError::session_expired());
        }

        // 5. Post-password-change revocation: on password change / reset /
        //    account split we bump `pw_epoch`; every access/refresh token
        //    issued before the epoch becomes invalid (`iat < epoch`).
        if crate::jwt_blacklist::is_token_stale(&state.redis, claims.sub, claims.iat).await {
            return Err(AppError::session_expired());
        }

        Ok(AuthenticatedUser {
            uid: claims.sub,
            usertype: claims.usertype,
            did: claims.did,
            jti: claims.jti,
            exp: claims.exp,
        })
    }
}

// ========== ClientIp ==========

/// Extract the client's real IP.
///
/// **Trust model**: only trust `X-Forwarded-For` / `X-Real-IP` when **peer_addr
/// belongs to a trusted upstream** (loopback / private IP) — the classic
/// "reverse proxy / LB in front" deployment topology. If the request comes
/// directly from a public-internet peer (no proxy), the headers are ignored,
/// preventing attackers from forging XFF to bypass rate limiting, login-failure
/// counters, or audit attribution.
///
/// Priority:
/// 1. Trusted peer_addr → first segment of `X-Forwarded-For`.
/// 2. Trusted peer_addr → `X-Real-IP`.
/// 3. peer_addr itself.
/// 4. Fallback to `"0.0.0.0"`.
///
/// Usage: add `ip: ClientIp` to the handler signature; pass `&ip.0` to audit /
/// rate-limit code paths.
#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

/// Whether `peer_addr` belongs to a trusted upstream (loopback / private). Only
/// when this returns `true` do we trust `XFF` / `XRI`.
fn is_trusted_peer(addr: &std::net::IpAddr) -> bool {
    match addr {
        std::net::IpAddr::V4(v4) => {
            v4.is_loopback() || v4.is_private() || v4.is_link_local()
        }
        std::net::IpAddr::V6(v6) => {
            v6.is_loopback()
                // ULA fc00::/7
                || (v6.segments()[0] & 0xfe00) == 0xfc00
                // Link-local fe80::/10
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                // IPv4-mapped private v4
                || v6.to_ipv4_mapped().map(|v4| {
                    v4.is_loopback() || v4.is_private() || v4.is_link_local()
                }).unwrap_or(false)
        }
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ClientIp {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let peer = parts
            .extensions
            .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
            .map(|c| c.0.ip());

        // Only consult XFF/XRI when peer_addr is a trusted upstream (loopback / private).
        let trust_forwarded = peer.as_ref().map(is_trusted_peer).unwrap_or(false);

        if trust_forwarded {
            // 1. X-Forwarded-For: take the first segment.
            if let Some(xff) = parts
                .headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
            {
                if let Some(first) = xff.split(',').next() {
                    let ip = first.trim();
                    if !ip.is_empty() {
                        return Ok(ClientIp(ip.to_string()));
                    }
                }
            }

            // 2. X-Real-IP
            if let Some(xri) = parts
                .headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
            {
                let ip = xri.trim();
                if !ip.is_empty() {
                    return Ok(ClientIp(ip.to_string()));
                }
            }
        }

        // 3. peer_addr (requires the server to have `ConnectInfo` enabled).
        if let Some(connect_info) = parts
            .extensions
            .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        {
            return Ok(ClientIp(connect_info.0.ip().to_string()));
        }

        // 4. Fallback.
        Ok(ClientIp("0.0.0.0".into()))
    }
}

/// Optional authentication — doesn't error if the user isn't logged in; the
/// handler decides what to do with `None`.
#[derive(Debug, Clone)]
pub struct MaybeUser(pub Option<AuthenticatedUser>);

impl FromRequestParts<AppState> for MaybeUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(MaybeUser(
            AuthenticatedUser::from_request_parts(parts, state).await.ok(),
        ))
    }
}

// ========== 2. Pagination ==========

#[derive(Debug, Deserialize)]
pub struct PaginationRaw {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    #[serde(default)]
    pub sort: Option<String>,
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub offset: u64,
    pub limit: u64,
}

impl Pagination {
    pub fn sql_limit(&self) -> (u64, u64) {
        (self.offset, self.limit)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Pagination {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(raw) = Query::<PaginationRaw>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::param_invalid(format!("pagination: {e}")))?;
        let page = raw.page.max(1);
        let page_size = raw.page_size.clamp(1, 200);
        Ok(Pagination {
            page,
            page_size,
            offset: ((page - 1) as u64) * (page_size as u64),
            limit: page_size as u64,
        })
    }
}

// ========== 3. ValidatedJson ==========

/// Extract the i18n key of the **first** failing field from
/// `validator::ValidationErrors`.
///
/// Convention: every `#[validate(... message = "validation.xxx.yyy")]` writes
/// the message as an i18n key. That way, when `IntoResponse` receives the
/// `AppError`, the `detail` is already a clean i18n key that can be looked up
/// directly.
///
/// For example, when a form has both `username` and `password` failing:
/// ```text
/// ValidationErrors: { username: [code: length, message: "validation.username.length"], ... }
/// ```
/// We only return the first, to avoid bombarding the user with N messages at
/// once.
fn first_validation_key(errors: &validator::ValidationErrors) -> String {
    use validator::ValidationErrorsKind::*;
    for (_field, kind) in errors.errors() {
        match kind {
            Field(errs) => {
                if let Some(first) = errs.first() {
                    if let Some(m) = &first.message {
                        return m.to_string();
                    }
                    // When `message` isn't set, fall back to the code (e.g. "length", "email").
                    return format!("validation.{}", first.code);
                }
            }
            Struct(nested) => return first_validation_key(nested),
            List(map) => {
                if let Some((_, nested)) = map.iter().next() {
                    return first_validation_key(nested);
                }
            }
        }
    }
    "validation.unknown".to_string()
}

/// Requires the body type to implement `validator::Validate`.
/// On validation failure, returns `AppError::InvalidParam(<i18n key>)`;
/// `IntoResponse` materializes the message by translating it using the current
/// `Lang`.
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + validator::Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::param_invalid(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::param_invalid(first_validation_key(&e)))?;
        Ok(ValidatedJson(value))
    }
}

/// Same shape as `ValidatedJson`, but reads from query string. Use this on
/// any handler that accepts a `Query<T>` whose fields end up in DB queries
/// (LIKE / WHERE / IN), so length / range / regex constraints are enforced
/// before SQL runs.
///
/// Migration tip: change `Query(q): Query<MyQuery>` →
/// `ValidatedQuery(q): ValidatedQuery<MyQuery>` and add `Validate` to the
/// struct's `derive`. Defaults set by `#[serde(default = "...")]` continue to
/// work because `serde::Deserialize` runs before `validator::Validate`.
pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + validator::Validate,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::param_invalid(format!("query: {e}")))?;
        value
            .validate()
            .map_err(|e| AppError::param_invalid(first_validation_key(&e)))?;
        Ok(ValidatedQuery(value))
    }
}

/// Same as `ValidatedJson`, but reads `application/x-www-form-urlencoded`.
pub struct ValidatedForm<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    S: Send + Sync,
    T: DeserializeOwned + validator::Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Form(value) = axum::Form::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::param_invalid(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::param_invalid(first_validation_key(&e)))?;
        Ok(ValidatedForm(value))
    }
}

#[cfg(test)]
mod ip_tests {
    use super::*;
    use axum::extract::ConnectInfo;
    use axum::http::{HeaderMap, HeaderValue};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn parts_with(headers: &[(&str, &str)], peer: Option<IpAddr>) -> Parts {
        let req = axum::http::Request::builder().body(()).unwrap();
        let (mut p, _) = req.into_parts();
        let mut h = HeaderMap::new();
        for (k, v) in headers {
            h.insert(
                axum::http::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            );
        }
        p.headers = h;
        if let Some(ip) = peer {
            p.extensions
                .insert(ConnectInfo(SocketAddr::new(ip, 0)));
        }
        p
    }

    fn loopback() -> IpAddr {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }
    fn public_ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(203, 0, 113, 99))
    }

    #[tokio::test]
    async fn xff_trusted_when_peer_is_loopback() {
        let mut p = parts_with(
            &[("x-forwarded-for", "203.0.113.1, 10.0.0.5")],
            Some(loopback()),
        );
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "203.0.113.1");
    }

    #[tokio::test]
    async fn xff_ignored_when_peer_is_public() {
        // Attacker directly connects from the public internet and forges XFF —
        // we should ignore the header and use peer.
        let mut p = parts_with(
            &[("x-forwarded-for", "1.2.3.4")],
            Some(public_ip()),
        );
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "203.0.113.99");
    }

    #[tokio::test]
    async fn x_real_ip_trusted_when_peer_is_private() {
        let mut p = parts_with(
            &[("x-real-ip", "198.51.100.42")],
            Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
        );
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "198.51.100.42");
    }

    #[tokio::test]
    async fn xff_beats_x_real_ip_when_trusted() {
        let mut p = parts_with(
            &[
                ("x-forwarded-for", "203.0.113.7"),
                ("x-real-ip", "10.0.0.99"),
            ],
            Some(loopback()),
        );
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "203.0.113.7");
    }

    #[tokio::test]
    async fn default_is_0_0_0_0_when_no_headers_no_peer() {
        let mut p = parts_with(&[], None);
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "0.0.0.0");
    }

    #[tokio::test]
    async fn empty_xff_falls_through_to_real_ip() {
        let mut p = parts_with(
            &[
                ("x-forwarded-for", "   "),
                ("x-real-ip", "198.51.100.1"),
            ],
            Some(loopback()),
        );
        let ip: ClientIp = ClientIp::from_request_parts(&mut p, &()).await.unwrap();
        assert_eq!(ip.0, "198.51.100.1");
    }
}
