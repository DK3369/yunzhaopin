//! Recruiter stats / tongji (PHP `member/com/zhaopin` + `tongji` + WAP week).

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::com_stats::repo::{self, BucketCount, BucketFmt, ClassCount};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::part::repo as part_repo;
use phpyun_models::site_setting::repo as setting_repo;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct KvNum {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TalentDims {
    pub exp: Vec<KvNum>,
    pub edu: Vec<KvNum>,
    pub salary: Vec<KvNum>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageItem {
    pub key: String,
    pub title: String,
    pub tc_num: String,
    pub num: String,
    pub unit: String,
    pub width: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrendOut {
    pub name: String,
    pub data: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RangeTotals {
    pub wkg: u64,
    pub kgw: u64,
    pub wdl: u64,
    pub xzjl: u64,
    pub tdjl: u64,
    pub yqms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartSeries {
    pub dates: Vec<String>,
    pub wkg: Vec<i64>,
    pub kgw: Vec<i64>,
    pub wdl: Vec<i64>,
    pub xzjl: Vec<i64>,
    pub tdjl: Vec<i64>,
    pub yqms: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TalentPack {
    pub wkg: TalentDims,
    pub kgw: TalentDims,
    pub xzjl: TalentDims,
    pub tdjl: TalentDims,
    pub yqms: TalentDims,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeekCard {
    pub title: String,
    pub num: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeekPack {
    pub dates: String,
    pub look_data: Vec<WeekCard>,
    pub resume_data: Vec<WeekCard>,
    pub ms_data: Vec<WeekCard>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TongjiTrend {
    pub tdnum: u64,
    pub lookjobnum: u64,
    pub useridmsg: u64,
    pub cgl: i32,
    pub apply: Vec<DayPoint>,
    pub look: Vec<DayPoint>,
    pub jobs: Vec<repo::JobBrief>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DayPoint {
    pub date: String,
    pub td: String,
    pub cnt: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PieSlice {
    pub fields: String,
    pub num: u64,
}

async fn require_zpdata(state: &AppState) -> AppResult<()> {
    if let Some(row) = setting_repo::find(state.db.reader(), "com_zpdata").await? {
        if !row.value.trim().is_empty() && row.value.trim() != "1" {
            return Err(ApiError::business("com_zpdata_closed"));
        }
    }
    Ok(())
}

fn parse_ymd(s: &str) -> AppResult<i64> {
    clock::parse_site_date(s.trim()).ok_or_else(|| ApiError::param_invalid("sdate"))
}

/// PHP `zhaopin::day()` — `days` last N calendar days, or `sdate`/`edate`.
pub fn trend_window(
    days: Option<i32>,
    sdate: Option<&str>,
    edate: Option<&str>,
) -> AppResult<(i64, i64, i32, BucketFmt)> {
    let now = clock::now_ts();
    if let Some(s) = sdate.map(str::trim).filter(|s| !s.is_empty()) {
        let start = parse_ymd(s)?;
        let end = match edate.map(str::trim).filter(|s| !s.is_empty()) {
            Some(e) => parse_ymd(e)?.saturating_add(86_400).saturating_sub(1),
            None => now,
        };
        let span = ((end.saturating_sub(start)) / 86_400).max(0) as i32;
        let days = span.max(1);
        let fmt = if days > 1 {
            BucketFmt::Day
        } else {
            BucketFmt::Hour
        };
        return Ok((start, end, days, fmt));
    }
    let days = days.unwrap_or(1).clamp(1, 366);
    let today = clock::start_of_day(now);
    if days <= 1 {
        Ok((today, today + 86_400 - 1, 1, BucketFmt::Hour))
    } else {
        let start = today - i64::from(days - 1) * 86_400;
        Ok((start, today + 86_400 - 1, days, BucketFmt::Day))
    }
}

/// PHP `formatTimes`: type 1 day range / 2 month / 3 year.
pub fn range_window(kind: i32, times: &serde_json::Value) -> AppResult<(i64, i64, i32)> {
    match kind {
        2 => {
            let raw = times.as_str().unwrap_or("").trim();
            let start = parse_ymd(&format!("{raw}-01")).map_err(|_| ApiError::param_invalid("times"))?;
            let now = clock::now_ts();
            let this_month = clock::start_of_day(now);
            // first day of current month
            let cur = clock::parse_site_date(&chrono_ym(now)).unwrap_or(this_month);
            let end = if start >= cur {
                clock::start_of_day(now) + 86_400 - 1
            } else {
                add_months(start, 1).saturating_sub(1)
            };
            Ok((start, end, 2))
        }
        3 => {
            let y = times.as_str().unwrap_or("").trim();
            let start = parse_ymd(&format!("{y}-01-01")).map_err(|_| ApiError::param_invalid("times"))?;
            let end = parse_ymd(&format!("{y}-12-31"))?
                .saturating_add(86_400)
                .saturating_sub(1);
            Ok((start, end, 3))
        }
        _ => {
            let arr = times.as_array().ok_or_else(|| ApiError::param_invalid("times"))?;
            let s = arr
                .first()
                .and_then(|v| v.as_str())
                .ok_or_else(|| ApiError::param_invalid("times"))?;
            let e = arr
                .get(1)
                .and_then(|v| v.as_str())
                .unwrap_or(s);
            let start = parse_ymd(s)?;
            let end = parse_ymd(e)?.saturating_add(86_400).saturating_sub(1);
            Ok((start, end, 1))
        }
    }
}

fn chrono_ym(ts: i64) -> String {
    let offset = i64::from(clock::tz().local_minus_utc());
    let local = ts + offset;
    let _days = local.div_euclid(86_400);
    // 1970-01-01 was Thursday; convert via NaiveDate in clock's tz by formatting.
    // Use parse of RFC from start_of_day: we only need YYYY-MM.
    let sod = clock::start_of_day(ts);
    let ymd = format_ymd(sod);
    format!("{}-{}", &ymd[..4], &ymd[5..7])
}

fn format_ymd(ts: i64) -> String {
    let offset = i64::from(clock::tz().local_minus_utc());
    let local = ts + offset;
    let (y, m, d) = civil_from_days(local.div_euclid(86_400));
    format!("{y:04}-{m:02}-{d:02}")
}

fn format_md(ts: i64) -> String {
    let offset = i64::from(clock::tz().local_minus_utc());
    let local = ts + offset;
    let (_y, m, d) = civil_from_days(local.div_euclid(86_400));
    format!("{m:02}-{d:02}")
}

fn format_md_cn(ts: i64) -> String {
    let offset = i64::from(clock::tz().local_minus_utc());
    let local = ts + offset;
    let (_y, m, d) = civil_from_days(local.div_euclid(86_400));
    format!("{m}月{d}日")
}

/// Howard Hinnant civil-from-days (Unix epoch day 0 = 1970-01-01).
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

fn add_months(start: i64, months: i32) -> i64 {
    let ymd = format_ymd(start);
    let y: i32 = ymd[0..4].parse().unwrap_or(1970);
    let m: i32 = ymd[5..7].parse().unwrap_or(1);
    let mut ny = y;
    let mut nm = m + months;
    while nm > 12 {
        nm -= 12;
        ny += 1;
    }
    while nm < 1 {
        nm += 12;
        ny -= 1;
    }
    clock::parse_site_date(&format!("{ny:04}-{nm:02}-01")).unwrap_or(start + 30 * 86_400)
}

fn merge_buckets(rows: Vec<BucketCount>) -> HashMap<String, i64> {
    let mut m = HashMap::new();
    for r in rows {
        *m.entry(r.bucket).or_insert(0) += r.cnt;
    }
    m
}

fn fill_hours(map: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut out = HashMap::new();
    for h in 0..24 {
        let k = format!("{h:02}:00");
        out.insert(k.clone(), map.get(&k).copied().unwrap_or(0));
    }
    out
}

fn fill_days_md(start: i64, end: i64, map: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut out = HashMap::new();
    let mut t = clock::start_of_day(start);
    let last = clock::start_of_day(end);
    while t <= last {
        let full = format_ymd(t);
        let md = &full[5..];
        let v = map
            .get(&full)
            .copied()
            .or_else(|| map.get(md).copied())
            .unwrap_or(0);
        out.insert(md.to_string(), v);
        t += 86_400;
    }
    out
}

fn chart_labels(kind: i32, start: i64, end: i64) -> (Vec<String>, Vec<String>) {
    if kind == 3 {
        let ymd = format_ymd(clock::now_ts());
        let this_y: i32 = ymd[0..4].parse().unwrap_or(1970);
        let start_y: i32 = format_ymd(start)[0..4].parse().unwrap_or(this_y);
        let n = if start_y == this_y {
            format_ymd(clock::now_ts())[5..7]
                .parse::<u32>()
                .unwrap_or(12)
        } else {
            12
        };
        let labels: Vec<String> = (1..=n).map(|i| format!("{i}月")).collect();
        let keys: Vec<String> = (1..=n).map(|i| i.to_string()).collect();
        (labels, keys)
    } else {
        let mut labels = Vec::new();
        let mut keys = Vec::new();
        let mut t = clock::start_of_day(start);
        let last = clock::start_of_day(end);
        while t <= last {
            labels.push(format_md(t));
            keys.push({
                let md = format_md(t);
                md.replace('-', "")
                    .trim_start_matches('0')
                    .to_string()
                    .chars()
                    .collect::<String>()
            });
            // PHP %m-%d for chart; %c month for year. For day/month chart keys are m-d after strip.
            t += 86_400;
        }
        // Use m-d as both label and lookup for MonthDay fmt; also try %c-less keys.
        let mut t = clock::start_of_day(start);
        keys.clear();
        while t <= last {
            keys.push(format_md(t));
            t += 86_400;
        }
        (labels, keys)
    }
}

fn detail_labels(kind: i32, start: i64, end: i64) -> (Vec<String>, Vec<String>) {
    if kind == 3 {
        return chart_labels(3, start, end);
    }
    let mut labels = Vec::new();
    let mut keys = Vec::new();
    let mut t = clock::start_of_day(start);
    let last = clock::start_of_day(end);
    while t <= last {
        labels.push(format_md_cn(t));
        let md = format_md(t).replace('-', "");
        keys.push(md.trim_start_matches('0').to_string());
        t += 86_400;
    }
    (labels, keys)
}

fn series_from(keys: &[String], labels: &[String], map: &HashMap<String, i64>) -> Vec<i64> {
    keys.iter()
        .zip(labels.iter())
        .map(|(k, lab)| {
            map.get(k)
                .copied()
                .or_else(|| map.get(lab).copied())
                .or_else(|| {
                    let stripped = lab.replace('月', "").replace('日', "");
                    map.get(&stripped).copied()
                })
                .unwrap_or(0)
        })
        .collect()
}

const SALARY: [(&str, i32, i32); 6] = [
    ("2k-4k", 2000, 4000),
    ("4k-6k", 4000, 6000),
    ("6k-8k", 6000, 8000),
    ("8k-10k", 8000, 10000),
    ("10k以上", 10000, 20000),
    ("20k以上", 20000, 0),
];

async fn map_class(
    state: &AppState,
    rows: Vec<ClassCount>,
    city: bool,
) -> AppResult<Vec<KvNum>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<i32> = rows.iter().map(|r| r.class_id).collect();
    let names = if city {
        repo::names_city(state.db.reader(), &ids).await?
    } else {
        repo::names_userclass(state.db.reader(), &ids).await?
    };
    let map: HashMap<i32, String> = names.into_iter().map(|n| (n.id, n.name)).collect();
    Ok(rows
        .into_iter()
        .filter_map(|r| {
            let name = map.get(&r.class_id).cloned().unwrap_or_default();
            if name.is_empty() {
                None
            } else {
                Some(KvNum {
                    name,
                    value: r.cnt.max(0) as u64,
                })
            }
        })
        .collect())
}

async fn talent_for_uids(state: &AppState, uids: &[u64]) -> AppResult<TalentDims> {
    if uids.is_empty() {
        return Ok(TalentDims {
            exp: vec![],
            edu: vec![],
            salary: vec![],
        });
    }
    let db = state.db.reader();
    let (exp, edu) = tokio::join!(
        repo::group_expect_exp(db, uids),
        repo::group_resume_edu(db, uids),
    );
    let exp = map_class(state, exp?, false).await?;
    let edu = map_class(state, edu?, false).await?;
    let mut salary = Vec::new();
    for (name, min, max) in SALARY {
        let n = repo::count_expect_salary(db, uids, min, max).await?;
        if n > 0 {
            salary.push(KvNum {
                name: name.into(),
                value: n,
            });
        }
    }
    Ok(TalentDims { exp, edu, salary })
}

pub async fn trend(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    days: Option<i32>,
    sdate: Option<&str>,
    edate: Option<&str>,
) -> AppResult<TrendOut> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let (start, end, day_n, fmt) = trend_window(days, sdate, edate)?;
    let db = state.db.reader();
    let uid = user.uid;
    let ut = i32::from(user.usertype);
    let (name, mut map) = match kind {
        2 => (
            "member_com_00372".to_string(),
            merge_buckets(repo::group_look_job(db, uid, start, end, fmt, None).await?),
        ),
        5 => {
            let a = repo::group_down_resume(db, "down_resume", uid, ut, start, end, fmt).await?;
            let b = repo::group_down_resume(db, "freedown_resume", uid, ut, start, end, fmt).await?;
            let mut m = merge_buckets(a);
            for (k, v) in merge_buckets(b) {
                *m.entry(k).or_insert(0) += v;
            }
            ("wap_00451".to_string(), m)
        }
        6 => (
            "wap_com_00235".to_string(),
            merge_buckets(repo::group_apply(db, uid, start, end, fmt, None).await?),
        ),
        8 => (
            "resume_00029".to_string(),
            merge_buckets(repo::group_invite(db, uid, start, end, fmt, None).await?),
        ),
        _ => (
            "member_com_00371".to_string(),
            merge_buckets(repo::group_look_resume(db, uid, ut, start, end, fmt).await?),
        ),
    };
    if day_n <= 1 {
        map = fill_hours(&map);
    } else {
        map = fill_days_md(start, end, &map);
    }
    Ok(TrendOut { name, data: map })
}

fn dash(v: i32) -> String {
    if v == 0 {
        "-".into()
    } else {
        v.to_string()
    }
}

fn width(tc: &str, num: &str) -> String {
    if tc == "-" {
        return "100".into();
    }
    let tc_n: f64 = tc.parse().unwrap_or(0.0);
    let num_n: f64 = num.parse().unwrap_or(0.0);
    if tc_n <= 0.0 || num_n <= 0.0 {
        return "0".into();
    }
    format!("{}", ((num_n / tc_n) * 100.0).round())
}

pub async fn package(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<PackageItem>> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let db = state.db.reader();
    let st = statis_repo::find_admin(db, user.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("statis_not_found"))?;
    let caps = repo::find_rating_caps(db, st.rating)
        .await?
        .unwrap_or(repo::RatingCaps {
            job_num: 0,
            breakjob_num: 0,
            resume: 0,
            interview: 0,
            zph_num: 0,
            top_num: 0,
            urgent_num: 0,
            rec_num: 0,
        });
    let mut job_left = st.job_num;
    if st.rating_type == 1 {
        let listed = job_repo::count_listed_by_uid(db, user.uid).await? as i32
            + part_repo::count_listed_by_uid(db, user.uid).await? as i32;
        job_left = (st.job_num - listed).max(0);
    }
    let time = st.rating_type == 2;
    let item = |key: &str, title: &str, tc: String, num: String, unit: &str| PackageItem {
        width: width(&tc, &num),
        key: key.into(),
        title: title.into(),
        tc_num: tc,
        num,
        unit: unit.into(),
    };
    let mut out = Vec::new();
    out.push(item(
        "ksj",
        "member_com_00134",
        caps.job_num.to_string(),
        job_left.to_string(),
        "个",
    ));
    if time {
        out.push(item(
            "ksx",
            "member_com_00136",
            dash(caps.breakjob_num),
            if st.breakjob_num == 0 {
                "-".into()
            } else {
                st.breakjob_num.to_string()
            },
            if st.breakjob_num == 0 {
                ""
            } else {
                "wap_com_00049"
            },
        ));
        out.push(item(
            "kms",
            "member_com_00137",
            dash(caps.interview),
            if st.invite_resume == 0 {
                "-".into()
            } else {
                st.invite_resume.to_string()
            },
            if st.invite_resume == 0 {
                ""
            } else {
                "wap_com_00049"
            },
        ));
        out.push(item(
            "kxz",
            "member_com_00135",
            dash(caps.resume),
            if st.down_resume == 0 {
                "-".into()
            } else {
                st.down_resume.to_string()
            },
            if st.down_resume == 0 {
                ""
            } else {
                "admin_system_00408"
            },
        ));
        out.push(item(
            "zd",
            "wap_user_00209",
            dash(caps.top_num),
            if st.top_num == 0 {
                "-".into()
            } else {
                st.top_num.to_string()
            },
            if st.top_num == 0 { "" } else { "天" },
        ));
        out.push(item(
            "jj",
            "wap_com_00043",
            dash(caps.urgent_num),
            if st.urgent_num == 0 {
                "-".into()
            } else {
                st.urgent_num.to_string()
            },
            if st.urgent_num == 0 { "" } else { "天" },
        ));
        out.push(item(
            "tj",
            "wap_com_00041",
            dash(caps.rec_num),
            if st.rec_num == 0 {
                "-".into()
            } else {
                st.rec_num.to_string()
            },
            if st.rec_num == 0 { "" } else { "天" },
        ));
        out.push(item(
            "zph",
            "member_com_00323",
            dash(caps.zph_num),
            if st.zph_num == 0 {
                "-".into()
            } else {
                st.zph_num.to_string()
            },
            if st.zph_num == 0 {
                ""
            } else {
                "wap_com_00049"
            },
        ));
    } else {
        out.push(item(
            "ksx",
            "member_com_00136",
            caps.breakjob_num.to_string(),
            st.breakjob_num.to_string(),
            "次",
        ));
        out.push(item(
            "kms",
            "member_com_00137",
            caps.interview.to_string(),
            st.invite_resume.to_string(),
            "次",
        ));
        out.push(item(
            "kxz",
            "member_com_00135",
            caps.resume.to_string(),
            st.down_resume.to_string(),
            "份",
        ));
        out.push(item(
            "zd",
            "wap_user_00209",
            caps.top_num.to_string(),
            st.top_num.to_string(),
            "天",
        ));
        out.push(item(
            "jj",
            "wap_com_00043",
            caps.urgent_num.to_string(),
            st.urgent_num.to_string(),
            "天",
        ));
        out.push(item(
            "tj",
            "wap_com_00041",
            caps.rec_num.to_string(),
            st.rec_num.to_string(),
            "天",
        ));
        out.push(item(
            "zph",
            "member_com_00323",
            caps.zph_num.to_string(),
            st.zph_num.to_string(),
            "次",
        ));
    }
    Ok(out)
}

pub async fn range_totals(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    times: &serde_json::Value,
) -> AppResult<RangeTotals> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let (start, end, _) = range_window(kind, times)?;
    let db = state.db.reader();
    let uid = user.uid;
    let ut = i32::from(user.usertype);
    let (wkg, kgw, wdl, xz, fxz, tdjl, yqms) = tokio::join!(
        phpyun_models::look_resume::count_by_com_range(db, uid, ut, start, end),
        phpyun_models::look_job::count_by_com_range(db, uid, start, end),
        repo::count_login_range(db, uid, start, end),
        phpyun_models::resume_download::repo::count_down_range(db, uid, ut, start, end),
        phpyun_models::resume_download::repo::count_freedown_range(db, uid, ut, start, end),
        phpyun_models::apply::repo::count_by_com_range(db, uid, start, end),
        phpyun_models::userid_msg::repo::count_by_fid_range(db, uid, start, end),
    );
    Ok(RangeTotals {
        wkg: wkg?,
        kgw: kgw?,
        wdl: wdl?,
        xzjl: xz? + fxz?,
        tdjl: tdjl?,
        yqms: yqms?,
    })
}

async fn six_series(
    state: &AppState,
    user: &AuthenticatedUser,
    start: i64,
    end: i64,
    fmt: BucketFmt,
    labels: &[String],
    keys: &[String],
) -> AppResult<ChartSeries> {
    let db = state.db.reader();
    let uid = user.uid;
    let ut = i32::from(user.usertype);
    let (wkg, kgw, wdl, xz, fxz, tdjl, yqms) = tokio::join!(
        repo::group_look_resume(db, uid, ut, start, end, fmt),
        repo::group_look_job(db, uid, start, end, fmt, None),
        repo::group_login(db, uid, start, end, fmt),
        repo::group_down_resume(db, "down_resume", uid, ut, start, end, fmt),
        repo::group_down_resume(db, "freedown_resume", uid, ut, start, end, fmt),
        repo::group_apply(db, uid, start, end, fmt, None),
        repo::group_invite(db, uid, start, end, fmt, None),
    );
    let mut xzjl = merge_buckets(xz?);
    for (k, v) in merge_buckets(fxz?) {
        *xzjl.entry(k).or_insert(0) += v;
    }
    Ok(ChartSeries {
        dates: labels.to_vec(),
        wkg: series_from(keys, labels, &merge_buckets(wkg?)),
        kgw: series_from(keys, labels, &merge_buckets(kgw?)),
        wdl: series_from(keys, labels, &merge_buckets(wdl?)),
        xzjl: series_from(keys, labels, &xzjl),
        tdjl: series_from(keys, labels, &merge_buckets(tdjl?)),
        yqms: series_from(keys, labels, &merge_buckets(yqms?)),
    })
}

pub async fn chart(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    times: &serde_json::Value,
) -> AppResult<ChartSeries> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let (start, end, k) = range_window(kind, times)?;
    let fmt = if k == 3 {
        BucketFmt::MonthNum
    } else {
        BucketFmt::MonthDay
    };
    let (labels, keys) = chart_labels(k, start, end);
    six_series(state, user, start, end, fmt, &labels, &keys).await
}

pub async fn details(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    times: &serde_json::Value,
) -> AppResult<ChartSeries> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let (start, end, k) = range_window(kind, times)?;
    let fmt = if k == 3 {
        BucketFmt::MonthNum
    } else {
        BucketFmt::MdCompact
    };
    let (labels, keys) = detail_labels(k, start, end);
    six_series(state, user, start, end, fmt, &labels, &keys).await
}

pub async fn talent(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    times: &serde_json::Value,
) -> AppResult<TalentPack> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let (start, end, _) = range_window(kind, times)?;
    let db = state.db.reader();
    let uid = user.uid;
    let ut = i32::from(user.usertype);
    let (wkg, kgw, xz, fxz, tdjl, yqms) = tokio::join!(
        repo::uids_look_resume(db, uid, ut, start, end),
        repo::uids_look_job(db, uid, start, end),
        repo::uids_down(db, "down_resume", uid, ut, start, end),
        repo::uids_down(db, "freedown_resume", uid, ut, start, end),
        repo::uids_apply(db, uid, start, end),
        repo::uids_invite(db, uid, start, end),
    );
    let mut xzjl = xz?;
    xzjl.extend(fxz?);
    xzjl.sort_unstable();
    xzjl.dedup();
    Ok(TalentPack {
        wkg: talent_for_uids(state, &wkg?).await?,
        kgw: talent_for_uids(state, &kgw?).await?,
        xzjl: talent_for_uids(state, &xzjl).await?,
        tdjl: talent_for_uids(state, &tdjl?).await?,
        yqms: talent_for_uids(state, &yqms?).await?,
    })
}

pub async fn week(
    state: &AppState,
    user: &AuthenticatedUser,
    times: i32,
) -> AppResult<WeekPack> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let times = times.clamp(1, 4);
    let today = clock::start_of_day(clock::now_ts());
    let start = if times == 4 {
        add_months(today, -1)
    } else {
        today - i64::from(times * 7 - 1) * 86_400
    };
    let end = today + 86_400 - 1;
    let db = state.db.reader();
    let uid = user.uid;
    let ut = i32::from(user.usertype);
    let (wkg, kgw, wdl, xz, fxz, tdjl, yqms) = tokio::join!(
        phpyun_models::look_resume::count_by_com_range(db, uid, ut, start, end),
        phpyun_models::look_job::count_by_com_range(db, uid, start, end),
        repo::count_login_range(db, uid, start, end),
        phpyun_models::resume_download::repo::count_down_range(db, uid, ut, start, end),
        phpyun_models::resume_download::repo::count_freedown_range(db, uid, ut, start, end),
        phpyun_models::apply::repo::count_by_com_range(db, uid, start, end),
        phpyun_models::userid_msg::repo::count_by_fid_range(db, uid, start, end),
    );
    let td = tdjl?;
    let dates = format!("{}-{}", format_ymd(start).replace('-', "."), format_ymd(today).replace('-', "."));
    Ok(WeekPack {
        dates,
        look_data: vec![
            WeekCard {
                title: "member_com_00371".into(),
                num: wkg?,
            },
            WeekCard {
                title: "member_com_00372".into(),
                num: kgw?,
            },
            WeekCard {
                title: "member_com_00370".into(),
                num: wdl?,
            },
        ],
        resume_data: vec![
            WeekCard {
                title: "wap_com_00426".into(),
                num: td,
            },
            WeekCard {
                title: "wap_00451".into(),
                num: xz? + fxz?,
            },
            WeekCard {
                title: "wap_com_00235".into(),
                num: td,
            },
        ],
        ms_data: vec![WeekCard {
            title: "resume_00029".into(),
            num: yqms?,
        }],
    })
}

pub async fn tongji_trend(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: Option<u64>,
    sdate: &str,
    edate: &str,
) -> AppResult<TongjiTrend> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let start = parse_ymd(sdate)?;
    let end = parse_ymd(edate)?.saturating_add(86_400).saturating_sub(1);
    let db = state.db.reader();
    let uid = user.uid;
    let jid = job_id.filter(|n| *n > 0);
    let (tdnum, lookjobnum, useridmsg, cgmsg, apply, look, jobs) = tokio::join!(
        repo::count_apply_range(db, uid, start, end, jid),
        repo::count_look_job_range(db, uid, start, end, jid),
        repo::count_invite_range(db, uid, start, end, jid, None),
        repo::count_invite_range(db, uid, start, end, jid, Some(3)),
        repo::group_apply(db, uid, start, end, BucketFmt::Ymd, jid),
        repo::group_look_job(db, uid, start, end, BucketFmt::Ymd, jid),
        repo::list_jobs_brief(db, uid),
    );
    let useridmsg = useridmsg?;
    let cgmsg = cgmsg?;
    let cgl = if useridmsg > 0 && cgmsg > 0 {
        ((cgmsg as f64 / useridmsg as f64) * 100.0).round() as i32
    } else {
        0
    };
    let apply_m = merge_buckets(apply?);
    let look_m = merge_buckets(look?);
    let mut apply_pts = Vec::new();
    let mut look_pts = Vec::new();
    let mut t = clock::start_of_day(start);
    let last = clock::start_of_day(end);
    while t <= last {
        let ymd = format_ymd(t).replace('-', "");
        let td = format_md(t);
        let date = format_ymd(t);
        apply_pts.push(DayPoint {
            date: date.clone(),
            td: td.clone(),
            cnt: apply_m.get(&ymd).copied().unwrap_or(0),
        });
        look_pts.push(DayPoint {
            date,
            td,
            cnt: look_m.get(&ymd).copied().unwrap_or(0),
        });
        t += 86_400;
    }
    Ok(TongjiTrend {
        tdnum: tdnum?,
        lookjobnum: lookjobnum?,
        useridmsg,
        cgl,
        apply: apply_pts,
        look: look_pts,
        jobs: jobs?,
    })
}

pub async fn tongji_pie(
    state: &AppState,
    user: &AuthenticatedUser,
    pie_type: i32,
    job_id: Option<u64>,
    sdate: &str,
    edate: &str,
) -> AppResult<Vec<PieSlice>> {
    user.require_employer()?;
    require_zpdata(state).await?;
    let start = parse_ymd(sdate)?;
    let end = parse_ymd(edate)?.saturating_add(86_400).saturating_sub(1);
    let db = state.db.reader();
    let uid = user.uid;
    let jid = job_id.filter(|n| *n > 0);
    match pie_type {
        1 => {
            let rows = repo::group_apply_province(db, uid, start, end, jid).await?;
            Ok(map_class(state, rows, true)
                .await?
                .into_iter()
                .map(|k| PieSlice {
                    fields: k.name,
                    num: k.value,
                })
                .collect())
        }
        2 => {
            let eids = repo::apply_eids(db, uid, start, end, jid).await?;
            let rows = repo::group_expect_edu_by_ids(db, &eids).await?;
            Ok(map_class(state, rows, false)
                .await?
                .into_iter()
                .map(|k| PieSlice {
                    fields: k.name,
                    num: k.value,
                })
                .collect())
        }
        3 => {
            let eids = repo::apply_eids(db, uid, start, end, jid).await?;
            let mut out = Vec::new();
            for (name, min, max) in SALARY {
                let n = repo::count_expect_salary_by_ids(db, &eids, min, max).await?;
                if n > 0 {
                    out.push(PieSlice {
                        fields: name.into(),
                        num: n,
                    });
                }
            }
            Ok(out)
        }
        _ => {
            let eids = repo::apply_eids(db, uid, start, end, jid).await?;
            let rows = repo::group_expect_exp_by_ids(db, &eids).await?;
            Ok(map_class(state, rows, false)
                .await?
                .into_iter()
                .map(|k| PieSlice {
                    fields: k.name,
                    num: k.value,
                })
                .collect())
        }
    }
}
