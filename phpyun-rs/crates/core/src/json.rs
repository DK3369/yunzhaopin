//! Thin facade over `serde_json`.
//!
//! Lets business code just `use phpyun_core::json;` without pulling in `serde_json` directly.
//! Failures are uniformly mapped to `AppError::internal(e)` (code=500, tag=internal),
//! so handlers don't need to write `map_err` manually.

use crate::error::AppError;
use serde::{de::DeserializeOwned, Serialize};

pub type Value = serde_json::Value;
pub type Map = serde_json::Map<String, Value>;

#[inline]
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String, AppError> {
    serde_json::to_string(value).map_err(AppError::internal)
}

#[inline]
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, AppError> {
    serde_json::from_str(s).map_err(AppError::internal)
}

#[inline]
pub fn to_value<T: Serialize>(value: &T) -> Result<Value, AppError> {
    serde_json::to_value(value).map_err(AppError::internal)
}

#[inline]
pub fn from_value<T: DeserializeOwned>(v: Value) -> Result<T, AppError> {
    serde_json::from_value(v).map_err(AppError::internal)
}

/// Construct inline JSON (used on error paths). Equivalent to `serde_json::json!`.
pub use serde_json::json;
