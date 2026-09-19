//! Admin payment gateway: merchants, methods, orders.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::pay_service::{self, MerchantCreated, MerchantSaveIn, MethodSaveIn, MethodView, MerchantView, OrderView};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/pay/orders/list", post(orders_list))
        .route("/pay/methods/list", post(methods_list))
        .route("/pay/methods/save", post(methods_save))
        .route("/pay/methods/status", post(methods_status))
        .route("/pay/methods/delete", post(methods_delete))
        .route("/pay/merchants/list", post(merchants_list))
        .route("/pay/merchants/save", post(merchants_save))
        .route("/pay/merchants/status", post(merchants_status))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OrderListForm {
    #[serde(default)]
    #[validate(length(max = 64))]
    pub merchant_code: String,
    #[serde(default)]
    #[validate(length(max = 64))]
    pub method_code: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub status: String,
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/orders/list",
    tag = "admin",
    security(("bearer" = [])),
    request_body = OrderListForm,
    responses((status = 200, description = "ok"))
)]
pub async fn orders_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(f): ValidatedJson<OrderListForm>,
) -> AppResult<ApiResponse<Paged<OrderView>>> {
    user.require_admin()?;
    let merchant = f.merchant_code.trim();
    let method = f.method_code.trim();
    let status = f.status.trim();
    let (list, total) = pay_service::admin_list_orders(
        &state,
        if merchant.is_empty() { None } else { Some(merchant) },
        if method.is_empty() { None } else { Some(method) },
        if status.is_empty() { None } else { Some(status) },
        page.offset,
        page.limit,
    )
    .await?;
    Ok(ApiResponse::data(Paged::from_listing(list, total, page)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MethodListForm {
    #[serde(default)]
    pub merchant_id: u64,
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/methods/list",
    tag = "admin",
    security(("bearer" = [])),
    request_body = MethodListForm,
    responses((status = 200, description = "ok"))
)]
pub async fn methods_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<MethodListForm>,
) -> AppResult<ApiResponse<Vec<MethodView>>> {
    user.require_admin()?;
    let mid = if f.merchant_id == 0 {
        None
    } else {
        Some(f.merchant_id)
    };
    Ok(ApiResponse::data(
        pay_service::admin_list_methods(&state, mid).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MethodSaveForm {
    #[serde(default)]
    pub id: u64,
    pub merchant_id: u64,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub status: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    #[validate(length(max = 256))]
    pub secret_key: String,
    #[serde(default)]
    #[validate(length(max = 256))]
    pub webhook_secret: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub currency: String,
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/methods/save",
    tag = "admin",
    security(("bearer" = [])),
    request_body = MethodSaveForm,
    responses((status = 200, description = "ok"))
)]
pub async fn methods_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<MethodSaveForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    let id = pay_service::admin_save_method(
        &state,
        MethodSaveIn {
            id: f.id,
            merchant_id: f.merchant_id,
            code: f.code,
            name: f.name,
            status: f.status,
            sort: f.sort,
            secret_key: f.secret_key,
            webhook_secret: f.webhook_secret,
            currency: f.currency,
        },
    )
    .await?;
    Ok(ApiResponse::data(serde_json::json!({ "id": id })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatusForm {
    pub id: u64,
    #[validate(length(min = 1, max = 16))]
    pub status: String,
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/methods/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn methods_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<StatusForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    pay_service::admin_set_method_status(&state, f.id, &f.status).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/methods/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn methods_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    pay_service::admin_delete_method(&state, b.id).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/merchants/list",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn merchants_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<MerchantView>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        pay_service::admin_list_merchants(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MerchantSaveForm {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    #[validate(length(max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[serde(default)]
    #[validate(length(max = 512))]
    pub notify_url: String,
    #[serde(default)]
    #[validate(length(max = 512))]
    pub return_url: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub status: String,
    #[serde(default)]
    pub rotate_secret: bool,
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/merchants/save",
    tag = "admin",
    security(("bearer" = [])),
    request_body = MerchantSaveForm,
    responses((status = 200, description = "ok"))
)]
pub async fn merchants_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<MerchantSaveForm>,
) -> AppResult<ApiResponse<MerchantCreated>> {
    user.require_admin()?;
    let created = pay_service::admin_save_merchant(
        &state,
        &user,
        MerchantSaveIn {
            id: f.id,
            code: f.code,
            name: f.name,
            notify_url: f.notify_url,
            return_url: f.return_url,
            status: f.status,
            rotate_secret: f.rotate_secret,
        },
    )
    .await?;
    Ok(ApiResponse::data(created))
}

#[utoipa::path(
    post,
    path = "/v1/admin/pay/merchants/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn merchants_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<StatusForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    pay_service::admin_set_merchant_status(&state, f.id, &f.status).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}
