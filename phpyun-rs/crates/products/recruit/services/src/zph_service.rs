//! Job fair service.

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::{
    job::repo as job_repo,
    zph::{
        entity::{Zph, ZphCompany, ZphReservation, ZphSpace},
        repo as zph_repo,
    },
};
use std::collections::HashSet;

pub use phpyun_models::job::repo::OwnJobBrief;

pub async fn list(
    state: &AppState,
    page: Pagination,
    keyword: Option<&str>,
) -> AppResult<Paged<Zph>> {
    let db = state.db.reader();
    let list = zph_repo::list(db, page.offset, page.limit, keyword).await?;
    let total = zph_repo::count(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_detail(state: &AppState, id: u64) -> AppResult<Zph> {
    let z = zph_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("zph_not_found"))?;
    if z.is_open != 1 {
        return Err(ApiError::business("zph_closed"));
    }
    Ok(z)
}

pub struct PublicSpace {
    pub space: ZphSpace,
    pub taken: bool,
}

/// Booth picker for the public fair page: leaf spaces (or parents if no children)
/// plus which bids are already reserved for this fair.
pub async fn list_public_spaces(state: &AppState, zid: u64) -> AppResult<Vec<PublicSpace>> {
    let _ = get_detail(state, zid).await?;
    let db = state.db.reader();
    let parents = zph_repo::list_spaces(db, None, None).await?;
    let mut booths: Vec<ZphSpace> = Vec::new();
    for p in &parents {
        let kids = zph_repo::space_children(db, p.id as i64).await?;
        if kids.is_empty() {
            booths.push(p.clone());
        } else {
            booths.extend(kids);
        }
    }
    let taken: HashSet<i32> = zph_repo::taken_bids(db, zid).await?.into_iter().collect();
    Ok(booths
        .into_iter()
        .map(|space| {
            let id_i = i32::try_from(space.id).unwrap_or(0);
            PublicSpace {
                taken: taken.contains(&id_i),
                space,
            }
        })
        .collect())
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
    pub bid: i32,
}

pub async fn reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<u64> {
    user.require_employer()?;
    let now = clock::now_ts();
    let reader = state.db.reader();
    let pool = state.db.pool();

    let zph = zph_repo::find_by_id(reader, zid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("zph_not_found"))?;
    if zph.status != 1 || zph.is_open != 1 {
        return Err(ApiError::business("zph_closed"));
    }
    // PHP: starttime already passed → too late; endtime passed → ended.
    if zph.start_at > 0 && zph.start_at < now {
        return Err(ApiError::business("zph_already_started"));
    }
    if zph.end_at > 0 && zph.end_at < now {
        return Err(ApiError::business("zph_ended"));
    }

    let com = phpyun_models::company::repo::find_by_uid(reader, user.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("company_not_found"))?;
    if com.name.as_deref().unwrap_or("").trim().is_empty() {
        return Err(ApiError::param_invalid("company_name_required"));
    }
    if com.r_status == 4 {
        return Err(ApiError::business("company_locked"));
    }
    if com.r_status != 1 {
        return Err(ApiError::business("company_not_verified"));
    }

    if input.bid <= 0 {
        return Err(ApiError::param_invalid("bid"));
    }
    if zph_repo::find_com_by_bid(reader, zid, input.bid)
        .await?
        .is_some()
    {
        return Err(ApiError::business("zph_booth_taken"));
    }
    if zph_repo::find_my_reservation(reader, zid, user.uid)
        .await?
        .is_some()
    {
        return Err(ApiError::business("zph_already_reserved"));
    }

    let space = zph_repo::find_space_by_id(reader, input.bid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("booth_not_found"))?;
    let parent = if space.keyid > 0 {
        zph_repo::find_space_by_id(reader, space.keyid as i32).await?
    } else {
        None
    };
    let cid = space.keyid as i32;
    let sid = parent.map(|p| p.keyid as i32).unwrap_or(0);

    let statis = phpyun_models::company_statis::repo::find_admin(reader, user.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("statis_not_found"))?;
    let vip_ok = statis.vip_etime == 0 || statis.vip_etime >= now;
    if !vip_ok {
        return Err(ApiError::business("zph_need_vip"));
    }
    if statis.rating_type == 1 {
        if statis.zph_num <= 0 {
            if space.price > 0 {
                return Err(ApiError::business("zph_need_pay"));
            }
        } else {
            let n = phpyun_models::company_statis::repo::dec_zph_num(pool, user.uid).await?;
            if n == 0 {
                return Err(ApiError::business("zph_need_pay"));
            }
        }
    }

    let com_name = input.name.trim();
    let name = if com_name.is_empty() {
        com.name.clone().unwrap_or_default()
    } else {
        com_name.to_string()
    };
    let _ = input.mobile;

    let id = zph_repo::upsert_reservation(
        pool,
        zph_repo::ReservationCreate {
            zid,
            uid: user.uid,
            job_ids: input.job_ids,
            name: &name,
            sid,
            cid,
            bid: input.bid,
        },
        now,
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

// ==================== Pre-apply status check (PHP `wap/ajax::ajaxComjob`) ====================

pub enum ComStatusOutcome {
    /// Already applied — `status` echoes `phpyun_zhaopinhui_com.status`
    /// (0 pending review, 1 approved, 2 rejected).
    Applied { status: i32 },
    /// Hasn't applied yet — present a list of own published jobs to attach.
    NotApplied { jobs: Vec<OwnJobBrief> },
    /// Hasn't applied AND has no published jobs — caller must publish first.
    NoJobs,
}

/// Counterpart of PHP `wap/ajax::ajaxComjob_action`. Returns either the
/// employer's existing application status for a fair, or — when not yet
/// applied — the list of their own active jobs (so the form can pre-fill
/// the "which jobs to bring" field). PHP also short-circuits with a clear
/// "no jobs" path when the company has nothing to offer.
pub async fn com_status_for_fair(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
) -> AppResult<ComStatusOutcome> {
    user.require_employer()?;
    let reader = state.db.reader();

    if let Some(existing) = zph_repo::find_my_reservation(reader, zid, user.uid).await? {
        return Ok(ComStatusOutcome::Applied {
            status: existing.status,
        });
    }

    // Mirror PHP filter: state=1 (active), status=0 (open), r_status<>2 (not rejected company-wide)
    let now = clock::now_ts();
    let rows = job_repo::list_active_for_job_fair(reader, user.uid, now, 50).await?;

    if rows.is_empty() {
        Ok(ComStatusOutcome::NoJobs)
    } else {
        Ok(ComStatusOutcome::NotApplied { jobs: rows })
    }
}
