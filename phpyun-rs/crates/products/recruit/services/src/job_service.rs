//! Public job-browsing service (WAP entry point).
//!
//! Implements the list and detail portions of PHPYun `wap/job::index_action` +
//! `wap/job::comapply_action`. Application submission lives in `apply_service`.

use chrono::Timelike;
use phpyun_core::utils::mask_contact;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::site_setting::repo as setting_repo;

/// Public search parameters. Field set mirrors PHPYun's WAP `wap/job` finder
/// + the `joblist` Smarty plugin (`smarty_internal_compile_joblist.php`).
#[derive(Debug, Default, Clone)]
pub struct JobSearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub job_type: Option<i32>,
    /// Industry dict id (`hy`).
    pub hy: Option<i32>,
    /// Gender dict id (`sex`).
    pub sex: Option<i32>,
    /// Salary cycle dict id (`report` — 月薪/年薪/时薪).
    pub report: Option<i32>,
    /// Company nature dict id (`pr`).
    pub pr: Option<i32>,
    /// Company size dict id (`mun`).
    pub mun: Option<i32>,
    /// Welfare DICT ID — service layer resolves to the name before sending
    /// to the repo (PHP stores welfare as a CSV of names, so the WHERE is a
    /// `LIKE '%<name>%'`).
    pub welfare: Option<i32>,
    /// Refresh-time bucket in days (1, 3, 7, 30, 90).
    pub uptime: Option<i32>,
    pub urgent: bool,
    pub rec: bool,
    pub cert: bool,
    /// PHP `bid=1` / `xsdate > now`.
    pub bid: bool,
    /// PHP `order=lastdate|sdate`
    pub order: Option<String>,
    pub uid: Option<u64>,
    pub did: u32,
}

pub struct JobPage {
    pub list: Vec<Job>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &JobSearch,
    page: Pagination,
) -> AppResult<JobPage> {
    let now = clock::now_ts();

    // Resolve the welfare id to its dict NAME (PHP: `$comclass_name[$id]`),
    // so the repo can do `welfare LIKE '%<name>%'`. Empty / unresolved ids
    // become None (== filter not applied).
    let dicts = crate::dict_service::get(state).await?;
    let welfare_name: Option<String> = match search.welfare {
        Some(id) if id > 0 => {
            let name = dicts.comclass(id);
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        }
        _ => None,
    };
    let edu_ids: Vec<i32> = search
        .edu
        .filter(|v| *v > 0)
        .map(|v| dicts.downward_comclass_ids("job_edu", v, false))
        .unwrap_or_default();
    let exp_ids: Vec<i32> = search
        .exp
        .filter(|v| *v > 0)
        .map(|v| dicts.downward_comclass_ids("job_exp", v, false))
        .unwrap_or_default();
    let kw_trim = search
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let keyword_city_ids: Vec<i32> = kw_trim
        .map(|k| dicts.city_ids_containing(k))
        .unwrap_or_default();
    let keyword_job_ids: Vec<i32> = kw_trim
        .map(|k| dicts.job_ids_containing(k))
        .unwrap_or_default();
    let uptime = crate::site_gate_service::default_uptime_days(
        state,
        search.uptime,
        "sy_datacycle_job",
    )
    .await;
    let keyword_full_text =
        crate::site_gate_service::setting_i32(state, "job_full_text_search").await == 1;

    let f = JobFilter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        job1: search.job1,
        job1_son: search.job1_son,
        job_post: search.job_post,
        min_salary: search.min_salary,
        max_salary: search.max_salary,
        exp: search.exp,
        edu: search.edu,
        job_type: search.job_type,
        hy: search.hy,
        sex: search.sex,
        report: search.report,
        pr: search.pr,
        mun: search.mun,
        welfare: welfare_name.as_deref(),
        uptime,
        urgent: search.urgent,
        rec: search.rec,
        cert: search.cert,
        bid: search.bid,
        order: search.order.as_deref(),
        uid: search.uid,
        did: search.did,
        keyword_full_text,
        edu_ids: if edu_ids.is_empty() {
            None
        } else {
            Some(edu_ids.as_slice())
        },
        exp_ids: if exp_ids.is_empty() {
            None
        } else {
            Some(exp_ids.as_slice())
        },
        keyword_city_ids: if keyword_city_ids.is_empty() {
            None
        } else {
            Some(keyword_city_ids.as_slice())
        },
        keyword_job_ids: if keyword_job_ids.is_empty() {
            None
        } else {
            Some(keyword_job_ids.as_slice())
        },
        ..Default::default()
    };

    // Run count + list concurrently to cut RTT
    let (total_res, list_res) = tokio::join!(
        job_repo::count_public(state.db.reader(), &f, now),
        job_repo::list_public(state.db.reader(), &f, page.offset, page.limit, now),
    );
    let list = list_res?;
    let ids: Vec<u64> = list.iter().map(|j| j.id).collect();
    if !ids.is_empty() {
        let pool = state.db.pool().clone();
        phpyun_core::background::spawn_best_effort("job.expoure", async move {
            let _ = job_repo::incr_jobexpoure(&pool, &ids).await;
        });
    }
    Ok(JobPage {
        total: total_res?,
        list,
    })
}

/// Public detail — approved jobs still render when stopped or expired (PHP
/// comapply stamp). Unreviewed / rejected remain hidden except to the owner.
pub async fn get_public(
    state: &AppState,
    id: u64,
    viewer: Option<&AuthenticatedUser>,
) -> AppResult<Job> {
    let j = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(ApiError::business("job_not_found"))?;
    if j.state != 1 || j.r_status != 1 {
        if viewer.is_some_and(|u| u.uid == j.uid) {
            return Ok(j);
        }
        return Err(ApiError::business("job_pending"));
    }
    Ok(j)
}

/// Job detail + company info + most recent HR login — full payload from PHPYun `comapply_action`.
pub struct JobDetailData {
    pub job: Job,
    pub com_logo: String,
    pub com_provinceid: i32,
    pub com_cityid: i32,
    pub com_mun: i32,
    pub com_pr: i32,
    pub com_hy: i32,
    pub com_rating: i32,
    pub comqcode: String,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub login_date: i64,
    pub com_address: String,
    pub com_name: String,
    pub yyzz_status: i32,
    pub moblie_status: i32,
    pub email_status: i32,
    pub fact_status: i32,
    pub money: i32,
    pub content: String,
    pub linkjob: String,
    /// PHP `$stop==1` / `$job.status==1` — not recruiting.
    pub offline: bool,
    /// `edate` in the past (still shown with a stamp).
    pub expired: bool,
}

pub async fn get_detail(
    state: &AppState,
    id: u64,
    viewer: Option<&AuthenticatedUser>,
) -> AppResult<JobDetailData> {
    let job = get_public(state, id, viewer).await?;
    let now = clock::now_ts();
    let offline = job.status != 0;
    let expired = job.edate > 0 && job.edate <= now;
    let db = state.db.reader();

    // Look up the company (JOIN-style call; user uid == company uid)
    let company = phpyun_models::company::repo::find_by_uid(db, job.uid).await?;

    // HR's last login time (read from phpyun_member)
    let login_date = phpyun_models::user::repo::login_date(db, job.uid).await?;

    // Increment view counter (background task)
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("job.hits", async move {
        let _ = phpyun_models::job::repo::incr_jobhits(&pool, id).await;
    });

    let (
        com_logo,
        com_provinceid,
        com_cityid,
        com_mun,
        com_pr,
        com_hy,
        com_rating,
        comqcode,
        linkman,
        com_address,
        com_name,
        yyzz_status,
        moblie_status,
        email_status,
        fact_status,
        money,
        content,
        linkjob,
    ) = if let Some(c) = company {
        (
            c.logo.unwrap_or_default(),
            c.provinceid,
            c.cityid,
            c.mun,
            c.pr,
            c.hy,
            c.rating,
            c.comqcode.unwrap_or_default(),
            c.linkman.unwrap_or_default(),
            c.address.unwrap_or_default(),
            c.name.unwrap_or_default(),
            c.yyzz_status,
            c.moblie_status,
            c.email_status,
            c.fact_status,
            c.money,
            c.content.unwrap_or_default(),
            c.linkjob.unwrap_or_default(),
        )
    } else {
        (
            String::new(),
            0,
            0,
            0,
            0,
            0,
            0,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            0,
            0,
            0,
            0,
            0,
            String::new(),
            String::new(),
        )
    };

    Ok(JobDetailData {
        job,
        com_logo,
        com_provinceid,
        com_cityid,
        com_mun,
        com_pr,
        com_hy,
        com_rating,
        comqcode,
        linkman,
        linktel: String::new(),
        linkphone: String::new(),
        linkmail: String::new(),
        login_date,
        com_address,
        com_name,
        yyzz_status,
        moblie_status,
        email_status,
        fact_status,
        money,
        content,
        linkjob,
        offline,
        expired,
    })
}

/// Public contact payload aligned with PHP `getCompanyJobTel` + `setCompanyLink`.
/// Full telephone is only present when `revealed`; email is never returned.
#[derive(Debug, Clone)]
pub struct PublicJobContact {
    pub job_id: u64,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linktel_n: String,
    pub linkphone_n: String,
    pub address: String,
    pub city_id: i32,
    pub x: String,
    pub y: String,
    pub link_code: i32,
    pub link_msg: String,
    pub link_sub: i32,
    pub revealed: bool,
    pub prvlinktel: String,
    pub prvtime: String,
}

fn cfg_i32(map: &std::collections::HashMap<String, String>, key: &str, default: i32) -> i32 {
    map.get(key)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(default)
}

fn cfg_csv_has(map: &std::collections::HashMap<String, String>, key: &str, rating: i32) -> bool {
    let raw = map.get(key).map(|s| s.trim()).unwrap_or("");
    if raw.is_empty() {
        return false;
    }
    raw.split(',')
        .any(|p| p.trim().parse::<i32>().ok() == Some(rating))
}

fn not_disturb_blocks(raw: &str, now: i64) -> bool {
    let parts: Vec<&str> = raw.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    fn hm(s: &str) -> Option<(u32, u32)> {
        let mut it = s.trim().split(':');
        let h = it.next()?.trim().parse().ok()?;
        let m = it.next().unwrap_or("0").trim().parse().ok()?;
        Some((h, m))
    }
    let Some((sh, sm)) = hm(parts[0]) else {
        return false;
    };
    let Some((eh, em)) = hm(parts[1]) else {
        return false;
    };
    let Some(dt) = chrono::DateTime::from_timestamp(now, 0) else {
        return false;
    };
    let cur = dt.hour() * 60 + dt.minute();
    let start = sh * 60 + sm;
    let end = eh * 60 + em;
    if start > end {
        // e.g. 22:00-06:00
        !(cur < start && cur > end)
    } else {
        cur > start && cur < end
    }
}

/// PHP `job.model.php::getCompanyJobTel` / `setCompanyLink` for the public site.
pub async fn resolve_job_contact(
    state: &AppState,
    job_id: u64,
    user: Option<&AuthenticatedUser>,
    isgetprv: bool,
) -> AppResult<PublicJobContact> {
    let raw = job_repo::get_job_contact(state.db.reader(), job_id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("job_not_found"))?;
    resolve_public_contact(state, raw, job_id, user, isgetprv).await
}

/// Company detail uses the same `setCompanyLink` gate, without a job overlay.
pub async fn resolve_company_contact(
    state: &AppState,
    com_uid: u64,
    user: Option<&AuthenticatedUser>,
    isgetprv: bool,
) -> AppResult<PublicJobContact> {
    let raw = job_repo::get_company_contact(state.db.reader(), com_uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("company_not_found"))?;
    resolve_public_contact(state, raw, 0, user, isgetprv).await
}

async fn resolve_public_contact(
    state: &AppState,
    raw: phpyun_models::job::repo::JobContact,
    job_id: u64,
    user: Option<&AuthenticatedUser>,
    isgetprv: bool,
) -> AppResult<PublicJobContact> {

    let cfg = setting_repo::find_many(
        state.db.reader(),
        &[
            "com_login_link",
            "com_link_look",
            "com_link_no",
            "sy_link_tips",
            "sy_comprivacy_open",
            "sy_privacy_rating",
        ],
    )
    .await
    .unwrap_or_default();

    let hidden_tip = {
        let custom = cfg.get("sy_link_tips").map(|s| s.trim()).unwrap_or("");
        if custom.is_empty() {
            "admin_user_company_00263".to_string()
        } else {
            custom.to_string()
        }
    };

    let mut link_code: i32 = 0;
    let mut link_msg = String::new();
    let mut link_sub: i32 = 0;

    let uid = user.map(|u| u.uid);
    let usertype = user.map(|u| i32::from(u.usertype)).unwrap_or(0);

    if uid == Some(raw.com_uid) {
        link_code = 1;
    } else if raw.infostatus == 2 {
        link_msg = hidden_tip.clone();
        link_code = 2;
    } else if job_id > 0 && raw.is_link == 3 {
        link_msg = hidden_tip.clone();
        link_code = 3;
    } else if cfg_csv_has(&cfg, "com_link_no", raw.rating) {
        link_msg = hidden_tip.clone();
        link_code = 4;
    } else if cfg_i32(&cfg, "com_link_look", 1) == 1 {
        match cfg_i32(&cfg, "com_login_link", 1) {
            2 => {
                link_msg = hidden_tip.clone();
                link_code = 5;
            }
            3 => {
                if usertype != 1 {
                    link_msg = "common_01411".to_string();
                    link_code = 6;
                }
            }
            4 => {
                if usertype != 1 {
                    link_msg = "common_01411".to_string();
                    link_code = 6;
                } else if let Some(uid) = uid {
                    let n = phpyun_models::resume::expect::count_by_uid(state.db.reader(), uid)
                        .await
                        .unwrap_or(0);
                    if n == 0 {
                        link_msg = "common_01541".to_string();
                        link_code = 7;
                    } else if let Ok(Some((state_n, _status))) =
                        phpyun_models::resume::expect::find_default_state_by_uid(
                            state.db.reader(),
                            uid,
                        )
                        .await
                    {
                        if state_n != 1 {
                            link_msg = "wap_00369".to_string();
                            link_code = 7;
                            link_sub = 1;
                        }
                    }
                }
            }
            5 => {
                if usertype != 1 {
                    link_msg = "common_01411".to_string();
                    link_code = 6;
                } else if let Some(uid) = uid {
                    let applied = if job_id > 0 {
                        let (sq, ms) = tokio::join!(
                            phpyun_models::apply::repo::count_active_by_uid_job(
                                state.db.reader(),
                                uid,
                                job_id,
                            ),
                            phpyun_models::apply::repo::count_userid_msg_by_uid_job(
                                state.db.reader(),
                                uid,
                                job_id,
                            ),
                        );
                        sq.unwrap_or(0) > 0 || ms.unwrap_or(0) > 0
                    } else {
                        phpyun_models::apply::repo::count_by_uid_to_company(
                            state.db.reader(),
                            uid,
                            raw.com_uid,
                        )
                        .await
                        .unwrap_or(0)
                            > 0
                    };
                    if applied {
                        if cfg_i32(&cfg, "sy_comprivacy_open", 0) == 1
                            && !cfg_csv_has(&cfg, "sy_privacy_rating", raw.rating)
                        {
                            link_msg = "common_01934".to_string();
                            link_code = 10;
                        }
                    } else {
                        link_msg = "common_01540".to_string();
                        link_code = 8;
                    }
                }
            }
            _ => {}
        }
    } else {
        link_msg = "common_02372".to_string();
        link_code = 9;
    }

    if link_code == 1
        && raw.infostatus == 1
        && not_disturb_blocks(&raw.not_disturb, clock::now_ts())
    {
        link_msg = "common_00973".to_string();
        link_code = 2;
    }

    if link_msg.is_empty() && link_code == 0 {
        link_code = 1;
    }

    let mut prvlinktel = String::new();
    let mut prvtime = String::new();
    if link_code == 10 && isgetprv {
        let real = if !raw.linktel.trim().is_empty() {
            raw.linktel.as_str()
        } else {
            raw.linkphone.as_str()
        };
        let seeker_tel = if usertype == 1 {
            if let Some(u) = user {
                resume_repo::find_by_uid(state.db.reader(), u.uid)
                    .await
                    .ok()
                    .flatten()
                    .and_then(|r| r.telphone)
                    .unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let seeker_uid = uid.unwrap_or(0);
        match crate::privacy_service::bind_middle_number(
            state,
            real,
            &seeker_tel,
            job_id,
            raw.com_uid,
            seeker_uid,
            2,
        )
        .await
        {
            Ok(bind) => {
                prvlinktel = bind.number;
                prvtime = bind.expire_n;
            }
            Err(_) => {
                link_code = 11;
                link_msg = "common_00332".to_string();
            }
        }
    }

    let revealed = link_code == 1;
    // PHP unsets plaintext tel/phone when linkMsg is set; we never return email.
    Ok(PublicJobContact {
        job_id,
        linkman: raw.linkman,
        linktel: if revealed {
            raw.linktel.clone()
        } else {
            String::new()
        },
        linkphone: if revealed {
            raw.linkphone.clone()
        } else {
            String::new()
        },
        linktel_n: mask_contact(&raw.linktel),
        linkphone_n: mask_contact(&raw.linkphone),
        address: raw.address,
        city_id: raw.cityid,
        x: raw.x,
        y: raw.y,
        link_code,
        link_msg,
        link_sub,
        revealed,
        prvlinktel,
        prvtime,
    })
}

/// Other active jobs from the same company.
pub async fn list_same_company(state: &AppState, job_id: u64, limit: u64) -> AppResult<Vec<Job>> {
    let now = clock::now_ts();
    let cur = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(ApiError::business("job_not_found"))?;
    Ok(job_repo::list_same_company(state.db.reader(), cur.uid, job_id, now, limit).await?)
}

/// Similar jobs (same job1 category, different company).
pub async fn list_similar(state: &AppState, job_id: u64, limit: u64) -> AppResult<Vec<Job>> {
    let now = clock::now_ts();
    let cur = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(ApiError::business("job_not_found"))?;
    Ok(job_repo::list_similar(state.db.reader(), cur.job1, job_id, cur.uid, now, limit).await?)
}

/// Public job list for a given company.
pub async fn list_by_company(
    state: &AppState,
    com_uid: u64,
    page: Pagination,
) -> AppResult<JobPage> {
    let now = clock::now_ts();
    let (total, list) = tokio::join!(
        job_repo::count_by_company_public(state.db.reader(), com_uid, now),
        job_repo::list_by_company_public(state.db.reader(), com_uid, now, page.offset, page.limit),
    );
    Ok(JobPage {
        total: total?,
        list: list?,
    })
}

// ==================== Phone-click log (mirrors PHP `addTelLog`) ====================

/// Log a "click on the job contact phone" action.
/// - At least one of jobid / comid must be supplied; when jobid is given the real com_uid is read from the job.
/// - Avoids self-noise: a company clicking its own phone is not recorded.
pub async fn log_tel_click(
    state: &AppState,
    viewer_uid: Option<u64>,
    job_id: u64,
    com_id_hint: u64,
    source: i32,
    client_ip: &str,
) -> AppResult<()> {
    let (final_jobid, final_comid) = if job_id > 0 {
        match job_repo::find_by_id(state.db.reader(), job_id).await? {
            Some(job) => (job.id, job.uid),
            None => (0, com_id_hint),
        }
    } else {
        (0, com_id_hint)
    };

    if final_comid == 0 {
        return Ok(()); // Cannot resolve a company -> drop silently (matches PHP behavior)
    }

    if let Some(uid) = viewer_uid {
        if uid == final_comid {
            return Ok(()); // Company clicking its own phone -> do not record
        }
    }

    let _ = phpyun_models::job_tellog::repo::insert(
        state.db.pool(),
        final_jobid,
        final_comid,
        viewer_uid.unwrap_or(0),
        source,
        client_ip,
        clock::now_ts(),
    )
    .await?;
    Ok(())
}
