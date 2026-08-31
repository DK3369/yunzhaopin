//! PHP system gap: keywords, domains, cron table, errorlog, sysmsg, navmap, myuser, tpl.

use phpyun_auth::md5_hex;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::extra as gap2;
use phpyun_models::admin_gap::repo as gap;
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::site_setting::repo as setting_repo;
use serde::Serialize;
use serde_json::{json, Value};

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
    rec: Option<i32>,
    check: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<HotKeyAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_hot_keys(db, r#type, keyword, rec, check, page.offset, page.limit).await?;
    let total = gap::count_hot_keys(db, r#type, keyword, rec, check).await?;
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
    let mut list = gap::list_domains(db, keyword, page.offset, page.limit).await?;
    for r in &mut list {
        r.name = r.title.clone();
    }
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
    style: &str,
    hy: i32,
    cityid: i32,
    province: i32,
    tpl: &str,
) -> AppResult<u64> {
    if title.trim().is_empty() || (domain.trim().is_empty() && indexdir.trim().is_empty()) {
        return Err(ApiError::param_invalid("title_domain"));
    }
    let nid = gap2::upsert_domain_full(
        state.db.pool(),
        id,
        title,
        domain,
        fz_type,
        mode,
        web_title,
        indexdir,
        style,
        hy,
        cityid,
        province,
        tpl,
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
    let mut list = gap::list_cron(db, page.offset, page.limit).await?;
    for r in &mut list {
        r.nowtime_n = if r.nowtime > 0 {
            fmt_dt(r.nowtime)
        } else {
            "-".into()
        };
        r.nexttime_n = if r.nexttime > 0 {
            fmt_dt(r.nexttime)
        } else {
            "-".into()
        };
    }
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
    pub qy_wxid: String,
    pub last_login: String,
    pub group_name: String,
    pub qy_app_id: String,
    pub agent_id: String,
    pub redirect_uri: String,
    pub state: String,
}

pub async fn my_user(state: &AppState, actor: &AuthenticatedUser) -> AppResult<MyUserView> {
    let row = rbac_repo::find_profile(state.db.reader(), actor.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_not_found"))?;
    let group_name = rbac_repo::group_name(state.db.reader(), row.5).await?;
    let cfg = setting_repo::list_all(state.db.reader()).await?;
    let mut map = std::collections::HashMap::new();
    for s in cfg {
        map.insert(s.key_name, s.value);
    }
    let web = map.get("sy_weburl").cloned().unwrap_or_default();
    let qy_app_id = map.get("wx_qy_corpid").cloned().unwrap_or_default();
    let agent_id = map
        .get("wx_photo_agentId")
        .cloned()
        .filter(|s| !s.is_empty())
        .or_else(|| map.get("wx_qy_agentid").cloned())
        .unwrap_or_default();
    let redirect_uri = {
        let base = web.trim_end_matches('/');
        format!("{base}/admin/myaccount")
    };
    Ok(MyUserView {
        username: row.0,
        real_name: row.1,
        mobile: row.2,
        wxid: row.3,
        qy_wxid: String::new(),
        last_login: phpyun_core::utils::fmt_ts(row.4, "%Y-%m-%d %H:%M:%S"),
        group_name,
        qy_app_id,
        agent_id,
        redirect_uri,
        state: String::new(),
    })
}

pub async fn unbind_wx(state: &AppState, actor: &AuthenticatedUser) -> AppResult<()> {
    let row = rbac_repo::find_profile(state.db.reader(), actor.uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_not_found"))?;
    if row.3.is_empty() {
        return Err(ApiError::business("admin_system_00024"));
    }
    rbac_repo::clear_wxid(state.db.pool(), actor.uid).await?;
    audit_write(state, actor, "admin.me.unbind_wx", format!("uid:{}", actor.uid)).await;
    Ok(())
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

pub async fn domain_detail(state: &AppState, id: u64) -> AppResult<DomainAdminRow> {
    let mut row = gap2::find_domain(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("domain_not_found"))?;
    row.name = row.title.clone();
    Ok(row)
}

pub async fn domain_config(state: &AppState) -> AppResult<Value> {
    let keys = [
        "sy_web_site",
        "sy_gotocity",
        "sy_indexcity",
        "sy_indexdomain",
        "sy_onedomain",
    ];
    let mut out = serde_json::Map::new();
    for k in keys {
        let v = setting_repo::find(state.db.reader(), k)
            .await?
            .map(|s| s.value)
            .unwrap_or_default();
        out.insert(k.to_string(), json!(v));
    }
    Ok(Value::Object(out))
}

pub async fn upsert_domain_admin(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: Option<u64>,
    username: &str,
    name: &str,
    password: Option<&str>,
    m_id: i32,
    did: u64,
) -> AppResult<u64> {
    if username.trim().is_empty() {
        return Err(ApiError::param_invalid("username"));
    }
    let hashed = password
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|p| md5_hex(&md5_hex(p)));
    let nid = gap2::upsert_domain_admin(
        state.db.pool(),
        uid,
        username,
        name,
        hashed.as_deref(),
        m_id,
        did,
    )
    .await?;
    audit_write(state, actor, "admin.domain.admin", format!("uid:{nid}")).await;
    Ok(nid)
}

pub async fn delete_domain_admins(
    state: &AppState,
    actor: &AuthenticatedUser,
    uids: &[u64],
) -> AppResult<()> {
    gap2::delete_domain_admins(state.db.pool(), uids).await?;
    audit_write(state, actor, "admin.domain.admin.delete", format!("{uids:?}")).await;
    Ok(())
}

pub async fn recup_keyword(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    col: &str,
    rec: i32,
) -> AppResult<()> {
    let n = gap2::recup_hot_key(state.db.pool(), id, col, rec).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("keyword_flag"));
    }
    audit_write(state, actor, "admin.keyword.recup", format!("id:{id}")).await;
    Ok(())
}

pub async fn batch_keyword_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    pid: &str,
    check: i32,
    tuijian: i32,
    bold: i32,
    color: &str,
    size: &str,
    r#type: Option<i32>,
) -> AppResult<()> {
    let ids = gap2::parse_id_csv(pid);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("pid"));
    }
    gap2::batch_hot_key_status(
        state.db.pool(),
        &ids,
        check,
        tuijian,
        bold,
        color,
        size,
        r#type,
    )
    .await?;
    audit_write(state, actor, "admin.keyword.status", format!("{ids:?}")).await;
    Ok(())
}

pub async fn cron_info(state: &AppState, id: Option<u64>) -> AppResult<Value> {
    let mut arrweek = Vec::new();
    let labels = [
        "wap_com_00338",
        "wap_com_00339",
        "wap_js_00029",
        "wap_js_00032",
        "wap_js_00030",
        "wap_js_00031",
        "wap_js_00033",
    ];
    for (i, lab) in labels.iter().enumerate() {
        arrweek.push(json!({ "label": lab, "value": i.to_string() }));
    }
    let montharr: Vec<Value> = (1..=31)
        .map(|i| json!({ "label": i.to_string(), "value": i.to_string() }))
        .collect();
    let hourarr: Vec<Value> = (0..=23)
        .map(|i| json!({ "label": i.to_string(), "value": i.to_string() }))
        .collect();
    let row = if let Some(id) = id.filter(|v| *v > 0) {
        gap2::find_cron(state.db.reader(), id)
            .await?
            .map(|mut r| {
                r.nowtime_n = if r.nowtime > 0 {
                    fmt_dt(r.nowtime)
                } else {
                    "-".into()
                };
                r.nexttime_n = if r.nexttime > 0 {
                    fmt_dt(r.nexttime)
                } else {
                    "-".into()
                };
                json!({
                    "id": r.id,
                    "name": r.name,
                    "dir": r.dir,
                    "type": r.r#type,
                    "week": r.week,
                    "month": r.month,
                    "hour": r.hour,
                    "minute": r.minute,
                    "display": r.display == 1,
                    "nowtime": r.nowtime,
                    "nexttime": r.nexttime,
                    "nowtime_n": r.nowtime_n,
                    "nexttime_n": r.nexttime_n,
                })
            })
            .unwrap_or(json!({}))
    } else {
        json!({})
    };
    Ok(json!({
        "arrweek": arrweek,
        "montharr": montharr,
        "hourarr": hourarr,
        "row": row,
    }))
}

pub async fn run_cron(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    let now = clock::now_ts();
    let n = gap2::touch_cron(state.db.pool(), id, now).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("cron_not_found"));
    }
    gap2::insert_cron_log(state.db.pool(), &id.to_string(), now).await?;
    audit_write(state, actor, "admin.cron.run", format!("id:{id}")).await;
    Ok(())
}

pub async fn list_cron_logs(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<CronLogRow>> {
    let db = state.db.reader();
    let mut list = gap2::list_cron_logs(db, keyword, page.offset, page.limit).await?;
    for r in &mut list {
        r.ctime_n = if r.ctime > 0 {
            fmt_dt(r.ctime)
        } else {
            String::new()
        };
    }
    let total = gap2::count_cron_logs(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}
