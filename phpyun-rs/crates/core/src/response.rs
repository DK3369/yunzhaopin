//! Unified API response contract.
//!
//! ```json
//! // Success
//! { "code": 200, "msg": "ok", "data": { ... } }
//!
//! // Failure (different errors carry different codes + short tags; see error.rs)
//! { "code": 401, "msg": "unauth",                "data": null }
//! { "code": 429, "msg": "rate_limit",            "data": null }
//! { "code": 500, "msg": "biz:insufficient_balance", "data": null }
//! ```
//!
//! ## Design points
//! - `code` aligns with the HTTP status: frontend, backend, and monitoring all
//!   read the same number.
//! - `msg` is a short ASCII tag (`unauth` / `rate_limit` / ...); the frontend
//!   handles i18n.
//! - `data` is the business payload on success; on failure it's `null`
//!   (`Option::is_none` omits the field during serialization to save bytes).
//! - `BusinessError(s)` can carry a custom phrase in the form `"biz:<phrase>"`.
//!
//! ## Frontend decision logic
//! ```js
//! if (resp.code === 200) {
//!   use resp.data
//! } else {
//!   // resp.msg is a short tag; the frontend maps it to user-visible copy
//!   showToast(i18n[resp.msg] ?? resp.msg)
//! }
//! ```

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Success code. Every successful endpoint uses this.
pub const CODE_OK: u16 = 200;

#[derive(Debug, Serialize)]
pub struct ApiBody<T: Serialize> {
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiBody<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: CODE_OK,
            msg: "ok".into(),
            data: Some(data),
        }
    }

    /// Failure envelope. Business code normally doesn't construct this directly;
    /// `AppError::into_response()` builds it automatically. The API is kept for
    /// the rare case where you need to bypass the `AppError` enum.
    pub fn err(code: u16, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

/// Handler return type. `impl IntoResponse` automatically wraps the value in
/// `{code:200, msg:"ok", data}`.
///
/// ```ignore
/// pub async fn me(...) -> AppResult<ApiJson<MeData>> {
///     Ok(ApiJson(profile))
/// }
/// ```
pub struct ApiJson<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiJson<T> {
    fn into_response(self) -> Response {
        Json(ApiBody::ok(self.0)).into_response()
    }
}

/// Success response with only `msg` and no `data` (e.g. `ApiOk("deleted")`).
pub struct ApiOk(pub &'static str);
impl IntoResponse for ApiOk {
    fn into_response(self) -> Response {
        Json(ApiBody::<()> {
            code: CODE_OK,
            msg: self.0.into(),
            data: None,
        })
        .into_response()
    }
}

/// Success response whose `msg` goes through the i18n table at runtime.
///
/// Pass either a fully-qualified key (`"messages.collect_added"`) or a short
/// snake_case form (`"collect_added"`); the latter is auto-prefixed with
/// `messages.` for lookup. Falls back to the literal string if no translation
/// matches.
///
/// ```ignore
/// pub async fn add(...) -> AppResult<ApiMsg> {
///     ...
///     Ok(ApiMsg("collect_added"))
/// }
/// ```
pub struct ApiMsg(pub &'static str);

impl IntoResponse for ApiMsg {
    fn into_response(self) -> Response {
        let lang = crate::i18n::current_lang();
        let msg = resolve_msg_key(self.0, lang);
        Json(ApiBody::<()> {
            code: CODE_OK,
            msg,
            data: None,
        })
        .into_response()
    }
}

/// Like `ApiMsg` but also carries a `data` payload — useful when the action
/// needs to tell the client *both* a translatable status message AND the
/// resulting state (e.g. toggle endpoints).
///
/// ```ignore
/// pub async fn toggle(...) -> AppResult<ApiMsgData<ToggleResp>> {
///     let now = collect_service::toggle(...).await?;
///     Ok(ApiMsgData {
///         msg_key: if now { "collect_added" } else { "collect_removed" },
///         data: ToggleResp { favorited: now },
///     })
/// }
/// ```
pub struct ApiMsgData<T: Serialize> {
    pub msg_key: &'static str,
    pub data: T,
}

impl<T: Serialize> IntoResponse for ApiMsgData<T> {
    fn into_response(self) -> Response {
        let lang = crate::i18n::current_lang();
        let msg = resolve_msg_key(self.msg_key, lang);
        Json(ApiBody {
            code: CODE_OK,
            msg,
            data: Some(self.data),
        })
        .into_response()
    }
}

fn resolve_msg_key(key: &'static str, lang: crate::i18n::Lang) -> String {
    let translated = crate::i18n::t(key, lang);
    if translated != key {
        translated
    } else if !key.contains('.') {
        let prefixed = format!("messages.{key}");
        let t2 = crate::i18n::t(&prefixed, lang);
        if t2 != prefixed {
            t2
        } else {
            key.to_string()
        }
    } else {
        key.to_string()
    }
}

/// Paged response body.
#[derive(Debug, Serialize)]
pub struct Paged<T: Serialize> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

impl<T: Serialize> Paged<T> {
    pub fn new(list: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        Self {
            list,
            total,
            page,
            page_size,
        }
    }

    /// Build a paged response from a service-returned `(list, total)` pair by
    /// converting each item via `T::from`. Replaces the 4-line
    /// `Paged::new(r.list.into_iter().map(T::from).collect(), r.total,
    /// page.page, page.page_size)` boilerplate that was copy-pasted into 50+
    /// list handlers.
    pub fn from_listing<U>(list: Vec<U>, total: u64, page: crate::Pagination) -> Self
    where
        T: From<U>,
    {
        Self {
            list: list.into_iter().map(T::from).collect(),
            total,
            page: page.page,
            page_size: page.page_size,
        }
    }
}

// Legacy alias kept for backward compatibility.
pub type ApiResponse<T> = ApiBody<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_body_has_code_200_and_msg_ok() {
        let body = ApiBody::ok(42u32);
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["code"], 200);
        assert_eq!(json["msg"], "ok");
        assert_eq!(json["data"], 42);
    }

    #[test]
    fn err_body_omits_data() {
        let body: ApiBody<()> = ApiBody::err(401, "unauth");
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["code"], 401);
        assert_eq!(json["msg"], "unauth");
        assert!(json.get("data").is_none(), "data should be omitted when None");
    }

    #[test]
    fn paged_serializes_list_total_page() {
        let p = Paged::new(vec![1u32, 2, 3], 10, 1, 3);
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["list"], serde_json::json!([1, 2, 3]));
        assert_eq!(json["total"], 10);
        assert_eq!(json["page"], 1);
        assert_eq!(json["page_size"], 3);
    }
}
