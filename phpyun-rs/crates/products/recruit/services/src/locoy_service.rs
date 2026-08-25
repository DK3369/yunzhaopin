//! Locoy collector ingest. PHP returned plain numeric codes:
//! 1 = ok, 2 = bad payload, 3 = duplicate, 4 = disabled, 5 = auth/ip.
//!
//! News and jobs are ingested; part-time / resume models stay closed until
//! their create-repos exist (returns 2, same as missing required fields).

use std::collections::HashMap;

use phpyun_core::{clock, AppResult, AppState};
use phpyun_models::article::repo as article_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::job::repo as job_repo;

use crate::site_setting_service;

pub const CODE_OK: &str = "1";
pub const CODE_BAD: &str = "2";
pub const CODE_DUP: &str = "3";
pub const CODE_DISABLED: &str = "4";
pub const CODE_AUTH: &str = "5";

fn field<'a>(post: &'a HashMap<String, String>, key: &str) -> &'a str {
    post.get(key).map(String::as_str).unwrap_or("")
}

fn parse_i32(post: &HashMap<String, String>, key: &str) -> i32 {
    field(post, key).parse().unwrap_or(0)
}

async fn configured_key(state: &AppState) -> Option<String> {
    if let Some(k) = state.config.locoy_key.clone() {
        return Some(k);
    }
    if let Ok(Some(row)) = site_setting_service::get(state, "locoy_key").await {
        if !row.value.is_empty() {
            return Some(row.value);
        }
    }
    None
}

fn keys_equal(a: &str, b: &str) -> bool {
    let aa = a.as_bytes();
    let bb = b.as_bytes();
    if aa.len() != bb.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in aa.iter().zip(bb.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub async fn ingest(
    state: &AppState,
    model: &str,
    key: &str,
    client_ip: &str,
    post: HashMap<String, String>,
) -> AppResult<&'static str> {
    let Some(expected) = configured_key(state).await else {
        return Ok(CODE_DISABLED);
    };
    if key.is_empty() || !keys_equal(key, &expected) {
        tracing::warn!(ip = client_ip, "locoy: invalid key");
        return Ok(CODE_AUTH);
    }
    if let Ok(Some(row)) = site_setting_service::get(state, "locoy_allow_ip").await {
        if !row.value.is_empty() {
            let allowed = row.value.split(',').map(str::trim).any(|ip| ip == client_ip);
            if !allowed {
                tracing::warn!(ip = client_ip, "locoy: ip denied");
                return Ok(CODE_AUTH);
            }
        }
    }

    match model {
        "news" => ingest_news(state, &post).await,
        "job" => ingest_job(state, &post).await,
        "partjob" | "user" => Ok(CODE_BAD),
        _ => Ok(CODE_BAD),
    }
}

async fn ingest_news(state: &AppState, post: &HashMap<String, String>) -> AppResult<&'static str> {
    let title = field(post, "title").trim();
    let content = field(post, "content");
    let nid = parse_i32(post, "nid");
    if title.is_empty() || content.is_empty() || nid <= 0 {
        return Ok(CODE_BAD);
    }
    let now = clock::now_ts();
    let datetime = field(post, "ctime")
        .parse::<i64>()
        .ok()
        .filter(|t| *t > 0)
        .unwrap_or(now);
    let description = {
        let d = field(post, "description").trim();
        if d.is_empty() {
            content.chars().take(180).collect::<String>()
        } else {
            d.to_string()
        }
    };
    let inserted = article_repo::ingest(
        state.db.pool(),
        article_repo::ArticleIngest {
            title,
            nid,
            did: parse_i32(post, "did"),
            author: field(post, "author"),
            description: &description,
            source: field(post, "source"),
            datetime,
            hits: parse_i32(post, "hits").max(0),
            sort: parse_i32(post, "sort").max(0),
            newsphoto: field(post, "newsphoto"),
            s_thumb: field(post, "s_thumb"),
            keyword: field(post, "keyword"),
            content,
        },
    )
    .await?;
    Ok(if inserted.is_some() {
        CODE_OK
    } else {
        CODE_DUP
    })
}

async fn ingest_job(state: &AppState, post: &HashMap<String, String>) -> AppResult<&'static str> {
    let job_name = field(post, "job_name").trim();
    let com_name = field(post, "com_name").trim();
    if job_name.is_empty() || com_name.is_empty() {
        return Ok(CODE_BAD);
    }
    let Some(uid) = company_repo::find_uid_by_name(state.db.reader(), com_name).await? else {
        return Ok(CODE_DUP);
    };
    let now = clock::now_ts();
    let id = job_repo::create(
        state.db.pool(),
        job_repo::JobCreate {
            uid,
            com_name: Some(com_name),
            name: job_name,
            job1: parse_i32(post, "job_cate"),
            job1_son: 0,
            job_post: 0,
            provinceid: 0,
            cityid: parse_i32(post, "job_city"),
            three_cityid: 0,
            minsalary: parse_i32(post, "minsalary"),
            maxsalary: parse_i32(post, "maxsalary"),
            job_type: parse_i32(post, "type"),
            number: 0,
            exp: parse_i32(post, "exp"),
            edu: parse_i32(post, "edu"),
            description: Some(field(post, "description")).filter(|s| !s.is_empty()),
            welfare: None,
            sdate: now,
            edate: now + 90 * 86_400i64,
            did: 0,
        },
        now,
    )
    .await?;
    let _ = id;
    Ok(CODE_OK)
}
