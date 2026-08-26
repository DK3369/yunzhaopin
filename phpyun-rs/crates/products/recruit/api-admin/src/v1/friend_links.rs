//! Friend links. PHP `link_name` / `link_url` / `link_type` / `link_state`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::friend_link::entity::FriendLink;
use phpyun_services::admin_cms_service::{self, FriendLinkUpsertIn};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/friend-links", post(upsert))
        .route("/friend-links/list", post(list))
        .route("/friend-links/delete", post(delete))
}

#[utoipa::path(post, path = "/v1/admin/friend-links/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<FriendLink>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_friend_links(&state, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FriendLinkForm {
    pub id: Option<u64>,
    #[serde(alias = "name")]
    #[validate(length(min = 1, max = 200))]
    pub link_name: String,
    #[serde(alias = "url")]
    #[validate(length(min = 1, max = 500))]
    pub link_url: String,
    #[serde(default, alias = "logo")]
    pub pic: String,
    #[serde(default, alias = "category")]
    pub link_type: String,
    #[serde(default, alias = "sort")]
    pub link_sorting: i32,
    #[serde(default, alias = "status")]
    pub link_state: i32,
}

#[utoipa::path(post, path = "/v1/admin/friend-links", tag = "admin", security(("bearer" = [])), request_body = FriendLinkForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FriendLinkForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_friend_link(
        &state,
        &user,
        FriendLinkUpsertIn {
            id: f.id,
            link_name: &f.link_name,
            link_url: &f.link_url,
            pic: &f.pic,
            link_type: &f.link_type,
            link_sorting: f.link_sorting,
            link_state: f.link_state,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/friend-links/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_friend_link(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}
