//! Password hashing and verification.
//!
//! Concurrency strategy:
//! - **Single hash/verify** uses `tokio::task::spawn_blocking`. argon2 is CPU-intensive (60~300ms);
//!   running it directly on the async runtime would block worker threads and drag down other requests.
//! - **Bulk migration** (one-shot upgrade of legacy md5 database) uses `rayon::par_iter`,
//!   parallelizing across all physical cores for ~10x the single-threaded throughput.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use md5::{Digest, Md5};
use rayon::prelude::*;

// ========== Low-level synchronous implementation ==========

/// Hardened Argon2id parameters (OWASP 2024 recommended minimums):
/// - 64 MiB memory (m = 65536 KiB)
/// - 3 iterations
/// - parallelism of 4
///
/// During verification, Argon2 reads params from the stored hash automatically, so old hashes
/// remain verifiable; this only affects **newly generated** hashes.
fn hasher() -> Argon2<'static> {
    let params = Params::new(64 * 1024, 3, 4, None).expect("argon2 params hardcoded; never fails");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

fn argon2_hash_sync(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    hasher()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("argon2 hash failed: {e}"))
}

pub fn argon2_hash(password: &str) -> anyhow::Result<String> {
    argon2_hash_sync(password)
}

/// Compatibility verification: new argon2 format or legacy md5(md5(pw) + salt)
pub fn verify_password(password: &str, stored_hash: &str, salt: &str) -> bool {
    if stored_hash.starts_with("$argon2") {
        match PasswordHash::new(stored_hash) {
            // verify_password parses the original params from PasswordHash, so using the default
            // instance is fine; it's independent of the params used at generation time, and old
            // hashes still verify correctly.
            Ok(parsed) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
            Err(_) => false,
        }
    } else {
        let step1 = format!("{:x}", Md5::digest(password.as_bytes()));
        let combined = format!("{}{}", step1, salt);
        let step2 = format!("{:x}", Md5::digest(combined.as_bytes()));
        step2.eq_ignore_ascii_case(stored_hash)
    }
}

// ========== Async wrapper (used on HTTP request paths) ==========

/// Async argon2 hash. Runs on the blocking pool so it doesn't block tokio workers.
pub async fn argon2_hash_async(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || argon2_hash_sync(&password))
        .await
        .map_err(|e| anyhow::anyhow!("spawn_blocking failed: {e}"))?
}

/// Async password verification. argon2 verification is still CPU-intensive (30~50ms even without hashing).
pub async fn verify_password_async(password: String, hash: String, salt: String) -> bool {
    tokio::task::spawn_blocking(move || verify_password(&password, &hash, &salt))
        .await
        .unwrap_or(false)
}

/// Plain md5(ascii-hex) — only for compatibility with niche PHPYun tables that store the password as
/// `md5(plaintext)` (e.g. `resume_tiny`). Do not use in new features.
pub fn md5_hex(input: &str) -> String {
    format!("{:x}", Md5::digest(input.as_bytes()))
}

// ========== Rayon batch processing (data migration scripts) ==========

/// Bulk-upgrade legacy md5 passwords to argon2. Uses rayon for parallelism, achieving roughly
/// N-times single-threaded throughput on an N-core machine.
///
/// Typical usage:
/// - Input: `Vec<(uid, plaintext_pw)>` — requires an extra side-channel entry point (e.g. collected during user login)
/// - Output: `Vec<(uid, new_argon2_hash)>` — then bulk UPDATE
///
/// **Do not** feed legacy md5 hashes directly as input — argon2 needs plaintext, and we only have
/// the plaintext at the moment a login verification succeeds.
pub fn batch_hash_passwords(plaintexts: Vec<(u64, String)>) -> Vec<(u64, anyhow::Result<String>)> {
    plaintexts
        .into_par_iter()
        .map(|(uid, pw)| (uid, argon2_hash_sync(&pw)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_md5_verify() {
        let expected = format!(
            "{:x}",
            md5::Md5::digest(b"5d41402abc4b2a76b9719d911017c592abc123")
        );
        assert!(verify_password("hello", &expected, "abc123"));
        assert!(!verify_password("wrong", &expected, "abc123"));
    }

    #[test]
    fn argon2_roundtrip() {
        let h = argon2_hash("s3cr3t").unwrap();
        assert!(verify_password("s3cr3t", &h, "ignored_for_argon2"));
        assert!(!verify_password("nope", &h, ""));
    }

    #[test]
    fn batch_rayon_works() {
        let input: Vec<(u64, String)> = (0..16).map(|i| (i, format!("pw-{i}"))).collect();
        let out = batch_hash_passwords(input);
        assert_eq!(out.len(), 16);
        for (_uid, r) in &out {
            assert!(r.is_ok());
        }
    }

    #[tokio::test]
    async fn async_hash_verify_no_block() {
        let h = argon2_hash_async("hello".into()).await.unwrap();
        assert!(verify_password_async("hello".into(), h.clone(), String::new()).await);
        assert!(!verify_password_async("nope".into(), h, String::new()).await);
    }
}
