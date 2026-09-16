//! Jobseeker identity-card cert. PHP `userinfo.model.php::upidcardInfo`.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::utils::mask_idcard;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::resume::expect as expect_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::site_setting::repo as setting_repo;

pub struct IdcardStatus {
    pub status: i32,
    pub statusbody: String,
    pub idcard: String,
    pub idcard_pic: String,
    pub name: String,
    pub cert_time: i64,
}

pub struct SubmitInput<'a> {
    pub idcard: &'a str,
    pub name: Option<&'a str>,
    pub idcard_pic: &'a str,
}

fn pic_ok(raw: &str) -> bool {
    let t = raw.trim();
    !t.is_empty()
        && t.len() <= 255
        && !t.contains("..")
        && !t.contains('\\')
        && !t.starts_with('/')
}

fn idcard_ok(raw: &str) -> bool {
    let t = raw.trim();
    let n = t.chars().count();
    (15..=18).contains(&n)
        && t.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == 'x' || c == 'X')
}

pub async fn status(state: &AppState, user: &AuthenticatedUser) -> AppResult<IdcardStatus> {
    user.require_jobseeker()?;
    let row = resume_repo::find_idcard(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let status = if row.idcard_pic.trim().is_empty() {
        -1
    } else {
        row.idcard_status
    };
    Ok(IdcardStatus {
        status,
        statusbody: row.statusbody,
        idcard: mask_idcard(&row.idcard),
        idcard_pic: row.idcard_pic,
        name: row.name,
        cert_time: row.cert_time,
    })
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    input: SubmitInput<'_>,
    client_ip: &str,
) -> AppResult<IdcardStatus> {
    user.require_jobseeker()?;
    let idcard = input.idcard.trim();
    let pic = input.idcard_pic.trim();
    if !idcard_ok(idcard) {
        return Err(ApiError::business("idcard_invalid"));
    }
    if !pic_ok(pic) {
        return Err(ApiError::business("idcard_pic_required"));
    }
    let row = resume_repo::find_idcard(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let name = input
        .name
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(row.name.as_str());
    if name.chars().count() > 25 {
        return Err(ApiError::param_invalid("name"));
    }
    let cfg = setting_repo::find(state.db.reader(), "user_idcard_status")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let mut st = if cfg.trim() == "1" { 0 } else { 1 };
    if row.r_status == 2 {
        st = 0;
    }
    let now = clock::now_ts();
    let n = resume_repo::submit_idcard(state.db.pool(), user.uid, name, idcard, pic, st, now).await?;
    if n == 0 {
        return Err(ApiError::business("not_found"));
    }
    let _ = expect_repo::set_idcard_status_for_uid(state.db.pool(), user.uid, st, name).await;
    let _ = audit::emit(
        state,
        AuditEvent::new("idcard.submit", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("resume:{uid}", uid = user.uid))
            .meta(&serde_json::json!({ "status": st })),
    )
    .await;
    status(state, user).await
}
