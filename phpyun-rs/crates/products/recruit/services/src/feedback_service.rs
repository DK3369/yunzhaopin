//! User feedback service.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, rate_limit, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::feedback::{entity::Feedback, repo as feedback_repo};
use std::time::Duration;

pub struct FeedbackPage {
    pub list: Vec<Feedback>,
    pub total: u64,
}

pub struct FeedbackInput<'a> {
    pub category: &'a str,
    pub content: &'a str,
    pub contact: &'a str,
}

/// Submit feedback. Rate-limited by uid (or ip) to at most 5 entries per 10 minutes to prevent abuse.
pub async fn submit(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    input: FeedbackInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let rl_key = match user {
        Some(u) => format!("rl:feedback:uid:{}", u.uid),
        None => format!("rl:feedback:ip:{client_ip}"),
    };
    rate_limit::check_and_incr(
        &state.redis,
        &rl_key,
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let id = feedback_repo::create(
        state.db.pool(),
        feedback_repo::FeedbackCreate {
            uid: user.map(|u| u.uid),
            category: input.category,
            content: input.content,
            contact: input.contact,
            client_ip,
        },
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "feedback.submit",
            match user {
                Some(u) => Actor::uid(u.uid).with_ip(client_ip),
                None => Actor::anonymous().with_ip(client_ip),
            },
        )
        .target(format!("feedback:{id}"))
        .meta(&serde_json::json!({ "category": input.category })),
    )
    .await;

    Ok(id)
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<FeedbackPage> {
    let (total, list) = tokio::join!(
        feedback_repo::count_by_user(state.db.reader(), user.uid),
        feedback_repo::list_by_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(FeedbackPage {
        total: total?,
        list: list?,
    })
}
