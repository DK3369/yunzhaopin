//! Company service (usertype=2). PHP `info.class` + `company.model::setCompany`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::cache::TieredCache;
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo::CompanyFilter;
use phpyun_models::company::{entity::Company, repo as company_repo};
use phpyun_models::job::repo as job_repo;
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

/// Drop public list L1 and DEL this company's detail Redis key.
pub async fn invalidate_company(state: &AppState, uid: u64) {
    list_cache().invalidate_prefix_local();
    detail_cache()
        .invalidate(&state.redis, &format!("companies:detail:{uid}"))
        .await;
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
    pub comqcode: Option<&'a str>,
    pub content: Option<&'a str>,
    pub linkman: Option<&'a str>,
    pub linkjob: Option<&'a str>,
    pub linkphone: Option<&'a str>,
    pub linktel: Option<&'a str>,
    pub linkmail: Option<&'a str>,
    pub address: Option<&'a str>,
    pub website: Option<&'a str>,
    pub linkqq: Option<&'a str>,
    pub sdate: Option<&'a str>,
    pub money: Option<i32>,
    pub moneytype: Option<i32>,
    pub infostatus: Option<i32>,
    pub welfare: Option<&'a str>,
    pub busstops: Option<&'a str>,
    pub not_disturb: Option<&'a str>,
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

fn nonempty(s: Option<&str>) -> Option<&str> {
    s.map(str::trim).filter(|v| !v.is_empty())
}

pub async fn update_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CompanyUpdateInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    company_repo::ensure_row(state.db.pool(), user.uid, user.did).await?;
    let old = company_repo::find_by_uid(state.db.pool(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("company_not_found"))?;

    let mut name = nonempty(input.name);
    if old.yyzz_status == 1 {
        name = None;
    }
    if let Some(n) = name {
        if n.len() < 2 {
            return Err(ApiError::business("common_05982"));
        }
        if company_repo::exists_name_except(state.db.pool(), n, user.uid).await? {
            return Err(ApiError::business("common_01222"));
        }
    }

    let mut linktel = nonempty(input.linktel);
    if old.moblie_status == 1 {
        linktel = None;
    }
    if let Some(tel) = linktel {
        if user_repo::exists_mobile_except(state.db.pool(), tel, Some(user.uid)).await? {
            return Err(ApiError::business("wap_js_00049"));
        }
    }
    let mut linkmail = nonempty(input.linkmail);
    if old.email_status == 1 {
        linkmail = None;
    }
    if let Some(em) = linkmail {
        if user_repo::exists_email_except(state.db.pool(), em, Some(user.uid)).await? {
            return Err(ApiError::business("wap_js_00120"));
        }
    }

    if nonempty(input.content).is_none() && old.content.as_deref().unwrap_or("").trim().is_empty() {
        return Err(ApiError::business("member_com_00445"));
    }
    if nonempty(input.linkman).is_none() && old.linkman.as_deref().unwrap_or("").trim().is_empty() {
        return Err(ApiError::business("member_com_00677"));
    }
    let phone_ok = nonempty(input.linktel)
        .or(nonempty(input.linkphone))
        .or(old.linktel.as_deref().filter(|s| !s.trim().is_empty()))
        .or(old.linkphone.as_deref().filter(|s| !s.trim().is_empty()))
        .is_some();
    if old.moblie_status != 1 && !phone_ok {
        return Err(ApiError::business("common_00670"));
    }

    let first_info = old.name.as_deref().unwrap_or("").trim().is_empty() && name.is_some();
    let first_map = old.x.as_deref().unwrap_or("").trim().is_empty()
        && nonempty(input.x).is_some()
        && nonempty(input.y).is_some();
    let content_html = nonempty(input.content).map(phpyun_core::html::sanitize_html);

    company_repo::update(
        state.db.pool(),
        user.uid,
        company_repo::CompanyUpdate {
            name,
            shortname: nonempty(input.shortname),
            hy: input.hy,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            logo: nonempty(input.logo),
            comqcode: nonempty(input.comqcode),
            content: content_html.as_deref(),
            linkman: nonempty(input.linkman),
            linkjob: nonempty(input.linkjob),
            linkphone: nonempty(input.linkphone),
            linktel,
            linkmail,
            address: nonempty(input.address),
            website: nonempty(input.website),
            linkqq: nonempty(input.linkqq),
            sdate: nonempty(input.sdate),
            money: input.money,
            moneytype: input.moneytype,
            infostatus: input.infostatus,
            welfare: nonempty(input.welfare),
            busstops: nonempty(input.busstops),
            not_disturb: nonempty(input.not_disturb),
            x: nonempty(input.x),
            y: nonempty(input.y),
            pr: input.pr,
            mun: input.mun,
        },
    )
    .await?;

    let saved = company_repo::find_by_uid(state.db.pool(), user.uid)
        .await?
        .unwrap_or(old);
    let member = user_repo::find_by_uid(state.db.pool(), user.uid).await?;
    let mob = if saved.moblie_status == 1 {
        member
            .as_ref()
            .and_then(|m| m.moblie.clone())
            .unwrap_or_default()
    } else {
        saved.linktel.clone().unwrap_or_default()
    };
    let em = if saved.email_status == 1 {
        member
            .as_ref()
            .and_then(|m| m.email.clone())
            .unwrap_or_default()
    } else {
        saved.linkmail.clone().unwrap_or_default()
    };
    let _ = user_repo::update_contact(
        state.db.pool(),
        user.uid,
        em.trim(),
        mob.trim(),
        saved.address.as_deref().unwrap_or(""),
    )
    .await;

    let _ = job_repo::sync_com_snapshot(
        state.db.pool(),
        user.uid,
        saved.name.as_deref().unwrap_or(""),
        saved.pr,
        saved.mun,
        saved.provinceid,
        saved.cityid,
        saved.three_cityid,
        saved.x.as_deref().unwrap_or(""),
        saved.y.as_deref().unwrap_or(""),
    )
    .await;

    if first_info {
        award_integral(state, user.uid, "integral_userinfo").await;
    }
    if first_map {
        award_integral(state, user.uid, "integral_map").await;
    }

    let _ = audit::emit(
        state,
        AuditEvent::new("company.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    invalidate_company(state, user.uid).await;
    Ok(())
}

async fn award_integral(state: &AppState, uid: u64, key: &str) {
    let _ = crate::integral_grant_service::grant_once(state, uid, 2, key, key, 0).await;
}

/// PHP `map.class::setMap`.
pub async fn set_map(
    state: &AppState,
    user: &AuthenticatedUser,
    x: &str,
    y: &str,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    company_repo::ensure_row(state.db.pool(), user.uid, user.did).await?;
    let old = company_repo::find_by_uid(state.db.pool(), user.uid).await?;
    let first = old
        .as_ref()
        .map(|c| c.x.as_deref().unwrap_or("").trim().is_empty())
        .unwrap_or(true);
    if x.trim().is_empty() || y.trim().is_empty() {
        return Err(ApiError::business("common_06414"));
    }
    company_repo::set_map(state.db.pool(), user.uid, x.trim(), y.trim()).await?;
    if first {
        award_integral(state, user.uid, "integral_map").await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("company.set_map", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    invalidate_company(state, user.uid).await;
    Ok(())
}

/// PHP `ajaxCheck` / `getCheckUsed`: name vs other companies, mobile vs other members.
pub async fn check_used(
    state: &AppState,
    user: &AuthenticatedUser,
    type_str: &str,
    check_str: &str,
) -> AppResult<bool> {
    user.require_employer()?;
    let s = check_str.trim();
    if s.is_empty() {
        return Ok(false);
    }
    match type_str {
        "name" => Ok(company_repo::exists_name_except(state.db.reader(), s, user.uid).await?),
        "linktel" => {
            Ok(user_repo::exists_mobile_except(state.db.reader(), s, Some(user.uid)).await?)
        }
        _ => Err(ApiError::param_invalid("type")),
    }
}
