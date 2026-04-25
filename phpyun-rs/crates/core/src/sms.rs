//! SMS sending facade — **pluggable backends**: Aliyun / Tencent Cloud / Twilio / ...
//!
//! Currently provided:
//! - `SmsBackend` trait — the contract for every backend.
//! - `NoopSmsBackend` — for dev: doesn't actually send; just logs the code at
//!   tracing INFO so the developer reads it from the console.
//! - (future) `AliyunSmsBackend` / `TencentSmsBackend` / `TwilioSmsBackend`.
//!
//! ## Templates
//! PHPYun's SMS messages are split into "register verify" / "login verify" /
//! "forgot password" / ... and each cloud has different template IDs. Here we
//! use the `SmsTemplate` enum as a provider-agnostic semantic identifier and
//! leave the concrete template-ID mapping to the backend implementation.
//!
//! ## Business conventions
//! - This module only handles "send"; verification codes are stored in Redis via
//!   `core::verify`.
//! - Rate limiting goes through `core::rate_limit::check_sms_rate`.
//! - Business code calls `Sms::send_code(mobile, code, tmpl)` and is unaware of
//!   the backend.

use crate::config::Config;
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use std::sync::Arc;

/// Semantic identifier for an SMS template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmsTemplate {
    RegisterVerify,
    LoginVerify,
    PasswordReset,
    MobileChange,
}

impl SmsTemplate {
    pub fn as_tag(&self) -> &'static str {
        match self {
            Self::RegisterVerify => "register_verify",
            Self::LoginVerify => "login_verify",
            Self::PasswordReset => "password_reset",
            Self::MobileChange => "mobile_change",
        }
    }
}

#[async_trait]
pub trait SmsBackend: Send + Sync + 'static {
    async fn send_code(&self, mobile: &str, code: &str, tmpl: SmsTemplate) -> AppResult<()>;
}

// ==================== Noop backend (for dev) ====================

pub struct NoopSmsBackend;

#[async_trait]
impl SmsBackend for NoopSmsBackend {
    async fn send_code(&self, mobile: &str, code: &str, tmpl: SmsTemplate) -> AppResult<()> {
        tracing::info!(
            mobile,
            code,
            template = tmpl.as_tag(),
            "SMS (noop): NOT actually sent; use code from logs in dev"
        );
        Ok(())
    }
}

// ==================== Aliyun / Tencent stubs (TODO) ====================

pub struct AliyunSmsBackend {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub sign_name: String,
    // TODO: integrate with the aliyun-rs SDK, or hand-roll HMAC signing + reqwest.
}

#[async_trait]
impl SmsBackend for AliyunSmsBackend {
    async fn send_code(&self, _mobile: &str, _code: &str, _tmpl: SmsTemplate) -> AppResult<()> {
        Err(AppError::internal(std::io::Error::other(
            "AliyunSmsBackend not implemented yet (TODO)",
        )))
    }
}

// ==================== Facade ====================

#[derive(Clone)]
pub struct Sms {
    inner: Arc<dyn SmsBackend>,
}

impl Sms {
    pub fn new<B: SmsBackend>(b: B) -> Self {
        Self { inner: Arc::new(b) }
    }

    pub fn from_config(cfg: &Config) -> AppResult<Self> {
        let kind = cfg.sms_kind.as_deref().unwrap_or("noop");
        match kind {
            "noop" => Ok(Self::new(NoopSmsBackend)),
            "aliyun" => {
                let ak = cfg
                    .sms_aliyun_ak
                    .clone()
                    .ok_or_else(|| AppError::param_invalid("SMS_ALIYUN_AK required"))?;
                let sk = cfg
                    .sms_aliyun_sk
                    .clone()
                    .ok_or_else(|| AppError::param_invalid("SMS_ALIYUN_SK required"))?;
                let sign = cfg
                    .sms_aliyun_sign
                    .clone()
                    .unwrap_or_else(|| "PHPYun".into());
                Ok(Self::new(AliyunSmsBackend {
                    access_key_id: ak,
                    access_key_secret: sk,
                    sign_name: sign,
                }))
            }
            other => Err(AppError::param_invalid(format!("unknown SMS_KIND: {other}"))),
        }
    }

    pub async fn send_code(
        &self,
        mobile: &str,
        code: &str,
        tmpl: SmsTemplate,
    ) -> AppResult<()> {
        self.inner.send_code(mobile, code, tmpl).await?;
        crate::metrics::counter_with("sms.sent", &[("template", tmpl.as_tag())]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn noop_backend_returns_ok() {
        let sms = Sms::new(NoopSmsBackend);
        sms.send_code("13800138000", "123456", SmsTemplate::RegisterVerify)
            .await
            .unwrap();
    }

    #[test]
    fn template_tags_distinct() {
        let tags = [
            SmsTemplate::RegisterVerify.as_tag(),
            SmsTemplate::LoginVerify.as_tag(),
            SmsTemplate::PasswordReset.as_tag(),
            SmsTemplate::MobileChange.as_tag(),
        ];
        assert_eq!(tags[0], "register_verify");
        assert_eq!(tags[1], "login_verify");
        assert_eq!(tags[2], "password_reset");
        assert_eq!(tags[3], "mobile_change");
    }
}
