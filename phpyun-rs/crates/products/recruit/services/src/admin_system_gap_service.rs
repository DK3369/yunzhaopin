//! PHP system gap: keywords, domains, cron table, errorlog, sysmsg, navmap, myuser, tpl.

use phpyun_auth::md5_hex;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::repo as gap;
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::site_setting::repo as setting_repo;
use serde::Serialize;

async fn audit_write(state: &AppState, actor: &AuthenticatedUser, action: &'static str, target: String) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

pub async fn list_keywords(
    state: &AppState,
    r#type: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<HotKeyAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_hot_keys(db, r#type, keyword, page.offset, page.limit).await?;
    let total = gap::count_hot_keys(db, r#type, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_keyword(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    key_name: &str,
    r#type: i32,
    check: i32,
    bold: i32,
    tuijian: i32,
    color: &str,
    size: &str,
) -> AppResult<u64> {
    if key_name.trim().is_empty() {
        return Err(ApiError::param_invalid("key_name"));
    }
    let nid = gap::upsert_hot_key(
        state.db.pool(),
        id,
        key_name,
        r#type,
        check,
        bold,
        tuijian,
        color,
        size,
    )
    .await?;
    audit_write(state, actor, "admin.keyword", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_keywords(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_hot_keys(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.keyword.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_domains(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<DomainAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_domains(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_domains(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_domain(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    title: &str,
    domain: &str,
    fz_type: i32,
    mode: i32,
    web_title: &str,
    indexdir: &str,
) -> AppResult<u64> {
    if title.trim().is_empty() || domain.trim().is_empty() {
        return Err(ApiError::param_invalid("title_domain"));
    }
    let nid = gap::upsert_domain(
        state.db.pool(),
        id,
        title,
        domain,
        fz_type,
        mode,
        web_title,
        indexdir,
    )
    .await?;
    audit_write(state, actor, "admin.domain", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_domains(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_domains(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.domain.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_domain_admins(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<DomainAdminUserRow>> {
    let db = state.db.reader();
    let list = gap::list_domain_admins(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_domain_admins(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_cron_table(state: &AppState, page: Pagination) -> AppResult<Paged<CronRow>> {
    let db = state.db.reader();
    let list = gap::list_cron(db, page.offset, page.limit).await?;
    let total = gap::count_cron(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_cron(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    dir: &str,
    r#type: i32,
    week: i32,
    month: i32,
    hour: i32,
    minute: i32,
    display: i32,
) -> AppResult<u64> {
    if name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let nid = gap::upsert_cron(
        state.db.pool(),
        id,
        name,
        dir,
        r#type,
        week,
        month,
        hour,
        minute,
        display,
        clock::now_ts(),
    )
    .await?;
    audit_write(state, actor, "admin.cron", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_cron(state: &AppState, actor: &AuthenticatedUser, ids: &[u64]) -> AppResult<()> {
    gap::delete_cron(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.cron.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_error_logs(
    state: &AppState,
    keyword: Option<&str>,
    logtype: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<ErrorLogRow>> {
    let db = state.db.reader();
    let list = gap::list_error_logs(db, keyword, logtype, page.offset, page.limit).await?;
    let total = gap::count_error_logs(db, keyword, logtype).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn delete_error_logs(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    all: bool,
) -> AppResult<()> {
    if all {
        gap::delete_error_logs(state.db.pool(), &[]).await?;
    } else {
        gap::delete_error_logs(state.db.pool(), ids).await?;
    }
    audit_write(state, actor, "admin.errorlog.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_sysmsgs(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<SysmsgAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_sysmsgs(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_sysmsgs(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn send_sysmsg(
    state: &AppState,
    actor: &AuthenticatedUser,
    utype: i32,
    content: &str,
    usernames: &[String],
) -> AppResult<u64> {
    if content.trim().is_empty() {
        return Err(ApiError::param_invalid("content"));
    }
    let now = clock::now_ts();
    let pool = state.db.pool();
    let mut n = 0u64;
    if utype == 5 {
        let rows = gap::find_members_by_usernames(pool, usernames).await?;
        for (uid, ut, _) in rows {
            gap::insert_sysmsg(pool, uid, ut, content, now).await?;
            n += 1;
        }
    } else {
        let uids = gap::list_member_uids_by_usertype(pool, utype, 0, 1000).await?;
        for uid in uids {
            gap::insert_sysmsg(pool, uid, utype, content, now).await?;
            n += 1;
        }
    }
    audit_write(state, actor, "admin.sysmsg.send", format!("n:{n}")).await;
    Ok(n)
}

pub async fn list_navmap(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<NavmapRow>> {
    let db = state.db.reader();
    let list = gap::list_navmap(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_navmap(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_navmap(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    nid: i32,
    name: &str,
    url: &str,
    sort: i32,
    display: i32,
    eject: i32,
    r#type: i32,
    furl: &str,
) -> AppResult<u64> {
    if name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let nid_out = gap::upsert_navmap(
        state.db.pool(),
        id,
        nid,
        name,
        url,
        sort,
        display,
        eject,
        r#type,
        furl,
    )
    .await?;
    audit_write(state, actor, "admin.navmap", format!("id:{nid_out}")).await;
    Ok(nid_out)
}

pub async fn delete_navmap(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_navmap(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.navmap.delete", format!("{ids:?}")).await;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct MyUserView {
    pub username: String,
    pub real_name: String,
    pub mobile: String,
    pub wxid: String,
    pub last_login: i64,
    pub group_name: String,
}

pub async fn my_user(state: &AppState, actor: &AuthenticatedUser) -> AppResult<MyUserView> {
    let row = rbac_repo::find_profile(state.db.reader(), actor.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_not_found"))?;
    let group_name = rbac_repo::group_name(state.db.reader(), row.5).await?;
    Ok(MyUserView {
        username: row.0,
        real_name: row.1,
        mobile: row.2,
        wxid: row.3,
        last_login: row.4,
        group_name,
    })
}

pub async fn save_password(
    state: &AppState,
    actor: &AuthenticatedUser,
    old_pwd: &str,
    new_pwd: &str,
    re_pwd: &str,
) -> AppResult<()> {
    if new_pwd.is_empty() || new_pwd != re_pwd {
        return Err(ApiError::param_invalid("password_mismatch"));
    }
    let user = rbac_repo::find_by_uid(state.db.reader(), actor.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_not_found"))?;
    let hashed = md5_hex(&md5_hex(old_pwd));
    if !hashed.eq_ignore_ascii_case(&user.password) {
        return Err(ApiError::param_invalid("old_password"));
    }
    let next = md5_hex(&md5_hex(new_pwd));
    rbac_repo::update_password(state.db.pool(), actor.uid, &next).await?;
    audit_write(state, actor, "admin.me.password", format!("uid:{}", actor.uid)).await;
    Ok(())
}

pub async fn update_my_profile(
    state: &AppState,
    actor: &AuthenticatedUser,
    name: Option<&str>,
    mobile: Option<&str>,
) -> AppResult<()> {
    rbac_repo::update_profile(state.db.pool(), actor.uid, name, mobile).await?;
    audit_write(state, actor, "admin.me.profile", format!("uid:{}", actor.uid)).await;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct ComTplRow {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub status: i32,
}

pub async fn list_comtpl(state: &AppState) -> AppResult<Vec<ComTplRow>> {
    let rows = gap::list_company_tpls(state.db.reader()).await?;
    Ok(rows
        .into_iter()
        .map(|(id, name, url, status)| ComTplRow {
            id,
            name,
            url,
            status,
        })
        .collect())
}

pub async fn current_style(state: &AppState) -> AppResult<String> {
    Ok(setting_repo::find(state.db.reader(), "style")
        .await?
        .map(|s| s.value)
        .unwrap_or_default())
}

pub async fn set_style(state: &AppState, actor: &AuthenticatedUser, dir: &str) -> AppResult<()> {
    if dir.trim().is_empty() {
        return Err(ApiError::param_invalid("dir"));
    }
    setting_repo::upsert(state.db.pool(), "style", dir, "", true, clock::now_ts()).await?;
    audit_write(state, actor, "admin.tpl.style", dir.to_string()).await;
    Ok(())
}

const MODULE_KEYS: &[&str] = &[
    "ask", "article", "once", "tiny", "part", "zph", "special", "redeem", "evaluate",
];

#[derive(Debug, Serialize)]
pub struct ModuleRow {
    pub key: String,
    pub web: String,
    pub ssl: String,
    pub domain: String,
    pub dir: String,
}

pub async fn list_modules(state: &AppState) -> AppResult<Vec<ModuleRow>> {
    let all = setting_repo::list_all(state.db.reader()).await?;
    let mut map = std::collections::HashMap::new();
    for s in all {
        map.insert(s.key_name, s.value);
    }
    Ok(MODULE_KEYS
        .iter()
        .map(|k| ModuleRow {
            key: (*k).to_string(),
            web: map
                .get(&format!("sy_{k}_web"))
                .cloned()
                .unwrap_or_default(),
            ssl: map.get(&format!("sy_{k}ssl")).cloned().unwrap_or_default(),
            domain: map
                .get(&format!("sy_{k}domain"))
                .cloned()
                .unwrap_or_default(),
            dir: map.get(&format!("sy_{k}dir")).cloned().unwrap_or_default(),
        })
        .collect())
}

pub async fn save_modules(
    state: &AppState,
    actor: &AuthenticatedUser,
    items: &[(String, String, String, String, String)],
) -> AppResult<()> {
    let now = clock::now_ts();
    let pool = state.db.pool();
    for (key, web, ssl, domain, dir) in items {
        if !MODULE_KEYS.contains(&key.as_str()) {
            continue;
        }
        setting_repo::upsert(pool, &format!("sy_{key}_web"), web, "", true, now).await?;
        setting_repo::upsert(pool, &format!("sy_{key}ssl"), ssl, "", true, now).await?;
        setting_repo::upsert(pool, &format!("sy_{key}domain"), domain, "", true, now).await?;
        setting_repo::upsert(pool, &format!("sy_{key}dir"), dir, "", true, now).await?;
    }
    audit_write(state, actor, "admin.modules", format!("n:{}", items.len())).await;
    Ok(())
}
