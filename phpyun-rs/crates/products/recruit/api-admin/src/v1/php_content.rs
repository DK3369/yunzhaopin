//! PHP-shaped named admin actions. Intentionally omitted from AdminDoc.

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_services::admin_php_content_service::{self, PhpOut};
use serde_json::Value;

pub fn routes() -> Router<AppState> {
    Router::new().route("/php-content/{module}/{action}", post(php_content))
}

pub async fn php_content(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((module, action)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> AppResult<Response> {
    user.require_admin()?;
    if !is_slug(&module) || !is_slug(&action) {
        return Err(ApiError::param_invalid("unknown_php_action"));
    }
    match admin_php_content_service::dispatch(&state, &user, &module, &action, &body).await? {
        PhpOut::Data(v) => Ok(ApiResponse::data(v).into_response()),
        PhpOut::Message(k) => Ok(ApiResponse::message(k).into_response()),
    }
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 40
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
