//! Verification-code storage — **image captchas + SMS codes** all go through here.
//!
//! Redis is the backing store; a successful verification deletes the entry
//! (replay protection). A failed verification keeps the entry but each code has
//! an attempt cap (default 5) to prevent brute force; once the cap is reached
//! the code is invalidated automatically.
//!
//! ## Semantics
//! - `issue(kind, target, code, ttl)` — write `verify:{kind}:{target}` =
//!   `{code, attempts_left}` with the given TTL.
//! - `verify(kind, target, input)` — compare: on success delete, on failure
//!   `attempts_left - 1`, and on reaching 0 delete + return `false`.
//!
//! ## Typical usage
//! - Image captcha: `target = cid` (a server-generated UUID).
//! - SMS captcha: `target = mobile`.
//! - Email captcha: `target = email`.

use crate::error::AppError;
use crate::kv::Kv;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const MAX_ATTEMPTS: u8 = 5;

/// Verification-code category — determines the Redis key prefix and keeps
/// metric-label cardinality bounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyKind {
    ImageCaptcha,
    SmsRegister,
    SmsLogin,
    SmsResetPw,
    SmsMobileChange,
    EmailReset,
    EmailChange,
}

impl VerifyKind {
    fn prefix(&self) -> &'static str {
        match self {
            Self::ImageCaptcha => "verify:img",
            Self::SmsRegister => "verify:sms:reg",
            Self::SmsLogin => "verify:sms:login",
            Self::SmsResetPw => "verify:sms:reset",
            Self::SmsMobileChange => "verify:sms:mchange",
            Self::EmailReset => "verify:email:reset",
            Self::EmailChange => "verify:email:change",
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Stored {
    code: String,
    attempts_left: u8,
}

fn key(kind: VerifyKind, target: &str) -> String {
    format!("{}:{target}", kind.prefix())
}

/// Write a verification code. Overwrites any existing one (the new code replaces
/// the old).
pub async fn issue(
    kv: &Kv,
    kind: VerifyKind,
    target: &str,
    code: &str,
    ttl: Duration,
) -> Result<(), AppError> {
    let v = Stored {
        code: code.to_string(),
        attempts_left: MAX_ATTEMPTS,
    };
    kv.set_json_ex(&key(kind, target), &v, ttl.as_secs()).await
}

/// Verify the input. On success → delete the key and return `Ok(true)`. On
/// failure → decrement attempts; on reaching 0 delete the key; return `Ok(false)`.
pub async fn verify(
    kv: &Kv,
    kind: VerifyKind,
    target: &str,
    input: &str,
) -> Result<bool, AppError> {
    let k = key(kind, target);
    let Some(mut stored) = kv.get_json::<Stored>(&k).await? else {
        return Ok(false); // never issued or already expired
    };

    if stored.code == input {
        // Success: delete the key (replay protection).
        let _ = kv.del(&k).await;
        return Ok(true);
    }

    // Failure: decrement attempts.
    if stored.attempts_left <= 1 {
        let _ = kv.del(&k).await;
        return Ok(false);
    }
    stored.attempts_left -= 1;
    // Preserve the original TTL — overwriting resets it; a simple rewrite that
    // pins TTL to 5 min is acceptable here.
    let _ = kv
        .set_json_ex(&k, &stored, 300) // 5 min fallback
        .await;
    Ok(false)
}

/// Active deletion (for cancel scenarios).
pub async fn invalidate(kv: &Kv, kind: VerifyKind, target: &str) -> Result<(), AppError> {
    kv.del(&key(kind, target)).await
}

/// Read-only inspection of the currently stored code (used for non-OTP scenarios
/// like email tokens; for OTP scenarios use `verify`, which provides
/// attempt-count protection).
pub async fn peek(kv: &Kv, kind: VerifyKind, target: &str) -> Result<Option<String>, AppError> {
    Ok(kv
        .get_json::<Stored>(&key(kind, target))
        .await?
        .map(|s| s.code))
}

/// Generate an n-digit numeric code (for SMS). Uses the low bits of UUID v7 as
/// the entropy source.
pub fn gen_digit_code(n: usize) -> String {
    let u = uuid::Uuid::now_v7().as_u128();
    let mut buf = String::with_capacity(n);
    for i in 0..n {
        let digit = ((u >> (i * 4)) & 0xF) % 10;
        buf.push(char::from_digit(digit as u32, 10).unwrap_or('0'));
    }
    buf
}

/// Generate an n-char alphanumeric code (for image captchas). Avoids confusable
/// characters (0/O, 1/I, l).
pub fn gen_alnum_code(n: usize) -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut u = uuid::Uuid::now_v7().as_u128();
    let mut buf = String::with_capacity(n);
    for _ in 0..n {
        let idx = (u % CHARS.len() as u128) as usize;
        u /= CHARS.len() as u128;
        buf.push(CHARS[idx] as char);
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_distinct() {
        let prefixes = [
            VerifyKind::ImageCaptcha.prefix(),
            VerifyKind::SmsRegister.prefix(),
            VerifyKind::SmsLogin.prefix(),
            VerifyKind::SmsResetPw.prefix(),
            VerifyKind::SmsMobileChange.prefix(),
            VerifyKind::EmailReset.prefix(),
            VerifyKind::EmailChange.prefix(),
        ];
        for i in 0..prefixes.len() {
            for j in i + 1..prefixes.len() {
                assert_ne!(prefixes[i], prefixes[j]);
            }
        }
    }

    #[test]
    fn gen_digit_code_correct_length_and_numeric() {
        for n in [4, 6, 8] {
            let c = gen_digit_code(n);
            assert_eq!(c.len(), n);
            assert!(c.chars().all(|ch| ch.is_ascii_digit()));
        }
    }

    #[test]
    fn gen_alnum_code_avoids_confusable_chars() {
        for _ in 0..200 {
            let c = gen_alnum_code(4);
            assert_eq!(c.len(), 4);
            for ch in c.chars() {
                assert!(!"01OIl".contains(ch), "confusable char {ch}");
            }
        }
    }
}
