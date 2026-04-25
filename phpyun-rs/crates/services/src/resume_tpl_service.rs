//! Jobseeker resume templates (`phpyun_resumetpl`): list + purchase + apply.
//!
//! Aligned with PHPYun `tpl.model::{payResumetpl, setResumetpl}` + `member/user/resumetpl`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::resume_tpl::entity::ResumeTpl;
use phpyun_models::resume_tpl::repo as tpl_repo;

pub async fn list(state: &AppState) -> AppResult<Vec<ResumeTpl>> {
    Ok(tpl_repo::list_public(state.db.reader()).await?)
}

pub struct BuyResult {
    pub tpl_id: u64,
    pub already_owned: bool,
    pub deducted_price: i32,
}

pub async fn buy(
    state: &AppState,
    user: &AuthenticatedUser,
    tpl_id: u64,
    client_ip: &str,
) -> AppResult<BuyResult> {
    user.require_jobseeker()?;
    let tpl = tpl_repo::find_by_id(state.db.reader(), tpl_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("tpl_not_found".into())))?;
    if tpl.status != 1 {
        return Err(InfraError::InvalidParam("tpl_disabled".into()).into());
    }

    // Already purchased -> return immediately
    let owned = tpl_repo::fetch_purchased_ids(state.db.reader(), user.uid).await?;
    let already_owned = owned
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .any(|s| s == tpl_id.to_string());
    if already_owned {
        return Ok(BuyResult {
            tpl_id,
            already_owned: true,
            deducted_price: 0,
        });
    }

    if tpl.price > 0 {
        let n = integral_repo::try_deduct(
            state.db.pool(),
            user.uid,
            tpl.price as u32,
            clock::now_ts(),
        )
        .await?;
        if n == 0 {
            return Err(InfraError::InvalidParam("integral_insufficient".into()).into());
        }
    }
    tpl_repo::append_purchased_id(state.db.pool(), user.uid, tpl_id).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("resume.tpl_buy", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("tpl:{tpl_id}"))
            .meta(&serde_json::json!({ "price": tpl.price })),
    )
    .await;

    Ok(BuyResult {
        tpl_id,
        already_owned: false,
        deducted_price: tpl.price,
    })
}

/// Apply a purchased template — must call `buy` before `apply`
pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    tpl_id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let owned = tpl_repo::fetch_purchased_ids(state.db.reader(), user.uid).await?;
    let ok = owned
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .any(|s| s == tpl_id.to_string());
    if !ok {
        // Allow one more case: template `price=0` means it is free
        let tpl = tpl_repo::find_by_id(state.db.reader(), tpl_id)
            .await?
            .ok_or_else(|| AppError::new(InfraError::InvalidParam("tpl_not_found".into())))?;
        if tpl.price > 0 {
            return Err(InfraError::InvalidParam("tpl_not_owned".into()).into());
        }
    }
    let n = tpl_repo::set_applied_tpl(state.db.pool(), user.uid, tpl_id).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.tpl_apply", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("tpl:{tpl_id}")),
    )
    .await;
    Ok(n)
}
