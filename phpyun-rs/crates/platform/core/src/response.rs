//! Unified API response contract.
//!
//! ```json
//! // Success
//! { "code": 200, "key": "ok",         "msg": "ok",            "data": { ... } }
//!
//! // Failure (see error.rs for stable keys and translation)
//! { "code": 401, "key": "unauth",     "msg": "Not logged in", "data": "" }
//! { "code": 429, "key": "rate_limit", "msg": "Too many requests…", "data": "" }
//! ```
//!
//! ## Design points
//! - `code` aligns with the HTTP status: frontend, backend, and monitoring all
//!   read the same number.
//! - `key` is the stable machine-readable identifier and is present on every
//!   response, success or failure. It never contains free-text detail.
//! - `msg` is already translated, for display only — never parse it.
//! - `data` is the business payload on success; when there is no payload it is
//!   serialized as an empty string.
//!
//! ## Frontend decision logic
//! ```js
//! if (resp.code === 200) {
//!   use resp.data
//! } else {
//!   // Branch on resp.key; resp.msg is already localized for display.
//!   if (resp.key === "session_expired") relogin()
//!   else showToast(resp.msg)
//! }
//! ```

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Serialize, Serializer};

/// Success code. Every successful endpoint uses this.
pub const CODE_OK: u16 = 200;

/// Success key. Present on every successful response so clients can read
/// `key` unconditionally instead of special-casing the success branch.
pub const KEY_OK: &str = "ok";

#[derive(Debug, Serialize)]
pub struct ApiBody<T: Serialize> {
    pub code: u16,
    /// Stable machine-readable identifier. `"ok"` on success, otherwise the
    /// `ApiError` key. Clients branch on this; `msg` is display-only.
    pub key: String,
    pub msg: String,
    #[serde(serialize_with = "serialize_data")]
    pub data: Option<T>,
}

fn serialize_data<T: Serialize, S: Serializer>(
    data: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match data {
        Some(data) => data.serialize(serializer),
        None => "".serialize(serializer),
    }
}

impl<T: Serialize> ApiBody<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: CODE_OK,
            key: KEY_OK.into(),
            msg: "ok".into(),
            data: Some(data),
        }
    }

    /// Failure envelope. `ApiError::into_response()` builds it automatically.
    pub fn err(code: u16, key: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code,
            key: key.into(),
            msg: msg.into(),
            data: None,
        }
    }
}

/// Unified successful handler response.
///
/// Use [`Self::data`] for a normal payload, [`Self::message`] for a translated
/// message without data, or [`Self::message_data`] when both are needed.
pub struct ApiResponse<T: Serialize = ()> {
    msg_key: Option<&'static str>,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn data(data: T) -> Self {
        Self {
            msg_key: None,
            data: Some(data),
        }
    }

    pub fn message_data(msg_key: &'static str, data: T) -> Self {
        Self {
            msg_key: Some(msg_key),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn message(msg_key: &'static str) -> Self {
        Self {
            msg_key: Some(msg_key),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let lang = crate::i18n::current_lang();
        let msg = self
            .msg_key
            .map(|key| resolve_msg_key(key, lang))
            .unwrap_or_else(|| KEY_OK.to_owned());
        Json(ApiBody {
            code: CODE_OK,
            key: self.msg_key.unwrap_or(KEY_OK).to_owned(),
            msg,
            data: self.data,
        })
        .into_response()
    }
}

fn resolve_msg_key(key: &'static str, lang: crate::i18n::Lang) -> String {
    let translated = crate::i18n::t(key, lang);
    if translated != key {
        return translated;
    }
    if !key.contains('.') {
        for prefix in ["messages.", "common."] {
            let prefixed = format!("{prefix}{key}");
            let t2 = crate::i18n::t(&prefixed, lang);
            if t2 != prefixed {
                return t2;
            }
        }
    }
    key.to_string()
}

/// Paged response body.
///
/// Not `#[derive(ToSchema)]`-able because many call sites instantiate this
/// over raw `phpyun_models::*` entities that don't implement `ToSchema`. For
/// OpenAPI documentation of paged endpoints, leave `body` off the
/// `responses(...)` macro — the response shape (`{list, total, page,
/// page_size}`) is implicit from the project's response contract.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_body_has_code_200_and_msg_ok() {
        let body = ApiBody::ok(42u32);
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["code"], 200);
        assert_eq!(json["key"], "ok");
        assert_eq!(json["msg"], "ok");
        assert_eq!(json["data"], 42);
    }

    #[test]
    fn err_body_uses_empty_string_data() {
        let body: ApiBody<()> = ApiBody::err(401, "unauth", "Not logged in");
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["code"], 401);
        assert_eq!(json["key"], "unauth");
        assert_eq!(json["msg"], "Not logged in");
        assert_eq!(json["data"], "");
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
