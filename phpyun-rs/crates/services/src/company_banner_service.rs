//! Company home-page banner service.
//!
//! Aligned with PHPYun `member/com/banner::*` + `company.model::addBanner/delBanner`.
//!
//! Business rules:
//! - Only companies (usertype=2) can manage their own banners.
//! - Per-company cap: passed in by the caller (handler) from admin config (aligned with PHP `com_banner_num`).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_models::company_banner::entity::CompanyBanner;
use phpyun_models::company_banner::repo as banner_repo;

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<CompanyBanner>> {
    user.require_employer()?;
    let list = banner_repo::list_by_uid(state.db.reader(), user.uid).await?;
    Ok(list)
}

pub struct AddInput<'a> {
    pub pic: &'a str,
    pub link: Option<&'a str>,
    pub sort: i32,
    /// Maximum banners per company; 0 = unlimited.
    pub max_per_company: u64,
}

pub async fn add(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &AddInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    if input.pic.is_empty() {
        return Err(InfraError::InvalidParam("pic".into()).into());
    }
    if input.max_per_company > 0 {
        let used = banner_repo::count_by_uid(state.db.reader(), user.uid).await?;
        if used >= input.max_per_company {
            return Err(InfraError::RateLimited.into());
        }
    }
    let id = banner_repo::create(
        state.db.pool(),
        user.uid,
        input.pic,
        input.link,
        input.sort,
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "company.banner_add",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("banner:{id}")),
    )
    .await;
    Ok(id)
}

pub struct UpdateInput<'a> {
    pub pic: Option<&'a str>,
    pub link: Option<&'a str>,
    pub sort: Option<i32>,
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: &UpdateInput<'_>,
) -> AppResult<u64> {
    user.require_employer()?;
    banner_repo::update(state.db.pool(), id, user.uid, input.pic, input.link, input.sort)
        .await
        .map_err(Into::into)
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    banner_repo::delete_by_ids(state.db.pool(), ids, user.uid)
        .await
        .map_err(Into::into)
}
