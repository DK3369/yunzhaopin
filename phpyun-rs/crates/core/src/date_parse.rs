//! Loose date/datetime parsing for handlers that accept PHPYun-shaped input.
//!
//! PHPYun's frontend sends dates as the human-readable formats produced by
//! its date pickers (`"YYYY-MM"`, `"YYYY-MM-DD"`, `"YYYY-MM-DD HH:MM:SS"`,
//! occasionally `"YYYY/MM/DD"`); the PHP backend then runs `strtotime()` on
//! them. The DB stores Unix timestamps in `int(10)` columns.
//!
//! This module provides serde deserializers that accept any of those forms
//! plus raw integer / string-encoded integer Unix timestamps, so the Rust
//! handler structs can keep `pub sdate: i64` and still take the wire shape
//! the existing frontend ships.
//!
//! Empty / null / missing → `0` (the same sentinel PHPYun uses for "not set").
//!
//! All parsing is in **UTC** — same convention as the rest of the codebase
//! (`crate::clock::now_ts`, `chrono::Utc`). PHPYun's PHP runs in server-local
//! TZ; for the demo install both happen to be UTC+8, which means we drift
//! by 8h on the boundary day. Acceptable for resume dates (granularity is
//! month) and matches what the PHP read path's `date('Y-m', $ts)` produces.

use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::de::{self, Deserialize, Deserializer};
use serde_json::Value;

/// Parse a PHPYun-shaped date string into a Unix timestamp (UTC, midnight).
/// Returns `None` if the string doesn't match any known format.
///
/// Recognised forms (most-common first):
///   - `"YYYY-MM"`              → first of month, 00:00 UTC
///   - `"YYYY-MM-DD"`           → 00:00 UTC
///   - `"YYYY-MM-DD HH:MM:SS"`  → that exact moment, UTC
///   - `"YYYY/MM/DD"`           → 00:00 UTC
pub fn parse_loose_date(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // YYYY-MM (the most common form for resume forms).
    if let Some((y, m)) = s.split_once('-') {
        if y.len() == 4 && m.len() == 2 && !m.contains('-') {
            if let (Ok(year), Ok(month)) = (y.parse::<i32>(), m.parse::<u32>()) {
                if let Some(d) = NaiveDate::from_ymd_opt(year, month, 1) {
                    return Some(
                        Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                            .timestamp(),
                    );
                }
            }
        }
    }

    // YYYY-MM-DD HH:MM:SS
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(Utc.from_utc_datetime(&dt).timestamp());
    }
    // ISO 8601 with T separator
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(Utc.from_utc_datetime(&dt).timestamp());
    }
    // YYYY-MM-DD
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(
            Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                .timestamp(),
        );
    }
    // YYYY/MM/DD
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y/%m/%d") {
        return Some(
            Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                .timestamp(),
        );
    }

    None
}

/// Coerce one JSON value into a Unix timestamp.
/// Returns `0` for `null` / empty string / unrecognised — matches PHPYun's
/// "not set" sentinel.
fn coerce_value_to_ts<E: de::Error>(v: Value) -> Result<i64, E> {
    match v {
        Value::Null => Ok(0),
        Value::Number(n) => Ok(n.as_i64().unwrap_or(0)),
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                return Ok(0);
            }
            // Numeric-string Unix timestamp wins over date parsing — frontends
            // sometimes stringify the int they got from a previous read.
            if let Ok(n) = t.parse::<i64>() {
                return Ok(n);
            }
            parse_loose_date(t).ok_or_else(|| {
                E::custom(format!(
                    "expected unix-ts / YYYY-MM / YYYY-MM-DD / YYYY-MM-DD HH:MM:SS, got {t:?}"
                ))
            })
        }
        other => Err(E::custom(format!(
            "expected number or string, got {other:?}"
        ))),
    }
}

/// serde `deserialize_with` adapter for `i64` fields that may arrive as a
/// number, a numeric string, an empty string, or any of the PHPYun-shaped
/// date strings.
pub fn de_loose_ts<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    let v = Value::deserialize(d)?;
    coerce_value_to_ts(v)
}

/// `Option<i64>` variant — `null` / missing field becomes `None`, an empty
/// string also becomes `None`, otherwise the same coercion as `de_loose_ts`.
pub fn de_loose_ts_opt<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    let v = Value::deserialize(d)?;
    if v.is_null() {
        return Ok(None);
    }
    if let Value::String(s) = &v {
        if s.trim().is_empty() {
            return Ok(None);
        }
    }
    coerce_value_to_ts(v).map(Some)
}

// ==================== Loose integer deserializers ====================
//
// PHPYun's PHP frontend posts every numeric form field as a string
// (`"17"` / `"57"` / `"0"`); the Rust handlers ask for native `i32` / `i64`,
// which serde refuses on a string. The loose deserializers below accept
// both shapes plus empty/null, defaulting to 0 for the "not set" case.

fn coerce_value_to_i64<E: de::Error>(v: Value) -> Result<i64, E> {
    match v {
        Value::Null => Ok(0),
        Value::Number(n) => Ok(n.as_i64().unwrap_or(0)),
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                Ok(0)
            } else {
                t.parse::<i64>().map_err(E::custom)
            }
        }
        other => Err(E::custom(format!("expected number or string, got {other:?}"))),
    }
}

/// `i32` field that may arrive as number, numeric string, empty, or null.
pub fn de_loose_i32<'de, D: Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    let v = Value::deserialize(d)?;
    let n = coerce_value_to_i64(v)?;
    Ok(n.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
}

/// `i64` field that may arrive as number, numeric string, empty, or null.
pub fn de_loose_i64<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    let v = Value::deserialize(d)?;
    coerce_value_to_i64(v)
}

/// `u8` field — PHPYun sends `usertype` / `regway` / etc. as strings.
/// Negative values clamp to 0; > 255 clamps to 255 (caller should range-validate).
pub fn de_loose_u8<'de, D: Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let v = Value::deserialize(d)?;
    let n = coerce_value_to_i64(v)?;
    Ok(n.clamp(0, u8::MAX as i64) as u8)
}

/// `u32` field — same loose semantics as `de_loose_i32`.
pub fn de_loose_u32<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    let v = Value::deserialize(d)?;
    let n = coerce_value_to_i64(v)?;
    Ok(n.clamp(0, u32::MAX as i64) as u32)
}

/// `u64` field — same loose semantics as `de_loose_i64`.
pub fn de_loose_u64<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    let v = Value::deserialize(d)?;
    let n = coerce_value_to_i64(v)?;
    Ok(n.max(0) as u64)
}

/// `Option<i32>` — `null` / missing / empty-string become `None`; otherwise
/// same coercion as [`de_loose_i32`].
pub fn de_loose_i32_opt<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    let v = Value::deserialize(d)?;
    if v.is_null() {
        return Ok(None);
    }
    if let Value::String(s) = &v {
        if s.trim().is_empty() {
            return Ok(None);
        }
    }
    let n = coerce_value_to_i64(v)?;
    Ok(Some(n.clamp(i32::MIN as i64, i32::MAX as i64) as i32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct T {
        #[serde(deserialize_with = "de_loose_ts")]
        sdate: i64,
        #[serde(default, deserialize_with = "de_loose_ts_opt")]
        edate: Option<i64>,
    }

    #[test]
    fn parses_year_month() {
        // 2020-01 first of month UTC = 1577836800
        assert_eq!(parse_loose_date("2020-01"), Some(1577836800));
        // 2026-03-01 00:00:00 UTC
        assert_eq!(parse_loose_date("2026-03"), Some(1772323200));
    }

    #[test]
    fn parses_full_date() {
        assert_eq!(parse_loose_date("2020-01-01"), Some(1577836800));
        assert_eq!(parse_loose_date("2020/01/01"), Some(1577836800));
        assert_eq!(
            parse_loose_date("2020-01-01 12:30:45"),
            Some(1577881845)
        );
    }

    #[test]
    fn empty_returns_none_then_zero() {
        assert_eq!(parse_loose_date(""), None);
        assert_eq!(parse_loose_date("   "), None);
    }

    #[test]
    fn invalid_returns_none() {
        assert_eq!(parse_loose_date("not a date"), None);
        assert_eq!(parse_loose_date("2020-13"), None);
    }

    #[test]
    fn de_accepts_int_string_yyyymm_empty_null() {
        let cases = vec![
            (r#"{"sdate": 1577836800}"#, 1577836800, None),
            (r#"{"sdate": "1577836800"}"#, 1577836800, None),
            (r#"{"sdate": "2020-01"}"#, 1577836800, None),
            (r#"{"sdate": "2020-01-01"}"#, 1577836800, None),
            (r#"{"sdate": "", "edate": ""}"#, 0, None),
            (r#"{"sdate": null, "edate": null}"#, 0, None),
            (
                r#"{"sdate": "2020-01", "edate": "2026-03"}"#,
                1577836800,
                Some(1772323200),
            ),
        ];
        for (json, want_s, want_e) in cases {
            let got: T = serde_json::from_str(json).expect(json);
            assert_eq!(got.sdate, want_s, "input: {json}");
            assert_eq!(got.edate, want_e, "input: {json}");
        }
    }
}
