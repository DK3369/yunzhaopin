//! PHP `serialize()` for string lists (`fromscore` / `option` / `score`).

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
    parse_php_array(s)
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

fn parse_php_array(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut i = 0;
    if b.len() >= 2 && b[0] == b'a' && b[1] == b':' {
        while i < b.len() && b[i] != b'{' {
            i += 1;
        }
        if i < b.len() {
            i += 1;
        }
    }
    let mut out = Vec::new();
    while i < b.len() {
        if b[i] == b'}' {
            break;
        }
        if parse_php_value(b, &mut i).is_none() {
            break;
        }
        match parse_php_value(b, &mut i) {
            Some(v) => out.push(v),
            None => break,
        }
    }
    out
}

fn parse_php_value(b: &[u8], i: &mut usize) -> Option<String> {
    if *i >= b.len() {
        return None;
    }
    match b[*i] {
        b's' => {
            *i += 1;
            if *i >= b.len() || b[*i] != b':' {
                return None;
            }
            *i += 1;
            let mut n = 0usize;
            while *i < b.len() && b[*i].is_ascii_digit() {
                n = n * 10 + (b[*i] - b'0') as usize;
                *i += 1;
            }
            if *i < b.len() && b[*i] == b':' {
                *i += 1;
            }
            if *i < b.len() && b[*i] == b'"' {
                *i += 1;
            }
            let end = (*i + n).min(b.len());
            let s = std::str::from_utf8(&b[*i..end]).ok()?.to_string();
            *i = end;
            if *i < b.len() && b[*i] == b'"' {
                *i += 1;
            }
            if *i < b.len() && b[*i] == b';' {
                *i += 1;
            }
            Some(s)
        }
        b'i' => {
            *i += 1;
            if *i >= b.len() || b[*i] != b':' {
                return None;
            }
            *i += 1;
            let start = *i;
            if *i < b.len() && b[*i] == b'-' {
                *i += 1;
            }
            while *i < b.len() && b[*i].is_ascii_digit() {
                *i += 1;
            }
            let s = std::str::from_utf8(&b[start..*i]).ok()?.to_string();
            if *i < b.len() && b[*i] == b';' {
                *i += 1;
            }
            Some(s)
        }
        b'N' => {
            *i += 1;
            if *i < b.len() && b[*i] == b';' {
                *i += 1;
            }
            Some(String::new())
        }
        _ => {
            *i += 1;
            None
        }
    }
}
