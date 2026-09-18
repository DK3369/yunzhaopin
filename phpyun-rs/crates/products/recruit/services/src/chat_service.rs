//! HTTP member chat (`phpyun_rs_chat`). Not PHP `chat_log`, not job Q&A (`phpyun_msg`).

use std::collections::HashMap;
use std::time::Duration;

use phpyun_core::{
    clock, db, rate_limit, ApiError, AppResult, AppState, AuthenticatedUser, Pagination,
};
use phpyun_models::blacklist::repo as bl_repo;
use phpyun_models::chat::entity::{ChatMessage, ConversationPreview};
use phpyun_models::chat::repo as chat_repo;
use phpyun_models::user::repo as user_repo;

pub struct ConversationItem {
    pub preview: ConversationPreview,
    pub peer_username: String,
    pub peer_usertype: i32,
}

fn peer_id(peer: u64, peer_uid: u64) -> AppResult<u64> {
    let id = if peer > 0 { peer } else { peer_uid };
    if id == 0 || id > 99_999_999 {
        return Err(ApiError::param_missing("peer"));
    }
    Ok(id)
}

fn clamp_with_limit(limit: u64) -> u64 {
    if limit == 0 {
        50
    } else {
        limit.clamp(1, 100)
    }
}

pub async fn send(
    state: &AppState,
    user: &AuthenticatedUser,
    peer: u64,
    peer_uid: u64,
    body: &str,
) -> AppResult<u64> {
    let peer = peer_id(peer, peer_uid)?;
    if peer == user.uid {
        return Err(ApiError::business("chat_self"));
    }
    let body = body.trim();
    if body.is_empty() {
        return Err(ApiError::business("chat_empty"));
    }
    if body.chars().count() > 2000 {
        return Err(ApiError::business("chat_body_too_long"));
    }

    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:chat:send:{}", user.uid),
        rate_limit::LimitRule {
            max: 30,
            window: Duration::from_secs(60),
        },
    )
    .await?;

    let db = state.db.reader();
    let (peer_user, blocked_me, blocked_them) = tokio::join!(
        user_repo::find_by_uid(db, peer),
        bl_repo::is_blocked(db, user.uid, peer),
        bl_repo::is_blocked(db, peer, user.uid),
    );
    let Some(peer_row) = peer_user? else {
        return Err(ApiError::business("chat_peer_missing"));
    };
    let me = i32::from(user.usertype);
    let them = peer_row.usertype;
    if !((me == 1 && them == 2) || (me == 2 && them == 1)) {
        return Err(ApiError::business("chat_role"));
    }
    if blocked_me? || blocked_them? {
        return Err(ApiError::business("chat_blocked"));
    }

    let exists = chat_repo::conv_exists(db, user.uid, peer).await?;
    if !exists && !crate::seeker_vip_service::can_initiate_chat(state, user).await? {
        return Err(ApiError::business("chat_need_vip"));
    }

    match chat_repo::insert(state.db.pool(), user.uid, peer, body, clock::now_ts()).await {
        Ok(id) => Ok(id),
        Err(e) if db::is_missing_table(&e) => Err(ApiError::business("chat_unavailable")),
        Err(e) => Err(e.into()),
    }
}

pub async fn with_peer(
    state: &AppState,
    user: &AuthenticatedUser,
    peer: u64,
    peer_uid: u64,
    limit: u64,
    before_id: u64,
) -> AppResult<(Vec<ChatMessage>, bool)> {
    let peer = peer_id(peer, peer_uid)?;
    if peer == user.uid {
        return Err(ApiError::business("chat_self"));
    }
    let limit = clamp_with_limit(limit);
    let key = chat_repo::conv_key(user.uid, peer);
    let mut rows = chat_repo::list_with(state.db.reader(), &key, before_id, limit + 1).await?;
    let has_more = (rows.len() as u64) > limit;
    if has_more {
        rows.truncate(limit as usize);
    }
    rows.reverse();
    Ok((rows, has_more))
}

pub async fn mark_read(
    state: &AppState,
    user: &AuthenticatedUser,
    peer: u64,
    peer_uid: u64,
) -> AppResult<u64> {
    let peer = peer_id(peer, peer_uid)?;
    if peer == user.uid {
        return Err(ApiError::business("chat_self"));
    }
    Ok(chat_repo::mark_read_from_peer(state.db.pool(), user.uid, peer).await?)
}

pub async fn unread_count(state: &AppState, user: &AuthenticatedUser) -> AppResult<u64> {
    Ok(chat_repo::count_unread(state.db.reader(), user.uid).await?)
}

pub async fn list_conversations(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<(Vec<ConversationItem>, u64)> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        chat_repo::list_conversations(db, user.uid, page.offset, page.limit),
        chat_repo::count_conversations(db, user.uid),
    );
    let list = list?;
    let total = total?;
    let mut uids: Vec<u64> = list.iter().map(|c| c.peer_uid).collect();
    uids.sort_unstable();
    uids.dedup();
    let names = chat_repo::list_member_names(db, &uids).await?;
    let mut map: HashMap<u64, (String, i32)> = HashMap::with_capacity(names.len());
    for n in names {
        map.insert(n.uid, (n.username, n.usertype));
    }
    let items = list
        .into_iter()
        .map(|preview| {
            let (peer_username, peer_usertype) = map
                .get(&preview.peer_uid)
                .cloned()
                .unwrap_or_else(|| (preview.peer_uid.to_string(), 0));
            ConversationItem {
                preview,
                peer_username,
                peer_usertype,
            }
        })
        .collect();
    Ok((items, total))
}
