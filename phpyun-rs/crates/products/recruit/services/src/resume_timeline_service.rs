//! Resume timeline — merges views / downloads / interviews into a single activity stream sorted by time descending.
//!
//! Powers the "my resume activity" view for jobseekers. Pulls top N from each category and merges them;
//! at scale this should be backed by a unified stream in Elasticsearch / ClickHouse instead.

use phpyun_core::{AppResult, AppState, AuthenticatedUser};
use phpyun_models::interview::repo as interview_repo;
use phpyun_models::resume_download::repo as download_repo;
use phpyun_models::view::entity::KIND_RESUME;
use phpyun_models::view::repo as view_repo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub kind: &'static str,
    pub ts: i64,
    pub actor_uid: u64,
    pub ref_id: u64,
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    limit: usize,
) -> AppResult<Vec<TimelineEvent>> {
    user.require_jobseeker()?;
    let db = state.db.reader();
    let per = limit.clamp(5, 100) as u64;

    let (views, downloads, interviews) = tokio::join!(
        view_repo::list_by_target(db, KIND_RESUME, user.uid, 0, per),
        download_repo::list_for_user(db, user.uid, 0, per),
        interview_repo::list_for_user(db, user.uid, 0, per),
    );

    let mut out: Vec<TimelineEvent> = Vec::new();
    if let Ok(v) = views {
        for x in v {
            out.push(TimelineEvent {
                kind: "view",
                ts: x.datetime,
                actor_uid: x.viewer_uid,
                ref_id: 0,
            });
        }
    }
    if let Ok(d) = downloads {
        for x in d {
            out.push(TimelineEvent {
                kind: "download",
                ts: x.datetime,
                actor_uid: x.com_id,
                ref_id: x.id,
            });
        }
    }
    if let Ok(iv) = interviews {
        for x in iv {
            out.push(TimelineEvent {
                kind: "interview",
                ts: x.created_at,
                actor_uid: x.com_id,
                ref_id: x.id,
            });
        }
    }

    // newest first
    out.sort_by_key(|e| std::cmp::Reverse(e.ts));
    out.truncate(limit);
    Ok(out)
}
