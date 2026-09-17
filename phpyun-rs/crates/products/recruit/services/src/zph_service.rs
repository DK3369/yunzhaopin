//! Job fair service.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::cache::TieredCache;
use phpyun_core::json;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::{
    job::repo as job_repo,
    vip::repo as vip_repo,
    zph::{
        entity::{Zph, ZphCompany, ZphPic, ZphReservation, ZphSpace},
        repo as zph_repo,
    },
};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use std::time::Duration;

pub use phpyun_models::job::repo::OwnJobBrief;

const LIST_TTL: Duration = Duration::from_secs(60);

static CACHE: OnceLock<TieredCache<Paged<Zph>>> = OnceLock::new();

fn cache() -> &'static TieredCache<Paged<Zph>> {
    CACHE.get_or_init(|| TieredCache::new(64, LIST_TTL))
}

fn cache_key(did: u32, page: &Pagination) -> String {
    format!("zph:{did}:{}:{}", page.page, page.page_size)
}

pub async fn invalidate_all(state: &AppState) {
    cache().invalidate_prefix_local();
    let _ = state;
}

pub async fn list(
    state: &AppState,
    page: Pagination,
    keyword: Option<&str>,
) -> AppResult<Paged<Zph>> {
    let kw = keyword.map(str::trim).filter(|s| !s.is_empty());
    if kw.is_some() {
        let db = state.db.reader();
        let list = zph_repo::list(db, page.offset, page.limit, keyword).await?;
        let total = zph_repo::count(db, keyword).await?;
        return Ok(Paged::new(list, total, page.page, page.page_size));
    }
    let did = 0u32;
    let key = cache_key(did, &page);
    let st = state.clone();
    let arc = cache()
        .get_or_load(
            &state.redis,
            key,
            LIST_TTL,
            "zph",
            move || async move {
                let db = st.db.reader();
                let list = zph_repo::list(db, page.offset, page.limit, None).await?;
                let total = zph_repo::count(db, None).await?;
                Ok(Paged::new(list, total, page.page, page.page_size))
            },
        )
        .await?;
    Ok((*arc).clone())
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

/// Past-event gallery (`phpyun_zhaopinhui_pic`). Additive on fair detail.
pub async fn list_pics(state: &AppState, zid: u64) -> AppResult<Vec<ZphPic>> {
    Ok(zph_repo::list_pics(state.db.reader(), zid).await?)
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

struct ZphBoothReady {
    job_ids: String,
    name: String,
    sid: i32,
    cid: i32,
    bid: i32,
    price: i32,
    rating_type: i32,
    zph_num: i32,
}

#[derive(Debug, Deserialize, Default)]
struct ZphOrderInfo {
    #[serde(default)]
    zid: u64,
    #[serde(default)]
    bid: i32,
    #[serde(default)]
    sid: i32,
    #[serde(default)]
    cid: i32,
    #[serde(default)]
    jobid: String,
    #[serde(default)]
    com_name: String,
}

async fn resolve_booth(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<ZphBoothReady> {
    user.require_employer()?;
    let now = clock::now_ts();
    let reader = state.db.reader();

    let zph = zph_repo::find_by_id(reader, zid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("zph_not_found"))?;
    if zph.status != 1 || zph.is_open != 1 {
        return Err(ApiError::business("zph_closed"));
    }
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

    let com_name = input.name.trim();
    let name = if com_name.is_empty() {
        com.name.clone().unwrap_or_default()
    } else {
        com_name.to_string()
    };
    let _ = input.mobile;

    Ok(ZphBoothReady {
        job_ids: input.job_ids.to_string(),
        name,
        sid,
        cid,
        bid: input.bid,
        price: space.price,
        rating_type: statis.rating_type,
        zph_num: statis.zph_num,
    })
}

pub async fn reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<u64> {
    let now = clock::now_ts();
    let pool = state.db.pool();
    let ready = resolve_booth(state, user, zid, input).await?;
    if ready.rating_type == 1 {
        if ready.zph_num <= 0 {
            if ready.price > 0 {
                return Err(ApiError::business("zph_need_pay"));
            }
        } else {
            let n = phpyun_models::company_statis::repo::dec_zph_num(pool, user.uid).await?;
            if n == 0 {
                return Err(ApiError::business("zph_need_pay"));
            }
        }
    }

    let id = zph_repo::upsert_reservation(
        pool,
        zph_repo::ReservationCreate {
            zid,
            uid: user.uid,
            job_ids: &ready.job_ids,
            name: &ready.name,
            sid: ready.sid,
            cid: ready.cid,
            bid: ready.bid,
        },
        now,
    )
    .await?;
    Ok(id)
}

pub struct CreatedZphOrder {
    pub order_no: String,
    pub price: f64,
}

pub async fn create_zph_order(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<CreatedZphOrder> {
    let ready = resolve_booth(state, user, zid, input).await?;
    if ready.rating_type != 1 || ready.zph_num > 0 || ready.price <= 0 {
        return Err(ApiError::param_invalid("price"));
    }
    for row in vip_repo::list_pending_zph_orders(state.db.reader(), user.uid).await? {
        let info: ZphOrderInfo = json::from_str(&row.order_info).unwrap_or_default();
        if info.zid == zid {
            return Err(ApiError::business("zph_order_exists"));
        }
    }
    let info = json::json!({
        "zid": zid,
        "bid": ready.bid,
        "sid": ready.sid,
        "cid": ready.cid,
        "jobid": ready.job_ids,
        "com_name": ready.name,
    });
    let raw = json::to_string(&info)?;
    let now = clock::now_ts();
    let price = f64::from(ready.price.max(0));
    let order_no = vip_repo::create_zph_order(state.db.pool(), user.uid, user.did, price, &raw, now)
        .await?;
    Ok(CreatedZphOrder { order_no, price })
}

pub async fn settle_zph_order(state: &AppState, order_no: &str, pay_tx_id: &str) -> AppResult<()> {
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.order_kind != 28 {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    if o.status == 1 {
        return Ok(());
    }
    if o.status != 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    let info: ZphOrderInfo = json::from_str(&o.order_info).unwrap_or_default();
    if info.zid == 0 || info.bid <= 0 {
        return Err(ApiError::param_invalid("order_info"));
    }
    let now = clock::now_ts();
    let price = (o.amount_cents.max(0) / 100) as i32;
    zph_repo::upsert_reservation_paid(
        state.db.pool(),
        info.zid,
        o.uid,
        &info.jobid,
        &info.com_name,
        info.sid,
        info.cid,
        info.bid,
        price,
        now,
    )
    .await?;
    let n = vip_repo::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    Ok(())
}

pub async fn my_reservation(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
) -> AppResult<Option<ZphReservation>> {
    Ok(zph_repo::find_my_reservation(state.db.reader(), zid, user.uid).await?)
}

pub struct MyReservationPage {
    pub list: Vec<phpyun_models::zph::repo::ZphReservationListRow>,
    pub total: u64,
}

/// Company job-fair sign-ups. `zid = 0` lists every fair this uid joined.
pub async fn list_my_reservations(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    page: Pagination,
) -> AppResult<MyReservationPage> {
    user.require_employer()?;
    let filter = (zid > 0).then_some(zid);
    let (list, total) = tokio::join!(
        zph_repo::list_my_reservations(state.db.reader(), user.uid, filter, page.offset, page.limit),
        zph_repo::count_my_reservations(state.db.reader(), user.uid, filter),
    );
    let mut list = list?;
    enrich_reservations(state, &mut list).await?;
    Ok(MyReservationPage {
        list,
        total: total?,
    })
}

fn parse_csv_ids(raw: &str) -> Vec<u64> {
    raw.split(|c: char| c == ',' || c == '|' || c.is_whitespace())
        .filter_map(|p| p.trim().parse::<u64>().ok())
        .filter(|n| *n > 0)
        .collect()
}

fn join_space_names(map: &HashMap<i32, String>, ids: [i32; 3]) -> String {
    ids.into_iter()
        .filter(|id| *id > 0)
        .filter_map(|id| map.get(&id).cloned().filter(|s| !s.is_empty()))
        .collect::<Vec<_>>()
        .join("-")
}

async fn enrich_reservations(
    state: &AppState,
    list: &mut [zph_repo::ZphReservationListRow],
) -> AppResult<()> {
    if list.is_empty() {
        return Ok(());
    }
    let now = clock::now_ts();
    let mut space_ids: Vec<i32> = Vec::new();
    let mut job_ids: Vec<u64> = Vec::new();
    for row in list.iter() {
        for id in [row.sid, row.cid, row.bid] {
            if id > 0 {
                space_ids.push(id);
            }
        }
        job_ids.extend(parse_csv_ids(&row.job_ids));
    }
    space_ids.sort_unstable();
    space_ids.dedup();
    job_ids.sort_unstable();
    job_ids.dedup();
    let reader = state.db.reader();
    let spaces = zph_repo::list_spaces_by_ids(reader, &space_ids).await?;
    let space_map: HashMap<i32, String> = spaces
        .into_iter()
        .map(|s| (i32::try_from(s.id).unwrap_or(0), s.name))
        .collect();
    let jobs = if job_ids.is_empty() {
        Vec::new()
    } else {
        job_repo::list_by_ids(reader, &job_ids).await?
    };
    let job_map: HashMap<u64, String> = jobs.into_iter().map(|j| (j.id, j.name)).collect();
    for row in list.iter_mut() {
        row.booth_name = join_space_names(&space_map, [row.sid, row.cid, row.bid]);
        row.job_names = parse_csv_ids(&row.job_ids)
            .into_iter()
            .filter_map(|id| job_map.get(&id).cloned())
            .collect::<Vec<_>>()
            .join(",");
        row.notstart = if row.start_at > now || row.status != 1 {
            1
        } else {
            0
        };
    }
    Ok(())
}

/// PHP `member/com/zhaopinhui::del_action`: delete own row; refund points when
/// pending (`status==0`) and `price>0`. Never refund `zph_num`.
pub async fn cancel_reservation(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_employer()?;
    let row = zph_repo::find_owned_com(state.db.reader(), id, user.uid)
        .await?
        .ok_or_else(|| ApiError::business("zph_reservation_not_found"))?;
    let n = zph_repo::delete_owned_com(state.db.pool(), id, user.uid).await?;
    if n == 0 {
        return Err(ApiError::business("zph_reservation_not_found"));
    }
    if row.status == 0 && row.price > 0 {
        let pts = i64::from(row.price);
        let now = clock::now_ts();
        let _ = phpyun_models::company_statis::repo::add_integral(state.db.pool(), user.uid, pts)
            .await;
        let order_id = format!("{now}{id}");
        let _ = phpyun_models::integral_transfer::repo::php_insert_pay_typed(
            state.db.pool(),
            &order_id,
            &pts.to_string(),
            now,
            user.uid,
            "member_com_00711",
            phpyun_models::integral_transfer::repo::LEDGER_KIND_INTEGRAL,
            2,
            2,
        )
        .await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("zph.cancel", Actor::uid(user.uid)).target(format!("zph_com:{id}")),
    )
    .await;
    Ok(())
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
