//! Shared rendering / masking helpers used across handlers.
//!
//! These were copy-pasted into 60+ handler files (`fmt_dt` alone had 63
//! definitions). Centralising them ensures one timezone-format-mistake
//! doesn't slip through code review N times in a row.

use crate::AppState;

// ==================== Time formatting ====================

/// Format a unix timestamp as `YYYY-MM-DD HH:MM`. Returns empty string for
/// `ts <= 0` to mirror PHPYun behaviour where `0` = "not set".
pub fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Format a unix timestamp as `YYYY-MM-DD`. Returns empty string for `ts <= 0`.
pub fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

/// Format a unix timestamp with a custom pattern. Used by handlers that want
/// e.g. `%Y-%m-%d %H:%M:%S`.
pub fn fmt_ts(ts: i64, pattern: &str) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format(pattern).to_string())
        .unwrap_or_default()
}

// ==================== Mask helpers ====================

/// Phone-number mask: keep first 3 + last 4, redact middle. Strings shorter
/// than 7 chars are returned untouched (avoids leaking partial info).
pub fn mask_tel(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 7 {
        return s.to_string();
    }
    let prefix: String = chars.iter().take(3).collect();
    let suffix: String = chars
        .iter()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{prefix}****{suffix}")
}

/// Display-name mask: first char + `**`. Used for resume detail when the
/// jobseeker hasn't agreed to publish their full name.
pub fn mask_name_short(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    match chars.len() {
        0 | 1 => s.to_string(),
        _ => format!("{}**", chars[0]),
    }
}

/// Resume-name mask: keep first char, replace each subsequent with `*`. Used
/// when `nametype != 1` to keep the family name visible while obscuring the
/// rest.
pub fn mask_name_resume(name: &str, nametype: i32) -> String {
    if nametype == 1 {
        return name.to_string();
    }
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if i == 0 {
            out.push(ch);
        } else {
            out.push('*');
        }
    }
    if out.is_empty() {
        "*".to_string()
    } else {
        out
    }
}

// ==================== CDN / picture URL normalisation ====================

/// Normalise a stored image path to a fully-qualified URL using
/// `state.storage` + `state.config.web_base_url`. Pass `Some("")` or `None`
/// for empty inputs and the helper will return an empty string (don't render
/// `<img src="">`-style stubs).
pub fn pic_n(state: &AppState, raw: Option<&str>) -> String {
    state
        .storage
        .normalize_legacy_url(raw.unwrap_or(""), state.config.web_base_url.as_deref())
}

/// Same as [`pic_n`] but takes a `&str` (most handler call sites have raw
/// columns as `String` not `Option<String>`).
pub fn pic_n_str(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

// ==================== Order-status name lookups ====================
//
// PHPYun has two order families with different status enums; both used to
// `match s { ... }` in handler files. Centralised so the i18n / dashboard
// strings can never drift between admin and member-center sides.

/// Redeem-order status (integral mall fulfilment).
/// `0=pending / 1=approved / 2=shipped / 3=completed / 4=rejected`.
pub fn redeem_order_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "approved",
        2 => "shipped",
        3 => "completed",
        4 => "rejected",
        _ => "unknown",
    }
}

/// VIP / once / cash-pay order status.
/// `0=pending / 1=paid / 2=refunded / 3=cancelled`.
pub fn pay_order_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "paid",
        2 => "refunded",
        3 => "cancelled",
        _ => "unknown",
    }
}

/// Generic review status used by reports / company-content / any moderator
/// queue: `0=pending / 1=approved / 2=rejected`.
pub fn review_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "approved",
        2 => "rejected",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_dt_zero_is_empty() {
        assert_eq!(fmt_dt(0), "");
        assert_eq!(fmt_dt(-1), "");
    }

    #[test]
    fn fmt_dt_known_timestamp() {
        // 2024-01-01 00:00:00 UTC = 1704067200
        assert_eq!(fmt_dt(1704067200), "2024-01-01 00:00");
    }

    #[test]
    fn fmt_date_zero_is_empty() {
        assert_eq!(fmt_date(0), "");
    }

    #[test]
    fn mask_tel_keeps_short_strings() {
        assert_eq!(mask_tel("123"), "123");
    }

    #[test]
    fn mask_tel_redacts_middle() {
        assert_eq!(mask_tel("13800138000"), "138****8000");
    }

    #[test]
    fn mask_name_short_keeps_first() {
        assert_eq!(mask_name_short("张三"), "张**");
        assert_eq!(mask_name_short("X"), "X");
        assert_eq!(mask_name_short(""), "");
    }

    #[test]
    fn mask_name_resume_respects_nametype() {
        assert_eq!(mask_name_resume("张三丰", 1), "张三丰");
        assert_eq!(mask_name_resume("张三丰", 2), "张**");
        assert_eq!(mask_name_resume("Alice", 2), "A****");
    }
}
