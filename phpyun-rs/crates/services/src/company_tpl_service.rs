//! Company template (skin) catalog + purchase/apply.
//!
//! Aligned with PHPYun `member/com/comtpl::settpl_action` + `tpl.model`.
//!
//! Business rules:
//! - On first apply of a template, check whether `member_statis.comtpl_all` already lists it.
//! - If not yet purchased, deduct the corresponding price (integral when `type=1` or balance when `type=2`) and append to `comtpl_all`.
//! - Then, whether first apply or repeat, update `member_statis.comtpl = url`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_models::company_tpl::entity::CompanyTpl;
use phpyun_models::company_tpl::repo as tpl_repo;
use phpyun_models::integral::repo as integral_repo;

pub async fn list(state: &AppState) -> AppResult<Vec<CompanyTpl>> {
    let list = tpl_repo::list_public(state.db.reader()).await?;
    Ok(list)
}

pub struct ApplyResult {
    pub tpl_id: u64,
    pub tpl_url: String,
    pub newly_purchased: bool,
    pub deducted_price: i32,
    /// 1 = integral / 2 = balance
    pub deducted_kind: i32,
}

pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    tpl_id: u64,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    user.require_employer()?;

    let tpl = tpl_repo::find_by_id(state.db.reader(), tpl_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("tpl_not_found".into())))?;
    if tpl.status != 1 {
        return Err(InfraError::InvalidParam("tpl_disabled".into()).into());
    }

    // Check whether already purchased
    let purchased_all = tpl_repo::fetch_purchased_urls(state.db.reader(), user.uid).await?;
    let already_purchased = purchased_all
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim())
        .any(|s| s == tpl.url);

    let mut newly_purchased = false;
    if !already_purchased && tpl.price > 0 {
        // Atomic deduction: when the balance is insufficient affected=0 -> insufficient-balance error
        let affected = integral_repo::try_deduct(
            state.db.pool(),
            user.uid,
            tpl.price as u32,
            clock::now_ts(),
        )
        .await?;
        if affected == 0 {
            return Err(AppError::new(InfraError::InvalidParam(
                "integral_insufficient".into(),
            )));
        }
        tpl_repo::append_purchased_url(state.db.pool(), user.uid, &tpl.url).await?;
        newly_purchased = true;
    }

    // Apply
    tpl_repo::set_applied_tpl(state.db.pool(), user.uid, &tpl.url).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "company.tpl_apply",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("tpl:{tpl_id}"))
        .meta(&serde_json::json!({
            "url": tpl.url,
            "newly_purchased": newly_purchased,
            "price": tpl.price,
            "kind": tpl.r#type,
        })),
    )
    .await;

    Ok(ApplyResult {
        tpl_id: tpl.id,
        tpl_url: tpl.url,
        newly_purchased,
        deducted_price: if newly_purchased { tpl.price } else { 0 },
        deducted_kind: tpl.r#type,
    })
}
