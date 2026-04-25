//! Q&A: authenticated interactions (ask / answer / follow / upvote / accept / delete / mine).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::qna_service::{self, CreateQuestionInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/questions", post(ask))
        .route("/questions/{id}", post(remove))
        .route("/questions/{id}/answers", post(answer))
        .route("/questions/{id}/answers/{aid}/accept", post(accept))
        .route("/questions/{id}/attention", post(toggle_attention))
        .route("/questions/{id}/support", post(support_question))
        .route("/answers/{id}/support", post(support_answer))
        .route("/answers/{aid}/comments", post(post_comment))
        .route("/comments/{id}", post(remove_comment))
        .route("/my/questions", get(my_questions))
        .route("/my/answers", get(my_answers))
        .route("/my/attended-questions", get(attended))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AskForm {
    #[validate(length(min = 2, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 20000))]
    pub content: String,
    #[serde(default)]
    pub category_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// Ask a question
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AskForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn ask(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AskForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = qna_service::create_question(
        &state,
        &user,
        CreateQuestionInput {
            title: &f.title,
            content: &f.content,
            category_id: f.category_id,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Delete my question
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    qna_service::delete_question(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AnswerForm {
    #[validate(length(min = 1, max = 20000))]
    pub content: String,
}

/// Answer
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/{id}/answers",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = AnswerForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn answer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<AnswerForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let aid = qna_service::answer(&state, &user, id, &f.content).await?;
    Ok(ApiJson(CreatedId { id: aid }))
}

/// Accept an answer (only the questioner can)
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/{id}/answers/{aid}/accept",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path), ("aid" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn accept(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((id, aid)): Path<(u64, u64)>,
) -> AppResult<ApiOk> {
    qna_service::accept_answer(&state, &user, id, aid).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Toggled {
    pub on: bool,
}

/// Follow / unfollow
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/{id}/attention",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn toggle_attention(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_attention(&state, &user, id).await?;
    Ok(ApiJson(Toggled { on }))
}

/// Upvote a question
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/{id}/support",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn support_question(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_support_question(&state, &user, id).await?;
    Ok(ApiJson(Toggled { on }))
}

/// Upvote an answer
#[utoipa::path(
    post,
    path = "/v1/mcenter/answers/{id}/support",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn support_answer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_support_answer(&state, &user, id).await?;
    Ok(ApiJson(Toggled { on }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyQuestion {
    pub id: u64,
    pub title: String,
    pub hits: u32,
    pub answer_count: u32,
    pub support_count: u32,
    pub created_at: i64,
}
impl From<phpyun_models::qna::entity::Question> for MyQuestion {
    fn from(q: phpyun_models::qna::entity::Question) -> Self {
        Self {
            id: q.id,
            title: q.title,
            hits: q.hits,
            answer_count: q.answer_count,
            support_count: q.support_count,
            created_at: q.created_at,
        }
    }
}

/// Questions I asked
#[utoipa::path(
    get,
    path = "/v1/mcenter/my/questions",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_questions(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyQuestion>>> {
    let r = qna_service::list_my_questions(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyQuestion::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyAnswer {
    pub id: u64,
    pub question_id: u64,
    pub content: String,
    pub support_count: u32,
    pub is_accepted: i32,
    pub created_at: i64,
}
impl From<phpyun_models::qna::entity::Answer> for MyAnswer {
    fn from(a: phpyun_models::qna::entity::Answer) -> Self {
        Self {
            id: a.id,
            question_id: a.question_id,
            content: a.content,
            support_count: a.support_count,
            is_accepted: a.is_accepted,
            created_at: a.created_at,
        }
    }
}

/// My answers
#[utoipa::path(
    get,
    path = "/v1/mcenter/my/answers",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_answers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyAnswer>>> {
    let r = qna_service::list_my_answers(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyAnswer::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Questions I follow
#[utoipa::path(
    get,
    path = "/v1/mcenter/my/attended-questions",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn attended(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyQuestion>>> {
    let r = qna_service::list_attended(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyQuestion::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CommentForm {
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
}

/// Comment on an answer (aligned with PHP `wap/ask::forcomment_action`)
#[utoipa::path(
    post,
    path = "/v1/mcenter/answers/{aid}/comments",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("aid" = u64, Path)),
    request_body = CommentForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn post_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(aid): Path<u64>,
    ValidatedJson(f): ValidatedJson<CommentForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = qna_service::add_review(&state, &user, aid, &f.content).await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Delete my own comment
#[utoipa::path(
    post,
    path = "/v1/mcenter/comments/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn remove_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    qna_service::delete_review(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}
