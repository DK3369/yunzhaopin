//! Company service (usertype=2).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::cache::TieredCache;
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo::CompanyFilter;
use phpyun_models::company::{entity::Company, repo as company_repo};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::sync::OnceLock;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyPage {
    pub list: Vec<Company>,
    pub total: u64,
}

const SIDEBAR_TTL: Duration = Duration::from_secs(60);

static SIDEBAR_CACHE: OnceLock<TieredCache<CompanyPage>> = OnceLock::new();

fn sidebar_cache() -> &'static TieredCache<CompanyPage> {
    SIDEBAR_CACHE.get_or_init(|| TieredCache::new(32, SIDEBAR_TTL))
}

fn filter_hash(filter: &CompanyFilter<'_>) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    format!("{filter:?}").hash(&mut h);
    h.finish()
}

fn is_sidebar_query(filter: &CompanyFilter<'_>, page: &Pagination) -> bool {
    if page.page != 1 || !filter.rec {
        return false;
    }
    filter
        .keyword
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
        && filter.province_id.is_none()
        && filter.city_id.is_none()
        && filter.three_city_id.is_none()
        && filter.city_ids.map(|ids| ids.is_empty()).unwrap_or(true)
        && filter.hy.is_none()
        && filter.pr.is_none()
        && filter.mun.is_none()
        && filter.welfare.is_none()
        && filter
            .welfare_name
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .is_none()
}

pub async fn invalidate_sidebar(state: &AppState) {
    sidebar_cache().invalidate_prefix_local();
    let _ = state;
}

/// Public company list search (keyword / province / city / industry).
pub async fn list_public(
    state: &AppState,
    filter: &CompanyFilter<'_>,
    page: Pagination,
) -> AppResult<CompanyPage> {
    if is_sidebar_query(filter, &page) {
        let key = format!(
            "companies:sidebar:{:x}:{}",
            filter_hash(filter),
            page.page
        );
        let now = clock::now_ts();
        let did = filter.did;
        let rec = filter.rec;
        let cert = filter.cert;
        let hy = filter.hy;
        let pr = filter.pr;
        let mun = filter.mun;
        let welfare = filter.welfare;
        let province_id = filter.province_id;
        let city_id = filter.city_id;
        let three_city_id = filter.three_city_id;
        let uptime = filter.uptime;
        let order = filter.order.map(|s| s.to_string());
        let st = state.clone();
        let arc = sidebar_cache()
            .get_or_load(
                &state.redis,
                key,
                SIDEBAR_TTL,
                "companies.sidebar",
                move || async move {
                    let f = CompanyFilter {
                        keyword: None,
                        province_id,
                        city_id,
                        three_city_id,
                        city_ids: None,
                        hy,
                        pr,
                        mun,
                        welfare,
                        welfare_name: None,
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
            .await?;
        return Ok((*arc).clone());
    }
    let now = clock::now_ts();
    let (total, list) = tokio::join!(
        company_repo::count_public(state.db.reader(), filter, now),
        company_repo::list_public(state.db.reader(), filter, page.offset, page.limit, now),
    );
    Ok(CompanyPage {
        total: total?,
        list: list?,
    })
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
    let c = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or(ApiError::business("company_not_found"))?;
    if viewer.is_some_and(|u| u.uid == c.uid) {
        if c.r_status == 1 {
            let pool = state.db.pool().clone();
            let uid = c.uid;
            phpyun_core::background::spawn_best_effort("company.hits", async move {
                let _ = company_repo::incr_hits(&pool, uid).await;
            });
        }
        return Ok(c);
    }
    match c.r_status {
        1 => {
            let pool = state.db.pool().clone();
            let uid = c.uid;
            phpyun_core::background::spawn_best_effort("company.hits", async move {
                let _ = company_repo::incr_hits(&pool, uid).await;
            });
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
