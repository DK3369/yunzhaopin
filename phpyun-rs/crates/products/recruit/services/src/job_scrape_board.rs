//! List jobs from public ATS board APIs (Greenhouse / Ashby / Lever / Workday).

use phpyun_core::http_client::{Http, RetryPolicy};
use serde_json::{json, Value};

use crate::job_scrape_jd::JdCache;

const BOARD_CAP: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardJob {
    pub url: String,
    pub company: String,
    pub role: String,
    pub location: String,
    pub posted_at: String,
}

pub fn is_supported(provider: &str) -> bool {
    matches!(
        provider,
        "greenhouse" | "ashby" | "lever" | "workday"
    )
}

pub async fn list_jobs(
    http: &Http,
    cache: &mut JdCache,
    provider: &str,
    careers_url: &str,
    api_url: &str,
    company: &str,
) -> Result<Vec<BoardJob>, String> {
    match provider {
        "greenhouse" => list_greenhouse(http, cache, careers_url, api_url, company).await,
        "ashby" => list_ashby(http, cache, careers_url, api_url, company).await,
        "lever" => list_lever(http, careers_url, api_url, company).await,
        "workday" => list_workday(http, careers_url, api_url, company).await,
        _ => Ok(Vec::new()),
    }
}

fn cap(mut jobs: Vec<BoardJob>) -> Vec<BoardJob> {
    if jobs.len() > BOARD_CAP {
        jobs.truncate(BOARD_CAP);
    }
    jobs
}

fn text_field(v: &Value, keys: &[&str]) -> String {
    for k in keys {
        if let Some(s) = v.get(*k).and_then(Value::as_str) {
            let t = s.trim();
            if !t.is_empty() {
                return t.to_string();
            }
        }
    }
    String::new()
}

fn location_of(v: &Value) -> String {
    if let Some(s) = v.get("location").and_then(Value::as_str) {
        return s.trim().to_string();
    }
    if let Some(s) = v
        .get("location")
        .and_then(|x| x.get("name"))
        .and_then(Value::as_str)
    {
        return s.trim().to_string();
    }
    if let Some(s) = v.get("locationName").and_then(Value::as_str) {
        return s.trim().to_string();
    }
    if let Some(s) = v
        .pointer("/categories/location")
        .and_then(Value::as_str)
    {
        return s.trim().to_string();
    }
    if let Some(s) = v.get("locationsText").and_then(Value::as_str) {
        return s.trim().to_string();
    }
    String::new()
}

fn split_host_path(url: &str) -> Option<(String, Vec<String>)> {
    let rest = url
        .trim()
        .split_once('#')
        .map(|(a, _)| a)
        .unwrap_or(url.trim())
        .split_once('?')
        .map(|(a, _)| a)
        .unwrap_or(url.trim());
    let rest = rest
        .strip_prefix("https://")
        .or_else(|| rest.strip_prefix("http://"))?;
    let mut parts = rest.split('/');
    let host = parts.next()?.to_ascii_lowercase();
    let segs: Vec<String> = parts
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Some((host, segs))
}

fn greenhouse_board(url: &str, api_url: &str) -> Option<String> {
    if let Some(board) = board_from_greenhouse_api(api_url) {
        return Some(board);
    }
    let (host, segs) = split_host_path(url)?;
    if !host.contains("greenhouse.io") {
        return None;
    }
    let skip = ["embed", "embed2", "jobs", "job", "job_app", "job-boards"];
    segs.into_iter().find(|s| {
        let l = s.to_ascii_lowercase();
        !skip.contains(&l.as_str()) && !l.is_empty()
    })
}

fn board_from_greenhouse_api(api_url: &str) -> Option<String> {
    let (_, segs) = split_host_path(api_url)?;
    let i = segs.iter().position(|s| s == "boards")?;
    segs.get(i + 1)
        .cloned()
        .filter(|s| !s.is_empty() && s != "jobs")
}

fn ashby_board(url: &str, api_url: &str) -> Option<String> {
    if let Some((_, segs)) = split_host_path(api_url) {
        if let Some(i) = segs.iter().position(|s| s == "job-board") {
            if let Some(slug) = segs.get(i + 1).filter(|s| !s.is_empty()) {
                return Some(slug.clone());
            }
        }
    }
    let (host, segs) = split_host_path(url)?;
    if !host.contains("ashbyhq.com") {
        return None;
    }
    segs.into_iter().find(|s| !s.is_empty() && s != "jobs" && s != "api")
}

fn lever_company(url: &str, api_url: &str) -> Option<String> {
    if let Some((_, segs)) = split_host_path(api_url) {
        if let Some(i) = segs.iter().position(|s| s == "postings") {
            if let Some(slug) = segs.get(i + 1).filter(|s| !s.is_empty()) {
                return Some(slug.clone());
            }
        }
    }
    let (host, segs) = split_host_path(url)?;
    if !host.contains("lever.co") {
        return None;
    }
    segs.into_iter().find(|s| !s.is_empty() && s != "jobs")
}

fn is_locale_seg(s: &str) -> bool {
    let s = s.replace('_', "-");
    let parts: Vec<&str> = s.split('-').collect();
    let word = |p: &str| p.len() == 2 && p.chars().all(|c| c.is_ascii_alphabetic());
    match parts.as_slice() {
        [a] => word(a),
        [a, b] => word(a) && word(b),
        _ => false,
    }
}

pub fn workday_tenant_site(url: &str) -> Option<(String, String, String)> {
    let (host, segs) = split_host_path(url)?;
    if !host.contains("myworkdayjobs.com") {
        return None;
    }
    let tenant = host.split('.').next()?.to_string();
    if tenant.is_empty() || tenant.starts_with("wd") {
        return None;
    }
    let site = segs
        .into_iter()
        .find(|s| !is_locale_seg(s) && !s.eq_ignore_ascii_case("job"))?;
    Some((host, tenant, site))
}

async fn list_greenhouse(
    http: &Http,
    cache: &mut JdCache,
    careers_url: &str,
    api_url: &str,
    company: &str,
) -> Result<Vec<BoardJob>, String> {
    let board = greenhouse_board(careers_url, api_url)
        .ok_or_else(|| "greenhouse_board".to_string())?;
    let api = if api_url.contains("boards-api.greenhouse.io") {
        api_url.trim().trim_end_matches('/').to_string()
    } else {
        format!("https://boards-api.greenhouse.io/v1/boards/{board}/jobs")
    };
    let val: Value = http
        .get_json_with(&api, RetryPolicy::NONE)
        .await
        .map_err(|e| e.to_string())?;
    cache.put_greenhouse(board.clone(), val.clone());
    let mut out = Vec::new();
    let jobs = val.get("jobs").and_then(Value::as_array).cloned().unwrap_or_default();
    for job in jobs {
        let url = text_field(&job, &["absolute_url", "url"]);
        let role = text_field(&job, &["title"]);
        if url.is_empty() || role.is_empty() {
            continue;
        }
        out.push(BoardJob {
            url,
            company: company.to_string(),
            role,
            location: location_of(&job),
            posted_at: text_field(&job, &["updated_at", "first_published"]),
        });
    }
    Ok(cap(out))
}

async fn list_ashby(
    http: &Http,
    cache: &mut JdCache,
    careers_url: &str,
    api_url: &str,
    company: &str,
) -> Result<Vec<BoardJob>, String> {
    let board = ashby_board(careers_url, api_url).ok_or_else(|| "ashby_board".to_string())?;
    let api = if api_url.contains("ashbyhq.com") {
        api_url.trim().trim_end_matches('/').to_string()
    } else {
        format!("https://api.ashbyhq.com/posting-api/job-board/{board}")
    };
    let val: Value = http
        .get_json_with(&api, RetryPolicy::NONE)
        .await
        .map_err(|e| e.to_string())?;
    cache.put_ashby(board, val.clone());
    let mut out = Vec::new();
    let jobs = val.get("jobs").and_then(Value::as_array).cloned().unwrap_or_default();
    for job in jobs {
        let url = text_field(&job, &["jobUrl", "applyUrl", "url"]);
        let role = text_field(&job, &["title"]);
        if url.is_empty() || role.is_empty() {
            continue;
        }
        out.push(BoardJob {
            url,
            company: company.to_string(),
            role,
            location: location_of(&job),
            posted_at: text_field(&job, &["publishedDate", "publishedAt"]),
        });
    }
    Ok(cap(out))
}

async fn list_lever(
    http: &Http,
    careers_url: &str,
    api_url: &str,
    company: &str,
) -> Result<Vec<BoardJob>, String> {
    let slug = lever_company(careers_url, api_url).ok_or_else(|| "lever_company".to_string())?;
    let api = if api_url.contains("api.lever.co") {
        api_url.trim().trim_end_matches('/').to_string()
    } else {
        format!("https://api.lever.co/v0/postings/{slug}")
    };
    let val: Value = http
        .get_json_with(&api, RetryPolicy::NONE)
        .await
        .map_err(|e| e.to_string())?;
    let jobs = val.as_array().cloned().unwrap_or_default();
    let mut out = Vec::new();
    for job in jobs {
        let url = text_field(&job, &["hostedUrl", "applyUrl", "url"]);
        let role = text_field(&job, &["text", "title"]);
        if url.is_empty() || role.is_empty() {
            continue;
        }
        let posted = job
            .get("createdAt")
            .and_then(|x| {
                x.as_i64()
                    .map(|n| n.to_string())
                    .or_else(|| x.as_u64().map(|n| n.to_string()))
                    .or_else(|| x.as_str().map(|s| s.to_string()))
            })
            .unwrap_or_default();
        out.push(BoardJob {
            url,
            company: company.to_string(),
            role,
            location: location_of(&job),
            posted_at: posted,
        });
    }
    Ok(cap(out))
}

async fn list_workday(
    http: &Http,
    careers_url: &str,
    api_url: &str,
    company: &str,
) -> Result<Vec<BoardJob>, String> {
    let (host, tenant, site) = if api_url.contains("/wday/cxs/") {
        let (host, segs) = split_host_path(api_url).ok_or_else(|| "workday_api".to_string())?;
        let i = segs.iter().position(|s| s == "cxs").ok_or_else(|| "workday_cxs".to_string())?;
        let tenant = segs.get(i + 1).cloned().filter(|s| !s.is_empty()).ok_or_else(|| "workday_tenant".to_string())?;
        let site = segs.get(i + 2).cloned().filter(|s| !s.is_empty() && *s != "jobs").ok_or_else(|| "workday_site".to_string())?;
        (host, tenant, site)
    } else {
        workday_tenant_site(careers_url).ok_or_else(|| "workday_url".to_string())?
    };
    let mut out = Vec::new();
    let mut offset = 0i32;
    let limit = 20i32;
    while out.len() < BOARD_CAP {
        let api = format!("https://{host}/wday/cxs/{tenant}/{site}/jobs");
        let body = json!({
            "appliedFacets": {},
            "limit": limit,
            "offset": offset,
            "searchText": ""
        });
        let val: Value = http
            .post_json_with(&api, &body, RetryPolicy::NONE)
            .await
            .map_err(|e| e.to_string())?;
        let postings = val
            .get("jobPostings")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if postings.is_empty() {
            break;
        }
        let n = postings.len();
        for job in postings {
            let path = text_field(&job, &["externalPath", "externalUrl"]);
            let role = text_field(&job, &["title"]);
            if path.is_empty() || role.is_empty() {
                continue;
            }
            let mut url = path.clone();
            if !path.starts_with("http://") && !path.starts_with("https://") {
                url = if path.starts_with('/') {
                    format!("https://{host}{path}")
                } else {
                    format!("https://{host}/{path}")
                };
            }
            out.push(BoardJob {
                url,
                company: company.to_string(),
                role,
                location: location_of(&job),
                posted_at: text_field(&job, &["postedOn", "postedDate"]),
            });
            if out.len() >= BOARD_CAP {
                break;
            }
        }
        offset += n as i32;
        if n < limit as usize {
            break;
        }
    }
    Ok(cap(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greenhouse_board_from_careers_and_api() {
        assert_eq!(
            greenhouse_board("https://job-boards.greenhouse.io/anthropic", "").as_deref(),
            Some("anthropic")
        );
        assert_eq!(
            greenhouse_board(
                "https://openai.com/careers",
                "https://boards-api.greenhouse.io/v1/boards/openai/jobs"
            )
            .as_deref(),
            Some("openai")
        );
        assert_eq!(
            greenhouse_board(
                "",
                "https://boards-api.greenhouse.io/v1/boards/polyai/jobs"
            )
            .as_deref(),
            Some("polyai")
        );
    }

    #[test]
    fn ashby_and_lever_slugs() {
        assert_eq!(
            ashby_board("https://jobs.ashbyhq.com/elevenlabs", "").as_deref(),
            Some("elevenlabs")
        );
        assert_eq!(
            lever_company("https://jobs.lever.co/mistral", "").as_deref(),
            Some("mistral")
        );
    }

    #[test]
    fn workday_tenant_from_careers() {
        let (host, tenant, site) = workday_tenant_site(
            "https://nvidia.wd5.myworkdayjobs.com/NVIDIAExternalCareerSite",
        )
        .unwrap();
        assert_eq!(host, "nvidia.wd5.myworkdayjobs.com");
        assert_eq!(tenant, "nvidia");
        assert_eq!(site, "NVIDIAExternalCareerSite");
        let (_, _, site) = workday_tenant_site(
            "https://uwaterloo.wd3.myworkdayjobs.com/en-US/Waterloo",
        )
        .unwrap();
        assert_eq!(site, "Waterloo");
    }

    #[test]
    fn greenhouse_jobs_json_maps_fields() {
        let job = json!({
            "id": 9,
            "title": "iOS Engineer",
            "absolute_url": "https://job-boards.greenhouse.io/x/jobs/9",
            "updated_at": "2026-01-01",
            "location": {"name": "Remote"}
        });
        assert_eq!(text_field(&job, &["title"]), "iOS Engineer");
        assert_eq!(location_of(&job), "Remote");
    }
}
