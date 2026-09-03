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

/// PHP `job.model.php::setContactHide` + `substr_replace(..., '****', 4, 4)`:
/// mobile numbers use first-3/last-4; other strings replace 4 chars at offset 4.
pub fn mask_contact(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let digits: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 11 {
        return mask_tel(&digits);
    }
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() > 8 {
        let head: String = chars.iter().take(4).collect();
        let tail: String = chars.iter().skip(8).collect();
        return format!("{head}****{tail}");
    }
    mask_tel(trimmed)
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

/// PHP `resume.model.php::setUsernameShow` inputs.
#[derive(Clone, Copy)]
pub struct ResumeNameOpts<'a> {
    pub name: &'a str,
    pub nametype: i32,
    /// Default expect id (`def_job`) used for `NO.{eid}`.
    pub eid: i32,
    pub sex: i32,
    /// Site `user_name`: 0/1 follow nametype, 2 = NO.eid, 3 = family+sex, 4 = full.
    pub user_name: i32,
    pub male_suffix: &'a str,
    pub female_suffix: &'a str,
}

fn family_plus_sex(name: &str, sex: i32, male: &str, female: &str) -> String {
    let first = name.chars().next().unwrap_or('*');
    let suffix = if sex == 1 { male } else { female };
    format!("{first}{suffix}")
}

fn stars_after_first(name: &str) -> String {
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

fn nametype_display(opts: ResumeNameOpts<'_>) -> String {
    let name = opts.name;
    if name.is_empty() {
        return String::new();
    }
    match opts.nametype {
        1 => {
            let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() >= 11 {
                mask_tel(&digits)
            } else {
                name.to_string()
            }
        }
        2 if opts.eid > 0 => format!("NO.{}", opts.eid),
        3 => family_plus_sex(name, opts.sex, opts.male_suffix, opts.female_suffix),
        2 => stars_after_first(name),
        _ => name.to_string(),
    }
}

/// PHP `setUsernameShow` — nametype 1/2/3 plus site `user_name` override.
pub fn mask_resume_username(opts: ResumeNameOpts<'_>) -> String {
    let name = opts.name;
    if name.is_empty() {
        return String::new();
    }
    let mode = if opts.user_name <= 0 { 1 } else { opts.user_name };
    let shown = match mode {
        2 if opts.eid > 0 => format!("NO.{}", opts.eid),
        3 => family_plus_sex(name, opts.sex, opts.male_suffix, opts.female_suffix),
        4 => name.to_string(),
        _ => nametype_display(opts),
    };
    if shown.is_empty() {
        name.to_string()
    } else {
        shown
    }
}

/// Resume-name mask: nametype 1 keeps the full name (mobiles still masked);
/// other types fall back to family + `*`. Prefer [`mask_resume_username`]
/// when eid / site `user_name` are available.
pub fn mask_name_resume(name: &str, nametype: i32) -> String {
    mask_resume_username(ResumeNameOpts {
        name,
        nametype,
        eid: 0,
        sex: 0,
        user_name: 1,
        male_suffix: "",
        female_suffix: "",
    })
}

/// PHP `setResumePhotoShow` without leaking hidden photos.
pub fn resume_photo_shown(
    photo: Option<&str>,
    phototype: i32,
    photo_status: i32,
    defphoto: i32,
    sex: i32,
    user_pic: i32,
    male_default: &str,
    female_default: &str,
) -> String {
    let photo = photo.unwrap_or("").trim();
    let fallback = if sex == 1 || sex == 152 {
        male_default
    } else {
        female_default
    };
    if defphoto == 2 && !photo.is_empty() {
        return photo.to_string();
    }
    let allow_real = user_pic <= 1
        && !photo.is_empty()
        && photo_status == 0
        && phototype != 1;
    if allow_real {
        photo.to_string()
    } else if user_pic == 2 {
        fallback.to_string()
    } else {
        fallback.to_string()
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
    fn mask_contact_mobile_and_landline() {
        assert_eq!(mask_contact("13800138000"), "138****8000");
        assert_eq!(mask_contact("010-88886666"), "010-****6666");
        assert_eq!(mask_contact(""), "");
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
        assert_eq!(
            mask_resume_username(ResumeNameOpts {
                name: "张三丰",
                nametype: 2,
                eid: 88,
                sex: 1,
                user_name: 1,
                male_suffix: "先生",
                female_suffix: "女士",
            }),
            "NO.88"
        );
        assert_eq!(
            mask_resume_username(ResumeNameOpts {
                name: "张三丰",
                nametype: 3,
                eid: 0,
                sex: 1,
                user_name: 1,
                male_suffix: "先生",
                female_suffix: "女士",
            }),
            "张先生"
        );
        assert_eq!(
            mask_resume_username(ResumeNameOpts {
                name: "13800138000",
                nametype: 1,
                eid: 0,
                sex: 1,
                user_name: 1,
                male_suffix: "先生",
                female_suffix: "女士",
            }),
            "138****8000"
        );
    }

    #[test]
    fn resume_photo_hides_unreviewed() {
        assert_eq!(
            resume_photo_shown(Some("a.jpg"), 1, 0, 1, 1, 1, "m.png", "f.png"),
            "m.png"
        );
        assert_eq!(
            resume_photo_shown(Some("a.jpg"), 0, 0, 1, 1, 1, "m.png", "f.png"),
            "a.jpg"
        );
    }
}
