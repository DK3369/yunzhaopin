//! English job scrape from a career-ops Next.js site into `phpyun_company_job`.

use std::collections::{HashMap, HashSet};

use phpyun_auth::{argon2_hash_async, md5_hex};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::category::repo as cat_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::dict_i18n::repo as dict_i18n_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::job_scrape::repo as scrape_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::user::repo as user_repo;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::job_scrape_jd::{self, JdCache};

const KEY_URL: &str = "job_scrape_url";
const KEY_ENABLED: &str = "job_scrape_enabled";
const KEY_HOURS: &str = "job_scrape_hours";
const KEY_MINUTES: &str = "job_scrape_minutes";
const KEY_LAST_RUN: &str = "job_scrape_last_run";
const KEY_LAST_MSG: &str = "job_scrape_last_msg";
const LOCK_KEY: &str = "job_scrape:run";
const DEFAULT_URL: &str = "http://72.62.75.195:3000/";
const DEFAULT_JOB1: i32 = 955; // Technology
const MAX_JOBS: usize = 400;
const COMPANY_NAME_MAX: usize = 25;
const JOB_NAME_MAX: usize = 50;
const ADDRESS_MAX: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapedJob {
    pub url: String,
    pub company: String,
    pub role: String,
    pub location: String,
    pub posted_at: String,
}

struct RunStats {
    fetched: i32,
    inserted: i32,
    skipped: i32,
    updated: i32,
    error: String,
}

pub async fn admin_get(state: &AppState) -> AppResult<Value> {
    let cfg = load_config(state).await?;
    let logs = scrape_repo::list_logs(state.db.reader(), 20)
        .await
        .unwrap_or_default();
    let log_rows: Vec<Value> = logs
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "started_at": r.started_at,
                "started_at_n": fmt_dt(r.started_at),
                "finished_at": r.finished_at,
                "finished_at_n": fmt_dt(r.finished_at),
                "fetched": r.fetched,
                "inserted": r.inserted,
                "skipped": r.skipped,
                "error": r.error,
            })
        })
        .collect();
    let running = lock_held(state).await;
    Ok(json!({
        "url": cfg.url,
        "enabled": if cfg.enabled { "1" } else { "0" },
        "hours": cfg.hours,
        "minutes": cfg.minutes,
        "last_run": cfg.last_run,
        "last_run_n": if cfg.last_run > 0 { fmt_dt(cfg.last_run) } else { String::new() },
        "last_msg": cfg.last_msg,
        "running": running,
        "logs": log_rows,
    }))
}

/// Drop a leftover Redis lock after restart (HTTP timeout used to cancel
/// the handler before `release_lock`). Safe: this process is the only runner.
pub async fn clear_run_lock(state: &AppState) {
    if let Err(e) = state.redis.del(LOCK_KEY).await {
        tracing::warn!(error = %e, "job_scrape: clear stale lock failed");
    }
}

async fn lock_held(state: &AppState) -> bool {
    state
        .redis
        .get_str(LOCK_KEY)
        .await
        .ok()
        .flatten()
        .is_some()
}

pub async fn admin_save(state: &AppState, body: &Value) -> AppResult<()> {
    let url = json_str(body, "url");
    let url = if url.is_empty() {
        DEFAULT_URL.to_string()
    } else {
        url
    };
    if !is_http_url(&url) {
        return Err(ApiError::business("job_scrape_bad_url"));
    }
    let enabled = matches!(json_str(body, "enabled").as_str(), "1" | "true");
    let mut hours = json_i32(body, "hours").max(0);
    let mut minutes = json_i32(body, "minutes").max(0).min(59);
    if enabled && hours == 0 && minutes == 0 {
        hours = 6;
        minutes = 0;
    }
    let pool = state.db.pool();
    let now = clock::now_ts();
    setting_repo::upsert(pool, KEY_URL, &url, "", false, now).await?;
    setting_repo::upsert(
        pool,
        KEY_ENABLED,
        if enabled { "1" } else { "0" },
        "",
        false,
        now,
    )
    .await?;
    setting_repo::upsert(pool, KEY_HOURS, &hours.to_string(), "", false, now).await?;
    setting_repo::upsert(pool, KEY_MINUTES, &minutes.to_string(), "", false, now).await?;
    Ok(())
}

pub async fn admin_run(state: &AppState) -> AppResult<Value> {
    let owner = Uuid::now_v7().to_string();
    let got = state
        .redis
        .acquire_lock(LOCK_KEY, &owner, 1_800_000)
        .await?;
    if !got {
        let cfg = load_config(state).await?;
        return Ok(json!({
            "started": false,
            "running": true,
            "fetched": 0,
            "inserted": 0,
            "skipped": 0,
            "updated": 0,
            "error": "",
            "last_msg": cfg.last_msg,
        }));
    }
    let now = clock::now_ts();
    let _ = setting_repo::upsert(state.db.pool(), KEY_LAST_MSG, "采集进行中", "", false, now).await;
    let bg = state.clone();
    tokio::spawn(async move {
        let res = run_inner(&bg).await;
        let _ = bg.redis.release_lock(LOCK_KEY, &owner).await;
        if let Err(e) = res {
            tracing::warn!(error = %e, "job_scrape: run failed");
            let ts = clock::now_ts();
            let _ = setting_repo::upsert(
                bg.db.pool(),
                KEY_LAST_MSG,
                &format!("error: {e}"),
                "",
                false,
                ts,
            )
            .await;
        }
    });
    Ok(json!({
        "started": true,
        "running": true,
        "fetched": 0,
        "inserted": 0,
        "skipped": 0,
        "updated": 0,
        "error": "",
    }))
}

/// Periodic tick: skip unless auto-scrape is on and the interval has elapsed.
pub async fn tick(state: &AppState) {
    let cfg = match load_config(state).await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(error = %e, "job_scrape: load config failed");
            return;
        }
    };
    if !cfg.enabled {
        return;
    }
    let interval = cfg.interval_secs();
    let now = clock::now_ts();
    if cfg.last_run > 0 && now.saturating_sub(cfg.last_run) < interval {
        return;
    }
    if let Err(e) = run_locked(state, false).await {
        tracing::warn!(error = %e, "job_scrape: tick failed");
    }
}

async fn run_locked(state: &AppState, fail_if_busy: bool) -> AppResult<RunStats> {
    let owner = Uuid::now_v7().to_string();
    let got = match state
        .redis
        .acquire_lock(LOCK_KEY, &owner, 1_800_000)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error = %e, "job_scrape: lock acquire failed");
            if fail_if_busy {
                return Err(e);
            }
            return Ok(RunStats {
                fetched: 0,
                inserted: 0,
                skipped: 0,
                updated: 0,
                error: String::new(),
            });
        }
    };
    if !got {
        if fail_if_busy {
            return Err(ApiError::business("job_scrape_busy"));
        }
        return Ok(RunStats {
            fetched: 0,
            inserted: 0,
            skipped: 0,
            updated: 0,
            error: String::new(),
        });
    }
    let result = run_inner(state).await;
    let _ = state.redis.release_lock(LOCK_KEY, &owner).await;
    result
}

async fn run_inner(state: &AppState) -> AppResult<RunStats> {
    let started = clock::now_ts();
    let cfg = load_config(state).await?;
    let mut stats = RunStats {
        fetched: 0,
        inserted: 0,
        skipped: 0,
        updated: 0,
        error: String::new(),
    };
    let mut jd_cache = JdCache::default();
    let pages = scrape_page_urls(&cfg.url);
    let mut jobs: Vec<ScrapedJob> = Vec::new();
    let mut seen = HashSet::new();
    for page in &pages {
        match state.http.get_text(page).await {
            Ok(html) => {
                for job in parse_career_ops_jobs(&html) {
                    if seen.insert(job.url.clone()) {
                        jobs.push(job);
                    }
                }
            }
            Err(e) => {
                tracing::warn!(url = %page, error = %e, "job_scrape: fetch failed");
                if stats.error.is_empty() {
                    stats.error = format!("fetch {page}: {e}");
                }
            }
        }
    }
    if jobs.len() > MAX_JOBS {
        jobs.truncate(MAX_JOBS);
    }
    stats.fetched = jobs.len() as i32;
    let classes = match load_job_classes(state).await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(error = %e, "job_scrape: job class map failed");
            JobClassMap::default()
        }
    };
    let salt: String = Uuid::now_v7()
        .simple()
        .to_string()
        .chars()
        .take(16)
        .collect();
    let pwd = format!("scrape-{started}");
    let password_hash = argon2_hash_async(format!("{pwd}{salt}")).await?;
    let creds = ImportedCreds {
        salt,
        password_hash,
    };
    for job in &jobs {
        match ingest_one(state, job, &classes, &creds, &mut jd_cache).await {
            Ok(IngestOut::Inserted) => stats.inserted += 1,
            Ok(IngestOut::Updated) => stats.updated += 1,
            Ok(IngestOut::Skipped) => stats.skipped += 1,
            Err(e) => {
                tracing::warn!(url = %job.url, error = %e, "job_scrape: ingest failed");
                stats.skipped += 1;
                if stats.error.len() < 200 {
                    if !stats.error.is_empty() {
                        stats.error.push(';');
                    }
                    stats.error.push_str(&e.to_string());
                }
            }
        }
    }
    let finished = clock::now_ts();
    let _ = setting_repo::upsert(
        state.db.pool(),
        KEY_LAST_RUN,
        &finished.to_string(),
        "",
        false,
        finished,
    )
    .await;
    let _ = setting_repo::upsert(
        state.db.pool(),
        KEY_LAST_MSG,
        &format!(
            "fetched={} inserted={} skipped={} updated={}",
            stats.fetched, stats.inserted, stats.skipped, stats.updated
        ),
        "",
        false,
        finished,
    )
    .await;
    let _ = scrape_repo::insert_log(
        state.db.pool(),
        started,
        finished,
        stats.fetched,
        stats.inserted,
        stats.skipped,
        &stats.error,
    )
    .await;
    Ok(stats)
}

struct ImportedCreds {
    salt: String,
    password_hash: String,
}

enum IngestOut {
    Inserted,
    Updated,
    Skipped,
}

async fn ingest_one(
    state: &AppState,
    job: &ScrapedJob,
    classes: &JobClassMap,
    creds: &ImportedCreds,
    jd_cache: &mut JdCache,
) -> AppResult<IngestOut> {
    if let Some(job_id) = scrape_repo::find_job_id_by_url(state.db.reader(), &job.url).await? {
        if job_id > 0 && refresh_stub_description(state, job_id, job, jd_cache).await? {
            return Ok(IngestOut::Updated);
        }
        return Ok(IngestOut::Skipped);
    }
    let uid = ensure_company(state, &job.company, &job.location, creds).await?;
    let job_name = clip_chars(&job.role, JOB_NAME_MAX);
    if job_name.is_empty() {
        return Ok(IngestOut::Skipped);
    }
    if job_repo::find_id_by_uid_name_listed(state.db.reader(), uid, &job_name)
        .await?
        .is_some()
    {
        return Ok(IngestOut::Skipped);
    }
    let now = clock::now_ts();
    let (job1, job1_son, job_post) = classes.match_role(&job.role);
    let com = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("common_06400"))?;
    let com_name = clip_chars(
        &com.name.clone().unwrap_or_else(|| job.company.clone()),
        JOB_NAME_MAX,
    );
    let com_logo = com.logo.clone().unwrap_or_default();
    let description = fetch_job_description(state, job, jd_cache).await;
    let rating = statis_repo::read_rating(state.db.reader(), uid)
        .await
        .unwrap_or(1);
    let write = job_repo::AdminJobWrite {
        uid,
        name: &job_name,
        com_name: &com_name,
        hy: com.hy,
        job1,
        job1_son,
        job_post,
        provinceid: com.provinceid,
        cityid: com.cityid,
        three_cityid: com.three_cityid,
        x: com.x.as_deref().unwrap_or(""),
        y: com.y.as_deref().unwrap_or(""),
        link_id: 0,
        is_link: 1,
        is_message: 1,
        is_email: 1,
        minsalary: 0,
        maxsalary: 0,
        description: &description,
        r_status: 1,
        number: 0,
        exp: 0,
        report: 0,
        age: 0,
        sex: 0,
        edu: 0,
        is_graduate: 0,
        marriage: 0,
        lang: "",
        welfare: "",
        state: 1,
        jobhits: 0,
        jobexpoure: 0,
        exp_req: "",
        edu_req: "",
        zp_num: 0,
        zp_minage: 0,
        zp_maxage: 0,
        minage_req: 0,
        maxage_req: 0,
        sex_req: 0,
        status: 0,
        com_logo: &com_logo,
        com_provinceid: com.provinceid,
        pr: com.pr,
        mun: com.mun,
        did: com.did as i64,
        yyzz_status: com.yyzz_status,
        rating,
    };
    let job_id = job_repo::insert_admin(state.db.pool(), write, now).await?;
    if job_id == 0 {
        return Err(ApiError::business("common_06400"));
    }
    let job_type = if job.location.to_ascii_lowercase().contains("remote") {
        5
    } else {
        1
    };
    let _ = job_repo::admin_set_type(state.db.pool(), job_id, job_type).await;
    let _ = company_repo::touch_jobtime(state.db.pool(), uid, now).await;
    scrape_repo::insert_item(
        state.db.pool(),
        &job.url,
        job_id,
        uid,
        &job_name,
        &job.company,
        now,
    )
    .await?;
    Ok(IngestOut::Inserted)
}

async fn fetch_job_description(
    state: &AppState,
    job: &ScrapedJob,
    jd_cache: &mut JdCache,
) -> String {
    let body = job_scrape_jd::official_body_html(&state.http, jd_cache, &job.url).await;
    job_scrape_jd::compose_description(
        &job.company,
        &job.location,
        &job.posted_at,
        &job.url,
        body.as_deref(),
    )
}

async fn refresh_stub_description(
    state: &AppState,
    job_id: u64,
    job: &ScrapedJob,
    jd_cache: &mut JdCache,
) -> AppResult<bool> {
    let current = job_repo::find_description(state.db.reader(), job_id)
        .await?
        .unwrap_or_default();
    if !job_scrape_jd::is_stub_description(&current) {
        return Ok(false);
    }
    let body = job_scrape_jd::official_body_html(&state.http, jd_cache, &job.url).await;
    let Some(body) = body.filter(|s| !s.trim().is_empty()) else {
        return Ok(false);
    };
    let next = job_scrape_jd::compose_description(
        &job.company,
        &job.location,
        &job.posted_at,
        &job.url,
        Some(&body),
    );
    if next == current || job_scrape_jd::is_stub_description(&next) {
        return Ok(false);
    }
    let n = job_repo::update_description_keep_listed(state.db.pool(), job_id, &next).await?;
    Ok(n > 0)
}

async fn ensure_company(
    state: &AppState,
    name: &str,
    location: &str,
    creds: &ImportedCreds,
) -> AppResult<u64> {
    let stored_name = company_db_name(name);
    if let Some(uid) = company_repo::find_uid_by_name(state.db.reader(), &stored_name).await? {
        return Ok(uid);
    }
    let now = clock::now_ts();
    let digest = md5_hex(&format!("job-scrape:{name}"));
    let mut username = format!("imp{}", &digest[..12]);
    for extra in 0..5u32 {
        if extra > 0 {
            username = format!("imp{}{extra}", &digest[..10]);
        }
        if !user_repo::exists_username(state.db.pool(), &username).await? {
            break;
        }
    }
    let uid = user_repo::create_member(
        state.db.pool(),
        &username,
        &creds.password_hash,
        &creds.salt,
        None,
        None,
        2,
        0,
        "0.0.0.0",
        now,
    )
    .await?;
    let short = clip_chars(&stored_name, COMPANY_NAME_MAX);
    let addr = clip_chars(location, ADDRESS_MAX);
    let last = now.to_string();
    if let Err(e) = company_repo::insert_admin_created(
        state.db.pool(),
        company_repo::AdminCompanyInsert {
            uid,
            name: &stored_name,
            shortname: &short,
            hy: 0,
            pr: 0,
            mun: 0,
            provinceid: 0,
            cityid: 0,
            three_cityid: 0,
            address: &addr,
            x: "",
            y: "",
            linkman: "HR",
            linktel: "",
            linkphone: "",
            linkmail: "",
            content: location,
            lastupdate: &last,
            rating: 1,
            rating_name: "普通会员",
            vipstime: 0,
            vipetime: 0,
        },
    )
    .await
    {
        let _ = user_repo::delete_member(state.db.pool(), uid).await;
        return Err(e.into());
    }
    let _ = statis_repo::ensure_row(state.db.pool(), uid).await;
    Ok(uid)
}

#[derive(Default)]
struct JobClassMap {
    parent: HashMap<i32, i32>,
    names: Vec<(i32, String)>,
}

impl JobClassMap {
    fn match_role(&self, role: &str) -> (i32, i32, i32) {
        let role_l = role.to_ascii_lowercase();
        let mut best: Option<(usize, i32)> = None;
        for (id, name) in &self.names {
            if name.len() < 3 {
                continue;
            }
            if role_l.contains(name) {
                let score = name.len();
                if best.map(|(s, _)| score > s).unwrap_or(true) {
                    best = Some((score, *id));
                }
            }
        }
        let leaf = best.map(|(_, id)| id).unwrap_or_else(|| keyword_leaf(&role_l));
        self.chain(leaf)
    }

    fn chain(&self, leaf: i32) -> (i32, i32, i32) {
        if leaf <= 0 {
            return (DEFAULT_JOB1, 0, 0);
        }
        let mut ids = vec![leaf];
        let mut cur = leaf;
        for _ in 0..6 {
            match self.parent.get(&cur).copied().filter(|p| *p > 0) {
                Some(p) => {
                    ids.push(p);
                    cur = p;
                }
                None => break,
            }
        }
        ids.reverse();
        let job1 = *ids.first().unwrap_or(&DEFAULT_JOB1);
        let job1_son = ids.get(1).copied().unwrap_or(0);
        let job_post = if ids.len() >= 3 {
            *ids.last().unwrap_or(&0)
        } else {
            0
        };
        (job1, job1_son, job_post)
    }
}

fn keyword_leaf(role_l: &str) -> i32 {
    const PAIRS: &[(&str, i32)] = &[
        ("ios", 983),
        ("iphone", 983),
        ("android", 982),
        ("flutter", 986),
        ("rust", 958),
        ("golang", 974),
        ("node.js", 973),
        ("nodejs", 973),
        ("python", 968),
        ("java", 960),
        ("c++", 961),
        ("php", 962),
        ("devops", 1003),
        ("frontend", 987),
        ("front-end", 987),
        ("javascript", 987),
        ("full-stack", 957),
        ("fullstack", 957),
        ("data analyst", 1017),
        ("data scientist", 1016),
        ("qa", 992),
        ("test engineer", 992),
    ];
    for (k, id) in PAIRS {
        if role_l.contains(k) {
            return *id;
        }
    }
    DEFAULT_JOB1
}

async fn load_job_classes(state: &AppState) -> AppResult<JobClassMap> {
    let cats = cat_repo::list_all(state.db.reader(), "job").await?;
    let i18n = dict_i18n_repo::list_by_kind(state.db.reader(), "job")
        .await
        .unwrap_or_default();
    let mut en: HashMap<i32, String> = HashMap::new();
    for (id, lang, text) in i18n {
        if lang == "en" {
            en.insert(id as i32, text);
        }
    }
    let mut parent = HashMap::new();
    let mut names = Vec::new();
    for c in cats {
        let id = c.id as i32;
        parent.insert(id, c.parent_id as i32);
        let n = en
            .get(&id)
            .cloned()
            .unwrap_or_else(|| c.name.clone())
            .to_ascii_lowercase();
        names.push((id, n));
    }
    Ok(JobClassMap { parent, names })
}

struct Cfg {
    url: String,
    enabled: bool,
    hours: i32,
    minutes: i32,
    last_run: i64,
    last_msg: String,
}

impl Cfg {
    fn interval_secs(&self) -> i64 {
        let secs = i64::from(self.hours) * 3600 + i64::from(self.minutes) * 60;
        secs.max(60)
    }
}

async fn load_config(state: &AppState) -> AppResult<Cfg> {
    let map = setting_repo::find_many(
        state.db.reader(),
        &[
            KEY_URL, KEY_ENABLED, KEY_HOURS, KEY_MINUTES, KEY_LAST_RUN, KEY_LAST_MSG,
        ],
    )
    .await?;
    let url = map
        .get(KEY_URL)
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .unwrap_or(DEFAULT_URL)
        .to_string();
    Ok(Cfg {
        url,
        enabled: map.get(KEY_ENABLED).map(String::as_str) == Some("1"),
        hours: map
            .get(KEY_HOURS)
            .and_then(|s| s.parse().ok())
            .unwrap_or(6),
        minutes: map
            .get(KEY_MINUTES)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        last_run: map
            .get(KEY_LAST_RUN)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        last_msg: map.get(KEY_LAST_MSG).cloned().unwrap_or_default(),
    })
}

fn json_str(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::String(s)) => s.trim().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn json_i32(v: &Value, key: &str) -> i32 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn clip_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}

fn company_db_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.chars().count() <= COMPANY_NAME_MAX {
        return clip_chars(trimmed, COMPANY_NAME_MAX);
    }
    let hash = md5_hex(trimmed);
    let suffix = format!("-{}", &hash[..4]);
    let keep = COMPANY_NAME_MAX.saturating_sub(suffix.len());
    format!("{}{suffix}", clip_chars(trimmed, keep))
}

fn is_http_url(url: &str) -> bool {
    let u = url.trim();
    (u.starts_with("http://") || u.starts_with("https://"))
        && !u.contains(' ')
        && u.len() < 400
}

fn origin_of(url: &str) -> String {
    let u = url.trim();
    if let Some(scheme_end) = u.find("://") {
        let rest = &u[scheme_end + 3..];
        if let Some(slash) = rest.find('/') {
            return u[..scheme_end + 3 + slash].to_string();
        }
        return u.trim_end_matches('/').to_string();
    }
    u.trim_end_matches('/').to_string()
}

fn scrape_page_urls(base: &str) -> Vec<String> {
    let origin = origin_of(base);
    let home = if base.trim().ends_with('/') {
        base.trim().to_string()
    } else {
        format!("{}/", base.trim())
    };
    let explore = format!("{origin}/explore");
    let mut out = vec![home.trim_end_matches('/').to_string()];
    if !out.contains(&explore) {
        out.push(explore);
    }
    out
}

fn has_cjk(s: &str) -> bool {
    s.chars().any(|c| {
        let u = c as u32;
        (0x4E00..=0x9FFF).contains(&u) || (0x3400..=0x4DBF).contains(&u)
    })
}

fn is_english_job(company: &str, role: &str) -> bool {
    if has_cjk(company) || has_cjk(role) {
        return false;
    }
    role.chars().any(|c| c.is_ascii_alphabetic())
}

fn strip_marks(s: &str) -> String {
    s.replace("~~", "").trim().to_string()
}

fn clean_field(s: &str) -> String {
    let t = strip_marks(s);
    if t == "$undefined" || t.eq_ignore_ascii_case("undefined") {
        String::new()
    } else {
        t
    }
}

fn take_json_string(s: &str) -> Option<(String, usize)> {
    let b = s.as_bytes();
    if b.first() != Some(&b'"') {
        return None;
    }
    let mut i = 1;
    while i < b.len() {
        match b[i] {
            b'"' => {
                let raw = &s[..=i];
                let val: String = serde_json::from_str(raw).ok()?;
                return Some((val, i + 1));
            }
            b'\\' => {
                i += 1;
                if i < b.len() {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    None
}

fn take_json_object(s: &str) -> Option<&str> {
    let b = s.as_bytes();
    if b.first() != Some(&b'{') {
        return None;
    }
    let mut depth = 0i32;
    let mut i = 0;
    let mut in_str = false;
    let mut esc = false;
    while i < b.len() {
        let c = b[i];
        if in_str {
            if esc {
                esc = false;
            } else if c == b'\\' {
                esc = true;
            } else if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                b'"' => in_str = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&s[..=i]);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

fn extract_next_f_blob(html: &str) -> String {
    let needle = "self.__next_f.push([1,";
    let mut out = String::new();
    let mut from = 0;
    while let Some(pos) = html[from..].find(needle) {
        let abs = from + pos + needle.len();
        let after = &html[abs..];
        if !after.starts_with('"') {
            from = abs + 1;
            continue;
        }
        match take_json_string(after) {
            Some((s, n)) => {
                out.push_str(&s);
                from = abs + n;
            }
            None => from = abs + 1,
        }
    }
    out
}

fn job_from_value(v: &Value) -> Option<ScrapedJob> {
    let url = clean_field(v.get("url")?.as_str()?);
    let company = clean_field(v.get("company")?.as_str()?);
    let role = clean_field(v.get("role")?.as_str()?);
    if url.len() < 8 || !url.starts_with("http") {
        return None;
    }
    if company.is_empty() || role.is_empty() {
        return None;
    }
    if !is_english_job(&company, &role) {
        return None;
    }
    let location = v
        .get("location")
        .and_then(Value::as_str)
        .map(clean_field)
        .unwrap_or_default();
    let posted_at = v
        .get("postedAt")
        .and_then(Value::as_str)
        .map(clean_field)
        .unwrap_or_default();
    Some(ScrapedJob {
        url: url.chars().take(512).collect(),
        company: company.chars().take(120).collect(),
        role: role.chars().take(120).collect(),
        location: location.chars().take(200).collect(),
        posted_at,
    })
}

/// Parse career-ops Next.js RSC / HTML for English job cards.
pub fn parse_career_ops_jobs(html: &str) -> Vec<ScrapedJob> {
    let blob = extract_next_f_blob(html);
    let hay = if blob.is_empty() {
        html.to_string()
    } else {
        format!("{html}\n{blob}")
    };
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut i = 0;
    while let Some(rel) = hay[i..].find(r#"{"done":"#) {
        let start = i + rel;
        if let Some(obj) = take_json_object(&hay[start..]) {
            if let Ok(v) = serde_json::from_str::<Value>(obj) {
                if let Some(job) = job_from_value(&v) {
                    if seen.insert(job.url.clone()) {
                        out.push(job);
                    }
                }
            }
            i = start + 1;
        } else {
            i = start + 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_career_ops_done_and_strikethrough() {
        let html = r#"self.__next_f.push([1,"{\"done\":false,\"url\":\"https://jobs.example.com/a\",\"company\":\"Vapi\",\"role\":\"iOS Developer\",\"location\":\"Remote\",\"compensation\":\"$undefined\",\"postedAt\":\"2026-07-03\"}"])"#;
        // The payload above is already a JSON string inside push; simulate unescaped blob in HTML too:
        let html2 = r#"{"done":false,"url":"https://jobs.example.com/a","company":"Vapi","role":"iOS Developer","location":"Remote","compensation":"$undefined","postedAt":"2026-07-03"}{"done":true,"url":"~~https://jobs.example.com/b","company":"SumUp","role":"Senior iOS Engineer~~","location":"London","compensation":"3.6/5","postedAt":"2026-03-23"}{"done":false,"url":"https://jobs.example.com/zh","company":"测试","role":"工程师","location":"北京","compensation":"","postedAt":"2026-01-01"}"#;
        let _ = html;
        let jobs = parse_career_ops_jobs(html2);
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].company, "Vapi");
        assert_eq!(jobs[0].role, "iOS Developer");
        assert_eq!(jobs[1].url, "https://jobs.example.com/b");
        assert_eq!(jobs[1].role, "Senior iOS Engineer");
    }

    #[test]
    fn origin_strips_path() {
        assert_eq!(
            origin_of("http://72.62.75.195:3000/"),
            "http://72.62.75.195:3000"
        );
        assert_eq!(
            origin_of("http://72.62.75.195:3000/explore"),
            "http://72.62.75.195:3000"
        );
    }

    #[test]
    fn company_name_fits_varchar25() {
        let n = company_db_name("This Company Name Is Definitely Longer Than Twenty Five");
        assert!(n.chars().count() <= 25, "{n}");
        assert_eq!(company_db_name("Vapi").chars().count(), 4);
    }
}
