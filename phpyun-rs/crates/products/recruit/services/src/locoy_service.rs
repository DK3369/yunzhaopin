//! Locoy collector ingest. PHP returned plain numeric codes:
//! 1 = ok, 2 = bad payload, 3 = duplicate, 4 = disabled, 5 = auth/ip.
//!
//! News / full-time jobs / part-time jobs / user (member + resume + expect).

use std::collections::HashMap;

use phpyun_core::{clock, AppResult, AppState};
use phpyun_models::article::repo as article_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::part::repo as part_repo;
use phpyun_models::resume::expect::{self as expect_repo, ExpectInput};
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::user::repo as user_repo;

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
        "partjob" => ingest_partjob(state, &post).await,
        "user" => ingest_user(state, &post).await,
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

async fn ingest_partjob(state: &AppState, post: &HashMap<String, String>) -> AppResult<&'static str> {
    let name = field(post, "part_name").trim();
    let com_name = field(post, "com_name").trim();
    if name.is_empty() || com_name.is_empty() {
        return Ok(CODE_BAD);
    }
    let Some(uid) = company_repo::find_uid_by_name(state.db.reader(), com_name).await? else {
        return Ok(CODE_DUP);
    };
    if part_repo::find_id_by_uid_name(state.db.reader(), uid, name)
        .await?
        .is_some()
    {
        return Ok(CODE_DUP);
    }
    let now = clock::now_ts();
    let sdate = parse_ts(post, "sdate").unwrap_or(now);
    let edate = parse_ts(post, "edate").unwrap_or(0);
    let content = {
        let c = field(post, "partcontent");
        if c.is_empty() {
            field(post, "content")
        } else {
            c
        }
    };
    let salary_type = parse_i32(post, "salary_type");
    let _ = part_repo::locoy_create(
        state.db.pool(),
        &part_repo::LocoyPartCreate {
            uid,
            name,
            com_name,
            r#type: parse_i32(post, "type"),
            provinceid: parse_i32(post, "provinceid"),
            cityid: parse_i32(post, "job_city").max(parse_i32(post, "city")),
            three_cityid: parse_i32(post, "three_cityid"),
            address: field(post, "address"),
            number: parse_i32(post, "number"),
            sex: parse_i32(post, "sex"),
            salary: parse_i32(post, "salary"),
            salary_type: if salary_type > 0 { salary_type } else { 15 },
            billing_cycle: parse_i32(post, "billing_cycle"),
            worktime: field(post, "worktime"),
            sdate,
            edate,
            content,
            linkman: field(post, "linkman"),
            linktel: {
                let t = field(post, "linktel");
                if t.is_empty() {
                    field(post, "moblie")
                } else {
                    t
                }
            },
            state: 0,
            x: field(post, "x"),
            y: field(post, "y"),
            deadline: now + 7 * 86_400,
            now,
            did: u32::try_from(parse_i32(post, "did").max(0)).unwrap_or(0),
        },
    )
    .await?;
    Ok(CODE_OK)
}

fn parse_ts(post: &HashMap<String, String>, key: &str) -> Option<i64> {
    let s = field(post, key).trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<i64>().ok().filter(|t| *t > 0)
}

fn locoy_sex(post: &HashMap<String, String>) -> i32 {
    match field(post, "info_sex").trim() {
        "男" | "1" => 1,
        "女" | "2" => 2,
        _ => 0,
    }
}

fn random_locoy_username(prefix: &str, length: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let u = uuid::Uuid::now_v7();
    let bytes = u.as_bytes();
    let extra: String = (0..length)
        .map(|i| {
            let idx = usize::from(bytes[i % bytes.len()]) % CHARS.len();
            char::from(CHARS[idx])
        })
        .collect();
    format!("{prefix}{extra}")
}

async fn locoy_setting(state: &AppState, key: &str) -> String {
    site_setting_service::get(state, key)
        .await
        .ok()
        .flatten()
        .map(|r| r.value)
        .unwrap_or_default()
}

async fn ingest_user(state: &AppState, post: &HashMap<String, String>) -> AppResult<&'static str> {
    let info_name = field(post, "info_name").trim();
    if info_name.is_empty() {
        return Ok(CODE_BAD);
    }
    let now = clock::now_ts();
    let mobile = field(post, "info_telphone").trim();
    let email = field(post, "info_email").trim();
    let uid = if let Some(uid) = resume_repo::find_uid_by_name(state.db.reader(), info_name).await? {
        uid
    } else {
        let prefix = {
            let p = locoy_setting(state, "locoy_name").await;
            if p.is_empty() {
                "locoy".to_string()
            } else {
                p
            }
        };
        let length = locoy_setting(state, "locoy_length")
            .await
            .parse::<usize>()
            .ok()
            .filter(|n| (4..=32).contains(n))
            .unwrap_or(8);
        let pwd = {
            let p = locoy_setting(state, "locoy_pwd").await;
            if p.is_empty() {
                "123456".to_string()
            } else {
                p
            }
        };
        let salt = uuid::Uuid::now_v7()
            .simple()
            .to_string()
            .chars()
            .take(16)
            .collect::<String>();
        let hash = phpyun_auth::argon2_hash_async(format!("{pwd}{salt}")).await?;
        let mut created: Option<u64> = None;
        for _ in 0..5 {
            let username = random_locoy_username(&prefix, length);
            if user_repo::exists_username(state.db.pool(), &username).await? {
                continue;
            }
            match user_repo::create_member(
                state.db.pool(),
                &username,
                &hash,
                &salt,
                if mobile.is_empty() { None } else { Some(mobile) },
                if email.is_empty() { None } else { Some(email) },
                1,
                0,
                "",
                now,
            )
            .await
            {
                Ok(uid) => {
                    created = Some(uid);
                    break;
                }
                Err(e) => {
                    tracing::warn!(error = %e, "locoy user member insert");
                    return Err(e.into());
                }
            }
        }
        let Some(uid) = created else {
            return Ok(CODE_DUP);
        };
        resume_repo::ensure_row(state.db.pool(), uid, 0, now).await?;
        let _ = phpyun_models::member_statis::repo::bump_fav_jobnum(state.db.pool(), uid, 0).await;
        let sex = locoy_sex(post);
        resume_repo::update(
            state.db.pool(),
            uid,
            resume_repo::ResumeUpdate {
                name: Some(info_name),
                nametype: None,
                sex: if sex > 0 { Some(sex) } else { None },
                birthday: {
                    let b = field(post, "info_birthday").trim();
                    if b.is_empty() {
                        None
                    } else {
                        Some(b)
                    }
                },
                marriage: None,
                education: None,
                telphone: if mobile.is_empty() { None } else { Some(mobile) },
                email: if email.is_empty() { None } else { Some(email) },
                photo: {
                    let p = field(post, "info_photo").trim();
                    if p.is_empty() {
                        None
                    } else {
                        Some(p)
                    }
                },
            },
            now,
        )
        .await?;
        uid
    };

    let class_name = field(post, "info_classid").trim();
    let expect_name = if class_name.is_empty() {
        info_name
    } else {
        class_name
    };
    let minsalary = parse_i32(post, "minsalary").max(0);
    let maxsalary = parse_i32(post, "maxsalary");
    expect_repo::create(
        state.db.pool(),
        uid,
        &ExpectInput {
            name: Some(expect_name),
            job_classid: 0,
            city_classid: 0,
            salary: 0,
            minsalary,
            maxsalary: if maxsalary > 0 { Some(maxsalary) } else { None },
            r#type: 0,
            report: 0,
            jobstatus: 45,
            hy: 0,
        },
        now,
    )
    .await?;
    Ok(CODE_OK)
}
