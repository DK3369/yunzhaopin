//! Marketing / special signup / weixin records / OSS / gsd / fastlogin / dataCall.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::repo as gap;
use phpyun_models::site_setting::repo as setting_repo;
use serde::Serialize;
use std::collections::HashMap;

use crate::mail_service;

async fn audit_write(state: &AppState, actor: &AuthenticatedUser, action: &'static str, target: String) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

pub async fn marketing_email_status(state: &AppState) -> AppResult<Vec<LastMsgAt>> {
    Ok(gap::last_email_msgs(state.db.reader(), 12).await?)
}

pub async fn marketing_sms_status(state: &AppState) -> AppResult<Vec<LastMsgAt>> {
    Ok(gap::last_sms_msgs(state.db.reader(), 12).await?)
}

pub async fn marketing_email_send(
    state: &AppState,
    actor: &AuthenticatedUser,
    emails: &[String],
    title: &str,
    content: &str,
) -> AppResult<u64> {
    marketing_email_send_typed(state, actor, emails, title, content, 0).await
}

pub async fn marketing_email_send_typed(
    state: &AppState,
    actor: &AuthenticatedUser,
    emails: &[String],
    title: &str,
    content: &str,
    utype: i32,
) -> AppResult<u64> {
    if title.trim().is_empty() || content.trim().is_empty() {
        return Err(ApiError::param_invalid("title_content_emails"));
    }
    let mut list: Vec<String> = emails
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if list.is_empty() && (1..=4).contains(&utype) {
        list = gap::list_member_emails(state.db.reader(), utype, 200).await?;
    }
    if list.is_empty() {
        return Err(ApiError::param_invalid("title_content_emails"));
    }
    let now = clock::now_ts();
    let mut n = 0u64;
    for email in list.iter().take(200) {
        let email = email.trim();
        if email.is_empty() {
            continue;
        }
        let state_flag = if mail_service::send_text(state, email, title, content)
            .await
            .is_ok()
        {
            1
        } else {
            0
        };
        gap::insert_email_log(state.db.pool(), actor.uid, email, title, content, now, state_flag)
            .await?;
        n += 1;
    }
    audit_write(state, actor, "admin.marketing.email", format!("n:{n}")).await;
    Ok(n)
}

pub async fn marketing_sms_send(
    state: &AppState,
    actor: &AuthenticatedUser,
    mobiles: &[String],
    content: &str,
) -> AppResult<u64> {
    marketing_sms_send_typed(state, actor, mobiles, content, 0).await
}

pub async fn marketing_sms_send_typed(
    state: &AppState,
    actor: &AuthenticatedUser,
    mobiles: &[String],
    content: &str,
    utype: i32,
) -> AppResult<u64> {
    if content.trim().is_empty() {
        return Err(ApiError::param_invalid("content_mobiles"));
    }
    let mut list: Vec<String> = mobiles
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if list.is_empty() && (1..=4).contains(&utype) {
        list = gap::list_member_mobiles(state.db.reader(), utype, 200).await?;
    }
    if list.is_empty() {
        return Err(ApiError::param_invalid("content_mobiles"));
    }
    let now = clock::now_ts();
    let mut n = 0u64;
    for m in list.iter().take(200) {
        let m = m.trim();
        if m.is_empty() {
            continue;
        }
        gap::insert_sms_log(state.db.pool(), actor.uid, m, content, now, 0).await?;
        n += 1;
    }
    audit_write(state, actor, "admin.marketing.sms", format!("n:{n}")).await;
    Ok(n)
}

pub async fn list_special_coms(
    state: &AppState,
    sid: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<SpecialComAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_special_coms(db, sid, page.offset, page.limit).await?;
    let total = gap::count_special_coms(db, sid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_special_com_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    let n = gap::set_special_com_status(state.db.pool(), id, status, statusbody).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("special_com_not_found"));
    }
    audit_write(state, actor, "admin.special.com", format!("id:{id}")).await;
    Ok(())
}

pub async fn list_wx_records(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<WxQrcodeRow>> {
    let db = state.db.reader();
    let list = gap::list_wxqrcodes(db, status, keyword, page.offset, page.limit).await?;
    let total = gap::count_wxqrcodes(db, status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_wxpub_temps(
    state: &AppState,
    keyword: Option<&str>,
    temptype: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<WxpubTempRow>> {
    let db = state.db.reader();
    let list = gap::list_wxpub_temps(db, keyword, temptype, page.offset, page.limit).await?;
    let total = gap::count_wxpub_temps(db, keyword, temptype).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_wxpub_temp(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    title: &str,
    header: &str,
    body: &str,
    footer: &str,
    r#type: &str,
    temptype: i32,
) -> AppResult<u64> {
    if title.trim().is_empty() {
        return Err(ApiError::param_invalid("title"));
    }
    let nid = gap::upsert_wxpub_temp(
        state.db.pool(),
        id,
        title,
        header,
        body,
        footer,
        r#type,
        temptype,
        clock::now_ts(),
    )
    .await?;
    audit_write(state, actor, "admin.wxpub", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_wxpub_temps(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_wxpub_temps(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.wxpub.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_data_call(state: &AppState, page: Pagination) -> AppResult<Paged<OutsideRow>> {
    let db = state.db.reader();
    let list = gap::list_outside(db, page.offset, page.limit).await?;
    let total = gap::count_outside(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_data_call(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    r#type: &str,
    titlelen: i32,
    infolen: i32,
    num: i32,
    code: &str,
) -> AppResult<u64> {
    if name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let nid = gap::upsert_outside(
        state.db.pool(),
        id,
        name,
        r#type,
        titlelen,
        infolen,
        num,
        code,
        clock::now_ts(),
    )
    .await?;
    audit_write(state, actor, "admin.datacall", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_data_call(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_outside(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.datacall.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_hr_logs(
    state: &AppState,
    uid: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<HrLogRow>> {
    let db = state.db.reader();
    let list = gap::list_hr_logs(db, uid, page.offset, page.limit).await?;
    let total = gap::count_hr_logs(db, uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

async fn kv_map(state: &AppState, keys: &[&str]) -> AppResult<HashMap<String, String>> {
    let mut out = HashMap::new();
    for k in keys {
        let v = setting_repo::find(state.db.reader(), k)
            .await?
            .map(|s| s.value)
            .unwrap_or_default();
        out.insert((*k).to_string(), v);
    }
    Ok(out)
}

async fn kv_save(
    state: &AppState,
    actor: &AuthenticatedUser,
    action: &'static str,
    items: &[(String, String)],
    allow: &[&str],
) -> AppResult<()> {
    let now = clock::now_ts();
    for (k, v) in items {
        if !allow.contains(&k.as_str()) {
            continue;
        }
        setting_repo::upsert(state.db.pool(), k, v, "", true, now).await?;
    }
    audit_write(state, actor, action, format!("n:{}", items.len())).await;
    Ok(())
}

const GSD_KEYS: &[&str] = &[
    "sy_ip",
    "sy_ip_appkey",
    "sy_ip_appsecret",
    "sy_mobile",
    "sy_mobile_appkey",
    "sy_mobile_appsecret",
];

pub async fn gsd_config(state: &AppState) -> AppResult<HashMap<String, String>> {
    kv_map(state, GSD_KEYS).await
}

pub async fn save_gsd(
    state: &AppState,
    actor: &AuthenticatedUser,
    items: &[(String, String)],
) -> AppResult<()> {
    kv_save(state, actor, "admin.gsd", items, GSD_KEYS).await
}

const OSS_KEYS: &[&str] = &[
    "sy_oss",
    "sy_ossurl",
    "oss_access_id",
    "oss_access_key",
    "oss_endpoint",
    "oss_bucket",
    "oss_userdomain",
];

pub async fn oss_config(state: &AppState) -> AppResult<HashMap<String, String>> {
    kv_map(state, OSS_KEYS).await
}

pub async fn save_oss(
    state: &AppState,
    actor: &AuthenticatedUser,
    items: &[(String, String)],
) -> AppResult<()> {
    kv_save(state, actor, "admin.oss", items, OSS_KEYS).await
}

const FASTLOGIN_KEYS: &[&str] = &[
    "sy_qqlogin",
    "sy_qqappid",
    "sy_qqappkey",
    "sy_qqdt",
    "sy_sinalogin",
    "sy_sinaappid",
    "sy_sinaappkey",
];

pub async fn fastlogin_config(state: &AppState) -> AppResult<HashMap<String, String>> {
    kv_map(state, FASTLOGIN_KEYS).await
}

pub async fn save_fastlogin(
    state: &AppState,
    actor: &AuthenticatedUser,
    items: &[(String, String)],
) -> AppResult<()> {
    kv_save(state, actor, "admin.fastlogin", items, FASTLOGIN_KEYS).await
}

#[derive(Debug, Serialize)]
pub struct KvMap(pub HashMap<String, String>);
