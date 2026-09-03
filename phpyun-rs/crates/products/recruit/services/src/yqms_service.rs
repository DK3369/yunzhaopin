//! PHP `job.model.php::addYqmsInfo` + `comtc.model.php::invite_resume`.
//! Writes `phpyun_userid_msg` and consumes `invite_resume`. Does **not**
//! require an existing `apply_id`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::blacklist::repo as bl_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::userid_msg::repo as msg_repo;

pub struct YqmsInput<'a> {
    pub seeker_uid: u64,
    pub job_id: u64,
    pub content: &'a str,
    pub address: &'a str,
    pub intertime: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub longitude: &'a str,
    pub latitude: &'a str,
}

fn today_start_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= today_start_ts(now)
}

pub async fn create_from_resume(
    state: &AppState,
    user: &AuthenticatedUser,
    input: YqmsInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    if input.seeker_uid == 0 || input.job_id == 0 {
        return Err(ApiError::param_invalid("bad_id"));
    }
    if input.intertime.trim().is_empty() {
        return Err(ApiError::business("intertime_required"));
    }
    if input.linktel.trim().is_empty() {
        return Err(ApiError::business("linktel_required"));
    }
    if input.address.trim().is_empty() {
        return Err(ApiError::business("address_required"));
    }

    let company = company_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("company_not_found"))?;
    if company.r_status != 1 {
        return Err(ApiError::business("company_unapproved"));
    }
    if bl_repo::is_blocked(state.db.reader(), user.uid, input.seeker_uid).await? {
        return Err(ApiError::business("blacklisted"));
    }
    if apply_repo::count_userid_msg_by_fid_uid(state.db.reader(), user.uid, input.seeker_uid)
        .await?
        > 0
    {
        return Err(ApiError::business("already_invited"));
    }

    let job = job_repo::find_by_id(state.db.reader(), input.job_id)
        .await?
        .ok_or(ApiError::business("job_not_found"))?;
    if job.uid != user.uid {
        return Err(ApiError::forbidden());
    }

    let now = clock::now_ts();
    let today = today_start_ts(now);
    let statis = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("need_buy_invite"))?;
    if !is_vip(statis.vip_etime, now) {
        return Err(ApiError::business("need_buy_invite"));
    }
    if statis.rating_type == 1 {
        if !statis_repo::try_consume_invite_resume(state.db.pool(), user.uid).await? {
            return Err(ApiError::business("need_buy_invite"));
        }
    } else if statis.rating_type == 2 {
        if statis.invite_resume <= 0 {
            return Err(ApiError::business("vip_day_limit"));
        }
        let today_n = apply_repo::count_userid_msg_today(state.db.reader(), user.uid, today).await?;
        if today_n >= statis.invite_resume as u64 {
            return Err(ApiError::business("vip_day_limit"));
        }
    } else {
        return Err(ApiError::business("need_buy_invite"));
    }

    let fname = company.name.clone().unwrap_or_default();
    let jobname = job.name.clone();
    let id = msg_repo::insert(
        state.db.pool(),
        msg_repo::UseridMsgCreate {
            uid: input.seeker_uid,
            title: "面试邀请",
            content: input.content,
            fid: user.uid,
            fname: &fname,
            datetime: now,
            address: input.address,
            intertime: input.intertime,
            linkman: input.linkman,
            linktel: input.linktel,
            jobid: input.job_id,
            jobname: &jobname,
            did: u64::from(user.did),
            x: input.longitude,
            y: input.latitude,
        },
    )
    .await?;
    let _ = apply_repo::mark_invited_by_seeker(state.db.pool(), user.uid, input.seeker_uid, now)
        .await;
    let _ = audit::emit(
        state,
        AuditEvent::new("yqms.create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", input.seeker_uid)),
    )
    .await;
    Ok(id)
}
