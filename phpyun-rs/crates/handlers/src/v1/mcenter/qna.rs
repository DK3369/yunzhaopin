//! Q&A: authenticated interactions (ask / answer / follow / upvote / accept / delete / mine).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::qna_service::{self, CreateQuestionInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/questions", post(ask))
        .route("/questions/delete", post(remove))
        .route("/questions/answers", post(answer))
        .route("/questions/answers/accept", post(accept))
        .route("/questions/attention", post(toggle_attention))
        .route("/questions/support", post(support_question))
        .route("/answers/support", post(support_answer))
        .route("/answers/comments", post(post_comment))
        .route("/comments/delete", post(remove_comment))
        .route("/my/questions", post(my_questions))
        .route("/my/answers", post(my_answers))
        .route("/my/attended-questions", post(attended))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AnswerBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 20000))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AcceptBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub aid: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CommentBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub aid: u64,
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AskForm {
    #[validate(length(min = 2, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 20000))]
    pub content: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub category_id: i32,
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
    path = "/v1/mcenter/questions/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiOk> {
    qna_service::delete_question(&state, &user, b.id).await?;
    Ok(ApiOk("deleted"))
}

/// Answer
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/answers",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AnswerBody,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn answer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<AnswerBody>,
) -> AppResult<ApiJson<CreatedId>> {
    let aid = qna_service::answer(&state, &user, b.id, &b.content).await?;
    Ok(ApiJson(CreatedId { id: aid }))
}

/// Accept an answer (only the questioner can)
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/answers/accept",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AcceptBody,
    responses((status = 200, description = "ok"))
)]
pub async fn accept(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<AcceptBody>,
) -> AppResult<ApiOk> {
    qna_service::accept_answer(&state, &user, b.id, b.aid).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Toggled {
    pub on: bool,
}

/// Follow / unfollow
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/attention",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn toggle_attention(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_attention(&state, &user, b.id).await?;
    Ok(ApiJson(Toggled { on }))
}

/// Upvote a question
#[utoipa::path(
    post,
    path = "/v1/mcenter/questions/support",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn support_question(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_support_question(&state, &user, b.id).await?;
    Ok(ApiJson(Toggled { on }))
}

/// Upvote an answer
#[utoipa::path(
    post,
    path = "/v1/mcenter/answers/support",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok", body = Toggled))
)]
pub async fn support_answer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<Toggled>> {
    let on = qna_service::toggle_support_answer(&state, &user, b.id).await?;
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
    post,
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
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
    post,
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Questions I follow
#[utoipa::path(
    post,
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Comment on an answer (aligned with PHP `wap/ask::forcomment_action`)
#[utoipa::path(
    post,
    path = "/v1/mcenter/answers/comments",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CommentBody,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn post_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<CommentBody>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = qna_service::add_review(&state, &user, b.aid, &b.content).await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Delete my own comment
#[utoipa::path(
    post,
    path = "/v1/mcenter/comments/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove_comment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiOk> {
    qna_service::delete_review(&state, &user, b.id).await?;
    Ok(ApiOk("deleted"))
}
