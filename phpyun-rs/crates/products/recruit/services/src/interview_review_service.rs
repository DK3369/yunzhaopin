//! Multi-dimension interview review (`phpyun_rs_interview_review`).
//! Bound to `phpyun_userid_msg.id` (true yqms invite), not generic ratings.

use phpyun_core::{clock, db, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::interview_review::repo as review_repo;
use phpyun_models::userid_msg::repo as msg_repo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dim {
    pub key: String,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewView {
    pub submitted: bool,
    pub yqms_id: u64,
    pub dimensions: Vec<Dim>,
    pub total: u32,
    pub comment: String,
}

fn empty(yqms_id: u64) -> ReviewView {
    ReviewView {
        submitted: false,
        yqms_id,
        dimensions: Vec::new(),
        total: 0,
        comment: String::new(),
    }
}

fn parse_dims(raw: &str) -> Vec<Dim> {
    serde_json::from_str::<Vec<Dim>>(raw).unwrap_or_default()
}

fn normalize(dims: &[Dim]) -> AppResult<(String, u32)> {
    if dims.is_empty() || dims.len() > 10 {
        return Err(ApiError::business("interview_review_dims"));
    }
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut sum: u32 = 0;
    for d in dims {
        let key = d.key.trim().to_ascii_lowercase();
        if key.is_empty()
            || key.len() > 32
            || !key
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '_')
        {
            return Err(ApiError::business("interview_review_dims"));
        }
        if !(1..=5).contains(&d.score) {
            return Err(ApiError::business("interview_review_dims"));
        }
        if !seen.insert(key.clone()) {
            return Err(ApiError::business("interview_review_dims"));
        }
        sum += d.score as u32;
        out.push(Dim {
            key,
            score: d.score,
        });
    }
    let total = (sum * 100) / (out.len() as u32);
    let json = serde_json::to_string(&out).unwrap_or_else(|_| "[]".into());
    Ok((json, total))
}

async fn party_or_err(
    state: &AppState,
    user: &AuthenticatedUser,
    yqms_id: u64,
) -> AppResult<(u64, u64)> {
    let row = msg_repo::find_by_id(state.db.reader(), yqms_id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    if user.uid != row.uid && user.uid != row.fid {
        return Err(ApiError::business("interview_review_not_party"));
    }
    let ratee = if user.uid == row.uid { row.fid } else { row.uid };
    Ok((user.uid, ratee))
}

pub async fn get_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    yqms_id: u64,
) -> AppResult<ReviewView> {
    if yqms_id == 0 {
        return Err(ApiError::param_missing("yqms_id"));
    }
    let (rater, _) = party_or_err(state, user, yqms_id).await?;
    let row = match review_repo::find_mine(state.db.reader(), rater, yqms_id).await {
        Ok(v) => v,
        Err(e) if db::is_missing_table(&e) => {
            return Err(ApiError::business("interview_review_unavailable"));
        }
        Err(e) => return Err(e.into()),
    };
    Ok(match row {
        None => empty(yqms_id),
        Some(r) => ReviewView {
            submitted: true,
            yqms_id: r.yqms_id,
            dimensions: parse_dims(&r.dimensions),
            total: r.total,
            comment: r.comment,
        },
    })
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    yqms_id: u64,
    dimensions: &[Dim],
    comment: &str,
) -> AppResult<ReviewView> {
    if yqms_id == 0 {
        return Err(ApiError::param_missing("yqms_id"));
    }
    let comment = comment.trim();
    if comment.chars().count() > 1000 {
        return Err(ApiError::param_invalid("comment"));
    }
    let (dims_json, total) = normalize(dimensions)?;
    let (rater, ratee) = party_or_err(state, user, yqms_id).await?;
    let now = clock::now_ts();
    if let Err(e) = review_repo::upsert(
        state.db.pool(),
        yqms_id,
        rater,
        ratee,
        &dims_json,
        total,
        comment,
        now,
    )
    .await
    {
        if db::is_missing_table(&e) {
            return Err(ApiError::business("interview_review_unavailable"));
        }
        return Err(e.into());
    }
    get_mine(state, user, yqms_id).await
}
