//! Resume download flow — company unlocks contact + counters.
//!
//! Aligned with PHPYun `downresume.model.php::downResume()`:
//! company audit, blacklist, already-unlocked, free_look for applicants,
//! package `down_resume` decrement, time-VIP daily cap, integral/cash single
//! purchase when quota exhausted, and contact payload on success.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::utils::mask_contact;
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::message::{entity as msg_entity, repo as message_repo};
use phpyun_models::resume::entity::Resume;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume_download::{entity::ResumeDownload, repo as download_repo};
use phpyun_models::site_setting::repo as setting_repo;
use serde::Serialize;

use crate::company_vip_day_service::{self, VipDayAction};

pub struct DownloadPage {
    pub list: Vec<ResumeDownload>,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadResult {
    /// PHP `status`: 2 = need confirm pay, 3 = success / already unlocked.
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waphtml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvusertel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f: Option<i32>,
}

fn today_start_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= today_start_ts(now)
}

fn has_contact(r: &Resume) -> bool {
    let tel = r.telphone.as_deref().unwrap_or("").trim();
    let home = r.telhome.as_deref().unwrap_or("").trim();
    let email = r.email.as_deref().unwrap_or("").trim();
    !tel.is_empty() || !home.is_empty() || !email.is_empty()
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

/// PHP `resume.model.php::setDayprice`.
async fn resume_day_price(state: &AppState, eid: u64, integral: bool) -> AppResult<i64> {
    let lastupdate = if eid > 0 {
        phpyun_models::resume::expect::find_by_id(state.db.reader(), eid)
            .await?
            .map(|e| e.lastupdate)
            .unwrap_or(0)
    } else {
        0
    };
    let now = clock::now_ts();
    let rday = if lastupdate > 0 {
        ((now - lastupdate).max(0) as f64) / 86_400.0
    } else {
        0.0
    };

    let dayprice_cfg = read_setting_str(state, "integral_down_resume_dayprice").await;
    let mut tier_price: Option<i64> = None;
    if !dayprice_cfg.trim().is_empty() {
        let mut tiers: Vec<(f64, i64)> = Vec::new();
        for part in dayprice_cfg.split(':') {
            if let Some((d, p)) = part.split_once('_') {
                if let (Ok(days), Ok(price)) = (d.parse::<f64>(), p.parse::<i64>()) {
                    tiers.push((days, price));
                }
            }
        }
        tiers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        for (days, price) in &tiers {
            if rday <= *days {
                tier_price = Some(*price);
                break;
            }
        }
        if tier_price.is_none() {
            tier_price = tiers.last().map(|(_, p)| *p);
        }
    }

    let base = read_setting_i64(state, "integral_down_resume").await;
    let proportion = read_setting_i64(state, "integral_proportion").await.max(1);
    let yuan = tier_price.unwrap_or(base).max(0);
    Ok(if integral {
        yuan.saturating_mul(proportion)
    } else {
        yuan
    })
}

fn build_contact_html(state: &AppState, r: &Resume, site_name: &str) -> (String, String) {
    let tel = r.telphone.as_deref().unwrap_or("").trim();
    let email = r.email.as_deref().unwrap_or("").trim();
    let mut pc = format!(
        "<div class=\"tcktouch_box\"><div class=\"tcktouch_box_tip\">联系我时请说是在{site_name}上看到的</div>"
    );
    if !tel.is_empty() {
        pc.push_str(&format!(
            "<div class=\"tcktouch_box_p\">手机：<span class=\"tcktouch_box_p_sj\">{tel}</span></div>"
        ));
    }
    if !email.is_empty() {
        pc.push_str(&format!("<div class=\"tcktouch_box_p\">邮箱：{email}</div>"));
    }
    pc.push_str("</div>");

    let mut wap = String::new();
    if !tel.is_empty() {
        wap.push_str(&format!("<a href=\"tel:{tel}\">{tel}</a>"));
    }
    if !email.is_empty() {
        if !wap.is_empty() {
            wap.push_str("<br/>");
        }
        wap.push_str(email);
    }
    let _ = state;
    (pc, wap)
}

async fn contact_payload(
    state: &AppState,
    r: &Resume,
) -> AppResult<(Option<String>, Option<String>, Option<String>, Option<String>)> {
    let site_name = read_setting_str(state, "sy_webname").await;
    let site_name = if site_name.trim().is_empty() {
        "本站".to_string()
    } else {
        site_name
    };
    let (html, waphtml) = build_contact_html(state, r, &site_name);
    let tel = r.telphone.as_deref().unwrap_or("").trim();
    let private_phone = if tel.is_empty() {
        None
    } else {
        Some(tel.to_string())
    };
    let prvusertel = if tel.is_empty() {
        None
    } else {
        Some(mask_contact(tel))
    };
    Ok((Some(html), Some(waphtml), private_phone, prvusertel))
}

async fn success_result(state: &AppState, r: &Resume, f: Option<i32>) -> AppResult<DownloadResult> {
    let (html, waphtml, private_phone, prvusertel) = contact_payload(state, r).await?;
    Ok(DownloadResult {
        status: 3,
        html,
        waphtml,
        private_phone,
        prvusertel,
        msg: None,
        msg_key: None,
        price: None,
        jifen: None,
        integral: None,
        online: None,
        pro: None,
        f,
    })
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
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: target_uid,
            recipient_usertype: 1,
            category: "download",
            title: "notifications.resume.downloaded_title",
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

async fn record_and_finish(
    state: &AppState,
    user: &AuthenticatedUser,
    r: &Resume,
    eid: u64,
    now: i64,
    free: bool,
    client_ip: &str,
    f: i32,
) -> AppResult<DownloadResult> {
    if free {
        let _ = download_repo::record_freedown(state.db.pool(), user.uid, r.uid, eid, now).await?;
        let _ = audit::emit(
            state,
            AuditEvent::new("resume.freedown", Actor::uid(user.uid).with_ip(client_ip))
                .target(format!("uid:{}", r.uid)),
        )
        .await;
    } else {
        let _ = download_repo::record(state.db.pool(), user.uid, r.uid, eid, now).await?;
        let _ = audit::emit(
            state,
            AuditEvent::new("resume.download", Actor::uid(user.uid).with_ip(client_ip))
                .target(format!("uid:{}", r.uid)),
        )
        .await;
    }
    notify_first(state, user.uid, r.uid, now, true).await?;
    success_result(state, r, Some(f)).await
}

fn need_pay_result(
    online: i32,
    price_yuan: i64,
    jifen: i64,
    integral: i64,
    proportion: i32,
    integral_mode: bool,
) -> DownloadResult {
    let (msg_key, price, jifen_out) = if integral_mode {
        ("common_00697".to_string(), None, Some(jifen))
    } else {
        ("common_00696".to_string(), Some(price_yuan as f64), None)
    };
    DownloadResult {
        status: 2,
        html: None,
        waphtml: None,
        private_phone: None,
        prvusertel: None,
        msg: None,
        msg_key: Some(msg_key),
        price,
        jifen: jifen_out,
        integral: Some(integral),
        online: Some(online),
        pro: Some(proportion),
        f: None,
    }
}

/// Company downloads a resume.
pub async fn download(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    eid: Option<u64>,
    confirm: bool,
    client_ip: &str,
) -> AppResult<DownloadResult> {
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

    let now = clock::now_ts();
    let today = today_start_ts(now);
    let default_eid = u64::try_from(r.def_job.max(0)).unwrap_or(0);
    let eid = match eid.filter(|e| *e > 0) {
        Some(e) => {
            let exp = phpyun_models::resume::expect::find_by_id(state.db.reader(), e)
                .await?
                .ok_or_else(|| ApiError::param_invalid("eid"))?;
            if exp.uid != target_uid {
                return Err(ApiError::param_invalid("eid"));
            }
            e
        }
        None => default_eid,
    };

    let already = download_repo::already_downloaded_eid(state.db.reader(), user.uid, eid)
        .await
        .unwrap_or(false);
    let already_free = download_repo::already_freedown_eid(state.db.reader(), user.uid, eid)
        .await
        .unwrap_or(false);
    if already {
        return success_result(state, &r, Some(2)).await;
    }
    if already_free {
        return success_result(state, &r, Some(1)).await;
    }

    let statis = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or(ApiError::business("need_buy_down_resume"))?;

    let online = read_setting_i64(state, "com_integral_online").await as i32;
    let proportion = read_setting_i64(state, "integral_proportion").await as i32;
    let only_price = read_setting_str(state, "sy_only_price").await;
    let single_can = read_setting_str(state, "com_single_can").await;
    let integral_mode =
        online == 3 && !only_price.split(',').any(|s| s.trim() == "downresume");
    let server_open = single_can.split(',').any(|s| s.trim() == "downresume");
    let com_integral = parse_integral(&statis.integral);

    let lietou = read_setting_i64(state, "com_lietou_job").await;
    if lietou == 1 && is_vip(statis.vip_etime, now) {
        let n = phpyun_models::job::repo::count_online_by_uid(state.db.reader(), user.uid)
            .await
            .unwrap_or(0);
        if n == 0 {
            return Err(ApiError::business("need_post_job"));
        }
    }

    let price_yuan_early = resume_day_price(state, eid, false).await?;
    if price_yuan_early == 0 && statis.down_resume == 0 {
        return record_and_finish(state, user, &r, eid, now, false, client_ip, 2).await;
    }

    if is_vip(statis.vip_etime, now) {
        let applied = phpyun_models::apply::repo::exists_by_com_eid(
            state.db.reader(),
            user.uid,
            eid,
        )
        .await
        .unwrap_or(false);
        let freelook = statis_repo::freelook_num(state.db.reader(), statis.rating)
            .await
            .unwrap_or(0);
        let used_free =
            download_repo::count_today_freedown(state.db.reader(), user.uid, today).await?;
        if applied && (used_free as i32) < freelook {
            return record_and_finish(state, user, &r, eid, now, true, client_ip, 1).await;
        }

        if statis.rating_type == 1 {
            if statis.down_resume > 0 {
                if !statis_repo::try_consume_down_resume(state.db.pool(), user.uid).await? {
                    return Err(ApiError::business("need_buy_down_resume"));
                }
                return record_and_finish(state, user, &r, eid, now, false, client_ip, 2).await;
            }
            // Package exhausted — single purchase
            if online == 4 || !server_open {
                return Err(ApiError::business("need_buy_down_resume"));
            }
            let price_yuan = resume_day_price(state, eid, false).await?;
            let jifen = resume_day_price(state, eid, true).await?;
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
            return record_and_finish(state, user, &r, eid, now, false, client_ip, 2).await;
        }

        if statis.rating_type == 2 {
            company_vip_day_service::check(state, VipDayAction::Resume, user.uid).await?;
            return record_and_finish(state, user, &r, eid, now, false, client_ip, 2).await;
        }

        return Err(ApiError::business("need_buy_down_resume"));
    }

    // Expired VIP — single purchase only
    if online == 4 || !server_open {
        return Err(ApiError::business("need_buy_down_resume"));
    }
    let price_yuan = resume_day_price(state, eid, false).await?;
    let jifen = resume_day_price(state, eid, true).await?;
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
    record_and_finish(state, user, &r, eid, now, false, client_ip, 2).await
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
