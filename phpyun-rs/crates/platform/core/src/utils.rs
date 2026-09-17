//! Shared rendering / masking helpers used across handlers.
//!
//! These were copy-pasted into 60+ handler files (`fmt_dt` alone had 63
//! definitions). Centralising them ensures one timezone-format-mistake
//! doesn't slip through code review N times in a row.

use crate::AppState;

// ==================== Time formatting ====================
//
// All three render in the **site** timezone ([`crate::clock::tz`], +08:00 by
// default). PHP runs under `date_default_timezone_set('PRC')`, so formatting in
// UTC here would show every timestamp 8 hours earlier than the legacy pages.

/// Format a unix timestamp as `YYYY-MM-DD HH:MM`. Returns empty string for
/// `ts <= 0` to mirror PHPYun behaviour where `0` = "not set".
pub fn fmt_dt(ts: i64) -> String {
    fmt_ts(ts, "%Y-%m-%d %H:%M")
}

/// Format a unix timestamp as `YYYY-MM-DD`. Returns empty string for `ts <= 0`.
pub fn fmt_date(ts: i64) -> String {
    fmt_ts(ts, "%Y-%m-%d")
}

/// Format a unix timestamp with a custom pattern. Used by handlers that want
/// e.g. `%Y-%m-%d %H:%M:%S`.
pub fn fmt_ts(ts: i64, pattern: &str) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.with_timezone(&crate::clock::tz()).format(pattern).to_string())
        .unwrap_or_default()
}

/// PHP `checkpic`: keep absolute http(s); otherwise prefix `sy_ossurl` then `sy_weburl`.
pub fn media_url(base: &str, path: &str) -> String {
    let p = path.trim();
    if p.is_empty() {
        return String::new();
    }
    if p.starts_with("http://") || p.starts_with("https://") {
        return p.to_string();
    }
    let base = base.trim().trim_end_matches('/');
    if base.is_empty() {
        return p.to_string();
    }
    format!("{}/{}", base, p.trim_start_matches('/'))
}

/// `sy_ossurl` wins over `sy_weburl` (same as PHP `checkpic`).
pub fn media_url_from_cfg(cfg: &std::collections::HashMap<String, String>, path: &str) -> String {
    let oss = cfg.get("sy_ossurl").map(|s| s.as_str()).unwrap_or("");
    let web = cfg.get("sy_weburl").map(|s| s.as_str()).unwrap_or("");
    let base = if !oss.trim().is_empty() { oss } else { web };
    media_url(base, path)
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

/// `a***@example.com`. Missing `@` falls back to [`mask_tel`].
pub fn mask_email(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let Some((user, domain)) = trimmed.split_once('@') else {
        return mask_tel(trimmed);
    };
    if user.is_empty() || domain.is_empty() {
        return trimmed.to_string();
    }
    let first = user.chars().next().unwrap_or('*');
    format!("{first}***@{domain}")
}

/// Keep first 3 and last 4 of an ID number; shorter strings pass through.
pub fn mask_idcard(s: &str) -> String {
    let chars: Vec<char> = s.trim().chars().collect();
    if chars.len() < 8 {
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
    format!("{prefix}***********{suffix}")
}

/// JPEG / PNG / GIF / WebP magic. Used by upload handlers so the declared
/// `Content-Type` is not trusted for the stored extension.
pub fn sniff_image(bytes: &[u8]) -> Option<(&'static str, &'static str)> {
    if bytes.len() >= 3 && bytes[0] == 0xFF && bytes[1] == 0xD8 && bytes[2] == 0xFF {
        return Some(("image/jpeg", "jpg"));
    }
    if bytes.len() >= 8
        && bytes[..8] == [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
    {
        return Some(("image/png", "png"));
    }
    if bytes.len() >= 6 && (bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a")) {
        return Some(("image/gif", "gif"));
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some(("image/webp", "webp"));
    }
    None
}

/// PDF / OLE doc / zip-based docx. Declared `Content-Type` is not trusted.
pub fn sniff_document(bytes: &[u8]) -> Option<(&'static str, &'static str)> {
    if bytes.starts_with(b"%PDF") {
        return Some(("application/pdf", "pdf"));
    }
    if bytes.len() >= 8
        && bytes[..8] == [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]
    {
        return Some(("application/msword", "doc"));
    }
    if bytes.starts_with(b"PK") {
        return Some((
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "docx",
        ));
    }
    None
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
        // 2024-01-01 00:00:00 UTC = 1704067200 → site TZ +08
        assert_eq!(fmt_dt(1704067200), "2024-01-01 08:00");
        assert_eq!(fmt_date(1704067200), "2024-01-01");
    }

    #[test]
    fn media_url_prefixes_relative() {
        assert_eq!(
            media_url("https://cdn.example", "/a/b.jpg"),
            "https://cdn.example/a/b.jpg"
        );
        assert_eq!(
            media_url("https://cdn.example/", "a/b.jpg"),
            "https://cdn.example/a/b.jpg"
        );
        assert_eq!(
            media_url("", "https://x.test/p.png"),
            "https://x.test/p.png"
        );
        let mut cfg = std::collections::HashMap::new();
        cfg.insert("sy_ossurl".into(), "https://oss".into());
        cfg.insert("sy_weburl".into(), "https://web".into());
        assert_eq!(media_url_from_cfg(&cfg, "x.png"), "https://oss/x.png");
        cfg.insert("sy_ossurl".into(), "".into());
        assert_eq!(media_url_from_cfg(&cfg, "x.png"), "https://web/x.png");
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

    #[test]
    fn sniff_document_pdf_ole_zip() {
        assert_eq!(
            sniff_document(b"%PDF-1.4 rest"),
            Some(("application/pdf", "pdf"))
        );
        let ole = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
        assert_eq!(
            sniff_document(&ole),
            Some(("application/msword", "doc"))
        );
        assert_eq!(
            sniff_document(b"PK\x03\x04xxxx"),
            Some((
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "docx"
            ))
        );
        assert!(sniff_document(b"<html>").is_none());
    }
}
