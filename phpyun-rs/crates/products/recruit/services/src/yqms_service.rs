//! PHP `job.model.php::addYqmsInfo` + `comtc.model.php::invite_resume`.
//! Writes `phpyun_userid_msg` and consumes `invite_resume`. Does **not**
//! require an existing `apply_id`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::blacklist::repo as bl_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::interview_template::repo as tpl_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::userid_msg::repo as msg_repo;
use serde::Serialize;

use crate::company_vip_day_service::{self, VipDayAction};

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
    pub mappic: Option<&'a str>,
    pub save_yqmb: bool,
    pub ymid: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct YqmsResult {
    /// PHP `status`: 2 = need confirm pay, 3 = success.
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jifen: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integral: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro: Option<i32>,
}

fn today_start_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= today_start_ts(now)
}

fn parse_integral(raw: &str) -> i64 {
    raw.trim().parse::<f64>().map(|v| v as i64).unwrap_or(0)
}

async fn read_setting_i64(state: &AppState, key: &str) -> i64 {
    setting_repo::find_many(state.db.reader(), &[key])
        .await
        .ok()
        .and_then(|m| m.get(key).and_then(|s| s.trim().parse().ok()))
        .unwrap_or(0)
}

async fn read_setting_str(state: &AppState, key: &str) -> String {
    setting_repo::find_many(state.db.reader(), &[key])
        .await
        .ok()
        .and_then(|m| m.get(key).cloned())
        .unwrap_or_default()
}

fn need_pay_result(
    online: i32,
    price_yuan: i64,
    jifen: i64,
    integral: i64,
    proportion: i32,
    integral_mode: bool,
) -> YqmsResult {
    let msg_key = if integral_mode {
        "common_00697".to_string()
    } else {
        "common_00696".to_string()
    };
    YqmsResult {
        status: 2,
        id: None,
        msg_key: Some(msg_key),
        price: if integral_mode {
            None
        } else {
            Some(price_yuan as f64)
        },
        jifen: if integral_mode { Some(jifen) } else { None },
        integral: Some(integral),
        online: Some(online),
        pro: Some(proportion),
    }
}

fn parse_intertime(raw: &str) -> Result<i64, ApiError> {
    let s = raw.trim();
    if s.is_empty() {
        return Err(ApiError::business("member_com_00681"));
    }
    if let Ok(ts) = s.parse::<i64>() {
        if ts <= 0 {
            return Err(ApiError::business("member_com_00681"));
        }
        return Ok(ts);
    }
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M",
    ];
    for fmt in formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Ok(dt.and_utc().timestamp());
        }
    }
    Err(ApiError::business("member_com_00681"))
}

async fn maybe_save_template(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &YqmsInput<'_>,
    jobname: &str,
    inter_ts: i64,
) -> AppResult<()> {
    if !input.save_yqmb {
        return Ok(());
    }
    let now = clock::now_ts();
    let tpl_name = format!("{jobname}admin_00709");
    if input.ymid > 0 {
        let _ = tpl_repo::update(
            state.db.pool(),
            input.ymid,
            user.uid,
            tpl_repo::TplUpdate {
                name: Some(&tpl_name),
                content: Some(input.content),
                address: Some(input.address),
                linkman: Some(input.linkman),
                linktel: Some(input.linktel),
                intertime: Some(inter_ts),
                status: None,
            },
            now,
        )
        .await?;
    } else {
        let used = tpl_repo::count_by_uid(state.db.reader(), user.uid).await?;
        if used < 10 {
            let _ = tpl_repo::create(
                state.db.pool(),
                tpl_repo::TplCreate {
                    uid: user.uid,
                    name: &tpl_name,
                    content: input.content,
                    address: input.address,
                    linkman: input.linkman,
                    linktel: input.linktel,
                    intertime: inter_ts,
                },
                now,
            )
            .await?;
        }
    }
    Ok(())
}

async fn do_insert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &YqmsInput<'_>,
    company_name: &str,
    jobname: &str,
    inter_ts: i64,
    client_ip: &str,
) -> AppResult<u64> {
    maybe_save_template(state, user, input, jobname, inter_ts).await?;
    let now = clock::now_ts();
    let id = msg_repo::insert(
        state.db.pool(),
        msg_repo::UseridMsgCreate {
            uid: input.seeker_uid,
            title: "wap_com_00046",
            content: input.content,
            fid: user.uid,
            fname: company_name,
            datetime: now,
            address: input.address,
            intertime: input.intertime,
            linkman: input.linkman,
            linktel: input.linktel,
            jobid: input.job_id,
            jobname,
            did: u64::from(user.did),
            x: input.longitude,
            y: input.latitude,
            mappic: input.mappic,
        },
    )
    .await?;
    let _ = apply_repo::mark_invited_by_seeker(
        state.db.pool(),
        user.uid,
        input.seeker_uid,
        now,
    )
    .await;
    let _ = audit::emit(
        state,
        AuditEvent::new("yqms.create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", input.seeker_uid)),
    )
    .await;
    Ok(id)
}

pub async fn create_from_resume(
    state: &AppState,
    user: &AuthenticatedUser,
    input: YqmsInput<'_>,
    confirm: bool,
    client_ip: &str,
) -> AppResult<YqmsResult> {
    user.require_employer()?;
    if input.seeker_uid == 0 || input.job_id == 0 {
        return Err(ApiError::param_invalid("bad_id"));
    }
    if input.linktel.trim().is_empty() {
        return Err(ApiError::business("common_06291"));
    }
    if input.address.trim().is_empty() {
        return Err(ApiError::business("member_com_00680"));
    }

    let inter_ts = parse_intertime(input.intertime)?;
    let now = clock::now_ts();
    if inter_ts <= now {
        return Err(ApiError::business("common_00752"));
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

    let statis = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("need_buy_invite"))?;
    if !is_vip(statis.vip_etime, now) {
        return Err(ApiError::business("need_buy_invite"));
    }

    let online = read_setting_i64(state, "com_integral_online").await as i32;
    let proportion = read_setting_i64(state, "integral_proportion").await as i32;
    let single_can = read_setting_str(state, "com_single_can").await;
    let integral_interview = read_setting_i64(state, "integral_interview").await;
    let server_open = single_can.split(',').any(|s| s.trim() == "invite");
    let integral_mode = online == 3;
    let com_integral = parse_integral(&statis.integral);
    let fname = company.name.clone().unwrap_or_default();
    let jobname = job.name.clone();

    let finish = |id: u64| YqmsResult {
        status: 3,
        id: Some(id),
        msg_key: None,
        price: None,
        jifen: None,
        integral: None,
        online: None,
        pro: None,
    };

    if statis.rating_type == 1 {
        if statis.invite_resume > 0 {
            if !statis_repo::try_consume_invite_resume(state.db.pool(), user.uid).await? {
                return Err(ApiError::business("need_buy_invite"));
            }
            let id = do_insert(state, user, &input, &fname, &jobname, inter_ts, client_ip).await?;
            return Ok(finish(id));
        }

        // Package exhausted — single purchase or free when integral_interview == 0
        if integral_interview == 0 && server_open {
            let id = do_insert(state, user, &input, &fname, &jobname, inter_ts, client_ip).await?;
            return Ok(finish(id));
        }

        if online == 4 || !server_open {
            return Err(ApiError::business("need_buy_invite"));
        }

        let jifen = if integral_mode {
            integral_interview.saturating_mul(proportion as i64)
        } else {
            integral_interview
        };
        let price_yuan = integral_interview.max(0);

        if !confirm {
            return Ok(need_pay_result(
                online,
                price_yuan,
                jifen,
                com_integral,
                proportion,
                integral_mode,
            ));
        }

        if integral_mode {
            if com_integral < jifen {
                return Err(ApiError::business("integral_insufficient"));
            }
            if statis_repo::try_deduct_integral(state.db.pool(), user.uid, jifen).await? == 0 {
                return Err(ApiError::business("integral_insufficient"));
            }
        }

        let id = do_insert(state, user, &input, &fname, &jobname, inter_ts, client_ip).await?;
        return Ok(finish(id));
    }

    if statis.rating_type == 2 {
        company_vip_day_service::check(state, VipDayAction::Interview, user.uid).await?;
        let id = do_insert(state, user, &input, &fname, &jobname, inter_ts, client_ip).await?;
        return Ok(finish(id));
    }

    Err(ApiError::business("need_buy_invite"))
}
