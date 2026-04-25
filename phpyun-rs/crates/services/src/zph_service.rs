//! Job fair service.

use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::zph::{
    entity::{Zph, ZphCompany, ZphReservation},
    repo as zph_repo,
};

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<Zph>> {
    let db = state.db.reader();
    let list = zph_repo::list(db, page.offset, page.limit).await?;
    let total = zph_repo::count(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_detail(state: &AppState, id: u64) -> AppResult<Zph> {
    zph_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("zph_not_found".into())))
}

pub async fn list_companies(
    state: &AppState,
    zid: u64,
    page: Pagination,
) -> AppResult<Paged<ZphCompany>> {
    let db = state.db.reader();
    let list = zph_repo::list_companies(db, zid, page.offset, page.limit).await?;
    let total = zph_repo::count_companies(db, zid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct ReserveInput<'a> {
    pub job_ids: &'a str,
    pub name: &'a str,
    pub mobile: &'a str,
}

pub async fn reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<u64> {
    // Must confirm the zph exists and is published
    let zph = zph_repo::find_by_id(state.db.reader(), zid)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("zph_not_found".into())))?;
    if zph.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("zph_unavailable".into())));
    }

    let id = zph_repo::upsert_reservation(
        state.db.pool(),
        zph_repo::ReservationCreate {
            zid,
            uid: user.uid,
            job_ids: input.job_ids,
            name: input.name,
            mobile: input.mobile,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn my_reservation(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
) -> AppResult<Option<ZphReservation>> {
    Ok(zph_repo::find_my_reservation(state.db.reader(), zid, user.uid).await?)
}
