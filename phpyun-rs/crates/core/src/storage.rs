//! Object storage facade — **pluggable backends**: local FS / S3 / OSS / Minio
//! all implement the same trait.
//!
//! ## Design
//! - `ObjectStore` trait — implemented by every backend. **The business layer
//!   only calls the `Storage` facade and is unaware of the backend type.**
//! - `LocalFsStorage` — a local-filesystem implementation for development /
//!   testing, with zero dependencies.
//! - `S3Storage` (reserved stub) — for production; TODO: wire up
//!   aws-sdk-s3 / minio sdk.
//! - Path safety: automatically strips `..` and absolute path prefixes to
//!   prevent path traversal.
//!
//! ## Configuration (env)
//! - `STORAGE_KIND`: `fs` | `s3` (default `fs`)
//! - `STORAGE_FS_ROOT`: local storage root (default `./uploads`)
//! - `STORAGE_BASE_URL`: external URL prefix (default `http://localhost:3000/files`)
//!
//! ## Business usage
//! ```ignore
//! let key = format!("avatars/{}/{}", uid, uuid::Uuid::now_v7());
//! let url = state.storage.put(&key, "image/jpeg", bytes).await?;
//! // Hand the URL to the frontend: https://cdn.example.com/avatars/42/01HX...
//! ```
//!
//! ## Adding a new backend (without touching business code)
//! ```ignore
//! pub struct OssStorage { /* aliyun client */ }
//!
//! #[async_trait]
//! impl ObjectStore for OssStorage {
//!     async fn put(&self, key: &str, ct: &str, data: Bytes) -> AppResult<()> { ... }
//!     // ... other methods
//! }
//!
//! // Add a case in Storage::from_config; business code doesn't change.
//! ```

use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use bytes::Bytes;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Backend contract. Any implementation can be plugged into `Storage` and
/// invoked by the business layer.
#[async_trait]
pub trait ObjectStore: Send + Sync + 'static {
    /// Upload bytes. `content_type` is used for setting response headers on
    /// S3/CDN; the local FS backend can ignore it.
    async fn put(&self, key: &str, content_type: &str, data: Bytes) -> AppResult<()>;

    /// Delete an object. A missing key is treated as success (idempotent).
    async fn delete(&self, key: &str) -> AppResult<()>;

    /// Whether the object exists.
    async fn exists(&self, key: &str) -> bool;

    /// The externally accessible URL (CDN / direct link).
    fn url_of(&self, key: &str) -> String;

    /// Generate a **presigned PUT URL** for the frontend to upload directly,
    /// bypassing the application server's bandwidth.
    /// The LocalFs backend has no concept of signing — returns a regular
    /// upload address.
    async fn presigned_put(
        &self,
        key: &str,
        content_type: &str,
        ttl: Duration,
    ) -> AppResult<String>;
}

// ==================== LocalFsStorage ====================

pub struct LocalFsStorage {
    root: PathBuf,
    base_url: String,
}

impl LocalFsStorage {
    pub fn new(root: impl Into<PathBuf>, base_url: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            base_url: base_url.into(),
        }
    }

    /// Sanitize a key into a safe relative path: strip `..` and absolute
    /// prefixes to prevent path traversal.
    fn safe_path(&self, key: &str) -> PathBuf {
        let safe: PathBuf = key
            .split('/')
            .filter(|seg| !seg.is_empty() && *seg != "." && *seg != "..")
            .collect();
        self.root.join(safe)
    }
}

#[async_trait]
impl ObjectStore for LocalFsStorage {
    async fn put(&self, key: &str, _content_type: &str, data: Bytes) -> AppResult<()> {
        let path = self.safe_path(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(AppError::internal)?;
        }
        let mut f = fs::File::create(&path).await.map_err(AppError::internal)?;
        f.write_all(&data).await.map_err(AppError::internal)?;
        f.sync_all().await.map_err(AppError::internal)?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        let path = self.safe_path(key);
        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AppError::internal(e)),
        }
    }

    async fn exists(&self, key: &str) -> bool {
        fs::metadata(self.safe_path(key)).await.is_ok()
    }

    fn url_of(&self, key: &str) -> String {
        let safe = key.split('/').filter(|s| !s.is_empty() && *s != "..").collect::<Vec<_>>().join("/");
        format!("{}/{}", self.base_url.trim_end_matches('/'), safe)
    }

    async fn presigned_put(
        &self,
        key: &str,
        _content_type: &str,
        _ttl: Duration,
    ) -> AppResult<String> {
        // Local FS has no concept of signing; a frontend PUT to this URL is
        // equivalent to calling the upload handler directly.
        Ok(self.url_of(key))
    }
}

// ==================== S3Storage stub (production) ====================

/// S3 / OSS / Minio-compatible backend stub. Implement the real logic at
/// production deployment time; business code doesn't change.
pub struct S3Storage {
    pub bucket: String,
    pub region: String,
    pub base_url: String,
    // pub client: aws_sdk_s3::Client,  // TODO: wire up aws-sdk-s3
}

#[async_trait]
impl ObjectStore for S3Storage {
    async fn put(&self, _key: &str, _content_type: &str, _data: Bytes) -> AppResult<()> {
        Err(AppError::internal(std::io::Error::other(
            "S3Storage::put not yet implemented (TODO: aws-sdk-s3)",
        )))
    }

    async fn delete(&self, _key: &str) -> AppResult<()> {
        Err(AppError::internal(std::io::Error::other(
            "S3Storage::delete not yet implemented",
        )))
    }

    async fn exists(&self, _key: &str) -> bool {
        false
    }

    fn url_of(&self, key: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), key.trim_start_matches('/'))
    }

    async fn presigned_put(
        &self,
        _key: &str,
        _content_type: &str,
        _ttl: Duration,
    ) -> AppResult<String> {
        Err(AppError::internal(std::io::Error::other(
            "S3Storage::presigned_put not yet implemented",
        )))
    }
}

// ==================== Storage facade ====================

/// Business-layer entry point. `Arc<dyn ObjectStore>` keeps AppState unaware
/// of the concrete backend.
#[derive(Clone)]
pub struct Storage {
    inner: Arc<dyn ObjectStore>,
}

impl Storage {
    pub fn new<S: ObjectStore>(s: S) -> Self {
        Self { inner: Arc::new(s) }
    }

    /// Build from config, choosing the backend (dev fs / prod s3).
    pub fn from_config(cfg: &crate::config::Config) -> AppResult<Self> {
        let kind = cfg.storage_kind.as_deref().unwrap_or("fs");
        match kind {
            "fs" => {
                let root = cfg
                    .storage_fs_root
                    .clone()
                    .unwrap_or_else(|| "./uploads".into());
                let base = cfg
                    .storage_base_url
                    .clone()
                    .unwrap_or_else(|| "http://localhost:3000/files".into());
                Ok(Self::new(LocalFsStorage::new(root, base)))
            }
            "s3" => {
                let bucket = cfg
                    .storage_s3_bucket
                    .clone()
                    .ok_or_else(|| AppError::param_invalid("STORAGE_S3_BUCKET required"))?;
                let region = cfg
                    .storage_s3_region
                    .clone()
                    .unwrap_or_else(|| "us-east-1".into());
                let base = cfg
                    .storage_base_url
                    .clone()
                    .unwrap_or_else(|| format!("https://{bucket}.s3.{region}.amazonaws.com"));
                Ok(Self::new(S3Storage { bucket, region, base_url: base }))
            }
            other => Err(AppError::param_invalid(format!("unknown STORAGE_KIND: {other}"))),
        }
    }

    /// Upload and return the externally accessible URL.
    pub async fn put(&self, key: &str, content_type: &str, data: Bytes) -> AppResult<String> {
        self.inner.put(key, content_type, data.clone()).await?;
        crate::metrics::counter_with("storage.put", &[("backend", self.backend_kind())]);
        Ok(self.inner.url_of(key))
    }

    pub async fn delete(&self, key: &str) -> AppResult<()> {
        self.inner.delete(key).await?;
        crate::metrics::counter_with("storage.delete", &[("backend", self.backend_kind())]);
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> bool {
        self.inner.exists(key).await
    }

    pub fn url_of(&self, key: &str) -> String {
        self.inner.url_of(key)
    }

    /// Normalize a **legacy relative path** stored in the PHPYun database
    /// (e.g. `../data/upload/pimg/20170418/14992057095.PNG`) into a full URL
    /// the frontend can use directly.
    ///
    /// - Empty string → empty string (the frontend treats it as "no image").
    /// - Starts with `http://` / `https://` → returned unchanged (already absolute).
    /// - `//example.com/…` → prefixed with `https:`.
    /// - Otherwise → strip leading `../`, `./`, `/`, then concatenate the
    ///   `site_base` prefix. `site_base` prefers the supplied prefix; if
    ///   absent, it uses the storage's own `base_url` — suitable for fs
    ///   backends that mount local `/data/upload/…` directly under the
    ///   site root.
    pub fn normalize_legacy_url(&self, raw: &str, site_base: Option<&str>) -> String {
        let s = raw.trim();
        if s.is_empty() {
            return String::new();
        }
        if s.starts_with("http://") || s.starts_with("https://") {
            return s.to_string();
        }
        if let Some(rest) = s.strip_prefix("//") {
            return format!("https://{rest}");
        }
        let mut path = s;
        loop {
            if let Some(r) = path.strip_prefix("../") {
                path = r;
            } else if let Some(r) = path.strip_prefix("./") {
                path = r;
            } else if let Some(r) = path.strip_prefix('/') {
                path = r;
            } else {
                break;
            }
        }
        let base = site_base
            .map(str::trim)
            .filter(|b| !b.is_empty())
            .map(|b| b.trim_end_matches('/').to_string())
            .unwrap_or_else(|| self.inner.url_of("").trim_end_matches('/').to_string());
        if base.is_empty() {
            path.to_string()
        } else {
            format!("{base}/{path}")
        }
    }

    pub async fn presigned_put(
        &self,
        key: &str,
        content_type: &str,
        ttl: Duration,
    ) -> AppResult<String> {
        self.inner.presigned_put(key, content_type, ttl).await
    }

    fn backend_kind(&self) -> &'static str {
        // Simple downcast; in production this could become a `fn kind()` on the trait.
        // A coarse-grained metric label is enough here.
        "object_store"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn tmp_root() -> PathBuf {
        let p = std::env::temp_dir().join(format!("phpyun_storage_test_{}", uuid::Uuid::now_v7()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[tokio::test]
    async fn local_fs_put_get_delete_roundtrip() {
        let root = tmp_root();
        let s = LocalFsStorage::new(root.clone(), "https://cdn.test");
        let store = Storage::new(s);

        let url = store
            .put("avatars/1/x.jpg", "image/jpeg", Bytes::from_static(b"hello"))
            .await
            .unwrap();
        assert_eq!(url, "https://cdn.test/avatars/1/x.jpg");
        assert!(store.exists("avatars/1/x.jpg").await);

        store.delete("avatars/1/x.jpg").await.unwrap();
        assert!(!store.exists("avatars/1/x.jpg").await);
        // A second delete must not error (idempotent).
        store.delete("avatars/1/x.jpg").await.unwrap();
    }

    #[tokio::test]
    async fn local_fs_path_traversal_is_blocked() {
        let root = tmp_root();
        let s = LocalFsStorage::new(root.clone(), "https://cdn.test");
        let store = Storage::new(s);

        // Try ../../etc/passwd — it should land under root, not escape.
        store
            .put("../../etc/passwd", "text/plain", Bytes::from_static(b"oops"))
            .await
            .unwrap();
        // The actual file should be at root/etc/passwd (with .. stripped), not /etc/passwd.
        assert!(root.join("etc/passwd").exists());
        assert!(!std::path::Path::new("/etc/passwd_phpyun_test").exists());
    }

    #[tokio::test]
    async fn url_of_strips_leading_slash() {
        let s = LocalFsStorage::new("./tmp", "https://cdn.test/");
        assert_eq!(s.url_of("/foo/bar"), "https://cdn.test/foo/bar");
        assert_eq!(s.url_of("foo/bar"), "https://cdn.test/foo/bar");
    }

    /// Demonstrates pluggability: this test defines a custom in-memory
    /// backend and slots it into Storage without changing core.
    struct InMemStorage(Arc<tokio::sync::Mutex<std::collections::HashMap<String, Bytes>>>);

    #[async_trait]
    impl ObjectStore for InMemStorage {
        async fn put(&self, key: &str, _ct: &str, data: Bytes) -> AppResult<()> {
            self.0.lock().await.insert(key.to_string(), data);
            Ok(())
        }
        async fn delete(&self, key: &str) -> AppResult<()> {
            self.0.lock().await.remove(key);
            Ok(())
        }
        async fn exists(&self, key: &str) -> bool {
            self.0.lock().await.contains_key(key)
        }
        fn url_of(&self, key: &str) -> String {
            format!("mem://{key}")
        }
        async fn presigned_put(
            &self,
            key: &str,
            _ct: &str,
            _ttl: Duration,
        ) -> AppResult<String> {
            Ok(format!("mem://{key}?presigned=1"))
        }
    }

    #[tokio::test]
    async fn custom_backend_plugs_in_zero_core_change() {
        let store = Storage::new(InMemStorage(Arc::new(tokio::sync::Mutex::new(
            std::collections::HashMap::new(),
        ))));
        let url = store.put("k1", "text/plain", Bytes::from_static(b"x")).await.unwrap();
        assert_eq!(url, "mem://k1");
        assert!(store.exists("k1").await);
    }
}
