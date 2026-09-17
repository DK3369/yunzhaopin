//! Home page aggregation (aligned with PHPYun `wap/index::index`).

use axum::{extract::State, routing::get, Router};
use phpyun_core::utils::fmt_date;
use phpyun_core::{ApiResponse, AppResult, AppState, ValidatedJsonOrQuery};
use phpyun_services::{ad_service, friend_link_service, home_service};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/home", "/v1/wap/home/full"];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/home", get(home).post(home))
        .route("/home/full", get(home_full).post(home_full))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HomeQuery {
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
    /// PHP `?tpltype=` homepage theme preview id.
    #[serde(default)]
    pub tpltype: u64,
}
fn default_did() -> u32 {
    0
}

/// Announcement entry -- fields aligned with the phpyun_announcement table.
#[derive(Debug, Serialize, ToSchema)]
pub struct AnnouncementSummary {
    pub id: u64,
    pub title: String,
    /// Summary (PHPYun `description`)
    pub description: String,
    /// View count (PHPYun `view_num`)
    pub view_num: u32,
    /// Publish time (unix)
    pub datetime: i64,
    /// Formatted time (Y-m-d)
    pub datetime_n: String,
    /// Publish-on time
    pub startime: i64,
    /// Take-down time
    pub endtime: i64,
    /// Sub-site id
    pub did: u64,
    pub status: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HotKeyword {
    pub keyword: String,
    pub hits: i32,
}

/// Home page aggregated data -- reuses the rich Summary types from each domain to keep field semantics consistent with list / detail pages.
#[derive(Debug, Serialize, ToSchema)]
pub struct HomeData {
    pub announcements: Vec<AnnouncementSummary>,
    /// Reuses `wap::jobs::JobSummary` (30+ dictionary-translated fields)
    pub hot_jobs: Vec<super::jobs::JobSummary>,
    pub rec_jobs: Vec<super::jobs::JobSummary>,
    pub latest_jobs: Vec<super::jobs::JobSummary>,
    pub urgent_jobs: Vec<super::jobs::JobSummary>,
    pub bid_jobs: Vec<super::jobs::JobSummary>,
    /// Reuses `wap::companies::CompanySummary` (famous companies from `phpyun_hotjob`)
    pub rec_companies: Vec<super::companies::CompanySummary>,
    /// Reuses `wap::articles::ArticleSummary` (27 fields, includes category name / CDN URL / formatted time)
    pub new_articles: Vec<super::articles::ArticleSummary>,
    /// PHP `{yun:}article type=t pic=1 limit=2{/yun}`
    pub featured_articles: Vec<super::articles::ArticleSummary>,
    /// PHP `{yun:}article type=indextj limit=10{/yun}`
    pub hot_articles: Vec<super::articles::ArticleSummary>,
    pub hot_keywords: Vec<HotKeyword>,
    pub index_tpl: Option<IndexTplView>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IndexTplView {
    pub id: u64,
    pub pic: String,
    pub height: i32,
    pub se: i32,
}

/// Home page
#[utoipa::path(post, path = "/v1/wap/home", tag = "wap", params(HomeQuery), responses((status = 200, description = "ok", body = HomeData)))]
pub async fn home(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<HomeQuery>,
) -> AppResult<ApiResponse<HomeData>> {
    Ok(ApiResponse::data(assemble_home(&state, &q).await?))
}

async fn assemble_home(state: &AppState, q: &HomeQuery) -> AppResult<HomeData> {
    let p = home_service::home(state, q.did).await?;
    let dicts = phpyun_services::dict_service::get(state).await?;
    let now = phpyun_core::clock::now_ts();
    // Take ownership for `into_iter`; HomePayload is small (5 short vecs of entities)
    // so this clone is cheaper than refactoring all the From conversions to borrow.
    let p = (*p).clone();
    let index_row = if q.tpltype > 0 {
        phpyun_models::admin_gap::extra::php_find_index_tpl(state.db.reader(), q.tpltype)
            .await
            .ok()
            .flatten()
    } else {
        phpyun_models::admin_gap::extra::php_active_index_tpl(state.db.reader(), now)
            .await
            .ok()
            .flatten()
    };

    let data = HomeData {
        announcements: p
            .announcements
            .into_iter()
            .map(|a| AnnouncementSummary {
                id: a.id,
                title: a.title,
                description: a.description,
                view_num: a.view_num,
                datetime_n: fmt_date(a.datetime),
                datetime: a.datetime,
                startime: a.startime,
                endtime: a.endtime,
                did: a.did,
                status: a.status,
            })
            .collect(),
        hot_jobs: p
            .hot_jobs
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        rec_jobs: p
            .rec_jobs
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        latest_jobs: p
            .latest_jobs
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        urgent_jobs: p
            .urgent_jobs
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        bid_jobs: p
            .bid_jobs
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        rec_companies: {
            let pics = p.rec_hot_pics;
            let mut list: Vec<super::companies::CompanySummary> = p
                .rec_companies
                .into_iter()
                .map(|c| super::companies::company_summary_from_dict(c, &dicts))
                .collect();
            for row in &mut list {
                if let Some(pic) = pics.get(&row.uid) {
                    row.hot_pic = Some(pic.clone());
                }
            }
            super::companies::fill_job_nums(state, &mut list).await;
            super::companies::fill_open_jobs(state, &mut list, 3).await;
            list
        },
        new_articles: p
            .new_articles
            .into_iter()
            .map(|a| super::articles::ArticleSummary::from_with_ctx(a, state))
            .collect(),
        featured_articles: p
            .featured_articles
            .into_iter()
            .map(|a| super::articles::ArticleSummary::from_with_ctx(a, state))
            .collect(),
        hot_articles: p
            .hot_articles
            .into_iter()
            .map(|a| super::articles::ArticleSummary::from_with_ctx(a, state))
            .collect(),
        hot_keywords: p
            .hot_keywords
            .into_iter()
            .map(|h| HotKeyword {
                keyword: h.keyword,
                hits: h.hits,
            })
            .collect(),
        index_tpl: index_row.map(|r| IndexTplView {
            id: r.id,
            pic: r.pic,
            height: r.height,
            se: r.se,
        }),
    };
    Ok(data)
}

const DEFAULT_HOME_SLOTS: &[(&str, u64)] = &[
    ("3", 5),
    ("50", 5),
    ("13", 3),
    ("14", 3),
    ("15", 3),
    ("72", 1),
    ("73", 1),
    ("92", 5),
    ("503", 3),
    ("506", 1),
    ("502", 1),
    ("10", 1),
    ("11", 1),
];

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HomeFullQuery {
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
    #[serde(default)]
    pub tpltype: u64,
    /// Ad slots; omit to use the 13 homepage slots from `index.vue`.
    #[serde(default)]
    #[validate(length(max = 32), nested)]
    pub slots: Vec<super::ads::AdQuery>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HomeFullData {
    pub home: HomeData,
    pub job_cats: Vec<super::categories::CatNode>,
    pub hot_job_class: Vec<super::categories::CatNode>,
    pub ads: BTreeMap<String, Vec<super::ads::AdView>>,
    pub friend_links: Vec<super::links::LinkItem>,
}

/// Homepage bundle: `home` + job categories + hot class + ads + friend links.
#[utoipa::path(
    post,
    path = "/v1/wap/home/full",
    tag = "wap",
    params(HomeFullQuery),
    responses((status = 200, description = "ok", body = HomeFullData))
)]
pub async fn home_full(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<HomeFullQuery>,
) -> AppResult<ApiResponse<HomeFullData>> {
    let home_q = HomeQuery {
        did: q.did,
        tpltype: q.tpltype,
    };
    let home_fut = assemble_home(&state, &home_q);
    let cats_fut = super::categories::load_kind(&state, "job");
    let hot_fut = super::categories::load_recommended(&state, "job", 20);
    let ads_fut = async {
        if q.slots.is_empty() {
            super::ads::load_map(&state, DEFAULT_HOME_SLOTS).await
        } else {
            let needs: Vec<ad_service::SlotNeed> = q
                .slots
                .iter()
                .map(|s| ad_service::SlotNeed {
                    slot: s.slot.clone(),
                    limit: s.limit,
                })
                .collect();
            super::ads::load_map_needs(&state, &needs).await
        }
    };
    let links_fut = friend_link_service::list(&state, None);
    let (home, job_cats, hot_job_class, ads, links) =
        tokio::join!(home_fut, cats_fut, hot_fut, ads_fut, links_fut);
    Ok(ApiResponse::data(HomeFullData {
        home: home?,
        job_cats: job_cats.unwrap_or_else(|_| Vec::new()),
        hot_job_class: hot_job_class.unwrap_or_else(|_| Vec::new()),
        ads: ads.unwrap_or_else(|_| BTreeMap::new()),
        friend_links: links
            .ok()
            .map(|v| v.iter().cloned().map(super::links::LinkItem::from).collect())
            .unwrap_or_default(),
    }))
}
