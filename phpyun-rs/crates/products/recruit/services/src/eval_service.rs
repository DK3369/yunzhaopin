//! Career evaluation (aligned with PHPYun `wap/evaluate`).
//!
//! Question structure: `options` is a JSON array `[{label, text, score}, ...]`.
//! Users submit `answers = {"<question_id>": "<label>", ...}` and the server tallies the totals from each option's `score`.

use phpyun_core::{
    background, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::eval::{
    entity::{EvalLog, EvalPaper, EvalQuestion},
    repo as eval_repo,
};
use std::collections::HashMap;

fn checked_score_add(score: i32, value: i32) -> Result<i32, sqlx::Error> {
    score.checked_add(value).ok_or_else(|| {
        phpyun_core::numeric::db_conversion_error::<i32>(
            "eval.total_score",
            format!("{score} + {value}"),
            "score addition overflow",
        )
    })
}

pub async fn list_papers(state: &AppState, page: Pagination) -> AppResult<Paged<EvalPaper>> {
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
        .ok_or_else(|| ApiError::param_invalid("paper_not_found"))?;
    let questions = eval_repo::list_questions(db, paper_id).await?;

    let pool = state.db.pool().clone();
    background::spawn_best_effort("eval.paper.view", async move {
        let _ = eval_repo::incr_paper_visits(&pool, paper_id).await;
    });

    Ok((paper, questions))
}

pub async fn submit(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    paper_id: u64,
    answers: HashMap<String, String>,
    nuid: Option<&str>,
) -> AppResult<(u64, i32, Option<String>)> {
    let db = state.db.pool();
    let reader = state.db.reader();
    let paper = eval_repo::find_paper(reader, paper_id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("paper_not_found"))?;

    let questions = eval_repo::list_questions(reader, paper_id).await?;
    if questions.is_empty() {
        return Err(ApiError::param_invalid("no_questions"));
    }

    // PHP: $scores += $score['q'.$id][$_POST['q'.$id]] — option index into the score array.
    let mut score: i32 = 0;
    for q in &questions {
        let qid_key = q.id.to_string();
        let Some(user_label) = answers.get(&qid_key) else {
            continue;
        };
        let Some(opts) = q.options.as_array() else {
            continue;
        };
        for opt in opts {
            if opt.get("label").and_then(|v| v.as_str()) == Some(user_label.as_str()) {
                if let Some(s) = opt.get("score").and_then(|v| v.as_i64()) {
                    let value = phpyun_core::numeric::checked_db::<i32, _>(s, "eval.option.score")?;
                    score = checked_score_add(score, value)?;
                }
                break;
            }
        }
    }

    let uid = user.map(|u| u.uid).unwrap_or(0);
    let id = eval_repo::upsert_log(db, uid, nuid, paper_id, score, clock::now_ts()).await?;
    let comment = match_grade_comment(&paper, score);
    Ok((id, score, comment))
}

fn match_grade_comment(paper: &EvalPaper, score: i32) -> Option<String> {
    use phpyun_models::eval::php_ser::unserialize_strings;
    let froms = unserialize_strings(&paper.fromscore_raw);
    let tos = unserialize_strings(&paper.toscore_raw);
    let comments = unserialize_strings(&paper.comment_raw);
    for (i, from_s) in froms.iter().enumerate() {
        let from: i32 = from_s.parse().unwrap_or(0);
        let to: i32 = tos.get(i).and_then(|s| s.parse().ok()).unwrap_or(i32::MAX);
        if score >= from && score <= to {
            return comments.get(i).cloned().filter(|s| !s.is_empty());
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::checked_score_add;

    #[test]
    fn database_score_overflow_is_a_decode_error() {
        assert_eq!(checked_score_add(20, 30).unwrap(), 50);
        let error = checked_score_add(i32::MAX, 1).unwrap_err();
        assert!(matches!(error, sqlx::Error::Decode(_)));
        assert!(error.to_string().contains("eval.total_score"));
    }
}
