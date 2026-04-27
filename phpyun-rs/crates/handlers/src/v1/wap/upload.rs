//! Image / attachment upload (mirrors PHPYun `wap/upload` + `company::upComYyzz` etc.)
//!
//! All endpoints:
//! - Require login (`AuthenticatedUser`)
//! - Content-Type must be whitelisted
//! - Per-file size limit (validated again in the business layer, does not rely on global `RequestBodyLimit`)
//! - Support the `Idempotency-Key` header (only effective when the router installs `idempotency::layer`)
//! - Persist via the `state.storage` facade

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::routing::{get, post};
use axum::Router;
use phpyun_core::{ApiJson, AppError, AppResult, AppState, AuthenticatedUser};
use serde::Serialize;
use utoipa::ToSchema;

// ---------- Size limits (bytes) ----------
const MAX_AVATAR_BYTES: usize = 1024 * 1024; // 1MB
const MAX_LOGO_BYTES: usize = 2 * 1024 * 1024;
const MAX_PHOTO_BYTES: usize = 2 * 1024 * 1024;
const MAX_CERT_BYTES: usize = 5 * 1024 * 1024;
const MAX_ATTACH_BYTES: usize = 10 * 1024 * 1024;

const IMG_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];
const DOC_TYPES: &[&str] = &[
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/upload/avatar", post(upload_avatar))
        .route("/upload/company-logo", post(upload_company_logo))
        .route("/upload/resume-photo", post(upload_resume_photo))
        .route("/upload/cert", post(upload_cert))
        .route("/upload/attachment", post(upload_attachment))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResult {
    /// Public URL (CDN / S3)
    pub url: String,
    /// Storage key (saved into the corresponding DB column)
    pub key: String,
    pub bytes: usize,
}

fn ct_of(headers: &HeaderMap) -> &str {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
}

fn check_type(ct: &str, allowed: &[&str]) -> AppResult<()> {
    if allowed.iter().any(|t| ct.starts_with(t)) {
        Ok(())
    } else {
        Err(AppError::param_invalid(format!("unsupported content-type: {ct}")))
    }
}

fn check_size(body: &Bytes, max: usize) -> AppResult<()> {
    if body.is_empty() {
        return Err(AppError::param_invalid("empty body"));
    }
    if body.len() > max {
        return Err(AppError::param_invalid(format!(
            "file too large: {} bytes > {}",
            body.len(),
            max
        )));
    }
    Ok(())
}

fn ext_of(ct: &str) -> &'static str {
    match ct {
        c if c.starts_with("image/jpeg") => "jpg",
        c if c.starts_with("image/png") => "png",
        c if c.starts_with("image/webp") => "webp",
        c if c.starts_with("application/pdf") => "pdf",
        c if c.starts_with("application/msword") => "doc",
        c if c.starts_with("application/vnd.openxmlformats-officedocument.wordprocessingml.document") => "docx",
        _ => "bin",
    }
}

async fn store(
    state: &AppState,
    uid: u64,
    dir: &str,
    ct: &str,
    body: Bytes,
) -> AppResult<UploadResult> {
    let ext = ext_of(ct);
    let key = format!("{dir}/{}/{}.{}", uid, uuid::Uuid::now_v7(), ext);
    let bytes_len = body.len();
    let url = state.storage.put(&key, ct, body).await?;
    Ok(UploadResult { url, key, bytes: bytes_len })
}

/// Upload avatar (1MB image)
#[utoipa::path(
    post,
    path = "/v1/wap/upload/avatar",
    tag = "upload",
    security(("bearer" = [])),
    request_body(content = Vec<u8>, content_type = "image/jpeg"),
    responses((status = 200, description = "ok", body = UploadResult))
)]
pub async fn upload_avatar(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiJson<UploadResult>> {
    let ct = ct_of(&headers);
    check_type(ct, IMG_TYPES)?;
    check_size(&body, MAX_AVATAR_BYTES)?;
    Ok(ApiJson(store(&state, user.uid, "avatars", ct, body).await?))
}

/// Upload company logo (2MB image, employer only)
#[utoipa::path(
    post,
    path = "/v1/wap/upload/company-logo",
    tag = "upload",
    security(("bearer" = [])),
    request_body(content = Vec<u8>, content_type = "image/jpeg"),
    responses((status = 200, description = "ok", body = UploadResult))
)]
pub async fn upload_company_logo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiJson<UploadResult>> {
    user.require_employer()?;
    let ct = ct_of(&headers);
    check_type(ct, IMG_TYPES)?;
    check_size(&body, MAX_LOGO_BYTES)?;
    Ok(ApiJson(store(&state, user.uid, "logos", ct, body).await?))
}

/// Upload resume photo (2MB image, jobseeker only)
#[utoipa::path(
    post,
    path = "/v1/wap/upload/resume-photo",
    tag = "upload",
    security(("bearer" = [])),
    request_body(content = Vec<u8>, content_type = "image/jpeg"),
    responses((status = 200, description = "ok", body = UploadResult))
)]
pub async fn upload_resume_photo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiJson<UploadResult>> {
    user.require_jobseeker()?;
    let ct = ct_of(&headers);
    check_type(ct, IMG_TYPES)?;
    check_size(&body, MAX_PHOTO_BYTES)?;
    Ok(ApiJson(store(&state, user.uid, "resume_photos", ct, body).await?))
}

/// Upload verification image (5MB) -- company business license / legal representative ID
#[utoipa::path(
    post,
    path = "/v1/wap/upload/cert",
    tag = "upload",
    security(("bearer" = [])),
    request_body(content = Vec<u8>, content_type = "image/jpeg"),
    responses((status = 200, description = "ok", body = UploadResult))
)]
pub async fn upload_cert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiJson<UploadResult>> {
    let ct = ct_of(&headers);
    check_type(ct, IMG_TYPES)?;
    check_size(&body, MAX_CERT_BYTES)?;
    Ok(ApiJson(store(&state, user.uid, "certs", ct, body).await?))
}

/// Upload attachment (10MB, pdf/doc/docx; for resume attachments / portfolio)
#[utoipa::path(
    post,
    path = "/v1/wap/upload/attachment",
    tag = "upload",
    security(("bearer" = [])),
    request_body(content = Vec<u8>, content_type = "application/pdf"),
    responses((status = 200, description = "ok", body = UploadResult))
)]
pub async fn upload_attachment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiJson<UploadResult>> {
    let ct = ct_of(&headers);
    check_type(ct, DOC_TYPES)?;
    check_size(&body, MAX_ATTACH_BYTES)?;
    Ok(ApiJson(store(&state, user.uid, "attachments", ct, body).await?))
}
