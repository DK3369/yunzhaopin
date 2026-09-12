//! Fetch official English job descriptions from ATS public APIs (or HTML fallback).

use std::collections::HashMap;

use phpyun_core::http_client::{Http, RetryPolicy};
use serde_json::Value;

const DESC_MAX_BYTES: usize = 60_000;
/// Visible job-body text (tags stripped). Shorter postings are not stored.
pub const MIN_BODY_CHARS: usize = 100;
const ALLOWED_TAGS: &[&str] = &[
    "p", "br", "ul", "ol", "li", "h1", "h2", "h3", "strong", "em", "b", "i", "a", "div", "span",
];

pub struct JdCache {
    ashby: HashMap<String, Value>,
    greenhouse: HashMap<String, Value>,
}

impl Default for JdCache {
    fn default() -> Self {
        Self {
            ashby: HashMap::new(),
            greenhouse: HashMap::new(),
        }
    }
}

impl JdCache {
    pub fn put_greenhouse(&mut self, board: impl Into<String>, val: Value) {
        self.greenhouse.insert(board.into(), val);
    }

    pub fn put_ashby(&mut self, board: impl Into<String>, val: Value) {
        self.ashby.insert(board.into(), val);
    }
}

pub async fn official_body_html(
    http: &Http,
    cache: &mut JdCache,
    url: &str,
    title: Option<&str>,
) -> Option<String> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return None;
    }
    if let Some(html) = fetch_ats(http, cache, url, title).await {
        if is_substantial(&html) && has_min_body(&html) {
            return Some(html);
        }
    }
    match http.get_text(url).await {
        Ok(page) => extract_from_html(&page).filter(|s| has_min_body(s)),
        Err(_) => None,
    }
}

#[cfg(test)]
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

/// Visible characters in HTML (tags stripped, whitespace collapsed).
pub fn visible_char_count(html: &str) -> usize {
    visible_text(html).chars().count()
}

pub fn has_min_body(html: &str) -> bool {
    visible_char_count(html) >= MIN_BODY_CHARS
}

/// Stored `compose_description` HTML: headers / Apply-link do not count as JD.
pub fn stored_description_has_min_body(html: &str) -> bool {
    if html.trim().is_empty() || html.contains("Apply / source") {
        return false;
    }
    has_min_body(&strip_scrape_headers(html))
}

/// Company/Location headers plus one wall-of-text `<p>` is not a formatted JD.
pub fn is_structured_jd(html: &str) -> bool {
    let b = strip_scrape_headers(html);
    b.contains("<ul")
        || b.contains("<ol")
        || b.contains("<h2")
        || b.contains("<h3")
        || b.contains("<br")
        || b.matches("<p>").count() >= 3
}

fn strip_scrape_headers(html: &str) -> String {
    let mut s = html.to_string();
    for label in ["Company", "Location", "Posted"] {
        let open = format!("<p><strong>{label}:</strong>");
        if let Some(i) = s.find(&open) {
            if let Some(rel) = s[i..].find("</p>") {
                let end = i + rel + 4;
                s.replace_range(i..end, "");
            }
        }
    }
    s
}

fn visible_text(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    unescape_basic(&out)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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

async fn fetch_ats(
    http: &Http,
    cache: &mut JdCache,
    url: &str,
    title: Option<&str>,
) -> Option<String> {
    let (host, segs, query) = split_url(url)?;
    if host.ends_with("ashbyhq.com") {
        return ashby_html(http, cache, &segs, url).await;
    }
    if host == "jobs.lever.co" || host.ends_with(".lever.co") {
        return lever_html(http, &segs).await;
    }
    if host.contains("greenhouse.io") || has_greenhouse_hint(&query) {
        if let Some(html) = greenhouse_html(http, cache, &host, &segs, &query, title).await {
            return Some(html);
        }
    }
    if host.contains("myworkdayjobs.com") {
        return workday_html(http, &host, &segs).await;
    }
    None
}

fn has_greenhouse_hint(query: &[(String, String)]) -> bool {
    query.iter().any(|(k, _)| k == "gh_jid" || k == "for" || k == "board")
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
    cache: &mut JdCache,
    host: &str,
    segs: &[String],
    query: &[(String, String)],
    title: Option<&str>,
) -> Option<String> {
    let (board, id) = greenhouse_ids(host, segs, query)?;
    if let Some(html) = greenhouse_job_api(http, &board, &id).await {
        return Some(html);
    }
    greenhouse_board_lookup(http, cache, &board, &id, title).await
}

async fn workday_html(http: &Http, host: &str, segs: &[String]) -> Option<String> {
    let api = workday_cxs_url(host, segs)?;
    let v = http
        .get_json_with::<Value>(&api, RetryPolicy::NONE)
        .await
        .ok()?;
    let d = v
        .pointer("/jobPostingInfo/jobDescription")
        .and_then(Value::as_str)?;
    if d.trim().is_empty() {
        return None;
    }
    Some(tidy_workday_html(d))
}

fn workday_cxs_url(host: &str, segs: &[String]) -> Option<String> {
    if !host.contains("myworkdayjobs.com") {
        return None;
    }
    let job_idx = segs.iter().position(|s| s.eq_ignore_ascii_case("job"))?;
    if job_idx == 0 {
        return None;
    }
    let tenant = host.split('.').next().unwrap_or("");
    if tenant.is_empty() || tenant.starts_with("wd") {
        return None;
    }
    let path = segs.join("/");
    Some(format!("https://{host}/wday/cxs/{tenant}/{path}"))
}

fn tidy_workday_html(input: &str) -> String {
    let mut s = sanitize_html(input);
    for tag in ["div", "span"] {
        s = s.replace(&format!("<{tag}>"), "");
        s = s.replace(&format!("</{tag}>"), "");
    }
    loop {
        let next = s.replace("<p></p>", "").replace("<p> </p>", "");
        if next == s {
            break;
        }
        s = next;
    }
    wrap_loose_text(&s)
}

/// Workday often puts labels in `<p>` and the value as a sibling text node.
fn wrap_loose_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len() + 32);
    let mut i = 0;
    while i < html.len() {
        if html[i..].starts_with("</p>") {
            out.push_str("</p>");
            i += 4;
            let rest = &html[i..];
            let next_lt = rest.find('<').unwrap_or(rest.len());
            let chunk = rest[..next_lt].trim();
            if !chunk.is_empty() {
                out.push_str("<p>");
                out.push_str(chunk);
                out.push_str("</p>");
            }
            i += next_lt;
        } else {
            let ch = html[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

async fn greenhouse_job_api(http: &Http, board: &str, id: &str) -> Option<String> {
    let api = format!("https://boards-api.greenhouse.io/v1/boards/{board}/jobs/{id}?content=true");
    let v = http
        .get_json_with::<Value>(&api, RetryPolicy::NONE)
        .await
        .ok()?;
    greenhouse_content_html(v.get("content").and_then(Value::as_str).unwrap_or(""))
}

async fn greenhouse_board_lookup(
    http: &Http,
    cache: &mut JdCache,
    board: &str,
    id: &str,
    title: Option<&str>,
) -> Option<String> {
    if !cache.greenhouse.contains_key(board) {
        let api = format!("https://boards-api.greenhouse.io/v1/boards/{board}/jobs");
        let val = http
            .get_json_with::<Value>(&api, RetryPolicy::NONE)
            .await
            .unwrap_or(Value::Null);
        cache.greenhouse.insert(board.to_string(), val);
    }
    let jobs = cache.greenhouse.get(board)?.get("jobs")?.as_array()?;
    let mut live_id: Option<String> = None;
    for job in jobs {
        if json_id(job) == id {
            live_id = Some(id.to_string());
            break;
        }
    }
    if live_id.is_none() {
        if let Some(title) = title {
            for job in jobs {
                let live_title = job.get("title").and_then(Value::as_str).unwrap_or("");
                if titles_match(title, live_title) {
                    let nid = json_id(job);
                    if !nid.is_empty() {
                        live_id = Some(nid);
                        break;
                    }
                }
            }
        }
    }
    let live_id = live_id?;
    if live_id == id {
        return None;
    }
    greenhouse_job_api(http, board, &live_id).await
}

fn json_id(v: &Value) -> String {
    v.get("id")
        .and_then(|x| {
            x.as_u64()
                .map(|n| n.to_string())
                .or_else(|| x.as_i64().map(|n| n.to_string()))
                .or_else(|| x.as_str().map(|s| s.to_string()))
        })
        .unwrap_or_default()
}

fn titles_match(stored: &str, live: &str) -> bool {
    let a = norm_title(stored);
    let b = norm_title(live);
    if a.len() < 8 || b.is_empty() {
        return false;
    }
    a == b || (a.len() >= 40 && b.starts_with(&a))
}

fn norm_title(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn greenhouse_content_html(content: &str) -> Option<String> {
    let mut s = unescape_basic(content);
    if s.contains("&lt;") {
        s = unescape_basic(&s);
    }
    let html = sanitize_html(&s);
    is_substantial(&html).then_some(html)
}

fn greenhouse_ids(
    host: &str,
    segs: &[String],
    query: &[(String, String)],
) -> Option<(String, String)> {
    let board_q = query
        .iter()
        .find(|(k, _)| k == "for" || k == "board")
        .map(|(_, v)| v.clone())
        .filter(|s| !s.is_empty());
    let token = query
        .iter()
        .find(|(k, _)| k == "token" || k == "gh_jid" || (host.contains("greenhouse.io") && k == "id"))
        .map(|(_, v)| v.clone())
        .filter(|s| !s.is_empty());
    let path_id = segs
        .iter()
        .position(|s| s == "jobs" || s == "job" || s == "positions")
        .and_then(|i| segs.get(i + 1).cloned())
        .filter(|s| !s.is_empty() && *s != "embed");
    let id = token.or(path_id)?;
    if let Some(b) = board_q {
        if b != id {
            return Some((b, id));
        }
    }
    if host.contains("greenhouse.io") {
        let skip = ["embed", "embed2", "jobs", "job", "job_app", "job-boards"];
        let board = segs
            .iter()
            .find(|s| {
                let l = s.to_ascii_lowercase();
                !skip.contains(&l.as_str()) && !l.is_empty()
            })?
            .clone();
        if !board.is_empty() && board != id {
            return Some((board, id));
        }
        return None;
    }
    let board = guess_greenhouse_board(host)?;
    if board != id {
        Some((board, id))
    } else {
        None
    }
}

fn guess_greenhouse_board(host: &str) -> Option<String> {
    let h = host.trim_start_matches("www.");
    if h.contains("greenhouse.io") || h.contains("ashbyhq.com") || h.contains("lever.co") {
        return None;
    }
    const SKIP: &[&str] = &[
        "www", "careers", "jobs", "job", "apply", "go", "en", "en-eu", "en-de", "global",
        "com", "io", "ai", "co", "net", "org", "eu", "de", "uk", "us", "app",
    ];
    let board = h
        .split('.')
        .find(|p| !SKIP.contains(p) && p.len() > 1)
        .map(|s| s.to_string())?;
    Some(board)
}

fn extract_from_html(page: &str) -> Option<String> {
    if let Some(html) = jsonld_description(page) {
        return Some(html);
    }
    if let Some(html) = extract_quoted_field(page, "descriptionHtml") {
        let html = sanitize_html(&html);
        if is_substantial(&html) {
            return Some(html);
        }
    }
    if let Some(html) = next_data_description(page).filter(|s| is_substantial(s)) {
        return Some(html);
    }
    if let Some(html) = extract_best_quoted_html(page, "content") {
        return Some(html);
    }
    if let Some(html) = description_block(page) {
        return Some(html);
    }
    None
}

fn is_substantial(html: &str) -> bool {
    let t = html.trim();
    if t.len() >= 400 {
        return true;
    }
    t.contains("<ul") || t.contains("<ol") || t.contains("<li") || t.contains("<h2") || t.contains("<h3")
}

fn next_data_description(page: &str) -> Option<String> {
    let start = page.find("__NEXT_DATA__")?;
    let after = &page[start..];
    let gt = after.find('>')?;
    let rest = &after[gt + 1..];
    let end = rest.find("</script")?;
    let v: Value = serde_json::from_str(rest[..end].trim()).ok()?;
    walk_jd_value(&v)
}

fn walk_jd_value(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) => {
            for key in ["descriptionHtml", "description_html", "content", "description"] {
                if let Some(s) = map.get(key).and_then(Value::as_str) {
                    if looks_like_jd_text(s) {
                        return Some(if s.contains('<') {
                            sanitize_html(s)
                        } else {
                            plain_to_html(s)
                        });
                    }
                }
            }
            map.values().find_map(walk_jd_value)
        }
        Value::Array(a) => a.iter().find_map(walk_jd_value),
        _ => None,
    }
}

fn looks_like_jd_text(s: &str) -> bool {
    s.len() >= 400
        || ((s.contains("<p") || s.contains("<li") || s.contains("<h")) && s.len() >= 200)
}

fn description_block(page: &str) -> Option<String> {
    let lower = page.to_ascii_lowercase();
    for needle in [
        "job-description",
        "job_description",
        "jobdescription",
        "posting-description",
        "job-posting-description",
    ] {
        let Some(pos) = lower.find(needle) else {
            continue;
        };
        let Some(gt) = page[pos..].find('>') else {
            continue;
        };
        let start = pos + gt + 1;
        let chunk = &page[start..page.len().min(start + 80_000)];
        let html = sanitize_html(chunk);
        if is_substantial(&html) {
            return Some(html);
        }
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
            if let Some(html) = jobposting_desc(&v).filter(|s| is_substantial(s)) {
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
    decode_quoted_from(&page[pos + needle.len()..])
}

fn extract_best_quoted_html(page: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\":\"");
    let mut from = 0;
    let mut best: Option<String> = None;
    while let Some(rel) = page[from..].find(&needle) {
        let pos = from + rel;
        if let Some(raw) = decode_quoted_from(&page[pos + needle.len()..]) {
            let html = sanitize_html(&unescape_basic(&raw));
            if is_substantial(&html) {
                let longer = best.as_ref().map(|b| html.len() > b.len()).unwrap_or(true);
                if longer {
                    best = Some(html);
                }
            }
        }
        from = pos + needle.len();
    }
    best
}

fn decode_quoted_from(rest: &str) -> Option<String> {
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
    fn sanitize_keeps_utf8_emoji() {
        let out = sanitize_html("<p>Lock \u{1F512} here</p>");
        assert!(out.contains("\u{1F512}"));
        assert!(out.contains("<p>Lock "));
    }

    #[test]
    fn min_body_counts_visible_chars_not_tags() {
        assert!(!has_min_body(""));
        assert!(!has_min_body("<p>short</p>"));
        let n99 = "a".repeat(99);
        let n100 = "a".repeat(100);
        assert!(!has_min_body(&format!("<p>{n99}</p>")));
        assert!(has_min_body(&format!("<p>{n100}</p>")));
        assert_eq!(visible_char_count("<p>你好世界</p>"), 4);
    }

    #[test]
    fn stored_rejects_apply_source_even_with_og() {
        let html = "<p><strong>Company:</strong> ITFS</p><p><strong>Location:</strong> Warszawa</p>\
            <p>Oferty pracy dla specjalistów oraz narzędzia wspomagające proces rekrutacji. SOLID.Jobs to dużo więcej niż job board.</p>\
            <p><a href=\"https://solid.jobs/x\">Apply / source</a></p>";
        assert!(!stored_description_has_min_body(html));
        let real = format!(
            "<p><strong>Company:</strong> X</p><p>{}</p>",
            "We are looking for an engineer. ".repeat(8)
        );
        assert!(stored_description_has_min_body(&real));
        assert!(!is_structured_jd(&real));
        assert!(is_structured_jd(
            "<p><strong>Company:</strong> X</p><p>A</p><p>B</p><p>C</p>"
        ));
        assert!(is_structured_jd("<p><strong>Company:</strong> X</p><ul><li>A</li></ul>"));
    }

    #[test]
    fn workday_cxs_url_from_posting() {
        let (host, segs, _) = split_url(
            "https://aah.wd5.myworkdayjobs.com/external/job/Charlotte-NC---3311-Beam-Rd/Field-Nurse_R187647",
        )
        .unwrap();
        assert_eq!(
            workday_cxs_url(&host, &segs).as_deref(),
            Some("https://aah.wd5.myworkdayjobs.com/wday/cxs/aah/external/job/Charlotte-NC---3311-Beam-Rd/Field-Nurse_R187647")
        );
    }

    #[test]
    fn tidy_workday_keeps_p_and_lists() {
        let raw = r#"<div><div><p style="x"><span><b>Department:</b></span></p></div></div><p></p>Part time<p><b>Pay Range:</b></p>$38.20<ul><li>PALS</li></ul>"#;
        let out = tidy_workday_html(raw);
        assert!(out.contains("<p><b>Department:</b></p>"), "{out}");
        assert!(out.contains("<p>Part time</p>"), "{out}");
        assert!(out.contains("<p>$38.20</p>"), "{out}");
        assert!(out.contains("<ul><li>PALS</li></ul>"));
        assert!(!out.contains("<div"));
        assert!(!out.contains("<span"));
        assert!(!out.contains("<p></p>"));
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

    #[test]
    fn extract_skips_short_og_keeps_next_data() {
        let page = r#"<meta property="og:description" content="A short teaser about the role.">
<script id="__NEXT_DATA__" type="application/json">{"props":{"pageProps":{"job":{"descriptionHtml":"<h2>About</h2><ul><li>Ship</li></ul>"}}}}</script>"#;
        let out = extract_from_html(page).expect("jd");
        assert!(out.contains("<h2>About</h2>"));
        assert!(out.contains("<li>Ship</li>"));
        assert!(!out.contains("short teaser"));
    }

    #[test]
    fn greenhouse_ids_from_job_boards_path() {
        let (host, segs, q) =
            split_url("https://job-boards.greenhouse.io/airtable/jobs/8498915002").unwrap();
        assert_eq!(
            greenhouse_ids(&host, &segs, &q),
            Some(("airtable".to_string(), "8498915002".to_string()))
        );
    }

    #[test]
    fn greenhouse_ids_from_custom_domain_gh_jid() {
        let (host, segs, q) =
            split_url("https://sumup.com/careers/positions/8213032002?gh_jid=8213032002").unwrap();
        assert_eq!(
            greenhouse_ids(&host, &segs, &q),
            Some(("sumup".to_string(), "8213032002".to_string()))
        );
        let (host, segs, q) = split_url(
            "https://coreweave.com/careers/job?4686914006&board=coreweave&gh_jid=4686914006",
        )
        .unwrap();
        assert_eq!(
            greenhouse_ids(&host, &segs, &q),
            Some(("coreweave".to_string(), "4686914006".to_string()))
        );
        let (host, segs, q) =
            split_url("https://careers.hellofresh.com/global/en/job/8076640?gh_jid=8076640")
                .unwrap();
        assert_eq!(
            greenhouse_ids(&host, &segs, &q),
            Some(("hellofresh".to_string(), "8076640".to_string()))
        );
    }

    #[test]
    fn titles_match_clipped_prefix() {
        assert!(titles_match(
            "Engineering Manager, Continuous Deployment and Cha",
            "Engineering Manager, Continuous Deployment and Change Management"
        ));
        assert!(!titles_match("Engineer", "Engineering Manager, Inference"));
    }

    #[test]
    fn extract_picks_longest_content_not_empty_board_field() {
        let page = r#"{"board":{"content":""},"job":{"content":"\u003ch2\u003eAbout the role\u003c/h2\u003e\u003cul\u003e\u003cli\u003eShip\u003c/li\u003e\u003c/ul\u003e"}}"#;
        let out = extract_from_html(page).expect("jd");
        assert!(out.contains("<h2>About the role</h2>"));
        assert!(out.contains("<li>Ship</li>"));
    }
}
