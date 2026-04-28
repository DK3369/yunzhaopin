//! Home page aggregation (aligned with PHPYun `wap/index::index`).

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::home_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::utils::{fmt_date};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/home", post(home))
        .route("/home/aggregate", post(aggregate))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HomeQuery {
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
}
fn default_did() -> u32 { 0 }

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
    /// Reuses `wap::companies::CompanySummary` (17 dictionary-translated fields)
    pub rec_companies: Vec<super::companies::CompanySummary>,
    /// Reuses `wap::articles::ArticleSummary` (27 fields, includes category name / CDN URL / formatted time)
    pub new_articles: Vec<super::articles::ArticleSummary>,
    pub hot_keywords: Vec<HotKeyword>,
}


/// Home page
#[utoipa::path(post, path = "/v1/wap/home", tag = "wap", params(HomeQuery), responses((status = 200, description = "ok", body = HomeData)))]
pub async fn home(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<HomeQuery>,
) -> AppResult<ApiJson<HomeData>> {
    let p = home_service::home(&state, q.did).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    // Take ownership for `into_iter`; HomePayload is small (5 short vecs of entities)
    // so this clone is cheaper than refactoring all the From conversions to borrow.
    let p = (*p).clone();

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
        rec_companies: p
            .rec_companies
            .into_iter()
            .map(|c| super::companies::company_summary_from_dict(c, &dicts))
            .collect(),
        new_articles: p
            .new_articles
            .into_iter()
            .map(|a| super::articles::ArticleSummary::from_with_ctx(a, &state))
            .collect(),
        hot_keywords: p
            .hot_keywords
            .into_iter()
            .map(|h| HotKeyword {
                keyword: h.keyword,
                hits: h.hits,
            })
            .collect(),
    };
    Ok(ApiJson(data))
}

// ==================== /home/aggregate ====================
//
// Lightweight home aggregation -- only returns **static operations content**: ad slots + nav + hot search + announcements + friend links.
//
// Recommended jobs / recommended companies are **not here** -- they involve paid recommendation / activity reward logic and are
// served by their own dedicated endpoints (`/v1/wap/jobs?rec=1`, `/v1/wap/companies?rec=1`, etc.).

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct AggregateQuery {
    /// Sub-site id
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
    /// Ad slot (string corresponding to phpyun_ad.class_id)
    #[validate(length(max = 100))]
    pub slot: Option<String>,
    /// Nav position (corresponds to phpyun_navigation.`type`); defaults to 1 (top) when omitted
    #[serde(default = "default_nav_position")]
    #[validate(length(min = 1, max = 16))]
    pub nav: String,
    /// Take top N hot keywords; default 10
    #[serde(default = "default_hot_limit")]
    #[validate(range(min = 1, max = 100))]
    pub hot_limit: u64,
    /// Hot search scope (string form of PHP `phpyun_hot_key.type`); default "0" (site-wide)
    #[serde(default = "default_hot_scope")]
    #[validate(length(min = 1, max = 16))]
    pub hot_scope: String,
}
fn default_nav_position() -> String {
    "1".to_string()
}
fn default_hot_limit() -> u64 {
    10
}
fn default_hot_scope() -> String {
    "0".to_string()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdItem {
    pub id: u64,
    pub title: String,
    pub image: String,
    /// Full URL after normalizing `image` (with site / CDN prefix)
    pub image_n: String,
    pub link: String,
    pub weight: i32,
    pub start_at: i64,
    pub end_at: i64,
    /// 1 = current window / 2 = new window
    pub target: i32,
    pub pic_width: String,
    pub pic_height: String,
    pub pic_content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NavItem {
    pub id: u64,
    pub label: String,
    pub url: String,
    pub icon: String,
    pub parent_id: u64,
    pub sort: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FriendLinkItem {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub logo: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AggregateData {
    pub ads: Vec<AdItem>,
    pub nav: Vec<NavItem>,
    pub hot_keywords: Vec<HotKeyword>,
    pub announcements: Vec<AnnouncementSummary>,
    pub friend_links: Vec<FriendLinkItem>,
}

#[utoipa::path(
    post,
    path = "/v1/wap/home/aggregate",
    tag = "wap",
    params(AggregateQuery),
    responses((status = 200, description = "ok", body = AggregateData))
)]
pub async fn aggregate(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<AggregateQuery>,
) -> AppResult<ApiJson<AggregateData>> {
    let db = state.db.reader();
    let now = phpyun_core::clock::now_ts();
    let _ = q.did; // current dictionaries / friend links are fetched site-wide; sub-site isolation will come in the next round

    // 5 parallel queries: ads / nav / hot_keys / announcements / friend_links
    let slot = q.slot.unwrap_or_default();
    let slot_ref: Option<&str> = if slot.is_empty() { None } else { Some(&slot) };
    let ads_fut = async {
        match slot_ref {
            Some(s) => phpyun_models::ad::repo::list_active(db, s, now, 10).await,
            None => Ok(vec![]),
        }
    };
    let nav_fut = phpyun_models::nav_menu::repo::list_public(db, &q.nav);
    let hot_fut =
        phpyun_models::hot_search::repo::top(db, &q.hot_scope, q.hot_limit);
    let ann_fut =
        phpyun_models::announcement::repo::list_published(db, 0, 5);
    let links_fut = phpyun_models::friend_link::repo::list_active(db, None);

    let (ads, nav, hots, anns, links) =
        tokio::join!(ads_fut, nav_fut, hot_fut, ann_fut, links_fut);

    let ads = ads.unwrap_or_default();
    let nav = nav.unwrap_or_default();
    let hots = hots.unwrap_or_default();
    let anns = anns.unwrap_or_default();
    let links = links.unwrap_or_default();

    let site_base = state.config.web_base_url.as_deref();
    Ok(ApiJson(AggregateData {
        ads: ads
            .into_iter()
            .map(|a| AdItem {
                image_n: state.storage.normalize_legacy_url(&a.image, site_base),
                id: a.id,
                title: a.title,
                image: a.image,
                link: a.link,
                weight: a.weight,
                start_at: a.start_at,
                end_at: a.end_at,
                target: a.target,
                pic_width: a.pic_width,
                pic_height: a.pic_height,
                pic_content: a.pic_content,
            })
            .collect(),
        nav: nav
            .into_iter()
            .map(|n| NavItem {
                id: n.id,
                label: n.label,
                url: n.url,
                icon: n.icon,
                parent_id: n.parent_id,
                sort: n.sort,
            })
            .collect(),
        hot_keywords: hots
            .into_iter()
            .map(|h| HotKeyword {
                keyword: h.keyword,
                hits: h.hits,
            })
            .collect(),
        announcements: anns
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
        friend_links: links
            .into_iter()
            .map(|l| FriendLinkItem {
                id: l.id,
                name: l.name,
                url: l.url,
                logo: l.logo,
            })
            .collect(),
    }))
}
