//! PHP admin `getCache` / `index_base_data` / nested settings JSON.

use std::collections::HashMap;
use std::path::PathBuf;

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::ad::repo as ad_repo;
use phpyun_models::admin_gap::extra as gap_extra;
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::article::repo as article_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_address::repo as addr_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::site_setting::repo as setting_repo;
use serde_json::{json, Map, Value};

use crate::{admin_dashboard_service, category_service, dict_service, redeem_service};

const SOURCE: &[(&str, &str)] = &[
    ("1", "网页"),
    ("2", "手机"),
    ("4", "微信"),
    ("6", "采集"),
    ("8", "QQ登录"),
    ("9", "微信扫一扫"),
    ("10", "微博"),
    ("11", "PC快速投递"),
    ("12", "WAP快速投递"),
    ("21", "账户分离"),
    ("26", "预留信息"),
];

const USERSET_KEYS: &[&str] = &[
    "user_height_resume",
    "user_idcard_status",
    "user_msg_status",
    "user_photo_status",
    "rshow_photo_status",
    "user_trust_status",
    "resume_status",
    "user_revise_state",
    "resume_statetime_start",
    "resume_statetime_end",
    "user_resume_status",
    "user_gzgzh",
    "resume_kstd",
    "resume_create_exp",
    "resume_create_edu",
    "resume_create_project",
    "expcreate",
    "educreate",
    "sy_resume_job_classid",
    "sy_resume_kh_td",
    "resume_kstd_req",
    "user_sqintegrity",
    "user_work_regiser",
    "user_edu_regiser",
    "user_project_regiser",
    "sy_rname_num",
    "user_number",
    "user_finder",
    "user_trust_number",
    "user_name",
    "user_pic",
    "resume_sx",
    "resume_open_check",
    "sy_user_visit_resume",
    "sy_shresume_applyjob",
    "com_resume_partapply",
    "sy_resumename_num",
    "sq_resume_interval",
];

const COMSET_KEYS: &[&str] = &[
    "com_status",
    "com_job_status",
    "com_partjob_status",
    "com_cert_status",
    "com_logo_status",
    "com_show_status",
    "com_banner_status",
    "com_revise_status",
    "com_yqmb_status",
    "com_enforce_info",
    "com_enforce_mobilecert",
    "com_enforce_emailcert",
    "com_enforce_licensecert",
    "com_enforce_setposition",
    "com_gzgzh",
    "com_social_credit",
    "com_cert_owner",
    "com_cert_wt",
    "com_cert_other",
    "exa_cert_wt",
    "com_message",
    "com_job_myswitch",
    "com_job_sexswitch",
    "com_free_status",
    "com_zpdata",
    "com_job_reserve",
    "sy_reserve_refresh_interval",
    "sy_reserve_refresh_price",
    "sy_reserve_service_id",
    "com_search",
    "com_status_search",
    "com_lietou_job",
    "com_finder",
    "com_yqmb_num",
    "sy_maturityday",
    "sy_sq_job_num",
    "joblist_top_index",
    "joblist_top",
    "joblock",
    "sqjob_req",
    "hotcom_top",
    "jobunder",
    "job_under_delay",
    "com_link_look",
    "com_login_link",
    "com_link_no",
    "sy_link_tips",
    "job_full_text_search",
];

fn str_map(pairs: &[(&str, &str)]) -> Map<String, Value> {
    let mut m = Map::new();
    for (k, v) in pairs {
        m.insert((*k).into(), Value::String((*v).into()));
    }
    m
}

fn id_name_map(rows: &[(i32, String)]) -> Map<String, Value> {
    let mut m = Map::new();
    for (id, name) in rows {
        m.insert(id.to_string(), Value::String(name.clone()));
    }
    m
}

fn userdata_from(dicts: &dict_service::LocalizedDicts) -> (Map<String, Value>, Map<String, Value>) {
    let names = id_name_map(&dicts.userclass_all());
    let mut data = Map::new();
    for var in dicts.userclass_var_names() {
        let ids: Vec<Value> = dicts
            .userclass_by_variable(&var)
            .into_iter()
            .map(|(id, _)| Value::from(id))
            .collect();
        data.insert(var, Value::Array(ids));
    }
    (data, names)
}

fn comdata_from(dicts: &dict_service::LocalizedDicts) -> (Map<String, Value>, Map<String, Value>) {
    let names = id_name_map(&dicts.comclass_all());
    let mut data = Map::new();
    for var in dicts.comclass_var_names() {
        let ids: Vec<Value> = dicts
            .comclass_by_variable(&var)
            .into_iter()
            .map(|(id, _)| Value::from(id))
            .collect();
        data.insert(var, Value::Array(ids));
    }
    (data, names)
}

fn job_cache(nodes: &[(u64, u64, String)]) -> (Map<String, Value>, Vec<u64>, Map<String, Value>) {
    let mut job_name = Map::new();
    let mut by_parent: HashMap<u64, Vec<u64>> = HashMap::new();
    for (id, pid, name) in nodes {
        job_name.insert(id.to_string(), Value::String(name.clone()));
        by_parent.entry(*pid).or_default().push(*id);
    }
    let job_index = by_parent.get(&0).cloned().unwrap_or_default();
    let mut job_type = Map::new();
    for (pid, kids) in by_parent {
        if pid == 0 {
            continue;
        }
        job_type.insert(
            pid.to_string(),
            Value::Array(kids.into_iter().map(Value::from).collect()),
        );
    }
    (job_name, job_index, job_type)
}

fn id_name_arr(rows: &[(i32, String)]) -> Vec<Value> {
    rows.iter()
        .map(|(id, name)| json!({ "id": id, "name": name }))
        .collect()
}

fn settings_map(rows: &[phpyun_models::site_setting::entity::SiteSetting]) -> Map<String, Value> {
    let mut m = Map::new();
    for r in rows {
        m.insert(r.key_name.clone(), Value::String(r.value.clone()));
    }
    m
}

fn pick(cfg: &Map<String, Value>, keys: &[&str]) -> Map<String, Value> {
    let mut out = Map::new();
    for k in keys {
        out.insert(
            (*k).into(),
            cfg.get(*k).cloned().unwrap_or(Value::String(String::new())),
        );
    }
    out
}

fn cfg_str(cfg: &Map<String, Value>, key: &str) -> String {
    cfg.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn time_search() -> Map<String, Value> {
    str_map(&[
        ("1", "common_01940"),
        ("3", "admin_user_00179"),
        ("7", "admin_user_00178"),
        ("15", "admin_user_00180"),
        ("30", "admin_user_00175"),
    ])
}

fn domain_object(rows: &[phpyun_models::domain::entity::DomainSite]) -> Map<String, Value> {
    let mut dname = Map::new();
    for d in rows {
        dname.insert(d.id.to_string(), Value::String(d.title.clone()));
    }
    dname
}

fn domain_options(rows: &[phpyun_models::domain::entity::DomainSite]) -> Vec<Value> {
    rows.iter()
        .map(|d| json!({ "label": d.title, "value": d.id.to_string() }))
        .collect()
}

fn search_kv(param: &str, name: &str, value: Map<String, Value>) -> Value {
    json!({ "param": param, "name": name, "value": value })
}

fn special_tpl_files(style: &str) -> Vec<String> {
    let dir = PathBuf::from("/www/wwwroot/zzzz.com/uploads/app/template")
        .join(if style.is_empty() { "default" } else { style })
        .join("special");
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for ent in rd.flatten() {
        let name = ent.file_name().to_string_lossy().into_owned();
        if name.ends_with(".htm") && name != "index.htm" && name != "job.htm" {
            out.push(name);
        }
    }
    out.sort();
    out
}

async fn cat_nodes(state: &AppState, kind: &str) -> AppResult<Vec<(u64, u64, String)>> {
    let rows = category_service::list(state, kind).await?;
    Ok(rows
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect())
}

/// PHP named cache / base-data / nested settings.
pub async fn php_page(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: &str,
    pid: i32,
) -> AppResult<Value> {
    user.require_admin()?;
    match kind {
        "resume_getCache" => resume_get_cache(state).await,
        "tiny_getCache" => tiny_get_cache(state).await,
        "once_getCache" => once_get_cache(state).await,
        "friendlink_getCache" => friendlink_get_cache(state).await,
        "admin_member_getCache" => admin_member_get_cache(state).await,
        "userset_indexBaseData" => userset_base(state).await,
        "userset_index" => userset_index(state).await,
        "comset_index" => comset_index(state).await,
        "emailset_index" => emailset_index(state).await,
        "getAdminCache" => get_admin_cache(state).await,
        "shop_list_base" => Ok(json!({
            "search_list": [
                search_kv("change", "wap_user_00002", time_search()),
                search_kv("status", "wap_com_00406", str_map(&[
                    ("-1", "wap_user_00166"),
                    ("1", "wap_user_00165"),
                    ("2", "wap_user_00167"),
                ])),
            ]
        })),
        "shop_reward_base" => shop_reward_base(state).await,
        "report_resume_base" => {
            let name = setting_repo::find(state.db.reader(), "integral_pricename")
                .await?
                .map(|s| s.value)
                .unwrap_or_else(|| "积分".into());
            Ok(json!({ "integral_pricename": name }))
        }
        "messagelog_base" => Ok(json!({
            "ports": {
                "1": "member_user_00094",
                "2": "WAP",
                "5": "wap_js_00101",
                "7": "ajax_00010",
                "8": "wap_00121"
            }
        })),
        "dataCall_base" => Ok(json!({ "dataCall": datacall_static() })),
        "member_search" => Ok(json!({
            "search_list": [
                search_kv("operas", "admin_user_00155", str_map(&[
                    ("88", "admin_user_00157"), ("2", "wap_com_00428"), ("6", "wap_00574"),
                    ("5", "wap_user_00193"), ("7", "wap_00456"), ("11", "admin_user_00152"),
                    ("8", "member_user_00226"), ("12", "member_com_00093"), ("16", "wap_js_00081"),
                    ("17", "common_06524"), ("18", "wap_user_00365"), ("19", "wap_user_00223"),
                    ("23", "wap_com_00350"), ("25", "admin_user_00154"), ("26", "wap_user_00221"),
                ])),
                search_kv("parrs", "wap_com_00030", str_map(&[
                    ("1", "admin_user_00156"), ("2", "wap_js_00073"),
                    ("3", "wap_js_00077"), ("4", "wap_user_00334"),
                ])),
                search_kv("end", "member_user_00241", time_search()),
            ]
        })),
        "hotjob_search" => hotjob_search(state).await,
        "trust_search" => Ok(json!({
            "search_list": [
                search_kv("status", "wap_com_00406", str_map(&[
                    ("1", "wap_user_00165"), ("2", "wap_user_00166"),
                ])),
                search_kv("end", "member_user_00241", time_search()),
            ]
        })),
        "news_getCache" => news_get_cache(state).await,
        "special_base" => special_base(state).await,
        "ad_base" => ad_base(state).await,
        "dataCollection_getCache" => data_collection_cache(state).await,
        "job_child_ids" => job_child_ids(state, pid).await,
        "city_child_ids" => city_child_ids(state, pid).await,
        _ => Err(ApiError::param_invalid("php_page_kind")),
    }
}

async fn resume_get_cache(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let (userdata, userclass_name) = userdata_from(&dicts);
    Ok(json!({ "userdata": userdata, "userclass_name": userclass_name }))
}

/// PHP `users_member::edit` / `users_resume::editResume` 字典块。
pub async fn resume_member_cache(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let (userdata, userclass_name) = userdata_from(&dicts);
    let hy = dicts.industry_all();
    let industry_index: Vec<i32> = hy.iter().map(|(id, _)| *id).collect();
    let mut industry_name = Map::new();
    for (id, name) in hy {
        industry_name.insert(id.to_string(), Value::String(name));
    }
    Ok(json!({
        "user_sex": { "1": "男", "2": "女" },
        "userdata": userdata,
        "userclass_name": userclass_name,
        "industry_index": industry_index,
        "industry_name": industry_name,
    }))
}

async fn tiny_get_cache(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let (userdata, userclass_name) = userdata_from(&dicts);
    let user_word: Vec<Value> = userdata
        .get("user_word")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter_map(|id| {
            let id = id.as_i64()?;
            let name = userclass_name.get(&id.to_string())?.as_str()?.to_string();
            Some(json!({ "id": id, "name": name }))
        })
        .collect();
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "user_sex": { "1": "男", "2": "女" },
        "user_word": user_word,
        "search_list": [
            search_kv("status", "wap_com_00406", str_map(&[
                ("1", "wap_user_00165"), ("3", "wap_user_00166"), ("2", "member_com_00304"),
            ])),
            search_kv("time", "admin_user_weipin_00030", time_search()),
        ],
        "dname": domain_object(&domains),
    }))
}

async fn once_get_cache(state: &AppState) -> AppResult<Value> {
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "search_list": [
            search_kv("status", "wap_com_00406", str_map(&[
                ("1", "wap_user_00165"), ("3", "wap_user_00166"), ("2", "member_com_00304"),
            ])),
            search_kv("time", "admin_user_weipin_00030", time_search()),
        ],
        "dname": domain_object(&domains),
    }))
}

async fn friendlink_get_cache(state: &AppState) -> AppResult<Value> {
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({ "domain": domain_options(&domains) }))
}

async fn admin_member_get_cache(state: &AppState) -> AppResult<Value> {
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "source": str_map(SOURCE),
        "dname": domain_object(&domains),
    }))
}

async fn userset_base(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let (userdata, userclass_name) = userdata_from(&dicts);
    let jobs = cat_nodes(state, "job").await?;
    let (job_name, job_index, job_type) = job_cache(&jobs);
    Ok(json!({
        "userdata": userdata,
        "userclass_name": userclass_name,
        "user_sex": { "1": "男", "2": "女" },
        "job_name": job_name,
        "job_index": job_index,
        "job_type": job_type,
    }))
}

async fn userset_index(state: &AppState) -> AppResult<Value> {
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let cfg = settings_map(&rows);
    let config = pick(&cfg, USERSET_KEYS);
    let dicts = dict_service::get(state).await?;
    let jobs = cat_nodes(state, "job").await?;
    let (job_name, _, _) = job_cache(&jobs);
    let mut selected = Map::new();
    for id in cfg_str(&cfg, "sy_resume_job_classid").split(',') {
        let id = id.trim();
        if id.is_empty() {
            continue;
        }
        if let Some(n) = job_name.get(id) {
            selected.insert(id.to_string(), n.clone());
        } else {
            let nid: i32 = id.parse().unwrap_or(0);
            let name = dicts.job(nid);
            if !name.is_empty() {
                selected.insert(id.to_string(), Value::String(name.to_string()));
            }
        }
    }
    Ok(json!({ "config": config, "selected": selected }))
}

async fn comset_index(state: &AppState) -> AppResult<Value> {
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let cfg = settings_map(&rows);
    let mut config = pick(&cfg, COMSET_KEYS);
    if cfg_str(&config, "com_zpdata").is_empty() {
        config.insert("com_zpdata".into(), Value::String("1".into()));
    }
    let qy_rows = company_repo::list_rating_options(state.db.reader()).await?;
    let com_servers = gap_extra::list_rating_services(state.db.reader()).await?;
    let servers: Vec<Value> = com_servers
        .into_iter()
        .filter(|s| s.display == 1)
        .map(|s| json!({ "id": s.id, "name": s.name, "display": s.display, "sort": s.sort }))
        .collect();
    let link_no: Vec<String> = cfg_str(&cfg, "com_link_no")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(json!({
        "config": config,
        "qy_rows": qy_rows,
        "com_servers": servers,
        "com_link_no": link_no,
    }))
}

async fn emailset_index(state: &AppState) -> AppResult<Value> {
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let cfg = settings_map(&rows);
    let smtp = gap_extra::list_admin_email(state.db.reader()).await?;
    let list: Vec<Value> = smtp
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "smtpserver": r.smtpserver,
                "smtpuser": r.smtpuser,
                "smtppass": r.smtppass,
                "smtpport": r.smtpport,
                "smtpnick": r.smtpnick,
                "default": r.default_flag,
            })
        })
        .collect();
    let online = if cfg_str(&cfg, "sy_email_online") == "1" {
        1
    } else {
        2
    };
    Ok(json!({
        "SMTPlist": list,
        "sy_email_online": online,
        "accesskey": cfg_str(&cfg, "accesskey"),
        "accesssecret": cfg_str(&cfg, "accesssecret"),
        "ali_email": cfg_str(&cfg, "ali_email"),
        "ali_tag": cfg_str(&cfg, "ali_tag"),
        "ali_name": cfg_str(&cfg, "ali_name"),
    }))
}

async fn get_admin_cache(state: &AppState) -> AppResult<Value> {
    let groups = rbac_repo::list_groups_by_type(state.db.reader(), 2).await?;
    let domains = domain_repo::list_all(state.db.reader()).await?;
    let group_arr: Vec<Value> = groups
        .into_iter()
        .map(|g| json!({ "id": g.id, "name": g.group_name }))
        .collect();
    let domain_arr: Vec<Value> = domains
        .into_iter()
        .map(|d| json!({ "id": d.id, "name": d.title }))
        .collect();
    Ok(json!({ "groupArr": group_arr, "domainArr": domain_arr }))
}

async fn shop_reward_base(state: &AppState) -> AppResult<Value> {
    let classes = redeem_service::list_classes(state, None).await?;
    let mut classdata = Map::new();
    for c in classes.iter() {
        classdata.insert(c.id.to_string(), Value::String(c.name.clone()));
    }
    Ok(json!({
        "search_list": [
            search_kv("status", "member_user_00181", str_map(&[
                ("1", "wap_com_00244"), ("2", "wap_com_00245"),
            ])),
            json!({ "param": "nid", "name": "admin_00223", "value": classdata }),
            search_kv("rec", "wap_01465", str_map(&[
                ("1", "admin_model_00059"), ("2", "admin_model_00060"),
            ])),
            search_kv("hot", "wap_js_00093", str_map(&[
                ("1", "admin_model_00059"), ("2", "admin_model_00060"),
            ])),
        ]
    }))
}

async fn news_get_cache(state: &AppState) -> AppResult<Value> {
    let groups = article_repo::list_groups(state.db.reader()).await?;
    let mut one_class = Vec::new();
    let mut two_by_parent: HashMap<u64, Vec<Value>> = HashMap::new();
    for g in &groups {
        if g.keyid == 0 {
            one_class.push(json!({ "id": g.id, "name": g.name }));
        } else {
            two_by_parent.entry(g.keyid as u64).or_default().push(json!({
                "id": g.id,
                "name": g.name,
            }));
        }
    }
    let mut two_class = Map::new();
    let mut class_arr = Vec::new();
    let mut class_cascader = Vec::new();
    for one in &one_class {
        let id = one.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        class_arr.push(one.clone());
        let mut children = Vec::new();
        if let Some(kids) = two_by_parent.get(&id) {
            let mut kid_map = Map::new();
            for k in kids {
                let kid_id = k.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let kid_name = k.get("name").and_then(|v| v.as_str()).unwrap_or("");
                kid_map.insert(kid_id.to_string(), k.clone());
                class_arr.push(json!({ "id": kid_id, "name": format!(" 　┗{kid_name}") }));
                children.push(json!({ "value": kid_id, "label": kid_name }));
            }
            two_class.insert(id.to_string(), Value::Object(kid_map));
        }
        class_cascader.push(json!({
            "value": id,
            "label": one.get("name").and_then(|v| v.as_str()).unwrap_or(""),
            "children": children,
        }));
    }
    let props = gap_extra::list_news_property(state.db.reader())
        .await
        .unwrap_or_default();
    let mut property = Map::new();
    for p in props {
        if !p.value.is_empty() {
            property.insert(p.value, Value::String(p.name));
        }
    }
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "one_class": one_class,
        "two_class": two_class,
        "Dname": domain_object(&domains),
        "property": property,
        "class_arr": class_arr,
        "class_cascader": class_cascader,
        "today": chrono_today(),
    }))
}

fn chrono_today() -> String {
    let ts = phpyun_core::clock::now_ts() + 8 * 3600;
    let days = ts.div_euclid(86_400);
    let secs = 86_400 * days;
    let ymd = unix_ymd(secs);
    format!("{:04}-{:02}-{:02}", ymd.0, ymd.1, ymd.2)
}

fn unix_ymd(secs: i64) -> (i32, u32, u32) {
    // Civil from days, Howard Hinnant algorithm.
    let z = secs.div_euclid(86_400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i32 + era as i32 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

async fn special_base(state: &AppState) -> AppResult<Value> {
    let qy_rows = company_repo::list_rating_options(state.db.reader()).await?;
    let style = setting_repo::find(state.db.reader(), "style")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    Ok(json!({
        "file": special_tpl_files(&style),
        "qy_rows": qy_rows,
    }))
}

async fn ad_base(state: &AppState) -> AppResult<Value> {
    let classes = ad_repo::list_classes(state.db.reader()).await?;
    let mut class_two: HashMap<i32, Vec<Value>> = HashMap::new();
    let max_len = classes
        .iter()
        .map(|c| c.id.to_string().len())
        .max()
        .unwrap_or(1);
    for c in &classes {
        let place = if c.place == 1 || c.place == 2 {
            c.place
        } else {
            3
        };
        let pad = format!("{:0>width$}", c.id, width = max_len);
        class_two.entry(place).or_default().push(json!({
            "id": c.id.to_string(),
            "name": c.class_name,
            "id_name": format!("{pad}   {}", c.class_name),
        }));
    }
    let class_one = vec![
        json!({ "id": 1, "name": "PC" }),
        json!({ "id": 2, "name": "WAP" }),
        json!({ "id": 3, "name": "common_01924" }),
    ];
    let mut class_data = Vec::new();
    for one in &class_one {
        let id = one.get("id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let children: Vec<Value> = class_two
            .get(&id)
            .into_iter()
            .flatten()
            .map(|v| {
                json!({
                    "label": v.get("id_name").and_then(|x| x.as_str()).unwrap_or("").replace("&nbsp;", " "),
                    "value": v.get("id").cloned().unwrap_or(Value::String(String::new())),
                })
            })
            .collect();
        let mut row = json!({
            "label": one.get("name").cloned().unwrap_or(Value::String(String::new())),
            "value": id.to_string(),
        });
        if !children.is_empty() {
            row["children"] = Value::Array(children);
        }
        class_data.push(row);
    }
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "classData": class_data,
        "domainData": domain_options(&domains),
    }))
}

async fn data_collection_cache(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let jobs = cat_nodes(state, "job").await?;
    let cities = cat_nodes(state, "city").await?;
    let hy = dicts.industry_all();
    let (job_name, job_index, _) = job_cache(&jobs);
    let job_one: Vec<Value> = job_index
        .iter()
        .map(|id| {
            json!({
                "id": id,
                "name": job_name.get(&id.to_string()).and_then(|v| v.as_str()).unwrap_or(""),
            })
        })
        .collect();
    let mut job_arr = Vec::new();
    for (id, pid, name) in &jobs {
        if *pid > 0 {
            job_arr.push(json!({ "id": id, "pid": pid, "name": name }));
        }
    }
    let province: Vec<Value> = dicts
        .city_provinces()
        .iter()
        .map(|(id, name)| json!({ "id": id, "name": name }))
        .collect();
    let mut city_arr = Vec::new();
    for (id, pid, name) in &cities {
        if *pid > 0 {
            city_arr.push(json!({ "id": id, "pid": pid, "name": name }));
        }
    }
    let (comdata, comclass_name) = comdata_from(&dicts);
    let (userdata, userclass_name) = userdata_from(&dicts);
    fn named_from(ids: Option<&Value>, names: &Map<String, Value>) -> Vec<Value> {
        ids.and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|id| {
                let key = match id {
                    Value::Number(n) => n.to_string(),
                    Value::String(s) => s.clone(),
                    _ => return None,
                };
                let name = names.get(&key)?.as_str()?.to_string();
                Some(json!({ "id": id.clone(), "name": name }))
            })
            .collect()
    }
    Ok(json!({
        "industryArr": id_name_arr(&hy),
        "jobOneArr": job_one,
        "jobArr": job_arr,
        "provinceArr": province,
        "cityArr": city_arr,
        "jobPrArr": named_from(comdata.get("job_pr"), &comclass_name),
        "jobMunArr": named_from(comdata.get("job_mun"), &comclass_name),
        "jobEduArr": named_from(comdata.get("job_edu"), &comclass_name),
        "jobExpArr": named_from(comdata.get("job_exp"), &comclass_name),
        "jobMarriageArr": named_from(comdata.get("job_marriage"), &comclass_name),
        "jobReportArr": named_from(comdata.get("job_report"), &comclass_name),
        "comSexArr": [
            { "id": "2", "name": "女" },
            { "id": "3", "name": "不限" },
        ],
        "userReportArr": named_from(userdata.get("user_report"), &userclass_name),
    }))
}

async fn hotjob_search(state: &AppState) -> AppResult<Value> {
    let ratings = company_repo::list_rating_options(state.db.reader()).await?;
    let mut ratingarr = Map::new();
    for r in ratings {
        ratingarr.insert(r.id.to_string(), Value::String(r.name));
    }
    Ok(json!({
        "rating": { "name": "admin_user_company_00018", "value": ratingarr },
        "time": { "name": "admin_user_company_00052", "value": {
            "1": "admin_tool_00622", "2": "common_01659", "3": "common_01897",
            "4": "common_01875", "5": "wap_com_00319"
        }},
    }))
}

async fn job_child_ids(state: &AppState, pid: i32) -> AppResult<Value> {
    if pid <= 0 {
        return Err(ApiError::param_invalid("pid"));
    }
    let kids = category_service::list_children(state, "job", pid as u64).await?;
    let ids: Vec<Value> = kids.iter().map(|c| Value::from(c.id)).collect();
    Ok(Value::Array(ids))
}

async fn city_child_ids(state: &AppState, pid: i32) -> AppResult<Value> {
    if pid <= 0 {
        return Err(ApiError::param_invalid("pid"));
    }
    let dicts = dict_service::get(state).await?;
    let ids: Vec<Value> = dicts
        .city_descendant_ids(pid)
        .into_iter()
        .map(Value::from)
        .collect();
    Ok(Value::Array(ids))
}

fn datacall_static() -> Value {
    json!({
        "resume": { "0": "简历" },
        "member": { "0": "用户" },
        "company": { "0": "公司" },
        "job": { "0": "职位" },
        "zph": { "0": "招聘会" },
        "news": { "0": "新闻" },
        "ask": { "0": "问答" },
        "link": { "0": "友情链接" },
        "once": { "0": "店铺招聘" },
        "tiny": { "0": "普工简历" },
        "keyword": { "0": "热门关键字" },
    })
}

fn json_str(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::String(s)) => s.trim().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn json_i32(v: &Value, key: &str) -> i32 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        Some(Value::Bool(true)) => 1,
        _ => 0,
    }
}

fn json_u64(v: &Value, key: &str) -> u64 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn json_csv(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|x| match x {
                Value::String(s) if !s.is_empty() => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::String(s)) => s.trim().to_string(),
        _ => String::new(),
    }
}

fn json_bool01(v: &Value, key: &str) -> i32 {
    match v.get(key) {
        Some(Value::Bool(true)) => 1,
        Some(Value::String(s)) if s == "true" || s == "1" => 1,
        Some(Value::Number(n)) if n.as_i64() == Some(1) => 1,
        _ => 0,
    }
}

fn php_job_description(raw: &str) -> String {
    raw.replace("&amp;", "&")
        .replace("background-color:#ffffff", "background-color:")
        .replace("background-color:#fff", "background-color:")
        .replace("white-space:nowrap;", "white-space:")
}

/// PHP `job::addJobInfo` (`utype=admin`)：新增或修改职位。
pub async fn save_admin_job(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<(&'static str, u64)> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let id = json_u64(body, "id");
    let mut name = json_str(body, "name");
    if name.is_empty() {
        if id == 0 {
            return Err(ApiError::business("member_com_00585"));
        }
        if let Some(old) = job_repo::find_by_id(state.db.reader(), id).await? {
            name = old.name;
        }
    }
    if let Some(exist) = job_repo::find_id_by_uid_name_listed(state.db.reader(), uid, &name).await?
    {
        if id == 0 || exist != id {
            return Err(ApiError::business("common_00293"));
        }
    }
    let com = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("common_06272"))?;
    if com.name.as_deref().unwrap_or("").is_empty() {
        return Err(ApiError::business("common_06272"));
    }

    let link_id_raw = json_i32(body, "link_id");
    let mut provinceid = 0;
    let mut cityid = 0;
    let mut three_cityid = 0;
    let mut x = String::new();
    let mut y = String::new();
    if link_id_raw == -1 {
        provinceid = com.provinceid;
        cityid = com.cityid;
        three_cityid = com.three_cityid;
        x = com.x.clone().unwrap_or_default();
        y = com.y.clone().unwrap_or_default();
    } else if link_id_raw > 0 {
        if let Some(addr) =
            addr_repo::find_by_id(state.db.reader(), link_id_raw as u64, uid).await?
        {
            provinceid = addr.provinceid;
            cityid = addr.cityid;
            three_cityid = addr.three_cityid;
            x = addr.x.unwrap_or_default();
            y = addr.y.unwrap_or_default();
        }
    }
    let mut is_link = json_i32(body, "is_link");
    if is_link == 0 {
        is_link = 1;
    }
    if link_id_raw > 0 && is_link == 1 {
        is_link = 2;
    }
    let is_message = {
        let n = json_i32(body, "is_message");
        if n == 0 { 1 } else { n }
    };
    let is_email = {
        let n = json_i32(body, "is_email");
        if n == 0 { 1 } else { n }
    };
    let salary_type = json_i32(body, "salary_type");
    let (minsalary, maxsalary) = if salary_type == 1 {
        (0, 0)
    } else {
        (json_i32(body, "minsalary"), json_i32(body, "maxsalary"))
    };
    let r_status = com.r_status;
    let job_state = if r_status == 1 { 1 } else { 0 };
    let rating = statis_repo::read_rating(state.db.reader(), uid).await?;
    let description = php_job_description(&json_str(body, "content"));
    let lang = json_csv(body, "checked_lang");
    let welfare = json_csv(body, "checked_welfare");
    let com_name = com.name.clone().unwrap_or_default();
    let com_logo = com.logo.clone().unwrap_or_default();
    let now = clock::now_ts();
    let exp_req = json_str(body, "exp_req");
    let edu_req = json_str(body, "edu_req");
    let write = job_repo::AdminJobWrite {
        uid,
        name: &name,
        com_name: &com_name,
        hy: json_i32(body, "hy"),
        job1: json_i32(body, "job1"),
        job1_son: json_i32(body, "job1_son"),
        job_post: json_i32(body, "job_post"),
        provinceid,
        cityid,
        three_cityid,
        x: &x,
        y: &y,
        link_id: if link_id_raw > 0 { link_id_raw } else { 0 },
        is_link,
        is_message,
        is_email,
        minsalary,
        maxsalary,
        description: &description,
        r_status,
        number: json_i32(body, "number"),
        exp: json_i32(body, "exp"),
        report: json_i32(body, "report"),
        age: json_i32(body, "age"),
        sex: json_i32(body, "sex"),
        edu: json_i32(body, "edu"),
        is_graduate: json_bool01(body, "is_graduate"),
        marriage: json_i32(body, "marriage"),
        lang: &lang,
        welfare: &welfare,
        state: job_state,
        jobhits: json_i32(body, "jobhits"),
        jobexpoure: json_i32(body, "jobexpoure"),
        exp_req: &exp_req,
        edu_req: &edu_req,
        zp_num: json_i32(body, "zp_num"),
        zp_minage: json_i32(body, "zp_minage"),
        zp_maxage: json_i32(body, "zp_maxage"),
        minage_req: json_i32(body, "minage_req"),
        maxage_req: json_i32(body, "maxage_req"),
        sex_req: json_i32(body, "sex_req"),
        status: json_i32(body, "status"),
        com_logo: &com_logo,
        com_provinceid: com.provinceid,
        pr: com.pr,
        mun: com.mun,
        did: com.did as i64,
        yyzz_status: com.yyzz_status,
        rating,
    };
    let (msg_key, job_id) = if id == 0 {
        let nid = job_repo::insert_admin(state.db.pool(), write, now).await?;
        if nid == 0 {
            return Err(ApiError::business("admin_01304"));
        }
        ("common_06273", nid)
    } else {
        let n = job_repo::update_admin(state.db.pool(), id, write).await?;
        if n == 0 {
            return Err(ApiError::business("member_user_00603"));
        }
        ("common_06274", id)
    };
    company_repo::touch_jobtime(state.db.pool(), uid, now).await?;
    Ok((msg_key, job_id))
}

/// PHP `company_job::add_action` GET (form). POST `save` 走 [`save_admin_job`]。
pub async fn job_php_add_form(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    user.require_admin()?;
    let dicts = dict_service::get(state).await?;
    let jobs = cat_nodes(state, "job").await?;
    let cities = cat_nodes(state, "city").await?;
    let mut payload = admin_dashboard_service::php_cache_payload(
        &jobs,
        &cities,
        &dicts.comclass_by_variable("job_edu"),
        &dicts.comclass_by_variable("job_exp"),
    );
    let (userdata, userclass_name) = userdata_from(&dicts);
    let (comdata, comclass_name) = comdata_from(&dicts);
    payload["cache"] = json!({
        "userdata": userdata,
        "userclass_name": userclass_name,
        "comdata": comdata,
        "comclass_name": comclass_name,
        "com_sex": { "2": "女", "3": "不限" },
        "com_sexreq": { "2": "女", "3": "不限" },
    });
    payload["cache_userdata"] = Value::Object(userdata.clone());
    payload["cache_userclassname"] = Value::Object(userclass_name);
    payload["cache_com_sexreq"] = json!({ "2": "女", "3": "不限" });
    let map_key = setting_repo::find(state.db.reader(), "map_key")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let map_secret = setting_repo::find(state.db.reader(), "map_secret")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    admin_dashboard_service::attach_amap(&mut payload, &map_key, &map_secret);

    let job_id = body.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let mut uid = body.get("uid").and_then(|v| v.as_u64()).unwrap_or(0);
    let mut show = Value::Object(Map::new());
    if job_id > 0 {
        if let Some(job) = job_repo::find_by_id(state.db.reader(), job_id).await? {
            uid = job.uid;
            let mut job_v = serde_json::to_value(&job).unwrap_or(json!({}));
            let welfare_names: Vec<String> = job
                .welfare
                .clone()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let mut all_welfare = welfare_names.clone();
            for (_, name) in dicts.comclass_by_variable("job_welfare") {
                if !all_welfare.contains(&name) {
                    all_welfare.push(name);
                }
            }
            job_v["all_welfare"] = json!(all_welfare);
            job_v["job_welfare"] = json!(welfare_names);
            job_v["arraywelfare"] = json!(welfare_names);
            job_v["is_graduate"] = json!(false);
            job_v["lang"] = json!(job
                .lang
                .clone()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>());
            show = job_v;
        }
    }
    let mut default_welfare: Vec<String> = dicts
        .comclass_by_variable("job_welfare")
        .into_iter()
        .map(|(_, n)| n)
        .collect();
    if uid > 0 && show.get("all_welfare").is_none() {
        payload["default_welfare"] = json!(default_welfare);
        payload["job_types"] = payload.get("job_types").cloned().unwrap_or(json!([]));
        payload["city_types"] = payload.get("city_types").cloned().unwrap_or(json!([]));
    } else if let Some(arr) = show.get("all_welfare") {
        default_welfare = arr
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        payload["default_welfare"] = json!(default_welfare);
    } else {
        payload["default_welfare"] = json!(default_welfare);
    }

    let mut company_v = json!({});
    if uid > 0 {
        if let Some(c) = company_repo::find_by_uid(state.db.reader(), uid).await? {
            let city_str = format!(
                "{}{}{}",
                dicts.city(c.provinceid),
                dicts.city(c.cityid),
                dicts.city(c.three_cityid)
            );
            let linkmsg = if c.linktel.as_deref().unwrap_or("").is_empty() {
                if c.linkphone.as_deref().unwrap_or("").is_empty() {
                    format!(
                        "{} - {} - {}",
                        c.linkman.as_deref().unwrap_or(""),
                        city_str,
                        c.address.as_deref().unwrap_or("")
                    )
                } else {
                    format!(
                        "{} - {} - {} - {}",
                        c.linkman.as_deref().unwrap_or(""),
                        c.linkphone.as_deref().unwrap_or(""),
                        city_str,
                        c.address.as_deref().unwrap_or("")
                    )
                }
            } else {
                format!(
                    "{} - {} - {} - {}",
                    c.linkman.as_deref().unwrap_or(""),
                    c.linktel.as_deref().unwrap_or(""),
                    city_str,
                    c.address.as_deref().unwrap_or("")
                )
            };
            company_v = json!({
                "uid": c.uid,
                "r_status": c.r_status,
                "linkman": c.linkman,
                "linktel": c.linktel,
                "linkphone": c.linkphone,
                "address": c.address,
                "provinceid": c.provinceid,
                "cityid": c.cityid,
                "three_cityid": c.three_cityid,
                "x": c.x,
                "y": c.y,
                "welfare": c.welfare,
                "linkmsg": linkmsg,
            });
        }
        let addrs = addr_repo::list_by_uid(state.db.reader(), uid, 0, 200).await?;
        payload["addressList"] = serde_json::to_value(addrs).unwrap_or(json!([]));
    } else {
        payload["addressList"] = json!([]);
    }
    payload["show"] = show;
    payload["company"] = company_v;
    payload["uid"] = json!(uid);
    Ok(payload)
}
