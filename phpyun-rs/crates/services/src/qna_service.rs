//! Q&A service (aligned with PHPYun ask_controller).
//!
//! Public: list / detail / top. Authenticated: ask / answer / follow / like / accept / delete.
//!
//! View counters are fire-and-forget so they never block the main path.

use phpyun_core::error::InfraError;
use phpyun_core::{background, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::qna::{
    entity::{Answer, AnswerReview, QClass, Question, SUPPORT_KIND_ANSWER, SUPPORT_KIND_QUESTION},
    repo as qna_repo,
};

pub struct QuestionListFilter<'a> {
    pub keyword: Option<&'a str>,
    pub category_id: Option<i32>,
    pub order: qna_repo::QuestionOrder,
}

pub async fn list_questions(
    state: &AppState,
    f: &QuestionListFilter<'_>,
    page: Pagination,
) -> AppResult<Paged<Question>> {
    let db = state.db.reader();
    let filter = qna_repo::QuestionFilter {
        keyword: f.keyword,
        category_id: f.category_id,
        order: match f.order {
            qna_repo::QuestionOrder::Hot => qna_repo::QuestionOrder::Hot,
            qna_repo::QuestionOrder::Latest => qna_repo::QuestionOrder::Latest,
        },
    };
    let list = qna_repo::list_questions(db, &filter, page.offset, page.limit).await?;
    let total = qna_repo::count_questions(db, &filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_question(state: &AppState, id: u64) -> AppResult<Question> {
    let q = qna_repo::find_question(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("question_not_found".into())))?;
    if q.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("question_unavailable".into())));
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("qna.question.hit", async move {
        let _ = qna_repo::incr_question_hit(&pool, id).await;
    });
    Ok(q)
}

pub struct CreateQuestionInput<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub category_id: i32,
}

pub async fn create_question(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateQuestionInput<'_>,
) -> AppResult<u64> {
    let id = qna_repo::create_question(
        state.db.pool(),
        qna_repo::QuestionCreate {
            uid: user.uid,
            title: input.title,
            content: input.content,
            category_id: input.category_id,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn delete_question(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let affected = qna_repo::delete_question(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn list_answers(
    state: &AppState,
    question_id: u64,
    page: Pagination,
) -> AppResult<Paged<Answer>> {
    let db = state.db.reader();
    let list = qna_repo::list_answers(db, question_id, page.offset, page.limit).await?;
    let total = qna_repo::count_answers(db, question_id).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn answer(
    state: &AppState,
    user: &AuthenticatedUser,
    question_id: u64,
    content: &str,
) -> AppResult<u64> {
    // The question must exist and be published
    let q = qna_repo::find_question(state.db.reader(), question_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("question_not_found".into())))?;
    if q.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("question_unavailable".into())));
    }
    let id = qna_repo::create_answer(
        state.db.pool(),
        qna_repo::AnswerCreate {
            question_id,
            uid: user.uid,
            content,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn accept_answer(
    state: &AppState,
    user: &AuthenticatedUser,
    question_id: u64,
    answer_id: u64,
) -> AppResult<()> {
    let q = qna_repo::find_question(state.db.reader(), question_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("question_not_found".into())))?;
    if q.uid != user.uid {
        return Err(AppError::new(InfraError::Forbidden));
    }
    qna_repo::mark_answer_accepted(state.db.pool(), answer_id, question_id).await?;
    Ok(())
}

pub async fn toggle_attention(
    state: &AppState,
    user: &AuthenticatedUser,
    question_id: u64,
) -> AppResult<bool> {
    Ok(qna_repo::toggle_attention(
        state.db.pool(),
        user.uid,
        question_id,
        clock::now_ts(),
    )
    .await?)
}

pub async fn list_attended(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Question>> {
    let db = state.db.reader();
    let list =
        qna_repo::list_attended_questions(db, user.uid, page.offset, page.limit).await?;
    let total = qna_repo::count_attended_questions(db, user.uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn toggle_support_question(
    state: &AppState,
    user: &AuthenticatedUser,
    question_id: u64,
) -> AppResult<bool> {
    Ok(qna_repo::toggle_support(
        state.db.pool(),
        user.uid,
        SUPPORT_KIND_QUESTION,
        question_id,
        clock::now_ts(),
    )
    .await?)
}

pub async fn toggle_support_answer(
    state: &AppState,
    user: &AuthenticatedUser,
    answer_id: u64,
) -> AppResult<bool> {
    Ok(qna_repo::toggle_support(
        state.db.pool(),
        user.uid,
        SUPPORT_KIND_ANSWER,
        answer_id,
        clock::now_ts(),
    )
    .await?)
}

pub async fn list_my_questions(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Question>> {
    let db = state.db.reader();
    let list =
        qna_repo::list_questions_by_user(db, user.uid, page.offset, page.limit).await?;
    let total = qna_repo::count_questions_by_user(db, user.uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_my_answers(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<Answer>> {
    let db = state.db.reader();
    let list = qna_repo::list_answers_by_user(db, user.uid, page.offset, page.limit).await?;
    let total = qna_repo::count_answers_by_user(db, user.uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

// ==================== Reviews/comments (phpyun_answer_review) ====================

pub async fn list_reviews(
    state: &AppState,
    aid: u64,
    page: Pagination,
) -> AppResult<Paged<AnswerReview>> {
    let db = state.db.reader();
    let list = qna_repo::list_reviews_by_answer(db, aid, page.offset, page.limit).await?;
    let total = qna_repo::count_reviews_by_answer(db, aid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn add_review(
    state: &AppState,
    user: &AuthenticatedUser,
    aid: u64,
    content: &str,
) -> AppResult<u64> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        // detail is an i18n key; IntoResponse will translate it to the correct language
        return Err(AppError::param_invalid("comment_empty"));
    }
    if trimmed.chars().count() > 2000 {
        return Err(AppError::param_invalid("comment_too_long"));
    }
    let db = state.db.pool();
    let row: Option<(i64, i32)> = sqlx::query_as(
        "SELECT CAST(COALESCE(qid,0) AS SIGNED), CAST(COALESCE(status,1) AS SIGNED) \
         FROM phpyun_answer WHERE id = ?",
    )
    .bind(aid as i64)
    .fetch_optional(db)
    .await?;
    let Some((qid, status)) = row else {
        return Err(AppError::new(InfraError::InvalidParam(
            "answer_not_found".into(),
        )));
    };
    if status != 1 {
        return Err(AppError::new(InfraError::InvalidParam(
            "answer_unavailable".into(),
        )));
    }
    let now = clock::now_ts();
    let usertype: i32 = if user.usertype > 0 { user.usertype as i32 } else { 1 };
    let id = qna_repo::create_review(
        db,
        qna_repo::ReviewCreate {
            aid,
            qid: qid.max(0) as u64,
            uid: user.uid,
            usertype,
            content: trimmed,
            status: 1,
        },
        now,
    )
    .await?;
    Ok(id)
}

pub async fn delete_review(
    state: &AppState,
    user: &AuthenticatedUser,
    review_id: u64,
) -> AppResult<()> {
    let db = state.db.pool();
    let n = qna_repo::delete_review(db, review_id, user.uid).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "review_not_found".into(),
        )));
    }
    Ok(())
}

// ==================== Categories (phpyun_q_class) ====================

/// In-process cache with 60s TTL: Q&A categories rarely change yet are requested by every home and list page.
/// moka's `try_get_with` provides built-in singleflight, so the cache-stampede scenario is safe.
static QCLASSES_CACHE: std::sync::OnceLock<
    moka::future::Cache<(), std::sync::Arc<Vec<QClass>>>,
> = std::sync::OnceLock::new();

fn qclasses_cache() -> &'static moka::future::Cache<(), std::sync::Arc<Vec<QClass>>> {
    QCLASSES_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(1)
            .time_to_live(std::time::Duration::from_secs(60))
            .build()
    })
}

pub async fn list_categories(state: &AppState) -> AppResult<std::sync::Arc<Vec<QClass>>> {
    let cache = qclasses_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with((), async move {
            let list = qna_repo::list_qclasses(&db).await?;
            Ok::<_, AppError>(std::sync::Arc::new(list))
        })
        .await
        .map_err(AppError::from_arc)
}

// ==================== Weekly hot questions ====================

pub async fn list_hotweek(state: &AppState, limit: u64) -> AppResult<Vec<Question>> {
    let db = state.db.reader();
    let since = clock::now_ts() - 7 * 86_400;
    Ok(qna_repo::hotweek_questions(db, since, limit.clamp(1, 50)).await?)
}
