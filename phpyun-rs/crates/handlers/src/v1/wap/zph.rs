//! Public browsing of job fairs (mirrors PHPYun `wap/zph`).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::zph_service;
use serde::Serialize;
use utoipa::ToSchema;



pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/zph", post(list))
        .route("/zph/detail", post(detail))
        .route("/zph/companies", post(list_companies))
        .route("/zph/jobs", post(list_jobs))
}

/// Job-fair list item -- mirrors all phpyun_zhaopinhui columns + city name + CDN URL + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct ZphSummary {
    pub id: u64,
    pub title: String,
    pub sid: i32,
    pub body: String,

    // ---- Visual assets ----
    pub banner: String,
    pub banner_n: String,
    pub banner_wap: String,
    pub banner_wap_n: String,
    pub pic: String,
    pub pic_n: String,
    pub zwpic: String,
    pub is_themb_wap: String,

    // ---- Location ----
    pub province_id: i32,
    pub city_id: i32,
    /// City name (dict resolve_city)
    pub city_name: String,
    /// Province name
    pub province_name: String,
    pub address: String,
    pub traffic: String,

    // ---- Time ----
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub created_at: i64,

    // ---- Contact ----
    pub phone: String,
    pub organizers: String,
    pub user: String,
    pub weburl: String,

    // ---- Status ----
    pub is_open: i32,
    pub status: i32,
    pub sort: i32,
}

impl ZphSummary {
    pub fn from_with_dict(
        z: phpyun_models::zph::entity::Zph,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
    ) -> Self {
        let banner_n = pic_n(state, &z.banner);
        let banner_wap_n = pic_n(state, &z.banner_wap);
        let pic_full = pic_n(state, &z.pic);
        let city_name = dicts.city(z.city_id).to_string();
        let province_name = dicts.city(z.province_id).to_string();
        Self {
            id: z.id,
            title: z.title,
            sid: z.sid,
            body: z.body,
            banner_n,
            banner: z.banner,
            banner_wap_n,
            banner_wap: z.banner_wap,
            pic_n: pic_full,
            pic: z.pic,
            zwpic: z.zwpic,
            is_themb_wap: z.is_themb_wap,
            province_id: z.province_id,
            city_id: z.city_id,
            city_name,
            province_name,
            address: z.address,
            traffic: z.traffic,
            start_at_n: fmt_dt(z.start_at),
            start_at: z.start_at,
            end_at_n: fmt_dt(z.end_at),
            end_at: z.end_at,
            created_at: z.created_at,
            phone: z.phone,
            organizers: z.organizers,
            user: z.user,
            weburl: z.weburl,
            is_open: z.is_open,
            status: z.status,
            sort: z.sort,
        }
    }
}

/// Compatibility with legacy callers (no dict/CDN context; city name etc. are left empty)
impl From<phpyun_models::zph::entity::Zph> for ZphSummary {
    fn from(z: phpyun_models::zph::entity::Zph) -> Self {
        Self {
            id: z.id,
            title: z.title,
            sid: z.sid,
            body: z.body,
            banner: z.banner.clone(),
            banner_n: z.banner,
            banner_wap: z.banner_wap.clone(),
            banner_wap_n: z.banner_wap,
            pic: z.pic.clone(),
            pic_n: z.pic,
            zwpic: z.zwpic,
            is_themb_wap: z.is_themb_wap,
            province_id: z.province_id,
            city_id: z.city_id,
            city_name: String::new(),
            province_name: String::new(),
            address: z.address,
            traffic: z.traffic,
            start_at_n: fmt_dt(z.start_at),
            start_at: z.start_at,
            end_at_n: fmt_dt(z.end_at),
            end_at: z.end_at,
            created_at: z.created_at,
            phone: z.phone,
            organizers: z.organizers,
            user: z.user,
            weburl: z.weburl,
            is_open: z.is_open,
            status: z.status,
            sort: z.sort,
        }
    }
}

/// Job-fair detail -- mirrors the full field set of PHPYun `wap/zph::show_action` + dict + CDN + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct ZphDetail {
    pub id: u64,
    pub title: String,
    pub sid: i32,
    pub body: String,

    // ---- Visual assets + CDN URL ----
    pub banner: String,
    pub banner_n: String,
    pub banner_wap: String,
    pub banner_wap_n: String,
    pub pic: String,
    pub pic_n: String,
    pub zwpic: String,
    pub zwpic_n: String,
    pub is_themb_wap: String,
    pub reserved: String,

    // ---- Location + dict names ----
    pub province_id: i32,
    pub province_name: String,
    pub city_id: i32,
    pub city_name: String,
    pub address: String,
    pub traffic: String,

    // ---- Time + formatted ----
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,

    // ---- Contact / organizer ----
    pub phone: String,
    pub organizers: String,
    pub user: String,
    pub weburl: String,

    // ---- Rich-text blocks ----
    pub media: String,
    pub packages: String,
    pub booth: String,
    pub participate: String,

    // ---- Status ----
    pub is_open: i32,
    pub status: i32,
    pub sort: i32,
}

impl ZphDetail {
    pub fn from_with_dict(
        z: phpyun_models::zph::entity::Zph,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
    ) -> Self {
        let banner_n = pic_n(state, &z.banner);
        let banner_wap_n = pic_n(state, &z.banner_wap);
        let pic_full = pic_n(state, &z.pic);
        let zwpic_n = pic_n(state, &z.zwpic);
        let city_name = dicts.city(z.city_id).to_string();
        let province_name = dicts.city(z.province_id).to_string();
        Self {
            id: z.id,
            title: z.title,
            sid: z.sid,
            body: z.body,
            banner_n,
            banner: z.banner,
            banner_wap_n,
            banner_wap: z.banner_wap,
            pic_n: pic_full,
            pic: z.pic,
            zwpic_n,
            zwpic: z.zwpic,
            is_themb_wap: z.is_themb_wap,
            reserved: z.reserved,
            province_id: z.province_id,
            province_name,
            city_id: z.city_id,
            city_name,
            address: z.address,
            traffic: z.traffic,
            start_at_n: fmt_dt(z.start_at),
            start_at: z.start_at,
            end_at_n: fmt_dt(z.end_at),
            end_at: z.end_at,
            created_at_n: fmt_date(z.created_at),
            created_at: z.created_at,
            phone: z.phone,
            organizers: z.organizers,
            user: z.user,
            weburl: z.weburl,
            media: z.media,
            packages: z.packages,
            booth: z.booth,
            participate: z.participate,
            is_open: z.is_open,
            status: z.status,
            sort: z.sort,
        }
    }
}

impl From<phpyun_models::zph::entity::Zph> for ZphDetail {
    fn from(z: phpyun_models::zph::entity::Zph) -> Self {
        Self {
            id: z.id,
            title: z.title,
            sid: z.sid,
            body: z.body,
            banner: z.banner.clone(),
            banner_n: z.banner,
            banner_wap: z.banner_wap.clone(),
            banner_wap_n: z.banner_wap,
            pic: z.pic.clone(),
            pic_n: z.pic,
            zwpic: z.zwpic.clone(),
            zwpic_n: z.zwpic,
            is_themb_wap: z.is_themb_wap,
            reserved: z.reserved,
            province_id: z.province_id,
            province_name: String::new(),
            city_id: z.city_id,
            city_name: String::new(),
            address: z.address,
            traffic: z.traffic,
            start_at_n: fmt_dt(z.start_at),
            start_at: z.start_at,
            end_at_n: fmt_dt(z.end_at),
            end_at: z.end_at,
            created_at_n: fmt_date(z.created_at),
            created_at: z.created_at,
            phone: z.phone,
            organizers: z.organizers,
            user: z.user,
            weburl: z.weburl,
            media: z.media,
            packages: z.packages,
            booth: z.booth,
            participate: z.participate,
            is_open: z.is_open,
            status: z.status,
            sort: z.sort,
        }
    }
}

/// Job-fair list
#[utoipa::path(post, path = "/v1/wap/zph", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ZphSummary>>> {
    let r = zph_service::list(&state, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|z| ZphSummary::from_with_dict(z, &state, &dicts))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Job-fair detail
#[utoipa::path(post,
    path = "/v1/wap/zph/detail",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = ZphDetail), (status = 404))
)]
pub async fn detail(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<ZphDetail>> {
    let id = b.id;
    let z = zph_service::get_detail(&state, id).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(ZphDetail::from_with_dict(z, &state, &dicts)))
}

/// Participating-company item -- all phpyun_zph_company columns + JOIN of company basic info (name/logo/hy/pr/mun/city).
#[derive(Debug, Serialize, ToSchema)]
pub struct ZphCompanyItem {
    /// `phpyun_zph_company.id`
    pub id: u64,
    /// Job-fair id
    pub zid: u64,
    pub uid: u64,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    // ---- Key columns from JOIN phpyun_company ----
    pub com_name: Option<String>,
    pub com_logo: Option<String>,
    pub com_logo_n: String,
    pub hy: i32,
    pub hy_n: String,
    pub pr: i32,
    pub pr_n: String,
    pub mun: i32,
    pub mun_n: String,
    pub province_id: i32,
    pub city_id: i32,
    pub city_name: String,
    pub province_name: String,
}

/// Participating-company list
#[utoipa::path(post,
    path = "/v1/wap/zph/companies",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_companies(State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<Paged<ZphCompanyItem>>> {
    let id = b.id;
    let r = zph_service::list_companies(&state, id, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;

    // Fetch all related phpyun_company rows in one query, keyed by uid
    let uids: Vec<u64> = r.list.iter().map(|c| c.uid).collect();
    let cards = phpyun_models::company::repo::list_cards_by_uids(state.db.reader(), &uids)
        .await
        .unwrap_or_default();
    let com_map: std::collections::HashMap<u64, _> =
        cards.into_iter().map(|c| (c.uid, c)).collect();

    let items: Vec<ZphCompanyItem> = r
        .list
        .into_iter()
        .map(|c| {
            let card = com_map.get(&c.uid);
            let com_name = card.and_then(|x| x.name.clone());
            let com_logo = card.and_then(|x| x.logo.clone());
            let hy = card.map(|x| x.hy).unwrap_or(0);
            let pr = card.map(|x| x.pr).unwrap_or(0);
            let mun = card.map(|x| x.mun).unwrap_or(0);
            let prov = card.map(|x| x.provinceid).unwrap_or(0);
            let city = card.map(|x| x.cityid).unwrap_or(0);
            let logo_full =
                pic_n(&state, com_logo.as_deref().unwrap_or(""));
            ZphCompanyItem {
                id: c.id,
                zid: c.zid,
                uid: c.uid,
                sort: c.sort,
                status: c.status,
                created_at_n: fmt_dt(c.created_at),
                created_at: c.created_at,
                com_name,
                com_logo_n: logo_full,
                com_logo,
                hy,
                hy_n: dicts.industry(hy).to_string(),
                pr,
                pr_n: dicts.comclass(pr).to_string(),
                mun,
                mun_n: dicts.comclass(mun).to_string(),
                province_id: prov,
                province_name: dicts.city(prov).to_string(),
                city_id: city,
                city_name: dicts.city(city).to_string(),
            }
        })
        .collect();

    Ok(ApiJson(Paged::new(
        items,
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== Jobs participating in a fair ====================
//
// PHP `app/zph/index::getJobList_action` computes the active job set for a
// fair by reading every signed-up company's `phpyun_zhaopinhui_com.jobid`
// (CSV) and then loading those rows from `phpyun_company_job`. We mirror the
// same shape: parse the CSVs, dedupe, and reuse `JobSummary` so the front
// end can render the same card it uses elsewhere.

use super::jobs::JobSummary;
use phpyun_core::dto::{IdBody};
use phpyun_core::utils::{fmt_date, fmt_dt, pic_n_str as pic_n};

/// Jobs participating in a recruitment fair. Counterpart of PHP
/// `app/zph/index::getJobList_action` — loads the job ids signed up for the
/// fair (`phpyun_zhaopinhui_com.jobid` CSV per company), dedupes, then
/// returns the live job rows that are still on-shelf (`state=1, r_status=1`).
#[utoipa::path(post,
    path = "/v1/wap/zph/jobs",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_jobs(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<Vec<JobSummary>>> {
    let id = b.id;
    let csvs = phpyun_models::zph::repo::jobid_csvs_for_zph(state.db.reader(), id).await?;

    // Flatten + dedupe, preserving first-seen order so the listing roughly
    // matches the PHP "sort DESC, ctime ASC" company ordering.
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();
    let mut job_ids: Vec<u64> = Vec::new();
    for csv in &csvs {
        for piece in csv.split(',') {
            if let Ok(jid) = piece.trim().parse::<u64>() {
                if jid > 0 && seen.insert(jid) {
                    job_ids.push(jid);
                }
            }
        }
    }
    if job_ids.is_empty() {
        return Ok(ApiJson(Vec::new()));
    }

    // Pull live rows in a single query; filter to on-shelf in-memory so we
    // can keep the CSV-derived ordering. The PHP page caps the list at 40 —
    // we keep that cap to avoid exploding the response on busy fairs.
    let mut jobs =
        phpyun_models::job::repo::list_by_ids(state.db.reader(), &job_ids)
            .await
            .unwrap_or_default();
    jobs.retain(|j| j.state == 1 && j.r_status == 1 && j.status == 0);
    let mut by_id: std::collections::HashMap<u64, phpyun_models::job::entity::Job> =
        jobs.into_iter().map(|j| (j.id, j)).collect();

    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();

    let items: Vec<JobSummary> = job_ids
        .into_iter()
        .take(40)
        .filter_map(|jid| by_id.remove(&jid))
        .map(|j| crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, false))
        .collect();

    Ok(ApiJson(items))
}

