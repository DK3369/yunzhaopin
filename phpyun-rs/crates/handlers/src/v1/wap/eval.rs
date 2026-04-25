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
