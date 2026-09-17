//! HTML whitelist sanitizer for fields rendered with `v-html`.
//!
//! Values may contain any character (quotes, `%`, emoji). This is XSS defense,
//! not SQL defense — SQL still uses binds.

pub const DESC_MAX_BYTES: usize = 60_000;

const ALLOWED_TAGS: &[&str] = &[
    "p", "br", "ul", "ol", "li", "h1", "h2", "h3", "strong", "em", "b", "i", "a", "div", "span",
];

pub fn strip_nul(s: &str) -> String {
    if !s.contains('\0') {
        return s.to_string();
    }
    s.replace('\0', "")
}

pub fn sanitize_opt(s: Option<String>) -> Option<String> {
    s.map(|v| sanitize_html(&v))
}

/// Site settings that are rendered with `v-html` (close page / IP ban).
pub fn sanitize_html_setting(key: &str, value: &str) -> String {
    match key {
        "sy_webclose" | "sy_bannedip_alert" => sanitize_html(value),
        _ => strip_nul(value),
    }
}

pub fn sanitize_html(input: &str) -> String {
    let mut s = strip_dangerous_blocks(input);
    s = s.replace('\0', "");
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some((tag, closing, attrs, next)) = parse_tag(&s, i) {
                let name = tag.to_ascii_lowercase();
                if ALLOWED_TAGS.contains(&name.as_str()) {
                    if closing {
                        out.push_str("</");
                        out.push_str(&name);
                        out.push('>');
                    } else if name == "br" {
                        out.push_str("<br>");
                    } else if name == "a" {
                        if let Some(href) = safe_href(&attrs) {
                            out.push_str("<a href=\"");
                            out.push_str(&esc(&href));
                            out.push_str("\" target=\"_blank\" rel=\"noopener\">");
                        }
                    } else {
                        out.push('<');
                        out.push_str(&name);
                        out.push('>');
                    }
                }
                i = next;
                continue;
            }
            out.push_str("&lt;");
            i += 1;
        } else {
            let ch = s[i..].chars().next().unwrap_or(' ');
            match ch {
                '&' => {
                    if s[i..].starts_with("&lt;")
                        || s[i..].starts_with("&gt;")
                        || s[i..].starts_with("&amp;")
                        || s[i..].starts_with("&quot;")
                        || s[i..].starts_with("&#")
                    {
                        out.push('&');
                        i += 1;
                    } else {
                        out.push_str("&amp;");
                        i += 1;
                    }
                }
                _ => {
                    out.push(ch);
                    i += ch.len_utf8();
                }
            }
        }
    }
    clip_bytes(&out, DESC_MAX_BYTES)
}

fn strip_dangerous_blocks(input: &str) -> String {
    let mut s = input.to_string();
    for tag in ["script", "style", "iframe", "object", "embed", "noscript"] {
        loop {
            let open = format!("<{tag}");
            let close = format!("</{tag}");
            let lower = s.to_ascii_lowercase();
            let Some(start) = lower.find(&open) else {
                break;
            };
            let after = lower[start..]
                .find('>')
                .map(|x| start + x + 1)
                .unwrap_or(s.len());
            if let Some(rel) = lower[after..].find(&close) {
                let end_tag = after + rel;
                let end = lower[end_tag..]
                    .find('>')
                    .map(|x| end_tag + x + 1)
                    .unwrap_or(s.len());
                s.replace_range(start..end, "");
            } else {
                s.replace_range(start..after, "");
                break;
            }
        }
    }
    s
}

fn parse_tag(s: &str, start: usize) -> Option<(String, bool, String, usize)> {
    if !s[start..].starts_with('<') {
        return None;
    }
    let rest = &s[start + 1..];
    let closing = rest.starts_with('/');
    let body = if closing { &rest[1..] } else { rest };
    let mut name = String::new();
    let mut chars = body.char_indices();
    for (i, c) in chars.by_ref() {
        if c.is_ascii_alphabetic() || (i > 0 && (c.is_ascii_digit() || c == '-')) {
            name.push(c);
        } else {
            let after_name = &body[i..];
            let gt = after_name.find('>')?;
            let attrs = after_name[..gt].trim().trim_end_matches('/').to_string();
            let next = start + 1 + usize::from(closing) + i + gt + 1;
            return Some((name, closing, attrs, next));
        }
    }
    None
}

fn safe_href(attrs: &str) -> Option<String> {
    let lower = attrs.to_ascii_lowercase();
    let key = "href=";
    let pos = lower.find(key)?;
    let rest = attrs[pos + key.len()..].trim_start();
    let (q, rest) = if rest.starts_with('"') {
        ('"', &rest[1..])
    } else if rest.starts_with('\'') {
        ('\'', &rest[1..])
    } else {
        return None;
    };
    let end = rest.find(q)?;
    let href = unescape_basic(&rest[..end]).trim().to_string();
    let h = href.to_ascii_lowercase();
    if h.starts_with("https://") || h.starts_with("http://") || h.starts_with("mailto:") {
        Some(href)
    } else {
        None
    }
}

pub fn unescape_basic(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn clip_bytes(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_drops_script_onerror_javascript_keeps_list() {
        let html = r#"<script>alert(1)</script><p>Hello</p><ul><li onclick="x">A</li></ul><img src=x onerror="alert(1)"><a href="javascript:alert(1)">x</a><a href="https://jobs.example.com/a">Apply</a>"#;
        let out = sanitize_html(html);
        assert!(!out.to_ascii_lowercase().contains("script"));
        assert!(!out.to_ascii_lowercase().contains("onerror"));
        assert!(!out.to_ascii_lowercase().contains("<img"));
        assert!(out.contains("<p>Hello</p>"));
        assert!(out.contains("<li>A</li>"));
        assert!(out.contains("https://jobs.example.com/a"));
        assert!(!out.contains("javascript:"));
    }

    #[test]
    fn sanitize_keeps_utf8_emoji_quotes_percent() {
        let out = sanitize_html("<p>Lock \u{1F512} 50% O'Reilly \"ok\"</p>");
        assert!(out.contains("\u{1F512}"));
        assert!(out.contains("50%"));
        assert!(out.contains("O'Reilly"));
        assert!(out.contains("&quot;ok&quot;") || out.contains("\"ok\""));
    }

    #[test]
    fn esc_encodes_angle_brackets_for_html_attrs() {
        let out = esc("<img src=x onerror=alert(1)>");
        assert!(out.contains("&lt;"));
        assert!(!out.contains("<img"));
    }

    #[test]
    fn strip_nul_drops_zero_bytes() {
        assert_eq!(strip_nul("a\0b"), "ab");
        assert_eq!(strip_nul("ok"), "ok");
    }

    #[test]
    fn sanitize_html_setting_only_washes_close_and_ban_copy() {
        let dirty = "<img src=x onerror=alert(1)><p>关站</p>";
        let out = sanitize_html_setting("sy_webclose", dirty);
        assert!(!out.to_ascii_lowercase().contains("onerror"));
        assert!(out.contains("关站"));
        assert_eq!(
            sanitize_html_setting("sy_webname", "A\0B"),
            "AB"
        );
    }
}
