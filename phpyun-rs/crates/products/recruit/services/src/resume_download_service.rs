//! Resume download flow — company unlocks contact + counters.
//!
//! Aligned with PHPYun `downresume.model.php::downResume()`:
//! company audit, blacklist, already-unlocked, free_look for applicants,
//! package `down_resume` decrement, time-VIP daily cap. Never writes a
//! download row when the company has no remaining quota.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::i18n::{t, Lang};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};

const NOTIF_LANG: Lang = Lang::ZhCN;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::message::{entity as msg_entity, repo as message_repo};
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume_download::{entity::ResumeDownload, repo as download_repo};

pub struct DownloadPage {
    pub list: Vec<ResumeDownload>,
    pub total: u64,
}

fn today_start_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= today_start_ts(now)
}

fn has_contact(r: &phpyun_models::resume::entity::Resume) -> bool {
    let tel = r.telphone.as_deref().unwrap_or("").trim();
    let home = r.telhome.as_deref().unwrap_or("").trim();
    let email = r.email.as_deref().unwrap_or("").trim();
    !tel.is_empty() || !home.is_empty() || !email.is_empty()
}

async fn notify_first(
    state: &AppState,
    com_id: u64,
    target_uid: u64,
    now: i64,
    first_time: bool,
) -> AppResult<()> {
    if !first_time {
        return Ok(());
    }
    let dl_title = t("notifications.resume.downloaded_title", NOTIF_LANG);
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: target_uid,
            recipient_usertype: 1,
            category: "download",
            title: &dl_title,
            body: None,
            ref_kind: msg_entity::REF_RESUME,
            ref_id: target_uid,
        },
        now,
    )
    .await;
    let _ = state
        .events
        .publish_json(
            "resume.downloaded",
            &serde_json::json!({
                "com_id": com_id,
                "uid": target_uid,
            }),
        )
        .await;
    Ok(())
}

/// Company downloads a resume
pub async fn download(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;

    let company = company_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("company_not_found"))?;
    if company.r_status != 1 {
        return Err(ApiError::business("company_unapproved"));
    }
    if phpyun_models::blacklist::repo::is_blocked(state.db.reader(), user.uid, target_uid).await? {
        return Err(ApiError::business("blacklisted"));
    }

    let r = resume_repo::find_visible_for_employer(state.db.reader(), target_uid, user.uid)
        .await?
        .ok_or(ApiError::business("resume_not_found"))?;
    if r.r_status != 1 {
        return Err(ApiError::business("resume_not_found"));
    }
    if !has_contact(&r) {
        return Err(ApiError::business("no_contact"));
    }

    let already = download_repo::already_downloaded(state.db.reader(), user.uid, target_uid)
        .await
        .unwrap_or(false);
    let already_free = download_repo::already_freedown(state.db.reader(), user.uid, target_uid)
        .await
        .unwrap_or(false);
    if already || already_free {
        return Ok(());
    }

    let now = clock::now_ts();
    let today = today_start_ts(now);
    let eid = u64::try_from(r.def_job.max(0)).unwrap_or(0);
    let first_time = true;

    let statis = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("need_buy_down_resume"))?;

    if is_vip(statis.vip_etime, now) {
        let applied = phpyun_models::apply::repo::count_by_uid_to_company(
            state.db.reader(),
            target_uid,
            user.uid,
        )
        .await
        .unwrap_or(0)
            > 0;
        let freelook = statis_repo::freelook_num(state.db.reader(), statis.rating)
            .await
            .unwrap_or(0);
        let used_free =
            download_repo::count_today_freedown(state.db.reader(), user.uid, today).await?;
        if applied && (used_free as i32) < freelook {
            let _ = download_repo::record_freedown(state.db.pool(), user.uid, target_uid, eid, now)
                .await?;
            notify_first(state, user.uid, target_uid, now, first_time).await?;
            let _ = audit::emit(
                state,
                AuditEvent::new("resume.freedown", Actor::uid(user.uid).with_ip(client_ip))
                    .target(format!("uid:{target_uid}")),
            )
            .await;
            return Ok(());
        }

        if statis.rating_type == 1 {
            if !statis_repo::try_consume_down_resume(state.db.pool(), user.uid).await? {
                return Err(ApiError::business("need_buy_down_resume"));
            }
            let _ = download_repo::record(state.db.pool(), user.uid, target_uid, eid, now).await?;
            notify_first(state, user.uid, target_uid, now, first_time).await?;
            let _ = audit::emit(
                state,
                AuditEvent::new("resume.download", Actor::uid(user.uid).with_ip(client_ip))
                    .target(format!("uid:{target_uid}")),
            )
            .await;
            return Ok(());
        }
        if statis.rating_type == 2 {
            if statis.down_resume <= 0 {
                return Err(ApiError::business("vip_day_limit"));
            }
            let today_n = download_repo::count_today_down(state.db.reader(), user.uid, today).await?;
            if today_n >= statis.down_resume as u64 {
                return Err(ApiError::business("vip_day_limit"));
            }
            let _ = download_repo::record(state.db.pool(), user.uid, target_uid, eid, now).await?;
            notify_first(state, user.uid, target_uid, now, first_time).await?;
            let _ = audit::emit(
                state,
                AuditEvent::new("resume.download", Actor::uid(user.uid).with_ip(client_ip))
                    .target(format!("uid:{target_uid}")),
            )
            .await;
            return Ok(());
        }
        return Err(ApiError::business("need_buy_down_resume"));
    }

    Err(ApiError::business("need_buy_down_resume"))
}

/// Remaining package / free_look counts for the resume detail confirm dialog.
pub async fn remaining_for(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<(i32, i32)> {
    user.require_employer()?;
    let now = clock::now_ts();
    let today = today_start_ts(now);
    let Some(statis) = statis_repo::find_admin(state.db.reader(), user.uid).await? else {
        return Ok((0, 0));
    };
    let freelook = statis_repo::freelook_num(state.db.reader(), statis.rating)
        .await
        .unwrap_or(0);
    let used = download_repo::count_today_freedown(state.db.reader(), user.uid, today)
        .await
        .unwrap_or(0) as i32;
    let free_left = (freelook - used).max(0);
    Ok((statis.down_resume.max(0), free_left))
}

/// Company views the resumes it has downloaded
pub async fn list_mine_as_company(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<DownloadPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        download_repo::count_for_company(state.db.reader(), user.uid),
        download_repo::list_for_company(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(DownloadPage {
        total: total?,
        list: list?,
    })
}

/// Jobseeker views who has downloaded their resume
pub async fn list_mine_as_user(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<DownloadPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        download_repo::count_for_user(state.db.reader(), user.uid),
        download_repo::list_for_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(DownloadPage {
        total: total?,
        list: list?,
    })
}
