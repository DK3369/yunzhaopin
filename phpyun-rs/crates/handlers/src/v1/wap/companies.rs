//! Public company browsing (mirrors PHPYun `wap/company::index_action` + `show_action`).

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, MaybeUser, Paged, Pagination};
use phpyun_models::company::repo::CompanyFilter;
use phpyun_services::company_service;
use phpyun_services::hot_search_service;
use phpyun_services::view_service::{self, KIND_COMPANY};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies", get(list_companies))
        .route("/companies/{uid}", get(company_detail))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct CompanyListQuery {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    /// Industry id
    pub hy: Option<i32>,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_did() -> u32 {
    1
}

/// Company list item -- mirrors the output of PHPYun `companyM::getList()`.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompanySummary {
    pub uid: u64,
    pub name: Option<String>,
    pub shortname: Option<String>,

    // ---- Raw ids ----
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub province_id: i32,
    pub city_id: i32,

    // ---- Dict names (PHP `hy_n / pr_n / mun_n / city_one / city_two`) ----
    pub hy_n: String,
    pub pr_n: String,
    pub mun_n: String,
    pub city_one: String,
    pub city_two: String,

    // ---- Other ----
    pub logo: Option<String>,
    pub rec: i32,
    pub hits: i32,
    pub rating: i32,
    pub rating_name: Option<String>,
}

impl CompanySummary {
    pub fn from_with_dict(
        c: phpyun_models::company::entity::Company,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
    ) -> Self {
        Self {
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
            rec: c.rec,
            hits: c.hits,
            rating: c.rating,
            rating_name: c.rating_name,
        }
    }
}

/// Public company list (filter by keyword / region / industry)
#[utoipa::path(
    get,
    path = "/v1/wap/companies",
    tag = "wap",
    params(CompanyListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_companies(
    State(state): State<AppState>,
    page: Pagination,
    Query(q): Query<CompanyListQuery>,
) -> AppResult<ApiJson<Paged<CompanySummary>>> {
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "company", kw.trim().to_string());
    }
    let filter = CompanyFilter {
        keyword: q.keyword.as_deref(),
        province_id: q.province_id,
        city_id: q.city_id,
        hy: q.hy,
        did: q.did,
    };
    let r = company_service::list_public(&state, &filter, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(|c| CompanySummary::from_with_dict(c, &dicts)).collect(),
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
    pub linkqq: Option<String>,
    pub linkphone: Option<String>,
    pub linktel: Option<String>,
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
    /// How many times the current jobseeker has applied to this company (PHP `userid_job`)
    pub userid_job: i32,

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

/// Public company detail
#[utoipa::path(
    get,
    path = "/v1/wap/companies/{uid}",
    tag = "wap",
    params(("uid" = u64, Path)),
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Company not approved / account locked"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn company_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(uid): Path<u64>,
) -> AppResult<ApiJson<CompanyDetail>> {
    let c = company_service::get_public(&state, uid).await?;
    if let Some(u) = user.as_ref() {
        view_service::record_async(&state, u.uid, KIND_COMPANY, uid);
    }
    // Number of currently open positions (PHP equivalent: `jobM->getJobNum(['uid'=>uid,'state'=>1,'status'=>0,'r_status'=>1])`)
    let zp_num: u64 = {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM phpyun_company_job \
             WHERE uid = ? AND state = 1 AND status = 0 AND r_status = 1 \
               AND (edate = 0 OR edate > UNIX_TIMESTAMP())",
        )
        .bind(uid as i64)
        .fetch_one(state.db.reader())
        .await
        .unwrap_or((0,));
        row.0.max(0) as u64
    };
    // Bump hit counter (+1, fire-and-forget)
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("company.hits", async move {
        let _ = sqlx::query(
            "UPDATE phpyun_company SET hits = hits + 1, expoure = expoure + 1 WHERE uid = ?",
        )
        .bind(uid as i64)
        .execute(&pool)
        .await;
    });
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let hy_n = dicts.industry(c.hy).to_string();
    let pr_n = dicts.comclass(c.pr).to_string();
    let mun_n = dicts.comclass(c.mun).to_string();
    let city_one = dicts.city(c.provinceid).to_string();
    let city_two = dicts.city(c.cityid).to_string();

    // Company showcase items (phpyun_company_show, status=0 means active)
    let show_items: Vec<CompanyShowItem> = {
        let rows: Vec<(i64, Option<String>, Option<String>, Option<String>, i64, i64)> =
            sqlx::query_as(
                "SELECT id, title, picurl, body, \
                        CAST(COALESCE(sort, 0) AS SIGNED), \
                        CAST(COALESCE(ctime, 0) AS SIGNED) \
                 FROM phpyun_company_show \
                 WHERE uid = ? AND status = 0 \
                 ORDER BY sort ASC, id ASC",
            )
            .bind(uid as i64)
            .fetch_all(state.db.reader())
            .await
            .unwrap_or_default();
        rows.into_iter()
            .map(|(id, t, p, b, s, ct)| CompanyShowItem {
                id: id as u64,
                title: t.unwrap_or_default(),
                picurl: p.unwrap_or_default(),
                body: b.unwrap_or_default(),
                sort: s as i32,
                ctime: ct,
            })
            .collect()
    };

    // From the logged-in jobseeker's perspective: follow flag + number of applications
    let (isatn, userid_job) = if let Some(u) = user.as_ref() {
        let db = state.db.reader();
        let uid_i = u.uid as i64;
        let com_uid = uid as i64;
        let atn_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_atn WHERE uid = ? AND sc_uid = ?",
        )
        .bind(uid_i)
        .bind(com_uid)
        .fetch_one(db);
        let apply_fut = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM phpyun_userid_job WHERE uid = ? AND com_id = ? AND isdel = 9",
        )
        .bind(uid_i)
        .bind(com_uid)
        .fetch_one(db);
        let (a, b) = tokio::join!(atn_fut, apply_fut);
        (
            a.map(|(n,)| if n > 0 { 1 } else { 0 }).unwrap_or(0),
            b.map(|(n,)| n as i32).unwrap_or(0),
        )
    } else {
        (0, 0)
    };

    Ok(ApiJson(CompanyDetail {
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
        linkqq: c.linkqq,
        linkphone: c.linkphone,
        linktel: c.linktel,
        linkmail: c.linkmail,
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
        addtime_n: chrono::DateTime::from_timestamp(c.addtime, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default(),
        addtime: c.addtime,
        login_date_n: chrono::DateTime::from_timestamp(c.login_date, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default(),
        login_date: c.login_date,

        did: c.did,

        zp_num,
        vip_active: c.vipetime > phpyun_core::clock::now_ts(),

        isatn,
        userid_job,

        show: show_items,
    }))
}
