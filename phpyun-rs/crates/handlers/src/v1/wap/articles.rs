//! Public article / news browsing. Aligned with PHPYun `wap/article`.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use validator::Validate;
use phpyun_models::article::repo::ArticleFilter;
use phpyun_services::article_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use phpyun_core::dto::{HitsResp, IdBody};
use phpyun_core::utils::{fmt_date, pic_n_str as pic_n};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/articles", post(list_articles))
        .route("/articles/detail", post(article_detail))
        .route("/articles/hits", post(bump_hits))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ArticleListQuery {
    #[validate(length(max = 64))]
    pub category: Option<String>,
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[serde(default)]
    pub rec_only: Option<bool>,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
}
fn default_did() -> u32 {
    1
}

/// `unix → Y-m-d` (used by PHP list rendering)

/// Article list item — aligned with all fields output by PHP `article.model::getList`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ArticleSummary {
    // ==== All columns of the phpyun_news_base main table ====
    pub id: u64,
    /// PHP `nid`: category foreign key
    pub nid: i32,
    /// Category name (PHP `name`, JOIN news_group.name)
    pub category: String,
    /// Sub-site id (PHP `did`)
    pub did: u64,
    pub title: String,
    /// Truncated title (first 60 chars; PHP has `tlen` parameter to control)
    pub title_short: String,
    /// Full title (PHP `title_all`)
    pub title_all: String,
    /// Display color (PHP `color`)
    pub color: Option<String>,
    /// SEO keywords CSV
    pub keyword: String,
    pub author: String,
    /// PHP `description`: original summary text
    pub summary: String,
    /// PHP `description_n`: first 30 chars of summary
    pub summary_short: String,
    /// Relative cover path (PHP `newsphoto`)
    pub cover: String,
    /// Full URL (PHP `picurl` = checkpic($newsphoto))
    pub picurl: String,
    /// Small thumbnail (PHP `s_thumb`)
    pub s_thumb: Option<String>,
    pub source: Option<String>,
    pub sort: i32,
    pub hits: i64,
    /// Raw describe CSV value
    pub describe: String,
    /// describe CSV split (PHP `describe_arr`)
    pub describe_arr: Vec<String>,
    /// describe contains "1" → recommended
    pub rec: i32,
    pub status: i32,
    pub published_at: i64,
    /// PHP `datetime_n` (Y-m-d)
    pub datetime_n: String,
    pub lastupdate: i64,
    pub starttime: i64,
    /// PHP `starttime_n`
    pub starttime_n: String,
    pub endtime: i64,
    /// PHP `endtime_n`
    pub endtime_n: String,
}

impl ArticleSummary {
    pub fn from_with_ctx(a: phpyun_models::article::entity::Article, state: &AppState) -> Self {
        let title_all = a.title.clone();
        let title_short = a.title.chars().take(60).collect::<String>();
        let summary_short = a.summary.chars().take(30).collect::<String>();
        let describe_arr = a
            .describe
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let picurl = pic_n(state, &a.cover);
        Self {
            id: a.id,
            nid: a.nid,
            category: a.category,
            did: a.did,
            title: title_short.clone(),
            title_short,
            title_all,
            color: a.color,
            keyword: a.keyword,
            author: a.author,
            summary: a.summary,
            summary_short,
            cover: a.cover,
            picurl,
            s_thumb: a.s_thumb,
            source: a.source,
            sort: a.sort,
            hits: a.hits,
            describe: a.describe,
            describe_arr,
            rec: a.rec,
            status: a.status,
            published_at: a.published_at,
            datetime_n: fmt_date(a.published_at),
            lastupdate: a.lastupdate,
            starttime: a.starttime,
            starttime_n: fmt_date(a.starttime),
            endtime: a.endtime,
            endtime_n: fmt_date(a.endtime),
        }
    }
}

/// Article detail — all ArticleSummary fields + content. Field order matches PHP `wap/article::show_action` rendering.
#[derive(Debug, Serialize, ToSchema)]
pub struct ArticleDetail {
    pub id: u64,
    pub nid: i32,
    pub category: String,
    pub did: u64,
    pub title: String,
    pub title_short: String,
    pub title_all: String,
    pub color: Option<String>,
    pub keyword: String,
    pub author: String,
    pub summary: String,
    pub summary_short: String,
    pub cover: String,
    pub picurl: String,
    pub s_thumb: Option<String>,
    pub source: Option<String>,
    pub sort: i32,
    pub hits: i64,
    pub describe: String,
    pub describe_arr: Vec<String>,
    pub rec: i32,
    pub status: i32,
    pub published_at: i64,
    pub datetime_n: String,
    pub lastupdate: i64,
    pub starttime: i64,
    pub starttime_n: String,
    pub endtime: i64,
    pub endtime_n: String,
    /// Body HTML (phpyun_news_content.content)
    pub content: Option<String>,
}

impl ArticleDetail {
    fn from_with_ctx(a: phpyun_models::article::entity::Article, state: &AppState) -> Self {
        let content = a.content.clone();
        let s = ArticleSummary::from_with_ctx(a, state);
        Self {
            id: s.id,
            nid: s.nid,
            category: s.category,
            did: s.did,
            title: s.title,
            title_short: s.title_short,
            title_all: s.title_all,
            color: s.color,
            keyword: s.keyword,
            author: s.author,
            summary: s.summary,
            summary_short: s.summary_short,
            cover: s.cover,
            picurl: s.picurl,
            s_thumb: s.s_thumb,
            source: s.source,
            sort: s.sort,
            hits: s.hits,
            describe: s.describe,
            describe_arr: s.describe_arr,
            rec: s.rec,
            status: s.status,
            published_at: s.published_at,
            datetime_n: s.datetime_n,
            lastupdate: s.lastupdate,
            starttime: s.starttime,
            starttime_n: s.starttime_n,
            endtime: s.endtime,
            endtime_n: s.endtime_n,
            content,
        }
    }
}

/// Public article list
#[utoipa::path(
    post,
    path = "/v1/wap/articles/detail",
    tag = "wap",
    params(ArticleListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_articles(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ArticleListQuery>,
) -> AppResult<ApiJson<Paged<ArticleSummary>>> {
    let filter = ArticleFilter {
        category: q.category.as_deref(),
        keyword: q.keyword.as_deref(),
        rec_only: q.rec_only.unwrap_or(false),
        did: q.did,
    };
    let r = article_service::list_public(&state, &filter, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|a| ArticleSummary::from_with_ctx(a, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Public article detail (hits +1 in async background)
#[utoipa::path(post,
    path = "/v1/wap/articles",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn article_detail(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<ArticleDetail>> {
    let id = b.id;
    let a = article_service::get_public(&state, id).await?;
    Ok(ApiJson(ArticleDetail::from_with_ctx(a, &state)))
}

/// Bump and return the new hit count. Counterpart of PHP
/// `wap/article::GetHits_action` (PHP echoes a `document.write(...)` snippet;
/// Rust returns clean JSON).
#[utoipa::path(post,
    path = "/v1/wap/articles/hits",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = HitsResp))
)]
pub async fn bump_hits(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<HitsResp>> {
    let id = b.id;
    let hits = phpyun_models::article::repo::bump_and_get_hits(state.db.pool(), id).await?;
    Ok(ApiJson(HitsResp { hits }))
}

