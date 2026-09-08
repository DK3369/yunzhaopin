//! Reader for PHP `serialize()` payloads stored in legacy columns.
//!
//! Two shapes show up: flat string lists (`phpyun_evaluate.option`) and whole
//! table rows keyed by column name (`phpyun_recycle.body`, written by PHP's
//! `insert_recycle`). Both come out of the same scalar parser here.
//!
//! Rows land in `phpyun_recycle` via `mysql_fetch_assoc`, so every value is a
//! string or SQL `NULL` — `Option<String>` keeps that distinction, which the
//! recycle-bin restore needs to write `NULL` back as `NULL`.
//!
//! Nested arrays and objects are not supported: a value we cannot read scalar
//! aborts the parse and the caller gets whatever was collected so far, which is
//! how PHP's own `unserialize` failure surfaces (a falsy body that restores
//! nothing).

/// A `NULL`-aware scalar. `None` is PHP `N;`.
pub type Scalar = Option<String>;

/// Writes a table row back out as PHP would, so `phpyun_recycle.body` stays
/// readable by the legacy admin as well as by `unserialize_pairs`.
///
/// PHP serializes the `mysql_fetch_assoc` array verbatim, which means string
/// keys, string values, and `N;` for SQL `NULL`. The lengths are byte counts,
/// matching `String::len`.
pub fn serialize_row(cols: &[(String, Scalar)]) -> String {
    let mut out = format!("a:{}:{{", cols.len());
    for (name, value) in cols {
        push_str(&mut out, name);
        match value {
            Some(v) => push_str(&mut out, v),
            None => out.push_str("N;"),
        }
    }
    out.push('}');
    out
}

fn push_str(out: &mut String, s: &str) {
    out.push_str(&format!("s:{}:\"{s}\";", s.len()));
}

/// Column name → value pairs from `a:N:{s:..;s:..;…}`.
///
/// Integer keys are stringified so callers can treat both key kinds uniformly;
/// the recycle bin only ever sees string keys.
pub fn unserialize_pairs(raw: &str) -> Vec<(String, Scalar)> {
    let b = raw.trim().as_bytes();
    let mut i = array_body_start(b);
    let mut out = Vec::new();
    while i < b.len() && b[i] != b'}' {
        let Some(Some(key)) = next_scalar(b, &mut i) else {
            break;
        };
        let Some(val) = next_scalar(b, &mut i) else {
            break;
        };
        out.push((key, val));
    }
    out
}

/// Values only, dropping the keys — for the flat `i:0;s:..;` lists.
pub fn unserialize_values(raw: &str) -> Vec<String> {
    let b = raw.trim().as_bytes();
    let mut i = array_body_start(b);
    let mut out = Vec::new();
    while i < b.len() && b[i] != b'}' {
        if next_scalar(b, &mut i).is_none() {
            break;
        }
        match next_scalar(b, &mut i) {
            Some(v) => out.push(v.unwrap_or_default()),
            None => break,
        }
    }
    out
}

/// Index just past the `{` of a leading `a:N:{`, or 0 when there is no header.
fn array_body_start(b: &[u8]) -> usize {
    if b.len() < 2 || b[0] != b'a' || b[1] != b':' {
        return 0;
    }
    match b.iter().position(|c| *c == b'{') {
        Some(p) => p + 1,
        None => b.len(),
    }
}

/// Reads one scalar and leaves `i` after its trailing `;`.
///
/// `Some(None)` is PHP `N;`; `None` means the token is not a scalar we read
/// (nested array/object) and the parse must stop.
fn next_scalar(b: &[u8], i: &mut usize) -> Option<Scalar> {
    if *i >= b.len() {
        return None;
    }
    match b[*i] {
        b's' => read_string(b, i).map(Some),
        // `i` / `d` / `b` all round-trip as their decimal text, which is what
        // the column held before PHP serialized it.
        b'i' | b'd' | b'b' => read_until_semicolon(b, i).map(Some),
        b'N' => {
            *i += 1;
            skip_byte(b, i, b';');
            Some(None)
        }
        _ => None,
    }
}

fn read_string(b: &[u8], i: &mut usize) -> Option<String> {
    *i += 1;
    skip_byte(b, i, b':')?;
    let len = read_usize(b, i)?;
    skip_byte(b, i, b':')?;
    skip_byte(b, i, b'"')?;
    let end = i.checked_add(len)?;
    if end > b.len() {
        return None;
    }
    // Lengths are byte counts, so a truncated multi-byte char means the payload
    // is corrupt; bail rather than guess.
    let s = std::str::from_utf8(&b[*i..end]).ok()?.to_string();
    *i = end;
    skip_byte(b, i, b'"')?;
    skip_byte(b, i, b';');
    Some(s)
}

fn read_until_semicolon(b: &[u8], i: &mut usize) -> Option<String> {
    *i += 1;
    skip_byte(b, i, b':')?;
    let start = *i;
    while *i < b.len() && b[*i] != b';' {
        *i += 1;
    }
    let s = std::str::from_utf8(&b[start..*i]).ok()?.to_string();
    skip_byte(b, i, b';');
    Some(s)
}

fn read_usize(b: &[u8], i: &mut usize) -> Option<usize> {
    let start = *i;
    while *i < b.len() && b[*i].is_ascii_digit() {
        *i += 1;
    }
    if start == *i {
        return None;
    }
    std::str::from_utf8(&b[start..*i]).ok()?.parse().ok()
}

/// Consumes `want` when present. `None` signals the payload is malformed;
/// callers that treat the byte as optional ignore the result.
fn skip_byte(b: &[u8], i: &mut usize, want: u8) -> Option<()> {
    if *i < b.len() && b[*i] == want {
        *i += 1;
        return Some(());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_recycle_row() {
        let raw = "a:3:{s:2:\"id\";s:1:\"7\";s:4:\"name\";s:6:\"张三\";s:6:\"remark\";N;}";
        assert_eq!(
            unserialize_pairs(raw),
            vec![
                ("id".to_string(), Some("7".to_string())),
                ("name".to_string(), Some("张三".to_string())),
                ("remark".to_string(), None),
            ]
        );
    }

    #[test]
    fn keeps_quotes_and_semicolons_inside_strings() {
        let raw = "a:1:{s:4:\"body\";s:12:\"a\";b:1;c\"xyz\";}";
        assert_eq!(
            unserialize_pairs(raw),
            vec![("body".to_string(), Some("a\";b:1;c\"xyz".to_string()))]
        );
    }

    #[test]
    fn stops_on_nested_array_instead_of_looping() {
        let raw = "a:2:{s:1:\"a\";s:1:\"1\";s:1:\"b\";a:1:{i:0;s:1:\"x\";}}";
        assert_eq!(
            unserialize_pairs(raw),
            vec![("a".to_string(), Some("1".to_string()))]
        );
    }

    #[test]
    fn reads_flat_value_lists() {
        let raw = "a:2:{i:0;s:1:\"a\";i:1;s:1:\"b\";}";
        assert_eq!(unserialize_values(raw), vec!["a", "b"]);
    }

    #[test]
    fn round_trips_a_row_through_php_syntax() {
        let row = vec![
            ("id".to_string(), Some("7".to_string())),
            ("name".to_string(), Some("张三".to_string())),
            ("remark".to_string(), None),
        ];
        let raw = serialize_row(&row);
        assert_eq!(
            raw,
            "a:3:{s:2:\"id\";s:1:\"7\";s:4:\"name\";s:6:\"张三\";s:6:\"remark\";N;}"
        );
        assert_eq!(unserialize_pairs(&raw), row);
    }

    #[test]
    fn empty_and_garbage_yield_nothing() {
        assert!(unserialize_pairs("").is_empty());
        assert!(unserialize_pairs("not-serialized").is_empty());
        assert!(unserialize_values("a:0:{}").is_empty());
    }
}
