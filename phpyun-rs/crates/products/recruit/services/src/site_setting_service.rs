//! Site settings (aligned with PHPYun `sy_*` global toggles).
//!
//! Public endpoint: read-only access to keys with `is_public=1`. Admin endpoint: full access plus create/update/delete.

use phpyun_core::{audit, clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::bank::repo as bank_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::poster_template::repo as whb_repo;
use phpyun_models::seo;
use phpyun_models::site_setting::{entity::SiteSetting, repo as setting_repo};
use serde_json::{json, Map, Value};

pub async fn list_public(state: &AppState) -> AppResult<Vec<SiteSetting>> {
    Ok(setting_repo::list_public(state.db.reader()).await?)
}

pub async fn get(state: &AppState, key: &str) -> AppResult<Option<SiteSetting>> {
    Ok(setting_repo::find(state.db.reader(), key).await?)
}

// ---------- admin ----------

pub async fn admin_list(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<SiteSetting>> {
    user.require_admin()?;
    Ok(setting_repo::list_all(state.db.reader()).await?)
}

pub struct UpsertInput<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub description: &'a str,
    pub is_public: bool,
}

pub async fn admin_upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: UpsertInput<'_>,
) -> AppResult<()> {
    user.require_admin()?;
    let now = clock::now_ts();
    setting_repo::upsert(
        state.db.pool(),
        input.key,
        input.value,
        input.description,
        input.is_public,
        now,
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.upsert", audit::Actor::uid(user.uid))
            .target(format!("key:{}", input.key)),
    )
    .await;
    Ok(())
}

pub async fn admin_delete(state: &AppState, user: &AuthenticatedUser, key: &str) -> AppResult<()> {
    user.require_admin()?;
    setting_repo::delete(state.db.pool(), key).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.delete", audit::Actor::uid(user.uid))
            .target(format!("key:{key}")),
    )
    .await;
    Ok(())
}

fn cfg_map(rows: &[SiteSetting]) -> Map<String, Value> {
    let mut out = Map::new();
    for s in rows {
        out.insert(s.key_name.clone(), Value::String(s.value.clone()));
    }
    out
}

fn pick_map(cfg: &Map<String, Value>, keys: &[(&str, &str)]) -> Map<String, Value> {
    let mut out = Map::new();
    for (k, default) in keys {
        let v = cfg.get(*k).and_then(|x| x.as_str()).unwrap_or(*default);
        out.insert((*k).to_string(), Value::String(v.to_string()));
    }
    out
}

const ALIPAY_KEYS: &[(&str, &str)] = &[
    ("alipaytype", "1"),
    ("sy_alipayname", ""),
    ("sy_alipayKeyType", "1"),
    ("sy_alipayid", ""),
    ("sy_alipaycode", ""),
    ("sy_alipayemail", ""),
    ("sy_alipayappid", ""),
    ("sy_alipayprivatekey", ""),
    ("sy_alipaypublickey", ""),
    ("sy_weburl", ""),
];

const TENPAY_KEYS: &[(&str, &str)] = &[
    ("sy_tenpayid", ""),
    ("sy_tenpaycode", ""),
    ("sy_weburl", ""),
];

/// PHP `set_payset::index_action`: `{config, alipaydata, tenpaydata, bankrows}`.
pub async fn payset_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let config = cfg_map(&rows);
    let alipaydata = pick_map(&config, ALIPAY_KEYS);
    let tenpaydata = pick_map(&config, TENPAY_KEYS);
    let bankrows = bank_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "config": Value::Object(config),
        "alipaydata": Value::Object(alipaydata),
        "tenpaydata": Value::Object(tenpaydata),
        "bankrows": bankrows,
    }))
}

async fn upsert_keys(
    state: &AppState,
    user: &AuthenticatedUser,
    pairs: &[(&str, String)],
) -> AppResult<()> {
    for (k, v) in pairs {
        if k.is_empty() || *k == "pay_config" || *k == "config" {
            continue;
        }
        admin_upsert(
            state,
            user,
            UpsertInput {
                key: k,
                value: v,
                description: "",
                is_public: false,
            },
        )
        .await?;
    }
    Ok(())
}

fn str_field(v: &Value, key: &str) -> String {
    v.get(key)
        .map(|x| match x {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => {
                if *b {
                    "1".into()
                } else {
                    "0".into()
                }
            }
            Value::Null => String::new(),
            other => other.to_string(),
        })
        .unwrap_or_default()
}

fn or_weburl(body: &Value, weburl: &str) -> String {
    let s = str_field(body, "sy_weburl");
    if s.is_empty() {
        weburl.to_string()
    } else {
        s
    }
}

/// PHP `set_payset::alipay_action` — persist keys to `phpyun_admin_config`.
pub async fn payset_alipay(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let weburl = setting_repo::find(state.db.reader(), "sy_weburl")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    upsert_keys(
        state,
        user,
        &[
            ("alipaytype", str_field(body, "alipaytype")),
            ("sy_alipayname", str_field(body, "sy_alipayname")),
            ("sy_alipayKeyType", str_field(body, "sy_alipayKeyType")),
            ("sy_alipayid", str_field(body, "sy_alipayid")),
            ("sy_alipaycode", str_field(body, "sy_alipaycode")),
            ("sy_alipayemail", str_field(body, "sy_alipayemail")),
            ("sy_alipayappid", str_field(body, "sy_alipayappid")),
            ("sy_alipayprivatekey", str_field(body, "sy_alipayprivatekey")),
            ("sy_alipaypublickey", str_field(body, "sy_alipaypublickey")),
            ("sy_weburl", or_weburl(body, &weburl)),
        ],
    )
    .await
}

/// PHP `set_payset::tenpay_action`.
pub async fn payset_tenpay(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let weburl = setting_repo::find(state.db.reader(), "sy_weburl")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    upsert_keys(
        state,
        user,
        &[
            ("sy_tenpayid", str_field(body, "sy_tenpayid")),
            ("sy_tenpaycode", str_field(body, "sy_tenpaycode")),
            ("sy_weburl", or_weburl(body, &weburl)),
        ],
    )
    .await
}

pub struct BankIn<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub bank_name: &'a str,
    pub bank_number: &'a str,
    pub bank_address: &'a str,
}

/// PHP `set_payset::bank_action` — unique `bank_number`.
pub async fn payset_bank_upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: BankIn<'_>,
) -> AppResult<u64> {
    user.require_admin()?;
    if let Some(existing) = bank_repo::find_by_number(state.db.reader(), input.bank_number).await? {
        let clash = match input.id.filter(|i| *i > 0) {
            Some(id) => existing.id != id,
            None => true,
        };
        if clash {
            return Err(ApiError::business("admin_system_00054"));
        }
    }
    let id = bank_repo::upsert(
        state.db.pool(),
        bank_repo::BankUpsert {
            id: input.id,
            name: input.name,
            bank_name: input.bank_name,
            bank_number: input.bank_number,
            bank_address: input.bank_address,
        },
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.payset.bank.upsert", audit::Actor::uid(user.uid))
            .target(format!("bank:{id}")),
    )
    .await;
    Ok(id)
}

/// PHP `set_payset::del_action`.
pub async fn payset_bank_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_admin()?;
    bank_repo::delete(state.db.pool(), id).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.payset.bank.delete", audit::Actor::uid(user.uid))
            .target(format!("bank:{id}")),
    )
    .await;
    Ok(())
}

const SEO_MODEL: &[(&str, &str)] = &[
    ("index", "首页"),
    ("job", "找工作"),
    ("resume", "找人才"),
    ("part", "兼职"),
    ("company", "公司"),
    ("article", "新闻公告"),
    ("hr", "工具箱"),
    ("zph", "招聘会"),
    ("ask", "问答"),
    ("evaluate", "测评"),
    ("once", "店铺"),
    ("tiny", "普工"),
    ("redeem", "商城"),
    ("map", "地图"),
    ("special", "专题"),
    ("login", "登录注册"),
    ("other", "其它"),
];

fn seo_model_map() -> Map<String, Value> {
    SEO_MODEL
        .iter()
        .map(|(k, v)| ((*k).to_string(), json!(*v)))
        .collect()
}

fn seo_config() -> Value {
    json!({
        "public": {
            "webname": "网站名称", "webkeyword": "网站关键字", "webdesc": "网站描述",
            "weburl": "网址", "city": "当前城市", "seacrh_class": "搜索类别",
            "search_city": "搜索城市", "search_job": "搜索职能",
        },
        "other": { "spename": "专题名称" },
        "article": {
            "news_class": "新闻类别", "news_title": "新闻标题", "news_keyword": "新闻关键字",
            "news_source": "新闻来源", "news_author": "新闻作者", "news_desc": "新闻描述",
            "gg_title": "公告标题", "gg_desc": "公告描述", "gz_title": "公招标题", "gz_desc": "公招描述",
        },
        "company": {
            "company_name": "企业名称", "company_name_desc": "企业简介",
            "company_product": "企业产品", "company_news": "企业新闻",
            "company_news_desc": "企业新闻描述", "industry_class": "行业类别",
        },
        "job": {
            "industry_class": "行业类别", "job_class": "职位类别", "job_name": "职位名称",
            "job_desc": "职位描述", "job_salary": "职位薪资", "company_name": "企业名称",
        },
        "part": { "part_name": "兼职名称" },
        "zph": { "zph_title": "招聘会标题", "zph_desc": "招聘会描述" },
        "ask": { "ask_title": "问答标题", "ask_desc": "问答描述", "ask_class_name": "分类名称" },
        "resume": { "resume_username": "简历姓名", "resume_job": "简历意向职位", "resume_city": "简历工作城市" },
        "tiny": { "tiny_username": "普工简历名称", "tiny_job": "普工简历职位", "tiny_desc": "普工简历描述" },
        "once": { "once_name": "店铺名称", "once_job": "店铺招聘职位", "once_desc": "店铺招聘描述" },
        "hr": { "hr_class": "类别名称", "hr_desc": "类别描述", "hr_name": "工具箱详情" },
        "gg": { "gg_title": "公告标题", "gg_desc": "公告描述" },
        "friend": { "company_name": "企业名称" },
    })
}

fn seo_php(row: &seo::SeoRow) -> Value {
    let mut v = serde_json::to_value(row).unwrap_or(json!({}));
    if let Some(o) = v.as_object_mut() {
        o.insert(
            "time_n".into(),
            json!(if row.time > 0 {
                phpyun_core::utils::fmt_dt(row.time)
            } else {
                String::new()
            }),
        );
    }
    v
}

fn php_str_array(s: &str) -> Vec<String> {
    let t = s.trim();
    if t.is_empty() {
        return Vec::new();
    }
    if t.starts_with('[') {
        return serde_json::from_str(t).unwrap_or_default();
    }
    let mut out = Vec::new();
    let b = t.as_bytes();
    let mut i = 0;
    while i + 2 < b.len() {
        if b[i] == b's' && b[i + 1] == b':' {
            i += 2;
            let start = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            let len: usize = std::str::from_utf8(&b[start..i])
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(0);
            if i < b.len() && b[i] == b':' {
                i += 1;
            }
            if i < b.len() && b[i] == b'"' {
                i += 1;
            }
            let end = (i + len).min(b.len());
            if let Ok(st) = std::str::from_utf8(&b[i..end]) {
                out.push(st.to_string());
            }
            i = end;
            if i < b.len() && b[i] == b'"' {
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    out
}

fn php_serialize_strs(items: &[String]) -> String {
    let mut inner = String::new();
    for (i, s) in items.iter().enumerate() {
        inner.push_str(&format!("i:{i};s:{}:\"{s}\";", s.len()));
    }
    format!("a:{}:{{{inner}}}", items.len())
}

fn json_str_list(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a
            .iter()
            .filter_map(|x| match x {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
            .collect(),
        Value::String(s) => {
            let t = s.trim();
            if t.starts_with('[') {
                serde_json::from_str(t).unwrap_or_else(|_| php_str_array(s))
            } else {
                php_str_array(s)
            }
        }
        _ => Vec::new(),
    }
}

/// PHP `set_seo::index_action`.
pub async fn seo_index(state: &AppState, user: &AuthenticatedUser, action: &str) -> AppResult<Value> {
    user.require_admin()?;
    if action.is_empty() {
        return Ok(json!({ "seomodel": seo_model_map() }));
    }
    let rows = seo::list_by_model(state.db.reader(), action).await?;
    let seolist: Vec<Value> = rows.iter().map(seo_php).collect();
    Ok(json!({ "seolist": seolist }))
}

/// PHP `set_seo::seoadd_action`.
pub async fn seo_add_form(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<Value> {
    user.require_admin()?;
    let domains = domain_repo::list_all(state.db.reader()).await?;
    let mut dname = Map::new();
    for d in domains {
        dname.insert(d.id.to_string(), json!(d.title));
    }
    let info = if id > 0 {
        seo::find_by_id(state.db.reader(), id)
            .await?
            .map(|r| seo_php(&r))
            .unwrap_or(json!({}))
    } else {
        json!({})
    };
    Ok(json!({
        "seomodel": seo_model_map(),
        "seoconfig": seo_config(),
        "Dname": dname,
        "info": info,
    }))
}

/// PHP `set_seo::save_action`.
pub async fn seo_save(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<u64> {
    user.require_admin()?;
    let id = match body.get("id") {
        Some(Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(Value::String(s)) => s.parse().unwrap_or(0),
        _ => 0,
    };
    let nid = seo::upsert(
        state.db.pool(),
        id,
        &str_field(body, "seoname"),
        &str_field(body, "ident"),
        &str_field(body, "seomodel"),
        &str_field(body, "title"),
        &str_field(body, "keywords"),
        &str_field(body, "php_url"),
        &str_field(body, "rewrite_url"),
        &str_field(body, "php_wap_url"),
        &str_field(body, "rewrite_wap_url"),
        &str_field(body, "description"),
        str_field(body, "did").parse().unwrap_or(0),
        clock::now_ts(),
    )
    .await?;
    if nid == 0 {
        return Err(ApiError::business("admin_model_00103"));
    }
    Ok(nid)
}

/// PHP `set_seo::del_action`.
pub async fn seo_del(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<()> {
    user.require_admin()?;
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = seo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("admin_model_00105"));
    }
    Ok(())
}

/// PHP `set_regset::index_action`.
pub async fn regset_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let config = cfg_map(&rows);
    let mut regconfig = Map::new();
    for (name, raw) in setting_repo::list_reg_config(state.db.reader()).await? {
        regconfig.insert(name, json!(php_str_array(&raw)));
    }
    for k in ["regname", "mobile_number", "mobile_white", "mobile_black"] {
        regconfig.entry(k.to_string()).or_insert(json!([]));
    }
    Ok(json!({ "config": config, "regconfig": regconfig }))
}

/// PHP `set_regset::save_action`.
pub async fn regset_save(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<()> {
    user.require_admin()?;
    let obj = body.as_object().cloned().unwrap_or_default();
    let skip = [
        "pytoken",
        "m",
        "c",
        "a",
        "config",
        "regname",
        "mobile_number",
        "mobile_white",
        "mobile_black",
    ];
    for (k, _) in &obj {
        if skip.iter().any(|s| *s == k.as_str()) || k.is_empty() || k.len() > 64 {
            continue;
        }
        if phpyun_core::validators::path_token(k).is_err() {
            continue;
        }
        admin_upsert(
            state,
            user,
            UpsertInput {
                key: k,
                value: &str_field(body, k),
                description: "",
                is_public: false,
            },
        )
        .await?;
    }
    for key in ["regname", "mobile_number", "mobile_white", "mobile_black"] {
        let list = json_str_list(body.get(key).unwrap_or(&Value::Null));
        setting_repo::upsert_reg_config(state.db.pool(), key, &php_serialize_strs(&list)).await?;
    }
    Ok(())
}

const MSGSET_KEYS: &[&str] = &[
    "sy_msg_isopen",
    "sy_msg_appkey",
    "sy_msg_appsecret",
    "ip_msgnum",
    "moblie_msgnum",
    "cert_msgtime",
    "moblie_codetime",
    "sy_kh_isopen",
    "sy_kh_appkey",
    "sy_kh_appsecret",
    "sy_kh_city",
    "sy_tyc_appkey",
    "sy_tyc_appsecret",
];

/// PHP `messageset::index_action`（余额不请求 ov6.com，填 0）。
pub async fn messageset_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let cfg = cfg_map(&rows);
    let mut out = Map::new();
    for k in MSGSET_KEYS {
        out.insert(
            (*k).to_string(),
            json!(cfg.get(*k).and_then(|v| v.as_str()).unwrap_or("")),
        );
    }
    out.insert("rest_msgnum".into(), json!(0));
    out.insert("rest_businessnum".into(), json!(0));
    out.insert("rest_khnum".into(), json!(0));
    Ok(Value::Object(out))
}

/// PHP `yingxiao_hbconfig::index_action`.
pub async fn hbconfig_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let cfg = cfg_map(&rows);
    let web = cfg.get("sy_weburl").and_then(|v| v.as_str()).unwrap_or("");
    let logo = cfg
        .get("sy_haibao_web_logo")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let logo_n = if logo.starts_with("http") || logo.is_empty() {
        logo.to_string()
    } else {
        format!("{}/{}", web.trim_end_matches('/'), logo.trim_start_matches('/'))
    };
    Ok(json!({
        "config": {
            "sy_haibao_isopen": cfg.get("sy_haibao_isopen").and_then(|v| v.as_str()).unwrap_or("1"),
            "sy_haibao_web_type": cfg.get("sy_haibao_web_type").and_then(|v| v.as_str()).unwrap_or("3"),
            "sy_haibao_web_name": cfg.get("sy_haibao_web_name").and_then(|v| v.as_str()).unwrap_or(""),
            "sy_haibao_web_logo_n": logo_n,
        }
    }))
}

/// PHP `yingxiao_hbconfig::saveSet_action`（跳过 logo 上传）。
pub async fn hbconfig_save_set(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    for k in ["sy_haibao_isopen", "sy_haibao_web_type", "sy_haibao_web_name"] {
        admin_upsert(
            state,
            user,
            UpsertInput {
                key: k,
                value: &str_field(body, k),
                description: "",
                is_public: false,
            },
        )
        .await?;
    }
    Ok(())
}

/// PHP `yingxiao_hbconfig::{job,com,inviteReg,gongzhao}_action`.
pub async fn hbconfig_list(state: &AppState, user: &AuthenticatedUser, typ: i32) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let web = cfg_map(&rows)
        .get("sy_weburl")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_end_matches('/')
        .to_string();
    let list = whb_repo::list_admin_by_type(state.db.reader(), typ).await?;
    let list: Vec<Value> = list
        .into_iter()
        .map(|r| {
            let pic_n = if r.pic.starts_with("http") || r.pic.is_empty() {
                r.pic.clone()
            } else {
                format!("{web}/{}", r.pic.trim_start_matches('/'))
            };
            json!({
                "id": r.id,
                "name": r.name,
                "pic": r.pic,
                "pic_n": pic_n,
                "sort": r.sort,
                "isopen": r.isopen,
                "type": r.r#type,
                "num": r.num,
                "style": r.style,
            })
        })
        .collect();
    Ok(json!({ "list": list }))
}

/// PHP `yingxiao_hbconfig::saveWhbConfig_action`.
pub async fn hbconfig_save_open(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let (typ, key) = if body.get("sy_job_hb").is_some() {
        (1, "sy_job_hb")
    } else if body.get("sy_com_hb").is_some() {
        (2, "sy_com_hb")
    } else if body.get("sy_invite_reg_hb").is_some() {
        (3, "sy_invite_reg_hb")
    } else if body.get("sy_gongzhao_hb").is_some() {
        (4, "sy_gongzhao_hb")
    } else {
        return Err(ApiError::business("wap_com_00228"));
    };
    let raw = str_field(body, key);
    let ids: Vec<u64> = raw
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .filter(|n: &u64| *n > 0)
        .collect();
    whb_repo::set_open_ids(state.db.pool(), typ, &ids).await?;
    Ok(())
}
