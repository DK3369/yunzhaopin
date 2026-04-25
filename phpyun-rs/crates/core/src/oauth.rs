//! Third-party login facade — **Google / Facebook / Apple** (and any future platforms).
//!
//! ## Architecture
//! - `OAuthProvider` trait: `verify(id_token) -> ProviderIdentity`
//! - `GoogleProvider` / `FacebookProvider` / `AppleProvider`: stub implementations
//! - `OAuth` facade: dispatches to the corresponding provider
//! - Business layer calls `state.oauth.verify("google", &id_token)` and is
//!   unaware of provider-specific details.
//!
//! ## id_token verification
//! This iteration is a **stub**: no signature verification (usable in dev,
//! must be replaced in production).
//! Real implementation references:
//! - Google: download RS256 public keys from
//!   `https://www.googleapis.com/oauth2/v3/certs` and verify the JWS signature
//! - Facebook: Graph API `me?access_token=...` to get id, or OAuth 2.0 JWT flow
//! - Apple: `https://appleid.apple.com/auth/keys` JWKS + ES256 verification
//!
//! Replacing it doesn't touch the business layer — only swap impls in core.
//!
//! ## Return value
//! On success, `verify` returns `ProviderIdentity { sub, email, name }`. The
//! business layer uses `sub` to look up the `{provider}_id` field in
//! `yun_member`.

use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderIdentity {
    /// Corresponds to the `{provider}_id` or `apple_sub` column on the member table.
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    Google,
    Facebook,
    Apple,
    /// WeChat Official Account snsapi_base OAuth (matches PHPYun `wxoauth`).
    /// The flow is not id_token based — it's code → /sns/oauth2/access_token → openid.
    /// The business layer calls `oauth_service::login_with_wechat_code` directly,
    /// not via `OAuth::verify`.
    WeChat,
}

impl ProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Facebook => "facebook",
            Self::Apple => "apple",
            Self::WeChat => "wechat",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "google" => Some(Self::Google),
            "facebook" => Some(Self::Facebook),
            "apple" => Some(Self::Apple),
            "wechat" | "wx" | "weixin" => Some(Self::WeChat),
            _ => None,
        }
    }

    /// The corresponding column name on the `phpyun_member` table.
    pub fn member_column(&self) -> &'static str {
        match self {
            Self::Google => "google_id",
            Self::Facebook => "fb_id",
            Self::Apple => "apple_sub",
            Self::WeChat => "wxid",
        }
    }
}

#[async_trait]
pub trait OAuthProvider: Send + Sync + 'static {
    fn kind(&self) -> ProviderKind;
    async fn verify(&self, id_token: &str) -> AppResult<ProviderIdentity>;
}

// ==================== Stub implementations (dev + structural example) ====================
//
// These implementations use jsonwebtoken's decode in **non-verifying** mode
// (`validate: false`) just to extract sub / email / name from the token.
// Production must wire up real JWKS.

fn stub_decode(id_token: &str) -> AppResult<ProviderIdentity> {
    // stub: parse JWT payload without verifying the signature
    // (dev only; production must wire up JWKS).
    // Split header.payload.signature and decode the payload from base64.
    let payload = id_token
        .split('.')
        .nth(1)
        .ok_or_else(|| AppError::param_invalid("id_token: not a JWT"))?;
    let bytes = base64::Engine::decode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        payload,
    )
    .or_else(|_| {
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD_NO_PAD, payload)
    })
    .map_err(|e| AppError::param_invalid(format!("id_token payload b64: {e}")))?;

    #[derive(Deserialize)]
    struct StubClaims {
        sub: String,
        email: Option<String>,
        name: Option<String>,
    }
    let claims: StubClaims = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::param_invalid(format!("id_token payload json: {e}")))?;

    Ok(ProviderIdentity {
        sub: claims.sub,
        email: claims.email,
        name: claims.name,
    })
}

pub struct GoogleProvider;

#[async_trait]
impl OAuthProvider for GoogleProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::Google
    }
    async fn verify(&self, id_token: &str) -> AppResult<ProviderIdentity> {
        // TODO: fetch Google JWKS, verify RS256 signature, check iss/aud
        // https://www.googleapis.com/oauth2/v3/certs
        stub_decode(id_token)
    }
}

pub struct FacebookProvider;

#[async_trait]
impl OAuthProvider for FacebookProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::Facebook
    }
    async fn verify(&self, id_token: &str) -> AppResult<ProviderIdentity> {
        // TODO: Facebook uses access_token via Graph API `/me?fields=id,email,name`
        // or OAuth 2.0 JWT flow (supported by newer SDKs).
        stub_decode(id_token)
    }
}

pub struct AppleProvider;

#[async_trait]
impl OAuthProvider for AppleProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::Apple
    }
    async fn verify(&self, id_token: &str) -> AppResult<ProviderIdentity> {
        // TODO: https://appleid.apple.com/auth/keys JWKS + ES256 verification
        stub_decode(id_token)
    }
}

// ==================== Facade ====================

#[derive(Clone)]
pub struct OAuth {
    providers: Arc<HashMap<ProviderKind, Arc<dyn OAuthProvider>>>,
}

impl OAuth {
    /// Registers Google / Facebook / Apple by default (stub implementations).
    pub fn default_stubs() -> Self {
        let mut m: HashMap<ProviderKind, Arc<dyn OAuthProvider>> = HashMap::new();
        m.insert(ProviderKind::Google, Arc::new(GoogleProvider));
        m.insert(ProviderKind::Facebook, Arc::new(FacebookProvider));
        m.insert(ProviderKind::Apple, Arc::new(AppleProvider));
        Self { providers: Arc::new(m) }
    }

    /// Manually register a provider (can replace a stub, e.g. swap in a
    /// signature-verifying implementation in production).
    pub fn with_provider<P: OAuthProvider>(mut self, p: P) -> Self {
        let kind = p.kind();
        let mut m = (*self.providers).clone();
        m.insert(kind, Arc::new(p));
        self.providers = Arc::new(m);
        self
    }

    /// Verify an id_token and return the identity from the provider.
    pub async fn verify(&self, kind: ProviderKind, id_token: &str) -> AppResult<ProviderIdentity> {
        let Some(p) = self.providers.get(&kind) else {
            return Err(AppError::param_invalid(format!(
                "oauth provider not configured: {}",
                kind.as_str()
            )));
        };
        let identity = p.verify(id_token).await?;
        crate::metrics::counter_with(
            "oauth.verify.success",
            &[("provider", kind.as_str())],
        );
        Ok(identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[test]
    fn kind_roundtrip() {
        for k in [
            ProviderKind::Google,
            ProviderKind::Facebook,
            ProviderKind::Apple,
            ProviderKind::WeChat,
        ] {
            assert_eq!(ProviderKind::parse(k.as_str()), Some(k));
        }
        // WeChat accepts multiple aliases
        assert_eq!(ProviderKind::parse("wx"), Some(ProviderKind::WeChat));
        assert_eq!(ProviderKind::parse("weixin"), Some(ProviderKind::WeChat));
        // Unknown provider
        assert_eq!(ProviderKind::parse("github"), None);
    }

    #[test]
    fn member_columns_distinct() {
        assert_ne!(
            ProviderKind::Google.member_column(),
            ProviderKind::Facebook.member_column()
        );
        assert_ne!(
            ProviderKind::Facebook.member_column(),
            ProviderKind::Apple.member_column()
        );
    }

    #[tokio::test]
    async fn stub_verify_extracts_sub_from_id_token() {
        // Build a JWT carrying sub (signature is irrelevant for the stub)
        #[derive(Serialize)]
        struct C {
            sub: &'static str,
            email: &'static str,
            name: &'static str,
        }
        // HS256 requires a secret of at least 32 bytes
        let secret = b"test-secret-at-least-32-bytes-long!";
        let token = encode(
            &Header::default(),
            &C {
                sub: "google-12345",
                email: "alice@x.com",
                name: "Alice",
            },
            &EncodingKey::from_secret(secret),
        )
        .unwrap();

        let oauth = OAuth::default_stubs();
        let id = oauth.verify(ProviderKind::Google, &token).await.unwrap();
        assert_eq!(id.sub, "google-12345");
        assert_eq!(id.email.as_deref(), Some("alice@x.com"));
        assert_eq!(id.name.as_deref(), Some("Alice"));
    }
}
