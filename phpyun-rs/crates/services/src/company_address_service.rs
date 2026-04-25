//! Company work-location service.
//!
//! Aligned with PHPYun `member/com/address` + `address.model`.
//!
//! Note: the PHP side uses a special branch where `$_POST['id'] == -1` means "write back to the
//! `phpyun_company` main table". The Rust version keeps these concerns separated — the address
//! book always writes to `phpyun_company_job_link`, while main-table fields are maintained by the
//! company profile flow (existing `company_service`). The result is cleaner.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
use phpyun_models::company_address::entity::CompanyAddress;
use phpyun_models::company_address::repo as addr_repo;

pub struct AddressPage {
    pub list: Vec<CompanyAddress>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<AddressPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        addr_repo::count_by_uid(state.db.reader(), user.uid),
        addr_repo::list_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(AddressPage {
        total: total?,
        list: list?,
    })
}

pub struct AddressInput<'a> {
    pub link_man: &'a str,
    pub link_moblie: &'a str,
    pub link_phone: &'a str,
    pub email: &'a str,
    pub link_address: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub x: &'a str,
    pub y: &'a str,
}

fn validate(input: &AddressInput<'_>) -> AppResult<()> {
    if input.link_man.trim().is_empty() {
        return Err(InfraError::InvalidParam("link_man".into()).into());
    }
    if input.link_moblie.trim().is_empty() {
        return Err(InfraError::InvalidParam("link_moblie".into()).into());
    }
    if input.provinceid == 0 && input.cityid == 0 {
        return Err(InfraError::InvalidParam("city".into()).into());
    }
    Ok(())
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &AddressInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    validate(input)?;
    let f = addr_repo::AddressFields {
        link_man: input.link_man,
        link_moblie: input.link_moblie,
        link_phone: input.link_phone,
        email: input.email,
        link_address: input.link_address,
        provinceid: input.provinceid,
        cityid: input.cityid,
        three_cityid: input.three_cityid,
        x: input.x,
        y: input.y,
    };
    let id = addr_repo::create(state.db.pool(), user.uid, &f).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("company.address_add", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("addr:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: &AddressInput<'_>,
) -> AppResult<u64> {
    user.require_employer()?;
    validate(input)?;
    let f = addr_repo::AddressFields {
        link_man: input.link_man,
        link_moblie: input.link_moblie,
        link_phone: input.link_phone,
        email: input.email,
        link_address: input.link_address,
        provinceid: input.provinceid,
        cityid: input.cityid,
        three_cityid: input.three_cityid,
        x: input.x,
        y: input.y,
    };
    Ok(addr_repo::update(state.db.pool(), id, user.uid, &f).await?)
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    Ok(addr_repo::delete_by_ids(state.db.pool(), ids, user.uid).await?)
}
