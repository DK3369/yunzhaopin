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
