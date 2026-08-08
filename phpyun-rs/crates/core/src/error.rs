//! Unified application error model.
//!
//! `AppError` is the only application-defined error type exposed across the
//! workspace. Its internal kind keeps classification private while the public
//! response contract remains stable:
//!
//! - success: `{code: 200, msg: "ok", data: ...}`
//! - unauthenticated / expired session: HTTP 401
//! - every other error: HTTP 500
//!
//! The stable `key` and translated `msg` are preserved for clients; the
//! internal error kind is never serialized.

use std::borrow::Cow;
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Internal classification for `AppError`.
///
/// This is deliberately private: business modules should only depend on
/// `AppError` and `AppResult`, not on a second public error hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppErrorType {
    Unauthenticated,
    SessionExpired,
    InvalidCredentials,
    Forbidden,
    RoleMismatch,
    AccountLocked,
    MissingParam,
    InvalidParam,
    InvalidCaptcha,
    RateLimited,
    Upstream,
    Business,
    Database,
    Redis,
    Internal,
}

/// Unified application error.
pub struct AppError {
    kind: AppErrorType,
    tag: Cow<'static, str>,
    source: Option<anyhow::Error>,
}

impl AppError {
    fn tagged(kind: AppErrorType, tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            tag: tag.into(),
            source: None,
        }
    }

    fn sourced(kind: AppErrorType, tag: &'static str, source: anyhow::Error) -> Self {
        Self {
            kind,
            tag: Cow::Borrowed(tag),
            source: Some(source),
        }
    }

    /// Create a business error while preserving the stable i18n key.
    pub fn business(key: impl Into<String>) -> Self {
        Self::tagged(AppErrorType::Business, Cow::Owned(key.into()))
    }

    pub fn unauth() -> Self {
        Self::tagged(AppErrorType::Unauthenticated, "unauth")
    }

    pub fn session_expired() -> Self {
        Self::tagged(AppErrorType::SessionExpired, "session_expired")
    }

    pub fn bad_credentials() -> Self {
        Self::tagged(AppErrorType::InvalidCredentials, "bad_credentials")
    }

    pub fn forbidden() -> Self {
        Self::tagged(AppErrorType::Forbidden, "forbidden")
    }

    pub fn role_mismatch() -> Self {
        Self::tagged(AppErrorType::RoleMismatch, "role_mismatch")
    }

    pub fn locked() -> Self {
        Self::tagged(AppErrorType::AccountLocked, "locked")
    }

    pub fn rate_limit() -> Self {
        Self::tagged(AppErrorType::RateLimited, "rate_limit")
    }

    pub fn captcha() -> Self {
        Self::tagged(AppErrorType::InvalidCaptcha, "captcha")
    }

    pub fn upstream(msg: impl Into<String>) -> Self {
        Self::tagged(
            AppErrorType::Upstream,
            Cow::Owned(format!("upstream: {}", msg.into())),
        )
    }

    pub fn param_invalid(msg: impl Into<String>) -> Self {
        Self::tagged(
            AppErrorType::InvalidParam,
            Cow::Owned(format!("param_invalid: {}", msg.into())),
        )
    }

    pub fn param_missing(name: &'static str) -> Self {
        Self::tagged(
            AppErrorType::MissingParam,
            Cow::Owned(format!("param_missing: {name}")),
        )
    }

    /// Wrap a source failure as a 500 internal error.
    pub fn internal<E: std::error::Error + Send + Sync + 'static>(source: E) -> Self {
        Self::sourced(
            AppErrorType::Internal,
            "internal",
            anyhow::Error::new(source),
        )
    }

    /// Public response code: only unauthenticated and expired sessions remain
    /// 401; all other application errors are 500.
    pub fn code(&self) -> u16 {
        match self.kind {
            AppErrorType::Unauthenticated | AppErrorType::SessionExpired => 401,
            _ => 500,
        }
    }

    pub fn tag(&self) -> Cow<'static, str> {
        self.tag.clone()
    }

    pub fn should_log(&self) -> bool {
        self.code() >= 500
    }

    pub fn http_status(&self) -> StatusCode {
        StatusCode::from_u16(self.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Rebuild an owned error after moka shares one `Arc<AppError>` with
    /// concurrent waiters. The source chain is intentionally not duplicated;
    /// the original loader logs it before this downgrade.
    pub fn from_arc(arc: Arc<AppError>) -> Self {
        Self {
            kind: arc.kind,
            tag: arc.tag.clone(),
            source: None,
        }
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppError")
            .field("kind", &self.kind)
            .field("tag", &self.tag)
            .field("source", &self.source)
            .finish()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}: {source}", self.tag),
            None => f.write_str(&self.tag),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(source: sqlx::Error) -> Self {
        Self::sourced(AppErrorType::Database, "db", anyhow::Error::new(source))
    }
}

impl From<redis::RedisError> for AppError {
    fn from(source: redis::RedisError) -> Self {
        Self::sourced(AppErrorType::Redis, "redis", anyhow::Error::new(source))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(source: anyhow::Error) -> Self {
        Self::sourced(AppErrorType::Internal, "internal", source)
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
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

        // Resolve the stable response key. Dotted details are already full
        // i18n keys; simple business details resolve under errors.*.
        let response_key = if let Some(d) = detail {
            let dotted_key = d.contains('.')
                && d.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
                && !d.starts_with('.')
                && !d.ends_with('.');
            if dotted_key {
                d.to_string()
            } else if (key_short == "param_invalid" || key_short == "param_missing")
                && d.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                && d.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
            {
                format!("errors.{d}")
            } else {
                format!("errors.{key_short}")
            }
        } else {
            format!("errors.{key_short}")
        };

        let i18n_msg = if let Some(d) = detail {
            let dotted_key = d.contains('.')
                && d.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
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
                "Request failed".to_string()
            }
        });

        (
            status,
            Json(json!({
                "code": code,
                "key": response_key,
                "msg": msg,
                "data": null,
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_authentication_session_errors_are_401() {
        assert_eq!(AppError::unauth().code(), 401);
        assert_eq!(AppError::session_expired().code(), 401);
        assert_eq!(AppError::bad_credentials().code(), 500);
        assert_eq!(AppError::business("not_found").code(), 500);
        assert_eq!(AppError::param_invalid("email").code(), 500);
        assert_eq!(AppError::forbidden().code(), 500);
        assert_eq!(AppError::rate_limit().code(), 500);
        assert_eq!(AppError::upstream("mail").code(), 500);
        assert_eq!(AppError::internal(std::io::Error::other("disk")).code(), 500);
    }

    #[test]
    fn tags_preserve_existing_i18n_contract() {
        assert_eq!(AppError::unauth().tag(), "unauth");
        assert_eq!(AppError::session_expired().tag(), "session_expired");
        assert_eq!(AppError::param_invalid("email_code").tag(), "param_invalid: email_code");
        assert_eq!(AppError::business("job_not_found").tag(), "job_not_found");
        assert_eq!(AppError::upstream("mail unavailable").tag(), "upstream: mail unavailable");
    }

    #[test]
    fn external_errors_convert_to_app_error() {
        let db: AppError = sqlx::Error::RowNotFound.into();
        assert_eq!(db.code(), 500);
        assert_eq!(db.tag(), "db");

        let internal: AppError = anyhow::anyhow!("disk full").into();
        assert_eq!(internal.code(), 500);
        assert_eq!(internal.tag(), "internal");
    }

    #[test]
    fn cache_downgrade_preserves_public_error_metadata() {
        let original = Arc::new(AppError::business("resume_not_found"));
        let degraded = AppError::from_arc(original);
        assert_eq!(degraded.code(), 500);
        assert_eq!(degraded.tag(), "resume_not_found");
    }
}
