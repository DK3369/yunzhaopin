//! Company talent-pool business service.
//!
//! Aligned with PHPYun `app/model/resume.model.php::addTalent` + `member/com/model/talent_pool`.
//!
//! Business rules:
//! - Only companies (usertype=2) may add or remove entries in their own talent pool.
//! - The same company can add the same resume eid only once.
//! - After adding, send the jobseeker a system message (`sysmsg`) indicating "company XX has favorited your resume" —
//!   use the public `message_service` endpoint for this.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::talent_pool::entity::TalentPoolItem;
use phpyun_models::talent_pool::repo as tp_repo;

use crate::domain_errors::CompanyError;

pub struct TalentPoolPage {
    pub list: Vec<TalentPoolItem>,
    pub total: u64,
}

pub async fn add(
    state: &AppState,
    user: &AuthenticatedUser,
    eid: u64,
    seeker_uid: u64,
    remark: Option<&str>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()
        .map_err(|_| CompanyError::NotVerified)?;

    // Deduplicate: the same company cannot add the same resume twice
    if let Some(existing) = tp_repo::find_by_com_and_eid(state.db.reader(), user.uid, eid).await? {
        return Ok(existing.id);
    }

    let id = tp_repo::create(
        state.db.pool(),
        user.uid,
        seeker_uid,
        eid,
        remark,
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("talent_pool.add", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("resume:{eid}"))
            .meta(&serde_json::json!({ "seeker_uid": seeker_uid })),
    )
    .await;

    // Event: the jobseeker side may send a system notification in the future
    let _ = state
        .events
        .publish_json(
            "talent_pool.added",
            &serde_json::json!({
                "com_uid": user.uid,
                "seeker_uid": seeker_uid,
                "eid": eid,
                "id": id,
            }),
        )
        .await;

    Ok(id)
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<TalentPoolPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        tp_repo::count_by_com(state.db.reader(), user.uid),
        tp_repo::list_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(TalentPoolPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    let n = tp_repo::delete_by_ids(state.db.pool(), ids, user.uid).await?;
    Ok(n)
}

pub async fn update_remark(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    remark: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    let n = tp_repo::update_remark(state.db.pool(), id, user.uid, remark).await?;
    Ok(n)
}
