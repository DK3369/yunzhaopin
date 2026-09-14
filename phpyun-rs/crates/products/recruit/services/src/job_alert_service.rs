//! Public email job alerts (`phpyun_subscribe`). Distinct from member finder
//! (`phpyun_finder`) and from `/v1/mcenter/saved-searches`.

use crate::mail_service;
use phpyun_core::i18n::{current_lang, t, t_args};
use phpyun_core::{clock, rate_limit, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::category::repo as cat_repo;
use phpyun_models::saved_search::repo as ss_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::user::repo as user_repo;
use std::time::Duration;
use uuid::Uuid;

pub const CYCLES: [i32; 4] = [3, 7, 14, 26];

#[derive(Debug, Clone)]
pub struct SubscribeMeta {
    pub jionly: i32,
    pub cionly: i32,
    pub cycles: Vec<i32>,
}

pub struct SubscribeInput<'a> {
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub time: i32,
    pub email: &'a str,
    pub r#type: i32,
}

fn looks_like_email(s: &str) -> bool {
    let s = s.trim();
    let Some((u, d)) = s.split_once('@') else {
        return false;
    };
    !u.is_empty() && d.contains('.') && s.len() <= 100 && s.bytes().all(|b| b.is_ascii())
}

fn jobclass_csv(job1: i32, job1_son: i32, job_post: i32) -> String {
    [job1, job1_son, job_post]
        .into_iter()
        .filter(|n| *n > 0)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

async fn only_one_level(state: &AppState, kind: &str) -> bool {
    match cat_repo::list_all(state.db.reader(), kind).await {
        Ok(list) => !list.iter().any(|c| c.parent_id > 0),
        Err(_) => false,
    }
}

pub async fn meta(state: &AppState) -> AppResult<SubscribeMeta> {
    let jionly = i32::from(only_one_level(state, "job").await);
    let cionly = i32::from(only_one_level(state, "city").await);
    Ok(SubscribeMeta {
        jionly,
        cionly,
        cycles: CYCLES.to_vec(),
    })
}

pub async fn create(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    input: SubscribeInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let rl = match user {
        Some(u) => format!("rl:subscribe:uid:{}", u.uid),
        None => format!("rl:subscribe:ip:{client_ip}"),
    };
    rate_limit::check_and_incr(
        &state.redis,
        &rl,
        rate_limit::LimitRule {
            max: 8,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let flags = meta(state).await?;
    if flags.jionly == 1 {
        if input.job1 <= 0 {
            return Err(ApiError::param_invalid("admin_user_company_00023"));
        }
    } else if input.job1_son <= 0 {
        return Err(ApiError::param_invalid("admin_user_company_00023"));
    }
    if flags.cionly == 1 {
        if input.provinceid <= 0 {
            return Err(ApiError::param_invalid("wap_00901"));
        }
    } else if input.cityid <= 0 {
        return Err(ApiError::param_invalid("wap_00901"));
    }
    if input.minsalary <= 0 {
        return Err(ApiError::param_invalid("subscribe_salary"));
    }
    if input.maxsalary > 0 && input.maxsalary < input.minsalary {
        return Err(ApiError::param_invalid("subscribe_salary_max"));
    }
    if !CYCLES.contains(&input.time) {
        return Err(ApiError::param_invalid("subscribe_cycle"));
    }

    let mut email = input.email.trim().to_string();
    let uid = user.map(|u| u.uid).unwrap_or(0);
    if email.is_empty() {
        if let Some(u) = user {
            if let Some(m) = user_repo::find_by_uid(state.db.reader(), u.uid).await? {
                email = m.email.unwrap_or_default().trim().to_string();
            }
        }
    }
    if email.is_empty() || !looks_like_email(&email) {
        return Err(ApiError::param_invalid("subscribe_email"));
    }
    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:subscribe:email:{email}"),
        rate_limit::LimitRule {
            max: 8,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let now = clock::now_ts();
    let code: String = Uuid::now_v7().simple().to_string().chars().take(8).collect();
    let jobclass_id = jobclass_csv(input.job1, input.job1_son, input.job_post);
    let r#type = if input.r#type == 2 { 2 } else { 1 };
    let id = ss_repo::create_job_alert(
        state.db.pool(),
        ss_repo::JobAlertInsert {
            uid,
            email: &email,
            job1: input.job1,
            job1_son: input.job1_son,
            job_post: input.job_post,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            salary: 0,
            r#type,
            status: 1,
            code: &code,
            cycle_time: now + i64::from(input.time) * 86_400,
            time: input.time,
            minsalary: input.minsalary,
            maxsalary: input.maxsalary.max(0),
            jobclass_id: &jobclass_id,
            stime: 0,
            ctime: now,
        },
    )
    .await?;
    Ok(id)
}

pub async fn send_notice(
    state: &AppState,
    email: &str,
    client_ip: &str,
) -> AppResult<()> {
    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:subscribe-mail:ip:{client_ip}"),
        rate_limit::LimitRule {
            max: 3,
            window: Duration::from_secs(600),
        },
    )
    .await?;
    let email = email.trim();
    if !looks_like_email(email) {
        return Err(ApiError::param_invalid("subscribe_email"));
    }
    let email_on = setting_repo::find(state.db.reader(), "sy_email_set")
        .await?
        .map(|s| s.value.trim().to_string())
        .unwrap_or_default();
    if !email_on.is_empty() && email_on != "1" {
        return Err(ApiError::business("subscribe_mail_fail"));
    }
    let lang = current_lang();
    let name = setting_repo::find(state.db.reader(), "sy_webname")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    mail_service::send_text(
        state,
        email,
        &t_args("errors.subscribe_mail_subject", lang, &[("site", name.as_str())]),
        &t("errors.subscribe_mail_body", lang),
    )
    .await
}
