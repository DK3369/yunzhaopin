//! Fetch official English job descriptions from ATS public APIs (or HTML fallback).

use std::collections::HashMap;

use phpyun_core::http_client::{Http, RetryPolicy};
use serde_json::Value;

const DESC_MAX_BYTES: usize = 60_000;
const ALLOWED_TAGS: &[&str] = &[
    "p", "br", "ul", "ol", "li", "h1", "h2", "h3", "strong", "em", "b", "i", "a", "div", "span",
];

pub struct JdCache {
    ashby: HashMap<String, Value>,
}

impl Default for JdCache {
    fn default() -> Self {
        Self {
            ashby: HashMap::new(),
        }
    }
}

pub async fn official_body_html(http: &Http, cache: &mut JdCache, url: &str) -> Option<String> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return None;
    }
    if let Some(html) = fetch_ats(http, cache, url).await {
        if !html.trim().is_empty() {
            return Some(html);
        }
    }
    match http.get_text(url).await {
        Ok(page) => extract_from_html(&page),
        Err(_) => None,
    }
}

pub fn is_stub_description(html: &str) -> bool {
    let t = html.trim();
    if t.is_empty() || t.contains("Apply / source") {
        return true;
    }
    // Real JDs almost always have lists or subheadings. Short Company/Location
    // cards (optionally plus one og:description paragraph) do not.
    if t.contains("<ul")
        || t.contains("<ol")
        || t.contains("<li")
        || t.contains("<h2")
        || t.contains("<h3")
    {
        return false;
    }
    let paras = t.matches("<p>").count();
    paras <= 4 && t.len() < 800
}

pub fn compose_description(
    company: &str,
    location: &str,
    posted_at: &str,
    url: &str,
    body_html: Option<&str>,
) -> String {
    let mut html = String::new();
    html.push_str("<p><strong>Company:</strong> ");
    html.push_str(&esc(company));
    html.push_str("</p><p><strong>Location:</strong> ");
    html.push_str(&esc(location));
    html.push_str("</p>");
    if !posted_at.is_empty() {
        html.push_str("<p><strong>Posted:</strong> ");
        html.push_str(&esc(posted_at));
        html.push_str("</p>");
    }
    if let Some(body) = body_html.map(str::trim).filter(|s| !s.is_empty()) {
        html.push_str(body);
    } else if !url.is_empty() {
        html.push_str("<p><a href=\"");
        html.push_str(&esc(url));
        html.push_str("\" target=\"_blank\" rel=\"noopener\">Apply / source</a></p>");
    }
    clip_bytes(&html, DESC_MAX_BYTES)
}

async fn fetch_ats(http: &Http, cache: &mut JdCache, url: &str) -> Option<String> {
    let (host, segs, query) = split_url(url)?;
    if host.ends_with("ashbyhq.com") {
        return ashby_html(http, cache, &segs, url).await;
    }
    if host == "jobs.lever.co" || host.ends_with(".lever.co") {
        return lever_html(http, &segs).await;
    }
    if host.contains("greenhouse.io") {
        return greenhouse_html(http, &segs, &query).await;
    }
    None
}

async fn ashby_html(http: &Http, cache: &mut JdCache, segs: &[String], page_url: &str) -> Option<String> {
    let board = segs.first()?.as_str();
    if board.is_empty() {
        return None;
    }
    let id = segs
        .iter()
        .rev()
        .find(|s| s.len() >= 8 && *s != "apply" && *s != "job")
        .cloned()
        .unwrap_or_default();
    if !cache.ashby.contains_key(board) {
        let api = format!("https://api.ashbyhq.com/posting-api/job-board/{board}");
        let val = http
            .get_json_with::<Value>(&api, RetryPolicy::NONE)
            .await
            .unwrap_or(Value::Null);
        cache.ashby.insert(board.to_string(), val);
    }
    let board_json = cache.ashby.get(board)?;
    let jobs = board_json.get("jobs").and_then(Value::as_array)?;
    for job in jobs {
        let jid = job.get("id").and_then(Value::as_str).unwrap_or("");
        let job_url = job.get("jobUrl").and_then(Value::as_str).unwrap_or("");
        let apply_url = job.get("applyUrl").and_then(Value::as_str).unwrap_or("");
        if (!id.is_empty() && (jid == id || job_url.contains(&id) || apply_url.contains(&id)))
            || job_url == page_url
            || apply_url == page_url
        {
            let html = job
                .get("descriptionHtml")
                .and_then(Value::as_str)
                .unwrap_or("");
            if !html.trim().is_empty() {
                return Some(sanitize_html(html));
            }
            let plain = job
                .get("descriptionPlain")
                .and_then(Value::as_str)
                .unwrap_or("");
            if !plain.trim().is_empty() {
                return Some(plain_to_html(plain));
            }
        }
    }
    None
}

async fn lever_html(http: &Http, segs: &[String]) -> Option<String> {
    let company = segs.first()?.as_str();
    let id = segs.get(1)?.as_str();
    if company.is_empty() || id.is_empty() || id == "apply" {
        return None;
    }
    let api = format!("https://api.lever.co/v0/postings/{company}/{id}");
    let v = http
        .get_json_with::<Value>(&api, RetryPolicy::NONE)
        .await
        .ok()?;
    let mut parts = String::new();
    if let Some(d) = v.get("description").and_then(Value::as_str) {
        parts.push_str(d);
    } else if let Some(d) = v.get("descriptionPlain").and_then(Value::as_str) {
        parts.push_str(&plain_to_html(d));
    }
    if let Some(lists) = v.get("lists").and_then(Value::as_array) {
        for item in lists {
            let text = item.get("text").and_then(Value::as_str).unwrap_or("");
            let content = item.get("content").and_then(Value::as_str).unwrap_or("");
            if !text.is_empty() {
                parts.push_str("<h3>");
                parts.push_str(&esc(text));
                parts.push_str("</h3>");
            }
            parts.push_str(content);
        }
    }
    if parts.trim().is_empty() {
        return None;
    }
    Some(sanitize_html(&parts))
}

async fn greenhouse_html(
    http: &Http,
    segs: &[String],
    query: &[(String, String)],
) -> Option<String> {
    let (board, id) = greenhouse_ids(segs, query)?;
    let api = format!("https://boards-api.greenhouse.io/v1/boards/{board}/jobs/{id}?content=true");
    let v = http
        .get_json_with::<Value>(&api, RetryPolicy::NONE)
        .await
        .ok()?;
    let content = v.get("content").and_then(Value::as_str).unwrap_or("");
    if content.trim().is_empty() {
        return None;
    }
    Some(sanitize_html(&unescape_basic(content)))
}

fn greenhouse_ids(segs: &[String], query: &[(String, String)]) -> Option<(String, String)> {
    let board_q = query.iter().find(|(k, _)| k == "for").map(|(_, v)| v.clone());
    let token = query
        .iter()
        .find(|(k, _)| k == "token" || k == "gh_jid" || k == "id")
        .map(|(_, v)| v.clone());
    if let (Some(b), Some(id)) = (board_q.clone(), token.clone()) {
        return Some((b, id));
    }
    let board = segs.first()?.clone();
    if let Some(i) = segs.iter().position(|s| s == "jobs") {
        let id = segs.get(i + 1)?.clone();
        if !board.is_empty() && !id.is_empty() {
            return Some((board, id));
        }
    }
    None
}

fn extract_from_html(page: &str) -> Option<String> {
    if let Some(html) = jsonld_description(page) {
        return Some(html);
    }
    if let Some(html) = extract_quoted_field(page, "descriptionHtml") {
        return Some(sanitize_html(&html));
    }
    if let Some(plain) = og_description(page) {
        return Some(plain_to_html(&plain));
    }
    None
}

fn jsonld_description(page: &str) -> Option<String> {
    let mut from = 0;
    while let Some(rel) = page[from..].find("application/ld+json") {
        let abs = from + rel;
        let after = &page[abs..];
        let Some(gt) = after.find('>') else {
            from = abs + 1;
            continue;
        };
        let rest = &after[gt + 1..];
        let Some(end) = rest.find("</script") else {
            from = abs + 1;
            continue;
        };
        let raw = rest[..end].trim();
        if let Ok(v) = serde_json::from_str::<Value>(raw) {
            if let Some(html) = jobposting_desc(&v) {
                return Some(html);
            }
        }
        from = abs + 1;
    }
    None
}

fn jobposting_desc(v: &Value) -> Option<String> {
    match v {
        Value::Array(items) => items.iter().find_map(jobposting_desc),
        Value::Object(_) => {
            let ty = v
                .get("@type")
                .and_then(|t| t.as_str().map(|s| s.to_string()).or_else(|| {
                    t.as_array()
                        .and_then(|a| a.iter().filter_map(Value::as_str).next().map(|s| s.to_string()))
                }))
                .unwrap_or_default();
            if ty.eq_ignore_ascii_case("JobPosting") {
                let d = v.get("description").and_then(Value::as_str)?;
                if d.contains('<') {
                    return Some(sanitize_html(d));
                }
                return Some(plain_to_html(d));
            }
            v.get("@graph").and_then(jobposting_desc)
        }
        _ => None,
    }
}

fn extract_quoted_field(page: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\":\"");
    let pos = page.find(&needle)?;
    let rest = &page[pos + needle.len()..];
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => {}
                Some('t') => out.push(' '),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(u) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(u) {
                            out.push(ch);
                        }
                    }
                }
                Some(other) => out.push(other),
                None => break,
            }
        } else if c == '"' {
            break;
        } else {
            out.push(c);
        }
        if out.len() > DESC_MAX_BYTES {
            break;
        }
    }
    if out.trim().is_empty() {
        None
    } else {
        Some(out)
    }
}

fn og_description(page: &str) -> Option<String> {
    let lower = page.to_ascii_lowercase();
    let key = "property=\"og:description\"";
    let pos = lower.find(key)?;
    let slice_end = (pos + 800).min(page.len());
    let slice = &page[pos..slice_end];
    let content_key = "content=\"";
    let cpos = slice.to_ascii_lowercase().find(content_key)?;
    let rest = &slice[cpos + content_key.len()..];
    let end = rest.find('"')?;
    let s = unescape_basic(&rest[..end]);
    if s.trim().len() < 20 {
        None
    } else {
        Some(s)
    }
}

fn split_url(url: &str) -> Option<(String, Vec<String>, Vec<(String, String)>)> {
    let (base, query_raw) = match url.split_once('?') {
        Some((b, q)) => (b, q.split('#').next().unwrap_or(q)),
        None => (url.split('#').next().unwrap_or(url), ""),
    };
    let rest = base
        .strip_prefix("https://")
        .or_else(|| base.strip_prefix("http://"))?;
    let mut parts = rest.split('/');
    let host = parts.next()?.to_ascii_lowercase();
    let segs: Vec<String> = parts
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let mut query = Vec::new();
    if !query_raw.is_empty() {
        for pair in query_raw.split('&') {
            let mut kv = pair.splitn(2, '=');
            let k = kv.next().unwrap_or("").to_string();
            let v = kv.next().unwrap_or("").to_string();
            if !k.is_empty() {
                query.push((k, v));
            }
        }
    }
    Some((host, segs, query))
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
            let after = lower[start..].find('>').map(|x| start + x + 1).unwrap_or(s.len());
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

fn plain_to_html(plain: &str) -> String {
    let mut out = String::from("<p>");
    out.push_str(&esc(plain).replace('\n', "<br>"));
    out.push_str("</p>");
    out
}

fn unescape_basic(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn clip_bytes(s: &str, max: usize) -> String {
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
    fn sanitize_drops_script_keeps_list() {
        let html = r#"<script>alert(1)</script><p>Hello</p><ul><li onclick="x">A</li></ul><a href="javascript:alert(1)">x</a><a href="https://jobs.example.com/a">Apply</a>"#;
        let out = sanitize_html(html);
        assert!(!out.to_ascii_lowercase().contains("script"));
        assert!(out.contains("<p>Hello</p>"));
        assert!(out.contains("<li>A</li>"));
        assert!(out.contains("https://jobs.example.com/a"));
        assert!(!out.contains("javascript:"));
    }

    #[test]
    fn stub_detects_placeholder() {
        assert!(is_stub_description(
            "<p><strong>Company:</strong> X</p><p><strong>Location:</strong> London</p>"
        ));
        assert!(is_stub_description(
            "<p><a href=\"https://jobs.ashbyhq.com/x\" target=\"_blank\" rel=\"noopener\">Apply / source</a></p>"
        ));
        assert!(!is_stub_description(
            "<p><strong>Company:</strong> X</p><p>We are looking for an iOS engineer with five years of experience building consumer products, collaborating with design, and shipping App Store releases every sprint. You will own the mobile roadmap.</p><h3>Requirements</h3><ul><li>Swift</li></ul>"
        ));
    }
}
