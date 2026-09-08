//! PHP `serialize()` for string lists (`fromscore` / `option` / `score`).
//!
//! Reading is delegated to [`crate::php_ser`], which the recycle bin shares.

pub fn serialize_strings(items: &[String]) -> String {
    let mut out = format!("a:{}:{{", items.len());
    for (i, s) in items.iter().enumerate() {
        out.push_str(&format!("i:{i};s:{}:\"{s}\";", s.len()));
    }
    out.push('}');
    out
}

pub fn unserialize_strings(raw: &str) -> Vec<String> {
    let s = raw.trim();
    if s.is_empty() {
        return Vec::new();
    }
    if s.starts_with('[') {
        if let Ok(v) = serde_json::from_str::<Vec<String>>(s) {
            return v;
        }
        if let Ok(v) = serde_json::from_str::<Vec<serde_json::Value>>(s) {
            return v
                .into_iter()
                .map(|x| match x {
                    serde_json::Value::String(t) => t,
                    other => other.to_string(),
                })
                .collect();
        }
    }
    crate::php_ser::unserialize_values(s)
}

pub fn json_to_strings(v: &serde_json::Value) -> Vec<String> {
    match v {
        serde_json::Value::Null => Vec::new(),
        serde_json::Value::String(s) => unserialize_strings(s),
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(|x| match x {
                serde_json::Value::String(t) => t.clone(),
                other => other.to_string(),
            })
            .collect(),
        other => unserialize_strings(&other.to_string()),
    }
}
