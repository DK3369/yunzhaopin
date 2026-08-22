//! SMS sending business logic (register / login / reset). Combines "rate limit + code generation + Redis storage + SMS dispatch"
//! into a single call. Aligned with PHPYun `notice.model.php::sendCode`.

use phpyun_core::metrics::auth_event;
use phpyun_core::sms::SmsTemplate;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{rate_limit, AppResult, AppState};
use std::time::Duration;

/// SMS scene — maps to a VerifyKind + template.
#[derive(Debug, Clone, Copy)]
pub enum SmsScene {
    Register,
    Login,
    ResetPw,
    MobileChange,
    /// Anonymous one-shot shop-posting (PHP `wap/once::sendmsg`).
    OnceJob,
    /// Anonymous tiny-resume posting (PHP `wap/tiny::sendmsg`).
    TinyResume,
}

impl SmsScene {
    fn verify_kind(self) -> VerifyKind {
        match self {
            Self::Register => VerifyKind::SmsRegister,
            Self::Login => VerifyKind::SmsLogin,
            Self::ResetPw => VerifyKind::SmsResetPw,
            Self::MobileChange => VerifyKind::SmsMobileChange,
            Self::OnceJob => VerifyKind::SmsOnceJob,
            Self::TinyResume => VerifyKind::SmsTinyResume,
        }
    }

    fn template(self) -> SmsTemplate {
        match self {
            Self::Register => SmsTemplate::RegisterVerify,
            Self::Login => SmsTemplate::LoginVerify,
            Self::ResetPw => SmsTemplate::PasswordReset,
            Self::MobileChange => SmsTemplate::MobileChange,
            Self::OnceJob => SmsTemplate::OnceJob,
            Self::TinyResume => SmsTemplate::TinyResume,
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Self::Register => "sms_register",
            Self::Login => "sms_login",
            Self::ResetPw => "sms_reset",
            Self::MobileChange => "sms_mobile_change",
            Self::OnceJob => "sms_once",
            Self::TinyResume => "sms_tiny",
        }
    }
}

pub async fn send_sms_code(state: &AppState, mobile: &str, scene: SmsScene) -> AppResult<()> {
    // 1. Rate limit (1 per minute + 5 per hour)
    rate_limit::check_sms_rate(&state.redis, mobile).await?;

    // 2. Generate a 6-digit code
    let code = verify::gen_digit_code(6);

    // 3. Store in Redis (valid for 5 minutes)
    verify::issue(
        &state.redis,
        scene.verify_kind(),
        mobile,
        &code,
        Duration::from_secs(300),
    )
    .await?;

    // 4. Send (dev uses a no-op backend that just logs)
    state.sms.send_code(mobile, &code, scene.template()).await?;

    auth_event("sms_code_sent", Some(scene.tag()));
    Ok(())
}
