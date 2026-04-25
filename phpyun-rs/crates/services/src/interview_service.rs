//! Interview-invitation service.
//!
//! - Company creates an invitation (based on an existing apply): writes `phpyun_yqmb` and automatically sends the jobseeker a message.
//! - Jobseeker views the list of invitations.
//! - Jobseeker responds (accept / decline).
//! - Company cancels an invitation.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::i18n::{t, Lang};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};

const NOTIF_LANG: Lang = Lang::ZhCN;
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::interview::{entity::Interview, repo as interview_repo};
use phpyun_models::message::entity as msg_entity;
use phpyun_models::message::repo as message_repo;

use crate::domain_errors::{ApplyError, JobError};

pub struct InterviewCreateInput<'a> {
    pub apply_id: u64,
    pub inter_time: i64,
    pub address: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub remark: Option<&'a str>,
}

pub async fn create_by_company(
    state: &AppState,
    user: &AuthenticatedUser,
    input: InterviewCreateInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    // Validate that the application exists and belongs to the current company
    let apply = apply_repo::find_by_id(state.db.reader(), input.apply_id)
        .await?
        .ok_or(ApplyError::NotFound)?;
    if apply.com_id != user.uid {
        return Err(ApplyError::NotOwner.into());
    }

    let now = clock::now_ts();
    let id = interview_repo::create(
        state.db.pool(),
        interview_repo::InterviewCreate {
            apply_id: apply.id,
            com_id: apply.com_id,
            uid: apply.uid,
            job_id: apply.job_id,
            inter_time: input.inter_time,
            address: input.address,
            linkman: input.linkman,
            linktel: input.linktel,
            remark: input.remark,
        },
        now,
    )
    .await?;

    // Synchronously mark the apply as invited (keeps the legacy `invited` flag in sync)
    let _ = apply_repo::invite(state.db.pool(), apply.id, user.uid, now).await;

    // Send a message to the jobseeker
    let invite_title = t("notifications.interview.invited_title", NOTIF_LANG);
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: apply.uid,
            category: "interview",
            title: &invite_title,
            body: input.remark,
            ref_kind: msg_entity::REF_INTERVIEW,
            ref_id: id,
        },
        now,
    )
    .await;

    // Event bus
    let _ = state
        .events
        .publish_json(
            "interview.created",
            &serde_json::json!({
                "interview_id": id,
                "com_id": user.uid,
                "uid": apply.uid,
                "apply_id": apply.id,
                "job_id": apply.job_id,
            }),
        )
        .await;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "interview.create",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("interview:{id}"))
        .meta(&serde_json::json!({
            "apply_id": apply.id,
            "inter_time": input.inter_time,
        })),
    )
    .await;

    Ok(id)
}

pub struct InterviewPage {
    pub list: Vec<Interview>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<InterviewPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        interview_repo::count_for_user(state.db.reader(), user.uid),
        interview_repo::list_for_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(InterviewPage {
        total: total?,
        list: list?,
    })
}

pub async fn list_by_company(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<InterviewPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        interview_repo::count_for_company(state.db.reader(), user.uid),
        interview_repo::list_for_company(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(InterviewPage {
        total: total?,
        list: list?,
    })
}

/// Jobseeker accept (status=1) / decline (status=2)
pub async fn respond(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    if !matches!(status, 1 | 2) {
        return Err(JobError::NotFound.into());
    }
    let affected = interview_repo::respond_by_user(state.db.pool(), id, user.uid, status).await?;
    if affected == 0 {
        return Err(ApplyError::NotOwner.into());
    }

    // Notify the company
    if let Ok(Some(iv)) = interview_repo::find_by_id(state.db.reader(), id).await {
        let title = if status == 1 {
            t("notifications.interview.accepted_by_jobseeker", NOTIF_LANG)
        } else {
            t("notifications.interview.declined_by_jobseeker", NOTIF_LANG)
        };
        let _ = message_repo::create(
            state.db.pool(),
            message_repo::MessageCreate {
                uid: iv.com_id,
                category: "interview",
                title: &title,
                body: None,
                ref_kind: msg_entity::REF_INTERVIEW,
                ref_id: id,
            },
            clock::now_ts(),
        )
        .await;
    }

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "interview.respond",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("interview:{id}"))
        .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

pub async fn cancel(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = interview_repo::cancel_by_company(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(ApplyError::NotOwner.into());
    }
    // Notify the jobseeker
    if let Ok(Some(iv)) = interview_repo::find_by_id(state.db.reader(), id).await {
        let cancel_title = t("notifications.interview.cancelled_by_company", NOTIF_LANG);
        let _ = message_repo::create(
            state.db.pool(),
            message_repo::MessageCreate {
                uid: iv.uid,
                category: "interview",
                title: &cancel_title,
                body: None,
                ref_kind: msg_entity::REF_INTERVIEW,
                ref_id: id,
            },
            clock::now_ts(),
        )
        .await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "interview.cancel",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("interview:{id}")),
    )
    .await;
    Ok(())
}
