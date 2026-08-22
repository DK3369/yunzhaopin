//! Peer-to-peer direct messages.

use phpyun_core::i18n::{t, Lang};
use phpyun_core::{clock, rate_limit, ApiError, AppResult, AppState, AuthenticatedUser};

// Notification copy: translated using the system default language when written to the DB; can be reworked once a recipient preferred-language is introduced.
const NOTIF_LANG: Lang = Lang::ZhCN;
use crate::notification_consumers::{ChatRead, ChatSent};
use phpyun_models::chat::{entity as chat_entity, entity::Chat, repo as chat_repo};
use phpyun_models::message::{entity as msg_entity, repo as message_repo};
use std::time::Duration;

/// Send one direct message. Rate limit: at most 20 per minute to prevent abuse.
pub async fn send(
    state: &AppState,
    user: &AuthenticatedUser,
    peer_uid: u64,
    body: &str,
) -> AppResult<u64> {
    if peer_uid == user.uid {
        return Err(ApiError::param_invalid("cannot_chat_self"));
    }
    if body.is_empty() || body.len() > 5000 {
        return Err(ApiError::param_invalid("body_length"));
    }

    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:chat:send:{}", user.uid),
        rate_limit::LimitRule {
            max: 20,
            window: Duration::from_secs(60),
        },
    )
    .await?;

    let now = clock::now_ts();
    let id = chat_repo::send(state.db.pool(), user.uid, peer_uid, body, now).await?;

    // Also push a notification into the recipient's message center
    let chat_title = t("notifications.chat.received_title", NOTIF_LANG);
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: peer_uid,
            recipient_usertype: 0, // unknown at send time; either side
            category: "chat",
            title: &chat_title,
            body: Some(&truncate(body, 80)),
            ref_kind: msg_entity::REF_NONE,
            ref_id: user.uid,
        },
        now,
    )
    .await;

    // Event bus. The body rides along so a live recipient can render the
    // message from the push alone; see `ChatSent` for why that is worth the
    // text passing through Redis.
    let _ = state
        .events
        .publish_json(
            "chat.sent",
            &ChatSent {
                id,
                sender: user.uid,
                receiver: peer_uid,
                conv_key: chat_entity::conv_key_for(user.uid, peer_uid),
                body: body.to_owned(),
                created_at: now,
            },
        )
        .await;

    Ok(id)
}

pub async fn list_with(
    state: &AppState,
    user: &AuthenticatedUser,
    peer_uid: u64,
    before_id: Option<u64>,
    limit: u64,
) -> AppResult<Vec<Chat>> {
    Ok(chat_repo::list_with_peer(state.db.reader(), user.uid, peer_uid, before_id, limit).await?)
}

pub async fn list_conversations(
    state: &AppState,
    user: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<Chat>> {
    Ok(chat_repo::list_conversations(state.db.reader(), user.uid, limit).await?)
}

pub async fn mark_read_with(
    state: &AppState,
    user: &AuthenticatedUser,
    peer_uid: u64,
) -> AppResult<u64> {
    let updated = chat_repo::mark_read_from_peer(state.db.pool(), user.uid, peer_uid).await?;

    // Only when something actually changed: re-opening a conversation you have
    // already read is the common case, and telling the peer about it every time
    // would be a stream of events that say nothing happened.
    if updated > 0 {
        let _ = state
            .events
            .publish_json(
                "chat.read",
                &ChatRead {
                    conv_key: chat_entity::conv_key_for(user.uid, peer_uid),
                    reader: user.uid,
                    peer: peer_uid,
                    at: clock::now_ts(),
                },
            )
            .await;
    }

    Ok(updated)
}

/// Messages in this user's conversations newer than `after_id`, oldest first.
///
/// This is the catch-up query behind SSE resumption: a client reports the last
/// message id it saw and gets exactly what it missed, across every conversation
/// at once rather than one thread at a time.
///
/// `limit` is a ceiling the caller uses to detect "too far behind" — ask for one
/// more than you intend to send, and a full result means the gap is wider than
/// the stream should carry.
pub async fn list_since_id(
    state: &AppState,
    uid: u64,
    after_id: u64,
    limit: u64,
) -> AppResult<Vec<Chat>> {
    Ok(chat_repo::list_after_id(state.db.reader(), uid, after_id, limit).await?)
}

pub async fn unread_count(state: &AppState, user: &AuthenticatedUser) -> AppResult<u64> {
    Ok(chat_repo::count_unread(state.db.reader(), user.uid).await?)
}

fn truncate(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}
