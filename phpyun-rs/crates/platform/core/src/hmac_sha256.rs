//! HMAC-SHA256 helpers (Stripe webhook signatures, similar gateways).
//!
//! Services must not take a `hmac` / `sha2` dependency; call this facade.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let hi = b >> 4;
        let lo = b & 0x0f;
        out.push(hex_digit(hi));
        out.push(hex_digit(lo));
    }
    out
}

fn hex_digit(n: u8) -> char {
    if n < 10 {
        char::from(b'0' + n)
    } else {
        char::from(b'a' + (n - 10))
    }
}

pub fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = hex_val(bytes[i])?;
        let lo = hex_val(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).unwrap_or_else(|_| HmacSha256::new_from_slice(&[0]).expect("hmac"));
    mac.update(msg);
    mac.finalize().into_bytes().to_vec()
}

pub fn hmac_sha256_verify(key: &[u8], msg: &[u8], expected_hex: &str) -> bool {
    let Some(expected) = hex_decode(expected_hex) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(key) else {
        return false;
    };
    mac.update(msg);
    mac.verify_slice(&expected).is_ok()
}

/// First `t=` unix timestamp in a Stripe-Signature header, if present.
pub fn stripe_signature_timestamp(header: &str) -> Option<i64> {
    for part in header.split(',') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("t=") {
            return rest.parse().ok();
        }
    }
    None
}

/// Stripe-Signature: `t=<unix>,v1=<hex>[,v1=...]`.
/// Signed payload is `{t}.{raw_body}`. Any matching `v1` is accepted (key rotation).
pub fn verify_stripe_signature(
    secret: &str,
    body: &[u8],
    header: &str,
    now_unix: i64,
    tolerance_secs: i64,
) -> bool {
    if secret.is_empty() || header.is_empty() {
        return false;
    }
    let mut timestamp: Option<i64> = None;
    let mut v1: Vec<&str> = Vec::new();
    for part in header.split(',') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("t=") {
            timestamp = rest.parse().ok();
        } else if let Some(rest) = part.strip_prefix("v1=") {
            v1.push(rest);
        }
    }
    let Some(t) = timestamp else {
        return false;
    };
    if v1.is_empty() {
        return false;
    }
    let delta = now_unix.abs_diff(t);
    let tol = if tolerance_secs < 0 {
        0u64
    } else {
        u64::try_from(tolerance_secs).unwrap_or(0)
    };
    if delta > tol {
        return false;
    }
    let mut signed = t.to_string().into_bytes();
    signed.push(b'.');
    signed.extend_from_slice(body);
    v1.iter()
        .any(|sig| hmac_sha256_verify(secret.as_bytes(), &signed, sig))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmac_rfc4231_case1() {
        let mac = hmac_sha256(b"key", b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            hex_encode(&mac),
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        );
        assert!(hmac_sha256_verify(
            b"key",
            b"The quick brown fox jumps over the lazy dog",
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        ));
        assert!(!hmac_sha256_verify(b"key", b"nope", "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"));
    }

    #[test]
    fn stripe_sig_roundtrip() {
        let secret = "whsec_test";
        let body = br#"{"id":"evt_1"}"#;
        let t = 1_700_000_000i64;
        let mut signed = t.to_string().into_bytes();
        signed.push(b'.');
        signed.extend_from_slice(body);
        let sig = hex_encode(&hmac_sha256(secret.as_bytes(), &signed));
        let header = format!("t={t},v1={sig}");
        assert!(verify_stripe_signature(secret, body, &header, t, 300));
        assert!(!verify_stripe_signature(secret, body, &header, t + 400, 300));
        assert!(!verify_stripe_signature("other", body, &header, t, 300));
        assert_eq!(stripe_signature_timestamp(&header), Some(t));
        assert_eq!(
            stripe_signature_timestamp(&format!("v1={sig}, t={t}")),
            Some(t)
        );
    }
}
