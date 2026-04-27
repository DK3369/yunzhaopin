//! Public browsing of career assessments (aligned with PHPYun `wap/evaluate`).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination};
use phpyun_services::eval_service;
use serde::Serialize;
use utoipa::ToSchema;

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/eval-papers", get(list_papers))
        .route("/eval-papers/{id}", get(paper_detail))
        .route("/eval-papers/{id}/messages", get(list_messages))
        .route("/eval-papers/{id}/recent-examinees", get(list_recent_examinees))
}

/// Assessment list item — all 7 columns of phpyun_eval_paper + CDN URL + formatted time.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaperSummary {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub cover_n: String,
    pub visits: u32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl PaperSummary {
    pub fn from_with_ctx(
        p: phpyun_models::eval::entity::EvalPaper,
        state: &AppState,
    ) -> Self {
        Self {
            cover_n: pic_n(state, &p.cover),
            id: p.id,
            name: p.name,
            description: p.description,
            cover: p.cover,
            visits: p.visits,
            status: p.status,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
        }
    }
}

impl From<phpyun_models::eval::entity::EvalPaper> for PaperSummary {
    fn from(p: phpyun_models::eval::entity::EvalPaper) -> Self {
        Self {
            id: p.id,
            name: p.name,
            description: p.description,
            cover: p.cover.clone(),
            cover_n: p.cover,
            visits: p.visits,
            status: p.status,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
        }
    }
}

/// Question view — all 5 columns of phpyun_eval_question (with score field stripped) + paper_id / sort.
#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionView {
    pub id: u64,
    pub paper_id: u64,
    pub content: String,
    pub sort: i32,
    /// Option list (shaped like `[{label:"A", text:"xxx"}, ...]`; score not returned to prevent cheating)
    pub options: json::Value,
}

/// Assessment detail — all PaperSummary fields + nested questions.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaperDetail {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub cover_n: String,
    pub visits: u32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub questions: Vec<QuestionView>,
}

/// Strip the score field from question options (to prevent cheating).
fn strip_scores(v: &json::Value) -> json::Value {
    match v.as_array() {
        Some(arr) => json::Value::Array(
            arr.iter()
                .map(|o| {
                    let mut m = json::Map::new();
                    if let Some(label) = o.get("label") {
                        m.insert("label".into(), label.clone());
                    }
                    if let Some(text) = o.get("text") {
                        m.insert("text".into(), text.clone());
                    }
                    json::Value::Object(m)
                })
                .collect(),
        ),
        None => v.clone(),
    }
}

/// Assessment list
#[utoipa::path(get, path = "/v1/wap/eval-papers", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list_papers(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<PaperSummary>>> {
    let r = eval_service::list_papers(&state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|p| PaperSummary::from_with_ctx(p, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Assessment detail (with questions; options exclude score — backend scores after submission)
#[utoipa::path(
    get,
    path = "/v1/wap/eval-papers/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = PaperDetail))
)]
pub async fn paper_detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<PaperDetail>> {
    let (p, qs) = eval_service::get_paper_with_questions(&state, id).await?;
    let cover_n = pic_n(&state, &p.cover);
    Ok(ApiJson(PaperDetail {
        id: p.id,
        name: p.name,
        description: p.description,
        cover_n,
        cover: p.cover,
        visits: p.visits,
        status: p.status,
        created_at_n: fmt_dt(p.created_at),
        created_at: p.created_at,
        questions: qs
            .into_iter()
            .map(|q| QuestionView {
                id: q.id,
                paper_id: q.paper_id,
                content: q.content,
                sort: q.sort,
                options: strip_scores(&q.options),
            })
            .collect(),
    }))
}

// ==================== Paper messages (read) ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct PaperMessageItem {
    pub id: u64,
    pub examid: u32,
    pub uid: String,
    pub usertype: Option<i32>,
    pub message: Option<String>,
    pub ctime: i64,
    pub ctime_n: String,
}

impl From<phpyun_models::eval::repo::PaperMessage> for PaperMessageItem {
    fn from(m: phpyun_models::eval::repo::PaperMessage) -> Self {
        Self {
            id: m.id,
            examid: m.examid,
            uid: m.uid,
            usertype: m.usertype,
            message: m.message,
            ctime_n: fmt_dt(m.ctime),
            ctime: m.ctime,
        }
    }
}

/// Public list of leave-messages on an assessment paper. Counterpart of PHP
/// `evaluate.model.php::getMessageList`. Read-only — write side lives at
/// `POST /v1/mcenter/eval-papers/{id}/messages`.
#[utoipa::path(
    get,
    path = "/v1/wap/eval-papers/{id}/messages",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list_messages(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<PaperMessageItem>>> {
    let examid = id as u32;
    let pool = state.db.reader();
    let (list, total) = tokio::join!(
        phpyun_models::eval::repo::list_paper_messages(pool, examid, page.offset, page.limit),
        phpyun_models::eval::repo::count_paper_messages(pool, examid),
    );
    Ok(ApiJson(Paged::new(
        list?.into_iter().map(PaperMessageItem::from).collect(),
        total?,
        page.page,
        page.page_size,
    )))
}

// ==================== Recent examinees (social-proof sidebar) ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamineeItem {
    pub uid: u64,
    pub last_taken_at: i64,
    pub last_taken_at_n: String,
    pub papers_taken: u64,
}

impl From<phpyun_models::eval::repo::ExamineeBrief> for ExamineeItem {
    fn from(b: phpyun_models::eval::repo::ExamineeBrief) -> Self {
        Self {
            uid: b.uid,
            last_taken_at_n: fmt_dt(b.last_taken_at),
            last_taken_at: b.last_taken_at,
            papers_taken: b.papers_taken,
        }
    }
}

/// Recent examinees who have taken this paper, grouped by uid. Counterpart
/// of PHP `evaluate.model.php::getEvaluateLogList(groupby:uid, orderby:ctime,desc)`
/// — drives the "他们也参加了测评" sidebar on the result page. PHP defaults
/// `limit=12`; we accept 1..=50.
#[utoipa::path(
    get,
    path = "/v1/wap/eval-papers/{id}/recent-examinees",
    tag = "wap",
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn list_recent_examinees(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<Vec<ExamineeItem>>> {
    let rows = phpyun_models::eval::repo::list_recent_examinees(state.db.reader(), id as u32, 12)
        .await?;
    Ok(ApiJson(rows.into_iter().map(ExamineeItem::from).collect()))
}
