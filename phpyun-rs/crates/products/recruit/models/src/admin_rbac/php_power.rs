//! Parse PHP `serialize()` of `admin_user_group.group_power` (flat id list).

/// Extract integer nav ids from a PHP serialized array such as
/// `a:2:{i:0;i:216;i:1;s:3:"226";}`. Nested arrays are flattened.
pub fn parse_group_power(raw: &str) -> Vec<i64> {
    let s = raw.trim();
    if s.is_empty() || s == "N;" {
        return Vec::new();
    }
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    parse_value(bytes, &mut i, &mut out);
    out.sort_unstable();
    out.dedup();
    out
}

fn parse_value(bytes: &[u8], i: &mut usize, out: &mut Vec<i64>) {
    skip_ws(bytes, i);
    if *i >= bytes.len() {
        return;
    }
    match bytes[*i] {
        b'N' => {
            *i += 1;
            eat(bytes, i, b';');
        }
        b'i' => {
            *i += 1;
            eat(bytes, i, b':');
            if let Some(n) = read_i64(bytes, i) {
                if n > 0 {
                    out.push(n);
                }
            }
            eat(bytes, i, b';');
        }
        b's' => {
            *i += 1;
            eat(bytes, i, b':');
            let len = read_usize(bytes, i);
            eat(bytes, i, b':');
            eat(bytes, i, b'"');
            let end = (*i + len).min(bytes.len());
            if let Ok(body) = std::str::from_utf8(&bytes[*i..end]) {
                if let Ok(n) = body.parse::<i64>() {
                    if n > 0 {
                        out.push(n);
                    }
                }
            }
            *i = end;
            eat(bytes, i, b'"');
            eat(bytes, i, b';');
        }
        b'a' => {
            *i += 1;
            eat(bytes, i, b':');
            let n = read_usize(bytes, i);
            eat(bytes, i, b':');
            eat(bytes, i, b'{');
            for _ in 0..n {
                // key
                skip_entry(bytes, i);
                // value
                parse_value(bytes, i, out);
            }
            eat(bytes, i, b'}');
        }
        _ => {
            *i += 1;
        }
    }
}

fn skip_entry(bytes: &[u8], i: &mut usize) {
    skip_ws(bytes, i);
    if *i >= bytes.len() {
        return;
    }
    match bytes[*i] {
        b'i' => {
            *i += 1;
            eat(bytes, i, b':');
            let _ = read_i64(bytes, i);
            eat(bytes, i, b';');
        }
        b's' => {
            *i += 1;
            eat(bytes, i, b':');
            let len = read_usize(bytes, i);
            eat(bytes, i, b':');
            eat(bytes, i, b'"');
            *i = (*i + len).min(bytes.len());
            eat(bytes, i, b'"');
            eat(bytes, i, b';');
        }
        _ => parse_value(bytes, i, &mut Vec::new()),
    }
}

fn skip_ws(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn eat(bytes: &[u8], i: &mut usize, c: u8) {
    if *i < bytes.len() && bytes[*i] == c {
        *i += 1;
    }
}

fn read_i64(bytes: &[u8], i: &mut usize) -> Option<i64> {
    let start = *i;
    if *i < bytes.len() && bytes[*i] == b'-' {
        *i += 1;
    }
    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        *i += 1;
    }
    std::str::from_utf8(&bytes[start..*i]).ok()?.parse().ok()
}

fn read_usize(bytes: &[u8], i: &mut usize) -> usize {
    read_i64(bytes, i).unwrap_or(0).max(0) as usize
}

#[cfg(test)]
mod tests {
    use super::parse_group_power;

    #[test]
    fn php_int_array_skips_keys() {
        let ids = parse_group_power("a:3:{i:0;i:216;i:1;i:226;i:2;i:40;}");
        assert_eq!(ids, vec![40, 216, 226]);
    }

    #[test]
    fn php_string_ids() {
        let ids = parse_group_power(r#"a:2:{i:0;s:3:"216";i:1;s:2:"40";}"#);
        assert_eq!(ids, vec![40, 216]);
    }

    #[test]
    fn php_null() {
        assert!(parse_group_power("N;").is_empty());
        assert!(parse_group_power("").is_empty());
    }
}
