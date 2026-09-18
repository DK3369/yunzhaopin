//! Job-seeker monthly VIP: catalog, type=31 settle, privilege flags.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::seeker_vip::{entity::SeekerVipPack, repo as pack_repo};
use phpyun_models::vip::{
    entity::{UserVip, VipPackage},
    repo as vip_repo,
};
use serde::Serialize;

const SECS_PER_DAY: i64 = 86_400;

#[derive(Debug, Clone, Default, Serialize)]
pub struct SeekerCaps {
    pub chat: bool,
    pub resume_top: bool,
    pub tpl_all: bool,
    pub refresh_free: bool,
}

pub fn pack_to_vip_package(p: &SeekerVipPack) -> VipPackage {
    VipPackage {
        id: p.id,
        code: p.code.clone(),
        name: p.name.clone(),
        target_usertype: 1,
        duration_days: p.duration_days(),
        price_cents: p.price_cents,
        desc_json: Some(serde_json::json!({
            "chat": p.chat,
            "resume_top": p.resume_top,
            "tpl_all": p.tpl_all,
            "refresh_free": p.refresh_free,
            "months": p.months,
        })),
        is_active: p.display,
        sort_order: p.sort,
        created_at: p.created_at,
    }
}

pub async fn list_buyable(state: &AppState) -> AppResult<Vec<VipPackage>> {
    let rows = pack_repo::list_buyable(state.db.reader()).await?;
    Ok(rows.iter().map(pack_to_vip_package).collect())
}

pub async fn active_for(state: &AppState, uid: u64) -> AppResult<Option<(UserVip, SeekerVipPack)>> {
    let now = clock::now_ts();
    let Some(v) = vip_repo::find_user_vip(state.db.reader(), uid).await? else {
        return Ok(None);
    };
    if v.expires_at <= now {
        return Ok(None);
    }
    let pack = pack_repo::find_by_code(state.db.reader(), &v.package_code)
        .await?
        .unwrap_or_else(|| SeekerVipPack {
            id: 0,
            code: v.package_code.clone(),
            name: String::new(),
            months: 1,
            price_cents: 0,
            chat: 1,
            resume_top: 1,
            tpl_all: 1,
            refresh_free: 1,
            sort: 0,
            display: 1,
            deleted: 0,
            created_at: v.started_at,
            updated_at: v.updated_at,
        });
    Ok(Some((v, pack)))
}

pub fn caps_from_pack(p: &SeekerVipPack) -> SeekerCaps {
    SeekerCaps {
        chat: SeekerVipPack::flag(p.chat),
        resume_top: SeekerVipPack::flag(p.resume_top),
        tpl_all: SeekerVipPack::flag(p.tpl_all),
        refresh_free: SeekerVipPack::flag(p.refresh_free),
    }
}

pub async fn seeker_caps(state: &AppState, uid: u64) -> AppResult<SeekerCaps> {
    Ok(match active_for(state, uid).await? {
        Some((_, p)) => caps_from_pack(&p),
        None => SeekerCaps::default(),
    })
}

pub async fn employer_can_chat(state: &AppState, uid: u64) -> AppResult<bool> {
    let now = clock::now_ts();
    let Some(st) = phpyun_models::company_statis::repo::find_admin(state.db.reader(), uid).await?
    else {
        return Ok(false);
    };
    Ok(st.vip_etime > now)
}

pub async fn can_initiate_chat(state: &AppState, user: &AuthenticatedUser) -> AppResult<bool> {
    match user.usertype {
        1 => {
            let caps = seeker_caps(state, user.uid).await?;
            Ok(caps.chat)
        }
        2 => employer_can_chat(state, user.uid).await,
        _ => Ok(false),
    }
}

pub async fn create_order(
    state: &AppState,
    user: &AuthenticatedUser,
    package_code: &str,
    channel: &str,
    client_ip: &str,
) -> AppResult<crate::vip_service::CreatedVipOrder> {
    user.require_jobseeker()?;
    if !phpyun_models::sql::ident_ok(package_code) {
        return Err(ApiError::param_invalid("package_code"));
    }
    let pkg = pack_repo::find_by_code(state.db.reader(), package_code)
        .await?
        .ok_or_else(|| ApiError::param_invalid("unknown package"))?;
    if pkg.display != 1 {
        return Err(ApiError::param_invalid("package_inactive"));
    }
    if pkg.price_cents <= 0 {
        return Err(ApiError::param_invalid("package_inactive"));
    }
    let now = clock::now_ts();
    let order_no = vip_repo::dingdan_id(now);
    vip_repo::create_seeker_vip_order(
        state.db.pool(),
        &order_no,
        user.uid,
        &pkg.code,
        pkg.price_cents,
        channel,
        now,
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("seeker_vip.order_create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "amount": pkg.price_cents,
            })),
    )
    .await;
    Ok(crate::vip_service::CreatedVipOrder {
        order_no,
        amount_cents: pkg.price_cents,
        subject: pkg.name,
    })
}

pub async fn mark_paid(state: &AppState, order_no: &str, pay_tx_id: &str) -> AppResult<()> {
    let order = vip_repo::find_order_by_no_and_type(
        state.db.reader(),
        order_no,
        vip_repo::SEEKER_VIP_ORDER_TYPE,
    )
    .await?
    .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if order.status != 0 {
        return Err(ApiError::param_invalid("order_not_pending"));
    }
    let pkg = pack_repo::find_by_code(state.db.reader(), &order.package_code)
        .await?
        .ok_or_else(|| ApiError::internal(std::io::Error::other("package gone")))?;
    let now = clock::now_ts();
    let affected = vip_repo::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("order_already_processed"));
    }
    apply_privileges(state, order.uid, &pkg, now).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("seeker_vip.order_paid", Actor::uid(order.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "pay_tx_id": pay_tx_id,
            })),
    )
    .await;
    Ok(())
}

pub async fn apply_privileges(
    state: &AppState,
    uid: u64,
    pkg: &SeekerVipPack,
    now: i64,
) -> AppResult<()> {
    let days = i64::from(pkg.duration_days());
    vip_repo::upsert_user_vip(state.db.pool(), uid, &pkg.code, days * SECS_PER_DAY, now).await?;
    if SeekerVipPack::flag(pkg.resume_top) {
        if let Some(eid) =
            phpyun_models::resume::expect::find_default_id_by_uid(state.db.reader(), uid).await?
        {
            let Some(v) = vip_repo::find_user_vip(state.db.reader(), uid).await? else {
                return Ok(());
            };
            let _ = phpyun_models::resume::expect::set_member_top(
                state.db.pool(),
                uid,
                eid,
                v.expires_at,
            )
            .await?;
        }
    }
    Ok(())
}

pub struct PackWriteIn {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub months: i32,
    pub price_cents: i32,
    pub chat: i32,
    pub resume_top: i32,
    pub tpl_all: i32,
    pub refresh_free: i32,
    pub sort: i32,
    pub display: i32,
}

pub async fn admin_list(state: &AppState) -> AppResult<Vec<SeekerVipPack>> {
    Ok(pack_repo::list_admin(state.db.reader()).await?)
}

pub async fn admin_save(state: &AppState, actor: &AuthenticatedUser, w: PackWriteIn) -> AppResult<u64> {
    actor.require_admin()?;
    if !phpyun_models::sql::ident_ok(&w.code) {
        return Err(ApiError::param_invalid("code"));
    }
    if w.name.trim().is_empty() || w.name.chars().count() > 64 {
        return Err(ApiError::param_invalid("name"));
    }
    if !(1..=36).contains(&w.months) {
        return Err(ApiError::param_invalid("months"));
    }
    if w.price_cents <= 0 {
        return Err(ApiError::param_invalid("price_cents"));
    }
    let now = clock::now_ts();
    let row = pack_repo::PackWrite {
        code: &w.code,
        name: w.name.trim(),
        months: w.months,
        price_cents: w.price_cents,
        chat: if w.chat != 0 { 1 } else { 0 },
        resume_top: if w.resume_top != 0 { 1 } else { 0 },
        tpl_all: if w.tpl_all != 0 { 1 } else { 0 },
        refresh_free: if w.refresh_free != 0 { 1 } else { 0 },
        sort: w.sort,
        display: if w.display != 0 { 1 } else { 0 },
    };
    let id = if w.id > 0 {
        let n = pack_repo::update(state.db.pool(), w.id, &row, now).await?;
        if n == 0 {
            return Err(ApiError::param_invalid("not_found"));
        }
        w.id
    } else {
        pack_repo::insert(state.db.pool(), &row, now).await?
    };
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.seeker_vip.save", Actor::uid(actor.uid)).target(format!("pack:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn admin_delete(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    actor.require_admin()?;
    let n = pack_repo::soft_delete(state.db.pool(), id, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.seeker_vip.delete", Actor::uid(actor.uid)).target(format!("pack:{id}")),
    )
    .await;
    Ok(())
}
