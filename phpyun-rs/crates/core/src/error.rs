//! Unified application error model.
//!
//! `ApiError` is the only application-defined error type exposed across the
//! workspace. It stores a typed [`ApiErrorKind`] and an optional source error;
//! the public response code, stable key, and translated message are all derived
//! from that kind:
//!
//! ```json
//! // success
//! {"code": 200, "key": "ok",             "msg": "ok",          "data": {...}}
//! // failure
//! {"code": 403, "key": "role_mismatch",  "msg": "权限不足",     "data": ""}
//! ```
//!
//! `code` always equals the HTTP status. `key` is the stable machine-readable
//! identifier clients branch on; `msg` is the localized copy for display and
//! must never be parsed.
//!
//! ## Status mapping
//!
//! Each kind maps to the HTTP status that actually describes it, so that
//! monitoring, gateways, and CDNs can tell a client mistake apart from a
//! backend outage. Only [`ApiErrorKind::Internal`], [`ApiErrorKind::Db`], and
//! [`ApiErrorKind::Redis`] are genuine 500s; [`ApiErrorKind::Upstream`] is a
//! 502 because the fault lies with a third party we called.

use std::borrow::Cow;
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

const CODE_PARAM: u16 = 400;
const CODE_UNAUTH: u16 = 401;
const CODE_FORBIDDEN: u16 = 403;
/// Request parsed and authorized fine, but a business rule rejected it. This is
/// the default for [`ApiErrorKind::Business`]; individual keys stay on 422
/// rather than being hand-classified into 404/409, because the `key` field
/// already tells the client exactly which rule fired.
const CODE_BUSINESS: u16 = 422;
const CODE_RATE_LIMIT: u16 = 429;
const CODE_ERROR: u16 = 500;
const CODE_UPSTREAM: u16 = 502;

/// Stable error kinds exposed by the API layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiErrorKind {
    Unauth,
    SessionExpired,
    BadCredentials,
    Forbidden,
    RoleMismatch,
    Locked,
    RateLimit,
    Captcha,
    Business(String),
    Upstream(String),
    ParamInvalid(String),
    ParamMissing(String),
    Internal,
    Db,
    Redis,
}

impl ApiErrorKind {
    fn code(&self) -> u16 {
        match self {
            Self::Unauth | Self::SessionExpired | Self::BadCredentials => CODE_UNAUTH,
            Self::Forbidden | Self::RoleMismatch | Self::Locked => CODE_FORBIDDEN,
            Self::Captcha | Self::ParamInvalid(_) | Self::ParamMissing(_) => CODE_PARAM,
            Self::RateLimit => CODE_RATE_LIMIT,
            Self::Business(_) => CODE_BUSINESS,
            Self::Upstream(_) => CODE_UPSTREAM,
            Self::Internal | Self::Db | Self::Redis => CODE_ERROR,
        }
    }

    /// Stable machine-readable key, without any free-text detail. This is what
    /// clients branch on and what `body.key` carries.
    fn key(&self) -> Cow<'static, str> {
        match self {
            Self::Business(key) => Cow::Owned(key.clone()),
            Self::Upstream(_) => Cow::Borrowed("upstream"),
            Self::ParamInvalid(_) => Cow::Borrowed("param_invalid"),
            Self::ParamMissing(_) => Cow::Borrowed("param_missing"),
            other => other.tag(),
        }
    }

    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::Unauth => Cow::Borrowed("unauth"),
            Self::SessionExpired => Cow::Borrowed("session_expired"),
            Self::BadCredentials => Cow::Borrowed("bad_credentials"),
            Self::Forbidden => Cow::Borrowed("forbidden"),
            Self::RoleMismatch => Cow::Borrowed("role_mismatch"),
            Self::Locked => Cow::Borrowed("locked"),
            Self::RateLimit => Cow::Borrowed("rate_limit"),
            Self::Captcha => Cow::Borrowed("captcha"),
            Self::Business(key) => Cow::Owned(key.clone()),
            Self::Upstream(msg) => Cow::Owned(format!("upstream: {msg}")),
            Self::ParamInvalid(msg) => Cow::Owned(format!("param_invalid: {msg}")),
            Self::ParamMissing(name) => Cow::Owned(format!("param_missing: {name}")),
            Self::Internal => Cow::Borrowed("internal"),
            Self::Db => Cow::Borrowed("db"),
            Self::Redis => Cow::Borrowed("redis"),
        }
    }
}

/// Unified application error.
pub struct ApiError {
    kind: ApiErrorKind,
    source: Option<anyhow::Error>,
}

impl ApiError {
    fn tagged(kind: ApiErrorKind) -> Self {
        Self { kind, source: None }
    }

    fn sourced(kind: ApiErrorKind, source: anyhow::Error) -> Self {
        Self {
            kind,
            source: Some(source),
        }
    }

    /// Create a business error while preserving the stable i18n key.
    pub fn business(key: impl Into<String>) -> Self {
        Self::tagged(ApiErrorKind::Business(key.into()))
    }

    pub fn unauth() -> Self {
        Self::tagged(ApiErrorKind::Unauth)
    }

    pub fn session_expired() -> Self {
        Self::tagged(ApiErrorKind::SessionExpired)
    }

    pub fn bad_credentials() -> Self {
        Self::tagged(ApiErrorKind::BadCredentials)
    }

    pub fn forbidden() -> Self {
        Self::tagged(ApiErrorKind::Forbidden)
    }

    pub fn role_mismatch() -> Self {
        Self::tagged(ApiErrorKind::RoleMismatch)
    }

    pub fn locked() -> Self {
        Self::tagged(ApiErrorKind::Locked)
    }

    pub fn rate_limit() -> Self {
        Self::tagged(ApiErrorKind::RateLimit)
    }

    pub fn captcha() -> Self {
        Self::tagged(ApiErrorKind::Captcha)
    }

    pub fn upstream(msg: impl Into<String>) -> Self {
        Self::tagged(ApiErrorKind::Upstream(msg.into()))
    }

    pub fn param_invalid(msg: impl Into<String>) -> Self {
        Self::tagged(ApiErrorKind::ParamInvalid(msg.into()))
    }

    pub fn param_missing(name: &'static str) -> Self {
        Self::tagged(ApiErrorKind::ParamMissing(name.to_owned()))
    }

    /// Wrap a source failure as a 500 internal error.
    pub fn internal<E: std::error::Error + Send + Sync + 'static>(source: E) -> Self {
        Self::sourced(ApiErrorKind::Internal, anyhow::Error::new(source))
    }

    pub fn kind(&self) -> &ApiErrorKind {
        &self.kind
    }

    /// Public response code, equal to the HTTP status. See the module docs for
    /// the kind-to-status table.
    pub fn code(&self) -> u16 {
        self.kind.code()
    }

    /// Stable machine-readable key exposed as `body.key`. Unlike [`Self::tag`]
    /// it never carries a free-text detail, so clients can match on it.
    pub fn key(&self) -> Cow<'static, str> {
        self.kind.key()
    }

    pub fn tag(&self) -> Cow<'static, str> {
        self.kind.tag()
    }

    pub fn should_log(&self) -> bool {
        self.code() >= 500
    }

    pub fn http_status(&self) -> StatusCode {
        StatusCode::from_u16(self.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Rebuild an owned error after moka shares one `Arc<ApiError>` with
    /// concurrent waiters. The source chain is intentionally not duplicated;
    /// the original loader logs it before this downgrade.
    pub fn from_arc(arc: Arc<ApiError>) -> Self {
        Self {
            kind: arc.kind.clone(),
            source: None,
        }
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError")
            .field("code", &self.code())
            .field("kind", &self.kind)
            .field("source", &self.source)
            .finish()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = self.tag();
        match &self.source {
            Some(source) => write!(f, "{tag}: {source}"),
            None => f.write_str(&tag),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(source: sqlx::Error) -> Self {
        Self::sourced(ApiErrorKind::Db, anyhow::Error::new(source))
    }
}

impl From<redis::RedisError> for ApiError {
    fn from(source: redis::RedisError) -> Self {
        Self::sourced(ApiErrorKind::Redis, anyhow::Error::new(source))
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(source: anyhow::Error) -> Self {
        Self::sourced(ApiErrorKind::Internal, source)
    }
}

pub type AppResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.http_status();
        let code = self.code();
        let raw_tag = self.tag();
        let lang = crate::i18n::current_lang();

        if self.should_log() {
            tracing::error!(
                error = %self,
                tag = %raw_tag,
                lang = lang.as_str(),
                "server error"
            );
        }

        // Split raw_tag into "<short_tag>: <detail>" or "<short_tag>".
        let (key_short, detail) = match raw_tag.find(": ") {
            Some(idx) => (&raw_tag[..idx], Some(&raw_tag[idx + 2..])),
            None => (raw_tag.as_ref(), None),
        };

        let i18n_msg = if let Some(d) = detail {
            let dotted_key = d.contains('.')
                && d.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
                && !d.starts_with('.')
                && !d.ends_with('.');

            if dotted_key {
                let translated = crate::i18n::t(d, lang);
                (translated != d).then_some(translated)
            } else {
                let business_key = format!("errors.{d}");
                let translated = crate::i18n::t(&business_key, lang);
                if translated != business_key {
                    Some(translated)
                } else {
                    let with_key = format!("errors.{key_short}_with");
                    let with_msg = crate::i18n::t_args(&with_key, lang, &[("detail", d)]);
                    if with_msg != with_key {
                        Some(with_msg)
                    } else {
                        let short_key = format!("errors.{key_short}");
                        let short_msg = crate::i18n::t(&short_key, lang);
                        (short_msg != short_key).then_some(short_msg)
                    }
                }
            }
        } else {
            let short_key = format!("errors.{key_short}");
            let short_msg = crate::i18n::t(&short_key, lang);
            (short_msg != short_key).then_some(short_msg)
        };

        let msg = i18n_msg.unwrap_or_else(|| {
            let fallback_key = format!("errors.{key_short}");
            let english = crate::i18n::t(&fallback_key, crate::i18n::Lang::En);
            if english != fallback_key {
                english
            } else {
                raw_tag.to_string()
            }
        });

        (
            status,
            Json(json!({
                "code": code,
                "key": self.key(),
                "msg": msg,
                "data": "",
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_maps_to_a_describing_status() {
        assert_eq!(ApiError::unauth().code(), 401);
        assert_eq!(ApiError::session_expired().code(), 401);
        assert_eq!(ApiError::bad_credentials().code(), 401);
        assert_eq!(ApiError::forbidden().code(), 403);
        assert_eq!(ApiError::role_mismatch().code(), 403);
        assert_eq!(ApiError::locked().code(), 403);
        assert_eq!(ApiError::captcha().code(), 400);
        assert_eq!(ApiError::param_invalid("email").code(), 400);
        assert_eq!(ApiError::param_missing("uid").code(), 400);
        assert_eq!(ApiError::rate_limit().code(), 429);
        assert_eq!(ApiError::business("not_found").code(), 422);
        assert_eq!(ApiError::upstream("mail").code(), 502);
        assert_eq!(
            ApiError::internal(std::io::Error::other("disk")).code(),
            500
        );
    }

    #[test]
    fn only_server_side_faults_are_logged() {
        assert!(!ApiError::param_invalid("email").should_log());
        assert!(!ApiError::business("not_found").should_log());
        assert!(!ApiError::rate_limit().should_log());
        assert!(ApiError::upstream("mail").should_log());
        assert!(ApiError::internal(std::io::Error::other("disk")).should_log());
    }

    #[test]
    fn key_strips_free_text_detail_but_keeps_business_keys() {
        assert_eq!(ApiError::param_invalid("email_code").key(), "param_invalid");
        assert_eq!(ApiError::param_missing("uid").key(), "param_missing");
        assert_eq!(ApiError::upstream("mail unavailable").key(), "upstream");
        assert_eq!(ApiError::business("job_not_found").key(), "job_not_found");
        assert_eq!(ApiError::unauth().key(), "unauth");
    }

    #[test]
    fn errors_use_centralized_api_error_kinds() {
        assert_eq!(ApiError::unauth().kind(), &ApiErrorKind::Unauth);
        assert_eq!(
            ApiError::business("job_not_found").kind(),
            &ApiErrorKind::Business("job_not_found".to_owned())
        );
        assert_eq!(
            ApiError::param_invalid("email").kind(),
            &ApiErrorKind::ParamInvalid("email".to_owned())
        );

        let db: ApiError = sqlx::Error::RowNotFound.into();
        assert_eq!(db.kind(), &ApiErrorKind::Db);
    }

    #[test]
    fn tags_preserve_existing_i18n_contract() {
        assert_eq!(ApiError::unauth().tag(), "unauth");
        assert_eq!(ApiError::session_expired().tag(), "session_expired");
        assert_eq!(
            ApiError::param_invalid("email_code").tag(),
            "param_invalid: email_code"
        );
        assert_eq!(ApiError::business("job_not_found").tag(), "job_not_found");
        assert_eq!(
            ApiError::upstream("mail unavailable").tag(),
            "upstream: mail unavailable"
        );
    }

    #[test]
    fn external_errors_convert_to_api_error() {
        let db: ApiError = sqlx::Error::RowNotFound.into();
        assert_eq!(db.code(), 500);
        assert_eq!(db.tag(), "db");

        let internal: ApiError = anyhow::anyhow!("disk full").into();
        assert_eq!(internal.code(), 500);
        assert_eq!(internal.tag(), "internal");
    }

    #[test]
    fn cache_downgrade_preserves_public_error_metadata() {
        let original = Arc::new(ApiError::business("resume_not_found"));
        let degraded = ApiError::from_arc(original);
        assert_eq!(degraded.code(), 422);
        assert_eq!(degraded.tag(), "resume_not_found");
    }
}
