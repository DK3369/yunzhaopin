//! Company service (usertype=2).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::{entity::Company, repo as company_repo};
use phpyun_models::company::repo::CompanyFilter;

use crate::domain_errors::CompanyError;

pub struct CompanyPage {
    pub list: Vec<Company>,
    pub total: u64,
}

/// Public company list search (keyword / province / city / industry).
pub async fn list_public(
    state: &AppState,
    filter: &CompanyFilter<'_>,
    page: Pagination,
) -> AppResult<CompanyPage> {
    let (total, list) = tokio::join!(
        company_repo::count_public(state.db.reader(), filter),
        company_repo::list_public(state.db.reader(), filter, page.offset, page.limit),
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
        .ok_or_else(|| CompanyError::NotFound.into())
}

/// Public company view (WAP detail page) — accessible to anyone, but the company must have `r_status=1` (passed review).
pub async fn get_public(state: &AppState, uid: u64) -> AppResult<Company> {
    let c = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or(CompanyError::NotFound)?;
    match c.r_status {
        1 => {
            // View counter +1 (best-effort, written in the background)
            let pool = state.db.pool().clone();
            let uid = c.uid;
            phpyun_core::background::spawn_best_effort("company.hits", async move {
                let _ = company_repo::incr_hits(&pool, uid).await;
            });
            Ok(c)
        }
        2 => Err(CompanyError::Locked.into()),
        _ => Err(CompanyError::NotVerified.into()),
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
