//! Admin recycle bin. List snapshots / view detail / permanent delete. Restoration is handled by each business service.
//!
//! The `php-*` routes serve the console's 数据回收站 page, which reads and
//! writes PHP's own `phpyun_recycle` shape (`serialize()`d bodies, md5 `ident`
//! per operation); see `admin_recycle_service`. The plain routes above them are
//! the JSON-bodied generic view and stay as they are.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{IdBody, IdsBody};
use phpyun_core::{
    ApiMessage, ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
    ValidatedJson,
};
use phpyun_services::admin_recycle_service;
use phpyun_services::recycle_bin_service::{self, RecycleView};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recycle-bin", post(list))
        .route("/recycle-bin/detail", post(detail))
        .route("/recycle-bin/purge", post(purge))
        .route("/recycle-bin/php-index", post(php_index))
        .route("/recycle-bin/php-recover", post(php_recover))
        .route("/recycle-bin/php-recover-all", post(php_recover_all))
        .route("/recycle-bin/php-del", post(php_del))
        .route("/recycle-bin/php-truncate", post(php_truncate))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 100))]
    pub tablename: Option<String>,
}

/// Recycle bin list
#[utoipa::path(
    post,
    path = "/v1/admin/recycle-bin",
    tag = "admin",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<RecycleView>>> {
    user.require_admin()?;
    let r = recycle_bin_service::list(&state, q.tablename.as_deref(), page).await?;
    Ok(ApiResponse::data(r))
}

/// Single record detail
#[utoipa::path(post,
    path = "/v1/admin/recycle-bin/detail",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<RecycleView>> {
    let id = b.id;
    user.require_admin()?;
    Ok(ApiResponse::data(
        recycle_bin_service::get(&state, id).await?,
    ))
}

/// Permanently delete
#[utoipa::path(post,
    path = "/v1/admin/recycle-bin/purge",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn purge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    let id = b.id;
    user.require_admin()?;
    recycle_bin_service::purge(&state, &user, id).await?;
    Ok(ApiResponse::message("purged"))
}

/// PHP `dataRecycle::index_action` filters. `keyword` searches the serialized
/// body, `table` the source table, `ident` narrows to one operation.
#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct PhpListQuery {
    #[validate(length(max = 60))]
    pub username: Option<String>,
    #[validate(length(max = 120))]
    pub keyword: Option<String>,
    #[validate(length(max = 60))]
    pub table: Option<String>,
    #[validate(length(max = 40))]
    pub ident: Option<String>,
    /// The date picker's `[start, end]` in milliseconds; it sends `''` when cleared.
    #[serde(default, deserialize_with = "de_ms_range")]
    #[schema(value_type = Option<Vec<i64>>)]
    pub time: Option<(i64, i64)>,
}

fn de_ms_range<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<(i64, i64)>, D::Error> {
    let v = serde_json::Value::deserialize(d)?;
    let Some(a) = v.as_array() else {
        return Ok(None);
    };
    let ms = |i: usize| a.get(i).and_then(serde_json::Value::as_i64).unwrap_or(0);
    let (from, to) = (ms(0), ms(1));
    Ok((from > 0 && to > 0).then_some((from, to)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdentBody {
    #[validate(length(min = 1, max = 40))]
    pub ident: String,
}

/// PHP `tuncateRecycle_action` guards on this literal (their spelling).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TruncateBody {
    #[serde(default)]
    pub recycle: String,
}

#[utoipa::path(post, path = "/v1/admin/recycle-bin/php-index", tag = "admin", security(("bearer" = [])), request_body = PhpListQuery, responses((status = 200, description = "ok")))]
pub async fn php_index(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<PhpListQuery>,
) -> AppResult<ApiResponse<AdminPaged<serde_json::Value>>> {
    let q = admin_recycle_service::ListQuery {
        username: q.username,
        keyword: q.keyword,
        table: q.table,
        ident: q.ident,
        time: q.time,
    };
    Ok(ApiResponse::data(AdminPaged::from(
        admin_recycle_service::list(&state, &user, &q, page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/recycle-bin/php-recover", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn php_recover(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiMessage> {
    let msg = admin_recycle_service::recover(&state, &user, &b.ids).await?;
    Ok(ApiMessage::new("common_06572", msg))
}

#[utoipa::path(post, path = "/v1/admin/recycle-bin/php-recover-all", tag = "admin", security(("bearer" = [])), request_body = IdentBody, responses((status = 200, description = "ok")))]
pub async fn php_recover_all(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdentBody>,
) -> AppResult<ApiMessage> {
    let msg = admin_recycle_service::recover_by_ident(&state, &user, &b.ident).await?;
    Ok(ApiMessage::new("common_06572", msg))
}

#[utoipa::path(post, path = "/v1/admin/recycle-bin/php-del", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn php_del(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiMessage> {
    let msg = admin_recycle_service::delete(&state, &user, &b.ids).await?;
    Ok(ApiMessage::new("admin_user_00187", msg))
}

#[utoipa::path(post, path = "/v1/admin/recycle-bin/php-truncate", tag = "admin", security(("bearer" = [])), request_body = TruncateBody, responses((status = 200, description = "ok")))]
pub async fn php_truncate(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<TruncateBody>,
) -> AppResult<ApiMessage> {
    let msg = admin_recycle_service::truncate(&state, &user, &b.recycle).await?;
    Ok(ApiMessage::new("admin_01458", msg))
}
