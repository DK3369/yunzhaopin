//! PHP admin named actions for 招聘会 / 新闻 / 问答 / 专题.
//! SQL stays in repos. Routes are `php-*` and stay out of AdminDoc.

use std::collections::HashMap;

use phpyun_core::utils::{fmt_date, fmt_dt};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::announcement::repo as announcement_repo;
use phpyun_models::article::repo::{self as article_repo, ArticleFilter};
use phpyun_models::company::repo as company_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::gongzhao::repo as gongzhao_repo;
use phpyun_models::poster_template::repo as whb_repo;
use phpyun_models::qna::repo as qna_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::special::repo as special_repo;
use phpyun_models::zph::repo as zph_repo;
use serde_json::{json, Value};

use crate::admin_cms_service;
use crate::site_setting_service;

pub enum PhpOut {
    Data(Value),
    Message(&'static str),
}

pub async fn dispatch(
    state: &AppState,
    user: &AuthenticatedUser,
    module: &str,
    action: &str,
    body: &Value,
) -> AppResult<PhpOut> {
    user.require_admin()?;
    match (module, action) {
        ("fairs", "index") => Ok(PhpOut::Data(fairs_index(state, body).await?)),
        ("fairs", "get-group") => Ok(PhpOut::Data(fairs_get_group(state).await?)),
        ("fairs", "add") => fairs_add(state, body).await,
        ("fairs", "delete") => fairs_del(state, body).await,
        ("fairs", "com") => Ok(PhpOut::Data(fairs_com(state, body).await?)),
        ("fairs", "status") => fairs_status(state, body).await,
        ("fairs", "audit") => Ok(PhpOut::Data(fairs_audit(state, body).await?)),
        ("fairs", "getjoblist") => Ok(PhpOut::Data(fairs_getjoblist(state, body).await?)),
        ("fairs", "upjob") => fairs_upjob(state, body).await,
        ("fairs", "comadd") => Ok(PhpOut::Data(fairs_comadd(state, body).await?)),
        ("fairs", "getcomlist") => Ok(PhpOut::Data(fairs_getcomlist(state, body).await?)),
        ("fairs", "getzhanwei") => Ok(PhpOut::Data(fairs_getzhanwei(state, body).await?)),
        ("fairs", "upzhanwei") => fairs_upzhanwei(state, body).await,
        ("fairs", "comaddsave") => fairs_comaddsave(state, body).await,
        ("fairs", "delcom") => fairs_delcom(state, body).await,
        ("fairs", "ajaxsort") => fairs_ajaxsort(state, body).await,
        ("fairs", "upisopen") => fairs_upisopen(state, user, body).await,
        ("fairs", "checksitedid") => fairs_checksitedid(state, body).await,
        ("news", "index") => Ok(PhpOut::Data(news_index(state, body).await?)),
        ("news", "addnews") => news_addnews(state, user, body).await,
        ("news", "delete") => news_del(state, user, body).await,
        ("news", "group") => Ok(PhpOut::Data(news_group(state).await?)),
        ("news", "addgroup") => news_addgroup(state, body).await,
        ("news", "delgroup") => news_delgroup(state, body).await,
        ("news", "ajax") => news_ajax(state, body).await,
        ("news", "recommend") => news_recommend(state, body).await,
        ("news", "changeClass") => news_change_class(state, body).await,
        ("news", "checksitedid") => news_checksitedid(state, body).await,
        ("news", "savepro") => news_savepro(state, body).await,
        ("news", "type") => Ok(PhpOut::Data(news_type(state, body).await?)),
        ("news", "property") => news_property(state, body).await,
        ("news", "delpro") => news_delpro(state, body).await,
        ("news", "delmenu") => news_delmenu(state, body).await,
        ("news", "changeSon") => news_change_son(state, body).await,
        ("gongzhao", "index") => Ok(PhpOut::Data(gongzhao_index(state, body).await?)),
        ("gongzhao", "getGroup") => Ok(PhpOut::Data(gongzhao_get_group(state).await?)),
        ("gongzhao", "add") => gongzhao_add(state, user, body).await,
        ("gongzhao", "delete") => gongzhao_del(state, body).await,
        ("gongzhao", "checksitedid") => gongzhao_checksitedid(state, body).await,
        ("gongzhao", "setRec") => gongzhao_set_rec(state, body).await,
        ("gongzhao", "whb") => Ok(PhpOut::Data(gongzhao_whb(state).await?)),
        ("announce", "getGroup") => Ok(PhpOut::Data(announce_get_group(state).await?)),
        ("announce", "checksitedid") => announce_checksitedid(state, body).await,
        ("question", "getGroup") => Ok(PhpOut::Data(question_get_group())),
        ("question", "index") => Ok(PhpOut::Data(question_index(state, body).await?)),
        ("question", "add") => Ok(PhpOut::Data(question_add(state, body).await?)),
        ("question", "save") => question_save(state, body).await,
        ("question", "delete") => question_del(state, body).await,
        ("question", "recommend") => question_recommend(state, body).await,
        ("question", "getanswer") => Ok(PhpOut::Data(question_getanswer(state, body).await?)),
        ("question", "statusAnswer") => question_status_answer(state, body).await,
        ("question", "save_answer") => question_save_answer(state, body).await,
        ("question", "delanswer") => question_delanswer(state, body).await,
        ("question", "getcomment") => Ok(PhpOut::Data(question_getcomment(state, body).await?)),
        ("question", "statusAnswerReview") => question_status_review(state, body).await,
        ("question", "save_review") => question_save_review(state, body).await,
        ("question", "delreview") => question_delreview(state, body).await,
        ("question", "config") => Ok(PhpOut::Data(question_config(state).await?)),
        ("question", "configSave") => question_config_save(state, user, body).await,
        ("special", "index") => Ok(PhpOut::Data(special_index(state, body).await?)),
        ("special", "add") => special_add(state, body).await,
        ("special", "delete") => special_del(state, body).await,
        ("special", "setOrder") => special_set_order(state, body).await,
        ("special", "recommend") => special_recommend(state, body).await,
        ("special", "ajaxsort") => special_ajaxsort(state, body).await,
        ("special", "setFamous") => special_set_famous(state, body).await,
        ("special", "addlist") => Ok(PhpOut::Data(special_addlist(state, body).await?)),
        ("special", "set_comaddsearch") => Ok(PhpOut::Data(special_comaddsearch(state).await?)),
        ("special", "audit") => Ok(PhpOut::Data(special_audit(state, body).await?)),
        ("special", "comjob") => Ok(PhpOut::Data(special_comjob(state, body).await?)),
        _ => Err(ApiError::param_invalid("unknown_php_action")),
    }
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

fn json_u64_val(v: &Value) -> u64 {
    match v {
        Value::Number(n) => n.as_u64().unwrap_or(0),
        Value::String(s) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn has_flag(body: &Value, key: &str) -> bool {
    match body.get(key) {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        Some(Value::String(s)) => !s.is_empty() && s != "0",
        Some(_) => true,
    }
}

fn ids_of(body: &Value) -> Vec<u64> {
    let raw = body
        .get("del")
        .or_else(|| body.get("id"))
        .or_else(|| body.get("ids"))
        .or_else(|| body.get("pid"))
        .or_else(|| body.get("uid"));
    match raw {
        Some(Value::Array(a)) => a
            .iter()
            .map(json_u64_val)
            .filter(|n| *n > 0)
            .collect(),
        Some(Value::String(s)) => s
            .split([',', ';'])
            .filter_map(|x| x.trim().parse().ok())
            .filter(|n: &u64| *n > 0)
            .collect(),
        Some(Value::Number(n)) => n.as_u64().filter(|n| *n > 0).into_iter().collect(),
        _ => Vec::new(),
    }
}

fn page_of(body: &Value) -> (u32, u32, u64, u64) {
    let page = json_u64(body, "page").max(1) as u32;
    let mut per = json_u64(body, "pageSize");
    if per == 0 {
        per = json_u64(body, "page_size");
    }
    if per == 0 {
        per = json_u64(body, "limit");
    }
    if per == 0 {
        per = json_u64(body, "perPage");
    }
    if per == 0 {
        per = 20;
    }
    let per = per.clamp(1, 100) as u32;
    let offset = u64::from(page.saturating_sub(1)) * u64::from(per);
    (page, per, offset, u64::from(per))
}

fn paged(list: Value, total: u64, page: u32, per: u32) -> Value {
    let sizes = vec![10, 20, 50, 100];
    json!({
        "list": list,
        "total": total,
        "perPage": per,
        "pageSize": per,
        "pageSizes": sizes,
        "page_sizes": sizes,
        "limit": per,
        "page": page,
    })
}

fn amp(s: &str) -> String {
    s.replace("&amp;", "&")
}

fn preview_base(state: &AppState) -> String {
    state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://zzzz.com".into())
}

fn domain_object(rows: &[phpyun_models::domain::entity::DomainSite]) -> Value {
    let mut m = serde_json::Map::new();
    for d in rows {
        m.insert(d.id.to_string(), Value::String(d.title.clone()));
    }
    Value::Object(m)
}

async fn fairs_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = zph_repo::AdminZphListFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
        status: json_i32(body, "status"),
    };
    let now = clock::now_ts();
    let db = state.db.reader();
    let rows = zph_repo::admin_list_filtered(db, &f, now, offset, limit).await?;
    let total = zph_repo::admin_count_filtered(db, &f, now).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "address": r.address,
                "starttime": r.starttime,
                "endtime": r.endtime,
                "did": r.did,
                "is_open": r.is_open.to_string(),
                "sid": r.sid,
                "reserved": r.reserved,
                "comnum": r.comnum,
                "booking": r.booking,
                "url": format!("{base}/index.php?m=zph&c=show&id={}", r.id),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn fairs_get_group(state: &AppState) -> AppResult<Value> {
    let spaces = zph_repo::list_spaces(state.db.reader(), None, None).await?;
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "preview_url": format!("{}/index.php?m=zph&c=show&id=", preview_base(state)),
        "Dname": domain_object(&domains),
        "space": spaces,
    }))
}

async fn fairs_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if has_flag(body, "submit") {
        let title = json_str(body, "title");
        if title.is_empty() {
            return Err(ApiError::business("admin_01351"));
        }
        let start = json_str(body, "starttime");
        let end = json_str(body, "endtime");
        if !start.is_empty() && !end.is_empty() {
            let st = chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M"))
                .ok()
                .map(|d| d.and_utc().timestamp())
                .unwrap_or(0);
            let et = chrono::NaiveDateTime::parse_from_str(&end, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&end, "%Y-%m-%d %H:%M"))
                .ok()
                .map(|d| d.and_utc().timestamp())
                .unwrap_or(0);
            if st > 0 && et > 0 && st > et {
                return Err(ApiError::business("admin_neirong_00027"));
            }
        }
        let reserved = reserved_csv(body);
        let id = json_u64(body, "id");
        let nid = zph_repo::upsert_info(
            state.db.pool(),
            zph_repo::ZphInfoWrite {
                id: if id > 0 { Some(id) } else { None },
                title: &title,
                sid: json_i32(body, "sid"),
                address: &json_str(body, "address"),
                traffic: &json_str(body, "traffic"),
                phone: &json_str(body, "phone"),
                organizers: &json_str(body, "organizers"),
                user: &json_str(body, "user"),
                starttime: &start,
                endtime: &end,
                body: &amp(&json_str(body, "body")),
                media: &amp(&json_str(body, "media")),
                packages: &amp(&json_str(body, "packages")),
                booth: &amp(&json_str(body, "booth")),
                participate: &amp(&json_str(body, "participate")),
                did: json_i32(body, "did"),
                reserved: &reserved,
                is_open: json_i32(body, "is_open"),
                is_themb: &json_str(body, "is_themb"),
                banner: &json_str(body, "banner"),
                is_themb_wap: &json_str(body, "is_themb_wap"),
                banner_wap: &json_str(body, "banner_wap"),
                now: clock::now_ts(),
            },
        )
        .await?;
        return Ok(PhpOut::Message(if id > 0 {
            "admin_model_00025"
        } else {
            let _ = nid;
            "admin_model_00027"
        }));
    }
    let domains = domain_repo::list_all(state.db.reader()).await?;
    let spaces = zph_repo::list_spaces(state.db.reader(), None, None).await?;
    let id = json_u64(body, "id");
    let info = if id > 0 {
        zph_repo::find_admin_form(state.db.reader(), id)
            .await?
            .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
            .unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    Ok(PhpOut::Data(json!({
        "Dname": domain_object(&domains),
        "info": info,
        "space": spaces,
    })))
}

fn reserved_csv(body: &Value) -> String {
    if let Some(arr) = body.get("reserved_arr").and_then(|v| v.as_array()) {
        return arr
            .iter()
            .filter_map(|item| {
                let pair = item.as_array()?;
                pair.get(1).map(json_u64_val).filter(|n| *n > 0)
            })
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
    }
    json_str(body, "reserved")
}

async fn fairs_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    zph_repo::delete_zph_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("admin_model_00031"))
}

async fn fairs_com(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let st = json_i32(body, "status");
    let status = match st {
        3 => Some(0),
        1 => Some(1),
        2 => Some(2),
        _ => None,
    };
    let kw = json_str(body, "keyword");
    let f = zph_repo::AdminZphComFilter {
        zid: Some(json_u64(body, "id")).filter(|n| *n > 0),
        status,
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
    };
    let db = state.db.reader();
    let rows = zph_repo::admin_list_coms(db, &f, offset, limit).await?;
    let total = zph_repo::admin_count_coms(db, &f).await?;
    let names: HashMap<u64, String> = zph_repo::space_name_map(db)
        .await?
        .into_iter()
        .collect();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let space_n = format!(
                "{} - {} - {}",
                names.get(&(r.sid as u64)).cloned().unwrap_or_default(),
                names.get(&(r.cid as u64)).cloned().unwrap_or_default(),
                names.get(&(r.bid as u64)).cloned().unwrap_or_default()
            );
            json!({
                "id": r.id,
                "uid": r.uid,
                "zid": r.zid,
                "jobid": r.jobid,
                "ctime": r.ctime,
                "status": r.status,
                "statusbody": r.statusbody,
                "sid": r.sid,
                "cid": r.cid,
                "bid": r.bid,
                "price": r.price,
                "com_name": r.com_name,
                "sort": r.sort,
                "space_n": space_n,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn fairs_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    zph_repo::update_coms_status(
        state.db.pool(),
        &ids,
        json_i32(body, "status"),
        &json_str(body, "statusbody"),
    )
    .await?;
    Ok(PhpOut::Message("admin_model_00032"))
}

async fn fairs_audit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let com = zph_repo::find_com_admin(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("zph_com_not_found"))?;
    let mut zph = json!({
        "id": com.id,
        "uid": com.uid,
        "status": com.status,
        "statusbody": com.statusbody,
        "jobid": com.jobid,
        "zid": com.zid,
    });
    if has_flag(body, "zph_info") {
        if let Some(z) = zph_repo::find_by_id(state.db.reader(), com.zid).await? {
            zph["title"] = json!(z.title);
        }
    }
    let info = company_repo::find_by_uid(state.db.reader(), com.uid).await?;
    let jobs = zph_repo::job_labels_for_uid(state.db.reader(), com.uid).await?;
    let jobid_arr: Vec<String> = com
        .jobid
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let job_list: Vec<Value> = jobs
        .into_iter()
        .map(|(jid, name)| {
            let sel = jobid_arr.iter().any(|x| x == &jid.to_string());
            json!({
                "id": jid,
                "name": name,
                "ch_n": if sel || jobid_arr.is_empty() { "admin_00302" } else { "admin_neirong_00032" },
            })
        })
        .collect();
    Ok(json!({
        "name": info.as_ref().and_then(|c| c.name.clone()).unwrap_or_default(),
        "uid": com.uid,
        "jobid_arr": jobid_arr,
        "job_list": job_list,
        "zph": zph,
    }))
}

async fn fairs_getjoblist(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "comid");
    let jobs = zph_repo::job_labels_for_uid(state.db.reader(), uid).await?;
    Ok(Value::Array(
        jobs.into_iter()
            .map(|(id, name)| json!({ "value": id, "label": name }))
            .collect(),
    ))
}

async fn fairs_upjob(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let job = json_str(body, "zphjob");
    if job.is_empty() {
        return Err(ApiError::business("admin_01356"));
    }
    let n = zph_repo::update_com_fields(
        state.db.pool(),
        json_u64(body, "zcomid"),
        Some(&job),
        None,
        None,
        None,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_01355"));
    }
    Ok(PhpOut::Message("admin_01354"))
}

async fn fairs_comadd(state: &AppState, body: &Value) -> AppResult<Value> {
    let zid = json_u64(body, "id");
    let zph = zph_repo::find_by_id(state.db.reader(), zid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("fair_not_found"))?;
    let space = zph_repo::space_children(state.db.reader(), i64::from(zph.sid)).await?;
    Ok(json!({ "spacelist": space }))
}

async fn fairs_getcomlist(state: &AppState, body: &Value) -> AppResult<Value> {
    let name = json_str(body, "comname");
    let rows = zph_repo::search_company_labels(state.db.reader(), &name, 20).await?;
    Ok(Value::Array(
        rows.into_iter()
            .map(|(uid, n)| json!({ "label": n, "value": uid }))
            .collect(),
    ))
}

async fn fairs_getzhanwei(state: &AppState, body: &Value) -> AppResult<Value> {
    let sid = json_i32(body, "sid");
    let zid = json_u64(body, "zid");
    if sid == 0 {
        return Ok(json!({ "reserved_arr": [], "space": [] }));
    }
    let taken = zph_repo::taken_bids(state.db.reader(), zid).await?;
    let zph = zph_repo::find_admin_form(state.db.reader(), zid).await?;
    let reserved: Vec<u64> = zph
        .as_ref()
        .map(|z| {
            z.reserved
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .filter(|n: &u64| *n > 0)
                .collect()
        })
        .unwrap_or_default();
    let pairs = zph_repo::reserved_parent_pairs(state.db.reader(), &reserved).await?;
    let parent: HashMap<u64, i64> = pairs.into_iter().collect();
    let reserved_arr: Vec<Value> = reserved
        .iter()
        .map(|id| json!([parent.get(id).copied().unwrap_or(0), *id]))
        .collect();
    let halls = zph_repo::space_children(state.db.reader(), i64::from(sid)).await?;
    let mut space = Vec::new();
    for h in halls {
        let booths = zph_repo::space_children(state.db.reader(), h.id as i64).await?;
        let children: Vec<Value> = booths
            .into_iter()
            .map(|b| {
                let mut o = json!({ "value": b.id, "label": b.name });
                if taken.contains(&(b.id as i32)) {
                    o["disabled"] = json!(true);
                }
                o
            })
            .collect();
        space.push(json!({
            "value": h.id,
            "label": h.name,
            "children": children,
        }));
    }
    Ok(json!({ "reserved_arr": reserved_arr, "space": space }))
}

async fn fairs_upzhanwei(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "zcomid");
    if id == 0 {
        return Err(ApiError::business("admin_neirong_00031"));
    }
    zph_repo::update_com_fields(
        state.db.pool(),
        id,
        None,
        Some(json_i32(body, "cid")),
        Some(json_i32(body, "bid")),
        None,
    )
    .await?;
    Ok(PhpOut::Message("admin_user_company_00208"))
}

async fn fairs_comaddsave(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "comid");
    let zid = json_u64(body, "zphid");
    if zph_repo::find_my_reservation(state.db.reader(), zid, uid)
        .await?
        .is_some()
    {
        return Err(ApiError::business("admin_neirong_00028"));
    }
    let name = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .and_then(|c| c.name)
        .unwrap_or_default();
    zph_repo::insert_zph_com(
        state.db.pool(),
        zid,
        uid,
        json_i32(body, "zphsid"),
        json_i32(body, "cid"),
        json_i32(body, "bid"),
        &json_str(body, "jobid"),
        &name,
        clock::now_ts(),
    )
    .await?;
    Ok(PhpOut::Message("admin_model_00035"))
}

async fn fairs_delcom(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    zph_repo::delete_coms(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn fairs_ajaxsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    zph_repo::update_com_fields(
        state.db.pool(),
        id,
        None,
        None,
        None,
        Some(json_i32(body, "sort")),
    )
    .await?;
    Ok(PhpOut::Message("admin_user_company_00208"))
}

async fn fairs_upisopen(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "pid");
    if id == 0 {
        return Err(ApiError::business("admin_neirong_00029"));
    }
    admin_cms_service::set_fair_open(state, user, id, json_i32(body, "is_open")).await?;
    Ok(PhpOut::Message("ok"))
}

async fn fairs_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    zph_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00033"))
}

fn nid_from_body(body: &Value) -> Option<String> {
    if let Some(arr) = body.get("cate").and_then(|v| v.as_array()) {
        if arr.len() >= 2 {
            let n = json_u64_val(&arr[1]);
            if n > 0 {
                return Some(n.to_string());
            }
        } else if let Some(first) = arr.first() {
            let n = json_u64_val(first);
            if n > 0 {
                return Some(n.to_string());
            }
        }
    }
    let cates = json_u64(body, "cates");
    if cates > 0 {
        Some(cates.to_string())
    } else {
        None
    }
}

fn days_ago_ts(days: i32) -> i64 {
    let now = clock::now_ts();
    if days <= 1 {
        let day = now - (now % 86_400);
        return day;
    }
    now - i64::from(days) * 86_400
}

fn parse_date_ts(s: &str) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    if let Ok(n) = s.parse::<i64>() {
        return n;
    }
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or(0)
}

fn pic_url(base: &str, pic: &str) -> String {
    if pic.is_empty() || pic.starts_with("http") {
        pic.to_string()
    } else {
        format!("{}/{}", base.trim_end_matches('/'), pic.trim_start_matches('/'))
    }
}

async fn news_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ty = json_i32(body, "type");
    let nid = nid_from_body(body);
    let publish = json_i32(body, "publish");
    let adtime = json_i32(body, "adtime");
    let days = if publish > 0 { publish } else { adtime };
    let f = ArticleFilter {
        category: nid.as_deref(),
        keyword: if ty != 2 && !kw.is_empty() {
            Some(kw.as_str())
        } else {
            None
        },
        rec_only: false,
        did: 0,
        datetime_min: if days > 0 { Some(days_ago_ts(days)) } else { None },
        author_kw: if ty == 2 && !kw.is_empty() {
            Some(kw.as_str())
        } else {
            None
        },
    };
    let db = state.db.reader();
    let rows = article_repo::list_admin(db, &f, offset, limit).await?;
    let total = article_repo::count_admin(db, &f).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|a| {
            json!({
                "id": a.id,
                "title": a.title,
                "color": a.color,
                "author": a.author,
                "nid": a.nid,
                "did": a.did,
                "hits": a.hits,
                "datetime": a.published_at,
                "datetime_n": fmt_dt(a.published_at),
                "describe": a.describe,
                "url": format!("{base}/index.php?m=news&c=show&id={}", a.id),
                "titype": "",
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn news_addnews(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    if has_flag(body, "add") {
        let id = json_u64(body, "id");
        let content = if id > 0 {
            article_repo::find_content(state.db.reader(), id)
                .await?
                .unwrap_or_default()
        } else {
            String::new()
        };
        return Ok(PhpOut::Data(json!({ "content": content })));
    }
    let title = json_str(body, "title");
    let nid = json_i32(body, "nid");
    let content = json_str(body, "content");
    if title.is_empty() {
        return Err(ApiError::business("admin_01332"));
    }
    if nid <= 0 {
        return Err(ApiError::business("admin_01333"));
    }
    if content.is_empty() {
        return Err(ApiError::business("admin_01334"));
    }
    admin_cms_service::upsert_article(
        state,
        user,
        admin_cms_service::ArticleUpsertIn {
            id: Some(json_u64(body, "id")).filter(|n| *n > 0),
            title: &title,
            nid,
            content: &amp(&content),
            author: &json_str(body, "author"),
            description: &json_str(body, "description"),
            keyword: &json_str(body, "keyword"),
            source: &json_str(body, "source"),
            newsphoto: &json_str(body, "newsphoto"),
            did: json_i32(body, "did"),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn news_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    article_repo::delete_ids(state.db.pool(), &ids).await?;
    let _ = user;
    Ok(PhpOut::Message("ok"))
}

async fn news_group(state: &AppState) -> AppResult<Value> {
    let groups = article_repo::list_groups_admin(state.db.reader()).await?;
    let counts: HashMap<i32, i64> = article_repo::news_count_by_nid(state.db.reader())
        .await?
        .into_iter()
        .collect();
    let mut by_id: HashMap<u64, Value> = HashMap::new();
    for g in &groups {
        by_id.insert(
            g.id,
            json!({
                "id": g.id,
                "name": g.name,
                "keyid": g.keyid,
                "sort": g.sort,
                "rec": g.rec == 1,
                "rec_news": g.rec_news == 1,
                "is_menu": g.is_menu,
                "roots": 0,
                "count": counts.get(&(g.id as i32)).copied().unwrap_or(0),
                "children": [],
            }),
        );
    }
    for g in &groups {
        if g.keyid > 0 {
            if let Some(parent) = by_id.get_mut(&(g.keyid as u64)) {
                if let Some(c) = parent.get("count").and_then(|v| v.as_i64()) {
                    let add = counts.get(&(g.id as i32)).copied().unwrap_or(0);
                    parent["count"] = json!(c + add);
                }
                if let Some(roots) = parent.get("roots").and_then(|v| v.as_i64()) {
                    parent["roots"] = json!(roots + 1);
                }
            }
        }
    }
    let mut list = Vec::new();
    for g in groups {
        if g.keyid == 0 {
            if let Some(mut row) = by_id.remove(&g.id) {
                let kids: Vec<Value> = by_id
                    .iter()
                    .filter(|(_, v)| v.get("keyid").and_then(|x| x.as_i64()) == Some(i64::from(g.id as i32)))
                    .map(|(_, v)| v.clone())
                    .collect();
                // rebuild children from remaining with this keyid
                let children: Vec<Value> = kids;
                row["children"] = json!(children);
                list.push(row);
            }
        }
    }
    Ok(json!({ "list": list, "type": [] }))
}

async fn news_addgroup(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let raw = json_str(body, "name");
    let names: Vec<&str> = raw.split('-').map(str::trim).filter(|s| !s.is_empty()).collect();
    if names.is_empty() {
        return Err(ApiError::business("admin_01200"));
    }
    let fid = json_i32(body, "fid");
    let rec = json_i32(body, "rec");
    for name in names {
        article_repo::insert_group(state.db.pool(), name, fid, rec).await?;
    }
    Ok(PhpOut::Message("admin_01335"))
}

async fn news_delgroup(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    article_repo::delete_groups(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn news_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let name = json_str(body, "name");
    if !name.is_empty() {
        article_repo::patch_group(state.db.pool(), id, Some(&name), None, None, None).await?;
    }
    if body.get("sort").is_some() {
        article_repo::patch_group(state.db.pool(), id, None, Some(json_i32(body, "sort")), None, None)
            .await?;
    }
    Ok(PhpOut::Message("admin_model_00179"))
}

async fn news_recommend(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let rec = json_i32(body, "rec");
    let ty = json_str(body, "type");
    if ty == "rec_news" {
        article_repo::patch_group(state.db.pool(), id, None, None, None, Some(rec)).await?;
    } else {
        article_repo::patch_group(state.db.pool(), id, None, None, Some(rec), None).await?;
    }
    Ok(PhpOut::Message("ok"))
}

async fn news_change_class(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    article_repo::set_nid_ids(state.db.pool(), &ids, json_i32(body, "nid")).await?;
    Ok(PhpOut::Message("admin_neirong_00015"))
}

async fn news_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    article_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00175"))
}

async fn news_savepro(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = match body.get("proid") {
        Some(Value::Array(a)) => a.iter().map(json_u64_val).filter(|n| *n > 0).collect(),
        Some(Value::String(s)) => s
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .filter(|n: &u64| *n > 0)
            .collect(),
        Some(Value::Number(n)) => n.as_u64().filter(|n| *n > 0).into_iter().collect(),
        _ => Vec::new(),
    };
    if ids.is_empty() {
        return Err(ApiError::business("wap_01298"));
    }
    let describe_add = match body.get("describe") {
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    };
    let ty = json_str(body, "type");
    if ty == "add" {
        if describe_add.is_empty() {
            return Err(ApiError::business("admin_01331"));
        }
        for id in ids {
            article_repo::set_describe(state.db.pool(), id, &describe_add).await?;
        }
        return Ok(PhpOut::Message("admin_model_00176"));
    }
    let drop: Vec<String> = describe_add.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let rows = article_repo::list_describe(state.db.pool(), &ids).await?;
    for (id, cur) in rows {
        let next: Vec<&str> = cur
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty() && !drop.iter().any(|d| d == s))
            .collect();
        article_repo::set_describe(state.db.pool(), id, &next.join(",")).await?;
    }
    Ok(PhpOut::Message("admin_model_00177"))
}

async fn news_type(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let rows = article_repo::list_properties(
        state.db.reader(),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
        offset,
        limit,
    )
    .await?;
    let total = article_repo::count_properties(
        state.db.reader(),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
    )
    .await?;
    Ok(paged(serde_json::to_value(rows).unwrap_or(json!([])), total, page, per))
}

async fn news_property(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    let value = json_str(body, "value");
    article_repo::upsert_property(
        state.db.pool(),
        Some(json_u64(body, "id")).filter(|n| *n > 0),
        &name,
        &value,
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn news_delpro(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    article_repo::delete_properties(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn news_delmenu(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("member_com_00320"));
    }
    article_repo::set_group_is_menu(state.db.pool(), id, 0).await?;
    Ok(PhpOut::Message("ok"))
}

async fn news_change_son(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    let nid = json_i32(body, "nid");
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    if ids.iter().any(|id| *id == nid as u64) {
        return Err(ApiError::business("admin_neirong_00011"));
    }
    article_repo::set_group_keyid(state.db.pool(), &ids, nid).await?;
    Ok(PhpOut::Message("admin_neirong_00022"))
}

async fn gongzhao_get_group(state: &AppState) -> AppResult<Value> {
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "Dname": domain_object(&domains),
        "today": fmt_date(clock::now_ts()),
    }))
}

async fn gongzhao_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let end = json_i32(body, "end");
    let order_col = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gongzhao_repo::GongzhaoAdminFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        datetime_min: if end > 0 { Some(days_ago_ts(end)) } else { None },
        order_col: if order_col.is_empty() { "id" } else { order_col.as_str() },
        order_dir: if order_dir.is_empty() { "desc" } else { order_dir.as_str() },
    };
    let db = state.db.reader();
    let rows = gongzhao_repo::list_admin(db, &f, offset, limit).await?;
    let total = gongzhao_repo::count_admin(db, &f).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "keyword": r.keyword,
                "description": r.description,
                "content": r.content,
                "pic": r.pic,
                "pic_n": pic_url(&base, &r.pic),
                "datetime": r.datetime,
                "datetime_n": fmt_date(r.datetime),
                "startime": r.startime,
                "startime_n": fmt_date(r.startime),
                "endtime": r.endtime,
                "endtime_n": fmt_date(r.endtime),
                "did": r.did.to_string(),
                "rec": r.rec,
                "isRec": r.rec == 1,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn gongzhao_add(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    if has_flag(body, "add") {
        return Ok(PhpOut::Data(json!({})));
    }
    let title = json_str(body, "title");
    if title.is_empty() {
        return Err(ApiError::business("admin_01329"));
    }
    let start_s = json_str(body, "startime_n");
    let end_s = json_str(body, "endtime_n");
    let startime = if start_s.is_empty() {
        days_ago_ts(1)
    } else {
        parse_date_ts(&start_s)
    };
    let endtime = parse_date_ts(&end_s);
    admin_cms_service::upsert_gongzhao(
        state,
        user,
        admin_cms_service::GongzhaoUpsertIn {
            id: Some(json_u64(body, "id")).filter(|n| *n > 0),
            title: &title,
            keyword: &json_str(body, "keyword"),
            description: &json_str(body, "description"),
            content: &amp(&json_str(body, "content")),
            pic: &json_str(body, "pic"),
            startime,
            endtime,
            did: json_i32(body, "did"),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn gongzhao_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    gongzhao_repo::delete_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn gongzhao_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    gongzhao_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00192"))
}

async fn gongzhao_set_rec(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "del");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let rec = json_i32(body, "rec");
    gongzhao_repo::set_rec(state.db.pool(), id, if rec == 1 { 1 } else { 0 }).await?;
    Ok(PhpOut::Message("ok"))
}

async fn gongzhao_whb(state: &AppState) -> AppResult<Value> {
    let base = preview_base(state);
    let rows = whb_repo::list_admin_by_type(state.db.reader(), 4).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .filter(|r| r.isopen == 1)
        .map(|r| {
            json!({
                "id": r.id,
                "name": r.name,
                "pic": r.pic,
                "pic_n": pic_url(&base, &r.pic),
                "sort": r.sort,
                "isopen": r.isopen,
            })
        })
        .collect();
    Ok(Value::Array(list))
}

async fn announce_get_group(state: &AppState) -> AppResult<Value> {
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "search_list": [
            {"param": "end", "name": "admin_user_weipin_00030", "value": {
                "1": "common_01940",
                "3": "admin_user_00179",
                "7": "admin_user_00178",
                "15": "admin_user_00180",
                "30": "admin_user_00175",
            }},
        ],
        "domainList": domain_object(&domains),
    }))
}

async fn announce_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    announcement_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00191"))
}

fn question_get_group() -> Value {
    json!({
        "search_list": [
            {"param": "is_recom", "name": "admin_00231", "value": {"1": "admin_01339", "2": "admin_system_00448"}},
            {"param": "status", "name": "wap_com_00406", "value": {"0": "wap_user_00166", "1": "wap_user_00165", "2": "wap_user_00167"}},
            {"param": "end", "name": "admin_00251", "value": {"1": "common_01940", "3": "admin_user_00179", "7": "admin_user_00178", "15": "admin_user_00180", "30": "admin_user_00175"}},
        ]
    })
}

async fn question_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let mut recom = None;
    match json_i32(body, "is_recom") {
        1 => recom = Some(1),
        2 => recom = Some(0),
        _ => {}
    }
    let status = if body.get("status").is_some() && json_str(body, "status") != "" {
        Some(json_i32(body, "status"))
    } else {
        None
    };
    let f = qna_repo::AdminQuestionFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        status,
        is_recom: recom,
    };
    let db = state.db.reader();
    let rows = qna_repo::admin_list_questions(db, &f, offset, limit).await?;
    let total = qna_repo::admin_count_questions(db, &f).await?;
    let classes = qna_repo::list_qclasses(db).await?;
    let class_map: HashMap<u64, String> = classes.into_iter().map(|c| (c.id, c.name)).collect();
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|q| {
            json!({
                "id": q.id,
                "title": q.title,
                "nickname": q.nickname,
                "cid": q.category_id,
                "classname": class_map.get(&(q.category_id as u64)).cloned().unwrap_or_default(),
                "state": q.status,
                "status": q.status,
                "is_recom": q.is_recom,
                "answer_num": q.answer_count,
                "add_time": q.created_at,
                "add_time_n": fmt_dt(q.created_at),
                "ask_url": format!("{base}/index.php?m=ask&c=content&id={}", q.id),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn question_add(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let info = if id > 0 {
        qna_repo::find_question(state.db.reader(), id)
            .await?
            .map(|q| serde_json::to_value(q).unwrap_or(Value::Null))
            .unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    let classes = qna_repo::list_qclasses(state.db.reader()).await?;
    let mut parents: HashMap<u64, Value> = HashMap::new();
    for c in &classes {
        if c.pid == 0 {
            parents.insert(
                c.id,
                json!({ "id": c.id, "name": c.name, "pid": c.pid, "children": [] }),
            );
        }
    }
    for c in &classes {
        if c.pid > 0 {
            if let Some(p) = parents.get_mut(&(c.pid as u64)) {
                if let Some(arr) = p.get_mut("children").and_then(|v| v.as_array_mut()) {
                    arr.push(json!({ "id": c.id, "name": c.name, "pid": c.pid }));
                }
            }
        }
    }
    let class_list: Vec<Value> = parents.into_values().collect();
    Ok(json!({ "info": info, "classList": class_list }))
}

async fn question_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let title = json_str(body, "title");
    let cid = json_i32(body, "cid");
    if title.is_empty() || cid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::upsert_question_admin(
        state.db.pool(),
        json_u64(body, "id"),
        &title,
        cid,
        &amp(&json_str(body, "content")),
        clock::now_ts(),
    )
    .await?;
    Ok(PhpOut::Message("admin_model_00008"))
}

async fn question_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::admin_delete_questions(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn question_recommend(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::set_question_recom(state.db.pool(), id, json_i32(body, "rec")).await?;
    Ok(PhpOut::Message("admin_model_00007"))
}

async fn question_getanswer(state: &AppState, body: &Value) -> AppResult<Value> {
    let qid = json_u64(body, "id");
    let aid = json_u64(body, "aid");
    let status = if body.get("status").is_some() && json_str(body, "status") != "" {
        Some(json_i32(body, "status"))
    } else {
        None
    };
    let list = qna_repo::list_answers_admin(
        state.db.reader(),
        if qid > 0 { Some(qid) } else { None },
        if aid > 0 { Some(aid) } else { None },
        status,
    )
    .await?;
    let ques = if qid > 0 {
        qna_repo::find_question(state.db.reader(), qid)
            .await?
            .map(|q| serde_json::to_value(q).unwrap_or(Value::Null))
            .unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    Ok(json!({ "list": list, "ques": ques }))
}

async fn question_status_answer(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let status = json_i32(body, "status");
    if id == 0 || status == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::set_answer_status(state.db.pool(), id, status, &json_str(body, "statusbody")).await?;
    Ok(PhpOut::Message("ok"))
}

async fn question_save_answer(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let content = json_str(body, "content");
    if id == 0 || content.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::update_answer_admin(state.db.pool(), id, &amp(&content), json_i32(body, "support")).await?;
    Ok(PhpOut::Message("admin_model_00011"))
}

async fn question_delanswer(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let qid = json_u64(body, "qid");
    let ids = ids_of(body);
    if ids.is_empty() || qid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = ids.len() as i32;
    qna_repo::delete_answers(state.db.pool(), &ids).await?;
    qna_repo::decr_answer_num(state.db.pool(), qid, n).await?;
    Ok(PhpOut::Message("admin_model_00012"))
}

async fn question_getcomment(state: &AppState, body: &Value) -> AppResult<Value> {
    let aid = json_u64(body, "aid");
    let id = json_u64(body, "id");
    let status = if body.get("status").is_some() && json_str(body, "status") != "" {
        Some(json_i32(body, "status"))
    } else {
        None
    };
    let list = qna_repo::list_reviews_admin(
        state.db.reader(),
        if aid > 0 { Some(aid) } else { None },
        if id > 0 && aid == 0 { Some(id) } else { None },
        status,
    )
    .await?;
    Ok(json!({ "list": list, "answer": Value::Null }))
}

async fn question_status_review(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let status = json_i32(body, "status");
    if id == 0 || status == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::set_review_status(state.db.pool(), id, status, &json_str(body, "statusbody")).await?;
    Ok(PhpOut::Message("ok"))
}

async fn question_save_review(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let content = json_str(body, "content");
    if id == 0 || content.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::update_review_content(state.db.pool(), id, &content).await?;
    Ok(PhpOut::Message("admin_model_00013"))
}

async fn question_delreview(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    qna_repo::delete_reviews(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("admin_model_00014"))
}

async fn question_config(state: &AppState) -> AppResult<Value> {
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let mut m = serde_json::Map::new();
    for r in rows {
        m.insert(r.key_name, json!(r.value));
    }
    Ok(json!({
        "config": {
            "sy_day_ask_num": m.get("sy_day_ask_num").cloned().unwrap_or(json!("")),
            "sy_ip_ask_num": m.get("sy_ip_ask_num").cloned().unwrap_or(json!("")),
            "ask_check": m.get("ask_check").cloned().unwrap_or(json!("")),
            "answer_check": m.get("answer_check").cloned().unwrap_or(json!("")),
            "answer_review_check": m.get("answer_review_check").cloned().unwrap_or(json!("")),
            "sy_friend_icon_n": m.get("sy_friend_icon").cloned().unwrap_or(json!("")),
        }
    }))
}

async fn question_config_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    for key in [
        "sy_day_ask_num",
        "sy_ip_ask_num",
        "ask_check",
        "answer_check",
        "answer_review_check",
    ] {
        if body.get(key).is_some() {
            site_setting_service::admin_upsert(
                state,
                user,
                site_setting_service::UpsertInput {
                    key,
                    value: &json_str(body, key),
                    description: "",
                    is_public: true,
                },
            )
            .await?;
        }
    }
    Ok(PhpOut::Message("ok"))
}

async fn special_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let db = state.db.reader();
    let rows = special_repo::list_admin_kw(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        offset,
        limit,
    )
    .await?;
    let total = special_repo::count_admin_kw(db, if kw.is_empty() { None } else { Some(kw.as_str()) }).await?;
    let base = preview_base(state);
    let mut list = Vec::new();
    for s in rows {
        let (comnum, booking) = special_repo::count_coms_by_sid(db, s.id).await?;
        list.push(json!({
            "id": s.id,
            "title": s.title,
            "title_href": format!("{base}/index.php?m=special&c=show&id={}", s.id),
            "tpl": s.tpl,
            "limit": s.max_count,
            "display": s.status,
            "display_switch": s.status == 1,
            "sort": s.sort,
            "comnum": comnum,
            "booking": booking,
        }));
    }
    Ok(paged(Value::Array(list), total, page, per))
}

async fn special_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if has_flag(body, "add") {
        let id = json_u64(body, "id");
        if id > 0 {
            if let Some(row) = special_repo::find(state.db.reader(), id).await? {
                let mut v = serde_json::to_value(&row).unwrap_or(json!({}));
                let rating: Vec<&str> = row.rating.split(',').filter(|s| !s.is_empty()).collect();
                v["rating"] = json!(rating);
                v["etime"] = if row.end_at > 0 {
                    json!(chrono::DateTime::from_timestamp(row.end_at, 0)
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_default())
                } else {
                    json!("")
                };
                v["limit"] = json!(row.max_count);
                v["display"] = json!(row.status);
                return Ok(PhpOut::Data(v));
            }
        }
        return Ok(PhpOut::Data(json!({})));
    }
    let title = json_str(body, "title");
    let tpl = json_str(body, "tpl");
    if title.is_empty() {
        return Err(ApiError::business("admin_01439"));
    }
    if tpl.is_empty() {
        return Err(ApiError::business("admin_01440"));
    }
    let rating = match body.get("rating") {
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()).or_else(|| {
                v.as_i64().map(|n| n.to_string())
            }))
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    };
    let etime_s = json_str(body, "etime");
    let etime = chrono::NaiveDate::parse_from_str(&etime_s, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc().timestamp())
        .unwrap_or(0);
    special_repo::upsert_special(
        state.db.pool(),
        special_repo::SpecialWrite {
            id: Some(json_u64(body, "id")).filter(|n| *n > 0),
            title: &title,
            tpl: &tpl,
            display: json_i32(body, "display"),
            integral: json_i32(body, "integral"),
            com_bm: json_i32(body, "com_bm"),
            sort: json_i32(body, "sort"),
            limit: json_i32(body, "limit"),
            etime,
            intro: &amp(&json_str(body, "intro")),
            rating: &rating,
            now: clock::now_ts(),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn special_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("model_00034"));
    }
    special_repo::delete_specials(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("admin_model_00056"))
}

async fn special_set_order(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = special_repo::set_sort(state.db.pool(), id, json_i32(body, "sort")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01443"));
    }
    Ok(PhpOut::Message("admin_model_00058"))
}

async fn special_recommend(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if json_str(body, "type") != "rec_display" {
        return Ok(PhpOut::Message("ok"));
    }
    special_repo::set_display(state.db.pool(), json_u64(body, "id"), json_i32(body, "rec")).await?;
    Ok(PhpOut::Message("admin_model_00063"))
}

async fn special_ajaxsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    special_repo::set_com_sort(state.db.pool(), id, json_i32(body, "sort")).await?;
    Ok(PhpOut::Message("admin_model_00057"))
}

async fn special_set_famous(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let sid = json_u64(body, "sid");
    let uid = json_u64(body, "uid");
    if sid == 0 || uid == 0 {
        return Err(ApiError::business("admin_01449"));
    }
    let famous = if json_i32(body, "famous") == 1 { 0 } else { 1 };
    special_repo::set_famous(state.db.pool(), sid, uid, famous).await?;
    Ok(PhpOut::Message("admin_model_00062"))
}

async fn special_addlist(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let sid = json_u64(body, "id").max(json_u64(body, "sid"));
    let kw = json_str(body, "keyword");
    let rows = special_repo::list_add_companies(
        state.db.reader(),
        sid,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
        offset,
        limit,
    )
    .await?;
    let total = special_repo::count_add_companies(
        state.db.reader(),
        sid,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
    )
    .await?;
    Ok(paged(serde_json::to_value(rows).unwrap_or(json!([])), total, page, per))
}

async fn special_comaddsearch(state: &AppState) -> AppResult<Value> {
    let rating = company_repo::list_rating_options(state.db.reader()).await?;
    let rating_list: Vec<Value> = rating
        .into_iter()
        .map(|r| json!({ "value": r.id, "label": r.name }))
        .collect();
    Ok(json!({
        "ratingList": rating_list,
        "timeList": [
            {"value": "1", "label": "admin_tool_00622"},
            {"value": "2", "label": "common_01659"},
            {"value": "3", "label": "common_01897"},
            {"value": "4", "label": "common_01875"},
            {"value": "5", "label": "wap_com_00319"},
        ],
        "statusList": [
            {"value": "1", "label": "wap_user_00165"},
            {"value": "2", "label": "admin_user_00138"},
            {"value": "3", "label": "wap_user_00167"},
            {"value": "4", "label": "wap_user_00166"},
            {"value": "5", "label": "admin_user_00184"},
        ],
        "sourceList": [],
        "recList": [
            {"value": "1", "label": "admin_model_00059"},
            {"value": "2", "label": "admin_model_00060"},
            {"value": "3", "label": "wap_com_00319"},
        ],
        "gwList": [
            {"value": "1", "label": "admin_01303"},
            {"value": "2", "label": "admin_user_company_00153"},
        ],
        "lotimeList": [
            {"value": "1", "label": "common_01940"},
            {"value": "3", "label": "admin_tool_00619"},
            {"value": "7", "label": "admin_tool_00622"},
        ],
        "adtimeList": [
            {"value": "1", "label": "common_01940"},
            {"value": "3", "label": "admin_tool_00619"},
            {"value": "7", "label": "admin_tool_00622"},
        ],
    }))
}

async fn special_audit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let com = special_repo::find_com_one(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("special_com_not_found"))?;
    let info = company_repo::find_by_uid(state.db.reader(), com.uid).await?;
    Ok(json!({
        "name": info.as_ref().and_then(|c| c.name.clone()).unwrap_or_default(),
        "linkman": info.as_ref().and_then(|c| c.linkman.clone()).unwrap_or_default(),
        "linktel": info.as_ref().and_then(|c| c.linktel.clone()).unwrap_or_default(),
        "special": com,
    }))
}

async fn special_comjob(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    let jobs = zph_repo::job_labels_for_uid(state.db.reader(), uid).await?;
    Ok(json!({
        "list": jobs.into_iter().map(|(id, name)| json!({ "id": id, "name": name })).collect::<Vec<_>>(),
    }))
}
