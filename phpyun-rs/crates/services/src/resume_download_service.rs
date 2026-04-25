//! Resume download flow — **company downloads resume + counter + notifies jobseeker**.
//!
//! Aligned with PHPYun `mcenter/down` (the company download action). Key behaviors:
//! - Each `(com_id, uid)` pair is recorded only once (UNIQUE constraint). Repeat downloads only refresh the timestamp.
//! - On the first download, sends a message to the jobseeker: "Company XX downloaded your resume".
//! - Contact info can be unlocked after a download (business rule; the frontend controls how it is displayed).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::i18n::{t, Lang};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};

const NOTIF_LANG: Lang = Lang::ZhCN;
use phpyun_models::message::{entity as msg_entity, repo as message_repo};
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume_download::{entity::ResumeDownload, repo as download_repo};

use crate::domain_errors::ResumeError;

pub struct DownloadPage {
    pub list: Vec<ResumeDownload>,
    pub total: u64,
}

/// Company downloads a resume
pub async fn download(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;

    // The resume must exist and be publicly accessible
    let r = resume_repo::find_public(state.db.reader(), target_uid)
        .await?
        .ok_or(ResumeError::NotFound)?;

    // Send a notification on the first download
    let first_time = !download_repo::already_downloaded(state.db.reader(), user.uid, target_uid)
        .await
        .unwrap_or(false);

    let now = clock::now_ts();
    let _ = download_repo::record(state.db.pool(), user.uid, target_uid, r.uid, now).await?;

    if first_time {
        let dl_title = t("notifications.resume.downloaded_title", NOTIF_LANG);
        let _ = message_repo::create(
            state.db.pool(),
            message_repo::MessageCreate {
                uid: target_uid,
                category: "download",
                title: &dl_title,
                body: None,
                ref_kind: msg_entity::REF_RESUME,
                ref_id: target_uid,
            },
            now,
        )
        .await;

        let _ = state
            .events
            .publish_json(
                "resume.downloaded",
                &serde_json::json!({
                    "com_id": user.uid,
                    "uid": target_uid,
                }),
            )
            .await;
    }

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.download",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("uid:{target_uid}")),
    )
    .await;

    Ok(())
}

/// Company views the resumes it has downloaded
pub async fn list_mine_as_company(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<DownloadPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        download_repo::count_for_company(state.db.reader(), user.uid),
        download_repo::list_for_company(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(DownloadPage {
        total: total?,
        list: list?,
    })
}

/// Jobseeker views who has downloaded their resume
pub async fn list_mine_as_user(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<DownloadPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        download_repo::count_for_user(state.db.reader(), user.uid),
        download_repo::list_for_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(DownloadPage {
        total: total?,
        list: list?,
    })
}
