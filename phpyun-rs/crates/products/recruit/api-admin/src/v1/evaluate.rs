//! PHP `neirong/evaluate`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::eval::admin::{AdminEvalLog, AdminEvalMessage, AdminEvalPaper};
use phpyun_models::eval::php_ser;
use phpyun_services::admin_eval_service::{self, AskUpsert, EvalGroupView, EvalPaperView, PaperUpsert};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/evaluate/papers", post(upsert_paper))
        .route("/evaluate/papers/list", post(list_papers))
        .route("/evaluate/papers/delete", post(delete_papers))
        .route("/evaluate/papers/detail", post(paper_detail))
        .route("/evaluate/groups", post(add_group))
        .route("/evaluate/groups/list", post(list_groups))
        .route("/evaluate/groups/delete", post(delete_group))
        .route("/evaluate/groups/patch", post(patch_group))
        .route("/evaluate/questions", post(save_questions))
        .route("/evaluate/questions/delete", post(delete_question))
        .route("/evaluate/messages", post(list_messages))
        .route("/evaluate/messages/delete", post(delete_messages))
        .route("/evaluate/logs", post(list_logs))
        .route("/evaluate/logs/delete", post(delete_logs))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct PaperListQuery {
    pub keyid: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/papers/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_papers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<PaperListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminEvalPaper>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_eval_service::list_papers(&state, q.keyid, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaperDetailForm {
    #[validate(range(min = 1))]
    pub id: u64,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/papers/detail", tag = "admin", security(("bearer" = [])), request_body = PaperDetailForm, responses((status = 200, description = "ok")))]
pub async fn paper_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PaperDetailForm>,
) -> AppResult<ApiResponse<EvalPaperView>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_eval_service::paper_detail(&state, f.id).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PjItem {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AskItem {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 2000))]
    pub question: String,
    #[serde(default)]
    pub option: serde_json::Value,
    #[serde(default)]
    pub score: serde_json::Value,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaperForm {
    /// PHP `add=1` loads the editor payload instead of saving.
    #[serde(default)]
    pub add: i32,
    pub id: Option<u64>,
    #[serde(default)]
    #[validate(length(max = 120))]
    pub name: String,
    #[serde(default)]
    pub keyid: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub top: i32,
    #[serde(default)]
    pub hot: i32,
    #[serde(default)]
    pub recommend: i32,
    #[serde(default)]
    pub description: String,
    pub pic: Option<String>,
    #[serde(default)]
    pub pj_arr: Vec<PjItem>,
    #[serde(default)]
    pub ask_arr: Vec<AskItem>,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/papers", tag = "admin", security(("bearer" = [])), request_body = PaperForm, responses((status = 200, description = "ok")))]
pub async fn upsert_paper(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PaperForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    if f.add == 1 {
        return Ok(ApiResponse::data(
            admin_eval_service::paper_add_load(&state, f.id).await?,
        ));
    }
    let fromscore: Vec<String> = f.pj_arr.iter().map(|p| p.from.clone()).collect();
    let toscore: Vec<String> = f.pj_arr.iter().map(|p| p.to.clone()).collect();
    let comment: Vec<String> = f.pj_arr.iter().map(|p| p.content.clone()).collect();
    let asks: Vec<AskUpsert> = f
        .ask_arr
        .iter()
        .map(|a| AskUpsert {
            id: a.id,
            question: a.question.clone(),
            option: php_ser::json_to_strings(&a.option),
            score: php_ser::json_to_strings(&a.score),
        })
        .collect();
    let id = admin_eval_service::upsert_paper(
        &state,
        &user,
        PaperUpsert {
            id: f.id,
            name: &f.name,
            keyid: f.keyid,
            sort: f.sort,
            top: f.top,
            hot: f.hot,
            recommend: f.recommend,
            description: &f.description,
            pic: f.pic.as_deref(),
            fromscore,
            toscore,
            comment,
            asks,
        },
    )
    .await?;
    Ok(ApiResponse::data(serde_json::json!({ "id": id, "nid": id })))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/papers/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_papers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_papers(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/groups/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_groups(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<EvalGroupView>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_eval_service::list_groups(&state).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GroupAddForm {
    #[validate(length(min = 1, max = 80))]
    pub classname: String,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/groups", tag = "admin", security(("bearer" = [])), request_body = GroupAddForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn add_group(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<GroupAddForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_eval_service::add_group(&state, &user, &f.classname).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GroupPatchForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub name: Option<String>,
    pub sort: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/groups/patch", tag = "admin", security(("bearer" = [])), request_body = GroupPatchForm, responses((status = 200, description = "ok")))]
pub async fn patch_group(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<GroupPatchForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::patch_group(&state, &user, f.id, f.name.as_deref(), f.sort).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/groups/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete_group(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_group(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QuestionsForm {
    #[validate(range(min = 1))]
    pub examid: u64,
    #[serde(default)]
    pub ask: Vec<AskItem>,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/questions", tag = "admin", security(("bearer" = [])), request_body = QuestionsForm, responses((status = 200, description = "ok")))]
pub async fn save_questions(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<QuestionsForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let asks: Vec<AskUpsert> = f
        .ask
        .iter()
        .map(|a| AskUpsert {
            id: a.id,
            question: a.question.clone(),
            option: php_ser::json_to_strings(&a.option),
            score: php_ser::json_to_strings(&a.score),
        })
        .collect();
    admin_eval_service::save_questions(&state, &user, f.examid, &asks).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QidForm {
    #[validate(range(min = 1))]
    pub qid: u64,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/questions/delete", tag = "admin", security(("bearer" = [])), request_body = QidForm, responses((status = 200, description = "ok")))]
pub async fn delete_question(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<QidForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_question(&state, &user, f.qid).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct MsgListQuery {
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    pub r#type: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/evaluate/messages", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_messages(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<MsgListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminEvalMessage>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_eval_service::list_messages(&state, q.keyword.as_deref(), q.r#type == Some(1), page)
            .await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/messages/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_messages(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_messages(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<MsgListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminEvalLog>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_eval_service::list_logs(&state, q.keyword.as_deref(), q.r#type == Some(2), page)
            .await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/evaluate/logs/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_logs(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}
