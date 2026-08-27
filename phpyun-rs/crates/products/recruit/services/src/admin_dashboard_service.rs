//! Admin dashboard aggregation: pending counts for each review queue plus the last 24h of registrations/applications/postings overview.

use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::admin_msg::repo as admin_msg_repo;
use phpyun_models::admin_msg::repo::AdminMsgNum;
use phpyun_models::company_cert::repo as cert_repo;
use phpyun_models::feedback::repo as feedback_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::report::repo as report_repo;
use phpyun_models::stats::repo as stats_repo;
use phpyun_models::user::entity::Member;

#[derive(Debug, Default)]
pub struct AdminOverview {
    pub pending_company_certs: u64,
    pub pending_jobs: u64,
    pub pending_reports: u64,
    pub pending_feedback: u64,
    pub total_users: u64, // best-effort from admin_count
    pub active_companies: u64,
    pub active_jobs: u64,
    pub active_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

fn today_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

pub async fn overview(state: &AppState, admin: &AuthenticatedUser) -> AppResult<AdminOverview> {
    admin.require_admin()?;
    let db = state.db.reader();
    let today = today_ts(clock::now_ts());

    // Parallel all counts
    let (
        certs,
        jobs_pending,
        reports_pending,
        fb_pending,
        active_jobs,
        active_coms,
        active_res,
        new_j,
        new_r,
    ) = tokio::join!(
        cert_repo::count_pending(db),
        job_repo::admin_count(db, Some(0)),
        report_repo::count_by_status(db, Some(0)),
        feedback_repo::count_by_status(db, Some(0)),
        stats_repo::count_active_jobs(db),
        stats_repo::count_active_companies(db),
        stats_repo::count_active_resumes(db),
        stats_repo::count_jobs_since(db, today),
        stats_repo::count_resumes_since(db, today),
    );

    // total_users: approximate via admin_list_count (no filter)
    let total_users = phpyun_models::user::repo::admin_count(
        db,
        &phpyun_models::user::repo::AdminUserFilter {
            keyword: None,
            usertype: None,
            status: None,
        },
    )
    .await
    .unwrap_or(0);

    Ok(AdminOverview {
        pending_company_certs: certs.unwrap_or(0),
        pending_jobs: jobs_pending.unwrap_or(0),
        pending_reports: reports_pending.unwrap_or(0),
        pending_feedback: fb_pending.unwrap_or(0),
        total_users,
        active_companies: active_coms.unwrap_or(0),
        active_jobs: active_jobs.unwrap_or(0),
        active_resumes: active_res.unwrap_or(0),
        today_new_jobs: new_j.unwrap_or(0),
        today_new_resumes: new_r.unwrap_or(0),
    })
}

/// Most recent signups (top N)
pub async fn recent_signups(
    state: &AppState,
    admin: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<Member>> {
    admin.require_admin()?;
    let limit = limit.clamp(1, 50);
    Ok(phpyun_models::user::repo::admin_list(
        state.db.reader(),
        &phpyun_models::user::repo::AdminUserFilter {
            keyword: None,
            usertype: None,
            status: None,
        },
        0,
        limit,
    )
    .await?)
}

pub async fn msg_num(state: &AppState, admin: &AuthenticatedUser) -> AppResult<AdminMsgNum> {
    admin.require_admin()?;
    Ok(admin_msg_repo::load(state.db.reader(), clock::now_ts()).await)
}

fn local_midnight(date: chrono::NaiveDate) -> i64 {
    date.and_hms_opt(0, 0, 0)
        .and_then(|dt| dt.and_local_timezone(chrono::Local).single())
        .map(|dt| dt.timestamp())
        .unwrap_or(0)
}

fn start_of_today(_now: i64) -> i64 {
    local_midnight(chrono::Local::now().date_naive())
}

fn start_of_month(_now: i64) -> i64 {
    use chrono::Datelike;
    let d = chrono::Local::now().date_naive();
    local_midnight(d.with_day(1).unwrap_or(d))
}

pub async fn home_data(
    state: &AppState,
    admin: &AuthenticatedUser,
) -> AppResult<serde_json::Value> {
    admin.require_admin()?;
    let row = phpyun_models::admin_rbac::repo::find_by_uid(state.db.reader(), admin.uid)
        .await?
        .ok_or_else(phpyun_core::ApiError::unauth)?;
    Ok(serde_json::json!({
        "index_lookstatistc": row.index_lookstatistc,
        "base": "",
        "topinfo": {
            "indextip_show": false,
            "mruser": 0,
            "msg_setting": 1,
            "pyu": false,
            "dirname": "",
            "updateUrl": ""
        },
        "sysinfo": {
            "version": "phpyun-rs",
            "soft": "axum",
            "kongjian": 0,
            "phpbanben": "-",
            "banben": "mysql",
            "yonghu": "",
            "server": ""
        }
    }))
}

pub async fn ajax_right(
    _state: &AppState,
    admin: &AuthenticatedUser,
) -> AppResult<serde_json::Value> {
    admin.require_admin()?;
    Ok(serde_json::json!({ "msgnum": 0 }))
}

#[derive(Debug, Default)]
pub struct AjaxStatisQuery {
    pub r#type: Option<String>,
    pub area: Option<String>,
}

pub async fn ajax_statis(
    state: &AppState,
    admin: &AuthenticatedUser,
    q: AjaxStatisQuery,
) -> AppResult<serde_json::Value> {
    admin.require_admin()?;
    let row = phpyun_models::admin_rbac::repo::find_by_uid(state.db.reader(), admin.uid)
        .await?
        .ok_or_else(phpyun_core::ApiError::unauth)?;
    if row.index_lookstatistc != 2 {
        return Ok(serde_json::json!({}));
    }
    let db = state.db.reader();
    let now = clock::now_ts();
    let today = start_of_today(now);
    let yesterday = today - 86_400;
    let month = start_of_month(now);

    if q.r#type.as_deref() == Some("month") && q.area.as_deref() == Some("member") {
        let month_member = stats_repo::members_since(db, month).await;
        let ommember = stats_repo::other_members_since(db, month).await;
        let user_mon = stats_repo::members_usertype_since(db, 1, month).await;
        let com_mon = stats_repo::companies_pid0_since(db, month).await;
        return Ok(serde_json::json!({
            "monthMemberNum": month_member,
            "ommemberNum": ommember,
            "userNumMon": user_mon,
            "companyNumMon": com_mon
        }));
    }
    if q.r#type.as_deref() == Some("month") && q.area.as_deref() == Some("money") {
        return Ok(serde_json::json!({
            "monthMoneyTotal": stats_repo::order_sum_since(db, month, 0).await,
            "monthMoneyVip": stats_repo::order_sum_since(db, month, 1).await,
            "monthMoneyService": stats_repo::order_sum_since(db, month, 5).await,
            "monthMoneyIntegral": stats_repo::order_sum_since(db, month, -1).await
        }));
    }

    let member_num = stats_repo::members_since(db, today).await;
    let otmember = stats_repo::other_members_since(db, today).await;
    let user_num = stats_repo::members_usertype_since(db, 1, today).await;
    let company_num = stats_repo::companies_pid0_since(db, today).await;
    let resume_num = stats_repo::expects_since(db, today).await;
    let old_resume = stats_repo::expects_between(db, yesterday, today).await;
    let job_num = stats_repo::jobs_sdate_since(db, today).await;
    let old_job = stats_repo::jobs_sdate_between(db, yesterday, today).await;
    let useridjob = stats_repo::userid_job_since(db, today).await;
    let old_useridjob = stats_repo::userid_job_between(db, yesterday, today).await;
    let down = stats_repo::down_resume_since(db, today).await;
    let old_down = stats_repo::down_resume_between(db, yesterday, today).await;
    let free_down = stats_repo::free_down_since(db, today).await;
    let tel = stats_repo::tellog_since(db, today).await;
    let old_tel = stats_repo::tellog_between(db, yesterday, today).await;

    Ok(serde_json::json!({
        "memberNum": member_num,
        "otmemberNum": otmember,
        "userNum": user_num,
        "companyNum": company_num,
        "resumeNum": resume_num,
        "useridjobNum": useridjob,
        "olduseridjobNum": old_useridjob,
        "oldresumeNum": old_resume,
        "jobNum": job_num,
        "oldjobNum": old_job,
        "moneyTotal": stats_repo::order_sum_since(db, today, 0).await,
        "moneyVip": stats_repo::order_sum_since(db, today, 1).await,
        "moneyIntegral": stats_repo::order_sum_since(db, today, -1).await,
        "moneyService": stats_repo::order_sum_since(db, today, 5).await,
        "downresumeNum": down,
        "olddownresumeNum": old_down,
        "tellognum": tel,
        "oldtellognum": old_tel,
        "freedownnum": free_down,
        "monthMoneyTotal": 0,
        "monthMoneyVip": 0,
        "monthMoneyIntegral": 0,
        "monthMoneyService": 0,
        "monthMemberNum": 0,
        "ommemberNum": 0,
        "userNumMon": 0,
        "companyNumMon": 0,
        "msgnum": 0
    }))
}

#[derive(Debug, Default)]
pub struct MonthStatisQuery {
    pub sdate: Option<String>,
    pub edate: Option<String>,
}

fn parse_day(s: &str) -> Option<i64> {
    let parts: Vec<_> = s.split(['-', '/']).collect();
    if parts.len() < 2 {
        return None;
    }
    let y: i32 = parts[0].parse().ok()?;
    let m: u32 = parts[1].parse().ok()?;
    let d: u32 = if parts.len() >= 3 {
        parts[2].parse().ok()?
    } else {
        1
    };
    chrono::NaiveDate::from_ymd_opt(y, m, d).map(local_midnight)
}

pub async fn month_statis(
    state: &AppState,
    admin: &AuthenticatedUser,
    q: MonthStatisQuery,
) -> AppResult<serde_json::Value> {
    admin.require_admin()?;
    let row = phpyun_models::admin_rbac::repo::find_by_uid(state.db.reader(), admin.uid)
        .await?
        .ok_or_else(phpyun_core::ApiError::unauth)?;
    if row.index_lookstatistc != 2 {
        return Ok(serde_json::json!({}));
    }
    let db = state.db.reader();
    let now = clock::now_ts();
    let (start, end) = match (q.sdate.as_deref(), q.edate.as_deref()) {
        (Some(s), Some(e)) => {
            let st = parse_day(s).unwrap_or_else(|| start_of_month(now));
            let en = parse_day(e).unwrap_or(now) + 86_400 - 1;
            (st, en)
        }
        _ => (start_of_month(now), now),
    };
    let resume = stats_repo::expects_between(db, start, end + 1).await;
    let job = stats_repo::jobs_sdate_between(db, start, end + 1).await;
    let company = stats_repo::members_usertype_since(db, 2, start).await;
    let user = stats_repo::members_usertype_since(db, 1, start).await;
    let gg = stats_repo::adclick_since(db, start).await;
    let userjob = stats_repo::userid_job_since(db, start).await;
    let yqms = stats_repo::yqms_since(db, start).await;
    let down = stats_repo::down_resume_since(db, start).await + stats_repo::free_down_since(db, start).await;
    let wxbd = stats_repo::wx_bound_since(db, start).await;
    let wx_user = stats_repo::wx_bound(db, 1).await;
    let wx_com = stats_repo::wx_bound(db, 2).await;
    let user_all = stats_repo::members_usertype(db, 1).await.max(1);
    let com_all = stats_repo::members_usertype(db, 2).await.max(1);
    Ok(serde_json::json!({
        "resumeNumMon": resume,
        "jobNumMon": job,
        "companyNumMon": company,
        "userNumMon": user,
        "ggNumMon": gg,
        "userjobNumMon": userjob,
        "yqmsNumMon": yqms,
        "downreusmeNumMon": down,
        "wxbdNumMon": wxbd,
        "wxbduserNumMon": wx_user,
        "wxbdcomNumMon": wx_com,
        "userwx_percent": (100 * wx_user / user_all),
        "comwx_percent": (100 * wx_com / com_all)
    }))
}

/// PHP `index::tjl` / `tj` chart payload for homecenter.
pub async fn chart(
    state: &AppState,
    admin: &AuthenticatedUser,
    kind: &str,
    q: MonthStatisQuery,
) -> AppResult<serde_json::Value> {
    admin.require_admin()?;
    let row = phpyun_models::admin_rbac::repo::find_by_uid(state.db.reader(), admin.uid)
        .await?
        .ok_or_else(phpyun_core::ApiError::unauth)?;
    if row.index_lookstatistc != 2 {
        return Ok(serde_json::json!({ "list": [], "name": [] }));
    }
    let now = clock::now_ts();
    let (start, end) = match (q.sdate.as_deref(), q.edate.as_deref()) {
        (Some(s), Some(e)) => {
            let st = parse_day(s).unwrap_or_else(|| start_of_month(now));
            let en = parse_day(e).unwrap_or(now) + 86_400 - 1;
            (st, en)
        }
        _ => (start_of_month(now), now),
    };
    let db = state.db.reader();
    let (series, names): (Vec<&'static str>, Vec<&'static str>) = match kind {
        "getweb" => (
            vec![
                "SELECT DATE_FORMAT(FROM_UNIXTIME(reg_date), '%d') AS d, COUNT(*) FROM phpyun_member WHERE usertype = 1 AND reg_date >= ? AND reg_date <= ? GROUP BY d",
                "SELECT DATE_FORMAT(FROM_UNIXTIME(ctime), '%d') AS d, COUNT(*) FROM phpyun_login_log WHERE usertype = 1 AND ctime >= ? AND ctime <= ? GROUP BY d",
            ],
            vec!["admin_00073", "admin_tool_00223", "admin_index_00040", "admin_index_00041"],
        ),
        "comtj" => (
            vec![
                "SELECT DATE_FORMAT(FROM_UNIXTIME(reg_date), '%d') AS d, COUNT(*) FROM phpyun_member WHERE usertype = 2 AND reg_date >= ? AND reg_date <= ? GROUP BY d",
                "SELECT DATE_FORMAT(FROM_UNIXTIME(ctime), '%d') AS d, COUNT(*) FROM phpyun_login_log WHERE usertype = 2 AND ctime >= ? AND ctime <= ? GROUP BY d",
            ],
            vec!["admin_00074", "admin_user_00335", "admin_index_00042", "admin_index_00043"],
        ),
        "resumetj" => (
            vec![
                "SELECT DATE_FORMAT(FROM_UNIXTIME(ctime), '%d') AS d, COUNT(*) FROM phpyun_resume_expect WHERE ctime >= ? AND ctime <= ? GROUP BY d",
                "SELECT DATE_FORMAT(FROM_UNIXTIME(r_time), '%d') AS d, COUNT(*) FROM phpyun_resume_refresh_log WHERE r_time >= ? AND r_time <= ? GROUP BY d",
            ],
            vec!["admin_tool_00016", "admin_tool_00176", "admin_index_00045", "admin_index_00044"],
        ),
        "jobtj" => (
            vec![
                "SELECT DATE_FORMAT(FROM_UNIXTIME(sdate), '%d') AS d, COUNT(*) FROM phpyun_company_job WHERE sdate >= ? AND sdate <= ? GROUP BY d",
                "SELECT DATE_FORMAT(FROM_UNIXTIME(r_time), '%d') AS d, COUNT(*) FROM phpyun_job_refresh_log WHERE r_time >= ? AND r_time <= ? GROUP BY d",
            ],
            vec!["admin_index_00074", "wap_com_00045", "admin_index_00047", "admin_index_00046"],
        ),
        "ujobtj" => (
            vec!["SELECT DATE_FORMAT(FROM_UNIXTIME(datetime), '%d') AS d, COUNT(*) FROM phpyun_userid_job WHERE datetime >= ? AND datetime <= ? GROUP BY d"],
            vec!["admin_tool_00109", "admin_index_00008"],
        ),
        "yqmstj" => (
            vec!["SELECT DATE_FORMAT(FROM_UNIXTIME(datetime), '%d') AS d, COUNT(*) FROM phpyun_userid_msg WHERE datetime >= ? AND datetime <= ? GROUP BY d"],
            vec!["admin_tool_00114", "admin_index_00009"],
        ),
        "downresumetj" => (
            vec!["SELECT DATE_FORMAT(FROM_UNIXTIME(downtime), '%d') AS d, COUNT(*) FROM phpyun_down_resume WHERE downtime >= ? AND downtime <= ? GROUP BY d"],
            vec!["admin_tool_00108", "admin_index_00007"],
        ),
        "adtj" => (
            vec!["SELECT DATE_FORMAT(FROM_UNIXTIME(addtime), '%d') AS d, COUNT(*) FROM phpyun_adclick WHERE addtime >= ? AND addtime <= ? GROUP BY d"],
            vec!["admin_tool_00099", "admin_index_00006"],
        ),
        "wxbdtj" => (
            vec![
                "SELECT DATE_FORMAT(FROM_UNIXTIME(wxbindtime), '%d') AS d, COUNT(*) FROM phpyun_member WHERE usertype = 1 AND wxid IS NOT NULL AND wxid <> '' AND wxbindtime >= ? AND wxbindtime <= ? GROUP BY d",
                "SELECT DATE_FORMAT(FROM_UNIXTIME(wxbindtime), '%d') AS d, COUNT(*) FROM phpyun_member WHERE usertype = 2 AND wxid IS NOT NULL AND wxid <> '' AND wxbindtime >= ? AND wxbindtime <= ? GROUP BY d",
            ],
            vec!["admin_index_00048", "admin_index_00049", "admin_index_00004", "admin_index_00005"],
        ),
        _ => (Vec::new(), Vec::new()),
    };
    let mut list = Vec::new();
    for sql in series {
        let rows = stats_repo::daily_day_counts(db, sql, start, end).await;
        list.push(serde_json::json!({ "list": fill_day_map(start, end, rows) }));
    }
    Ok(serde_json::json!({ "list": list, "name": names }))
}

fn fill_day_map(
    start: i64,
    end: i64,
    rows: Vec<(String, i64)>,
) -> serde_json::Map<String, serde_json::Value> {
    use chrono::{Datelike, Duration, TimeZone};
    let counts: std::collections::HashMap<String, i64> = rows.into_iter().collect();
    let start_d = chrono::Local
        .timestamp_opt(start, 0)
        .single()
        .map(|t| t.date_naive())
        .unwrap_or_else(|| chrono::Local::now().date_naive());
    let end_d = chrono::Local
        .timestamp_opt(end, 0)
        .single()
        .map(|t| t.date_naive())
        .unwrap_or(start_d);
    let mut out = serde_json::Map::new();
    let mut i = 0u32;
    let mut d = start_d;
    while d <= end_d {
        let key = format!("{i}");
        let td = format!("{:02}", d.day());
        let cnt = *counts.get(&td).unwrap_or(&0);
        out.insert(key, serde_json::json!({ "td": td, "cnt": cnt }));
        i += 1;
        match d.checked_add_signed(Duration::days(1)) {
            Some(n) => d = n,
            None => break,
        }
        if i > 40 {
            break;
        }
    }
    out
}

/// PHP `getCacheData` / `common/cache` payload (cascader trees + search filters).
pub fn php_cache_payload(
    job_nodes: &[(u64, u64, String)],
    city_nodes: &[(u64, u64, String)],
    edu: &[(i32, String)],
    exp: &[(i32, String)],
) -> serde_json::Value {
    fn map_pairs(rows: &[(i32, String)]) -> serde_json::Map<String, serde_json::Value> {
        let mut m = serde_json::Map::new();
        for (id, name) in rows {
            m.insert(id.to_string(), serde_json::Value::String(name.clone()));
        }
        m
    }
    let job_types = cascader_nodes(job_nodes);
    let city_types = cascader_nodes(city_nodes);
    let mut comclass_name = map_pairs(edu);
    comclass_name.extend(map_pairs(exp));
    serde_json::json!({
        "cache": {},
        "comdata": {
            "job_edu": edu.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            "job_exp": exp.iter().map(|(id, _)| *id).collect::<Vec<_>>()
        },
        "comclass_name": comclass_name,
        "job_types": job_types,
        "city_types": city_types,
        "jionly": if job_types.is_empty() { 1 } else { 0 },
        "curr_time": clock::now_ts(),
        "search_list": {
            "state": { "name": "wap_com_00406", "value": { "1": "wap_user_00165", "4": "wap_user_00166", "3": "wap_user_00167", "2": "admin_user_00138" } },
            "status": { "name": "member_user_00178", "value": { "1": "wap_com_00242", "2": "wap_com_00243" } },
            "jtype": { "name": "wap_00516", "value": { "urgent": "member_com_00326", "xuanshang": "member_com_00327", "rec": "member_com_00324" } },
            "exp": { "name": "wap_user_00240", "value": map_pairs(exp) },
            "edu": { "name": "wap_com_00283", "value": map_pairs(edu) },
            "source": { "name": "admin_yunying_00139", "value": {} },
            "rating": { "name": "admin_user_company_00018", "value": {} },
            "openautho": { "name": "admin_00749", "value": { "1": "wap_js_00098", "2": "admin_user_company_00304" } },
            "is_depower": { "name": "admin_user_00068", "value": { "1": "是", "2": "否" } }
        },
        "hbNum": 0,
        "hb_isopen": "0"
    })
}

fn cascader_nodes(nodes: &[(u64, u64, String)]) -> Vec<serde_json::Value> {
    use std::collections::HashMap;
    let mut by_parent: HashMap<u64, Vec<&(u64, u64, String)>> = HashMap::new();
    for n in nodes {
        by_parent.entry(n.1).or_default().push(n);
    }
    fn rec(
        pid: u64,
        by_parent: &HashMap<u64, Vec<&(u64, u64, String)>>,
    ) -> Vec<serde_json::Value> {
        let Some(kids) = by_parent.get(&pid) else {
            return Vec::new();
        };
        kids.iter()
            .map(|n| {
                let children = rec(n.0, by_parent);
                let mut o = serde_json::json!({ "value": n.0, "label": n.2 });
                if !children.is_empty() {
                    o["children"] = serde_json::Value::Array(children);
                }
                o
            })
            .collect()
    }
    rec(0, &by_parent)
}
