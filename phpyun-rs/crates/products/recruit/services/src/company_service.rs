//! Company service (usertype=2). PHP `info.class` + `company.model::setCompany`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::cache::TieredCache;
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo::CompanyFilter;
use phpyun_models::company::{entity::Company, repo as company_repo};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::user::repo as user_repo;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyPage {
    pub list: Vec<Company>,
    pub total: u64,
}

const LIST_TTL: Duration = Duration::from_secs(60);

static LIST_CACHE: OnceLock<TieredCache<CompanyPage>> = OnceLock::new();
static DETAIL_CACHE: OnceLock<TieredCache<Company>> = OnceLock::new();

fn list_cache() -> &'static TieredCache<CompanyPage> {
    LIST_CACHE.get_or_init(|| TieredCache::new(256, LIST_TTL))
}

fn detail_cache() -> &'static TieredCache<Company> {
    DETAIL_CACHE.get_or_init(|| TieredCache::new(512, LIST_TTL))
}

fn i32z(v: Option<i32>) -> i32 {
    v.unwrap_or(0)
}

fn is_default_list(filter: &CompanyFilter<'_>, page: &Pagination) -> bool {
    if page.page != 1 {
        return false;
    }
    filter
        .keyword
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
}

fn list_cache_key(filter: &CompanyFilter<'_>, page: &Pagination) -> String {
    let city_ids = filter
        .city_ids
        .unwrap_or(&[])
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "companies:list:p{}:n{}:d{}:pv{}:ct{}:th{}:ids{}:hy{}:pr{}:mu{}:wf{}:wn{}:cf{}:rc{}:up{}:od{}",
        page.page,
        page.page_size,
        filter.did,
        i32z(filter.province_id),
        i32z(filter.city_id),
        i32z(filter.three_city_id),
        city_ids,
        i32z(filter.hy),
        i32z(filter.pr),
        i32z(filter.mun),
        i32z(filter.welfare),
        filter.welfare_name.unwrap_or(""),
        filter.cert as u8,
        filter.rec as u8,
        i32z(filter.uptime),
        filter.order.unwrap_or(""),
    )
}

pub async fn invalidate_sidebar(state: &AppState) {
    list_cache().invalidate_prefix_local();
    detail_cache().invalidate_prefix_local();
    let _ = state;
}

/// Public company list search (keyword / province / city / industry).
pub async fn list_public(
    state: &AppState,
    filter: &CompanyFilter<'_>,
    page: Pagination,
) -> AppResult<Arc<CompanyPage>> {
    if is_default_list(filter, &page) {
        let key = list_cache_key(filter, &page);
        let now = clock::now_ts();
        let did = filter.did;
        let rec = filter.rec;
        let cert = filter.cert;
        let hy = filter.hy;
        let pr = filter.pr;
        let mun = filter.mun;
        let welfare = filter.welfare;
        let welfare_name = filter.welfare_name.map(|s| s.to_string());
        let province_id = filter.province_id;
        let city_id = filter.city_id;
        let three_city_id = filter.three_city_id;
        let city_ids = filter.city_ids.map(|s| s.to_vec());
        let uptime = filter.uptime;
        let order = filter.order.map(|s| s.to_string());
        let st = state.clone();
        return list_cache()
            .get_or_load(
                &state.redis,
                key,
                LIST_TTL,
                "companies.list",
                move || async move {
                    let city_owned = city_ids;
                    let wn = welfare_name;
                    let f = CompanyFilter {
                        keyword: None,
                        province_id,
                        city_id,
                        three_city_id,
                        city_ids: city_owned.as_deref(),
                        hy,
                        pr,
                        mun,
                        welfare,
                        welfare_name: wn.as_deref(),
                        cert,
                        rec,
                        did,
                        uptime,
                        order: order.as_deref(),
                    };
                    let (total, list) = tokio::join!(
                        company_repo::count_public(st.db.reader(), &f, now),
                        company_repo::list_public(
                            st.db.reader(),
                            &f,
                            page.offset,
                            page.limit,
                            now
                        ),
                    );
                    Ok(CompanyPage {
                        total: total?,
                        list: list?,
                    })
                },
            )
            .await;
    }
    let now = clock::now_ts();
    let (total, list) = tokio::join!(
        company_repo::count_public(state.db.reader(), filter, now),
        company_repo::list_public(state.db.reader(), filter, page.offset, page.limit, now),
    );
    Ok(Arc::new(CompanyPage {
        total: total?,
        list: list?,
    }))
}

pub struct CompanyUpdateInput<'a> {
    pub name: Option<&'a str>,
    pub shortname: Option<&'a str>,
    pub hy: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    pub logo: Option<&'a str>,
    pub content: Option<&'a str>,
    pub linkman: Option<&'a str>,
    pub linkjob: Option<&'a str>,
    pub linkphone: Option<&'a str>,
    pub linkmail: Option<&'a str>,
    pub x: Option<&'a str>,
    pub y: Option<&'a str>,
    pub pr: Option<i32>,
    pub mun: Option<i32>,
}

/// Company reads its own profile; if no row exists, calls ensure_row first.
pub async fn get_mine(state: &AppState, user: &AuthenticatedUser) -> AppResult<Company> {
    user.require_employer()?;
    if let Some(c) = company_repo::find_by_uid(state.db.reader(), user.uid).await? {
        return Ok(c);
    }
    company_repo::ensure_row(state.db.pool(), user.uid, user.did).await?;
    company_repo::find_by_uid(state.db.pool(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("company_not_found"))
}

/// Public company view (WAP detail page) — accessible to anyone, but the company must have `r_status=1` (passed review), except the owner.
pub async fn get_public(
    state: &AppState,
    uid: u64,
    viewer: Option<&AuthenticatedUser>,
) -> AppResult<Company> {
    if viewer.is_some_and(|u| u.uid == uid) {
        return load_company_for_viewer(state, uid, viewer).await;
    }
    let st = state.clone();
    let loaded = detail_cache()
        .get_or_load(
            &state.redis,
            format!("companies:detail:{uid}"),
            LIST_TTL,
            "companies.detail",
            move || async move { load_public_company(&st, uid).await },
        )
        .await;
    match loaded {
        Ok(arc) => {
            bump_company_hits(state, uid);
            Ok(Arc::unwrap_or_clone(arc))
        }
        Err(e) => Err(e),
    }
}

fn bump_company_hits(state: &AppState, uid: u64) {
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("company.hits", async move {
        let _ = company_repo::incr_hits(&pool, uid).await;
    });
}

async fn load_public_company(state: &AppState, uid: u64) -> AppResult<Company> {
    let c = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or(ApiError::business("company_not_found"))?;
    match c.r_status {
        1 => Ok(c),
        2 => Err(ApiError::business("company_locked")),
        _ => Err(ApiError::business("company_not_verified")),
    }
}

async fn load_company_for_viewer(
    state: &AppState,
    uid: u64,
    viewer: Option<&AuthenticatedUser>,
) -> AppResult<Company> {
    let c = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or(ApiError::business("company_not_found"))?;
    if viewer.is_some_and(|u| u.uid == c.uid) {
        if c.r_status == 1 {
            bump_company_hits(state, c.uid);
        }
        return Ok(c);
    }
    match c.r_status {
        1 => {
            bump_company_hits(state, c.uid);
            Ok(c)
        }
        2 => Err(ApiError::business("company_locked")),
        _ => Err(ApiError::business("company_not_verified")),
    }
}

pub async fn update_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CompanyUpdateInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    company_repo::ensure_row(state.db.pool(), user.uid, user.did).await?;
    company_repo::update(
        state.db.pool(),
        user.uid,
        company_repo::CompanyUpdate {
            name: input.name,
            shortname: input.shortname,
            hy: input.hy,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            logo: input.logo,
            content: input.content,
            linkman: input.linkman,
            linkjob: input.linkjob,
            linkphone: input.linkphone,
            linkmail: input.linkmail,
            x: input.x,
            y: input.y,
            pr: input.pr,
            mun: input.mun,
        },
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("company.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(())
}
