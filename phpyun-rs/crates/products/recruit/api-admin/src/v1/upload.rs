//! PHP `index/uploadfile` + `layui_upload` + `common_upload` (admin multipart via BFF).

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::routing::post;
use axum::Router;
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser};
use serde::Serialize;

const MAX_BYTES: usize = 10 * 1024 * 1024;
const IMG_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp", "image/gif"];

pub fn routes() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}

#[derive(Debug, Serialize)]
pub struct UploadResult {
    pub url: String,
    pub key: String,
    pub bytes: usize,
}

fn ct_of(headers: &HeaderMap) -> &str {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
}

/// PHP admin image upload. BFF strips multipart and posts raw bytes.
pub async fn upload(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<UploadResult>> {
    user.require_admin()?;
    if body.is_empty() {
        return Err(ApiError::param_invalid("empty body"));
    }
    if body.len() > MAX_BYTES {
        return Err(ApiError::param_invalid("file too large"));
    }
    let ct = ct_of(&headers);
    if !IMG_TYPES.iter().any(|t| ct.starts_with(t)) && !ct.starts_with("application/octet-stream") {
        return Err(ApiError::param_invalid(format!("unsupported content-type: {ct}")));
    }
    let ext = match ct {
        c if c.starts_with("image/jpeg") => "jpg",
        c if c.starts_with("image/png") => "png",
        c if c.starts_with("image/webp") => "webp",
        c if c.starts_with("image/gif") => "gif",
        _ => "bin",
    };
    let key = format!("admin/{}/{}.{}", user.uid, uuid::Uuid::now_v7(), ext);
    let bytes_len = body.len();
    let url = state.storage.put(&key, ct, body).await?;
    Ok(ApiResponse::data(UploadResult {
        url,
        key,
        bytes: bytes_len,
    }))
}
