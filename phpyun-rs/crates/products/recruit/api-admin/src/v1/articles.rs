//! News / articles admin CRUD. PHP `neirong/news` fields: title, nid, content.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::article::entity::{Article, NewsGroup};
use phpyun_services::admin_cms_service::{self, ArticleUpsertIn};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/articles", post(upsert))
        .route("/articles/list", post(list))
        .route("/articles/delete", post(delete))
        .route("/articles/groups", post(groups))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    pub nid: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/articles/list", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<Article>>> {
    user.require_admin()?;
    let r = admin_cms_service::list_articles(&state, q.nid, q.keyword.as_deref(), page).await?;
    Ok(ApiResponse::data(r))
}

#[utoipa::path(post, path = "/v1/admin/articles/groups", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn groups(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<NewsGroup>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_cms_service::list_article_groups(&state).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ArticleForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(range(min = 1))]
    pub nid: i32,
    #[validate(length(min = 1, max = 500_000))]
    pub content: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub keyword: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub newsphoto: String,
    #[serde(default)]
    pub did: i32,
}

#[utoipa::path(post, path = "/v1/admin/articles", tag = "admin", security(("bearer" = [])), request_body = ArticleForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ArticleForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_article(
        &state,
        &user,
        ArticleUpsertIn {
            id: f.id,
            title: &f.title,
            nid: f.nid,
            content: &f.content,
            author: &f.author,
            description: &f.description,
            keyword: &f.keyword,
            source: &f.source,
            newsphoto: &f.newsphoto,
            did: f.did,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/articles/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_article(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}
