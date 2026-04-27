//! Public Q&A browsing (aligned with the index/list/content parts of PHPYun `wap/ask`).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, MaybeUser, Paged, Pagination, ValidatedQuery};
use validator::Validate;
use phpyun_models::qna::repo::QuestionOrder;
use phpyun_services::qna_service::{self, QuestionListFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// `unix -> Y-m-d H:i` (equivalent to PHP `date('Y-m-d H:i', $ts)`); returns empty string when ts<=0.
fn fmt_ts(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Convert the relative avatar path in the PHPYun database to a full URL (PHP `checkpic($pic)`).
fn pic_n(state: &AppState, pic: Option<&str>) -> String {
    let raw = pic.unwrap_or("");
    let site = state.config.web_base_url.as_deref();
    state.storage.normalize_legacy_url(raw, site)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/questions", get(list_questions))
        .route("/questions/{id}", get(question_detail))
        .route("/questions/{id}/answers", get(list_answers))
        .route("/qna/categories", get(list_categories))
        .route("/qna/hotweek", get(list_hotweek))
        .route("/qna/top-answerers", get(list_top_answerers))
        .route("/answers/{aid}/comments", get(list_comments))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct QListQuery {
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[validate(range(min = 0, max = 9_999_999))]
    pub category_id: Option<i32>,
    /// latest / hot
    #[serde(default = "default_order")]
    #[validate(length(min = 1, max = 16))]
    pub order: String,
}
fn default_order() -> String {
    "latest".to_string()
}

/// Question list item -- aligned with the full set of fields rendered by PHP `wap/ask::list_action`.
#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionSummary {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    /// Excerpt (first 120 chars of content; the PHP list page also truncates for display)
    pub content_excerpt: String,
    /// PHPYun `cid`
    pub category_id: i32,
    /// Category name (dict)
    pub catname: String,
    /// PHPYun `visit`
    pub hits: u32,
    /// PHPYun `answer_num`
    pub answer_count: u32,
    /// Follow count (PHPYun `atnnum`)
    pub support_count: u32,
    /// PHPYun `state`
    pub status: i32,
    /// 0=not recommended / 1=recommended
    pub is_recom: i32,
    /// Recommended boolean (PHP `is_recom_n`)
    pub is_recom_n: bool,
    /// Asker nickname (inline)
    pub nickname: Option<String>,
    /// Asker avatar (PHPYun raw `pic`)
    pub pic: Option<String>,
    /// Full asker avatar CDN URL (PHP `checkpic`)
    pub pic_n: String,
    pub created_at: i64,
    /// Formatted `add_time` string
    pub created_at_n: String,
    pub lastupdate: i64,
    /// Formatted `lastupdate` string
    pub lastupdate_n: String,
}

impl QuestionSummary {
    pub fn from_with_ctx(
        q: phpyun_models::qna::entity::Question,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
    ) -> Self {
        let pic_full = pic_n(state, q.pic.as_deref());
        let catname = dicts.question(q.category_id).to_string();
        let excerpt: String = q.content.chars().take(120).collect();
        Self {
            id: q.id,
            uid: q.uid,
            title: q.title,
            content_excerpt: excerpt,
            category_id: q.category_id,
            catname,
            hits: q.hits,
            answer_count: q.answer_count,
            support_count: q.support_count,
            status: q.status,
            is_recom: q.is_recom,
            is_recom_n: q.is_recom == 1,
            nickname: q.nickname,
            pic: q.pic,
            pic_n: pic_full,
            created_at_n: fmt_ts(q.created_at),
            created_at: q.created_at,
            lastupdate_n: fmt_ts(q.lastupdate),
            lastupdate: q.lastupdate,
        }
    }
}

/// Backward-compatible call -- convert directly from entity without dict/CDN context.
impl From<phpyun_models::qna::entity::Question> for QuestionSummary {
    fn from(q: phpyun_models::qna::entity::Question) -> Self {
        let excerpt: String = q.content.chars().take(120).collect();
        Self {
            id: q.id,
            uid: q.uid,
            title: q.title,
            content_excerpt: excerpt,
            category_id: q.category_id,
            catname: String::new(),
            hits: q.hits,
            answer_count: q.answer_count,
            support_count: q.support_count,
            status: q.status,
            is_recom: q.is_recom,
            is_recom_n: q.is_recom == 1,
            nickname: q.nickname,
            pic: q.pic.clone(),
            pic_n: q.pic.unwrap_or_default(),
            created_at_n: fmt_ts(q.created_at),
            created_at: q.created_at,
            lastupdate_n: fmt_ts(q.lastupdate),
            lastupdate: q.lastupdate,
        }
    }
}

/// Asker card (PHP `userinfo`).
#[derive(Debug, Serialize, ToSchema)]
pub struct AskerInfo {
    pub uid: u64,
    pub username: String,
    pub usertype: i32,
    /// Whether the author is followed (phpyun_atn): 0=no / 1=followed / 2=self
    pub useratn: i32,
}

/// Current viewer card (PHP `myinfo.pic`).
#[derive(Debug, Serialize, ToSchema)]
pub struct ViewerInfo {
    /// Current logged-in user's avatar (returns default when not logged in)
    pub pic_n: String,
    /// Whether logged in
    pub logged_in: bool,
}

/// Q&A detail -- fields aligned with PHP `wap/ask::content_action`.
#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionDetail {
    // === Full master-table phpyun_question fields ===
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub hits: u32,
    pub answer_count: u32,
    pub support_count: u32,
    pub status: i32,
    pub is_recom: i32,
    pub is_recom_n: bool,
    pub created_at: i64,
    pub created_at_n: String,
    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub nickname: Option<String>,
    pub pic: Option<String>,
    pub pic_n: String,
    pub ip: Option<String>,

    // === Dict translations ===
    /// Category name (PHP `catname`)
    pub catname: String,

    // === Inline top-N answers ===
    pub top_answers: Vec<AnswerItem>,

    // === Current user context (PHP `qatn / isAttention / isSupport`) ===
    /// Whether the question is followed: 0/1 (self-asked question is 2)
    pub qatn: i32,
    /// Whether followed (= alias of qatn, for backward compatibility)
    pub is_attention: i32,
    /// Whether liked (PHPYun has no detail table; always 0)
    pub is_support: i32,

    // === Asker card + viewer card ===
    pub asker: AskerInfo,
    pub viewer: ViewerInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AnswerItem {
    pub id: u64,
    pub question_id: u64,
    pub category_id: i32,
    pub uid: u64,
    pub content: String,
    pub support_count: u32,
    pub is_accepted: i32,
    /// 0=under review / 1=normal / 2=deleted
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub nickname: Option<String>,
    pub pic: Option<String>,
    pub pic_n: String,
    /// 1=job seeker / 2=company
    pub usertype: i32,
    pub comment_count: u32,
    pub oppose_count: u32,
    /// Whether the answer author is followed: 0=no / 1=followed / 2=self
    pub is_atn: i32,
}

impl AnswerItem {
    fn from_with_ctx(
        a: phpyun_models::qna::entity::Answer,
        state: &AppState,
        viewer_uid: Option<u64>,
        atn_uids: &std::collections::HashSet<u64>,
    ) -> Self {
        let pic_full = pic_n(state, a.pic.as_deref());
        let is_atn = match viewer_uid {
            Some(uid) if uid == a.uid => 2,
            Some(_) if atn_uids.contains(&a.uid) => 1,
            _ => 0,
        };
        Self {
            id: a.id,
            question_id: a.question_id,
            category_id: a.category_id,
            uid: a.uid,
            content: a.content,
            support_count: a.support_count,
            is_accepted: a.is_accepted,
            status: a.status,
            created_at_n: fmt_ts(a.created_at),
            created_at: a.created_at,
            nickname: a.nickname,
            pic: a.pic,
            pic_n: pic_full,
            usertype: a.usertype,
            comment_count: a.comment_count,
            oppose_count: a.oppose_count,
            is_atn,
        }
    }
}

/// Backward-compatible call -- no viewer context; is_atn always 0.
impl From<phpyun_models::qna::entity::Answer> for AnswerItem {
    fn from(a: phpyun_models::qna::entity::Answer) -> Self {
        Self {
            id: a.id,
            question_id: a.question_id,
            category_id: a.category_id,
            uid: a.uid,
            content: a.content,
            support_count: a.support_count,
            is_accepted: a.is_accepted,
            status: a.status,
            created_at_n: fmt_ts(a.created_at),
            created_at: a.created_at,
            nickname: a.nickname,
            pic: a.pic.clone(),
            pic_n: a.pic.unwrap_or_default(),
            usertype: a.usertype,
            comment_count: a.comment_count,
            oppose_count: a.oppose_count,
            is_atn: 0,
        }
    }
}

fn parse_order(s: &str) -> QuestionOrder {
    match s {
        "hot" => QuestionOrder::Hot,
        _ => QuestionOrder::Latest,
    }
}

/// Question list
#[utoipa::path(get, path = "/v1/wap/questions", tag = "wap", params(QListQuery), responses((status = 200, description = "ok")))]
pub async fn list_questions(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<QListQuery>,
) -> AppResult<ApiJson<Paged<QuestionSummary>>> {
    let f = QuestionListFilter {
        keyword: q.keyword.as_deref(),
        category_id: q.category_id,
        order: parse_order(&q.order),
    };
    let r = qna_service::list_questions(&state, &f, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|q| QuestionSummary::from_with_ctx(q, &state, &dicts))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Question detail (hits +1 asynchronously)
#[utoipa::path(
    get,
    path = "/v1/wap/questions/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = QuestionDetail), (status = 404))
)]
pub async fn question_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<QuestionDetail>> {
    let q = qna_service::get_question(&state, id).await?;
    let cid = q.category_id;
    let q_uid = q.uid;
    let viewer_uid = user.as_ref().map(|u| u.uid);

    // Dict + top 5 answers in parallel
    let dicts_fut = phpyun_services::dict_service::get(&state);
    let top_fut =
        phpyun_models::qna::repo::list_answers(state.db.reader(), id, 0, 5);
    let (dicts_res, top_raw) = tokio::join!(dicts_fut, top_fut);
    let dicts = dicts_res?;
    let top_raw = top_raw.unwrap_or_default();
    let catname = dicts.question(cid).to_string();
    let pic_full = pic_n(&state, q.pic.as_deref());

    // === Fetch the current user's follow list (for the question author + every answer author) in one shot ===
    let mut atn_uids: std::collections::HashSet<u64> =
        std::collections::HashSet::new();
    if let Some(uid) = viewer_uid {
        // PHP `atnM->getatnList(uid=> $this->uid, field => sc_uid)` -- list of followed users
        let rows: Vec<(i64,)> = sqlx::query_as(
            "SELECT CAST(COALESCE(sc_uid,0) AS SIGNED) FROM phpyun_atn WHERE uid = ?",
        )
        .bind(uid as i64)
        .fetch_all(state.db.reader())
        .await
        .unwrap_or_default();
        atn_uids = rows.into_iter().map(|(v,)| v.max(0) as u64).collect();
    }
    let useratn: i32 = match viewer_uid {
        Some(uid) if uid == q_uid => 2,
        Some(_) if atn_uids.contains(&q_uid) => 1,
        _ => 0,
    };

    // === Whether the question is followed (whether this question id is in phpyun_attention.ids CSV) ===
    let qatn: i32 = if let Some(uid) = viewer_uid {
        if uid == q_uid {
            2
        } else {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT COALESCE(ids,'') FROM phpyun_attention \
                 WHERE uid = ? AND type = 1 LIMIT 1",
            )
            .bind(uid as i64)
            .fetch_optional(state.db.reader())
            .await
            .unwrap_or(None);
            let ids = row.map(|(s,)| s).unwrap_or_default();
            let id_str = id.to_string();
            let hit = ids
                .split(',')
                .any(|s| !s.is_empty() && s.trim() == id_str);
            if hit {
                1
            } else {
                0
            }
        }
    } else {
        0
    };

    // === Asker card (phpyun_member.username + usertype) ===
    let asker = {
        let row: Option<(Option<String>, i32)> = sqlx::query_as(
            "SELECT username, CAST(COALESCE(usertype,0) AS SIGNED) \
             FROM phpyun_member WHERE uid = ? LIMIT 1",
        )
        .bind(q_uid as i64)
        .fetch_optional(state.db.reader())
        .await
        .unwrap_or(None);
        let (username, usertype) = row.unwrap_or((None, 0));
        AskerInfo {
            uid: q_uid,
            username: username.unwrap_or_default(),
            usertype,
            useratn,
        }
    };

    // === Viewer card (PHP `myinfo.pic`) ===
    let viewer = {
        let pic = if let Some(uid) = viewer_uid {
            let row: Option<(Option<String>,)> = sqlx::query_as(
                "SELECT photo FROM phpyun_resume WHERE uid = ? LIMIT 1",
            )
            .bind(uid as i64)
            .fetch_optional(state.db.reader())
            .await
            .unwrap_or(None);
            row.and_then(|(p,)| p)
        } else {
            None
        };
        ViewerInfo {
            pic_n: pic_n(&state, pic.as_deref()),
            logged_in: viewer_uid.is_some(),
        }
    };

    let top_answers: Vec<AnswerItem> = top_raw
        .into_iter()
        .map(|a| AnswerItem::from_with_ctx(a, &state, viewer_uid, &atn_uids))
        .collect();

    Ok(ApiJson(QuestionDetail {
        id: q.id,
        uid: q.uid,
        title: q.title,
        content: q.content,
        category_id: q.category_id,
        hits: q.hits,
        answer_count: q.answer_count,
        support_count: q.support_count,
        status: q.status,
        is_recom: q.is_recom,
        is_recom_n: q.is_recom == 1,
        created_at: q.created_at,
        created_at_n: fmt_ts(q.created_at),
        lastupdate: q.lastupdate,
        lastupdate_n: fmt_ts(q.lastupdate),
        nickname: q.nickname,
        pic: q.pic,
        pic_n: pic_full,
        ip: q.ip,
        catname,
        top_answers,
        qatn,
        is_attention: qatn,
        is_support: 0,
        asker,
        viewer,
    }))
}

/// Answer list
#[utoipa::path(
    get,
    path = "/v1/wap/questions/{id}/answers",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list_answers(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    Path(id): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<AnswerItem>>> {
    let r = qna_service::list_answers(&state, id, page).await?;
    let viewer_uid = user.as_ref().map(|u| u.uid);
    let mut atn_uids = std::collections::HashSet::<u64>::new();
    if let Some(uid) = viewer_uid {
        let rows: Vec<(i64,)> = sqlx::query_as(
            "SELECT CAST(COALESCE(sc_uid,0) AS SIGNED) FROM phpyun_atn WHERE uid = ?",
        )
        .bind(uid as i64)
        .fetch_all(state.db.reader())
        .await
        .unwrap_or_default();
        atn_uids = rows.into_iter().map(|(v,)| v.max(0) as u64).collect();
    }
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|a| AnswerItem::from_with_ctx(a, &state, viewer_uid, &atn_uids))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ========== Categories / weekly hot / comments (public read) ==========

#[derive(Debug, Serialize, ToSchema)]
pub struct CategoryItem {
    pub id: u64,
    pub name: String,
    pub pid: i32,
    pub pic: String,
    pub sort: i32,
    pub intro: String,
}

impl From<phpyun_models::qna::entity::QClass> for CategoryItem {
    fn from(c: phpyun_models::qna::entity::QClass) -> Self {
        Self {
            id: c.id,
            name: c.name,
            pid: c.pid,
            pic: c.pic.unwrap_or_default(),
            sort: c.sort,
            intro: c.intro.unwrap_or_default(),
        }
    }
}

/// Q&A category list (aligned with PHP `wap/ask::qclass_action`)
#[utoipa::path(get, path = "/v1/wap/qna/categories", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list_categories(
    State(state): State<AppState>,
) -> AppResult<ApiJson<Vec<CategoryItem>>> {
    let list = qna_service::list_categories(&state).await?;
    // Arc<Vec<QClass>> -- 60s TTL cache; cloning each entity is enough, with zero extra alloc overall
    Ok(ApiJson(list.iter().cloned().map(CategoryItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct HotweekQuery {
    /// Default 10, max 50
    #[serde(default = "default_hot_limit")]
    #[validate(range(min = 1, max = 50))]
    pub limit: u64,
}
fn default_hot_limit() -> u64 {
    10
}

/// Hot questions of the week (aligned with PHP `wap/ask::hotweek_action`)
#[utoipa::path(get, path = "/v1/wap/qna/hotweek", tag = "wap", params(HotweekQuery), responses((status = 200, description = "ok")))]
pub async fn list_hotweek(
    State(state): State<AppState>,
    ValidatedQuery(q): ValidatedQuery<HotweekQuery>,
) -> AppResult<ApiJson<Vec<QuestionSummary>>> {
    let list = qna_service::list_hotweek(&state, q.limit).await?;
    Ok(ApiJson(list.into_iter().map(QuestionSummary::from).collect()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommentItem {
    pub id: u64,
    pub aid: u64,
    pub qid: u64,
    pub uid: u64,
    /// 1=job seeker / 2=company
    pub usertype: i32,
    pub content: String,
    pub support: i32,
    pub add_time: i64,
    pub nickname: Option<String>,
    pub pic: Option<String>,
}

impl From<phpyun_models::qna::entity::AnswerReview> for CommentItem {
    fn from(r: phpyun_models::qna::entity::AnswerReview) -> Self {
        Self {
            id: r.id,
            aid: r.aid,
            qid: r.qid,
            uid: r.uid,
            usertype: r.usertype,
            content: r.content,
            support: r.support,
            add_time: r.add_time,
            nickname: r.nickname,
            pic: r.pic,
        }
    }
}

/// List comments under an answer (aligned with PHP `wap/ask::getcomment_action`)
#[utoipa::path(
    get,
    path = "/v1/wap/answers/{aid}/comments",
    tag = "wap",
    params(("aid" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list_comments(
    State(state): State<AppState>,
    Path(aid): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<CommentItem>>> {
    let r = qna_service::list_reviews(&state, aid, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(CommentItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== Top answerers leaderboard ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct TopAnswerersQuery {
    /// Lookback window in days (1..=365). PHP hard-codes 30; we expose it for
    /// flexibility but default to the same value.
    #[serde(default = "default_top_days")]
    #[validate(range(min = 1, max = 365))]
    pub days: i64,
    /// 1..=50; PHP defaults to 6.
    #[serde(default = "default_top_limit")]
    #[validate(range(min = 1, max = 50))]
    pub limit: u64,
}
fn default_top_days() -> i64 {
    30
}
fn default_top_limit() -> u64 {
    6
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TopAnswererItem {
    pub uid: u64,
    pub nickname: Option<String>,
    pub answer_count: u64,
    pub support_total: u64,
}

impl From<phpyun_services::qna_service::AnswererBrief> for TopAnswererItem {
    fn from(a: phpyun_services::qna_service::AnswererBrief) -> Self {
        Self {
            uid: a.uid,
            nickname: a.nickname,
            answer_count: a.answer_count,
            support_total: a.support_total,
        }
    }
}

/// Top answerers in the last N days — counterpart of PHP
/// `ask::getAnswersList(groupby:uid, orderby:num)` powering the "热门回答者"
/// sidebar on `ask/topic` and `ask/search` pages.
#[utoipa::path(
    get,
    path = "/v1/wap/qna/top-answerers",
    tag = "wap",
    params(TopAnswerersQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_top_answerers(
    State(state): State<AppState>,
    ValidatedQuery(q): ValidatedQuery<TopAnswerersQuery>,
) -> AppResult<ApiJson<Vec<TopAnswererItem>>> {
    let rows = qna_service::list_top_answerers(&state, q.days, q.limit).await?;
    Ok(ApiJson(rows.into_iter().map(TopAnswererItem::from).collect()))
}
