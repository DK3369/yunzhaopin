//! Referral rewards (aligned with PHPYun `reward.model.php`).
//!
//! Trigger: after a successful signup, `registration_service` calls `record_on_signup`
//! to register the invitee brought in by the inviter and award points to the inviter.
//! Idempotency is guaranteed by the UK on `invitee_uid`.

use phpyun_core::{audit, clock, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::referral::{entity::Referral, repo as ref_repo};

/// Points awarded to the inviter for each successful invitation. In production this should be read from site_setting.
const REFERRAL_POINTS: i32 = 20;

/// Called after a successful signup. On failure we only warn (no error returned) — a reward failure must not roll back the signup.
pub async fn record_on_signup(
    state: &AppState,
    inviter_uid: u64,
    invitee_uid: u64,
) {
    if inviter_uid == 0 || inviter_uid == invitee_uid {
        return;
    }
    let now = clock::now_ts();
    let affected = match ref_repo::record(
        state.db.pool(),
        inviter_uid,
        invitee_uid,
        REFERRAL_POINTS,
        now,
    )
    .await
    {
        Ok(n) => n,
        Err(e) => {
            tracing::warn!(?e, inviter_uid, invitee_uid, "referral record failed");
            return;
        }
    };

    if affected > 0 {
        if let Err(e) = integral_repo::add_balance(
            state.db.pool(),
            inviter_uid,
            REFERRAL_POINTS,
            now,
        )
        .await
        {
            tracing::warn!(?e, inviter_uid, "referral grant integral failed");
            return;
        }
        let _ = audit::emit(
            state,
            audit::AuditEvent::new("referral.grant", audit::Actor::uid(inviter_uid))
                .target(format!("invitee:{invitee_uid}"))
                .meta(&serde_json::json!({ "points": REFERRAL_POINTS })),
        )
        .await;
    }
}

#[derive(Debug, Default)]
pub struct RefSummary {
    pub count: u64,
    pub total_points: i64,
}

pub async fn summary(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<RefSummary> {
    let db = state.db.reader();
    let (count, total) = tokio::join!(
        ref_repo::count_by_inviter(db, user.uid),
        ref_repo::total_points_earned(db, user.uid),
    );
    Ok(RefSummary {
        count: count.unwrap_or(0),
        total_points: total.unwrap_or(0),
    })
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Referral>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        ref_repo::list_by_inviter(db, user.uid, page.offset, page.limit),
        ref_repo::count_by_inviter(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
