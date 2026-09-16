//! PHP `index/uploadfile` + `layui_upload` + `common_upload` (admin multipart via BFF).

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::routing::post;
use axum::Router;
use phpyun_core::utils::sniff_image;
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser};
use serde::Serialize;

const MAX_BYTES: usize = 10 * 1024 * 1024;

pub fn routes() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}

#[derive(Debug, Serialize)]
pub struct UploadResult {
    pub url: String,
    pub key: String,
    pub bytes: usize,
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
    let Some((ct, ext)) = sniff_image(&body) else {
        let declared = headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        return Err(ApiError::param_invalid(format!(
            "unsupported image (declared {declared})"
        )));
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
