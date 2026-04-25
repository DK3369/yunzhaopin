//! Public browsing of job fairs (mirrors PHPYun `wap/zph`).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination};
use phpyun_services::zph_service;
use serde::Serialize;
use utoipa::ToSchema;

fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/zph", get(list))
        .route("/zph/{id}", get(detail))
        .route("/zph/{id}/companies", get(list_companies))
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
#[utoipa::path(get, path = "/v1/wap/zph", tag = "wap", responses((status = 200, description = "ok")))]
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
#[utoipa::path(
    get,
    path = "/v1/wap/zph/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = ZphDetail), (status = 404))
)]
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<ZphDetail>> {
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
#[utoipa::path(
    get,
    path = "/v1/wap/zph/{id}/companies",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list_companies(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ZphCompanyItem>>> {
    let r = zph_service::list_companies(&state, id, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;

    // Fetch all related phpyun_company rows in one query, keyed by uid
    let uids: Vec<u64> = r.list.iter().map(|c| c.uid).collect();
    let mut com_map: std::collections::HashMap<
        u64,
        (
            Option<String>,
            Option<String>,
            i32,
            i32,
            i32,
            i32,
            i32,
        ),
    > = std::collections::HashMap::new();
    if !uids.is_empty() {
        let placeholders = std::iter::repeat("?")
            .take(uids.len())
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "SELECT CAST(uid AS UNSIGNED), name, logo, \
                CAST(COALESCE(hy,0) AS SIGNED), \
                CAST(COALESCE(pr,0) AS SIGNED), \
                CAST(COALESCE(mun,0) AS SIGNED), \
                CAST(COALESCE(provinceid,0) AS SIGNED), \
                CAST(COALESCE(cityid,0) AS SIGNED) \
             FROM phpyun_company WHERE uid IN ({placeholders})"
        );
        let mut q = sqlx::query_as::<_, (u64, Option<String>, Option<String>, i32, i32, i32, i32, i32)>(&sql);
        for u in &uids {
            q = q.bind(*u as i64);
        }
        let rows = q.fetch_all(state.db.reader()).await.unwrap_or_default();
        for (uid, name, logo, hy, pr, mun, prov, city) in rows {
            com_map.insert(uid, (name, logo, hy, pr, mun, prov, city));
        }
    }

    let items: Vec<ZphCompanyItem> = r
        .list
        .into_iter()
        .map(|c| {
            let info = com_map.get(&c.uid).cloned();
            let (com_name, com_logo, hy, pr, mun, prov, city) = info.unwrap_or_else(|| {
                (None, None, 0, 0, 0, 0, 0)
            });
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
