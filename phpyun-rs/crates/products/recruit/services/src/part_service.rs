//! Part-time-job service.
//!
//! Aligns with PHPYun `app/model/part.model.php` + `app/controller/wap/part.class.php`.
//!
//! Three tables involved:
//! - `phpyun_partjob`      — part-time jobs
//! - `phpyun_part_apply`   — applications
//! - `phpyun_part_collect` — favourites/collections
//!
//! Business rules mirror PHP exactly:
//! - Public list: state=1 & status=0 & r_status=1 & not expired
//! - Apply preconditions: job online / not expired / company in good standing
//! - Deduplication: a given (uid, jobid) can only apply / favourite once
//! - Roles: only jobseekers (usertype=1) may apply or favourite

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::part::entity::{PartApply, PartCollect, PartJob};
use phpyun_models::part::repo as part_repo;

// ==================== Public browsing ====================

#[derive(Debug, Clone, Default)]
pub struct PartSearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub part_type: Option<i32>,
    pub salary_type: Option<i32>,
    pub billing_cycle: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    /// `rec=true` keeps only sticky/promoted listings (mirrors PHP partlist
    /// `rec_time > now`).
    pub rec: bool,
    pub did: u32,
}

pub struct PartPage<T> {
    pub list: Vec<T>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &PartSearch,
    page: Pagination,
) -> AppResult<PartPage<PartJob>> {
    let now = clock::now_ts();
    let filter = part_repo::PartFilter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        part_type: search.part_type,
        salary_type: search.salary_type,
        billing_cycle: search.billing_cycle,
        min_salary: search.min_salary,
        max_salary: search.max_salary,
        rec: search.rec,
        did: search.did,
        uptime: crate::site_gate_service::default_uptime_days(
            state,
            None,
            "sy_datacycle_job",
        )
        .await,
    };
    let (total, list) = tokio::join!(
        part_repo::count_public(state.db.reader(), &filter, now),
        part_repo::list_public(state.db.reader(), &filter, page.offset, page.limit, now),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

/// Detail — equivalent to PHPYun `wap/part::show_action`: returns the job and bumps hits.
pub async fn get_public(state: &AppState, id: u64) -> AppResult<PartJob> {
    let job = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(ApiError::business("part_not_found"))?;

    // Status checks aligned with PHP
    if job.status == 1 {
        return Err(ApiError::business("part_offline"));
    }
    if job.state != 1 || job.r_status != 1 {
        return Err(ApiError::business("part_pending"));
    }
    let now = clock::now_ts();
    if job.edate > 0 && job.edate <= now {
        return Err(ApiError::business("part_expired"));
    }

    // Async hit increment (failures are ignored, matches PHP's upInfo)
    let pool = state.db.pool().clone();
    let job_id = id;
    tokio::spawn(async move {
        let _ = part_repo::incr_hits(&pool, job_id).await;
    });

    Ok(job)
}

/// PHP `part.model.php::getLink()` — never return plaintext `linktel` to strangers.
pub struct PartLink {
    pub linktel: Option<String>,
    pub linktel_n: String,
    pub link_tip: i32,
}

fn csv_has_rating(raw: &str, rating: i32) -> bool {
    raw.split(',')
        .any(|p| p.trim().parse::<i32>().ok() == Some(rating))
}

pub async fn resolve_part_link(
    state: &AppState,
    job: &PartJob,
    user: Option<&AuthenticatedUser>,
    rating: i32,
    infostatus: i32,
) -> PartLink {
    let tel = job.linktel.clone().unwrap_or_default();
    let masked = phpyun_core::utils::mask_tel(&tel);
    let uid = user.map(|u| u.uid);
    let usertype = user.map(|u| i32::from(u.usertype)).unwrap_or(0);

    if uid == Some(job.uid) {
        return PartLink {
            linktel: if tel.is_empty() { None } else { Some(tel) },
            linktel_n: job.linktel.clone().unwrap_or_default(),
            link_tip: 0,
        };
    }

    let cfg = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &["com_login_link", "com_link_no"],
    )
    .await
    .unwrap_or_default();
    let login_link = cfg
        .get("com_login_link")
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(1);
    let link_no = cfg.get("com_link_no").map(|s| s.as_str()).unwrap_or("");

    let mut show = false;
    let mut tip = 0i32;
    if csv_has_rating(link_no, rating) {
        tip = 1;
    } else if login_link == 1 {
        if infostatus == 1 {
            show = true;
        } else {
            tip = 2;
        }
    } else if login_link == 2 {
        tip = 3;
    } else if login_link == 3 {
        if usertype == 1 {
            show = true;
        } else {
            tip = 7;
        }
    } else if login_link == 4 {
        if usertype == 1 {
            if let Some(uid) = uid {
                match phpyun_models::resume::expect::find_default_state_by_uid(
                    state.db.reader(),
                    uid,
                )
                .await
                {
                    Ok(Some((state_n, status_n))) if state_n == 1 && status_n == 1 => {
                        show = true;
                    }
                    Ok(Some(_)) => tip = 4,
                    _ => tip = 5,
                }
            } else {
                tip = 7;
            }
        } else {
            tip = 7;
        }
    } else if login_link == 5 {
        if usertype == 1 {
            if let Some(uid) = uid {
                let applied = part_repo::find_apply(state.db.reader(), uid, job.id)
                    .await
                    .ok()
                    .flatten()
                    .is_some();
                if applied {
                    show = true;
                } else {
                    tip = 6;
                }
            } else {
                tip = 7;
            }
        } else {
            tip = 7;
        }
    }

    if show {
        PartLink {
            linktel: if tel.is_empty() { None } else { Some(tel.clone()) },
            linktel_n: tel,
            link_tip: 0,
        }
    } else {
        PartLink {
            linktel: None,
            linktel_n: masked,
            link_tip: tip,
        }
    }
}

// ==================== Apply (sign up) ====================

pub struct ApplyResult {
    pub id: u64,
    pub job_id: u64,
}

pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    // PHPYun rule: only jobseekers can apply
    user.require_jobseeker()
        .map_err(|_| ApiError::business("part_role_not_allowed"))?;

    let job = part_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(ApiError::business("part_not_found"))?;

    // Expiry and state
    let now = clock::now_ts();
    if job.edate > 0 && job.edate < now {
        return Err(ApiError::business("part_expired"));
    }
    if job.status == 1 {
        return Err(ApiError::business("part_offline"));
    }
    if job.state != 1 {
        return Err(ApiError::business("part_pending"));
    }

    // Deduplicate
    if part_repo::find_apply(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(ApiError::business("part_apply_duplicate"));
    }

    let id = part_repo::create_apply(state.db.pool(), user.uid, job_id, job.uid, now).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("part.apply", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("partjob:{job_id}"))
            .meta(&serde_json::json!({ "apply_id": id, "com_id": job.uid })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "part.apply.created",
            &serde_json::json!({
                "apply_id": id,
                "uid": user.uid,
                "job_id": job_id,
                "com_id": job.uid,
            }),
        )
        .await;

    Ok(ApplyResult { id, job_id })
}

// ==================== Favourite (collect) ====================

pub async fn collect(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    com_id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_jobseeker()
        .map_err(|_| ApiError::business("part_role_not_allowed"))?;

    let job = part_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(ApiError::business("part_not_found"))?;

    // PHP does no strict check on comid; we override it with the com_id from the job (prevents client tampering)
    let real_com = job.uid;
    if com_id != 0 && com_id != real_com {
        // Lenient: prefer the server-side value, do not error (matches PHP's permissive behavior)
    }

    if part_repo::find_collect(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(ApiError::business("part_collect_duplicate"));
    }

    let id =
        part_repo::create_collect(state.db.pool(), user.uid, job_id, real_com, clock::now_ts())
            .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("part.collect", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("partjob:{job_id}")),
    )
    .await;

    Ok(id)
}

// ==================== Jobseeker: my applications / favourites ====================

pub async fn list_my_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartApply>> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        part_repo::count_applies_by_uid(state.db.reader(), user.uid),
        part_repo::list_applies_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_my_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = part_repo::delete_applies(state.db.pool(), ids, Some(user.uid), None).await?;
    Ok(n)
}

pub async fn list_my_collects(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartCollect>> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        part_repo::count_collects_by_uid(state.db.reader(), user.uid),
        part_repo::list_collects_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_my_collects(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = part_repo::delete_collects(state.db.pool(), ids, Some(user.uid)).await?;
    Ok(n)
}

// ==================== Company: manage own part-time listings ====================

pub async fn list_com_parts(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartJob>> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        part_repo::count_by_com(state.db.reader(), user.uid),
        part_repo::list_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_com_parts(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    // Delete the main rows first (uid filter prevents privilege escalation), then cascade child rows
    let affected = part_repo::delete_by_ids(state.db.pool(), ids, Some(user.uid)).await?;
    if affected > 0 {
        part_repo::cascade_delete_children(state.db.pool(), ids).await?;
    }
    Ok(affected)
}

// Company view: see every application against the company's own part-time jobs
pub async fn list_com_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartApply>> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        part_repo::count_applies_by_com(state.db.reader(), user.uid),
        part_repo::list_applies_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn update_com_apply_status(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    status: i32,
) -> AppResult<u64> {
    user.require_employer()?;
    // PHPYun semantics: 1 = unread / 2 = read / 3 = contacted
    if !(1..=3).contains(&status) {
        return Err(ApiError::param_invalid("status"));
    }
    let n = part_repo::update_apply_status(state.db.pool(), apply_id, user.uid, status).await?;
    Ok(n)
}

pub struct MemberPartInput<'a> {
    pub name: &'a str,
    pub r#type: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: &'a str,
    pub number: i32,
    pub sex: i32,
    pub salary: i32,
    pub salary_type: i32,
    pub billing_cycle: i32,
    pub worktime: &'a str,
    pub sdate: i64,
    pub edate: i64,
    pub content: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub x: &'a str,
    pub y: &'a str,
    pub deadline: i64,
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= now
}

async fn consume_part_refresh_quota(state: &AppState, uid: u64, n: i32) -> AppResult<()> {
    let now = clock::now_ts();
    let st = phpyun_models::company_statis::repo::find_admin(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("zph_need_vip"))?;
    if !is_vip(st.vip_etime, now) {
        return Err(ApiError::business("zph_need_vip"));
    }
    if st.rating_type == 2 {
        return Ok(());
    }
    if st.rating_type == 1 {
        if !phpyun_models::company_statis::repo::try_consume_breakpart(state.db.pool(), uid, n)
            .await?
        {
            return Err(ApiError::business("part_refresh_quota"));
        }
        return Ok(());
    }
    Err(ApiError::business("part_refresh_quota"))
}

fn write_from_input<'a>(input: &'a MemberPartInput<'a>) -> part_repo::MemberPartWrite<'a> {
    part_repo::MemberPartWrite {
        name: input.name,
        r#type: input.r#type,
        provinceid: input.provinceid,
        cityid: input.cityid,
        three_cityid: input.three_cityid,
        address: input.address,
        number: input.number,
        sex: input.sex,
        salary: input.salary,
        salary_type: input.salary_type,
        billing_cycle: input.billing_cycle,
        worktime: input.worktime,
        sdate: input.sdate,
        edate: input.edate,
        content: input.content,
        linkman: input.linkman,
        linktel: input.linktel,
        x: input.x,
        y: input.y,
        deadline: input.deadline,
    }
}

pub async fn create_com_part(
    state: &AppState,
    user: &AuthenticatedUser,
    input: MemberPartInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    let now = clock::now_ts();
    let looked_up = phpyun_models::company::repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .and_then(|c| c.name)
        .unwrap_or_default();
    let id = part_repo::locoy_create(
        state.db.pool(),
        &part_repo::LocoyPartCreate {
            uid: user.uid,
            name: input.name,
            com_name: looked_up.as_str(),
            r#type: input.r#type,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            address: input.address,
            number: input.number,
            sex: input.sex,
            salary: input.salary,
            salary_type: input.salary_type,
            billing_cycle: input.billing_cycle,
            worktime: input.worktime,
            sdate: input.sdate,
            edate: input.edate,
            content: input.content,
            linkman: input.linkman,
            linktel: input.linktel,
            state: 0,
            x: input.x,
            y: input.y,
            deadline: input.deadline,
            now,
            did: user.did,
        },
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("part.create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("part:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn update_com_part(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: MemberPartInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let now = clock::now_ts();
    let n = part_repo::update_for_com(state.db.pool(), id, user.uid, &write_from_input(&input), now)
        .await?;
    if n == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("part.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("part:{id}")),
    )
    .await;
    Ok(())
}

pub async fn set_com_part_status(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let status = match status {
        0 => 0,
        1 | 2 => 1,
        _ => return Err(ApiError::param_invalid("status")),
    };
    let job = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    if status == 0 && job.state != 1 {
        return Err(ApiError::business("job_pending"));
    }
    let n = part_repo::set_status_for_com(state.db.pool(), id, user.uid, status).await?;
    if n == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("part.status_change", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("part:{id}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

pub async fn refresh_com_part(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let _job = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    consume_part_refresh_quota(state, user.uid, 1).await?;
    let now = clock::now_ts();
    let n = part_repo::refresh_for_com(state.db.pool(), id, user.uid, now).await?;
    if n == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("part.refresh", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("part:{id}")),
    )
    .await;
    Ok(())
}
