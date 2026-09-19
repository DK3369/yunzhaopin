//! JSON helpers for `phpyun_rs_pay_method.config_json`.

use serde_json::Value;

pub fn config_str(json: &str, key: &str) -> String {
    let Ok(v) = serde_json::from_str::<Value>(json) else {
        return String::new();
    };
    match v.get(key) {
        Some(Value::String(s)) => s.trim().to_string(),
        _ => String::new(),
    }
}

pub fn secret_key(json: &str) -> String {
    let mut s = config_str(json, "secret_key");
    if s.is_empty() {
        s = config_str(json, "sk");
    }
    s
}

pub fn set_str(json: &str, key: &str, value: &str) -> String {
    let mut v: Value = serde_json::from_str(json).unwrap_or_else(|_| serde_json::json!({}));
    if !v.is_object() {
        v = serde_json::json!({});
    }
    v[key] = serde_json::json!(value);
    v.to_string()
}
