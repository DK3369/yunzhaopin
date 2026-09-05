//! Resume service (usertype=1 jobseekers).
//!
//! Covers the core paths of PHPYun `wap/resume` + `mcenter/resume`: viewing, updating the master table, and toggling display status.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::extractors::USERTYPE_EMPLOYER;
use phpyun_core::ApiError;
use phpyun_core::{background, clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::resume::repo::ResumeFilter;
use phpyun_models::resume::{entity::Resume, repo as resume_repo};

pub struct ResumePage {
    pub list: Vec<Resume>,
    pub total: u64,
}

/// Public resume search (aligned with PHP `wap/resume`: guests may list;
/// contact fields are masked in the handler for non-employers).
pub async fn list_public(
    state: &AppState,
    filter: &ResumeFilter<'_>,
    page: Pagination,
) -> AppResult<ResumePage> {
    let db = state.db.reader();
    let (total, list, tops) = tokio::join!(
        resume_repo::count_public(db, filter),
        resume_repo::list_public(db, filter, page.offset, page.limit),
        async {
            if page.page <= 1 && !filter.top {
                resume_repo::list_top_random(db, filter, 5).await
            } else {
                Ok(Vec::new())
            }
        },
    );
    let total = total?;
    let mut list = list?;
    let tops = tops?;
    if !tops.is_empty() {
        let top_uids: std::collections::HashSet<u64> = tops.iter().map(|r| r.uid).collect();
        list.retain(|r| !top_uids.contains(&r.uid));
        let mut merged = tops;
        merged.extend(list);
        list = merged;
    }
    Ok(ResumePage { total, list })
}

/// Public resume detail. Employers also see `status=3` when the seeker applied.
pub async fn get_public(
    state: &AppState,
    uid: u64,
    viewer: Option<&AuthenticatedUser>,
) -> AppResult<Resume> {
    let owner = viewer.is_some_and(|u| u.uid == uid && u.usertype == 1);
    let resume = if owner {
        resume_repo::find_by_uid(state.db.reader(), uid)
            .await?
            .ok_or_else(|| ApiError::business("resume_not_found"))?
    } else if let Some(u) = viewer.filter(|u| u.usertype == USERTYPE_EMPLOYER) {
        resume_repo::find_visible_for_employer(state.db.reader(), uid, u.uid)
            .await?
            .ok_or_else(|| ApiError::business("resume_not_found"))?
    } else {
        resume_repo::find_public(state.db.reader(), uid)
            .await?
            .ok_or_else(|| ApiError::business("resume_not_found"))?
    };
    if !owner {
        if let Some(expect) =
            phpyun_models::resume::expect::find_default_by_uid(state.db.reader(), uid).await?
        {
            if expect.state == 0 {
                return Err(ApiError::business("resume_unavailable"));
            }
            if expect.r_status == 2 {
                return Err(ApiError::business("resume_hidden"));
            }
            if expect.state == 3 {
                return Err(ApiError::business("resume_bad_status"));
            }
        }
    }
    Ok(resume)
}

/// PHP `lookresume.model.php::browseResume` — company/hunter viewing a resume.
pub fn browse_resume_async(
    state: &AppState,
    viewer: &AuthenticatedUser,
    resume_uid: u64,
    eid: u64,
    ip: String,
) {
    if viewer.uid == resume_uid {
        return;
    }
    if viewer.usertype != 2 && viewer.usertype != 3 {
        return;
    }
    let pool = state.db.pool().clone();
    let com_id = viewer.uid;
    let usertype = i32::from(viewer.usertype);
    let did = viewer.did;
    let mut eid = eid;
    background::spawn_best_effort("look_resume.browse", async move {
        if eid == 0 {
            eid = phpyun_models::resume::expect::find_default_id_by_uid(&pool, resume_uid)
                .await
                .ok()
                .flatten()
                .unwrap_or(0);
        }
        if eid == 0 {
            return;
        }
        if usertype == 2 {
            let name_ok = phpyun_models::company::repo::find_by_uid(&pool, com_id)
                .await
                .ok()
                .flatten()
                .and_then(|c| c.name)
                .filter(|n| !n.trim().is_empty())
                .is_some();
            if !name_ok {
                return;
            }
        }
        let now = clock::now_ts();
        if let Ok(Some(id)) = resume_repo::find_look_resume(&pool, com_id, eid, usertype).await {
            let _ = resume_repo::touch_look_resume(&pool, id, now).await;
        } else {
            let _ = resume_repo::insert_look_resume(
                &pool, resume_uid, eid, com_id, did, usertype, now, &ip,
            )
            .await;
        }
        let _ = resume_repo::mark_userid_job_browsed(&pool, com_id, eid).await;
    });
}

pub struct ResumeUpdateInput<'a> {
    pub name: Option<&'a str>,
    pub nametype: Option<i32>,
    pub sex: Option<i32>,
    pub birthday: Option<&'a str>,
    pub marriage: Option<i32>,
    pub education: Option<i32>,
    pub telphone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub photo: Option<&'a str>,
    pub exp: Option<i32>,
    pub living: Option<&'a str>,
    pub domicile: Option<&'a str>,
    pub height: Option<&'a str>,
    pub weight: Option<&'a str>,
    pub address: Option<&'a str>,
    pub description: Option<&'a str>,
    pub qq: Option<&'a str>,
    pub idcard: Option<&'a str>,
    pub idcard_pic: Option<&'a str>,
}

/// Fetch the jobseeker's own resume. If the resume row does not exist (legacy data / new signup), create an empty row automatically.
pub async fn get_mine(state: &AppState, user: &AuthenticatedUser) -> AppResult<Resume> {
    user.require_jobseeker()?;

    // Read from the reader first; if missing, ensure_row and re-read
    if let Some(r) = resume_repo::find_by_uid(state.db.reader(), user.uid).await? {
        return Ok(r);
    }
    resume_repo::ensure_row(state.db.pool(), user.uid, user.did, clock::now_ts()).await?;
    resume_repo::find_by_uid(state.db.pool(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("resume_not_found"))
}

pub async fn update_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    input: ResumeUpdateInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    resume_repo::ensure_row(state.db.pool(), user.uid, user.did, clock::now_ts()).await?;
    resume_repo::update(
        state.db.pool(),
        user.uid,
        resume_repo::ResumeUpdate {
            name: input.name,
            nametype: input.nametype,
            sex: input.sex,
            birthday: input.birthday,
            marriage: input.marriage,
            education: input.education,
            telphone: input.telphone,
            email: input.email,
            photo: input.photo,
            exp: input.exp,
            living: input.living,
            domicile: input.domicile,
            height: input.height,
            weight: input.weight,
            address: input.address,
            description: input.description,
            qq: input.qq,
            idcard: input.idcard,
            idcard_pic: input.idcard_pic,
        },
        clock::now_ts(),
    )
    .await?;

    tracing::info!(
        op = "resume.update_main", uid = user.uid, ip = client_ip,
        name = ?input.name, sex = ?input.sex, education = ?input.education,
        "wizard write"
    );

    let _ = audit::emit(
        state,
        AuditEvent::new("resume.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(())
}

/// Refresh the resume (bump lastupdate) — backs the jobseeker's "refresh resume" button.
/// Rate limit: at most once every 5 minutes per uid to prevent abuse.
pub async fn refresh_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    client_ip: &str,
) -> AppResult<i64> {
    user.require_jobseeker()?;
    // Rate limit
    phpyun_core::rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:resume_refresh:{}", user.uid),
        phpyun_core::rate_limit::LimitRule {
            max: 1,
            window: std::time::Duration::from_secs(300),
        },
    )
    .await?;

    let now = clock::now_ts();
    resume_repo::touch_lastupdate(state.db.pool(), user.uid, now).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.refresh", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(now)
}

pub async fn set_status(
    state: &AppState,
    user: &AuthenticatedUser,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    if !matches!(status, 1..=3) {
        return Err(ApiError::business("resume_bad_status"));
    }
    resume_repo::update_status(state.db.pool(), user.uid, status).await?;

    let label = match status {
        1 => "public",
        2 => "hidden",
        3 => "bound_only",
        _ => "unknown",
    };
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.status_change",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("uid:{}", user.uid))
        .meta(&serde_json::json!({ "status": label })),
    )
    .await;
    Ok(())
}

/// PHP `$resumeCkeck`: 1 = full body, 2 = counts-only summary.
pub struct ResumeOpenGate {
    pub resume_check: i32,
    /// Site config `resume_open_check` (1–4), for the front-end `showresdet` branch.
    pub resume_open_check: i32,
}

/// PHP `resume.model.php::openResumeCheck`. Owner always sees the full body.
pub async fn open_resume_check(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    resume_uid: u64,
) -> ResumeOpenGate {
    let cfg = phpyun_models::site_setting::repo::find_many(state.db.reader(), &["resume_open_check"])
        .await
        .unwrap_or_default();
    let mode = cfg
        .get("resume_open_check")
        .and_then(|s| s.trim().parse::<i32>().ok())
        .filter(|n| (1..=4).contains(n))
        .unwrap_or(2);

    if user.is_some_and(|u| u.uid == resume_uid && u.usertype == 1) {
        return ResumeOpenGate {
            resume_check: 1,
            resume_open_check: mode,
        };
    }

    let full = match mode {
        1 => true,
        // PHP: `($uid && usertype==2) || usertype==3`
        2 => user.is_some_and(|u| u.usertype == 2 || u.usertype == 3),
        3 => match user {
            Some(u) if u.usertype == 2 => {
                phpyun_models::job::repo::count_posted_by_uid(state.db.reader(), u.uid)
                    .await
                    .unwrap_or(0)
                    > 0
            }
            _ => false,
        },
        4 => downloaded_resume_body(state, user, resume_uid).await,
        _ => false,
    };

    ResumeOpenGate {
        resume_check: if full { 1 } else { 2 },
        resume_open_check: mode,
    }
}

async fn downloaded_resume_body(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    resume_uid: u64,
) -> bool {
    let Some(u) = user else {
        return false;
    };
    let db = state.db.reader();
    if phpyun_models::resume_download::repo::already_downloaded(db, u.uid, resume_uid)
        .await
        .unwrap_or(false)
    {
        return true;
    }
    if u.usertype != 2 {
        return false;
    }
    let vip_ok = match phpyun_models::company_statis::repo::find_admin(db, u.uid).await {
        Ok(Some(st)) => st.vip_etime == 0 || st.vip_etime >= clock::now_ts(),
        _ => false,
    };
    if !vip_ok {
        return false;
    }
    phpyun_models::resume_download::repo::already_freedown(db, u.uid, resume_uid)
        .await
        .unwrap_or(false)
}

pub struct LookResumePage {
    pub list: Vec<phpyun_models::look_resume::LookResume>,
    pub total: u64,
}

/// PHP `member/user/look` — companies who viewed my resume.
pub async fn list_look_resumes(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<LookResumePage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        phpyun_models::look_resume::count_by_resume_uid(state.db.reader(), user.uid),
        phpyun_models::look_resume::list_by_resume_uid(
            state.db.reader(),
            user.uid,
            page.offset,
            page.limit
        ),
    );
    Ok(LookResumePage {
        total: total?,
        list: list?,
    })
}

pub async fn hide_look_resume(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = phpyun_models::look_resume::hide_by_uid(state.db.pool(), id, user.uid).await?;
    if n == 0 {
        return Err(ApiError::business("not_found"));
    }
    Ok(n)
}

/// PHP `member/com/look_resume` — resumes I viewed.
pub async fn list_look_resumes_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<LookResumePage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        phpyun_models::look_resume::count_by_com(state.db.reader(), user.uid),
        phpyun_models::look_resume::list_by_com(
            state.db.reader(),
            user.uid,
            page.offset,
            page.limit
        ),
    );
    Ok(LookResumePage {
        total: total?,
        list: list?,
    })
}

/// PHP `userpay.model.php::buyZdresume`.
pub struct ResumeTopResult {
    pub status: i32,
    pub order_id: String,
    pub price: f64,
    pub msg: Option<String>,
}

pub async fn buy_top(
    state: &AppState,
    user: &AuthenticatedUser,
    resume_id: u64,
    days: i32,
    paytype: &str,
) -> AppResult<ResumeTopResult> {
    user.require_jobseeker()?;
    if days <= 0 || days > 365 {
        return Err(ApiError::param_invalid("days"));
    }
    let db = state.db.reader();
    let topdate = phpyun_models::resume::expect::find_topdate(db, resume_id, user.uid)
        .await?
        .ok_or_else(|| ApiError::business("common_06645"))?;
    let unit = phpyun_models::site_setting::repo::find(db, "integral_resume_top")
        .await?
        .and_then(|r| r.value.trim().parse::<f64>().ok())
        .unwrap_or(0.0);
    let price = (unit * f64::from(days) * 100.0).round() / 100.0;
    let now = clock::now_ts();
    let order_id = format!("{now}{:05}", (now % 90_000) + 10_000);
    if price <= 0.0 {
        let next = if topdate > now {
            topdate.saturating_add(i64::from(days) * 86_400)
        } else {
            now.saturating_add(i64::from(days) * 86_400)
        };
        phpyun_models::resume::expect::set_member_top(state.db.pool(), user.uid, resume_id, next)
            .await?;
        let _ = phpyun_models::resume::expect::insert_top_order(
            state.db.pool(),
            user.uid,
            &order_id,
            paytype,
            0.0,
            resume_id,
            days,
            now,
            2,
        )
        .await;
        return Ok(ResumeTopResult {
            status: 3,
            order_id,
            price: 0.0,
            msg: None,
        });
    }
    phpyun_models::resume::expect::insert_top_order(
        state.db.pool(),
        user.uid,
        &order_id,
        paytype,
        price,
        resume_id,
        days,
        now,
        1,
    )
    .await?;
    Ok(ResumeTopResult {
        status: 2,
        order_id,
        price,
        msg: Some("wap_user_00207".into()),
    })
}

pub async fn settle_top_order(state: &AppState, order_no: &str) -> AppResult<()> {
    let row = phpyun_models::resume::expect::find_top_order(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if row.order_state == 2 {
        return Ok(());
    }
    let n = phpyun_models::resume::expect::mark_top_order_paid(state.db.pool(), order_no).await?;
    if n == 0 {
        return Ok(());
    }
    let resume_id = u64::try_from(row.rating).unwrap_or(0);
    let days = row.sid;
    if resume_id == 0 || days <= 0 {
        return Ok(());
    }
    let now = clock::now_ts();
    let topdate = phpyun_models::resume::expect::find_topdate(
        state.db.reader(),
        resume_id,
        row.uid,
    )
    .await?
    .unwrap_or(0);
    let next = if topdate > now {
        topdate.saturating_add(i64::from(days) * 86_400)
    } else {
        now.saturating_add(i64::from(days) * 86_400)
    };
    let _ = phpyun_models::resume::expect::set_member_top(
        state.db.pool(),
        row.uid,
        resume_id,
        next,
    )
    .await;
    Ok(())
}
