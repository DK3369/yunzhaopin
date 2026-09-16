//! Employer job management — publish / update / list/unlist / refresh / delete + my jobs list.
//!
//! Aligns with the PHPYun `mcenter/job` controller. usertype=2 only; service-layer validation.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::admin_gap::extra as gap_extra;
use phpyun_models::category::repo as category_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_address::repo as company_address_repo;
use phpyun_models::company_cert::repo as company_cert_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::{entity::Job, repo as job_repo};
use phpyun_models::part::repo as part_repo;
use phpyun_models::site_setting::repo as setting_repo;

fn store_is_email(v: i32) -> i32 {
    if v == 2 || v == 3 {
        3
    } else {
        1
    }
}

fn store_is_message(v: i32) -> i32 {
    if v == 2 {
        2
    } else {
        1
    }
}

struct ResolvedLink {
    is_link: i32,
    link_id: i32,
    provinceid: i32,
    cityid: i32,
    three_cityid: i32,
    x: String,
    y: String,
}

async fn resolve_job_link(
    state: &AppState,
    uid: u64,
    link_id: i32,
    provinceid: i32,
    cityid: i32,
    three_cityid: i32,
    default_x: &str,
    default_y: &str,
) -> AppResult<ResolvedLink> {
    if link_id > 0 {
        if let Some(addr) =
            company_address_repo::find_by_id(state.db.reader(), link_id as u64, uid)
                .await?
        {
            let stored_id = i32::try_from(addr.id).unwrap_or(link_id);
            return Ok(ResolvedLink {
                is_link: 2,
                link_id: stored_id,
                provinceid: addr.provinceid,
                cityid: addr.cityid,
                three_cityid: addr.three_cityid,
                x: addr.x.unwrap_or_default(),
                y: addr.y.unwrap_or_default(),
            });
        }
    }
    Ok(ResolvedLink {
        is_link: 1,
        link_id: 0,
        provinceid,
        cityid,
        three_cityid,
        x: default_x.to_string(),
        y: default_y.to_string(),
    })
}

fn overlay_hide(mut link: ResolvedLink, is_link: i32) -> ResolvedLink {
    if is_link == 3 {
        link.is_link = 3;
    }
    link
}

async fn maybe_tblink(
    state: &AppState,
    uid: u64,
    is_tblink: i32,
    link: &ResolvedLink,
) -> AppResult<()> {
    if is_tblink != 1 {
        return Ok(());
    }
    let _ = job_repo::sync_contact_by_uid(
        state.db.pool(),
        uid,
        link.link_id,
        link.is_link,
        link.provinceid,
        link.cityid,
        link.three_cityid,
        link.x.as_str(),
        link.y.as_str(),
    )
    .await?;
    Ok(())
}

// ==================== Create ====================

pub struct CreateJobInput<'a> {
    pub name: &'a str,
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    // salary deprecated in PHPYun schema
    pub minsalary: i32,
    pub maxsalary: i32,
    pub job_type: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    pub content: Option<&'a str>,
    pub wel: Option<&'a str>,
    pub sdate: i64,
    pub edate: i64,
    pub hy: i32,
    pub report: i32,
    pub age: i32,
    pub sex: i32,
    pub marriage: i32,
    pub lang: &'a str,
    pub is_graduate: i32,
    pub zp_minage: i32,
    pub zp_maxage: i32,
    pub link_id: i32,
    pub is_link: i32,
    pub is_message: i32,
    pub is_email: i32,
    pub exp_req: &'a str,
    pub edu_req: &'a str,
    pub sex_req: i32,
    pub minage_req: i32,
    pub maxage_req: i32,
    pub salary_type: i32,
    pub is_tblink: i32,
    pub zp_num: i32,
    pub jobclassid: i32,
    pub x: &'a str,
    pub y: &'a str,
    pub custom_link_man: &'a str,
    pub custom_link_moblie: &'a str,
}

async fn setting_on(state: &AppState, key: &str) -> bool {
    match setting_repo::find(state.db.reader(), key).await {
        Ok(Some(row)) => row.value.trim() == "1",
        _ => false,
    }
}

async fn setting_raw(state: &AppState, key: &str) -> String {
    match setting_repo::find(state.db.reader(), key).await {
        Ok(Some(row)) => row.value.trim().to_string(),
        _ => String::new(),
    }
}

async fn setting_i32(state: &AppState, key: &str, default: i32) -> i32 {
    setting_raw(state, key)
        .await
        .parse::<i32>()
        .unwrap_or(default)
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= now
}

struct VipPack {
    addjobnum: i32,
    job_num: i32,
    rating: i32,
    integral: i64,
}

async fn listed_job_count(state: &AppState, uid: u64) -> AppResult<u64> {
    let (jobs, parts, lts) = tokio::join!(
        job_repo::count_listed_by_uid(state.db.reader(), uid),
        part_repo::count_listed_by_uid(state.db.reader(), uid),
        job_repo::count_listed_lt_by_uid(state.db.reader(), uid),
    );
    Ok(jobs? + parts? + lts?)
}

async fn contains_forbidden_keyword(state: &AppState, texts: &[&str]) -> bool {
    let raw = setting_raw(state, "sy_fkeyword").await;
    if raw.is_empty() {
        return false;
    }
    let hay = texts.join(" ").to_lowercase();
    raw.split(|c: char| c == ',' || c == '，' || c == '|' || c == ' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .any(|k| hay.contains(&k.to_lowercase()))
}

async fn vip_pack(state: &AppState, uid: u64) -> AppResult<VipPack> {
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), uid).await?;
    let Some(s) = st else {
        return Ok(VipPack {
            addjobnum: 0,
            job_num: 0,
            rating: 0,
            integral: 0,
        });
    };
    let integral = s.integral.parse::<i64>().unwrap_or(0);
    let vip = is_vip(s.vip_etime, now);
    let listed = listed_job_count(state, uid).await?;
    let addjobnum = if !vip || s.rating_type == 0 {
        0
    } else if listed >= s.job_num.max(0) as u64 {
        2
    } else {
        1
    };
    Ok(VipPack {
        addjobnum,
        job_num: s.job_num,
        rating: s.rating,
        integral,
    })
}

async fn resolve_job_class(
    state: &AppState,
    job1: i32,
    job1_son: i32,
    job_post: i32,
    jobclassid: i32,
) -> AppResult<(i32, i32, i32)> {
    let leaf = if jobclassid > 0 {
        jobclassid
    } else if job_post > 0 {
        job_post
    } else if job1_son > 0 {
        job1_son
    } else {
        job1
    };
    if leaf <= 0 {
        return Err(ApiError::business("member_com_00586"));
    }
    let parent = category_repo::find_job_class_parent(state.db.reader(), leaf)
        .await?
        .unwrap_or(0);
    if parent == 0 {
        return Ok((leaf, 0, 0));
    }
    let grand = category_repo::find_job_class_parent(state.db.reader(), parent)
        .await?
        .unwrap_or(0);
    if grand == 0 {
        Ok((parent, leaf, 0))
    } else {
        Ok((grand, parent, leaf))
    }
}

fn validate_salary(myswitch: bool, salary_type: i32, min: i32, max: i32) -> AppResult<(i32, i32)> {
    if salary_type == 1 {
        if !myswitch {
            return Err(ApiError::business("member_com_00238"));
        }
        return Ok((0, 0));
    }
    if min <= 0 {
        return Err(ApiError::business("member_com_00238"));
    }
    if max > 0 && max < min {
        return Err(ApiError::business("wap_com_00264"));
    }
    if max > 0 && max == min {
        return Err(ApiError::business("wap_com_00255"));
    }
    Ok((min, max))
}

fn validate_age(v: i32) -> AppResult<()> {
    if v != 0 && v < 16 {
        return Err(ApiError::business("wap_com_00257"));
    }
    if v > 99 {
        return Err(ApiError::business("wap_com_00269"));
    }
    Ok(())
}

async fn compute_job_state(state: &AppState, uid: u64, r_status: i32, rating: i32) -> AppResult<i32> {
    if r_status != 1 {
        return Ok(0);
    }
    let cert = company_cert_repo::find(state.db.reader(), uid).await?;
    let cert_ok = cert.as_ref().map(|c| c.status == 1).unwrap_or(false);
    if setting_on(state, "com_free_status").await && cert_ok {
        return Ok(1);
    }
    let ms = setting_raw(state, "job_ms_rating").await;
    if !ms.is_empty() {
        let hit = ms.split(',').any(|s| s.trim().parse::<i32>().ok() == Some(rating));
        if hit {
            return Ok(1);
        }
    }
    Ok(setting_i32(state, "com_job_status", 0).await)
}

pub struct PublishJobResult {
    pub id: u64,
    pub state: i32,
    pub status: i32,
}

pub struct PublishGap {
    pub key: String,
    pub href: String,
}

pub struct PublishCheck {
    pub addjobnum: i32,
    pub job_num: i32,
    pub integral: i64,
    pub job_state: i32,
    pub gaps: Vec<PublishGap>,
}

pub async fn publish_check(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<PublishCheck> {
    user.require_employer()?;
    let company = company_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("member_com_00692"))?;
    let mut gaps = Vec::new();
    let name_ok = company
        .name
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    let tel_ok = company
        .linktel
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
        || company
            .linkphone
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
    if !name_ok || company.provinceid == 0 || !tel_ok {
        gaps.push(PublishGap {
            key: "member_com_00692".into(),
            href: "/com/profile".into(),
        });
    }
    if setting_on(state, "com_enforce_emailcert").await && company.email_status != 1 {
        gaps.push(PublishGap {
            key: "wap_com_00186".into(),
            href: "/com/binding".into(),
        });
    }
    if setting_on(state, "com_enforce_mobilecert").await && company.moblie_status != 1 {
        gaps.push(PublishGap {
            key: "member_com_00071".into(),
            href: "/com/binding".into(),
        });
    }
    if setting_on(state, "com_enforce_licensecert").await && company.yyzz_status != 1 {
        let cert = company_cert_repo::find(state.db.reader(), user.uid).await?;
        let deny = match cert {
            None => true,
            Some(c) if c.status == 2 => true,
            _ => false,
        };
        if deny {
            gaps.push(PublishGap {
                key: "member_com_00187".into(),
                href: "/com/cert".into(),
            });
        }
    }
    if setting_on(state, "com_enforce_setposition").await {
        let x = company.x.as_deref().unwrap_or("").trim();
        let y = company.y.as_deref().unwrap_or("").trim();
        if x.is_empty() || y.is_empty() {
            gaps.push(PublishGap {
                key: "member_com_00694".into(),
                href: "/com/profile".into(),
            });
        }
    }
    if setting_on(state, "com_gzgzh").await {
        gaps.push(PublishGap {
            key: "member_com_00695".into(),
            href: "/com".into(),
        });
    }
    let pack = vip_pack(state, user.uid).await?;
    let job_state = compute_job_state(state, user.uid, company.r_status, company.rating).await?;
    Ok(PublishCheck {
        addjobnum: pack.addjobnum,
        job_num: pack.job_num,
        integral: pack.integral,
        job_state,
        gaps,
    })
}

async fn ensure_can_publish(state: &AppState, user: &AuthenticatedUser) -> AppResult<()> {
    if setting_raw(state, "sy_job_web").await == "2" {
        return Err(ApiError::business("member_com_00696"));
    }
    let check = publish_check(state, user).await?;
    if let Some(g) = check.gaps.first() {
        if g.key != "member_com_00695" {
            return Err(ApiError::business(g.key.clone()));
        }
    }
    if check.addjobnum == 0 {
        return Err(ApiError::business("member_com_00696"));
    }
    Ok(())
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateJobInput<'_>,
    _com_name: Option<&str>,
    client_ip: &str,
) -> AppResult<PublishJobResult> {
    save_job(state, user, None, input, client_ip).await
}

async fn save_job(
    state: &AppState,
    user: &AuthenticatedUser,
    edit_id: Option<u64>,
    input: CreateJobInput<'_>,
    client_ip: &str,
) -> AppResult<PublishJobResult> {
    user.require_employer()?;
    if edit_id.is_none() {
        ensure_can_publish(state, user).await?;
    }
    let name = input.name.trim();
    if name.len() < 2 {
        return Err(ApiError::business("member_com_00585"));
    }
    let desc = input.content.unwrap_or("");
    if strip_tags(desc).trim().is_empty() {
        return Err(ApiError::business("member_com_00587"));
    }
    let zp_num = if input.zp_num > 0 {
        input.zp_num
    } else {
        input.number
    };
    if zp_num <= 0 {
        return Err(ApiError::business("wap_00888"));
    }
    validate_age(input.zp_minage)?;
    validate_age(input.zp_maxage)?;
    validate_age(input.minage_req)?;
    validate_age(input.maxage_req)?;
    let myswitch = setting_on(state, "com_job_myswitch").await;
    let (minsalary, maxsalary) =
        validate_salary(myswitch, input.salary_type, input.minsalary, input.maxsalary)?;
    let (job1, job1_son, job_post) = resolve_job_class(
        state,
        input.job1,
        input.job1_son,
        input.job_post,
        input.jobclassid,
    )
    .await?;

    let company = company_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("common_06272"))?;
    let com_name = company.name.clone().unwrap_or_default();
    if com_name.trim().is_empty() {
        return Err(ApiError::business("common_06272"));
    }
    let com_logo = company.logo.clone().unwrap_or_default();
    let com_x = company.x.clone().unwrap_or_default();
    let com_y = company.y.clone().unwrap_or_default();
    let pack = vip_pack(state, user.uid).await?;
    if edit_id.is_none() {
        if pack.job_num == 0 {
            return Err(ApiError::business("api_wxapp_00002"));
        }
        if pack.addjobnum == 0 {
            return Err(ApiError::business("wap_01287"));
        }
    }
    if let Some(dup) = job_repo::find_id_by_uid_name_listed(state.db.reader(), user.uid, name).await?
    {
        if edit_id != Some(dup) {
            return Err(ApiError::business("common_00293"));
        }
    }

    let use_form_geo = input.provinceid > 0 && input.link_id <= 0;
    let default_x = if use_form_geo && !input.x.is_empty() {
        input.x
    } else {
        com_x.as_str()
    };
    let default_y = if use_form_geo && !input.y.is_empty() {
        input.y
    } else {
        com_y.as_str()
    };
    let geo_province = if use_form_geo {
        input.provinceid
    } else {
        company.provinceid
    };
    let geo_city = if use_form_geo {
        input.cityid
    } else {
        company.cityid
    };
    let geo_three = if use_form_geo {
        input.three_cityid
    } else {
        company.three_cityid
    };
    let mut link = overlay_hide(
        resolve_job_link(
            state,
            user.uid,
            input.link_id,
            geo_province,
            geo_city,
            geo_three,
            default_x,
            default_y,
        )
        .await?,
        input.is_link,
    );
    if input.is_link == 2 && input.link_id <= 0 {
        let man = input.custom_link_man.trim();
        let mob = input.custom_link_moblie.trim();
        if man.is_empty() || mob.is_empty() {
            return Err(ApiError::business("common_01306"));
        }
        let new_id = company_address_repo::create(
            state.db.pool(),
            user.uid,
            &company_address_repo::AddressFields {
                link_man: man,
                link_moblie: mob,
                link_phone: "",
                email: "",
                link_address: "",
                provinceid: geo_province,
                cityid: geo_city,
                three_cityid: geo_three,
                x: default_x,
                y: default_y,
            },
        )
        .await?;
        link.is_link = 2;
        link.link_id = i32::try_from(new_id).unwrap_or(0);
        link.provinceid = geo_province;
        link.cityid = geo_city;
        link.three_cityid = geo_three;
        link.x = default_x.to_string();
        link.y = default_y.to_string();
    }
    if input.is_link > 1 && input.link_id == 0 && link.link_id == 0 {
        return Err(ApiError::business("common_01306"));
    }
    if input.link_id > 0 && link.is_link == 1 {
        link.is_link = 2;
    }

    let hy = if input.hy != 0 { input.hy } else { company.hy };
    let mut job_state = compute_job_state(state, user.uid, company.r_status, company.rating).await?;
    if contains_forbidden_keyword(
        state,
        &[name, desc, input.wel.unwrap_or("")],
    )
    .await
    {
        job_state = 0;
    }
    let listed = listed_job_count(state, user.uid).await?;
    let mut status = 0;
    if edit_id.is_none() && listed >= pack.job_num.max(0) as u64 {
        status = 1;
    }
    let now = clock::now_ts();
    let lock_name = setting_on(state, "joblock").await && edit_id.is_some();
    let stored_name = if lock_name { None } else { Some(name) };

    let id = if let Some(jid) = edit_id {
        let existing = job_repo::find_by_id(state.db.reader(), jid)
            .await?
            .filter(|j| j.uid == user.uid)
            .ok_or_else(|| ApiError::business("job_not_found"))?;
        let name_for_update = if lock_name { None } else { stored_name };
        let affected = job_repo::update(
            state.db.pool(),
            jid,
            user.uid,
            job_repo::JobUpdate {
                name: name_for_update,
                job1: Some(job1),
                job1_son: Some(job1_son),
                job_post: Some(job_post),
                provinceid: Some(link.provinceid),
                cityid: Some(link.cityid),
                three_cityid: Some(link.three_cityid),
                minsalary: Some(minsalary),
                maxsalary: Some(maxsalary),
                job_type: Some(input.job_type),
                number: Some(input.number),
                exp: Some(input.exp),
                edu: Some(input.edu),
                description: Some(desc),
                welfare: input.wel,
                sdate: None,
                edate: if input.edate > 0 { Some(input.edate) } else { None },
                hy: Some(hy),
                report: Some(input.report),
                age: Some(input.age),
                sex: Some(input.sex),
                marriage: Some(input.marriage),
                lang: Some(input.lang),
                is_graduate: Some(input.is_graduate),
                zp_minage: Some(input.zp_minage),
                zp_maxage: Some(input.zp_maxage),
                is_link: Some(link.is_link),
                link_id: Some(link.link_id),
                is_message: Some(store_is_message(input.is_message)),
                is_email: Some(store_is_email(input.is_email)),
                exp_req: Some(input.exp_req),
                edu_req: Some(input.edu_req),
                sex_req: Some(input.sex_req),
                minage_req: Some(input.minage_req),
                maxage_req: Some(input.maxage_req),
                zp_num: Some(zp_num),
                x: Some(link.x.as_str()),
                y: Some(link.y.as_str()),
                state: Some(job_state),
                r_status: Some(company.r_status),
                com_name: Some(com_name.as_str()),
                com_logo: Some(com_logo.as_str()),
                com_provinceid: Some(company.provinceid),
                pr: Some(company.pr),
                mun: Some(company.mun),
                yyzz_status: Some(company.yyzz_status),
                rating: Some(pack.rating),
            },
            now,
        )
        .await?;
        if affected == 0 {
            return Err(ApiError::business("job_not_found"));
        }
        let fav_name = name_for_update.unwrap_or(existing.name.as_str());
        let _ = job_repo::update_fav_job_name(state.db.pool(), jid, fav_name).await;
        let _ = job_repo::touch_hotjob(state.db.pool(), user.uid, now).await;
        jid
    } else {
        job_repo::create(
            state.db.pool(),
            job_repo::JobCreate {
                uid: user.uid,
                com_name: Some(com_name.as_str()),
                name,
                job1,
                job1_son,
                job_post,
                provinceid: link.provinceid,
                cityid: link.cityid,
                three_cityid: link.three_cityid,
                minsalary,
                maxsalary,
                job_type: input.job_type,
                number: input.number,
                exp: input.exp,
                edu: input.edu,
                description: Some(desc),
                welfare: input.wel,
                sdate: now,
                edate: input.edate,
                did: user.did,
                x: link.x.as_str(),
                y: link.y.as_str(),
                hy,
                report: input.report,
                age: input.age,
                sex: input.sex,
                marriage: input.marriage,
                lang: input.lang,
                is_graduate: input.is_graduate,
                zp_minage: input.zp_minage,
                zp_maxage: input.zp_maxage,
                is_link: link.is_link,
                link_id: link.link_id,
                is_message: store_is_message(input.is_message),
                is_email: store_is_email(input.is_email),
                exp_req: input.exp_req,
                edu_req: input.edu_req,
                sex_req: input.sex_req,
                minage_req: input.minage_req,
                maxage_req: input.maxage_req,
                zp_num,
                state: job_state,
                status,
                r_status: company.r_status,
                com_logo: com_logo.as_str(),
                com_provinceid: company.provinceid,
                pr: company.pr,
                mun: company.mun,
                yyzz_status: company.yyzz_status,
                rating: pack.rating,
            },
            now,
        )
        .await?
    };
    if edit_id.is_none() && status == 0 {
        let _ = gap_extra::insert_company_statis_detail(
            state.db.pool(),
            user.uid,
            1,
            1,
            "common_00567",
            "/v1/mcenter/jobs",
            client_ip,
            now,
        )
        .await;
    }
    maybe_tblink(state, user.uid, input.is_tblink, &link).await?;
    let _ = company_repo::touch_jobtime(state.db.pool(), user.uid, now).await;
    let ev = if edit_id.is_some() {
        "job.update"
    } else {
        "job.create"
    };
    let _ = audit::emit(
        state,
        AuditEvent::new(ev, Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}"))
            .meta(&serde_json::json!({ "name": name, "state": job_state, "status": status })),
    )
    .await;
    let stored = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    Ok(PublishJobResult {
        id,
        state: stored.state,
        status: stored.status,
    })
}

// ==================== Update ====================

pub struct UpdateJobInput<'a> {
    pub name: Option<&'a str>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    // salary deprecated
    pub minsalary: Option<i32>,
    pub maxsalary: Option<i32>,
    pub job_type: Option<i32>,
    pub number: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub content: Option<&'a str>,
    pub wel: Option<&'a str>,
    pub sdate: Option<i64>,
    pub edate: Option<i64>,
    pub hy: Option<i32>,
    pub report: Option<i32>,
    pub age: Option<i32>,
    pub sex: Option<i32>,
    pub marriage: Option<i32>,
    pub lang: Option<&'a str>,
    pub is_graduate: Option<i32>,
    pub zp_minage: Option<i32>,
    pub zp_maxage: Option<i32>,
    pub is_link: Option<i32>,
    pub link_id: Option<i32>,
    pub is_message: Option<i32>,
    pub is_email: Option<i32>,
    pub exp_req: Option<&'a str>,
    pub edu_req: Option<&'a str>,
    pub sex_req: Option<i32>,
    pub minage_req: Option<i32>,
    pub maxage_req: Option<i32>,
    pub salary_type: Option<i32>,
    pub is_tblink: Option<i32>,
    pub zp_num: Option<i32>,
    pub jobclassid: Option<i32>,
    pub x: Option<&'a str>,
    pub y: Option<&'a str>,
    pub custom_link_man: Option<&'a str>,
    pub custom_link_moblie: Option<&'a str>,
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: UpdateJobInput<'_>,
    client_ip: &str,
) -> AppResult<PublishJobResult> {
    let existing = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let name = input.name.unwrap_or(existing.name.as_str());
    let lang_owned = input
        .lang
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.lang.clone().unwrap_or_default());
    let desc_owned = input
        .content
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.description.clone().unwrap_or_default());
    let wel_owned = input.wel.map(|s| s.to_string()).or(existing.welfare.clone());
    let exp_req = input
        .exp_req
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.exp_req.clone());
    let edu_req = input
        .edu_req
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.edu_req.clone());
    let x_owned = input
        .x
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.x.clone().unwrap_or_default());
    let y_owned = input
        .y
        .map(|s| s.to_string())
        .unwrap_or_else(|| existing.y.clone().unwrap_or_default());
    let create = CreateJobInput {
        name,
        job1: input.job1.unwrap_or(existing.job1),
        job1_son: input.job1_son.unwrap_or(existing.job1_son),
        job_post: input.job_post.unwrap_or(existing.job_post),
        provinceid: input.provinceid.unwrap_or(existing.provinceid),
        cityid: input.cityid.unwrap_or(existing.cityid),
        three_cityid: input.three_cityid.unwrap_or(existing.three_cityid),
        minsalary: input.minsalary.unwrap_or(existing.minsalary),
        maxsalary: input.maxsalary.unwrap_or(existing.maxsalary),
        job_type: input.job_type.unwrap_or(existing.r#type),
        number: input.number.unwrap_or(existing.number),
        exp: input.exp.unwrap_or(existing.exp),
        edu: input.edu.unwrap_or(existing.edu),
        content: Some(desc_owned.as_str()),
        wel: wel_owned.as_deref(),
        sdate: input.sdate.unwrap_or(existing.sdate),
        edate: input.edate.unwrap_or(existing.edate),
        hy: input.hy.unwrap_or(existing.hy),
        report: input.report.unwrap_or(existing.report),
        age: input.age.unwrap_or(existing.age),
        sex: input.sex.unwrap_or(existing.sex),
        marriage: input.marriage.unwrap_or(existing.marriage),
        lang: lang_owned.as_str(),
        is_graduate: input.is_graduate.unwrap_or(existing.is_graduate),
        zp_minage: input.zp_minage.unwrap_or(existing.zp_minage),
        zp_maxage: input.zp_maxage.unwrap_or(existing.zp_maxage),
        link_id: input.link_id.unwrap_or(existing.link_id),
        is_link: input.is_link.unwrap_or(existing.is_link),
        is_message: input.is_message.unwrap_or(existing.is_message),
        is_email: input.is_email.unwrap_or(existing.is_email),
        exp_req: exp_req.as_str(),
        edu_req: edu_req.as_str(),
        sex_req: input.sex_req.unwrap_or(existing.sex_req),
        minage_req: input.minage_req.unwrap_or(existing.minage_req),
        maxage_req: input.maxage_req.unwrap_or(existing.maxage_req),
        salary_type: input.salary_type.unwrap_or(0),
        is_tblink: input.is_tblink.unwrap_or(0),
        zp_num: input.zp_num.unwrap_or(existing.zp_num),
        jobclassid: input.jobclassid.unwrap_or(0),
        x: x_owned.as_str(),
        y: y_owned.as_str(),
        custom_link_man: input.custom_link_man.unwrap_or(""),
        custom_link_moblie: input.custom_link_moblie.unwrap_or(""),
    };
    save_job(state, user, Some(id), create, client_ip).await
}

// ==================== List/unlist ====================

pub async fn set_status(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    // PHP `status`: 0 recruiting / 1 unlisted. Map legacy client `2` to unlisted.
    let status = match status {
        0 => 0,
        1 | 2 => 1,
        _ => return Err(ApiError::business("job_not_found")),
    };
    let job = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    if status == 0 {
        if job.state != 1 {
            return Err(ApiError::business("job_pending"));
        }
        let pack = vip_pack(state, user.uid).await?;
        if pack.addjobnum == 0 {
            return Err(ApiError::business("model_00056"));
        }
        let listed = listed_job_count(state, user.uid).await?;
        let extra = if job.status == 0 { 0 } else { 1 };
        if listed + extra > pack.job_num.max(0) as u64 {
            return Err(ApiError::business("model_00056"));
        }
    }
    let affected = job_repo::set_status(state.db.pool(), id, user.uid, status).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let label = if status == 0 { "online" } else { "offline" };
    let _ = audit::emit(
        state,
        AuditEvent::new("job.status_change", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}"))
            .meta(&serde_json::json!({ "status": label })),
    )
    .await;
    Ok(())
}

async fn consume_refresh_quota(
    state: &AppState,
    uid: u64,
    n: i32,
    job_ids: &[u64],
) -> AppResult<bool> {
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("zph_need_vip"))?;
    if !is_vip(st.vip_etime, now) {
        return Err(ApiError::business("zph_need_vip"));
    }
    if st.rating_type == 2 {
        return Ok(true);
    }
    if st.rating_type != 1 {
        return Ok(false);
    }
    let budget = phpyun_models::admin_gap::extra::reserve_refresh_budget(state.db.reader(), uid)
        .await?;
    if budget < i64::from(n.max(0)) {
        return Ok(false);
    }
    let free_left = (budget - i64::from(st.breakjob_num.max(0))).max(0);
    let mut remain_free = free_left;
    let mut paid = 0i32;
    for _ in 0..n.max(0) {
        if remain_free > 0 {
            remain_free -= 1;
        } else {
            paid += 1;
        }
    }
    if paid > 0 && !statis_repo::try_consume_breakjob(state.db.pool(), uid, paid).await? {
        return Ok(false);
    }
    let mut remain_free = free_left;
    for id in job_ids.iter().take(n.max(0) as usize) {
        let free = if remain_free > 0 {
            remain_free -= 1;
            1
        } else {
            2
        };
        let _ = job_repo::insert_refresh_log(state.db.pool(), uid, *id, now, free, i32::from(free == 1))
            .await;
    }
    Ok(true)
}

#[derive(Debug, Clone)]
pub struct RefreshResult {
    pub status: i32,
    pub integral: i64,
    pub price: f64,
}

async fn refresh_pay_quote(state: &AppState, n: i32) -> AppResult<(i64, f64)> {
    let unit = setting_raw(state, "integral_jobefresh")
        .await
        .parse::<f64>()
        .unwrap_or(0.0);
    let pro = setting_raw(state, "integral_proportion")
        .await
        .parse::<f64>()
        .unwrap_or(1.0);
    let price = unit * f64::from(n.max(0));
    let integral = (price * pro).round() as i64;
    Ok((integral, price))
}

// ==================== Refresh ====================

pub async fn refresh(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    confirm: bool,
    client_ip: &str,
) -> AppResult<RefreshResult> {
    user.require_employer()?;
    let _owned = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let ok = consume_refresh_quota(state, user.uid, 1, &[id]).await?;
    if !ok {
        let (integral, price) = refresh_pay_quote(state, 1).await?;
        if price <= 0.0 {
            // PHP: integral_jobefresh==0 且开单项购买时直接刷新
        } else if !confirm {
            return Ok(RefreshResult {
                status: 2,
                integral,
                price,
            });
        } else if integral > 0
            && statis_repo::try_deduct_integral(state.db.pool(), user.uid, integral).await? == 0
        {
            return Err(ApiError::business("integral_insufficient"));
        }
    }
    let affected = job_repo::refresh(state.db.pool(), id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.refresh", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}")),
    )
    .await;
    Ok(RefreshResult {
        status: 1,
        integral: 0,
        price: 0.0,
    })
}

// ==================== Delete ====================

pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = job_repo::delete(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.delete", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}")),
    )
    .await;
    Ok(())
}

// ==================== Batch operations ====================

pub struct BatchReport {
    pub requested: usize,
    pub affected: u64,
}

/// Batch refresh: bump `lastupdate` for several jobs owned by the caller.
pub async fn batch_refresh(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    let ok = consume_refresh_quota(
        state,
        user.uid,
        i32::try_from(ids.len()).unwrap_or(i32::MAX),
        ids,
    )
    .await?;
    if !ok {
        return Err(ApiError::business("job_refresh_quota"));
    }
    let now = clock::now_ts();
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::refresh(state.db.pool(), *id, user.uid, now).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_refresh", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

/// Batch unlist.
pub async fn batch_close(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::set_status(state.db.pool(), *id, user.uid, 1).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_close", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

/// Batch delete (hard delete; only the caller's own rows).
pub async fn batch_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::delete(state.db.pool(), *id, user.uid).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_delete", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

// ==================== List ====================

pub struct MyJobsPage {
    pub list: Vec<Job>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    w: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<MyJobsPage> {
    user.require_employer()?;
    let (total_res, list_res) = tokio::join!(
        job_repo::count_own_w(state.db.reader(), user.uid, w, keyword),
        job_repo::list_own_w(
            state.db.reader(),
            user.uid,
            w,
            keyword,
            page.offset,
            page.limit
        ),
    );
    Ok(MyJobsPage {
        total: total_res?,
        list: list_res?,
    })
}

/// PHP `job.class.php` tab counters: w0 待审 / w1 招聘中 / w3 未过 / w4 下架 / w5 全部.
pub struct JobStateCounts {
    pub w0: u64,
    pub w1: u64,
    pub w3: u64,
    pub w4: u64,
    pub w5: u64,
    pub online: u64,
    pub pending: u64,
    pub closed: u64,
    pub breakjob_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

pub async fn counts_by_state(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<JobStateCounts> {
    user.require_employer()?;
    let db = state.db.reader();
    let (tabs, st) = tokio::join!(
        job_repo::count_member_tabs(db, user.uid),
        statis_repo::find_admin(db, user.uid),
    );
    let tabs = tabs?;
    let st = st?;
    let w0 = tabs.w0.max(0) as u64;
    let w1 = tabs.w1.max(0) as u64;
    let w3 = tabs.w3.max(0) as u64;
    let w4 = tabs.w4.max(0) as u64;
    let w5 = tabs.w5.max(0) as u64;
    Ok(JobStateCounts {
        w0,
        w1,
        w3,
        w4,
        w5,
        online: w1,
        pending: w0,
        closed: w4,
        breakjob_num: st.as_ref().map(|s| s.breakjob_num).unwrap_or(0),
        top_num: st.as_ref().map(|s| s.top_num).unwrap_or(0),
        rec_num: st.as_ref().map(|s| s.rec_num).unwrap_or(0),
        urgent_num: st.as_ref().map(|s| s.urgent_num).unwrap_or(0),
    })
}

fn promote_kind(kind: &str) -> AppResult<&'static str> {
    match kind {
        "top" => Ok("top"),
        "rec" => Ok("rec"),
        "urgent" => Ok("urgent"),
        _ => Err(ApiError::param_invalid("kind")),
    }
}

fn insufficient_key(kind: &str) -> &'static str {
    match kind {
        "top" => "common_00207",
        "rec" => "common_00206",
        _ => "common_00180",
    }
}

fn job_expire_at(job: &Job, kind: &str) -> i64 {
    match kind {
        "top" => job.xsdate,
        "rec" => job.rec_time,
        "urgent" => job.urgent_time,
        _ => 0,
    }
}

fn job_promote_active(job: &Job, kind: &str, now: i64) -> bool {
    match kind {
        "top" => job.xsdate > now,
        "rec" => job.rec == 1 && job.rec_time > now,
        "urgent" => job.urgent == 1 && job.urgent_time > now,
        _ => false,
    }
}

fn remain_for(st: &phpyun_models::company_statis::repo::AdminStatisRow, kind: &str) -> i32 {
    match kind {
        "top" => st.top_num,
        "rec" => st.rec_num,
        "urgent" => st.urgent_num,
        _ => 0,
    }
}

/// PHP `closeJobPromote` leftover days when `tg_back=1`.
fn leftover_promote_days(expiry: i64, now: i64) -> i32 {
    if expiry <= now {
        return 0;
    }
    const OFFSET: i64 = 8 * 3600;
    let local = now + OFFSET;
    let today_start = (local - local.rem_euclid(86_400)) - OFFSET;
    let numer = expiry - today_start - 86_400;
    if numer <= 0 {
        return 0;
    }
    let end_day = (numer + 86_400 - 1) / 86_400;
    i32::try_from((end_day - 1).max(0)).unwrap_or(0)
}

pub struct PromoteQuote {
    pub kind: String,
    pub remain: i32,
    pub expire_at: i64,
    pub active: bool,
}

pub async fn quote_promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
) -> AppResult<PromoteQuote> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), user.uid).await?;
    Ok(PromoteQuote {
        kind: kind.to_string(),
        remain: st.as_ref().map(|s| remain_for(s, kind)).unwrap_or(0),
        expire_at: job_expire_at(&job, kind),
        active: job_promote_active(&job, kind, now),
    })
}

pub async fn promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
    days: i32,
    client_ip: &str,
) -> AppResult<PromoteQuote> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    if !(1..=365).contains(&days) {
        return Err(ApiError::param_invalid("days"));
    }
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let st = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business(insufficient_key(kind)))?;
    if remain_for(&st, kind) < days {
        return Err(ApiError::business(insufficient_key(kind)));
    }
    if !statis_repo::try_consume_promote(state.db.pool(), user.uid, kind, days).await? {
        return Err(ApiError::business(insufficient_key(kind)));
    }
    let now = clock::now_ts();
    let affected =
        job_repo::apply_member_promote(state.db.pool(), job.id, user.uid, kind, days, now).await?;
    if affected == 0 {
        let _ = statis_repo::add_promote_num(state.db.pool(), user.uid, kind, days).await;
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.promote", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "kind": kind, "days": days })),
    )
    .await;
    quote_promote(state, user, job_id, kind).await
}

pub struct PromoteCloseResult {
    pub refunded: i32,
}

pub async fn close_promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
    client_ip: &str,
) -> AppResult<PromoteCloseResult> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let now = clock::now_ts();
    let refund = if setting_on(state, "tg_back").await {
        leftover_promote_days(job_expire_at(&job, kind), now)
    } else {
        0
    };
    let affected = job_repo::close_member_promote(state.db.pool(), job.id, user.uid, kind).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    if refund > 0 {
        let _ = statis_repo::add_promote_num(state.db.pool(), user.uid, kind, refund).await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.promote_close", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "kind": kind, "refunded": refund })),
    )
    .await;
    Ok(PromoteCloseResult { refunded: refund })
}

/// PHP `job.model::reserveUpJob` / admin `company_job::upReserveJob_action`.
/// `uid` is the employer that owns the jobs (mcenter uses the session uid).
pub async fn up_reserve(
    state: &AppState,
    uid: u64,
    job_ids: &[u64],
    status: i32,
    end_time_raw: &str,
    interval: i32,
    s_time: &str,
    e_time: &str,
) -> AppResult<()> {
    use phpyun_models::admin_gap::extra as gap_extra;

    if uid == 0 || job_ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let opening = status == 1;
    let db = state.db.pool();

    if status != 2 {
        let price = setting_repo::find(db, "sy_reserve_refresh_price")
            .await?
            .and_then(|s| s.value.trim().parse::<i64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(1);
        if gap_extra::reserve_refresh_budget(db, uid).await? / price == 0 {
            return Err(ApiError::business("common_00982"));
        }
    }
    if !setting_on(state, "com_job_reserve").await {
        return Err(ApiError::business("common_01177"));
    }

    let eligible = gap_extra::eligible_reserve_job_ids(db, uid, job_ids).await?;
    if eligible.is_empty() {
        return Err(ApiError::business("model_00008"));
    }

    let end_time = parse_reserve_end_ts(end_time_raw);
    if opening && end_time > 0 && end_time < clock::start_of_today() + 86_400 {
        return Err(ApiError::business("wap_com_00212"));
    }
    let floor = setting_repo::find(db, "sy_reserve_refresh_interval")
        .await?
        .and_then(|s| s.value.trim().parse::<i32>().ok())
        .unwrap_or(0);
    if opening && interval < floor {
        return Err(ApiError::business("common_00606"));
    }
    if reserve_window_invalid(s_time, e_time) {
        return Err(ApiError::business("common_00227"));
    }

    let now = clock::now_ts();
    let v = gap_extra::ReserveScheduleIn {
        status,
        interval,
        start_time: now,
        end_time,
        next_time: now + i64::from(interval.max(0)) * 60,
        s_time,
        e_time,
    };
    let existing = gap_extra::existing_reserve_job_ids(db, uid, &eligible).await?;
    let fresh: Vec<u64> = eligible
        .iter()
        .copied()
        .filter(|id| !existing.contains(id))
        .collect();
    gap_extra::insert_reserve_schedules(db, uid, &fresh, &v).await?;
    gap_extra::update_reserve_schedules(db, uid, &existing, &v).await?;
    gap_extra::set_jobs_is_reserve(db, uid, &eligible, i32::from(opening)).await?;
    Ok(())
}

/// PHP member `job::reserveInfo` — pre-fill the auto-refresh dialog.
pub struct ReserveInfo {
    pub status: i32,
    pub interval: i32,
    pub s_time: String,
    pub e_time: String,
    pub end_time: i64,
}

pub async fn get_reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
) -> AppResult<ReserveInfo> {
    user.require_employer()?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    if job.uid != user.uid {
        return Err(ApiError::business("job_not_found"));
    }
    let row = phpyun_models::admin_gap::extra::find_reserve_schedule(state.db.reader(), job_id)
        .await?;
    Ok(match row {
        Some(r) => ReserveInfo {
            status: r.status,
            interval: r.interval,
            s_time: r.s_time,
            e_time: r.e_time,
            end_time: r.end_time,
        },
        None => ReserveInfo {
            status: 2,
            interval: 0,
            s_time: String::new(),
            e_time: String::new(),
            end_time: 0,
        },
    })
}

fn parse_reserve_end_ts(s: &str) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    if let Ok(n) = s.parse::<i64>() {
        return n;
    }
    clock::parse_site_date(s).unwrap_or(0)
}

fn reserve_window_invalid(s_time: &str, e_time: &str) -> bool {
    if s_time.is_empty() || e_time.is_empty() {
        return false;
    }
    let parse = |v: &str| {
        let mut it = v.split(':');
        let h: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
        let m: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
        h * 60 + m
    };
    parse(s_time) >= parse(e_time)
}

#[cfg(test)]
mod tests {
    use super::{strip_tags, validate_age, validate_salary};

    #[test]
    fn salary_requires_min_when_not_negotiable() {
        let err = validate_salary(false, 0, 0, 0).unwrap_err();
        assert_eq!(err.key(), "member_com_00238");
    }

    #[test]
    fn salary_negotiable_needs_switch() {
        assert!(validate_salary(false, 1, 0, 0).is_err());
        assert_eq!(validate_salary(true, 1, 8, 9).unwrap(), (0, 0));
    }

    #[test]
    fn salary_max_must_exceed_min() {
        assert!(validate_salary(true, 0, 3000, 3000).is_err());
        assert!(validate_salary(true, 0, 3000, 2000).is_err());
        assert_eq!(validate_salary(true, 0, 3000, 5000).unwrap(), (3000, 5000));
        assert_eq!(validate_salary(true, 0, 3000, 0).unwrap(), (3000, 0));
    }

    #[test]
    fn age_floor_is_sixteen() {
        assert!(validate_age(0).is_ok());
        assert!(validate_age(16).is_ok());
        assert!(validate_age(15).is_err());
        assert!(validate_age(100).is_err());
    }

    #[test]
    fn strip_tags_drops_html() {
        assert_eq!(strip_tags("<p>hello <b>x</b></p>").trim(), "hello x");
        assert!(strip_tags("<p></p>").trim().is_empty());
    }
}
