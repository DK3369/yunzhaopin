//! Public company browsing (mirrors PHPYun `wap/company::index_action` + `show_action`).

use axum::{
    extract::State,
    http::HeaderMap,
    routing::{get, post},
    Router,
};
use phpyun_core::dto::{CreatedId, UidBody};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    verify::{self, VerifyKind},
    ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, MaybeUser, Paged, Pagination,
    ValidatedJson, ValidatedJsonOrQuery,
};
use phpyun_models::company::repo::CompanyFilter;
use phpyun_services::company_service;
use phpyun_services::hot_search_service;
use phpyun_services::job_msg_service::{self, CreateCompanyInput};
use phpyun_services::job_service;
use phpyun_services::view_service::{self, KIND_COMPANY};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[
    "/v1/wap/companies",
    "/v1/wap/companies/hot",
    "/v1/wap/companies/autocomplete",
    "/v1/wap/companies/detail",
    "/v1/wap/companies/contact",
];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies", get(list_companies).post(list_companies))
        .route("/companies/hot", get(hot_companies).post(hot_companies))
        .route(
            "/companies/autocomplete",
            get(autocomplete).post(autocomplete),
        )
        .route(
            "/companies/detail",
            get(company_detail).post(company_detail),
        )
        .route(
            "/companies/contact",
            get(company_contact).post(company_contact),
        )
        .route("/companies/messages", post(list_company_messages))
        .route("/companies/messages/post", post(create_company_message))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct CompanyListQuery {
    /// Matched against `name` AND `shortname` via LIKE (mirrors PHP `comlist`).
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: Option<i32>,
    /// Industry dict id (PHP `hy`).
    #[validate(range(min = 0, max = 9_999_999))]
    pub hy: Option<i32>,
    /// Company-type dict id — 国企/外资/民营/… (PHP `pr`).
    #[validate(range(min = 0, max = 99))]
    pub pr: Option<i32>,
    /// Staff-count dict id — 50人以下/50-200/… (PHP `mun`).
    #[validate(range(min = 0, max = 99))]
    pub mun: Option<i32>,
    /// Welfare dict id (FIND_IN_SET on `phpyun_company.welfare`).
    #[validate(range(min = 0, max = 9_999_999))]
    pub welfare: Option<i32>,
    /// `cert=true` keeps only companies with a verified business license
    /// (`yyzz_status = 1`).
    #[serde(default)]
    pub cert: bool,
    /// `rec=true` keeps only sticky/promoted companies (PHP composite:
    /// `rec=1 AND hotstart <= now AND hottime > now`).
    #[serde(default)]
    pub rec: bool,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
    #[validate(range(min = 0, max = 3650))]
    pub uptime: Option<i32>,
}
fn default_did() -> u32 {
    0
}

/// Company list item — definition lives in `phpyun_models::company::view`,
/// re-exported here so the legacy path keeps working. See
/// `company_summary_from_dict` for the dict-aware constructor.
pub use phpyun_models::company::view::CompanySummary;

/// Build a fully-populated `CompanySummary` (dict-translated names).
pub fn company_summary_from_dict(
    c: phpyun_models::company::entity::Company,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) -> CompanySummary {
    CompanySummary {
        uid: c.uid,
        name: c.name,
        shortname: c.shortname,

        hy: c.hy,
        pr: c.pr,
        mun: c.mun,
        province_id: c.provinceid,
        city_id: c.cityid,

        hy_n: dicts.industry(c.hy).to_string(),
        // pr / mun live in phpyun_comclass (grouped by keyid)
        pr_n: dicts.comclass(c.pr).to_string(),
        mun_n: dicts.comclass(c.mun).to_string(),
        city_one: dicts.city(c.provinceid).to_string(),
        city_two: dicts.city(c.cityid).to_string(),

        logo: c.logo,
        hot_pic: None,
        rec: c.rec,
        hits: c.hits,
        rating: c.rating,
        rating_name: c.rating_name,
        job_num: 0,
        yyzz_status: c.yyzz_status,
        fact_status: c.fact_status,
        ant_num: c.ant_num,
        isatn: 0,
        welfare_n: c
            .welfare
            .as_deref()
            .map(|s| dicts.welfare_labels(s))
            .unwrap_or_default(),
        open_jobs: Vec::new(),
    }
}

/// Fill `job_num` on list cards (one grouped COUNT). Best-effort: leave 0 on error.
pub async fn fill_job_nums(state: &AppState, list: &mut [CompanySummary]) {
    let uids: Vec<u64> = list.iter().map(|c| c.uid).collect();
    let Ok(rows) =
        phpyun_models::company::repo::count_open_jobs_by_uids(state.db.reader(), &uids).await
    else {
        return;
    };
    let map: std::collections::HashMap<u64, u64> = rows.into_iter().collect();
    for row in list {
        row.job_num = map.get(&row.uid).copied().unwrap_or(0);
    }
}

/// Fill up to `max_per` open job names per company. Best-effort.
pub async fn fill_open_jobs(state: &AppState, list: &mut [CompanySummary], max_per: usize) {
    let uids: Vec<u64> = list.iter().map(|c| c.uid).collect();
    let Ok(rows) =
        phpyun_models::company::repo::list_open_job_briefs_by_uids(state.db.reader(), &uids).await
    else {
        return;
    };
    let cap = max_per.max(1);
    let mut map: std::collections::HashMap<u64, Vec<phpyun_models::company::view::CompanyOpenJob>> =
        std::collections::HashMap::new();
    for r in rows {
        let bucket = map.entry(r.uid).or_default();
        if bucket.len() >= cap {
            continue;
        }
        bucket.push(phpyun_models::company::view::CompanyOpenJob {
            id: r.id,
            name: r.name,
        });
    }
    for row in list {
        row.open_jobs = map.remove(&row.uid).unwrap_or_default();
    }
}

async fn fill_isatn(
    state: &AppState,
    user: Option<&phpyun_core::AuthenticatedUser>,
    list: &mut [CompanySummary],
) {
    let Some(u) = user.filter(|u| u.usertype == phpyun_core::extractors::USERTYPE_JOBSEEKER) else {
        return;
    };
    let uids: Vec<u64> = list.iter().map(|c| c.uid).collect();
    let Ok(set) = phpyun_models::atn::repo::followed_sc_uids(state.db.reader(), u.uid, &uids).await
    else {
        return;
    };
    for row in list {
        row.isatn = i32::from(set.contains(&row.uid));
    }
}

/// Public company list (filter by keyword / region / industry)
#[utoipa::path(
    post,
    path = "/v1/wap/companies",
    tag = "wap",
    params(CompanyListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_companies(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    headers: HeaderMap,
    page: Pagination,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<CompanyListQuery>,
) -> AppResult<ApiResponse<Paged<CompanySummary>>> {
    phpyun_services::site_gate_service::ensure_list_login(
        &state,
        user.as_ref(),
        &crate::v1::wap::request_user_agent(&headers),
    )
    .await?;
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "company", kw.trim().to_string());
    }
    let filter = CompanyFilter {
        keyword: q.keyword.as_deref(),
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        hy: q.hy,
        pr: q.pr,
        mun: q.mun,
        welfare: q.welfare,
        cert: q.cert,
        rec: q.rec,
        did: q.did,
        uptime: {
            let explicit = q.uptime;
            phpyun_services::site_gate_service::default_uptime_days(
                &state,
                explicit,
                "sy_datacycle_com",
            )
            .await
        },
    };
    let r = company_service::list_public(&state, &filter, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let mut list: Vec<CompanySummary> = r
        .list
        .into_iter()
        .map(|c| crate::v1::wap::companies::company_summary_from_dict(c, &dicts))
        .collect();
    fill_job_nums(&state, &mut list).await;
    fill_open_jobs(&state, &mut list, 10).await;
    fill_isatn(&state, user.as_ref(), &mut list).await;
    Ok(ApiResponse::data(Paged::new(
        list,
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Company detail -- strictly mirrors the field set of PHPYun `wap/company::show_action`.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompanyDetail {
    pub uid: u64,
    pub name: Option<String>,
    pub shortname: Option<String>,

    // ---- Industry / size / nature ----
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub sdate: Option<String>,
    pub money: i32,
    pub moneytype: i32,

    // ---- Dict names (mirrors PHP `hy_n / pr_n / mun_n / city_one / city_two`) ----
    pub hy_n: String,
    pub pr_n: String,
    pub mun_n: String,

    // ---- Address ----
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub city_one: String,
    pub city_two: String,
    pub address: Option<String>,
    pub zip: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,

    // ---- Contact info ----
    pub linkman: Option<String>,
    pub linkjob: Option<String>,
    #[serde(skip_serializing)]
    pub linkqq: Option<String>,
    #[serde(skip_serializing)]
    pub linkphone: Option<String>,
    #[serde(skip_serializing)]
    pub linktel: Option<String>,
    #[serde(skip_serializing)]
    pub linkmail: Option<String>,
    pub website: Option<String>,

    // ---- Images ----
    pub logo: Option<String>,
    pub logo_status: i32,
    pub firmpic: Option<String>,
    pub comqcode: Option<String>,

    // ---- Body ----
    pub content: Option<String>,

    // ---- Status ----
    pub r_status: i32,
    pub rec: i32,
    pub hits: i32,
    pub expoure: i32,
    pub moblie_status: i32,
    pub email_status: i32,
    pub yyzz_status: i32,
    pub fact_status: i32,

    // ---- Membership tier / VIP / finance ----
    pub rating: i32,
    pub rating_name: Option<String>,
    pub vipstime: i64,
    pub vipetime: i64,
    /// Total amount paid (PHPYun `payd`)
    pub payd: i32,
    /// Points balance (PHPYun `integral`)
    pub integral: i32,

    // ---- Time ----
    pub lastupdate: Option<String>,
    pub addtime: i64,
    /// addtime formatted (Y-m-d H:i)
    pub addtime_n: String,
    pub login_date: i64,
    /// login_date formatted
    pub login_date_n: String,

    // ---- Sub-site ----
    /// Sub-site id (PHPYun `did`)
    pub did: u64,

    // ---- Computed fields ----
    /// Number of currently open positions (PHP: `jobM->getJobNum(...)`)
    pub zp_num: u64,
    /// Whether VIP is currently active (vipetime > now())
    pub vip_active: bool,

    // ---- Current-user context (0 when unauthenticated) ----
    /// Whether the current jobseeker follows this company (PHP `isatn`)
    pub isatn: i32,
    /// Followers (PHP `ant_num`).
    pub ant_num: i32,
    /// How many times the current jobseeker has applied to this company (PHP `userid_job`)
    pub userid_job: i32,

    /// PHP `$com.welfare_n` tags. Additive.
    pub welfare_n: Vec<String>,
    /// PHP `$invite_resume` (面试邀请数). Additive.
    pub invite_resume: u64,
    /// Reply rate `(1 - unread/total)*100`.
    pub pre: i32,
    /// `source==6 && claim==0 && email`.
    pub claimable: i32,

    /// Masked contact + `setCompanyLink` codes. Never includes plaintext tel/email.
    pub contact: CompanyPublicContact,

    // ---- Company showcase items (PHP `show[]` from `phpyun_company_show`) ----
    pub show: Vec<CompanyShowItem>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompanyShowItem {
    pub id: u64,
    pub title: String,
    pub picurl: String,
    pub body: String,
    pub sort: i32,
    pub ctime: i64,
}

/// Public company contact — same codes as job `setCompanyLink`, no email.
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct CompanyPublicContact {
    pub linkman: String,
    pub linktel_n: String,
    pub linkphone_n: String,
    pub address: String,
    pub link_code: i32,
    pub link_msg: String,
    pub link_sub: i32,
    pub prvlinktel: String,
    pub prvtime: String,
}

/// Public company detail
#[utoipa::path(
    post,
    path = "/v1/wap/companies/detail",
    tag = "wap",
    request_body = UidBody,
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Company not approved / account locked"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn company_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<UidBody>,
) -> AppResult<ApiResponse<CompanyDetail>> {
    let uid = b.uid;
    let c = company_service::get_public(&state, uid, user.as_ref()).await?;
    if let Some(u) = user.as_ref() {
        view_service::record_async(&state, u.uid, KIND_COMPANY, uid);
    }
    // Number of currently open positions (PHP equivalent: `jobM->getJobNum(['uid'=>uid,'state'=>1,'status'=>0,'r_status'=>1])`)
    let zp_num = phpyun_models::company::repo::count_open_jobs(state.db.reader(), uid)
        .await
        .unwrap_or(0);
    let invite_resume = phpyun_models::company::repo::count_interview_invites(state.db.reader(), uid)
        .await
        .unwrap_or(0);
    let apply_total = phpyun_models::apply::repo::count_by_com(
        state.db.reader(),
        uid,
        phpyun_models::apply::repo::ApplyFilter::default(),
    )
    .await
    .unwrap_or(0);
    let apply_unread = phpyun_models::apply::repo::count_unread_by_company(state.db.reader(), uid)
        .await
        .unwrap_or(0);
    let pre = if apply_total == 0 {
        100
    } else {
        (((1.0 - (apply_unread as f64 / apply_total as f64)) * 100.0).round() as i32).clamp(0, 100)
    };
    let claimable = match phpyun_models::user::repo::claim_eligibility(state.db.reader(), uid).await
    {
        Ok(Some((6, 0, true))) => 1,
        _ => 0,
    };
    // Bump hit + expoure counters (fire-and-forget — page renders even if write fails).
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("company.hits", async move {
        let _ = phpyun_models::company::repo::incr_hits_and_expoure(&pool, uid).await;
    });
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let hy_n = dicts.industry(c.hy).to_string();
    let pr_n = dicts.comclass(c.pr).to_string();
    let mun_n = dicts.comclass(c.mun).to_string();
    let city_one = dicts.city(c.provinceid).to_string();
    let city_two = dicts.city(c.cityid).to_string();
    let welfare_n = c
        .welfare
        .as_deref()
        .map(|s| dicts.welfare_labels(s))
        .unwrap_or_default();

    // Company showcase items (phpyun_company_show, status=0 means active)
    let show_items: Vec<CompanyShowItem> =
        phpyun_models::company::repo::list_show_items(state.db.reader(), uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|r| CompanyShowItem {
                id: r.id,
                title: r.title.unwrap_or_default(),
                picurl: r.picurl.unwrap_or_default(),
                body: r.body.unwrap_or_default(),
                sort: r.sort,
                ctime: r.ctime,
            })
            .collect();

    // From the logged-in jobseeker's perspective: follow flag + number of applications
    let (isatn, userid_job) = if let Some(u) = user.as_ref() {
        let db = state.db.reader();
        let atn_fut = phpyun_models::atn::repo::exists_pair(db, u.uid, uid);
        let apply_fut = phpyun_models::apply::repo::count_by_uid_to_company(db, u.uid, uid);
        let (a, b) = tokio::join!(atn_fut, apply_fut);
        (
            a.map(|x| if x { 1 } else { 0 }).unwrap_or(0),
            b.map(phpyun_core::numeric::saturating_count_i32)
                .unwrap_or(0),
        )
    } else {
        (0, 0)
    };

    Ok(ApiResponse::data(CompanyDetail {
        uid: c.uid,
        name: c.name,
        shortname: c.shortname,

        hy: c.hy,
        pr: c.pr,
        mun: c.mun,
        sdate: c.sdate,
        money: c.money,
        moneytype: c.moneytype,

        hy_n,
        pr_n,
        mun_n,

        province_id: c.provinceid,
        city_id: c.cityid,
        three_city_id: c.three_cityid,
        city_one,
        city_two,
        address: c.address,
        zip: c.zip,
        x: c.x,
        y: c.y,

        linkman: c.linkman,
        linkjob: c.linkjob,
        linkqq: None,
        linkphone: None,
        linktel: None,
        linkmail: None,
        website: c.website,

        logo: c.logo,
        logo_status: c.logo_status,
        firmpic: c.firmpic,
        comqcode: c.comqcode,

        content: c.content,

        r_status: c.r_status,
        rec: c.rec,
        hits: c.hits,
        expoure: c.expoure,
        moblie_status: c.moblie_status,
        email_status: c.email_status,
        yyzz_status: c.yyzz_status,
        fact_status: c.fact_status,

        rating: c.rating,
        rating_name: c.rating_name,
        vipstime: c.vipstime,
        vipetime: c.vipetime,
        payd: c.payd,
        integral: c.integral,

        lastupdate: c.lastupdate,
        addtime_n: fmt_dt(c.addtime),
        addtime: c.addtime,
        login_date_n: fmt_dt(c.login_date),
        login_date: c.login_date,

        did: c.did,

        zp_num,
        vip_active: c.vipetime > phpyun_core::clock::now_ts(),

        isatn,
        ant_num: c.ant_num,
        userid_job,
        welfare_n,
        invite_resume,
        pre,
        claimable,
        contact: {
            let ctc = job_service::resolve_company_contact(&state, uid, user.as_ref(), false).await?;
            CompanyPublicContact {
                linkman: ctc.linkman,
                linktel_n: ctc.linktel_n,
                linkphone_n: ctc.linkphone_n,
                address: ctc.address,
                link_code: ctc.link_code,
                link_msg: ctc.link_msg,
                link_sub: ctc.link_sub,
                prvlinktel: ctc.prvlinktel,
                prvtime: ctc.prvtime,
            }
        },

        show: show_items,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompanyContactView {
    pub uid: u64,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linktel_n: String,
    pub linkphone_n: String,
    pub address: String,
    pub link_code: i32,
    pub link_msg: String,
    pub link_sub: i32,
    pub revealed: bool,
    pub prvlinktel: String,
    pub prvtime: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct CompanyContactQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub isgetprv: Option<i32>,
}

/// Reveal company telephone when `setCompanyLink` yields code 1. Never returns email.
#[utoipa::path(
    post,
    path = "/v1/wap/companies/contact",
    tag = "wap",
    request_body = CompanyContactQuery,
    responses((status = 200, description = "ok", body = CompanyContactView))
)]
pub async fn company_contact(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<CompanyContactQuery>,
) -> AppResult<ApiResponse<CompanyContactView>> {
    let isgetprv = b.isgetprv.unwrap_or(0) == 1;
    let c = job_service::resolve_company_contact(&state, b.uid, user.as_ref(), isgetprv).await?;
    let plain = c.revealed && c.link_code == 1;
    Ok(ApiResponse::data(CompanyContactView {
        uid: b.uid,
        linkman: c.linkman,
        linktel: if plain {
            c.linktel
        } else {
            String::new()
        },
        linkphone: if plain {
            c.linkphone
        } else {
            String::new()
        },
        linktel_n: c.linktel_n,
        linkphone_n: c.linkphone_n,
        address: c.address,
        link_code: c.link_code,
        link_msg: c.link_msg,
        link_sub: c.link_sub,
        revealed: plain,
        prvlinktel: c.prvlinktel,
        prvtime: c.prvtime,
    }))
}

// ==================== Hot / featured companies (homepage banner) ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HotCompaniesQuery {
    /// `default` (paid sort, ASC), `recent` (job lastupdate DESC), `random`.
    /// PHPYun config `hotcom_top` maps 0/1/2; we accept friendlier names.
    #[serde(default)]
    #[validate(length(max = 16))]
    pub order: Option<String>,
    #[serde(default = "default_hot_limit")]
    #[validate(range(min = 1, max = 100))]
    pub limit: u32,
}

fn default_hot_limit() -> u32 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HotCompanyView {
    pub uid: u64,
    pub name: String,
    pub shortname: Option<String>,
    pub logo: Option<String>,
    /// CDN-resolved logo URL (alongside the raw column for legacy clients).
    pub logo_n: String,
    pub hot_pic: Option<String>,
    pub hot_pic_n: String,
    /// 0=sort ASC, 1=lastupdate DESC, 2=random — echoed back so the client
    /// can short-cache appropriately.
    pub sort_mode: i32,
}

/// Featured companies on the homepage.
///
/// Counterpart of PHP `wap/index::getmq_action` (the "首页名企" widget).
/// Uses an INNER JOIN on `phpyun_hotjob` × `phpyun_company`, filtered by
/// `c.hottime > now AND c.r_status = 1 AND h.time_start < now AND h.time_end > now`.
#[utoipa::path(
    post,
    path = "/v1/wap/companies/hot",
    tag = "wap",
    params(HotCompaniesQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn hot_companies(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<HotCompaniesQuery>,
) -> AppResult<ApiResponse<Vec<HotCompanyView>>> {
    let sort_mode = match q.order.as_deref() {
        Some("recent") => 1,
        Some("random") => 2,
        _ => 0,
    };
    let limit = u64::from(q.limit.clamp(1, 50));
    let now = phpyun_core::clock::now_ts();

    let rows =
        phpyun_models::company::repo::list_hot(state.db.reader(), sort_mode, limit, now, None).await?;
    let web_base = state.config.web_base_url.as_deref();
    let storage = &state.storage;
    let out: Vec<HotCompanyView> = rows
        .into_iter()
        .map(|c| {
            let logo_n = storage.normalize_legacy_url(c.logo.as_deref().unwrap_or(""), web_base);
            let hot_pic_n =
                storage.normalize_legacy_url(c.hot_pic.as_deref().unwrap_or(""), web_base);
            HotCompanyView {
                uid: c.uid,
                name: c.name,
                shortname: c.shortname,
                logo: c.logo,
                logo_n,
                hot_pic: c.hot_pic,
                hot_pic_n,
                sort_mode: c.sort_mode,
            }
        })
        .collect();
    Ok(ApiResponse::data(out))
}

// ==================== Autocomplete ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct CompanyAutoQuery {
    /// Free-text fragment to match against `phpyun_company.name`. Required;
    /// empty input returns an empty list (mirrors PHP behaviour).
    #[validate(length(min = 1, max = 100))]
    pub keyword: String,
    #[serde(default = "default_auto_limit")]
    #[validate(range(min = 1, max = 20))]
    pub limit: u32,
}

fn default_auto_limit() -> u32 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompanyAutoItem {
    /// `uid` of the company — pass to `GET /v1/wap/companies/{uid}` to load
    /// the full record.
    pub value: u64,
    /// Display string for the picker (raw company name).
    pub name: String,
    /// CDN-resolved logo (empty string when the row has no logo). Optional —
    /// PHP returns only `(name, value)`, but the Rust port adds `logo_n` since
    /// many UIs display a tiny logo next to the name.
    pub logo_n: String,
}

/// Lightweight company name autocomplete — counterpart of PHP
/// `ajax::getComBySearch_action`. Designed for typeahead widgets, returns up
/// to `limit` rows (clamped to 1..=20) where `name` matches `LIKE %keyword%`.
#[utoipa::path(
    post,
    path = "/v1/wap/companies/autocomplete",
    tag = "wap",
    params(CompanyAutoQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn autocomplete(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<CompanyAutoQuery>,
) -> AppResult<ApiResponse<Vec<CompanyAutoItem>>> {
    let keyword = q.keyword.trim();
    if keyword.is_empty() {
        return Ok(ApiResponse::data(Vec::new()));
    }
    let limit = u64::from(q.limit.clamp(1, 20));
    let rows =
        phpyun_models::company::repo::search_brief(state.db.reader(), keyword, limit).await?;
    let web_base = state.config.web_base_url.as_deref();
    let storage = &state.storage;
    let out: Vec<CompanyAutoItem> = rows
        .into_iter()
        .map(|c| CompanyAutoItem {
            value: c.uid,
            name: c.name,
            logo_n: storage.normalize_legacy_url(c.logo.as_deref().unwrap_or(""), web_base),
        })
        .collect();
    Ok(ApiResponse::data(out))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CompanyMessageForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[validate(length(min = 1, max = 4000))]
    pub content: String,
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub authcode: String,
}

/// Public answered Q&A for a company page (PHP company show `msgList`).
#[utoipa::path(
    post,
    path = "/v1/wap/companies/messages",
    tag = "wap",
    request_body = UidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_company_messages(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiResponse<Paged<super::job_messages::JobMsgView>>> {
    let r = job_msg_service::list_public_for_company(&state, b.uid, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

/// Jobseeker leaves a public message on a company page — login + image captcha.
#[utoipa::path(
    post,
    path = "/v1/wap/companies/messages/post",
    tag = "wap",
    security(("bearer" = [])),
    request_body = CompanyMessageForm,
    responses(
        (status = 200, description = "Created", body = CreatedId),
        (status = 400, description = "Validation / captcha / blocked / company not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Only jobseekers may post messages"),
    )
)]
pub async fn create_company_message(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CompanyMessageForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let code = f.authcode.to_uppercase();
    if !verify::verify(
        &state.redis,
        VerifyKind::ImageCaptcha,
        &f.captcha_cid,
        &code,
    )
    .await?
    {
        return Err(ApiError::captcha());
    }

    let mid = job_msg_service::create_for_company(
        &state,
        &user,
        CreateCompanyInput {
            job_uid: f.uid,
            content: &f.content,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id: mid }))
}
