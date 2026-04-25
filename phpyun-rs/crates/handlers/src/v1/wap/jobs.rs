//! Public job browsing (WAP, aligned with `wap/job::index_action` / detail portion of `wap/job::comapply_action`).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, ClientIp, MaybeUser, Paged, Pagination, ValidatedJson,
};
use validator::Validate;
use phpyun_services::hot_search_service;
use phpyun_services::job_service::{self, JobSearch};
use phpyun_services::view_service::{self, KIND_JOB};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// UNIX timestamp -> local time string. `ts<=0` returns an empty string (aligned with PHPYun `date('Y-m-d H:i', $ts)`).
fn fmt_ts(ts: i64, pattern: &str) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format(pattern).to_string())
        .unwrap_or_default()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(list_jobs))
        .route("/jobs/{id}", get(job_detail))
        .route("/jobs/{id}/similar", get(similar_jobs))
        .route("/jobs/{id}/same-company", get(same_company_jobs))
        .route("/companies/{uid}/jobs", get(company_jobs))
        .route("/jobs/{id}/tel-click", post(log_tel_click))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct JobListQuery {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub job1: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    /// 1 = full-time / 2 = part-time / 3 = internship / 4 = temporary
    pub job_type: Option<i32>,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_did() -> u32 {
    // PHPYun did=0 means site-wide; did>0 means a sub-site. Default to site-wide; clients must pass did explicitly when targeting a sub-site.
    0
}

/// Job list item -- aligned with the field set returned by PHPYun `JobM::getList()`.
#[derive(Debug, Serialize, ToSchema)]
pub struct JobSummary {
    // Basics
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub com_logo: Option<String>,

    // Category ids
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub hy: i32,

    // Category names (results of dictionary lookup, corresponding to PHP `job_one_n / job_two_n / job_three_n / job_hy`)
    pub job_one_n: String,
    pub job_two_n: String,
    pub job_three_n: String,
    pub job_hy: String,
    /// Three-level job category joined as "Frontend / Web Frontend / React Developer" (PHP `jobname`)
    pub jobname: String,

    // Location
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    /// Province name (PHP `job_city_one`)
    pub job_city_one: String,
    /// City name (PHP `job_city_two`)
    pub job_city_two: String,

    // Salary
    pub salary: i32,
    pub min_salary: i32,
    pub max_salary: i32,

    // Requirements
    pub exp: i32,
    pub edu: i32,

    // Promotion status (computed from rec_time / urgent_time vs. now, aligned with PHP `isrec / isurgent`)
    pub rec: i32,
    pub urgent: i32,
    pub is_rec: bool,
    pub is_urgent: bool,
    pub rec_time: i64,
    pub urgent_time: i64,

    // Time
    pub sdate: i64,
    pub lastupdate: i64,
    /// Posted within the last 2 days (PHP `newtime`)
    pub newtime: bool,

    // Stats
    pub jobhits: i32,

    /// Whether the *current* user has favorited this job. Always `false` for
    /// unauthenticated requests. Populated in batch by the list handlers
    /// (`collect_service::favorited_set`) so there's no N+1 query.
    pub is_favorited: bool,
}

impl JobSummary {
    pub fn from_with_dict(
        j: phpyun_models::job::entity::Job,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
        now: i64,
    ) -> Self {
        Self::from_with_dict_fav(j, dicts, now, false)
    }

    /// Variant that lets the caller stamp the favorited bit (already resolved
    /// from a batch query). Use this from the list / search / home handlers.
    pub fn from_with_dict_fav(
        j: phpyun_models::job::entity::Job,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
        now: i64,
        is_favorited: bool,
    ) -> Self {
        let mut s = Self::from_with_dict_inner(j, dicts, now);
        s.is_favorited = is_favorited;
        s
    }

    fn from_with_dict_inner(
        j: phpyun_models::job::entity::Job,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
        now: i64,
    ) -> Self {
        let job_one_n = dicts.job(j.job1).to_string();
        let job_two_n = dicts.job(j.job1_son).to_string();
        let job_three_n = dicts.job(j.job_post).to_string();
        let parts: Vec<&str> = [&job_one_n, &job_two_n, &job_three_n]
            .iter()
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .collect();
        let jobname = parts.join(" / ");
        let job_hy = dicts.industry(j.hy).to_string();
        let job_city_one = dicts.city(j.provinceid).to_string();
        let job_city_two = dicts.city(j.cityid).to_string();

        let is_rec = j.rec == 1 && j.rec_time > now;
        let is_urgent = j.urgent == 1 && j.urgent_time > now;
        let newtime = j.sdate > now - 2 * 86_400;

        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            com_logo: j.com_logo,

            job1: j.job1,
            job1_son: j.job1_son,
            job_post: j.job_post,
            hy: j.hy,

            job_one_n,
            job_two_n,
            job_three_n,
            job_hy,
            jobname,

            province_id: j.provinceid,
            city_id: j.cityid,
            three_city_id: j.three_cityid,
            job_city_one,
            job_city_two,

            salary: (j.minsalary + j.maxsalary) / 2,
            min_salary: j.minsalary,
            max_salary: j.maxsalary,

            exp: j.exp,
            edu: j.edu,

            rec: j.rec,
            urgent: j.urgent,
            is_rec,
            is_urgent,
            rec_time: j.rec_time,
            urgent_time: j.urgent_time,

            sdate: j.sdate,
            lastupdate: j.lastupdate,
            newtime,

            jobhits: j.jobhits,
            is_favorited: false,
        }
    }
}

/// Public job list (paginated + searchable). Response `data` field looks like `{list, total, page, page_size}`.
#[utoipa::path(
    get,
    path = "/v1/wap/jobs",
    tag = "wap",
    params(JobListQuery),
    responses(
        (status = 200, description = "ok"),
    )
)]
pub async fn list_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    page: Pagination,
    Query(q): Query<JobListQuery>,
) -> AppResult<ApiJson<Paged<JobSummary>>> {
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "job", kw.trim().to_string());
        if let Some(u) = user.as_ref() {
            phpyun_services::search_history_service::record_async(
                &state,
                u.uid,
                "job",
                kw.trim().to_string(),
            );
        }
    }
    let search = JobSearch {
        keyword: q.keyword,
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        job1: q.job1,
        min_salary: q.min_salary,
        max_salary: q.max_salary,
        exp: q.exp,
        edu: q.edu,
        job_type: q.job_type,
        did: q.did,
    };
    let r = job_service::list_public(&state, &search, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = r.list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                JobSummary::from_with_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Public job detail -- returned as a nested map **grouped by business concern**.
///
/// Response structure:
/// - `job`: job main-table fields passed through (field names match DB columns, e.g. `provinceid` / `minsalary` / `zp_num`)
/// - `company`: company-related fields (`logo` / `provinceid` / `cityid` / `hy` / `linkman` / `linktel` / ...)
/// - `dict`: dictionary translations (`job_one/two/three` / `jobname` / `hy_n` / `welfare_names` / `langname` / `city_one/two`)
/// - `user_context`: counters tied to the currently logged-in user (all 0 when not logged in)
/// - `msg_list`: latest 5 job inquiries
/// - `formatted`: display-ready formatted strings (timestamps, average salary)
#[utoipa::path(
    get,
    path = "/v1/wap/jobs/{id}",
    tag = "wap",
    params(("id" = u64, Path, description = "Job id")),
    responses(
        (status = 200, description = "ok"),
        (status = 404, description = "Not found"),
        (status = 410, description = "Off-shelf / expired"),
    )
)]
pub async fn job_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    let d = job_service::get_detail(&state, id).await?;
    // Logged-in user: record visit footprint + bump view count (fire-and-forget)
    if let Some(u) = user.as_ref() {
        view_service::record_async(&state, u.uid, KIND_JOB, id);
    }
    let now = phpyun_core::clock::now_ts();

    // --- Dictionary translations ---
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let job_one = dicts.job(d.job.job1).to_string();
    let job_two = dicts.job(d.job.job1_son).to_string();
    let job_three = dicts.job(d.job.job_post).to_string();
    let jobname = [&job_one, &job_two, &job_three]
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(" / ");
    let hy_n = dicts.industry(d.job.hy).to_string();
    let welfare_names = d
        .job
        .welfare
        .as_deref()
        .map(|s| dicts.comclass_csv(s))
        .unwrap_or_default();
    let langname = d
        .job
        .lang
        .as_deref()
        .map(|s| dicts.comclass_csv(s))
        .unwrap_or_default();
    let city_one = dicts.city(d.job.provinceid).to_string();
    let city_two = dicts.city(d.job.cityid).to_string();

    // --- Latest 5 inquiries (raw table fields passed through) ---
    let msg_list: Vec<json::Value> = {
        let rows: Vec<(
            i64, i64, Option<String>, Option<String>, Option<String>, i64, i64,
        )> = sqlx::query_as(
            "SELECT id, CAST(COALESCE(uid,0) AS SIGNED), username, content, reply, \
                    CAST(COALESCE(datetime,0) AS SIGNED), CAST(COALESCE(reply_time,0) AS SIGNED) \
             FROM phpyun_msg \
             WHERE jobid = ? AND job_uid = ? AND status = 1 \
               AND reply IS NOT NULL AND reply != '' AND del_status = 0 \
             ORDER BY datetime DESC LIMIT 5",
        )
        .bind(id as i64)
        .bind(d.job.uid as i64)
        .fetch_all(state.db.reader())
        .await
        .unwrap_or_default();
        rows.into_iter()
            .map(|(id, uid, username, content, reply, datetime, reply_time)| {
                json::json!({
                    "id": id,
                    "uid": uid,
                    "username": username.unwrap_or_default(),
                    "content": content.unwrap_or_default(),
                    "reply": reply.unwrap_or_default(),
                    "datetime": datetime,
                    "reply_time": reply_time,
                })
            })
            .collect()
    };

    // --- Current user context (4 parallel counter queries) ---
    let (fav_job, userid_job, report_job, invite_job) = if let Some(u) = user.as_ref() {
        let db = state.db.reader();
        let uid = u.uid as i64;
        let jid = id as i64;
        let com_id = d.job.uid as i64;
        let fav_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_fav_job WHERE uid = ? AND job_id = ? AND type = 1",
        )
        .bind(uid)
        .bind(jid)
        .fetch_one(db);
        let apply_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_userid_job \
             WHERE uid = ? AND job_id = ? AND isdel = 9",
        )
        .bind(uid)
        .bind(jid)
        .fetch_one(db);
        let report_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_report WHERE p_uid = ? AND eid = ? AND c_uid = ?",
        )
        .bind(uid)
        .bind(jid)
        .bind(com_id)
        .fetch_one(db);
        let invite_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_yqmb WHERE uid = ? AND did = ? AND status != 0",
        )
        .bind(com_id)
        .bind(uid)
        .fetch_one(db);
        let (f, a, r, i) = tokio::join!(fav_fut, apply_fut, report_fut, invite_fut);
        (
            f.map(|(n,)| n as i32).unwrap_or(0),
            a.map(|(n,)| n as i32).unwrap_or(0),
            r.map(|(n,)| n as i32).unwrap_or(0),
            i.map(|(n,)| n as i32).unwrap_or(0),
        )
    } else {
        (0, 0, 0, 0)
    };

    // --- Assemble the response grouped by business concern ---
    Ok(ApiJson(json::json!({
        // Job main table (Job entity serialized directly; field names = original DB column names)
        "job": d.job,

        // Company-related
        "company": {
            "logo": d.com_logo,
            "provinceid": d.com_provinceid,
            "cityid": d.com_cityid,
            "mun": d.com_mun,
            "hy": d.com_hy,
            "rating": d.com_rating,
            "qcode": d.comqcode,
            "linkman": d.linkman,
            "linktel": d.linktel,
            "linkphone": d.linkphone,
            "linkmail": d.linkmail,
            "login_date": d.login_date,
        },

        // Dictionary translation results (display-only, read-only)
        "dict": {
            "job_one": job_one,
            "job_two": job_two,
            "job_three": job_three,
            "jobname": jobname,
            "hy_n": hy_n,
            "welfare_names": welfare_names,
            "langname": langname,
            "city_one": city_one,
            "city_two": city_two,
        },

        // Currently logged-in user context (all 0 when not logged in)
        "user_context": {
            "fav_job": fav_job,
            // Boolean alias of `fav_job > 0` — front-end button state ("已收藏 / 收藏").
            "is_favorited": fav_job > 0,
            "userid_job": userid_job,
            // Whether the user has already applied to this job (alias of `userid_job > 0`).
            "is_applied": userid_job > 0,
            "report_job": report_job,
            "invite_job": invite_job,
            "job_rec": if d.job.rec_time > now { 1 } else { 0 },
            "job_urgent": if d.job.urgent_time > now { 1 } else { 0 },
        },

        // Latest 5 job inquiries (raw fields passed through)
        "msg_list": msg_list,

        // Display-ready formatted strings
        "formatted": {
            "sdate_n": fmt_ts(d.job.sdate, "%Y-%m-%d %H:%M"),
            "lastupdate_n": fmt_ts(d.job.lastupdate, "%Y-%m-%d %H:%M"),
            "salary": (d.job.minsalary + d.job.maxsalary) / 2,
        },
    })))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct RecQuery {
    #[serde(default = "default_rec_limit")]
    pub limit: u64,
}
fn default_rec_limit() -> u64 { 6 }

/// Similar jobs (same job1 category)
#[utoipa::path(
    get,
    path = "/v1/wap/jobs/{id}/similar",
    tag = "wap",
    params(("id" = u64, Path), RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn similar_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(id): Path<u64>,
    axum::extract::Query(q): axum::extract::Query<RecQuery>,
) -> AppResult<ApiJson<Vec<JobSummary>>> {
    let list = job_service::list_similar(&state, id, q.limit.clamp(1, 30)).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(
        list.into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                JobSummary::from_with_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
    ))
}

/// Other jobs from the same company
#[utoipa::path(
    get,
    path = "/v1/wap/jobs/{id}/same-company",
    tag = "wap",
    params(("id" = u64, Path), RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn same_company_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(id): Path<u64>,
    axum::extract::Query(q): axum::extract::Query<RecQuery>,
) -> AppResult<ApiJson<Vec<JobSummary>>> {
    let list = job_service::list_same_company(&state, id, q.limit.clamp(1, 30)).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(
        list.into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                JobSummary::from_with_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
    ))
}

/// Public job list for a given company (paginated)
#[utoipa::path(
    get,
    path = "/v1/wap/companies/{uid}/jobs",
    tag = "wap",
    params(("uid" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn company_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(uid): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<JobSummary>>> {
    let r = job_service::list_by_company(&state, uid, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = r.list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                JobSummary::from_with_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== Phone click logging ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TelClickBody {
    /// 1 = pc / 2 = wap / 3 = wxapp / ... source channel identifier (aligned with PHPYun `source` field)
    #[serde(default = "default_source")]
    pub source: i32,
    /// Optional: when jobid does not exist, use this to locate the company
    #[serde(default)]
    pub com_id: u64,
}
fn default_source() -> i32 {
    2
}

/// Records a "click on job contact phone" action (aligned with PHPYun `wap/ajax::addJobTelLog_action`)
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/{id}/tel-click",
    tag = "wap",
    params(("id" = u64, Path)),
    request_body = TelClickBody,
    responses((status = 200, description = "ok"))
)]
pub async fn log_tel_click(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<TelClickBody>,
) -> AppResult<ApiJson<json::Value>> {
    let viewer_uid = user.as_ref().map(|u| u.uid);
    job_service::log_tel_click(&state, viewer_uid, id, b.com_id, b.source, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
