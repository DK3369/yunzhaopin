//! Image captcha business — issue + verify. **This iteration: real PNG rendering + base64.**
//!
//! - Server generates a 4-digit code and stores it in Redis
//! - Renders a PNG with interference lines/noise via the `captcha` crate (secure, OCR-resistant)
//! - The response returns `{cid, image}`, where `image` is a data URI of the form
//!   `data:image/png;base64,...` — the client can render it directly with `<img src={image} />`
//! - **No longer exposes the plaintext code** (the previous `code_dev_only` was just a placeholder)
//!
//! Verification flows through `core::verify::verify(ImageCaptcha, cid, input)`; after 5
//! failures the key is invalidated automatically.

use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{metrics as m, AppError, AppResult, AppState};
use std::time::Duration;
use uuid::Uuid;

pub struct IssuedCaptcha {
    pub cid: String,
    /// data URI of the form `data:image/png;base64,iVBOR...`
    pub image: String,
}

/// Issue a fresh image captcha.
/// The `captcha` crate randomly picks the characters itself, so the flow is:
/// **render first, obtain the code, then store it in Redis**.
pub async fn issue(state: &AppState) -> AppResult<IssuedCaptcha> {
    let cid = Uuid::now_v7().simple().to_string()[..16].to_string();

    let (code, png_bytes) = render_png()?;
    let image = format!(
        "data:image/png;base64,{}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_bytes)
    );

    verify::issue(
        &state.redis,
        VerifyKind::ImageCaptcha,
        &cid,
        &code,
        Duration::from_secs(300),
    )
    .await?;

    m::counter("captcha.issued");
    Ok(IssuedCaptcha { cid, image })
}

/// Render a 4-character PNG with noise/distortion, returning (the actually picked code,
/// PNG bytes). Verification is case-insensitive — the returned code is uppercased before
/// being stored.
fn render_png() -> AppResult<(String, Vec<u8>)> {
    use captcha::filters::{Noise, Wave};
    use captcha::Captcha;

    let mut c = Captcha::new();
    c.add_chars(4);
    let code = c.chars_as_string().to_uppercase();

    c.apply_filter(Noise::new(0.3))
        .apply_filter(Wave::new(2.0, 20.0))
        .view(160, 60);

    let png = c
        .as_png()
        .ok_or_else(|| AppError::internal(std::io::Error::other("captcha PNG render failed")))?;
    Ok((code, png))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_png_produces_bytes() {
        let (code, bytes) = render_png().expect("render ok");
        // PNG magic: 0x89 P N G
        assert_eq!(&bytes[..4], &[0x89, b'P', b'N', b'G']);
        // size should be > 0 and < some sane upper bound
        assert!(bytes.len() > 100);
        assert!(bytes.len() < 50_000);
        // code is 4 chars
        assert_eq!(code.chars().count(), 4);
        // Fully uppercase (we always upper-case)
        assert_eq!(code, code.to_uppercase());
    }

    #[test]
    fn render_png_randomness() {
        // Render multiple times: the codes must not all be identical
        let mut codes = std::collections::HashSet::new();
        for _ in 0..10 {
            let (code, _) = render_png().unwrap();
            codes.insert(code);
        }
        assert!(codes.len() > 1, "random codes should vary");
    }
}
