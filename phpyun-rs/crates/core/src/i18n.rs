//! Localization (i18n) support.
//!
//! ## Design points
//!
//! - **Storage**: `rust-i18n` embeds `locales/<lang>.json` into the binary's
//!   `.rodata` at compile time, so runtime lookups do zero IO, take no locks,
//!   and don't allocate on the heap. Lookup cost is ~80–150 ns. We chose JSON
//!   so the backend, app, and web frontends can share the same translation
//!   files.
//! - **Translation timing**: error objects only carry an i18n key plus args;
//!   **translation happens exactly once, during `IntoResponse` materialization**,
//!   so service / repo / handler intermediate layers pay zero translation cost.
//! - **Lang resolution**: parsed once per request and cached into
//!   `Request::extensions`; every later step reads from the extension.
//!
//! ## How clients pass the language
//!
//! Priority (highest to lowest):
//!
//! 1. URL query: `?lang=en` (highest; for dev / test).
//! 2. HTTP header: `Accept-Language: zh-TW` (W3C standard; mobile / browser
//!    sends it automatically).
//! 3. Cookie: `lang=en` (persistent site preference).
//! 4. Server default: `zh-CN`.
//!
//! ```text
//! curl -H 'Accept-Language: en' http://api.example.com/...   ← recommended
//! curl 'http://api.example.com/...?lang=zh-TW'               ← debugging / static sharing
//! ```
//!
//! ## Response format
//!
//! Error responses return both a machine-readable `key` and a `msg` translated
//! into the current language:
//!
//! ```json
//! {
//!   "code": 400,
//!   "key":  "errors.bad_credentials",
//!   "msg":  "Invalid account or password",
//!   "data": null
//! }
//! ```
//!
//! Two ways for the client to use this:
//! - Simple display: use `msg` directly.
//! - Custom: branch on `key` (e.g. on "bad password" auto-jump to forgot
//!   password) and ignore `msg`.

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use serde::Serialize;
use std::convert::Infallible;

// `rust_i18n::i18n!` is invoked once at the top of lib.rs (it generates
// crate-root symbols); submodules just use `t!` directly.

/// Supported languages. `Copy` type, so propagation / comparison is zero-cost.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lang {
    #[serde(rename = "zh-CN")]
    ZhCN,
    #[serde(rename = "zh-TW")]
    ZhTW,
    En,
}

impl Default for Lang {
    fn default() -> Self {
        Self::ZhCN
    }
}

impl Lang {
    /// `rust-i18n` / `Accept-Language` standard BCP-47 tag.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ZhCN => "zh-CN",
            Self::ZhTW => "zh-TW",
            Self::En => "en",
        }
    }

    /// Parse a string into `Lang`, normalizing common variants:
    /// `zh / zh-cn / cn` → `ZhCN`, `zh-tw / zh-hk / tw` → `ZhTW`,
    /// `en / en-us / en-gb` → `En`. Unrecognized inputs return `None`.
    pub fn parse_tag(s: &str) -> Option<Self> {
        let lower = s.trim().to_ascii_lowercase();
        match lower.as_str() {
            "zh" | "zh-cn" | "zh-hans" | "cn" | "zh-hans-cn" => Some(Self::ZhCN),
            "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant" | "tw" | "zh-hant-tw" => Some(Self::ZhTW),
            "en" | "en-us" | "en-gb" | "en-au" | "en-ca" | "us" => Some(Self::En),
            _ => None,
        }
    }

    /// Fallback chain: current language → Simplified Chinese → English.
    /// Used for DB dictionary lookups: first try the current language, then
    /// fall back through this chain on misses.
    pub fn fallback_chain(self) -> &'static [Lang] {
        match self {
            Self::ZhTW => &[Self::ZhTW, Self::ZhCN, Self::En],
            Self::En => &[Self::En, Self::ZhCN],
            Self::ZhCN => &[Self::ZhCN, Self::En],
        }
    }
}

/// Axum extractor that detects the language from the request and caches it in
/// `Request::extensions`.
///
/// Multiple extractions on the same request only parse once.
impl<S: Send + Sync> FromRequestParts<S> for Lang {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Already parsed → reuse (middleware / multiple extractors don't re-parse).
        if let Some(l) = parts.extensions.get::<Lang>().copied() {
            return Ok(l);
        }
        let lang = detect_lang(parts);
        parts.extensions.insert(lang);
        Ok(lang)
    }
}

/// Detect the language: query > header > cookie > default.
fn detect_lang(parts: &Parts) -> Lang {
    // 1. URL ?lang=
    if let Some(q) = parts.uri.query() {
        for kv in q.split('&') {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            if k == "lang" {
                if let Some(l) = Lang::parse_tag(v) {
                    return l;
                }
            }
        }
    }

    // 2. Accept-Language: zh-CN,zh;q=0.9,en;q=0.8
    if let Some(al) = parts
        .headers
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
    {
        // Simplified parsing: take the first recognizable tag after sorting by q.
        // q defaults to 1.0; entries without an explicit q have the highest priority.
        // We cheat here and don't strictly sort per RFC — we just take the first recognizable one in order.
        for entry in al.split(',') {
            let tag = entry.split(';').next().unwrap_or("").trim();
            if let Some(l) = Lang::parse_tag(tag) {
                return l;
            }
        }
    }

    // 3. Cookie: lang=zh-TW
    if let Some(cookies) = parts
        .headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
    {
        for c in cookies.split(';') {
            let kv = c.trim();
            if let Some(v) = kv.strip_prefix("lang=") {
                if let Some(l) = Lang::parse_tag(v) {
                    return l;
                }
            }
        }
    }

    // 4. Default
    Lang::default()
}

// ============== Task-level Lang context ==============
//
// Errors bubble up through `?` across multiple layers, and translation only
// needs to happen when `IntoResponse` materializes. `IntoResponse` doesn't
// have access to the original Request, so we use `tokio::task_local!` to
// attach the current request's Lang to the task context, and IntoResponse
// reads it directly. Each request gets its own scope — no cross-talk.

tokio::task_local! {
    /// The language of the current request. Set per-request by the `lang_layer` middleware.
    pub static CURRENT_LANG: Lang;
}

/// Reads Lang during error response materialization. Falls back to the
/// default when the task-local is absent (e.g. not on a request path).
pub fn current_lang() -> Lang {
    CURRENT_LANG.try_with(|l| *l).unwrap_or_default()
}

/// Global middleware: detects the language for this request, writes it into
/// `Request::extensions`, and sets the task-local.
///
/// Automatically attached by `mw::install`; business routes don't need to add it manually.
pub async fn lang_layer(
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let detected = detect_lang_from(req.uri(), req.headers());
    // Store in extensions so a later Lang extractor can reuse it and avoid re-parsing.
    req.extensions_mut().insert(detected);
    // Run inner inside the task-local scope so IntoResponse can read it on materialization.
    CURRENT_LANG.scope(detected, next.run(req)).await
}

/// Parse lang from uri + headers (used by lang_layer, doesn't depend on axum::http::request::Parts).
fn detect_lang_from(uri: &axum::http::Uri, headers: &axum::http::HeaderMap) -> Lang {
    if let Some(q) = uri.query() {
        for kv in q.split('&') {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            if k == "lang" {
                if let Some(l) = Lang::parse_tag(v) {
                    return l;
                }
            }
        }
    }
    if let Some(al) = headers
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
    {
        for entry in al.split(',') {
            let tag = entry.split(';').next().unwrap_or("").trim();
            if let Some(l) = Lang::parse_tag(tag) {
                return l;
            }
        }
    }
    if let Some(cookies) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) {
        for c in cookies.split(';') {
            let kv = c.trim();
            if let Some(v) = kv.strip_prefix("lang=") {
                if let Some(l) = Lang::parse_tag(v) {
                    return l;
                }
            }
        }
    }
    Lang::default()
}

// ============== Translation entry points ==============
//
// Business code does not call `rust_i18n::t!` directly (that macro only works
// with locales known at compile time). Use `t(key, lang)` / `t_args(key,
// lang, args)` instead, which switch the locale at runtime.

/// Simple translation, no parameter interpolation.
pub fn t(key: &str, lang: Lang) -> String {
    rust_i18n::t!(key, locale = lang.as_str()).into_owned()
}

/// Translation with named parameters. `args` is `[("name", "value"), ...]`.
///
/// Example:
/// ```ignore
/// let msg = t_args("errors.param_missing", Lang::ZhCN, &[("field", "username")]);
/// // → "缺少必填参数：username"
/// ```
///
/// Implementation: first pull out the localized template via `t!` (which has
/// its own fallback), then run `replace_patterns` to interpolate the parameters.
pub fn t_args(key: &str, lang: Lang, args: &[(&str, &str)]) -> String {
    let raw = rust_i18n::t!(key, locale = lang.as_str()).into_owned();
    if args.is_empty() {
        return raw;
    }
    let patterns: Vec<&str> = args.iter().map(|(k, _)| *k).collect();
    let values: Vec<String> = args.iter().map(|(_, v)| (*v).to_string()).collect();
    rust_i18n::replace_patterns(&raw, &patterns, &values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "debug helper"]
    fn debug_available_locales() {
        let langs: Vec<_> = rust_i18n::available_locales!();
        eprintln!("Available locales: {:?}", langs);
        eprintln!(
            "zh-CN errors.unauth: {:?}",
            t("errors.unauth", Lang::ZhCN)
        );
    }

    #[test]
    fn parse_common_tags() {
        assert_eq!(Lang::parse_tag("zh-CN"), Some(Lang::ZhCN));
        assert_eq!(Lang::parse_tag("ZH-cn"), Some(Lang::ZhCN));
        assert_eq!(Lang::parse_tag("zh"), Some(Lang::ZhCN));
        assert_eq!(Lang::parse_tag("zh-TW"), Some(Lang::ZhTW));
        assert_eq!(Lang::parse_tag("zh-Hant"), Some(Lang::ZhTW));
        assert_eq!(Lang::parse_tag("en"), Some(Lang::En));
        assert_eq!(Lang::parse_tag("en-US"), Some(Lang::En));
        assert_eq!(Lang::parse_tag("ja"), None);
    }

    #[test]
    fn translate_basic() {
        assert!(t("errors.unauth", Lang::ZhCN).contains("未登录"));
        assert!(t("errors.unauth", Lang::ZhTW).contains("未登入"));
        assert_eq!(t("errors.unauth", Lang::En), "Not logged in");
    }

    #[test]
    fn translate_with_args() {
        let s = t_args(
            "errors.param_missing",
            Lang::En,
            &[("field", "username")],
        );
        assert_eq!(s, "Missing required parameter: username");
    }

    #[test]
    fn translate_fallback() {
        // Intentionally request a key that doesn't exist — should return the key itself.
        let s = t("errors.does_not_exist", Lang::En);
        assert_eq!(s, "errors.does_not_exist");
    }
}
