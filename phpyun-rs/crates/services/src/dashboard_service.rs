//! Member-center home aggregation (aligned with PHPYun `ajax::msgNum` + member_index counter blocks).
//!
//! Returns multi-dimensional counters in a single request so the frontend can render badges directly.

use phpyun_core::{AppResult, AppState, AuthenticatedUser};
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::chat::repo as chat_repo;
use phpyun_models::collect::repo as collect_repo;
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::interview::repo as interview_repo;
use phpyun_models::message::repo as message_repo;
use phpyun_models::sign_in::repo as sign_repo;
use phpyun_models::view::entity::{KIND_JOB as VK_JOB, KIND_RESUME as VK_RESUME};
use phpyun_models::view::repo as view_repo;

#[derive(Debug, Default)]
pub struct DashboardCounts {
    pub unread_messages: u64,
    pub unread_chats: u64,
    pub apply_count: u64,
    pub interview_count: u64,
    pub favorite_count: u64,
    pub view_count: u64,
    pub integral_balance: i32,
    pub signday: u32,
}

/// Company-side home aggregation (aligned with PHPYun `com/tongji` + `com/zhaopin` counter blocks).
#[derive(Debug, Default)]
pub struct ComDashboardCounts {
    /// Total applications/submissions received
    pub applies_received: u64,
    /// Unviewed applications (PHP `is_browse=1`)
    pub applies_unread: u64,
    /// Interview invitations sent
    pub interviews_sent: u64,
    /// Resumes downloaded by us
    pub resume_downloads: u64,
    /// Unread chat messages
    pub unread_chats: u64,
    /// Overall unread system notifications
    pub unread_messages: u64,
    /// Current integral balance
    pub integral_balance: i32,
}

pub async fn counts(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<DashboardCounts> {
    let db = state.db.reader();
    let uid = user.uid;

    // PHPYun only stores job favorites (`phpyun_fav_job`); company / resume
    // favorites have no backing table, so the dashboard total is just job-fav count.
    let (messages, chats, applies, interviews, fav_job, views_job, views_res, bal, sign) = tokio::join!(
        message_repo::count(db, uid, None, true),
        chat_repo::count_unread(db, uid),
        apply_repo::count_by_uid(db, uid, None),
        interview_repo::count_for_user(db, uid),
        collect_repo::count_by_user(db, uid),
        view_repo::count_by_viewer(db, uid, VK_JOB),
        view_repo::count_by_viewer(db, uid, VK_RESUME),
        integral_repo::get_balance(db, uid),
        sign_repo::get_user_sign(db, uid),
    );

    let fav_total = fav_job.unwrap_or(0);
    let view_total = views_job.unwrap_or(0) + views_res.unwrap_or(0);

    Ok(DashboardCounts {
        unread_messages: messages.unwrap_or(0),
        unread_chats: chats.unwrap_or(0),
        apply_count: applies.unwrap_or(0),
        interview_count: interviews.unwrap_or(0),
        favorite_count: fav_total,
        view_count: view_total,
        integral_balance: bal.map(|b| b.balance).unwrap_or(0),
        signday: sign.map(|s| s.signday).unwrap_or(0),
    })
}

/// Company-side home aggregation. Aligned with the counter block in PHPYun `member/com/index::index_action`.
pub async fn com_counts(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<ComDashboardCounts> {
    user.require_employer()?;
    let db = state.db.reader();
    let uid = user.uid;

    let applies_total_f = apply_repo::count_by_com(db, uid, apply_repo::ApplyFilter::default());
    let applies_unread_f = apply_repo::count_by_com(
        db,
        uid,
        apply_repo::ApplyFilter {
            unread_only: Some(true),
            invited_only: None,
        },
    );
    let interviews_f = interview_repo::count_for_company(db, uid);
    let downloads_f = phpyun_models::resume_download::repo::count_for_company(db, uid);
    let chats_f = chat_repo::count_unread(db, uid);
    let messages_f = message_repo::count(db, uid, None, true);
    let bal_f = integral_repo::get_balance(db, uid);

    let (applies_total, applies_unread, interviews, downloads, chats, messages, bal) = tokio::join!(
        applies_total_f,
        applies_unread_f,
        interviews_f,
        downloads_f,
        chats_f,
        messages_f,
        bal_f,
    );

    Ok(ComDashboardCounts {
        applies_received: applies_total.unwrap_or(0),
        applies_unread: applies_unread.unwrap_or(0),
        interviews_sent: interviews.unwrap_or(0),
        resume_downloads: downloads.unwrap_or(0),
        unread_chats: chats.unwrap_or(0),
        unread_messages: messages.unwrap_or(0),
        integral_balance: bal.map(|b| b.balance).unwrap_or(0),
    })
}
