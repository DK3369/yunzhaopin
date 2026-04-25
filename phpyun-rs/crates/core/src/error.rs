//! Error architecture — **trait-based and pluggable**.
//!
//! ## Layering
//!
//! ```text
//!            trait ApiError (the contract)
//!                    ▲
//!        ┌───────────┼─────────────────────────┐
//!        │           │                         │
//!   InfraError   SystemError       <any custom error from a business crate>
//!   (core,       (core, system     (e.g. services::UserError,
//!    common)     errors)            future payment::PaymentError)
//!        └───────────┼─────────────────────────┘
//!                    │ via blanket `impl<E: ApiError> From<E> for AppError`
//!                    ▼
//!          AppError(Box<dyn ApiError + Send + Sync>)
//! ```
//!
//! ## Response contract
//! - Success: `{code: 200, msg: "ok", data: T}`.
//! - Failure: `{code: <HTTP status>, msg: "<short tag>", data: null}`.
//! - `code` == HTTP status; `msg` is a short ASCII tag.
//!
//! ## Adding a new domain error (3 steps, zero changes to `core`)
//!
//! ```ignore
//! use std::borrow::Cow;
//! use thiserror::Error;
//! use phpyun_core::error::ApiError;
//!
//! // 1. Define the enum.
//! #[derive(Error, Debug, Clone)]
//! pub enum PaymentError {
//!     #[error("payment expired")]     Expired,
//!     #[error("insufficient funds")]  InsufficientFunds,
//! }
//!
//! // 2. Implement ApiError.
//! impl ApiError for PaymentError {
//!     fn code(&self) -> u16 { match self {
//!         Self::Expired            => 410,  // Gone
//!         Self::InsufficientFunds  => 402,  // Payment Required
//!     }}
//!     fn tag(&self) -> Cow<'static, str> { match self {
//!         Self::Expired           => "payment_expired".into(),
//!         Self::InsufficientFunds => "insufficient_funds".into(),
//!     }}
//! }
//!
//! // 3. Propagate with `?` directly (the blanket impl converts to AppError).
//! async fn pay(...) -> AppResult<Receipt> {
//!     if expired { return Err(PaymentError::Expired.into()); }
//!     ...
//! }
//! ```

use std::borrow::Cow;
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

// ========================================================================
// Contract: ApiError trait
// ========================================================================

/// Every error that becomes part of an HTTP response must implement this trait.
///
/// - `code()`: number aligned with the HTTP status (also used as `body.code`).
/// - `tag()`: short ASCII tag for the frontend's i18n (e.g. `unauth`,
///   `rate_limit`, ...).
/// - `should_log()`: whether to write a server error log (default: only for
///   5xx responses).
pub trait ApiError: std::error::Error + Send + Sync + 'static {
    fn code(&self) -> u16;

    fn tag(&self) -> Cow<'static, str>;

    fn should_log(&self) -> bool {
        self.code() >= 500
    }
}

// ========================================================================
// Generic errors built into core
// ========================================================================

/// **Infrastructure errors**: framework-level errors that any crate might hit.
/// Business crates should not extend this enum — to add a new error, define your
/// own enum in your own crate and `impl ApiError`. No changes to `core` are
/// required.
#[derive(Error, Debug, Clone)]
pub enum InfraError {
    // Authentication (401).
    #[error("unauthenticated")]
    Unauthenticated,
    #[error("session expired")]
    SessionExpired,
    #[error("bad credentials")]
    InvalidCredentials,

    // Authorization / role / account state (403).
    #[error("forbidden")]
    Forbidden,
    #[error("role mismatch")]
    RoleMismatch,
    #[error("account locked")]
    AccountLocked,

    // Parameter (400).
    #[error("param missing: {0}")]
    MissingParam(&'static str),
    #[error("param invalid: {0}")]
    InvalidParam(String),
    #[error("captcha error")]
    InvalidCaptcha,

    // Throttling (429).
    #[error("rate limited")]
    RateLimited,

    // Upstream third party (502).
    #[error("upstream: {0}")]
    Upstream(String),
}

impl ApiError for InfraError {
    fn code(&self) -> u16 {
        use InfraError::*;
        match self {
            Unauthenticated | SessionExpired | InvalidCredentials => 401,
            Forbidden | RoleMismatch | AccountLocked => 403,
            MissingParam(_) | InvalidParam(_) | InvalidCaptcha => 400,
            RateLimited => 429,
            Upstream(_) => 502,
        }
    }

    fn tag(&self) -> Cow<'static, str> {
        use InfraError::*;
        match self {
            Unauthenticated => "unauth".into(),
            SessionExpired => "session_expired".into(),
            InvalidCredentials => "bad_credentials".into(),
            Forbidden => "forbidden".into(),
            RoleMismatch => "role_mismatch".into(),
            AccountLocked => "locked".into(),
            // Include the specific field / reason so the frontend can pinpoint it.
            MissingParam(f) => format!("param_missing: {f}").into(),
            InvalidParam(s) => format!("param_invalid: {s}").into(),
            InvalidCaptcha => "captcha".into(),
            RateLimited => "rate_limit".into(),
            Upstream(s) => format!("upstream: {s}").into(),
        }
    }
}

/// **System errors**: source errors that aren't `Clone` (sqlx / redis / anyhow),
/// uniformly mapped to 500. They live in their own enum because they can't
/// `Clone`; mixing them into `InfraError` would force the latter to drop its
/// `Clone` impl.
#[derive(Error, Debug)]
pub enum SystemError {
    #[error("database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("redis: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("internal: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError for SystemError {
    fn code(&self) -> u16 {
        500
    }

    fn tag(&self) -> Cow<'static, str> {
        use SystemError::*;
        match self {
            Database(_) => "db".into(),
            Redis(_) => "redis".into(),
            Internal(_) => "internal".into(),
        }
    }
}

/// **Lightweight error for sharing across tasks** — for scenarios like
/// `moka::try_get_with` that hand back `Arc<AppError>`: when reconstructing from
/// the `Arc`, only `code` + `tag` are preserved; the original detail is lost.
#[derive(Debug, Clone)]
pub struct SharedError {
    code: u16,
    tag: Cow<'static, str>,
}

impl SharedError {
    pub fn new(code: u16, tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code,
            tag: tag.into(),
        }
    }

    pub fn from_dyn(e: &dyn ApiError) -> Self {
        Self {
            code: e.code(),
            tag: e.tag(),
        }
    }
}

impl std::fmt::Display for SharedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.tag, self.code)
    }
}

impl std::error::Error for SharedError {}

impl ApiError for SharedError {
    fn code(&self) -> u16 {
        self.code
    }
    fn tag(&self) -> Cow<'static, str> {
        self.tag.clone()
    }
}

// ========================================================================
// Top-level AppError — Box<dyn ApiError>
// ========================================================================

/// Top-level error type. Any `impl ApiError` value converts via `?`
/// automatically.
pub struct AppError(Box<dyn ApiError + Send + Sync>);

impl AppError {
    pub fn new<E: ApiError>(e: E) -> Self {
        AppError(Box::new(e))
    }

    pub fn code(&self) -> u16 {
        self.0.code()
    }

    pub fn tag(&self) -> Cow<'static, str> {
        self.0.tag()
    }

    pub fn should_log(&self) -> bool {
        self.0.should_log()
    }

    pub fn http_status(&self) -> StatusCode {
        StatusCode::from_u16(self.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Take a new `AppError` from an `Arc<AppError>`, **preserving only
    /// `code` + `tag`**. Used for the `Arc<E>` → owned `E` downgrade path with
    /// moka's `try_get_with`.
    pub fn from_arc(arc: Arc<AppError>) -> AppError {
        AppError::new(SharedError::from_dyn(&*arc.0))
    }

    /// Access the underlying `dyn ApiError` for downcast etc.
    pub fn as_dyn(&self) -> &dyn ApiError {
        &*self.0
    }

    // ---- Shorthand constructors for common InfraError variants (used heavily
    // workspace-wide; short names keep `.into()` from polluting call sites). ----

    pub fn unauth() -> Self {
        AppError::new(InfraError::Unauthenticated)
    }
    pub fn session_expired() -> Self {
        AppError::new(InfraError::SessionExpired)
    }
    pub fn bad_credentials() -> Self {
        AppError::new(InfraError::InvalidCredentials)
    }
    pub fn forbidden() -> Self {
        AppError::new(InfraError::Forbidden)
    }
    pub fn locked() -> Self {
        AppError::new(InfraError::AccountLocked)
    }
    pub fn rate_limit() -> Self {
        AppError::new(InfraError::RateLimited)
    }
    pub fn captcha() -> Self {
        AppError::new(InfraError::InvalidCaptcha)
    }
    pub fn upstream(msg: impl Into<String>) -> Self {
        AppError::new(InfraError::Upstream(msg.into()))
    }
    pub fn param_invalid(msg: impl Into<String>) -> Self {
        AppError::new(InfraError::InvalidParam(msg.into()))
    }
    pub fn param_missing(name: &'static str) -> Self {
        AppError::new(InfraError::MissingParam(name))
    }

    /// Wrap any `std::error::Error` as an internal error (500 / tag `"internal"`).
    /// Rarely used directly by business code — most cases work through `?` and
    /// the various `From` impls.
    pub fn internal<E: std::error::Error + Send + Sync + 'static>(e: E) -> Self {
        AppError::new(SystemError::Internal(anyhow::Error::new(e)))
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError({})", self.0)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for AppError {}

/// Any `ApiError` implementer converts to `AppError` automatically. This is the
/// heart of pluggability: a business crate's enum only needs to `impl ApiError`
/// and it can flow in through `?` without touching `core`.
impl<E: ApiError> From<E> for AppError {
    fn from(e: E) -> Self {
        AppError(Box::new(e))
    }
}

/// Convenience conversions for common external error types. We can't
/// `impl ApiError` for these crates' types (orphan rule), so we use bespoke
/// `From` impls instead.
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        SystemError::Database(e).into()
    }
}

impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        SystemError::Redis(e).into()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        SystemError::Internal(e).into()
    }
}

pub type AppResult<T> = Result<T, AppError>;

// ========================================================================
// HTTP response
// ========================================================================

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.http_status();
        let code = self.code();
        let raw_tag = self.tag(); // e.g. "param_invalid: missing field `password`" or "bad_credentials"
        let lang = crate::i18n::current_lang();

        if self.should_log() {
            // Always log the raw tag (English key) plus the language, for ops.
            tracing::error!(
                error = %self.0,
                tag = %raw_tag,
                lang = lang.as_str(),
                "server error"
            );
        }

        // Split raw_tag into "<short_tag>: <detail>" or "<short_tag>" alone.
        // Examples:
        //   "bad_credentials"                          → key_short=bad_credentials, detail=None
        //   "param_invalid: cannot_block_self"         → key_short=param_invalid,   detail=Some("cannot_block_self")
        //   "param_invalid: validation: username: ..." → key_short=param_invalid,   detail=Some("validation: username: ...")
        let (key_short, detail) = match raw_tag.find(": ") {
            Some(idx) => (&raw_tag[..idx], Some(&raw_tag[idx + 2..])),
            None => (raw_tag.as_ref(), None),
        };

        // i18n translation strategy (by priority):
        // 1. If `detail` is shaped like "<namespace>.<key>" (e.g.
        //    "validation.username.length") → translate that full key directly.
        // 2. If `detail` is a snake_case business key (e.g. "cannot_block_self")
        //    → translate `errors.<detail>`.
        // 3. If `detail` is free text → use the `errors.<short>_with` template
        //    (which carries a `%{detail}` placeholder) and interpolate.
        // 4. No `detail` → translate `errors.<short>`.
        // 5. Nothing matches → return the raw tag verbatim.
        let i18n_msg = if let Some(d) = detail {
            // Detect whether `d` is an i18n key (shaped like "namespace.subkey",
            // pure ASCII).
            let dotted_key = d.contains('.')
                && d.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
                && !d.starts_with('.')
                && !d.ends_with('.');

            if dotted_key {
                // `detail` itself is a complete i18n key (e.g.
                // "validation.username.length").
                let translated = crate::i18n::t(d, lang);
                (translated != d).then_some(translated)
            } else {
                // Try the business key `errors.<detail>`.
                let business_key = format!("errors.{d}");
                let translated = crate::i18n::t(&business_key, lang);
                if translated != business_key {
                    Some(translated)
                } else {
                    // `*_with` template interpolation.
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

        // When the translation misses, fall back to the raw tag (backward
        // compatible; avoids showing the user "errors.xxx").
        let msg = i18n_msg.unwrap_or_else(|| raw_tag.to_string());

        (
            status,
            Json(json!({
                "code": code,
                "msg": msg,
                "data": null,
            })),
        )
            .into_response()
    }
}

// ========================================================================
// Unit tests: contract + pluggability
// ========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- 1. code/tag for built-in InfraError variants ----

    #[test]
    fn infra_error_codes_are_http_aligned() {
        assert_eq!(InfraError::Unauthenticated.code(), 401);
        assert_eq!(InfraError::SessionExpired.code(), 401);
        assert_eq!(InfraError::AccountLocked.code(), 403);
        assert_eq!(InfraError::RateLimited.code(), 429);
        assert_eq!(InfraError::Upstream("x".into()).code(), 502);
        assert_eq!(InfraError::InvalidParam("x".into()).code(), 400);
    }

    #[test]
    fn infra_error_tags_are_ascii_short() {
        for e in [
            InfraError::Unauthenticated,
            InfraError::SessionExpired,
            InfraError::InvalidCredentials,
            InfraError::Forbidden,
            InfraError::AccountLocked,
            InfraError::RateLimited,
        ] {
            assert!(e.tag().is_ascii(), "{:?} tag must be ASCII", e);
            assert!(e.tag().len() <= 20, "{:?} tag too long", e);
        }
    }

    // ---- 2. SystemError ----

    #[test]
    fn system_errors_all_500() {
        assert_eq!(SystemError::Internal(anyhow::anyhow!("x")).code(), 500);
        // Constructing sqlx::Error / redis::RedisError is heavy; skip them here.
    }

    // ---- 3. AppError box wrapping + From chain ----

    #[test]
    fn blanket_from_lets_infra_error_propagate() {
        let e: AppError = InfraError::RateLimited.into();
        assert_eq!(e.code(), 429);
        assert_eq!(e.tag(), "rate_limit");
    }

    #[test]
    fn sqlx_error_auto_converts_to_appeerror_via_system() {
        // RowNotFound is a sqlx::Error variant we can build without performing IO.
        let sqlx_err = sqlx::Error::RowNotFound;
        let e: AppError = sqlx_err.into();
        assert_eq!(e.code(), 500);
        assert_eq!(e.tag(), "db");
    }

    #[test]
    fn anyhow_converts_to_internal() {
        let a: AppError = anyhow::anyhow!("disk full").into();
        assert_eq!(a.code(), 500);
        assert_eq!(a.tag(), "internal");
    }

    // ---- 4. Pluggability — custom domain errors ----

    /// Simulates an error from a business crate; **`core` is completely
    /// untouched** — we only `impl ApiError`.
    #[derive(Error, Debug, Clone)]
    enum PaymentError {
        #[error("payment expired")]
        Expired,
        #[error("insufficient funds: need {0}")]
        InsufficientFunds(u64),
    }

    impl ApiError for PaymentError {
        fn code(&self) -> u16 {
            match self {
                Self::Expired => 410,
                Self::InsufficientFunds(_) => 402,
            }
        }
        fn tag(&self) -> Cow<'static, str> {
            match self {
                Self::Expired => "payment_expired".into(),
                Self::InsufficientFunds(_) => "insufficient_funds".into(),
            }
        }
    }

    #[test]
    fn custom_domain_error_plugs_in_without_core_changes() {
        // Auto-converted via the blanket From impl.
        let e: AppError = PaymentError::Expired.into();
        assert_eq!(e.code(), 410);
        assert_eq!(e.tag(), "payment_expired");

        let e: AppError = PaymentError::InsufficientFunds(100).into();
        assert_eq!(e.code(), 402);
        assert_eq!(e.tag(), "insufficient_funds");
    }

    #[test]
    fn question_mark_operator_chains_across_domain_and_infra() {
        // Simulated handler that mixes InfraError / PaymentError / sqlx::Error.
        fn handler(mode: u8) -> AppResult<()> {
            match mode {
                0 => Err(InfraError::RateLimited.into()),
                1 => Err(PaymentError::Expired.into()),
                2 => Err(sqlx::Error::RowNotFound.into()),
                _ => Ok(()),
            }
        }
        assert_eq!(handler(0).unwrap_err().code(), 429);
        assert_eq!(handler(1).unwrap_err().code(), 410);
        assert_eq!(handler(2).unwrap_err().code(), 500);
        assert!(handler(3).is_ok());
    }

    // ---- 5. SharedError downgrade path ----

    #[test]
    fn from_arc_degrades_to_shared_error_preserving_code_tag() {
        let original: AppError = PaymentError::Expired.into();
        let arc = Arc::new(original);
        let degraded = AppError::from_arc(arc);
        assert_eq!(degraded.code(), 410);
        assert_eq!(degraded.tag(), "payment_expired");
    }

    // ---- 6. should_log ----

    #[test]
    fn should_log_defaults_to_5xx_only() {
        assert!(!InfraError::Unauthenticated.should_log()); // 401
        assert!(!InfraError::RateLimited.should_log()); // 429
        assert!(InfraError::Upstream("x".into()).should_log()); // 502
        assert!(SystemError::Internal(anyhow::anyhow!("x")).should_log()); // 500
    }
}
