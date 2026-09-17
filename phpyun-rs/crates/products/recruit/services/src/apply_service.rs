//! Application flow: jobseeker submits a resume + employer reviews + interactions
//! (mark as read / invite to interview).
//!
//! Aligned with PHPYun `wap/job::comapply_action` (submit) + `mcenter/applicant`
//! (employer view) + `mcenter/apply` (jobseeker view).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::apply::{entity::Apply, repo as apply_repo};
use phpyun_models::category::repo as category_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::job_scrape::repo as scrape_repo;
use phpyun_models::message::repo as message_repo;
use phpyun_models::resume::expect as expect_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume_download::repo as download_repo;
use phpyun_models::site_setting::repo as setting_repo;
use std::collections::HashMap;

// ==================== Jobseeker submission ====================

pub struct ApplyResult {
    pub id: u64,
    pub job_id: u64,
    pub apply_url: String,
}

fn parse_req_id(raw: &str) -> i32 {
    raw.trim().parse().unwrap_or(0)
}

fn cfg_i32(map: &std::collections::HashMap<String, String>, key: &str) -> i32 {
    map.get(key)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn php_year_age(birthday: &str) -> i32 {
    let by: i32 = birthday
        .trim()
        .get(..4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if by <= 0 {
        return 0;
    }
    i32::from(clock::now_year()) - by
}

async fn sort_below(
    pool: &sqlx::MySqlPool,
    have_id: i32,
    need_id: i32,
) -> AppResult<bool> {
    let need = category_repo::userclass_sort(pool, need_id).await?;
    let Some(need_sort) = need else {
        return Ok(false);
    };
    let have = category_repo::userclass_sort(pool, have_id).await?;
    Ok(match have {
        Some(h) => h < need_sort,
        None => true,
    })
}

fn gate_or_mark(strict: bool, key: &'static str, is_browse: &mut i32) -> AppResult<()> {
    if strict {
        Err(ApiError::business(key))
    } else {
        *is_browse = 4;
        Ok(())
    }
}

pub async fn apply_to_job(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    user.require_jobseeker()?;

    // 1. The job must be applicable: online / approved / not expired
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(ApiError::business("job_not_found"))?;
    if job.status == 2 {
        return Err(ApiError::business("job_offline"));
    }
    if job.state != 1 || job.r_status != 1 {
        return Err(ApiError::business("job_pending"));
    }
    if job.edate > 0 && job.edate <= clock::now_ts() {
        return Err(ApiError::business("job_expired"));
    }

    // 2. Cannot apply to your own posting (edge case where jobseeker uid = employer uid)
    if job.uid == user.uid {
        return Err(ApiError::business("apply_own_job"));
    }

    if !state
        .redis
        .acquire_lock(&format!("apply:{}:{job_id}", user.uid), "1", 8_000)
        .await?
    {
        return Err(ApiError::business("apply_duplicate"));
    }

    let source_url = scrape_repo::find_url_by_job_id(state.db.reader(), job_id)
        .await?
        .unwrap_or_default();
    if !source_url.is_empty() {
        return apply_scrape_job(state, user, &job, &source_url, client_ip).await;
    }

    // 3. Prevent duplicate applications
    if apply_repo::find_by_uid_job(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(ApiError::business("apply_duplicate"));
    }

    let expect = expect_repo::find_apply_expect(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("common_00475"))?;
    if expect.uname.trim().is_empty() || expect.edu == 0 || expect.exp == 0 {
        return Err(ApiError::business("common_01055"));
    }
    let city = expect.city_classid.trim();
    if city.is_empty() || city == "0" {
        return Err(ApiError::business("common_00675"));
    }

    let cfg = setting_repo::find_many(
        state.db.reader(),
        &["user_sqintegrity", "sy_shresume_applyjob", "sqjob_req"],
    )
    .await
    .unwrap_or_default();
    let need_integrity = cfg_i32(&cfg, "user_sqintegrity");
    if need_integrity > 0 && expect.integrity < need_integrity {
        return Err(ApiError::business("common_01148"));
    }
    let sh_ok = cfg.get("sy_shresume_applyjob").map(|s| s.trim() == "1").unwrap_or(false);
    match expect.state {
        0 if !sh_ok => return Err(ApiError::business("common_06286")),
        2 => return Err(ApiError::business("common_00801")),
        3 => return Err(ApiError::business("common_06287")),
        _ => {}
    }
    if expect.status == 2 {
        return Err(ApiError::business("default_00002"));
    }

    let strict = cfg.get("sqjob_req").map(|s| s.trim() == "1").unwrap_or(false);
    let pool = state.db.reader();
    let mut is_browse = 1i32;
    let exp_req = parse_req_id(&job.exp_req);
    if exp_req > 0 && sort_below(pool, expect.exp, exp_req).await? {
        gate_or_mark(strict, "common_00708", &mut is_browse)?;
    }
    let edu_req = parse_req_id(&job.edu_req);
    if edu_req > 0 && sort_below(pool, expect.edu, edu_req).await? {
        gate_or_mark(strict, "common_00883", &mut is_browse)?;
    }
    if job.sex_req > 0
        && expect.sex != job.sex_req
        && expect.sex != 3
        && job.sex_req != 3
    {
        gate_or_mark(strict, "common_00885", &mut is_browse)?;
    }
    if job.minage_req > 0 || job.maxage_req > 0 {
        let age = php_year_age(&expect.birthday);
        if (job.minage_req > 0 && age < job.minage_req)
            || (job.maxage_req > 0 && age > job.maxage_req)
        {
            gate_or_mark(strict, "common_00884", &mut is_browse)?;
        }
    }

    let com_name = job.com_name.clone().unwrap_or_default();
    let id = match apply_repo::create(
        state.db.pool(),
        apply_repo::ApplyCreate {
            uid: user.uid,
            job_id,
            job_name: &job.name,
            apply_url: "",
            com_id: job.uid,
            com_name: &com_name,
            eid: expect.id,
            now: clock::now_ts(),
            is_browse,
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) if apply_repo::is_unique_violation(&e) => {
            return Err(ApiError::business("apply_duplicate"));
        }
        Err(e) => return Err(e.into()),
    };

    // 5. Audit + event bus (paves the way for future email notifications)
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.apply", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "apply_id": id, "com_id": job.uid })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "apply.created",
            &serde_json::json!({
                "apply_id": id,
                "uid": user.uid,
                "job_id": job_id,
                "com_id": job.uid,
            }),
        )
        .await;

    Ok(ApplyResult {
        id,
        job_id,
        apply_url: String::new(),
    })
}

async fn apply_scrape_job(
    state: &AppState,
    user: &AuthenticatedUser,
    job: &phpyun_models::job::entity::Job,
    source_url: &str,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    let apply_url: String = source_url.chars().take(512).collect();
    if let Some(existing) = apply_repo::find_by_uid_job(state.db.reader(), user.uid, job.id).await? {
        if existing.apply_url.is_empty() {
            let _ = apply_repo::set_apply_url(state.db.pool(), existing.id, &apply_url).await;
        }
        let url = if existing.apply_url.is_empty() {
            apply_url
        } else {
            existing.apply_url
        };
        return Ok(ApplyResult {
            id: existing.id,
            job_id: job.id,
            apply_url: url,
        });
    }
    let eid = expect_repo::find_apply_expect(state.db.reader(), user.uid)
        .await?
        .map(|e| e.id)
        .unwrap_or(0);
    let com_name = job.com_name.clone().unwrap_or_default();
    let id = match apply_repo::create(
        state.db.pool(),
        apply_repo::ApplyCreate {
            uid: user.uid,
            job_id: job.id,
            job_name: &job.name,
            apply_url: &apply_url,
            com_id: job.uid,
            com_name: &com_name,
            eid,
            now: clock::now_ts(),
            is_browse: 1,
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) if apply_repo::is_unique_violation(&e) => {
            return Err(ApiError::business("apply_duplicate"));
        }
        Err(e) => return Err(e.into()),
    };
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.apply", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{}", job.id))
            .meta(&serde_json::json!({
                "apply_id": id,
                "com_id": job.uid,
                "apply_url": apply_url,
            })),
    )
    .await;
    Ok(ApplyResult {
        id,
        job_id: job.id,
        apply_url,
    })
}

// ==================== Jobseeker: my applications ====================

pub struct ApplyPage {
    pub list: Vec<Apply>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    state_filter: Option<i32>,
    days: Option<i32>,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_uid(state.db.reader(), user.uid, state_filter, days),
        apply_repo::list_by_uid(
            state.db.reader(),
            user.uid,
            state_filter,
            days,
            page.offset,
            page.limit,
        ),
    );
    Ok(ApplyPage {
        total: total?,
        list: list?,
    })
}

pub async fn withdraw(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let affected = apply_repo::withdraw(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.apply_withdraw",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

pub async fn hide_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let affected = apply_repo::hide_by_uid(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.apply_delete",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

/// PHP hr `uptime`: 1 means "updated since midnight today", any other N means
/// "within the last N days".
pub fn resume_updated_cutoff(days: i32, now: i64) -> i64 {
    if days == 1 {
        now - now.rem_euclid(86_400)
    } else {
        now - i64::from(days) * 86_400
    }
}

pub async fn list_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    filter: apply_repo::ApplyFilter,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_com(state.db.reader(), user.uid, &filter),
        apply_repo::list_by_com(state.db.reader(), user.uid, &filter, page.offset, page.limit),
    );
    let mut list = list?;
    let uids: Vec<u64> = list.iter().map(|a| a.uid).collect();
    let names = apply_repo::resume_names_by_uids(state.db.reader(), &uids).await?;
    for row in &mut list {
        if let Some(n) = names.get(&row.uid) {
            row.uname = n.clone();
        }
    }
    Ok(ApplyPage {
        total: total?,
        list,
    })
}

#[derive(Debug, Clone, Default)]
pub struct ApplicantCard {
    pub photo: String,
    pub sex_n: String,
    pub age: i32,
    pub edu_n: String,
    pub exp_n: String,
    pub salary: String,
    pub telphone: String,
    pub islink: i32,
}

fn salary_label(min: i32, max: i32) -> String {
    if min > 0 && max > 0 && max != min {
        format!("{min}-{max}")
    } else if min > 0 {
        min.to_string()
    } else if max > 0 {
        max.to_string()
    } else {
        String::new()
    }
}

/// Batch resume + download flags for one page of applications. Keyed by seeker uid.
pub async fn applicant_cards(
    state: &AppState,
    com_id: u64,
    rows: &[(u64, u64)],
) -> AppResult<HashMap<u64, ApplicantCard>> {
    let uids: Vec<u64> = rows.iter().map(|(uid, _)| *uid).collect();
    let eids: Vec<u64> = rows.iter().map(|(_, eid)| *eid).filter(|e| *e > 0).collect();
    let (resumes, salaries, unlocked, dicts) = tokio::join!(
        resume_repo::cards_by_uids(state.db.reader(), &uids),
        expect_repo::salary_by_ids(state.db.reader(), &eids),
        download_repo::unlocked_uids(state.db.reader(), com_id, &uids),
        crate::dict_service::get(state),
    );
    let resumes = resumes?;
    let salaries = salaries?;
    let unlocked = unlocked?;
    let dicts = dicts?;
    let resume_map: HashMap<u64, resume_repo::ResumeCard> =
        resumes.into_iter().map(|r| (r.uid, r)).collect();
    let eid_by_uid: HashMap<u64, u64> = rows.iter().copied().collect();
    let mut out = HashMap::new();
    for uid in uids {
        let islink = i32::from(unlocked.contains(&uid));
        let r = resume_map.get(&uid);
        let (min, max) = eid_by_uid
            .get(&uid)
            .and_then(|eid| salaries.get(eid))
            .copied()
            .unwrap_or((0, 0));
        let photo = r
            .and_then(|c| {
                if c.phototype == 1 {
                    None
                } else {
                    c.photo
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                }
            })
            .unwrap_or_default();
        let sex = r.map(|c| c.sex).unwrap_or(0);
        let edu = r.map(|c| c.education).unwrap_or(0);
        let exp = r.map(|c| c.exp).unwrap_or(0);
        let age = r
            .and_then(|c| c.birthday.as_deref())
            .map(php_year_age)
            .filter(|a| *a > 0)
            .unwrap_or(0);
        let tel = if islink == 1 {
            r.and_then(|c| c.telphone.clone())
                .unwrap_or_default()
        } else {
            String::new()
        };
        out.insert(
            uid,
            ApplicantCard {
                photo,
                sex_n: crate::enum_labels::sex_n(sex),
                age,
                edu_n: dicts.user_or_com(edu).to_string(),
                exp_n: dicts.user_or_com(exp).to_string(),
                salary: salary_label(min, max),
                telphone: tel,
                islink,
            },
        );
    }
    Ok(out)
}

/// Tab counts for the received-applications screen. `browse_state` is cleared
/// so selecting one tab does not zero out the others.
pub async fn state_counts_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    mut filter: apply_repo::ApplyFilter,
) -> AppResult<std::collections::HashMap<i32, u64>> {
    user.require_employer()?;
    filter.browse_state = None;
    filter.unread_only = None;
    Ok(apply_repo::count_states_by_com(state.db.reader(), user.uid, &filter).await?)
}

pub async fn mark_browsed(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = apply_repo::mark_browsed(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        // Not found / already read — both treated as success: idempotent
    }
    Ok(())
}

/// PHP `ReadSqJob`: bulk "mark as read" from the list checkboxes.
pub async fn mark_browsed_batch(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    Ok(apply_repo::mark_browsed_batch(state.db.pool(), ids, user.uid).await?)
}

/// PHP `delSqJob` on the employer side: hide the application from this company.
pub async fn delete_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = apply_repo::hide_by_com(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "application.delete",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

pub async fn delete_for_company_ids(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    if ids.is_empty() {
        return Err(ApiError::param_invalid("id"));
    }
    let affected = apply_repo::hide_by_com_ids(state.db.pool(), ids, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "application.delete",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{}", ids.len()))
        .meta(&serde_json::json!({ "ids": ids })),
    )
    .await;
    Ok(affected)
}

/// Employer side: set the application's `is_browse` to any enum value.
/// PHPYun convention: 1=not viewed / 2=viewed / 3=interviewed / 4=not a fit / 5=unreachable / 7=hired.
/// Invalid values are rejected.
pub async fn set_browse_state(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    new_state: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    if !matches!(new_state, 1 | 2 | 3 | 4 | 5 | 7) {
        return Err(ApiError::param_invalid("state"));
    }
    let apply = apply_repo::find_by_id(state.db.reader(), apply_id)
        .await?
        .filter(|a| a.com_id == user.uid)
        .ok_or_else(|| ApiError::business("apply_not_owner"))?;
    let affected =
        apply_repo::set_browse_state(state.db.pool(), apply_id, user.uid, new_state).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let now = clock::now_ts();
    if apply.job_id > 0 {
        let _ = job_repo::touch_operatime(state.db.pool(), apply.job_id, now).await;
    }
    if new_state == 4 {
        let _ = message_repo::create(
            state.db.pool(),
            message_repo::MessageCreate {
                uid: apply.uid,
                recipient_usertype: 1,
                title: "sqzwhf",
                body: Some("sqzwhf"),
                category: "apply",
                ref_kind: 0,
                ref_id: apply_id,
            },
            now,
        )
        .await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "application.state_change",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}"))
        .meta(&serde_json::json!({ "new_state": new_state })),
    )
    .await;
    Ok(())
}

pub async fn invite_interview(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = apply_repo::invite(state.db.pool(), apply_id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("interview.invite", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

pub async fn next_unread(
    state: &AppState,
    user: &AuthenticatedUser,
    after_id: u64,
) -> AppResult<Option<(u64, u64, u64)>> {
    user.require_employer()?;
    Ok(apply_repo::next_unread_id(state.db.reader(), user.uid, after_id).await?)
}

pub async fn ever_applied(state: &AppState, user: &AuthenticatedUser, eid: u64) -> AppResult<bool> {
    user.require_employer()?;
    Ok(apply_repo::exists_by_com_eid(state.db.reader(), user.uid, eid).await?)
}
