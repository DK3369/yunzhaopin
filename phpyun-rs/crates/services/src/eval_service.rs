//! Career evaluation (aligned with PHPYun `wap/evaluate`).
//!
//! Question structure: `options` is a JSON array `[{label, text, score}, ...]`.
//! Users submit `answers = {"<question_id>": "<label>", ...}` and the server tallies the totals from each option's `score`.

use phpyun_core::error::InfraError;
use phpyun_core::{background, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::eval::{
    entity::{EvalLog, EvalPaper, EvalQuestion},
    repo as eval_repo,
};
use std::collections::HashMap;

pub async fn list_papers(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<EvalPaper>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        eval_repo::list_papers(db, page.offset, page.limit),
        eval_repo::count_papers(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get_paper_with_questions(
    state: &AppState,
    paper_id: u64,
) -> AppResult<(EvalPaper, Vec<EvalQuestion>)> {
    let db = state.db.reader();
    let paper = eval_repo::find_paper(db, paper_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("paper_not_found".into())))?;
    let questions = eval_repo::list_questions(db, paper_id).await?;

    let pool = state.db.pool().clone();
    background::spawn_best_effort("eval.paper.view", async move {
        let _ = eval_repo::incr_paper_visits(&pool, paper_id).await;
    });

    Ok((paper, questions))
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    paper_id: u64,
    answers: HashMap<String, String>,
) -> AppResult<(u64, i32)> {
    let db = state.db.pool();
    let reader = state.db.reader();
    let _paper = eval_repo::find_paper(reader, paper_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("paper_not_found".into())))?;

    let questions = eval_repo::list_questions(reader, paper_id).await?;
    if questions.is_empty() {
        return Err(AppError::new(InfraError::InvalidParam("no_questions".into())));
    }

    // Scoring: for each question, look up the option.score for the label submitted by the user
    let mut score: i32 = 0;
    for q in &questions {
        let qid_key = q.id.to_string();
        let Some(user_label) = answers.get(&qid_key) else { continue };
        let Some(opts) = q.options.as_array() else { continue };
        for opt in opts {
            if opt.get("label").and_then(|v| v.as_str()) == Some(user_label.as_str()) {
                if let Some(s) = opt.get("score").and_then(|v| v.as_i64()) {
                    score += s as i32;
                }
                break;
            }
        }
    }

    let answers_json = serde_json::to_value(&answers).map_err(AppError::internal)?;
    let id =
        eval_repo::create_log(db, user.uid, paper_id, score, &answers_json, clock::now_ts()).await?;
    Ok((id, score))
}

pub async fn list_my_logs(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<EvalLog>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        eval_repo::list_logs_by_user(db, user.uid, page.offset, page.limit),
        eval_repo::count_logs_by_user(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
