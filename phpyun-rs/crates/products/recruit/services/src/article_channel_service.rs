//! News-channel subscriptions (`phpyun_rs_article_channel_sub`).
//! Not the job-alert mailbox at `/v1/wap/subscribe`.

use phpyun_core::{clock, db, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::article::entity::NewsGroup;
use phpyun_models::article::repo as article_repo;
use phpyun_models::article_channel::repo as sub_repo;
use std::collections::HashSet;

pub struct ChannelItem {
    pub id: u64,
    pub name: String,
    pub keyid: i32,
    pub subscribed: bool,
}

pub struct ChannelList {
    pub list: Vec<ChannelItem>,
    pub ids: Vec<u64>,
}

fn parse_ids(raw: &str) -> Vec<u64> {
    serde_json::from_str::<Vec<u64>>(raw).unwrap_or_default()
}

fn valid_ids(ids: &[u64], groups: &[NewsGroup]) -> AppResult<Vec<u64>> {
    if ids.len() > 100 {
        return Err(ApiError::business("article_channel_ids"));
    }
    let known: HashSet<u64> = groups.iter().map(|g| g.id).collect();
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for id in ids {
        if *id == 0 || !known.contains(id) {
            return Err(ApiError::business("article_channel_ids"));
        }
        if seen.insert(*id) {
            out.push(*id);
        }
    }
    Ok(out)
}

pub async fn list(state: &AppState, user: &AuthenticatedUser) -> AppResult<ChannelList> {
    let groups = article_repo::list_groups(state.db.reader()).await?;
    let sub = match sub_repo::find(state.db.reader(), user.uid).await {
        Ok(v) => v,
        Err(e) if db::is_missing_table(&e) => {
            return Err(ApiError::business("article_channel_unavailable"));
        }
        Err(e) => return Err(e.into()),
    };
    let ids = sub
        .as_ref()
        .map(|s| parse_ids(&s.group_ids))
        .unwrap_or_default();
    let set: HashSet<u64> = ids.iter().copied().collect();
    let list = groups
        .into_iter()
        .map(|g| ChannelItem {
            subscribed: set.contains(&g.id),
            id: g.id,
            name: g.name,
            keyid: g.keyid,
        })
        .collect();
    Ok(ChannelList { list, ids })
}

pub async fn save(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<ChannelList> {
    let groups = article_repo::list_groups(state.db.reader()).await?;
    let ids = valid_ids(ids, &groups)?;
    let json = serde_json::to_string(&ids).unwrap_or_else(|_| "[]".into());
    if let Err(e) = sub_repo::upsert(state.db.pool(), user.uid, &json, clock::now_ts()).await {
        if db::is_missing_table(&e) {
            return Err(ApiError::business("article_channel_unavailable"));
        }
        return Err(e.into());
    }
    list(state, user).await
}
