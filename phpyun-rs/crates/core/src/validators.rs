//! Generic argument validators — used with `validator::Validate`'s
//! `#[validate(custom(function = ...))]`.
//!
//! Business code that needs new validation can add a function in its own crate
//! and attach `validate` directly; `core` doesn't have to change. This module
//! only contains **site-wide common** validators, like mobile / email / captcha /
//! ID card / strong password.
//!
//! ## Usage
//!
//! ```ignore
//! use phpyun_core::validators;
//!
//! #[derive(Deserialize, Validate)]
//! struct RegisterForm {
//!     #[validate(custom(function = "validators::cn_mobile"))]
//!     mobile: String,
//!
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(custom(function = "validators::strong_password"))]
//!     password: String,
//!
//!     #[validate(custom(function = "validators::captcha"))]
//!     captcha: String,
//! }
//! ```

use std::sync::OnceLock;
use validator::ValidationError;

/// Mainland China mobile number: `1[3-9]\d{9}`.
pub fn cn_mobile(v: &str) -> Result<(), ValidationError> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"^1[3-9]\d{9}$").expect("valid regex"));
    if re.is_match(v) {
        Ok(())
    } else {
        Err(ValidationError::new("cn_mobile"))
    }
}

/// Mainland China resident ID card: 18 chars (last char is X / x / digit). Format
/// check only — does not validate the check digit.
pub fn cn_id_card(v: &str) -> Result<(), ValidationError> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"^\d{17}[\dXx]$").expect("valid regex")
    });
    if re.is_match(v) {
        Ok(())
    } else {
        Err(ValidationError::new("cn_id_card"))
    }
}

/// 4–6 digit numeric captcha (commonly used for SMS / image captchas).
pub fn captcha(v: &str) -> Result<(), ValidationError> {
    if v.len() >= 4 && v.len() <= 6 && v.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(ValidationError::new("captcha"))
    }
}

/// Strong password: 8–128 chars and contains at least 2 character classes
/// (lower / upper / digit / symbol).
pub fn strong_password(v: &str) -> Result<(), ValidationError> {
    let len = v.chars().count();
    if !(8..=128).contains(&len) {
        return Err(ValidationError::new("password_length"));
    }
    let has_lower = v.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = v.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = v.chars().any(|c| c.is_ascii_digit());
    let has_sym = v.chars().any(|c| !c.is_alphanumeric());
    let classes = [has_lower, has_upper, has_digit, has_sym]
        .iter()
        .filter(|b| **b)
        .count();
    if classes >= 2 {
        Ok(())
    } else {
        Err(ValidationError::new("password_weak"))
    }
}

/// Username: 3–30 chars; letters / digits / underscore; first char can't be a digit.
pub fn username(v: &str) -> Result<(), ValidationError> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| regex::Regex::new(r"^[A-Za-z_][A-Za-z0-9_]{2,29}$").expect("valid regex"));
    if re.is_match(v) {
        Ok(())
    } else {
        Err(ValidationError::new("username"))
    }
}

/// URL (http / https) — rejects schemes like `javascript:` / `data:`.
pub fn http_url(v: &str) -> Result<(), ValidationError> {
    if v.starts_with("http://") || v.starts_with("https://") {
        if v.len() > 2048 {
            Err(ValidationError::new("url_too_long"))
        } else {
            Ok(())
        }
    } else {
        Err(ValidationError::new("url_scheme"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cn_mobile_examples() {
        assert!(cn_mobile("13812345678").is_ok());
        assert!(cn_mobile("15912345678").is_ok());
        assert!(cn_mobile("12812345678").is_err()); // 12x is invalid
        assert!(cn_mobile("1381234567").is_err()); // one digit short
        assert!(cn_mobile("138123456789").is_err()); // one digit too many
        assert!(cn_mobile("abc12345678").is_err());
    }

    #[test]
    fn cn_id_card_format() {
        assert!(cn_id_card("110101199001012345").is_ok());
        assert!(cn_id_card("11010119900101234X").is_ok());
        assert!(cn_id_card("11010119900101234x").is_ok());
        assert!(cn_id_card("11010119900101234").is_err()); // 17 chars
        assert!(cn_id_card("abc").is_err());
    }

    #[test]
    fn captcha_sizes() {
        assert!(captcha("1234").is_ok());
        assert!(captcha("123456").is_ok());
        assert!(captcha("123").is_err());
        assert!(captcha("1234567").is_err());
        assert!(captcha("12ab").is_err());
    }

    #[test]
    fn strong_password_rules() {
        // Weak: only lowercase
        assert!(strong_password("abcdefgh").is_err());
        // Strong: lowercase + digits
        assert!(strong_password("abc12345").is_ok());
        // Strong: lower + upper
        assert!(strong_password("Password").is_ok());
        // Too short
        assert!(strong_password("Ab1").is_err());
        // Too long
        assert!(strong_password(&"a".repeat(200)).is_err());
    }

    #[test]
    fn username_rules() {
        assert!(username("alice").is_ok());
        assert!(username("bob_42").is_ok());
        assert!(username("_hidden").is_ok());
        assert!(username("42alice").is_err()); // first char is a digit
        assert!(username("ab").is_err()); // too short
        assert!(username(&"a".repeat(40)).is_err());
        assert!(username("with space").is_err());
    }

    #[test]
    fn http_url_rules() {
        assert!(http_url("https://example.com").is_ok());
        assert!(http_url("http://localhost:8080/foo").is_ok());
        assert!(http_url("javascript:alert(1)").is_err());
        assert!(http_url("data:text/html,x").is_err());
        assert!(http_url("ftp://example.com").is_err());
    }
}
