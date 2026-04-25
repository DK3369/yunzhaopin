//! JWT issuance and verification — core infrastructure, kept in `core` so that
//! extractors can reuse it.
//!
//! Design:
//! - Token pair: access (short, 15 min) + refresh (long, 7 days).
//! - Signing algorithm HS256 (symmetric, simple; production can switch to
//!   RS256/EdDSA).
//! - `jti` is used for revocation (Redis blacklist).
//! - Errors are uniformly `AppResult`, so business code never sees `anyhow` or
//!   `jsonwebtoken::errors::Error`.

use crate::clock;
use crate::error::{AppError, AppResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: u64,
    pub usertype: u8,
    pub did: u32,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
    /// "access" | "refresh"
    pub typ: String,
}

pub struct JwtIssued {
    pub access: String,
    pub refresh: String,
    pub access_exp: i64,
    pub refresh_exp: i64,
    pub jti_access: String,
    pub jti_refresh: String,
}

/// Issue a pair of (access, refresh) tokens. TTLs come from `Config` so they
/// can be tuned via `JWT_ACCESS_TTL_SECS` / `JWT_REFRESH_TTL_SECS` env vars
/// without a code change.
pub fn issue_pair(
    cfg: &crate::config::Config,
    uid: u64,
    usertype: u8,
    did: u32,
) -> AppResult<JwtIssued> {
    let now = clock::now_ts();
    let access_exp = now + cfg.jwt_access_ttl_secs;
    // Guarantee refresh outlives access even if env was misconfigured.
    let refresh_exp = now + cfg.jwt_refresh_ttl_secs.max(cfg.jwt_access_ttl_secs);
    let secret = cfg.jwt_secret.as_str();

    let jti_a = Uuid::now_v7().to_string();
    let jti_r = Uuid::now_v7().to_string();

    let key = EncodingKey::from_secret(secret.as_bytes());
    let access = encode_claim(
        &key,
        Claims {
            sub: uid,
            usertype,
            did,
            iat: now,
            exp: access_exp,
            jti: jti_a.clone(),
            typ: "access".into(),
        },
    )?;
    let refresh = encode_claim(
        &key,
        Claims {
            sub: uid,
            usertype,
            did,
            iat: now,
            exp: refresh_exp,
            jti: jti_r.clone(),
            typ: "refresh".into(),
        },
    )?;

    Ok(JwtIssued {
        access,
        refresh,
        access_exp,
        refresh_exp,
        jti_access: jti_a,
        jti_refresh: jti_r,
    })
}

fn encode_claim(key: &EncodingKey, claims: Claims) -> AppResult<String> {
    encode(&Header::default(), &claims, key).map_err(AppError::internal)
}

pub fn verify(secret: &str, token: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|_| AppError::session_expired())
}

/// Accept only tokens with `typ="access"`.
pub fn verify_access(secret: &str, token: &str) -> AppResult<Claims> {
    let claims = verify(secret, token)?;
    if claims.typ != "access" {
        return Err(AppError::session_expired());
    }
    Ok(claims)
}

/// Accept only tokens with `typ="refresh"`.
pub fn verify_refresh(secret: &str, token: &str) -> AppResult<Claims> {
    let claims = verify(secret, token)?;
    if claims.typ != "refresh" {
        return Err(AppError::session_expired());
    }
    Ok(claims)
}
