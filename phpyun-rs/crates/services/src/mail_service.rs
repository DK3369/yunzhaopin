//! Transactional email delivery through the host MTA (`sendmail`).

use phpyun_core::{error::InfraError, AppResult, AppState};
use std::io::Write;
use std::process::{Command, Stdio};

async fn setting(state: &AppState, key: &str) -> AppResult<Option<String>> {
    Ok(phpyun_models::site_setting::repo::find(state.db.reader(), key)
        .await?
        .map(|row| row.value.trim().to_string())
        .filter(|value| !value.is_empty()))
}

/// Submit a plain-text message to the host MTA and wait until it accepts it.
pub async fn send_text(state: &AppState, to: &str, subject: &str, body: &str) -> AppResult<()> {
    let from = setting(state, "sy_webemail")
        .await?
        .unwrap_or_else(|| "no-reply@localhost".to_string());
    let site_name = setting(state, "sy_webname")
        .await?
        .unwrap_or_else(|| "PHPYun".to_string());
    let clean = |value: &str| value.replace(['\r', '\n'], " ");
    let message = format!(
        "From: {} <{}>\nTo: {}\nSubject: {}\nMIME-Version: 1.0\nContent-Type: text/plain; charset=UTF-8\nContent-Transfer-Encoding: 8bit\n\n{}\n",
        clean(&site_name), clean(&from), clean(to), clean(subject), body
    );

    let result = tokio::task::spawn_blocking(move || -> std::io::Result<std::process::Output> {
        let mut child = Command::new("/usr/sbin/sendmail")
            .args(["-i", "-t"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        child.stdin.take().expect("sendmail stdin").write_all(message.as_bytes())?;
        child.wait_with_output()
    })
    .await
    .map_err(|e| InfraError::Upstream(format!("mail task failed: {e}")))?
    .map_err(|e| InfraError::Upstream(format!("sendmail unavailable: {e}")))?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr).trim().to_string();
        return Err(InfraError::Upstream(format!("mail delivery rejected: {stderr}")).into());
    }
    Ok(())
}
