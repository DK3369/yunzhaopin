//! Public job browsing (WAP, aligned with `wap/job::index_action` / detail portion of `wap/job::comapply_action`).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, ClientIp, MaybeUser, Paged, Pagination, ValidatedJson};
use validator::Validate;
use phpyun_services::hot_search_service;
use phpyun_services::job_service::{self, JobSearch};
use phpyun_services::view_service::{self, KIND_JOB};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use phpyun_core::dto::{HitsResp, IdBody, UidBody};
use phpyun_core::utils::{fmt_ts};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SimilarBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[serde(default = "default_rec_limit")]
    #[validate(range(min = 1, max = 30))]
    pub limit: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TelClickBodyFull {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[serde(default = "default_source")]
    #[validate(range(min = 0, max = 99))]
    pub source: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub com_id: u64,
}

/// UNIX timestamp -> local time string. `ts<=0` returns an empty string (aligned with PHPYun `date('Y-m-d H:i', $ts)`).

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", post(list_jobs))
        .route("/jobs/detail", post(job_detail))
        .route("/jobs/similar", post(similar_jobs))
        .route("/jobs/same-company", post(same_company_jobs))
        .route("/companies/jobs", post(company_jobs))
        .route("/jobs/tel-click", post(log_tel_click))
        .route("/jobs/share-text", post(share_text))
        .route("/jobs/hits", post(bump_jobhits))
        .route("/jobs/contact", post(job_contact))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct JobListQuery {
    /// When provided, `/v1/wap/jobs` returns the **detail** of a single job
    /// instead of the paginated list — same endpoint, two response shapes.
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: Option<u64>,
    /// Free-text search; capped at 100 chars to keep LIKE plans sane and
    /// guard against memory-exhaustion via 10MB-keyword requests.
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: Option<i32>,
    #[validate(range(min = 0, max = 9_999_999))]
    pub job1: Option<i32>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub min_salary: Option<i32>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub max_salary: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub exp: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub edu: Option<i32>,
    /// 1 = full-time / 2 = part-time / 3 = internship / 4 = temporary
    #[validate(range(min = 0, max = 4))]
    pub job_type: Option<i32>,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
}
fn default_did() -> u32 {
    // PHPYun did=0 means site-wide; did>0 means a sub-site. Default to site-wide; clients must pass did explicitly when targeting a sub-site.
    0
}

/// Job list item — defined in `phpyun_models::job::view` and re-exported here
/// so legacy paths (`wap::jobs::JobSummary`) keep working. See the model crate
/// for the field reference; dict-translation builds via `job_summary_from_dict`
/// below.
pub use phpyun_models::job::view::JobSummary;

/// Build a fully-populated `JobSummary` (dictionary names + time-derived
/// flags). Mirrors the previous `crate::v1::wap::jobs::job_summary_from_dict` method.
pub fn job_summary_from_dict(
    j: phpyun_models::job::entity::Job,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
    now: i64,
) -> JobSummary {
    job_summary_from_dict_fav(j, dicts, now, false)
}

/// Same as [`job_summary_from_dict`] but stamps the `is_favorited` bit (already
/// resolved by a batch query in the calling handler).
pub fn job_summary_from_dict_fav(
    j: phpyun_models::job::entity::Job,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
    now: i64,
    is_favorited: bool,
) -> JobSummary {
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

    JobSummary {
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
        exp_n: dicts.comclass(j.exp).to_string(),
        edu: j.edu,
        edu_n: dicts.comclass(j.edu).to_string(),

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
        is_favorited,
    }
}

/// Public job endpoint with two modes:
/// - **Detail mode**: when the body carries `id`, returns the same rich
///   shape as `/v1/wap/jobs/detail` (`{job, company, dict, user_context,
///   msg_list, formatted}`).
/// - **List mode**: when `id` is absent, returns the paginated browse list
///   (`{list, total, page, page_size}`).
#[utoipa::path(
    post,
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
    ValidatedJson(q): ValidatedJson<JobListQuery>,
) -> AppResult<ApiJson<json::Value>> {
    // Detail mode: body carried an id → defer to the same logic as
    // `/v1/wap/jobs/detail` so callers get the full job document.
    if let Some(id) = q.id {
        return Ok(ApiJson(
            build_job_detail_value(&state, user.as_ref(), id).await?,
        ));
    }

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
    let paged: Paged<JobSummary> = Paged::new(
        r.list
            .into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
        r.total,
        page.page,
        page.page_size,
    );
    Ok(ApiJson(json::to_value(&paged).map_err(phpyun_core::AppError::internal)?))
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
    post,
    path = "/v1/wap/jobs/detail",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok"),
        (status = 404, description = "Not found"),
        (status = 410, description = "Off-shelf / expired"),
    )
)]
pub async fn job_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    Ok(ApiJson(build_job_detail_value(&state, user.as_ref(), b.id).await?))
}

/// Detail-page body builder, callable from both `/v1/wap/jobs/detail` and
/// `/v1/wap/jobs` (when caller supplies an `id`). Extracted from `job_detail`
/// so the dispatch in `list_jobs` can reuse the same response shape.
pub async fn build_job_detail_value(
    state: &AppState,
    user: Option<&phpyun_core::AuthenticatedUser>,
    id: u64,
) -> AppResult<json::Value> {
    let d = job_service::get_detail(state, id).await?;
    // Logged-in user: record visit footprint + bump view count (fire-and-forget)
    if let Some(u) = user {
        view_service::record_async(state, u.uid, KIND_JOB, id);
    }
    let now = phpyun_core::clock::now_ts();

    // --- Dictionary translations ---
    let dicts = phpyun_services::dict_service::get(state).await?;
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
        )> = sqlx::query_as( // TODO(arch): job_msg::repo::list_public_for_job filters by `type` — add a no-type-filter variant for job-detail page
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

    // --- Current user context (4 parallel checks) ---
    // The "invite_job" check below queries `phpyun_yqmb` directly: that
    // table tracks **interview invitations** (面邀), exposed elsewhere via
    // `interview::repo`, but the predicate we want here ("has THIS company
    // already invited THIS user for an interview on this job?") doesn't fit
    // the `interview::repo` API. Same for `phpyun_report` — kept as raw SQL.
    let (fav_job, userid_job, report_job, invite_job) = if let Some(u) = user.as_ref() {
        let db = state.db.reader();
        let com_id = d.job.uid as i64;
        let fav_fut = phpyun_models::collect::repo::exists_with_type(db, u.uid, id, 1);
        let apply_fut = phpyun_models::apply::repo::find_by_uid_job(db, u.uid, id);
        let report_fut = sqlx::query_as::<_, (i64,)>( // raw query — see note above
            "SELECT COUNT(*) FROM phpyun_report WHERE p_uid = ? AND eid = ? AND c_uid = ?",
        )
        .bind(u.uid as i64)
        .bind(id as i64)
        .bind(com_id)
        .fetch_one(db);
        let invite_fut = sqlx::query_as::<_, (i64,)>( // raw query — see note above
            "SELECT COUNT(*) FROM phpyun_yqmb WHERE uid = ? AND did = ? AND status != 0",
        )
        .bind(com_id)
        .bind(u.uid as i64)
        .fetch_one(db);
        let (f, a, r, i) = tokio::join!(fav_fut, apply_fut, report_fut, invite_fut);
        (
            i32::from(f.unwrap_or(false)),
            i32::from(a.unwrap_or(None).is_some()),
            r.map(|(n,)| n as i32).unwrap_or(0),
            i.map(|(n,)| n as i32).unwrap_or(0),
        )
    } else {
        (0, 0, 0, 0)
    };

    // --- Assemble the response grouped by business concern ---
    Ok(json::json!({
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
    }))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RecQuery {
    #[serde(default = "default_rec_limit")]
    #[validate(range(min = 1, max = 30))]
    pub limit: u64,
}
fn default_rec_limit() -> u64 { 6 }

/// Similar jobs (same job1 category)
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/similar",
    tag = "wap",
    request_body = SimilarBody,
    responses((status = 200, description = "ok"))
)]
pub async fn similar_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedJson(b): ValidatedJson<SimilarBody>,
) -> AppResult<ApiJson<Vec<JobSummary>>> {
    let id = b.id;
    let list = job_service::list_similar(&state, id, b.limit.clamp(1, 30)).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(
        list.into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
    ))
}

/// Other jobs from the same company
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/same-company",
    tag = "wap",
    request_body = SimilarBody,
    responses((status = 200, description = "ok"))
)]
pub async fn same_company_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedJson(b): ValidatedJson<SimilarBody>,
) -> AppResult<ApiJson<Vec<JobSummary>>> {
    let id = b.id;
    let list = job_service::list_same_company(&state, id, b.limit.clamp(1, 30)).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = list.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(
        list.into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
    ))
}

/// Public job list for a given company (paginated)
#[utoipa::path(
    post,
    path = "/v1/wap/companies/jobs",
    tag = "wap",
    request_body = UidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn company_jobs(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiJson<Paged<JobSummary>>> {
    let r = job_service::list_by_company(&state, b.uid, page).await?;
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
                crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, fav)
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
    #[validate(range(min = 0, max = 99))]
    pub source: i32,
    /// Optional: when jobid does not exist, use this to locate the company
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub com_id: u64,
}
fn default_source() -> i32 {
    2
}

/// Records a "click on job contact phone" action (aligned with PHPYun `wap/ajax::addJobTelLog_action`)
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/tel-click",
    tag = "wap",
    request_body = TelClickBodyFull,
    responses((status = 200, description = "ok"))
)]
pub async fn log_tel_click(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<TelClickBodyFull>,
) -> AppResult<ApiJson<json::Value>> {
    let viewer_uid = user.as_ref().map(|u| u.uid);
    job_service::log_tel_click(&state, viewer_uid, b.id, b.com_id, b.source, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// ==================== Share text (copyable / Weibo / WeChat) ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct JobShareText {
    pub job_id: u64,
    pub job_name: String,
    pub com_name: String,
    /// `{salary} | {city} | {edu}` — primary one-line summary suitable for
    /// chat / Weibo card. Empty fields are omitted.
    pub summary: String,
    /// Plain-text card the user pastes elsewhere. Pre-rendered with
    /// short URL (`share_url`) appended.
    pub plain_text: String,
    pub share_url: String,
}

/// Get pre-formatted share text — counterpart of PHP `wap/job::getJobWb_action`.
/// PHP renders the text inside the `wxpubtemp` template; we expose the data
/// directly so any client (Weibo / WeChat / clipboard) can reuse it.
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/share-text",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = JobShareText),
        (status = 404, description = "Job not found"),
    )
)]
pub async fn share_text(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<JobShareText>> {
    let id = b.id;
    let job = phpyun_models::job::repo::find_public_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| {
            phpyun_core::AppError::new(phpyun_core::InfraError::InvalidParam("job_not_found".into()))
        })?;

    let jid = job.id;
    let job_name = job.name.clone();
    let com_name = job.com_name.clone().unwrap_or_default();
    let minsalary = job.minsalary;
    let maxsalary = job.maxsalary;
    let prov_id = job.provinceid;

    let dicts = phpyun_services::dict_service::get(&state).await?;
    // Salary: derive from the explicit numeric range. PHPYun's legacy `salary`
    // bucket column was dropped from the schema we observe today.
    let salary = if minsalary > 0 && maxsalary > 0 && maxsalary != minsalary {
        format!("{minsalary}-{maxsalary}元")
    } else if minsalary > 0 {
        format!("{minsalary}元以上")
    } else {
        String::new()
    };
    let city = dicts.city(prov_id).to_string();

    let mut parts = Vec::with_capacity(2);
    if !salary.is_empty() {
        parts.push(salary.clone());
    }
    if !city.is_empty() {
        parts.push(city.clone());
    }
    let summary = parts.join(" | ");

    let share_url = state
        .config
        .web_base_url
        .as_deref()
        .map(|b| format!("{b}/wap/job/comapply?id={jid}"))
        .unwrap_or_else(|| format!("/wap/job/comapply?id={jid}"));

    let plain_text = format!(
        "【{}】{}：{}\n{}",
        com_name,
        job_name,
        if summary.is_empty() {
            "查看详情"
        } else {
            summary.as_str()
        },
        share_url
    );

    Ok(ApiJson(JobShareText {
        job_id: jid,
        job_name,
        com_name,
        summary,
        plain_text,
        share_url,
    }))
}

// ==================== Job hits counter ====================

/// Bump and return the new job-hit count. Counterpart of PHP
/// `wap/job::GetHits_action` (which echoes a `document.write(N)` snippet;
/// we return clean JSON). The hit goes to `phpyun_company_job.jobhits`.
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/hits",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = HitsResp))
)]
pub async fn bump_jobhits(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<HitsResp>> {
    let hits = phpyun_models::job::repo::bump_and_get_jobhits(state.db.pool(), b.id).await?;
    Ok(ApiJson(HitsResp { hits }))
}

// ==================== Job contact reveal (getJobLink) ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct JobContactView {
    pub job_id: u64,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub address: String,
    pub city_id: i32,
    pub city_name: String,
    /// Longitude (BD-09 normalised same way as the company detail endpoint).
    pub x: String,
    /// Latitude
    pub y: String,
}

/// Resolve the contact info for a single job. Counterpart of PHP
/// `app/job/comapply::getJobLink_action`. The job row's `is_link` field
/// selects between the company's default contact (1), the per-job alternate
/// (`company_job_link`, with fallback to default — 2), and the alternate
/// without fallback (3). 404 when the job is missing.
#[utoipa::path(
    post,
    path = "/v1/wap/jobs/contact",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = JobContactView),
        (status = 404, description = "Job not found"),
    )
)]
pub async fn job_contact(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<JobContactView>> {
    let id = b.id;
    let c = phpyun_models::job::repo::get_job_contact(state.db.reader(), id)
        .await?
        .ok_or_else(|| phpyun_core::AppError::param_invalid("job_not_found"))?;

    // Resolve city name from the dict cache so the front-end doesn't need a
    // second round-trip just to render it.
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let city_name = dicts.city(c.cityid).to_string();

    Ok(ApiJson(JobContactView {
        job_id: id,
        linkman: c.linkman,
        linktel: c.linktel,
        linkphone: c.linkphone,
        linkmail: c.linkmail,
        address: c.address,
        city_id: c.cityid,
        city_name,
        x: c.x,
        y: c.y,
    }))
}
