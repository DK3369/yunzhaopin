//! PHP admin named actions for 招聘会 / 新闻 / 问答 / 专题.
//! SQL stays in repos. Routes are `php-*` and stay out of AdminDoc.

use std::collections::HashMap;

use phpyun_core::i18n;
use phpyun_core::utils::{fmt_date, fmt_dt, fmt_ts};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::ad::repo as ad_repo;
use phpyun_models::admin_gap::extra as gap_extra;
use phpyun_models::admin_gap::repo as gap_repo;
use phpyun_models::admin_msg::repo as admin_msg_repo;
use phpyun_models::company_statis::repo as cstatis_repo;
use phpyun_models::integral_transfer::repo as pay_repo;
use phpyun_models::member_statis::repo as mstatis_repo;
use phpyun_models::vip::repo as vip_repo;
use phpyun_models::announcement::repo as announcement_repo;
use phpyun_models::article::repo::{self as article_repo, ArticleFilter};
use phpyun_auth::{argon2_hash_async, md5_hex};
use phpyun_models::category::repo as cat_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::description::repo as desc_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::gongzhao::repo as gongzhao_repo;
use phpyun_models::interview_template::repo as yqmb_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::member_logout::repo as logout_repo;
use phpyun_models::once_job::repo as once_repo;
use phpyun_models::part::repo as part_repo;
use phpyun_models::poster_template::repo as whb_repo;
use phpyun_models::qna::repo as qna_repo;
use phpyun_models::resume::expect as expect_repo;
use phpyun_models::resume::other as other_repo;
use phpyun_models::resume::project as project_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume::skill as skill_repo;
use phpyun_models::site_page::repo as site_page_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::special::repo as special_repo;
use phpyun_models::tiny::repo as tiny_repo;
use phpyun_models::user::repo as user_repo;
use phpyun_models::wx_nav::repo as wx_nav_repo;
use phpyun_models::zph::repo as zph_repo;
use serde_json::{json, Value};

use crate::admin_cms_service;
use crate::admin_longtail_service;
use crate::description_service;
use crate::dict_service;
use crate::site_setting_service;
use crate::wechat_api_service;
use uuid::Uuid;

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
        ("fairs", "comxlscheck") => fairs_comxlscheck(state, body).await,
        ("fairs", "comxls") => Ok(PhpOut::Data(fairs_comxls(state, body).await?)),
        ("fairs", "upload") => Err(ApiError::business("upload_not_supported")),
        ("fairs", "uploadsave") => Err(ApiError::business("upload_not_supported")),
        ("fairs", "setthemb") => Err(ApiError::business("upload_not_supported")),
        ("fairs", "delpic") => Err(ApiError::business("upload_not_supported")),
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
        ("ads", "index") => Ok(PhpOut::Data(ads_index(state, body).await?)),
        ("ads", "get_base_data") => Ok(PhpOut::Data(ads_get_base(state).await?)),
        ("ads", "info") => Ok(PhpOut::Data(ads_info(state, body).await?)),
        ("ads", "ad_saveadd") => ads_saveadd(state, body).await,
        ("ads", "delete") => ads_del(state, body).await,
        ("ads", "preview") => Ok(PhpOut::Data(ads_preview(state, body).await?)),
        ("ads", "check") => ads_check(state, body).await,
        ("ads", "cache_ad") => Ok(PhpOut::Message("admin_01172")),
        ("ads", "ctime") => ads_ctime(state, body).await,
        ("ads", "upsort") => ads_upsort(state, body).await,
        ("ad-class", "index") => Ok(PhpOut::Data(ad_class_index(state, body).await?)),
        ("ad-class", "info") => Ok(PhpOut::Data(ad_class_info(state, body).await?)),
        ("ad-class", "addclass") => ad_class_add(state, body).await,
        ("ad-class", "delete") => ad_class_del(state, body).await,
        ("ad-class", "delbuy") => ad_class_delbuy(state, body).await,
        ("ad-class", "upsort") => ad_class_upsort(state, body).await,
        ("finance-order", "searchType") => Ok(PhpOut::Data(finance_order_search_type(state).await?)),
        ("finance-order", "index") => Ok(PhpOut::Data(finance_order_index(state, body).await?)),
        ("finance-order", "edit") => Ok(PhpOut::Data(finance_order_edit(state, body).await?)),
        ("finance-order", "save") => finance_order_save(state, body).await,
        ("finance-order", "setpay") => finance_order_setpay(state, body).await,
        ("finance-order", "delete") => finance_order_del(state, body).await,
        ("finance-order", "xls") => Ok(PhpOut::Data(finance_order_xls(state, body).await?)),
        ("finance-order", "multiupload") => Err(ApiError::business("upload_not_supported")),
        ("finance-order", "uploadsave") => Err(ApiError::business("upload_not_supported")),
        ("finance-order", "htpic_del") => Err(ApiError::business("upload_not_supported")),
        ("finance-pay", "index") => Ok(PhpOut::Data(finance_pay_index(state, body).await?)),
        ("finance-pay", "delete") => finance_pay_del(state, body).await,
        ("finance-recharge", "index") => Ok(PhpOut::Data(finance_recharge_index(state).await?)),
        ("finance-recharge", "jifenSave") => finance_jifen_save(state, body).await,
        ("finance-recharge", "comvip") => finance_comvip(state, body).await,
        ("finance-recharge", "comservice") => finance_comservice(state, body).await,
        ("finance-recharge", "getservice") => Ok(PhpOut::Data(finance_getservice(state, body).await?)),
        ("finance-recharge", "searchname") => Ok(PhpOut::Data(finance_searchname(state, body, true).await?)),
        ("finance-recharge", "searchcom") => Ok(PhpOut::Data(finance_searchname(state, body, false).await?)),
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
        ("once", "index") => Ok(PhpOut::Data(once_index(state, body).await?)),
        ("once", "once-num") => Ok(PhpOut::Data(once_num(state).await?)),
        ("once", "status") => Ok(PhpOut::Data(once_status(state, body).await?)),
        ("once", "checksitedid") => once_checksitedid(state, body).await,
        ("once", "price_gear") => Ok(PhpOut::Data(once_price_gear(state).await?)),
        ("once", "price_gear_add") => once_price_gear_add(state, body).await,
        ("once", "price_gear_ajax") => once_price_gear_ajax(state, body).await,
        ("once", "price_gear_del") => once_price_gear_del(state, body).await,
        ("once", "set") => Ok(PhpOut::Data(once_set(state).await?)),
        ("once", "onceset") => once_onceset(state, user, body).await,
        ("once", "edit") => Ok(PhpOut::Data(once_edit(state, body).await?)),
        ("once", "save") => once_save(state, body).await,
        ("once", "del") => once_del(state, body).await,
        ("once", "ctime") => once_ctime(state, body).await,
        ("once", "refresh_job") => once_refresh(state, body).await,
        ("tiny", "index") => Ok(PhpOut::Data(tiny_index(state, body).await?)),
        ("tiny", "tiny-num") => Ok(PhpOut::Data(tiny_num(state).await?)),
        ("tiny", "status") => tiny_status(state, body).await,
        ("tiny", "checksitedid") => tiny_checksitedid(state, body).await,
        ("tiny", "set") => Ok(PhpOut::Data(tiny_set(state).await?)),
        ("tiny", "tinyset") => tiny_tinyset(state, user, body).await,
        ("tiny", "save") => tiny_save(state, body).await,
        ("tiny", "del") => tiny_del(state, body).await,
        ("tiny", "refresh") => tiny_refresh(state, body).await,
        ("part", "show") => part_show(state, body).await,
        ("part", "partAudit") => Ok(PhpOut::Data(part_audit(state, body).await?)),
        ("part", "recommend") => part_recommend(state, body).await,
        ("part", "ctime") => part_ctime(state, body).await,
        ("part", "refresh") => part_refresh(state, body).await,
        ("part", "del") => part_del(state, body).await,
        ("part", "checkstate") => part_checkstate(state, body).await,
        ("hotjob", "save") => hotjob_save(state, user, body).await,
        ("hotjob", "getComList") => Ok(PhpOut::Data(hotjob_com_list(state, body).await?)),
        ("hotjob", "gethotjob") => Ok(PhpOut::Data(hotjob_get(state, body).await?)),
        ("hotjob", "hotjobinfo") => Ok(PhpOut::Data(hotjob_info(state, body).await?)),
        ("hotjob", "hotNum") => Ok(PhpOut::Data(hotjob_num(state).await?)),
        ("resume", "skill") => resume_skill(state, body).await,
        ("resume", "project") => resume_project(state, body).await,
        ("resume", "other") => resume_other(state, body).await,
        ("resume", "rec") => resume_rec(state, body).await,
        ("resume", "top") => resume_top(state, body).await,
        ("resume", "refresh") => resume_refresh(state, body).await,
        ("interview", "index") => Ok(PhpOut::Data(interview_index(state, body).await?)),
        ("interview", "save") => interview_save(state, body).await,
        ("interview", "status") => interview_status(state, body).await,
        ("interview", "delYqmb") => interview_del(state, body).await,
        ("comlog", "userid-job") => Ok(PhpOut::Data(comlog_userid_job(state, body).await?)),
        ("comlog", "deluseridjob") => comlog_del_userid_job(state, body).await,
        ("pages", "index") => Ok(PhpOut::Data(pages_index(state, body).await?)),
        ("pages", "add") => Ok(PhpOut::Data(pages_add(state, body).await?)),
        ("pages", "save") => pages_save(state, body).await,
        ("pages", "delete") => pages_del(state, body).await,
        ("pages", "make") => Ok(PhpOut::Message("admin_system_00059")),
        ("pages", "ajax") => pages_ajax(state, body).await,
        ("desc-class", "index") => Ok(PhpOut::Data(desc_class_index(state, body).await?)),
        ("desc-class", "add") => Ok(PhpOut::Data(desc_class_add(state, body).await?)),
        ("desc-class", "ajax") => desc_class_ajax(state, body).await,
        ("desc-class", "delete") => desc_class_del(state, body).await,
        ("job-class", "ajax") => job_class_ajax(state, body).await,
        ("job-class", "setrec") => job_class_setrec(state, body).await,
        ("job-class", "get_class") => Ok(PhpOut::Data(job_class_get(state, body).await?)),
        ("job-class", "up") => Ok(PhpOut::Data(job_class_up(state, body).await?)),
        ("job-class", "getJobClass") => Ok(PhpOut::Data(job_class_roots(state).await?)),
        ("job-class", "classadd") => Ok(PhpOut::Data(job_class_classadd(state, body).await?)),
        ("job-class", "ajaxchachong") => Ok(PhpOut::Data(job_class_chachong(state, body).await?)),
        ("job-class", "ajaxpinyin") => Ok(PhpOut::Message("admin_system_00081")),
        ("job-class", "move") => job_class_move(state, body).await,
        ("wx-nav", "wxnav") => Ok(PhpOut::Data(wx_nav_list(state).await?)),
        ("wx-nav", "savenav") => Ok(PhpOut::Data(wx_nav_savenav(state, body).await?)),
        ("wx-nav", "delnav") => wx_nav_del(state, body).await,
        ("wx-nav", "ajaxnav") => wx_nav_ajax(state, body).await,
        ("wx-nav", "creatnav") => Ok(PhpOut::Data(wx_nav_creatnav(state).await?)),
        ("wx-nav", "config") => Ok(PhpOut::Data(wx_nav_config(state).await?)),
        ("wx-nav", "zdkeyword") => Ok(PhpOut::Data(wx_zdkeyword_list(state, body).await?)),
        ("wx-nav", "delkeyword") => wx_zdkeyword_del(state, body).await,
        ("wx-nav", "getzdkeyword") => Ok(PhpOut::Data(wx_zdkeyword_get(state, body).await?)),
        ("wx-nav", "save-zdkeyword") => wx_zdkeyword_save(state, body).await,
        ("cat-class", "list") => Ok(PhpOut::Data(cat_class_list(state, body).await?)),
        ("cat-class", "children") => Ok(PhpOut::Data(cat_class_children(state, body).await?)),
        ("cat-class", "add") | ("cat-class", "save") => cat_class_save(state, body).await,
        ("cat-class", "del") => cat_class_del(state, body).await,
        ("cat-class", "ajax") => cat_class_ajax(state, body).await,
        ("cat-class", "up") => Ok(PhpOut::Data(cat_class_up(state, body).await?)),
        ("cat-class", "add_single") => cat_class_add_single(state, body).await,
        ("cat-class", "up_single") => cat_class_up_single(state, body).await,
        ("cat-class", "upp") => cat_class_upp(state, body).await,
        ("cat-class", "ajaxpinyin") => Ok(PhpOut::Message("admin_system_00081")),
        ("cat-class", "clearpinyin") => cat_class_clearpinyin(state).await,
        ("cat-class", "ajaxchachong") => Ok(PhpOut::Data(cat_class_chachong(state, body).await?)),
        ("cat-class", "classadd") => Ok(PhpOut::Data(cat_class_one(state, body).await?)),
        ("user-gap", "company-num") => Ok(PhpOut::Data(user_gap_company_num(state).await?)),
        ("user-gap", "resume-num") => Ok(PhpOut::Data(user_gap_resume_num(state).await?)),
        ("user-gap", "user-num") => Ok(PhpOut::Data(user_gap_user_num(state).await?)),
        ("user-gap", "mem-num") => Ok(PhpOut::Data(user_gap_mem_num(state).await?)),
        ("user-gap", "mem-index") => Ok(PhpOut::Data(user_gap_mem_index(state, body).await?)),
        ("user-gap", "logout-index") => Ok(PhpOut::Data(user_gap_logout_index(state, body).await?)),
        ("user-gap", "appeal-index") => Ok(PhpOut::Data(user_gap_appeal_index(state, body).await?)),
        ("user-gap", "login-index") => Ok(PhpOut::Data(user_gap_login_index(state, body).await?)),
        ("user-gap", "login-del") => user_gap_login_del(state, body).await,
        ("user-gap", "memlog-index") => Ok(PhpOut::Data(user_gap_memlog_index(state, body).await?)),
        ("user-gap", "memlog-del") => user_gap_memlog_del(state, body).await,
        ("user-gap", "mem-imitate") => Ok(PhpOut::Data(user_gap_mem_imitate(state, body).await?)),
        ("user-gap", "mem-lock") => user_gap_mem_lock(state, body).await,
        ("user-gap", "mem-edit") => user_gap_mem_edit(state, body).await,
        ("user-gap", "mem-del") => user_gap_mem_del(state, body).await,
        ("user-gap", "appeal-info") => Ok(PhpOut::Data(user_gap_appeal_info(state, body).await?)),
        ("user-gap", "appeal-success") => user_gap_appeal_success(state, body).await,
        ("user-gap", "appeal-del") => user_gap_appeal_del(state, body).await,
        ("user-gap", "logout-status") => user_gap_logout_status(state, body).await,
        ("user-gap", "logout-del") => user_gap_logout_del(state, body).await,
        ("user-gap", "logout-num") => Ok(PhpOut::Data(user_gap_logout_num(state).await?)),
        ("user-gap", "resume-config") => Ok(PhpOut::Data(user_gap_resume_config(state).await?)),
        ("user-gap", "user-config") => Ok(PhpOut::Data(user_gap_user_config(state).await?)),
        ("keyword", "map") => Ok(PhpOut::Data(keyword_type_map())),
        ("web-config", "index") => Ok(PhpOut::Data(web_config_index(state).await?)),
        ("web-config", "city") => Ok(PhpOut::Data(web_config_city(state, body).await?)),
        ("user-gap", "reset-password") => user_gap_reset_password(state, body).await,
        ("user-gap", "matching") => Ok(PhpOut::Data(user_gap_matching(state, body).await?)),
        ("user-gap", "company-index") => Ok(PhpOut::Data(user_gap_company_index(state, body).await?)),
        ("user-gap", "resume-index") => Ok(PhpOut::Data(user_gap_resume_index(state, body).await?)),
        ("user-gap", "job-refresh-index") => {
            Ok(PhpOut::Data(user_gap_job_refresh_index(state, body).await?))
        }
        ("user-gap", "job-refresh-del") => user_gap_job_refresh_del(state, body).await,
        ("user-gap", "resume-audit") => {
            Ok(PhpOut::Data(user_gap_resume_audit(state, user, body).await?))
        }
        ("email-set", "ceshi") => email_set_ceshi(state, body).await,
        ("email-set", "gettpl") => Ok(PhpOut::Data(email_set_gettpl(state, body).await?)),
        ("email-set", "savetpl") => email_set_savetpl(state, body).await,
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

fn ids_named(body: &Value, key: &str) -> Vec<u64> {
    match body.get(key) {
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

fn json_f64(v: &Value, key: &str) -> f64 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn json_opt_i32(v: &Value, key: &str) -> Option<i32> {
    match v.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.trim().is_empty() => None,
        _ => Some(json_i32(v, key)),
    }
}

fn json_csv(body: &Value, key: &str) -> String {
    match body.get(key) {
        Some(Value::Array(a)) => a
            .iter()
            .map(|v| match v {
                Value::String(s) => s.trim().to_string(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(","),
        _ => json_str(body, key),
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
        "page_size": per,
        "pageSizes": sizes,
        "page_sizes": sizes,
        "limit": per,
        "page": page,
    })
}

/// PHP admin tables that bind `res.data.data` (not `list`).
fn php_data_table(data: Vec<Value>, total: u64) -> Value {
    let sizes = vec![10, 20, 50, 100];
    json!({
        "data": data,
        "total": total,
        "pageSizes": sizes,
        "page_sizes": sizes,
    })
}

fn json_day_range(body: &Value, key: &str) -> (Option<i64>, Option<i64>) {
    let arr = match body.get(key) {
        Some(Value::Array(a)) if a.len() >= 2 => a,
        _ => return (None, None),
    };
    let a = match &arr[0] {
        Value::String(s) => s.as_str(),
        _ => return (None, None),
    };
    let b = match &arr[1] {
        Value::String(s) => s.as_str(),
        _ => return (None, None),
    };
    if a.trim().is_empty() || b.trim().is_empty() {
        return (None, None);
    }
    let from = chrono::NaiveDate::parse_from_str(a.trim(), "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|t| t.and_utc().timestamp());
    let to = chrono::NaiveDate::parse_from_str(b.trim(), "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(23, 59, 59))
        .map(|t| t.and_utc().timestamp());
    (from, to)
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

fn trunc_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn city_n(dicts: &dict_service::LocalizedDicts, p: i32, c: i32, t: i32) -> String {
    let mut parts = Vec::new();
    for id in [p, c, t] {
        if id > 0 {
            let n = dicts.city(id);
            if !n.is_empty() {
                parts.push(n.to_string());
            }
        }
    }
    parts.join("-")
}

fn sex_n(sex: i32) -> &'static str {
    match sex {
        1 => "男",
        2 => "女",
        _ => "",
    }
}

fn php_time_min(body: &Value) -> Option<i64> {
    let n = json_i32(body, "time");
    if n <= 0 {
        return None;
    }
    let now = clock::now_ts();
    if n == 1 {
        Some(start_of_utc_day(now))
    } else {
        Some(now - i64::from(n) * 86_400)
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
        ..Default::default()
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

fn json_date_pair(v: &Value, key: &str) -> (String, String) {
    let alt = format!("{key}[]");
    let arr = v.get(key).or_else(|| v.get(&alt));
    match arr {
        Some(Value::Array(a)) if a.len() >= 2 => {
            let s0 = match &a[0] {
                Value::String(s) => s.trim().to_string(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            };
            let s1 = match &a[1] {
                Value::String(s) => s.trim().to_string(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            };
            (s0, s1)
        }
        _ => (
            json_str(v, "ad_time_start"),
            json_str(v, "ad_time_end"),
        ),
    }
}

fn ad_ended(time_end: &str, now: i64) -> bool {
    let ts = parse_date_ts(time_end);
    ts > 0 && ts + 86_399 < now
}

fn ad_row_json(r: &ad_repo::AdAdminRow, base: &str, now: i64, dname: &HashMap<i32, String>) -> Value {
    let ended = ad_ended(&r.time_end, now);
    let pic_n = pic_url(base, &r.pic_url);
    let pic_url_list: Vec<String> = if r.ad_type == "pic" && !pic_n.is_empty() {
        pic_n
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| pic_url(base, s))
            .collect()
    } else {
        Vec::new()
    };
    let ad_typename = match r.ad_type.as_str() {
        "word" => "admin_01140",
        "pic" => "admin_01141",
        "flash" => "admin_01169",
        "lianmeng" => "admin_yunying_00072",
        _ => "",
    };
    let d_title = if r.did > 0 {
        dname
            .get(&r.did)
            .cloned()
            .unwrap_or_default()
    } else if r.did == -1 {
        "api_wxapp_00018".into()
    } else {
        "ajax_00021".into()
    };
    json!({
        "id": r.id,
        "ad_name": r.ad_name,
        "class_id": r.class_id.to_string(),
        "class_name": r.class_name,
        "hits": r.hits,
        "ad_type": r.ad_type,
        "ad_typename": ad_typename,
        "pic_url": r.pic_url,
        "pic_url_n": pic_n,
        "pic_url_list": pic_url_list,
        "pic_src": r.pic_src,
        "word_url": r.word_url,
        "word_info": r.word_info,
        "time_start": r.time_start,
        "time_end": r.time_end,
        "did": r.did.to_string(),
        "d_title": d_title,
        "sort": r.sort,
        "is_open": r.is_open.to_string(),
        "is_check": r.is_check.to_string(),
        "is_end": if ended { "1" } else { "0" },
        "target": r.target.to_string(),
        "pic_width": r.pic_width,
        "pic_height": r.pic_height,
        "pic_content": r.pic_content,
        "remark": r.remark,
        "flash_url": r.flash_url,
        "flash_src": r.flash_src,
        "flash_width": r.flash_width,
        "flash_height": r.flash_height,
        "lianmeng_url": r.lianmeng_url,
    })
}

async fn ads_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let is_check_raw = json_str(body, "is_check");
    let expired = is_check_raw == "2";
    let is_check = match is_check_raw.as_str() {
        "1" => Some(1),
        "-1" => Some(0),
        _ => None,
    };
    let ad_code = json_str(body, "ad");
    let ad_type = match ad_code.as_str() {
        "1" => Some("word"),
        "2" => Some("pic"),
        "3" => Some("flash"),
        _ => None,
    };
    let name = json_str(body, "name");
    let f = ad_repo::AdAdminFilter {
        class_id: Some(json_i32(body, "class_id")).filter(|n| *n > 0),
        is_check: if expired { None } else { is_check },
        expired,
        name: if name.is_empty() { None } else { Some(name.as_str()) },
        ad_type,
    };
    let now_date = fmt_date(clock::now_ts());
    let now = clock::now_ts();
    let db = state.db.reader();
    let rows = ad_repo::list_admin_php(db, &f, &now_date, offset, limit).await?;
    let total = ad_repo::count_admin_php(db, &f, &now_date).await?;
    let domains = domain_repo::list_all(db).await?;
    let dname: HashMap<i32, String> = domains
        .into_iter()
        .map(|d| (d.id as i32, d.title))
        .collect();
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .iter()
        .map(|r| ad_row_json(r, &base, now, &dname))
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn ads_get_base(state: &AppState) -> AppResult<Value> {
    let classes = ad_repo::list_classes(state.db.reader()).await?;
    let mut class_two: HashMap<i32, Vec<Value>> = HashMap::new();
    let max_len = classes
        .iter()
        .map(|c| c.id.to_string().len())
        .max()
        .unwrap_or(1);
    for c in &classes {
        let place = if c.place == 1 || c.place == 2 { c.place } else { 3 };
        let pad = format!("{:0>width$}", c.id, width = max_len);
        class_two.entry(place).or_default().push(json!({
            "label": format!("{pad}   {}", c.class_name),
            "value": c.id.to_string(),
        }));
    }
    let class_data = [1, 2, 3]
        .into_iter()
        .map(|id| {
            let label = match id {
                1 => "PC",
                2 => "WAP",
                _ => "common_01924",
            };
            let children = class_two.get(&id).cloned().unwrap_or_default();
            let mut row = json!({ "label": label, "value": id.to_string() });
            if !children.is_empty() {
                row["children"] = Value::Array(children);
            }
            row
        })
        .collect::<Vec<_>>();
    let domains = domain_repo::list_all(state.db.reader()).await?;
    let domain_data: Vec<Value> = domains
        .iter()
        .map(|d| json!({ "label": d.title, "value": d.id.to_string() }))
        .collect();
    Ok(json!({ "classData": class_data, "domainData": domain_data }))
}

async fn ads_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let now = clock::now_ts();
    let base = preview_base(state);
    let info = if id > 0 {
        if let Some(r) = ad_repo::find_admin(state.db.reader(), id).await? {
            let dname = HashMap::new();
            ad_row_json(&r, &base, now, &dname)
        } else {
            json!({})
        }
    } else {
        json!({})
    };
    Ok(json!({ "info": info, "appad": 0 }))
}

async fn ads_saveadd(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ad_name = json_str(body, "ad_name");
    if ad_name.is_empty() {
        return Err(ApiError::business("admin_01413"));
    }
    let (start, end) = json_date_pair(body, "ad_time");
    if start.is_empty() || end.is_empty() {
        return Err(ApiError::business("admin_01414"));
    }
    let ad_type = json_str(body, "ad_type");
    let mut pic_url = json_str(body, "pic_url_n");
    if pic_url.is_empty() {
        pic_url = json_str(body, "pic_url");
    }
    let flash_url = json_str(body, "flash_url");
    let pictures = if ad_type == "flash" && !flash_url.is_empty() {
        flash_url.clone()
    } else {
        pic_url
    };
    let target = if json_i32(body, "target") == 2 { 2 } else { 1 };
    ad_repo::upsert_php(
        state.db.pool(),
        ad_repo::AdPhpWrite {
            id: Some(json_u64(body, "id")).filter(|n| *n > 0),
            ad_name: &ad_name,
            target,
            time_start: &start,
            time_end: &end,
            ad_type: &ad_type,
            class_id: json_i32(body, "class_id"),
            is_check: 1,
            did: json_i32(body, "did"),
            is_open: json_i32(body, "is_open"),
            sort: json_i32(body, "sort"),
            remark: &json_str(body, "remark"),
            pic_url: if ad_type == "pic" { &pictures } else { "" },
            pic_src: &json_str(body, "pic_src"),
            pic_content: &json_str(body, "pic_content"),
            word_info: &json_str(body, "word_info"),
            word_url: &json_str(body, "word_url"),
            pic_width: &json_str(body, "pic_width"),
            pic_height: &json_str(body, "pic_height"),
            flash_url: if ad_type == "flash" { &pictures } else { "" },
            lianmeng_url: &json_str(body, "lianmeng_url"),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn ads_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01066"));
    }
    let n = ad_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn ads_preview(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let r = ad_repo::find_admin(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("wap_js_00113"))?;
    let now = clock::now_ts();
    let base = preview_base(state);
    let dname = HashMap::new();
    let mut row = ad_row_json(&r, &base, now, &dname);
    let html = match r.ad_type.as_str() {
        "word" => format!(
            "<a href=\"{}\">{}</a>",
            r.word_url,
            r.word_info
        ),
        "pic" => {
            let h = if r.pic_height.is_empty() {
                String::new()
            } else {
                format!("height=\"{}\"", r.pic_height)
            };
            let w = if r.pic_width.is_empty() {
                String::new()
            } else {
                format!("width=\"{}\"", r.pic_width)
            };
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"nofollow\"><img src=\"{}\"  {} {} ></a>",
                r.pic_src,
                pic_url(&base, &r.pic_url),
                h,
                w
            )
        }
        "flash" => {
            let url = pic_url(&base, &r.flash_url);
            format!(
                "<object type=\"application/x-shockwave-flash\" data=\"{url}\" width=\"{}\" height=\"{}\"><param name=\"movie\" value=\"{url}\" /><param value=\"transparent\" name=\"wmode\"></object>",
                r.flash_width, r.flash_height
            )
        }
        "lianmeng" => r.lianmeng_url.clone(),
        _ => String::new(),
    };
    row["html"] = json!(html);
    row["src"] = json!(format!(
        "{}/data/plus/yunimg.php?classid={}&ad_id={}",
        base.trim_end_matches('/'),
        r.class_id,
        r.id
    ));
    Ok(row)
}

async fn ads_check(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    ad_repo::set_check(state.db.pool(), id, json_i32(body, "val")).await?;
    Ok(PhpOut::Message("ok"))
}

async fn ads_ctime(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let days = json_i32(body, "endtime");
    let ids = match body.get("jobid") {
        Some(Value::Array(a)) => a.iter().map(json_u64_val).filter(|n| *n > 0).collect(),
        Some(Value::String(s)) => s
            .split([',', ';'])
            .filter_map(|x| x.trim().parse().ok())
            .filter(|n: &u64| *n > 0)
            .collect(),
        Some(Value::Number(n)) => n.as_u64().filter(|n| *n > 0).into_iter().collect(),
        _ => Vec::new(),
    };
    if days < 1 || ids.is_empty() {
        return Err(ApiError::business("common_01716"));
    }
    let n = ad_repo::extend_end_days(state.db.pool(), &ids, days).await?;
    if n == 0 {
        return Err(ApiError::business("wap_01715"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn ads_upsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("common_01716"));
    }
    ad_repo::set_sort(state.db.pool(), id, json_i32(body, "sort")).await?;
    Ok(PhpOut::Data(json!({})))
}

fn place_n(place: i32) -> &'static str {
    match place {
        1 => "PC",
        2 => "WAP",
        _ => "common_01924",
    }
}

async fn ad_class_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let kw_type = json_i32(body, "type");
    let db = state.db.reader();
    let rows = ad_repo::list_classes_admin(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        kw_type,
        offset,
        limit,
    )
    .await?;
    let total = ad_repo::count_classes_admin(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        kw_type,
    )
    .await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "class_name": r.class_name,
                "place": r.place,
                "place_n": place_n(r.place),
                "orders": r.orders,
                "type": r.r#type,
                "href": r.href,
                "hrefn": pic_url(&base, &r.href),
                "integral_buy": r.integral_buy,
                "btype": r.btype,
                "x": r.x,
                "y": r.y,
                "remark": r.remark,
            })
        })
        .collect();
    let pricename = setting_repo::find(db, "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let pic_max = setting_repo::find(db, "pic_maxsize")
        .await?
        .map(|s| s.value)
        .unwrap_or_else(|| "5".into());
    let pic_type = setting_repo::find(db, "pic_type")
        .await?
        .map(|s| s.value)
        .unwrap_or_else(|| "jpg,png,jpeg,bmp,gif".into());
    let mut out = paged(Value::Array(list), total, page, per);
    out["integral_pricename"] = json!(pricename);
    out["pic_maxsize"] = json!(pic_max);
    out["pic_type"] = json!(pic_type);
    Ok(out)
}

async fn ad_class_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let r = ad_repo::find_class(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("admin_00351"))?;
    let base = preview_base(state);
    Ok(json!({
        "id": r.id,
        "class_name": r.class_name,
        "place": r.place,
        "place_n": place_n(r.place),
        "orders": r.orders,
        "type": r.r#type,
        "href": r.href,
        "hrefn": pic_url(&base, &r.href),
        "integral_buy": r.integral_buy,
        "btype": r.btype,
        "x": r.x,
        "y": r.y,
        "remark": r.remark,
    }))
}

async fn ad_class_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let class_name = json_str(body, "class_name");
    if class_name.is_empty() {
        return Err(ApiError::param_invalid("class_name"));
    }
    let id = json_u64(body, "id");
    let ty = json_i32(body, "type");
    if id == 0 && ty == 0 {
        return Err(ApiError::business("api_wxapp_00012"));
    }
    let href = json_str(body, "href");
    ad_repo::upsert_class(
        state.db.pool(),
        ad_repo::AdClassWrite {
            id: Some(id).filter(|n| *n > 0),
            class_name: &class_name,
            orders: json_i32(body, "orders"),
            place: json_i32(body, "place"),
            r#type: ty,
            btype: &json_str(body, "btype"),
            integral_buy: &json_str(body, "integral_buy"),
            href: &href,
            x: &json_str(body, "x"),
            y: &json_str(body, "y"),
            remark: &json_str(body, "remark"),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn ad_class_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("admin_01415"));
    }
    for id in &ids {
        if ad_repo::count_ads_in_class(state.db.reader(), *id).await? > 0 {
            return Err(ApiError::business("admin_yunying_00002"));
        }
    }
    ad_repo::delete_classes(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn ad_class_delbuy(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    let n = ad_repo::clear_class_buy(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("model_00004"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn ad_class_upsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("common_01716"));
    }
    ad_repo::set_class_orders(state.db.pool(), id, json_i32(body, "orders")).await?;
    Ok(PhpOut::Data(json!({})))
}

fn pay_name(code: &str) -> &'static str {
    match code {
        "alipay" => "支付宝",
        "tenpay" => "财富通",
        "bank" => "银行转帐",
        "alipaydual" => "支付宝双接口",
        "alipayescow" => "担保交易",
        "adminpay" => "管理员充值",
        "balance" => "余额支付",
        "admincut" => "管理员扣款",
        "wapalipay" => "支付宝手机支付",
        _ => "",
    }
}

fn order_kind_name(kind: i32) -> &'static str {
    match kind {
        1 => "购买会员",
        2 => "积分充值",
        3 => "银行转帐",
        4 => "金额充值",
        5 => "购买增值包",
        10 => "职位置顶",
        11 => "职位紧急",
        12 => "职位推荐",
        13 => "自动刷新",
        14 => "简历置顶",
        16 => "刷新职位",
        17 => "刷新兼职",
        19 => "下载简历",
        20 => "发布职位",
        21 => "发布兼职",
        23 => "面试邀请",
        24 => "兼职推荐",
        25 => "店铺招聘",
        28 => "招聘会报名",
        _ => "",
    }
}

fn pay_state_html(state: i32) -> &'static str {
    match state {
        0 => "<font color=red>支付失败</font>",
        1 => "<font color=green>等待付款</font>",
        2 => "<font color=#3d7dfd>支付成功</font>",
        3 => "<font color=#c30ad9>等待确认</font>",
        4 => "<font color=red>交易关闭</font>",
        _ => "",
    }
}

fn json_present_i32(v: &Value, key: &str) -> Option<i32> {
    match v.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.trim().is_empty() => None,
        Some(_) => Some(json_i32(v, key)),
    }
}

fn finance_order_json(r: &vip_repo::PhpOrderRow) -> Value {
    let rating_name = if r.r#type == 1 && !r.rating_name.is_empty() {
        format!("：{}", r.rating_name)
    } else {
        String::new()
    };
    json!({
        "id": r.id,
        "uid": r.uid,
        "order_id": r.order_id,
        "order_price": r.order_price,
        "type": r.r#type.to_string(),
        "type_n": order_kind_name(r.r#type),
        "rating": r.rating,
        "rating_name": rating_name,
        "rating_name_n": r.rating_name,
        "order_state": r.order_state.to_string(),
        "order_state_n": pay_state_html(r.order_state),
        "order_type": r.order_type,
        "order_type_n": pay_name(&r.order_type),
        "order_time": fmt_dt(r.order_time),
        "order_time_ymd": fmt_dt(r.order_time),
        "once_id": r.once_id,
        "crm_uid": r.crm_uid,
        "crm_name": r.crm_name,
        "usertype": r.usertype.to_string(),
        "integral": r.integral,
        "order_remark": r.order_remark,
        "username": r.username,
        "comname": r.comname,
        "bankname": r.bank_name,
        "bankid": r.bank_id,
    })
}

fn new_dingdan(now: i64) -> String {
    format!("{now}{:05}", (now % 90_000) + 10_000)
}

async fn finance_order_query(state: &AppState, body: &Value) -> AppResult<OrderQ> {
    let typezf = json_str(body, "typezf");
    let order_id_kw = json_str(body, "keyword");
    let typeca = json_i32(body, "typeca");
    let mut uid_in = Vec::new();
    let mut order_id_owned = String::new();
    if !order_id_kw.is_empty() {
        match typeca {
            2 => {
                uid_in = vip_repo::find_member_uids_like(state.db.reader(), &order_id_kw).await?;
            }
            3 => {
                uid_in = vip_repo::find_company_uids_like(state.db.reader(), &order_id_kw).await?;
            }
            _ => order_id_owned = order_id_kw,
        }
    }
    let (t0, t1) = json_date_pair(body, "times");
    let time_min = if t0.is_empty() {
        None
    } else {
        Some(parse_date_ts(&t0))
    };
    let time_max = if t1.is_empty() {
        None
    } else {
        Some(parse_date_ts(&t1) + 86_400)
    };
    let mut ids = ids_of(body);
    if ids.is_empty() {
        match body.get("uid") {
            Some(Value::Array(a)) => {
                ids = a.iter().map(json_u64_val).filter(|n| *n > 0).collect();
            }
            Some(Value::String(s)) => {
                ids = s
                    .split([',', ';'])
                    .filter_map(|x| x.trim().parse().ok())
                    .filter(|n: &u64| *n > 0)
                    .collect();
            }
            Some(Value::Number(n)) => {
                ids = n.as_u64().filter(|n| *n > 0).into_iter().collect();
            }
            _ => {}
        }
    }
    Ok(OrderQ {
        uid: Some(json_u64(body, "comid")).filter(|n| *n > 0),
        usertype: if json_u64(body, "comid") > 0 {
            Some(2)
        } else {
            None
        },
        typezf,
        order_kind: json_present_i32(body, "typedd"),
        rating: json_present_i32(body, "rating"),
        order_state: json_present_i32(body, "order_state"),
        order_id_kw: order_id_owned,
        uid_in,
        time_min,
        time_max,
        ids,
    })
}

struct OrderQ {
    uid: Option<u64>,
    usertype: Option<i32>,
    typezf: String,
    order_kind: Option<i32>,
    rating: Option<i32>,
    order_state: Option<i32>,
    order_id_kw: String,
    uid_in: Vec<u64>,
    time_min: Option<i64>,
    time_max: Option<i64>,
    ids: Vec<u64>,
}

fn order_q_filter<'a>(q: &'a OrderQ) -> vip_repo::PhpOrderFilter<'a> {
    vip_repo::PhpOrderFilter {
        uid: q.uid,
        usertype: q.usertype,
        order_type: if q.typezf.is_empty() {
            None
        } else {
            Some(q.typezf.as_str())
        },
        order_kind: q.order_kind,
        rating: q.rating,
        order_state: q.order_state,
        order_id_kw: if q.order_id_kw.is_empty() {
            None
        } else {
            Some(q.order_id_kw.as_str())
        },
        uid_in: if q.uid_in.is_empty() {
            None
        } else {
            Some(q.uid_in.as_slice())
        },
        time_min: q.time_min,
        time_max: q.time_max,
        ids: if q.ids.is_empty() {
            None
        } else {
            Some(q.ids.as_slice())
        },
    }
}

async fn finance_order_search_type(state: &AppState) -> AppResult<Value> {
    let ratings = company_repo::list_rating_options(state.db.reader()).await?;
    let ratingarr: Vec<Value> = ratings
        .into_iter()
        .map(|r| json!({ "value": r.id, "label": r.name }))
        .collect();
    Ok(json!({
        "pay": {
            "alipay": "支付宝",
            "tenpay": "财富通",
            "bank": "银行转帐",
            "alipaydual": "支付宝双接口",
            "alipayescow": "担保交易",
            "adminpay": "管理员充值",
            "balance": "余额支付",
            "admincut": "管理员扣款",
            "wapalipay": "支付宝手机支付",
        },
        "ordertype": {
            "1": "购买会员",
            "2": "积分充值",
            "3": "银行转帐",
            "4": "金额充值",
            "5": "购买增值包",
            "10": "职位置顶",
            "11": "职位紧急",
            "12": "职位推荐",
            "13": "自动刷新",
            "14": "简历置顶",
            "16": "刷新职位",
            "17": "刷新兼职",
            "19": "下载简历",
            "20": "发布职位",
            "21": "发布兼职",
            "23": "面试邀请",
            "24": "兼职推荐",
            "25": "店铺招聘",
            "28": "招聘会报名",
        },
        "ratingarr": ratingarr,
    }))
}

async fn finance_order_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let mut q = finance_order_query(state, body).await?;
    q.ids.clear();
    let f = order_q_filter(&q);
    let db = state.db.reader();
    let rows = vip_repo::php_list_orders(db, &f, offset, limit).await?;
    let total = vip_repo::php_count_orders(db, &f).await?;
    let sum = vip_repo::php_sum_orders(db, &f).await?;
    let list: Vec<Value> = rows.iter().map(finance_order_json).collect();
    Ok(json!({
        "data": list,
        "total": total,
        "pageSizes": [10, 20, 50, 100],
        "perPage": per,
        "page": page,
        "orderSum": {
            "orderPriceAll": sum.all_price,
            "orderPayed": sum.payed,
            "orderPaying": sum.paying,
            "orderPay": sum.wait_pay,
        },
    }))
}

async fn finance_order_edit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let r = vip_repo::php_find_order(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("common_01237"))?;
    let pricename = setting_repo::find(state.db.reader(), "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    Ok(json!({
        "detail": finance_order_json(&r),
        "htpics": [],
        "integral_pricename": pricename,
    }))
}

async fn finance_order_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let old = vip_repo::php_find_order(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("common_01237"))?;
    let price = json_str(body, "order_price");
    let mut remark = json_str(body, "order_remark");
    let new_oid = if price != old.order_price {
        let oid = new_dingdan(clock::now_ts());
        remark = format!("{remark} 改价 {} -> {price}", old.order_id);
        Some(oid)
    } else {
        None
    };
    vip_repo::php_update_order(
        state.db.pool(),
        id,
        &price,
        &remark,
        new_oid.as_deref(),
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_order_setpay(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let row = vip_repo::php_find_order(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("common_01237"))?;
    if row.order_state != 1 && row.order_state != 3 {
        return Err(ApiError::business("common_00735"));
    }
    if row.r#type == 2 && row.integral > 0 && row.uid > 0 {
        let _ = cstatis_repo::add_integral(state.db.pool(), row.uid, i64::from(row.integral)).await;
    }
    vip_repo::php_set_order_state(state.db.pool(), id, 2).await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_order_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    vip_repo::php_delete_orders(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_order_xls(state: &AppState, body: &Value) -> AppResult<Value> {
    let q = finance_order_query(state, body).await?;
    let f = order_q_filter(&q);
    let rows = vip_repo::php_list_orders(state.db.reader(), &f, 0, 5000).await?;
    if rows.is_empty() {
        return Err(ApiError::business("admin_yunying_00004"));
    }
    let mut csv = String::from("id,username,comname,order_id,order_type,type,order_price,order_time,order_state,crm_name\n");
    for r in &rows {
        let state_plain = match r.order_state {
            0 => "支付失败",
            1 => "等待付款",
            2 => "支付成功",
            3 => "等待确认",
            4 => "交易关闭",
            _ => "",
        };
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            r.id,
            csv_cell(&r.username),
            csv_cell(&r.comname),
            csv_cell(&r.order_id),
            csv_cell(pay_name(&r.order_type)),
            csv_cell(order_kind_name(r.r#type)),
            r.order_price,
            fmt_dt(r.order_time),
            csv_cell(state_plain),
            csv_cell(&r.crm_name),
        ));
    }
    let file = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, csv.as_bytes());
    Ok(json!({
        "file": file,
        "file_name": format!("orders-{}.csv", fmt_date(clock::now_ts())),
        "status": 1,
    }))
}

fn csv_cell(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

async fn finance_pay_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ty = json_i32(body, "type");
    let mut uid_in = Vec::new();
    let mut order_id = String::new();
    let mut remark = String::new();
    if !kw.is_empty() {
        match ty {
            2 => uid_in = vip_repo::find_member_uids_like(state.db.reader(), &kw).await?,
            3 => remark = kw,
            _ => order_id = kw,
        }
    }
    let end = json_i32(body, "end");
    let time_min = if end == 1 {
        Some(days_ago_ts(1))
    } else if end > 1 {
        Some(days_ago_ts(end))
    } else {
        None
    };
    let f = pay_repo::PhpPayFilter {
        com_id: Some(json_u64(body, "comid")).filter(|n| *n > 0),
        usertype: if json_u64(body, "comid") > 0 {
            Some(2)
        } else {
            None
        },
        order_id_kw: if order_id.is_empty() { None } else { Some(order_id.as_str()) },
        remark_kw: if remark.is_empty() { None } else { Some(remark.as_str()) },
        uid_in: if uid_in.is_empty() { None } else { Some(uid_in.as_slice()) },
        pay_state: json_present_i32(body, "pay_state"),
        time_min,
    };
    let db = state.db.reader();
    let rows = pay_repo::php_list_pay(db, &f, offset, limit).await?;
    let total = pay_repo::php_count_pay(db, &f).await?;
    let pricename = setting_repo::find(db, "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let price_str = if r.r#type == 1 {
                format!("{}{}", r.order_price, pricename)
            } else {
                format!("{}元", r.order_price)
            };
            json!({
                "id": r.id,
                "order_id": r.order_id,
                "order_price": r.order_price,
                "price_str": price_str,
                "pay_time": fmt_dt(r.pay_time),
                "pay_state": r.pay_state.to_string(),
                "pay_state_n": pay_state_html(r.pay_state),
                "pay_remark": r.pay_remark,
                "username": r.username,
                "comname": r.comname,
                "type": r.r#type,
            })
        })
        .collect();
    Ok(json!({
        "data": list,
        "total": total,
        "pageSizes": [10, 20, 50, 100],
        "perPage": per,
        "page": page,
    }))
}

async fn finance_pay_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    pay_repo::php_delete_pay(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_recharge_index(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let pkgs = gap_repo::list_rating_packages(db, None, 0, 200).await?;
    let rating_list: Vec<Value> = pkgs
        .iter()
        .map(|r| {
            json!({
                "id": r.id,
                "name": r.name,
                "service_price": r.service_price,
                "service_time": r.service_time,
            })
        })
        .collect();
    let ratingid = pkgs
        .iter()
        .find(|r| r.service_time == 0)
        .map(|r| r.id)
        .or_else(|| pkgs.first().map(|r| r.id))
        .unwrap_or(0);
    let services = gap_extra::list_rating_services(db).await?;
    let service_list: Vec<Value> = services
        .into_iter()
        .filter(|s| s.display == 1)
        .map(|s| json!({ "id": s.id, "name": s.name, "display": s.display, "sort": s.sort }))
        .collect();
    let pricename = setting_repo::find(db, "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let priceunit = setting_repo::find(db, "integral_priceunit")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    Ok(json!({
        "rating_list": rating_list,
        "ratingid": ratingid,
        "service_list": service_list,
        "integral_pricename": pricename,
        "integral_priceunit": priceunit,
    }))
}

async fn finance_jifen_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let userarr = json_str(body, "userarr");
    if userarr.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let integral = json_i32(body, "integral");
    if integral < 1 {
        return Err(ApiError::business("admin_yunying_00006"));
    }
    let fs = json_i32(body, "fs");
    let delta = if fs == 2 { -i64::from(integral) } else { i64::from(integral) };
    let names: Vec<String> = userarr
        .replace('，', ",")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let members = gap_repo::find_members_by_usernames(state.db.pool(), &names).await?;
    if members.is_empty() {
        return Err(ApiError::business("wap_js_00103"));
    }
    let now = clock::now_ts();
    let remark = json_str(body, "remark");
    let price = json_str(body, "order_price");
    let order_type = if fs == 2 { "admincut" } else { "adminpay" };
    let kind = if fs == 2 { 5 } else { 2 };
    for (uid, usertype, _) in &members {
        if *usertype == 2 {
            cstatis_repo::adjust_integral(state.db.pool(), *uid, delta).await?;
        } else {
            mstatis_repo::add_balance(state.db.pool(), *uid, delta, now).await?;
        }
        let oid = new_dingdan(now);
        vip_repo::php_insert_order(
            state.db.pool(),
            vip_repo::PhpOrderInsert {
                order_id: &oid,
                uid: *uid,
                order_type,
                order_price: &price,
                order_time: now,
                order_state: 2,
                order_remark: &remark,
                r#type: kind,
                rating: 0,
                integral,
                usertype: *usertype,
            },
        )
        .await?;
        pay_repo::php_insert_pay(
            state.db.pool(),
            &oid,
            &integral.to_string(),
            now,
            *uid,
            &remark,
            1,
            *usertype,
        )
        .await?;
    }
    Ok(PhpOut::Message("ok"))
}

async fn finance_comvip(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("model_00025"));
    }
    let ratingid = json_u64(body, "ratingid");
    if ratingid == 0 {
        return Err(ApiError::business("admin_yunying_00091"));
    }
    let vipprice = json_str(body, "vipprice");
    if vipprice.is_empty() {
        return Err(ApiError::business("common_01408"));
    }
    let pkgs = gap_repo::list_rating_packages(state.db.reader(), Some(ratingid), 0, 1).await?;
    let pkg = pkgs
        .into_iter()
        .next()
        .ok_or_else(|| ApiError::business("admin_yunying_00091"))?;
    let now = clock::now_ts();
    let etime = if pkg.service_time == 0 {
        0
    } else {
        let s = json_str(body, "vipetime");
        if s.is_empty() {
            return Err(ApiError::business("model_00026"));
        }
        parse_date_ts(&s)
    };
    company_repo::set_rating(state.db.pool(), uid, pkg.id as i32, &pkg.name).await?;
    company_repo::set_vip_times(state.db.pool(), uid, now, etime).await?;
    if let Some(mut st) = cstatis_repo::find_admin(state.db.reader(), uid).await? {
        st.rating = pkg.id as i32;
        st.rating_name = pkg.name.clone();
        st.vip_stime = now;
        st.vip_etime = etime;
        let _ = cstatis_repo::update_admin_quotas(state.db.pool(), uid, &st).await?;
    }
    let oid = new_dingdan(now);
    let remark = json_str(body, "remark");
    vip_repo::php_insert_order(
        state.db.pool(),
        vip_repo::PhpOrderInsert {
            order_id: &oid,
            uid,
            order_type: "adminpay",
            order_price: &vipprice,
            order_time: now,
            order_state: 2,
            order_remark: &remark,
            r#type: 1,
            rating: pkg.id as i32,
            integral: 0,
            usertype: 2,
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_comservice(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("model_00025"));
    }
    let pkg = json_u64(body, "service_package");
    if pkg == 0 {
        return Err(ApiError::business("common_01307"));
    }
    let price = json_str(body, "service_price");
    if price.is_empty() {
        return Err(ApiError::business("common_01408"));
    }
    let detail = gap_extra::find_rating_detail(state.db.reader(), pkg)
        .await?
        .ok_or_else(|| ApiError::business("common_01237"))?;
    cstatis_repo::add_service_nums(
        state.db.pool(),
        uid,
        detail.job_num,
        detail.breakjob_num,
        detail.resume,
        detail.interview,
        detail.zph_num,
        detail.top_num,
        detail.rec_num,
        detail.urgent_num,
    )
    .await?;
    let now = clock::now_ts();
    let oid = new_dingdan(now);
    vip_repo::php_insert_order(
        state.db.pool(),
        vip_repo::PhpOrderInsert {
            order_id: &oid,
            uid,
            order_type: "adminpay",
            order_price: &price,
            order_time: now,
            order_state: 2,
            order_remark: "common_01293",
            r#type: 5,
            rating: pkg as i32,
            integral: 0,
            usertype: 2,
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn finance_getservice(state: &AppState, body: &Value) -> AppResult<Value> {
    let ty = json_u64(body, "type");
    let rows = gap_extra::list_rating_details(state.db.reader(), ty).await?;
    if rows.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    Ok(json!(rows))
}

async fn finance_searchname(state: &AppState, body: &Value, by_user: bool) -> AppResult<Value> {
    let kw = if by_user {
        json_str(body, "username")
    } else {
        json_str(body, "comname")
    };
    if kw.is_empty() {
        return Ok(json!({ "error": -1, "namelist": [] }));
    }
    let rows = vip_repo::search_member_companies(
        state.db.reader(),
        if by_user { Some(kw.as_str()) } else { None },
        if by_user { None } else { Some(kw.as_str()) },
    )
    .await?;
    if rows.is_empty() {
        return Ok(json!({ "error": -1, "namelist": [] }));
    }
    let namelist: Vec<Value> = rows
        .into_iter()
        .map(|(uid, username, comname, rating_name, vipetime)| {
            json!({
                "uid": uid,
                "username": username,
                "comname": comname,
                "rating_name": rating_name,
                "vipetime": vipetime,
                "vipetime_ymd": if vipetime > 0 { fmt_date(vipetime) } else { "common_01936".into() },
            })
        })
        .collect();
    Ok(json!({ "error": 0, "namelist": namelist }))
}

async fn cfg_of(state: &AppState, key: &str) -> String {
    setting_repo::find(state.db.reader(), key)
        .await
        .ok()
        .flatten()
        .map(|s| s.value)
        .unwrap_or_default()
}

async fn upsert_cfg(
    state: &AppState,
    user: &AuthenticatedUser,
    key: &str,
    value: &str,
) -> AppResult<()> {
    site_setting_service::admin_upsert(
        state,
        user,
        site_setting_service::UpsertInput {
            key,
            value,
            description: "",
            is_public: true,
        },
    )
    .await
}

async fn once_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let now = clock::now_ts();
    let f = once_repo::AdminOncePhpFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
        list_status: json_opt_i32(body, "status").filter(|v| *v > 0),
        ctime_min: php_time_min(body),
        now,
    };
    let db = state.db.reader();
    let total = once_repo::admin_php_count(db, &f).await?;
    let rows = if total > 0 {
        once_repo::admin_php_list(
            db,
            &f,
            offset,
            limit,
            &json_str(body, "t"),
            &json_str(body, "order"),
        )
        .await?
    } else {
        Vec::new()
    };
    let dicts = dict_service::get(state).await?;
    let base = preview_base(state);
    let icon = cfg_of(state, "sy_once_icon").await;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let expired = r.edate > 0 && r.edate < now;
            let pic = if r.pic.is_empty() { icon.clone() } else { r.pic.clone() };
            json!({
                "id": r.id,
                "title": r.title,
                "companyname": r.companyname,
                "linkman": trunc_chars(&r.linkman, 5),
                "phone": r.phone,
                "provinceid": r.provinceid,
                "cityid": r.cityid,
                "three_cityid": r.three_cityid,
                "address": r.address,
                "require": r.require,
                "salary": r.salary,
                "status": if expired { 2 } else { r.status },
                "ctime": r.ctime,
                "ctime_n": fmt_date(r.ctime),
                "edate": r.edate,
                "edate_n": fmt_date(r.edate),
                "did": r.did,
                "pic": r.pic,
                "pic_n": pic_url(&base, &pic),
                "yyzz": r.yyzz,
                "yyzz_n": pic_url(&base, &r.yyzz),
                "hits": r.hits,
                "pay": r.pay,
                "city_n": city_n(&dicts, r.provinceid, r.cityid, r.three_cityid),
                "once_url": format!("{base}/index.php?m=once&c=show&id={}", r.id),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn once_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let now = clock::now_ts();
    let mut out = serde_json::Map::new();
    let all = once_repo::count_all(db).await?;
    if all > 0 {
        out.insert("onceAllNum".into(), json!(all));
    }
    let pending = once_repo::count_pending_unexpired(db, now).await?;
    if pending > 0 {
        out.insert("onceStatusNum1".into(), json!(pending));
    }
    let expired = once_repo::count_expired(db, now).await?;
    if expired > 0 {
        out.insert("onceStatusNum2".into(), json!(expired));
    }
    Ok(Value::Object(out))
}

async fn once_status(state: &AppState, body: &Value) -> AppResult<Value> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if once_repo::count_pay_eq(state.db.reader(), &ids, 1).await? > 0 {
        return Ok(json!({ "status": 3 }));
    }
    let raw = json_i32(body, "status");
    let db_status = if raw == 2 { 1 } else { raw };
    once_repo::admin_set_status_ids(state.db.pool(), &ids, db_status).await?;
    Ok(json!({ "status": db_status }))
}

async fn once_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    once_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00143"))
}

async fn once_price_gear(state: &AppState) -> AppResult<Value> {
    let list = once_repo::list_price_gears(state.db.reader()).await?;
    Ok(json!({ "list": list }))
}

async fn once_price_gear_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let days = json_i32(body, "days");
    if days <= 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if once_repo::find_price_gear_by_days(state.db.pool(), days, 0)
        .await?
        .is_some()
    {
        return Err(ApiError::business("admin_user_00103"));
    }
    let _ = once_repo::insert_price_gear(state.db.pool(), days, json_f64(body, "price")).await?;
    Ok(PhpOut::Message("ok"))
}

async fn once_price_gear_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let days = if has_flag(body, "days") {
        Some(json_i32(body, "days"))
    } else {
        None
    };
    if let Some(d) = days {
        if d > 0 {
            if once_repo::find_price_gear_by_days(state.db.pool(), d, id)
                .await?
                .is_some()
            {
                return Err(ApiError::business("admin_user_00103"));
            }
        }
    }
    let price = if body.get("price").is_some() {
        Some(json_f64(body, "price"))
    } else {
        None
    };
    let n = once_repo::update_price_gear(state.db.pool(), id, days, price).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01281"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn once_price_gear_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    once_repo::delete_price_gears(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn once_set(state: &AppState) -> AppResult<Value> {
    let icon = cfg_of(state, "sy_once_icon").await;
    Ok(json!({
        "config": {
            "sy_once": cfg_of(state, "sy_once").await,
            "sy_once_totalnum": cfg_of(state, "sy_once_totalnum").await,
            "user_wzp_link": cfg_of(state, "user_wzp_link").await,
            "com_fast_status": cfg_of(state, "com_fast_status").await,
            "sy_once_yyzz": cfg_of(state, "sy_once_yyzz").await,
            "com_xin": cfg_of(state, "com_xin").await,
            "sy_once_icon_n": pic_url(&preview_base(state), &icon),
        }
    }))
}

async fn once_onceset(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    for key in [
        "sy_once",
        "sy_once_totalnum",
        "user_wzp_link",
        "com_fast_status",
        "sy_once_yyzz",
        "com_xin",
    ] {
        if body.get(key).is_some() {
            upsert_cfg(state, user, key, &json_str(body, key)).await?;
        }
    }
    let icon = json_str(body, "sy_once_icon");
    if !icon.is_empty() && !icon.starts_with("data:") && !icon.contains("blob:") {
        upsert_cfg(state, user, "sy_once_icon", &icon).await?;
    }
    Ok(PhpOut::Message("admin_user_00106"))
}

async fn once_edit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let info = if id > 0 {
        once_repo::find_admin(state.db.reader(), id).await?
    } else {
        None
    };
    let base = preview_base(state);
    Ok(json!({
        "info": info.map(|r| json!({
            "id": r.id,
            "title": r.title,
            "companyname": r.companyname,
            "linkman": r.linkman,
            "phone": r.phone,
            "provinceid": r.provinceid,
            "cityid": r.cityid,
            "three_cityid": r.three_cityid,
            "address": r.address,
            "require": r.require,
            "require_n": r.require,
            "salary": r.salary,
            "status": r.status,
            "ctime": r.ctime,
            "ctime_n": fmt_dt(r.ctime),
            "edate": r.edate,
            "edate_n": if r.edate > 0 { fmt_date(r.edate) } else { String::new() },
            "did": r.did,
            "pic": r.pic,
            "pic_n": pic_url(&base, &r.pic),
            "yyzz": r.yyzz,
            "yyzz_n": pic_url(&base, &r.yyzz),
            "hits": r.hits,
            "password": "",
            "city_n": "",
        })).unwrap_or(json!({}))
    }))
}

async fn once_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if has_flag(body, "yyzz") || has_flag(body, "file") {
        return Err(ApiError::business("upload_not_supported"));
    }
    let days = json_i32(body, "edate");
    let now = clock::now_ts();
    let edate = now + i64::from(days.max(0)) * 86_400;
    let pwd = json_str(body, "password");
    let hashed = if pwd.is_empty() {
        None
    } else {
        Some(md5_hex(&pwd))
    };
    let hashed_ref = hashed.as_deref();
    let _ = once_repo::admin_save(
        state.db.pool(),
        json_u64(body, "id"),
        &once_repo::AdminOnceSave {
            title: &json_str(body, "title"),
            companyname: &json_str(body, "companyname"),
            linkman: &json_str(body, "linkman"),
            phone: &json_str(body, "phone"),
            provinceid: json_i32(body, "provinceid"),
            cityid: json_i32(body, "cityid"),
            three_cityid: json_i32(body, "three_cityid"),
            address: &json_str(body, "address"),
            require: &json_str(body, "require"),
            salary: &json_str(body, "salary"),
            password_md5: hashed_ref,
            edate,
            did: json_i32(body, "did"),
            now,
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn once_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    once_repo::delete_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn once_ctime(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    let days = json_i32(body, "endtime");
    if ids.is_empty() || days <= 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    once_repo::extend_edate(state.db.pool(), &ids, days, clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

async fn once_refresh(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    once_repo::refresh_ctime(state.db.pool(), &ids, clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

async fn tiny_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ui_status = json_opt_i32(body, "status").filter(|v| *v > 0);
    let db_status = match ui_status {
        Some(2) => Some(0),
        Some(1) => Some(1),
        _ => None,
    };
    let f = tiny_repo::AdminTinyPhpFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
        status: db_status,
        sex: json_opt_i32(body, "sex").filter(|v| *v > 0),
        exp: json_opt_i32(body, "exp").filter(|v| *v > 0),
        time_min: php_time_min(body),
    };
    let db = state.db.reader();
    let total = tiny_repo::admin_php_count(db, &f).await?;
    let rows = if total > 0 {
        tiny_repo::admin_php_list(
            db,
            &f,
            offset,
            limit,
            &json_str(body, "t"),
            &json_str(body, "order"),
        )
        .await?
    } else {
        Vec::new()
    };
    let dicts = dict_service::get(state).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let city_one = if r.provinceid > 0 {
                dicts.city(r.provinceid).to_string()
            } else {
                String::new()
            };
            let city_two = if r.cityid > 0 {
                format!("-{}", dicts.city(r.cityid))
            } else {
                String::new()
            };
            let city_three = if r.three_cityid > 0 {
                format!("-{}", dicts.city(r.three_cityid))
            } else {
                String::new()
            };
            json!({
                "id": r.id,
                "username": r.username,
                "sex": r.sex,
                "sex_n": sex_n(r.sex),
                "exp": r.exp,
                "exp_n": dicts.userclass(r.exp),
                "job": r.job,
                "mobile": r.mobile,
                "provinceid": r.provinceid,
                "cityid": r.cityid,
                "three_cityid": r.three_cityid,
                "city_one": city_one,
                "city_two": city_two,
                "city_three": city_three,
                "production": r.production,
                "status": r.status,
                "time": r.time,
                "time_n": fmt_dt(r.time),
                "lastupdate": r.lastupdate,
                "lastupdate_n": fmt_date(r.lastupdate),
                "did": r.did,
                "hits": r.hits,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn tiny_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let mut out = serde_json::Map::new();
    let all = tiny_repo::count_all(db).await?;
    if all > 0 {
        out.insert("tinyAllNum".into(), json!(all));
    }
    let pending = tiny_repo::count_by_status(db, 0).await?;
    if pending > 0 {
        out.insert("tinyStatusNum".into(), json!(pending));
    }
    Ok(Value::Object(out))
}

async fn tiny_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = tiny_repo::admin_set_status_ids(state.db.pool(), &ids, json_i32(body, "status")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00113"));
    }
    Ok(PhpOut::Message("admin_01324"))
}

async fn tiny_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    tiny_repo::set_did_ids(state.db.pool(), &ids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00142"))
}

async fn tiny_set(state: &AppState) -> AppResult<Value> {
    Ok(json!({
        "config": {
            "sy_tiny": cfg_of(state, "sy_tiny").await,
            "sy_tiny_totalnum": cfg_of(state, "sy_tiny_totalnum").await,
            "user_wjl": cfg_of(state, "user_wjl").await,
            "user_wjl_link": cfg_of(state, "user_wjl_link").await,
        }
    }))
}

async fn tiny_tinyset(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    for key in ["sy_tiny", "sy_tiny_totalnum", "user_wjl", "user_wjl_link"] {
        if body.get(key).is_some() {
            upsert_cfg(state, user, key, &json_str(body, key)).await?;
        }
    }
    Ok(PhpOut::Message("admin_user_00112"))
}

async fn tiny_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let pwd = json_str(body, "password");
    let hashed = if pwd.is_empty() {
        None
    } else {
        Some(md5_hex(&pwd))
    };
    let hashed_ref = hashed.as_deref();
    let _ = tiny_repo::admin_save(
        state.db.pool(),
        json_u64(body, "id"),
        &tiny_repo::AdminTinySave {
            username: &json_str(body, "username"),
            sex: json_i32(body, "sex"),
            exp: json_i32(body, "exp"),
            job: &json_str(body, "job"),
            mobile: &json_str(body, "mobile"),
            provinceid: json_i32(body, "provinceid"),
            cityid: json_i32(body, "cityid"),
            three_cityid: json_i32(body, "three_cityid"),
            production: &json_str(body, "production"),
            password_md5: hashed_ref,
            now: clock::now_ts(),
            did: json_u64(body, "did") as u32,
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn tiny_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    tiny_repo::delete_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn tiny_refresh(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    tiny_repo::refresh_ids(state.db.pool(), &ids, clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

fn part_show_json(p: &phpyun_models::part::entity::PartJob, statusbody: &str) -> Value {
    let worktime_n: Vec<String> = p
        .worktime
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    json!({
        "id": p.id,
        "uid": p.uid,
        "name": p.name,
        "com_name": p.com_name,
        "type": p.r#type,
        "provinceid": p.provinceid,
        "cityid": p.cityid,
        "three_cityid": p.three_cityid,
        "address": p.address,
        "number": p.number,
        "sex": p.sex,
        "salary": p.salary,
        "salary_type": p.salary_type,
        "billing_cycle": p.billing_cycle,
        "worktime": p.worktime,
        "worktime_n": worktime_n,
        "workcishu": worktime_n.len(),
        "sdate": p.sdate,
        "sdate_n": if p.sdate > 0 { fmt_date(p.sdate) } else { String::new() },
        "edate": p.edate,
        "edate_n": if p.edate > 0 { fmt_date(p.edate) } else { String::new() },
        "content": p.content,
        "linkman": p.linkman,
        "linktel": p.linktel,
        "state": p.state,
        "status": p.status,
        "statusbody": statusbody,
        "r_status": p.r_status,
        "rec_time": p.rec_time,
        "lastupdate": p.lastupdate,
        "addtime": p.addtime,
        "did": p.did,
        "x": p.x,
        "y": p.y,
    })
}

async fn part_show(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if has_flag(body, "update") {
        let id = json_u64(body, "id");
        if id == 0 {
            return Err(ApiError::param_invalid("wap_com_00228"));
        }
        let r_status = json_i32(body, "r_status");
        let job_state = if r_status == 1 { 1 } else { 0 };
        let edate = if has_flag(body, "timetype") {
            0
        } else {
            parse_date_ts(&json_str(body, "edate"))
        };
        part_repo::admin_update_info(
            state.db.pool(),
            id,
            &part_repo::AdminPartSave {
                name: &json_str(body, "name"),
                r#type: json_i32(body, "type"),
                sdate: parse_date_ts(&json_str(body, "sdate")),
                edate,
                worktime: &json_csv(body, "worktime"),
                number: json_i32(body, "number"),
                sex: json_i32(body, "sex"),
                salary: json_i32(body, "salary"),
                salary_type: json_i32(body, "salary_type"),
                billing_cycle: json_i32(body, "billing_cycle"),
                provinceid: json_i32(body, "provinceid"),
                cityid: json_i32(body, "cityid"),
                three_cityid: json_i32(body, "three_cityid"),
                address: &json_str(body, "address"),
                r_status,
                x: &json_str(body, "x"),
                y: &json_str(body, "y"),
                content: &json_str(body, "content"),
                linkman: &json_str(body, "linkman"),
                linktel: &json_str(body, "linktel"),
                state: job_state,
                now: clock::now_ts(),
            },
        )
        .await?;
        return Ok(PhpOut::Message("ok"));
    }
    let id = json_u64(body, "id");
    let p = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let statusbody = part_repo::get_statusbody(state.db.reader(), id).await?;
    let company = company_repo::find_by_uid(state.db.reader(), p.uid).await?;
    Ok(PhpOut::Data(json!({
        "show": part_show_json(&p, &statusbody),
        "company": company.map(|c| json!({"uid": c.uid, "r_status": c.r_status})).unwrap_or(json!({})),
        "today": fmt_date(clock::now_ts()),
    })))
}

async fn part_audit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let p = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let statusbody = part_repo::get_statusbody(state.db.reader(), id).await?;
    let mut info = part_show_json(&p, &statusbody);
    if let Some(m) = user_repo::find_admin_extras(state.db.reader(), p.uid).await? {
        info["c_status"] = json!(m.status);
        info["lock_info"] = json!(m.lock_info);
    }
    let snum = part_repo::count_pending_except(state.db.reader(), id).await?;
    Ok(json!({ "info": info, "snum": snum }))
}

async fn part_recommend(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "pid");
    if ids.is_empty() {
        ids = ids_of(body);
    }
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if json_i32(body, "s") == 1 {
        part_repo::set_rec_time(state.db.pool(), &ids, 0).await?;
    } else {
        part_repo::add_rec_days(state.db.pool(), &ids, json_i32(body, "days"), clock::now_ts())
            .await?;
    }
    Ok(PhpOut::Message("ok"))
}

async fn part_ctime(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "jobid");
    if ids.is_empty() {
        ids = ids_of(body);
    }
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    part_repo::extend_edate(state.db.pool(), &ids, json_i32(body, "days"), clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

async fn part_refresh(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    part_repo::refresh_lastupdate(state.db.pool(), &ids, clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

async fn part_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    part_repo::cascade_delete_children(state.db.pool(), &ids).await?;
    part_repo::delete_by_ids(state.db.pool(), &ids, None).await?;
    Ok(PhpOut::Message("ok"))
}

async fn part_checkstate(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let mut st = json_i32(body, "state");
    if id == 0 || st == 0 {
        return Ok(PhpOut::Message("ok"));
    }
    if st == 2 {
        st = 0;
    }
    part_repo::set_publish_status(state.db.pool(), id, st).await?;
    Ok(PhpOut::Message("ok"))
}

async fn hotjob_com_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Ok(json!([]));
    }
    let rows = company_repo::search_brief(state.db.reader(), &name, 20).await?;
    Ok(json!(rows
        .into_iter()
        .map(|c| json!({"value": c.uid, "label": c.name}))
        .collect::<Vec<_>>()))
}

fn hotjob_php_json(h: &company_repo::HotJobRow, base: &str) -> Value {
    json!({
        "id": h.id,
        "uid": h.uid,
        "username": h.username,
        "hot_pic": h.hot_pic,
        "hot_pic_n": pic_url(base, &h.hot_pic),
        "time_start": h.time_start,
        "time_start_n": if h.time_start > 0 { fmt_date(h.time_start) } else { String::new() },
        "time_end": h.time_end,
        "time_end_n": if h.time_end > 0 { fmt_date(h.time_end) } else { String::new() },
        "sort": h.sort,
        "beizhu": h.beizhu,
        "rating_id": h.rating_id,
    })
}

async fn hotjob_get(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let base = preview_base(state);
    let com = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    if com.rec == 1 {
        if let Some(h) = company_repo::hotjob_find_by_uid(state.db.reader(), uid).await? {
            return Ok(hotjob_php_json(&h, &base));
        }
    }
    let st = cstatis_repo::find_admin(state.db.reader(), uid).await?;
    let logo = com.logo.clone().unwrap_or_default();
    Ok(json!({
        "uid": com.uid,
        "username": com.name,
        "hot_pic": logo,
        "hot_pic_n": pic_url(&base, &logo),
        "rating_id": st.as_ref().map(|s| s.rating).unwrap_or(0),
        "rating": st.as_ref().map(|s| s.rating_name.clone()).unwrap_or_default(),
        "time_start": clock::now_ts(),
        "time_start_n": fmt_date(clock::now_ts()),
        "time_end": 0,
        "time_end_n": "",
        "sort": 0,
        "beizhu": "",
    }))
}

async fn hotjob_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let uid = json_u64(body, "uid");
    let base = preview_base(state);
    if id > 0 {
        if let Some(h) = company_repo::hotjob_find_by_uid(state.db.reader(), id).await? {
            return Ok(hotjob_php_json(&h, &base));
        }
        if let Some(h) = company_repo::hotjob_find_by_id(state.db.reader(), id).await? {
            return Ok(hotjob_php_json(&h, &base));
        }
    }
    if uid > 0 {
        return hotjob_get(state, body).await;
    }
    Ok(json!({}))
}

async fn hotjob_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    if has_flag(body, "mqlogo") {
        return Err(ApiError::business("upload_not_supported"));
    }
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let existing = company_repo::hotjob_find_by_uid(state.db.reader(), uid).await?;
    let mut hot_pic = json_str(body, "hot_pic");
    if hot_pic.is_empty() {
        if let Some(h) = existing.as_ref() {
            hot_pic = h.hot_pic.clone();
        } else if let Some(c) = company_repo::find_by_uid(state.db.reader(), uid).await? {
            hot_pic = c.logo.unwrap_or_default();
        }
    }
    let id = json_u64(body, "id");
    if id == 0 && existing.is_none() && hot_pic.is_empty() {
        return Err(ApiError::business("admin_user_00072"));
    }
    let username = json_str(body, "username");
    let beizhu = json_str(body, "beizhu");
    admin_cms_service::upsert_hotjob(
        state,
        user,
        admin_cms_service::HotJobUpsertIn {
            id: if id > 0 { Some(id) } else { existing.map(|h| h.id) },
            uid,
            username: &username,
            hot_pic: &hot_pic,
            time_start: parse_date_ts(&json_str(body, "time_start_n")),
            time_end: parse_date_ts(&json_str(body, "time_end_n")),
            sort: json_i32(body, "sort"),
            beizhu: &beizhu,
            rating_id: json_i32(body, "rating_id"),
        },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn hotjob_num(state: &AppState) -> AppResult<Value> {
    let n = company_repo::hotjob_count(state.db.reader()).await?;
    Ok(json!({ "all": n }))
}

async fn resume_skill(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let eid = json_u64(body, "eid");
    if uid == 0 || eid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let id = skill_repo::php_upsert(
        state.db.pool(),
        json_u64(body, "id"),
        uid,
        eid,
        &json_str(body, "name"),
        json_i32(body, "ing"),
        json_i32(body, "longtime"),
    )
    .await?;
    Ok(PhpOut::Data(json!({ "id": id })))
}

async fn resume_project(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let eid = json_u64(body, "eid");
    if uid == 0 || eid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let title = json_str(body, "title");
    let content = json_str(body, "content");
    let input = project_repo::ProjectInput {
        name: &json_str(body, "name"),
        sdate: parse_date_ts(&json_str(body, "sdate")),
        edate: parse_date_ts(&json_str(body, "edate")),
        role: Some(title.as_str()),
        content: Some(content.as_str()),
    };
    let id = json_u64(body, "id");
    let nid = if id > 0 {
        project_repo::update(state.db.pool(), id, uid, &input).await?;
        id
    } else {
        project_repo::create(state.db.pool(), uid, eid, &input).await?
    };
    Ok(PhpOut::Data(json!({ "id": nid })))
}

async fn resume_other(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let eid = json_u64(body, "eid");
    if uid == 0 || eid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let input = other_repo::OtherInput {
        name: &json_str(body, "name"),
        content: &json_str(body, "content"),
    };
    let id = json_u64(body, "id");
    let nid = if id > 0 {
        other_repo::update(state.db.pool(), id, uid, &input).await?;
        id
    } else {
        other_repo::create(state.db.pool(), uid, eid, &input).await?
    };
    Ok(PhpOut::Data(json!({ "id": nid })))
}

async fn pages_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let is_type = json_opt_i32(body, "is_type");
    let db = state.db.reader();
    let rows = desc_repo::php_list(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        is_type,
        offset,
        limit,
    )
    .await?;
    let total = desc_repo::php_count(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        is_type,
    )
    .await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "name": r.name,
                "title": r.title,
                "is_type": r.is_type,
                "is_type_n": desc_is_type_n(r.is_type),
                "is_nav": r.is_nav.to_string(),
                "sort": r.sort,
                "url": r.url,
                "url_pc": desc_preview_url(state, r.id),
                "ctime": r.ctime,
                "ctime_n": fmt_date(r.ctime),
                "nid": r.nid,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

fn desc_is_type_n(is_type: i32) -> String {
    let lang = i18n::current_lang();
    let key = match is_type {
        1 => "messages.admin_system_00661",
        2 => "messages.admin_00198",
        _ => "messages.admin_system_00663",
    };
    i18n::t(key, lang)
}

fn desc_preview_url(state: &AppState, id: u64) -> String {
    format!("{}/get/{id}", preview_base(state).trim_end_matches('/'))
}

fn desc_content(raw: &str) -> String {
    amp(raw)
        .replace("background-color:#ffffff", "")
        .replace("background-color:#fff", "")
        .replace("white-space:nowrap;", "")
        .replace("<img ", "<img style=\"max-width:100%\" ")
}

/// PHP `singlepage::save_action` static-html path checks.
fn validate_static_html_url(url: &str) -> AppResult<String> {
    let mut u = url.trim().replace('\\', "/");
    if u.contains("..") {
        return Err(ApiError::business("messages.admin_system_00060"));
    }
    if let Some(rest) = u.strip_prefix('/') {
        u = rest.to_string();
    }
    if u.is_empty() || u.split('/').any(|p| p.is_empty() || p == "." || p == "..") {
        return Err(ApiError::business("messages.admin_system_00060"));
    }
    let last = u
        .rsplit('/')
        .next()
        .unwrap_or(u.as_str())
        .to_ascii_lowercase();
    if !last.ends_with(".html") {
        return Err(ApiError::business("messages.admin_system_00058"));
    }
    Ok(u)
}

async fn pages_add(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let info = if id > 0 {
        desc_repo::php_get(state.db.reader(), id).await?
    } else {
        None
    };
    let class = desc_repo::php_list_all_classes(state.db.reader()).await?;
    Ok(json!({
        "info": info.map(|r| json!({
            "id": r.id,
            "name": r.name,
            "title": r.title,
            "content": r.content,
            "is_type": r.is_type.to_string(),
            "is_nav": r.is_nav.to_string(),
            "sort": r.sort,
            "url": r.url,
            "nid": r.nid,
            "keyword": r.keyword,
            "descs": r.descs,
            "description": r.descs,
            "top_tpl": r.top_tpl.to_string(),
            "top_tpl_dir": r.top_tpl_dir,
            "footer_tpl": r.footer_tpl.to_string(),
            "footer_tpl_dir": r.footer_tpl_dir,
        })).unwrap_or(json!({})),
        "class": class,
    }))
}

async fn pages_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let is_type = json_i32(body, "is_type");
    let raw_url = json_str(body, "url");
    let url = if is_type == 1 {
        validate_static_html_url(&raw_url)?
    } else {
        raw_url
    };
    let name = json_str(body, "name");
    let title = json_str(body, "title");
    let keyword = json_str(body, "keyword");
    let descs = json_str(body, "description");
    let content = desc_content(&json_str(body, "content"));
    let top_tpl_dir = json_str(body, "top_tpl_dir");
    let footer_tpl_dir = json_str(body, "footer_tpl_dir");
    let _ = desc_repo::php_upsert(
        state.db.pool(),
        json_u64(body, "id"),
        &desc_repo::PhpDescSave {
            name: &name,
            nid: json_u64(body, "nid"),
            url: &url,
            title: &title,
            keyword: &keyword,
            descs: &descs,
            content: &content,
            sort: json_i32(body, "sort"),
            is_nav: json_i32(body, "is_nav"),
            is_type,
            top_tpl: json_i32(body, "top_tpl"),
            top_tpl_dir: &top_tpl_dir,
            footer_tpl: json_i32(body, "footer_tpl"),
            footer_tpl_dir: &footer_tpl_dir,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn pages_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = desc_repo::php_delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("messages.admin_user_00186"));
    }
    Ok(PhpOut::Message("common_06472"))
}

async fn pages_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    desc_repo::php_set_sort(state.db.pool(), id, json_i32(body, "sort")).await?;
    Ok(PhpOut::Message("ok"))
}

fn desc_class_names(body: &Value) -> Vec<String> {
    json_str(body, "name")
        .split('-')
        .map(|s| {
            let t = s.trim();
            t.chars().take(50).collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .collect()
}

/// PHP `singleclass::index_action`: list + total + perPage (`sy_listnum` default 10).
async fn desc_class_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let page = json_u64(body, "page").max(1) as u32;
    let mut per = json_u64(body, "page_size");
    if per == 0 {
        per = json_u64(body, "limit");
    }
    if per == 0 {
        per = json_u64(body, "perPage");
    }
    if per == 0 {
        per = 10;
    }
    let per = per.clamp(1, 100) as u32;
    let offset = u64::from(page.saturating_sub(1)) * u64::from(per);
    let db = state.db.reader();
    let rows = desc_repo::php_list_classes(db, offset, u64::from(per)).await?;
    let total = desc_repo::php_count_classes(db).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|c| json!({ "id": c.id, "name": c.name, "sort": c.sort }))
        .collect();
    Ok(json!({
        "list": list,
        "total": total,
        "perPage": per,
        "page_size": per,
        "page": page,
    }))
}

/// PHP `addDesClass`: error 1=duplicate, 2=ok, 3=fail.
async fn desc_class_add(state: &AppState, body: &Value) -> AppResult<Value> {
    let names = desc_class_names(body);
    if names.is_empty() {
        return Ok(json!({ "error": 3 }));
    }
    if desc_repo::php_class_names_exist(state.db.pool(), &names).await? {
        return Ok(json!({ "error": 1 }));
    }
    let now = clock::now_ts();
    for n in &names {
        desc_repo::insert_class(state.db.pool(), n, 0, now).await?;
    }
    description_service::invalidate_classes_cache().await;
    Ok(json!({ "error": 2 }))
}

async fn desc_class_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let name = json_str(body, "name");
    let sort = if has_flag(body, "sort") {
        Some(json_i32(body, "sort"))
    } else {
        None
    };
    let name_ref = if has_flag(body, "name") {
        Some(name.as_str())
    } else {
        None
    };
    desc_repo::php_update_class(state.db.pool(), id, name_ref, sort).await?;
    description_service::invalidate_classes_cache().await;
    Ok(PhpOut::Message("ok"))
}

async fn desc_class_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = desc_repo::php_delete_class_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("messages.admin_user_00186"));
    }
    description_service::invalidate_classes_cache().await;
    Ok(PhpOut::Message("common_06471"))
}

async fn job_class_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let name = json_str(body, "name");
    let e_name = json_str(body, "e_name");
    let s_name = json_str(body, "s_name");
    cat_repo::patch_job_class(
        state.db.pool(),
        id,
        if name.is_empty() { None } else { Some(name.as_str()) },
        if has_flag(body, "sort") {
            Some(json_i32(body, "sort"))
        } else {
            None
        },
        if e_name.is_empty() {
            None
        } else {
            Some(e_name.as_str())
        },
        if s_name.is_empty() {
            None
        } else {
            Some(s_name.as_str())
        },
        None,
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn job_class_setrec(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    cat_repo::patch_job_class(
        state.db.pool(),
        id,
        None,
        None,
        None,
        None,
        Some(json_i32(body, "rec")),
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn job_class_get(state: &AppState, body: &Value) -> AppResult<Value> {
    let nid = json_u64(body, "nid");
    let rows = cat_repo::list_children(state.db.reader(), "job", nid).await?;
    Ok(json!(rows
        .into_iter()
        .map(|c| json!({"id": c.id, "name": c.name, "keyid": c.parent_id}))
        .collect::<Vec<_>>()))
}

async fn wx_nav_list(state: &AppState) -> AppResult<Value> {
    let list = wx_nav_repo::list_all(state.db.reader()).await?;
    Ok(json!({ "list": list }))
}

async fn wx_nav_savenav(state: &AppState, body: &Value) -> AppResult<Value> {
    let name = json_str(body, "name");
    if name.is_empty() || body.get("keyid").is_none() {
        return Ok(json!({ "error": 1 }));
    }
    let keyid = json_i32(body, "keyid");
    let nav_type = json_str(body, "type");
    let key = json_str(body, "key");
    let url = json_str(body, "url");
    let appid = json_str(body, "appid");
    let apppage = json_str(body, "apppage");
    if keyid > 0 {
        if nav_type == "click" && key.is_empty() {
            return Ok(json!({ "error": 1 }));
        }
        if nav_type == "miniprogram" && (url.is_empty() || appid.is_empty() || apppage.is_empty()) {
            return Ok(json!({ "error": 1 }));
        }
        if nav_type == "view" && url.is_empty() {
            return Ok(json!({ "error": 1 }));
        }
    }
    let navid = json_u64(body, "navid");
    if wx_nav_repo::count_dup_name(state.db.pool(), &name, keyid, navid).await? > 0 {
        return Ok(json!({ "error": 2 }));
    }
    wx_nav_repo::upsert_php(
        state.db.pool(),
        if navid > 0 { Some(navid) } else { None },
        &name,
        keyid,
        &key,
        &url,
        &nav_type,
        json_i32(body, "sort"),
        &appid,
        &apppage,
    )
    .await?;
    Ok(json!({ "error": 3 }))
}

async fn wx_nav_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    wx_nav_repo::delete_with_children(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn wx_nav_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let name = json_str(body, "name");
    wx_nav_repo::patch_field(
        state.db.pool(),
        id,
        if has_flag(body, "sort") {
            Some(json_i32(body, "sort"))
        } else {
            None
        },
        if name.is_empty() { None } else { Some(name.as_str()) },
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn fairs_comxlscheck(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let zid = json_u64(body, "zid");
    if zid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let f = zph_repo::AdminZphComFilter {
        zid: Some(zid),
        status: None,
        keyword: None,
        keyword_type: 0,
    };
    let n = zph_repo::admin_count_coms(state.db.reader(), &f).await?;
    if n == 0 {
        return Err(ApiError::business("admin_yunying_00004"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn fairs_comxls(state: &AppState, body: &Value) -> AppResult<Value> {
    let zid = json_u64(body, "zid");
    let f = zph_repo::AdminZphComFilter {
        zid: Some(zid).filter(|n| *n > 0),
        status: None,
        keyword: None,
        keyword_type: 0,
    };
    let mut rows = zph_repo::admin_list_coms(state.db.reader(), &f, 0, 5000).await?;
    let want = ids_named(body, "cid");
    if !want.is_empty() {
        rows.retain(|r| want.contains(&r.id));
    }
    if rows.is_empty() {
        return Err(ApiError::business("admin_yunying_00004"));
    }
    let mut csv = String::from("id,uid,com_name,status,sid,cid,bid,jobid,ctime\n");
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            r.id,
            r.uid,
            csv_cell(&r.com_name),
            r.status,
            r.sid,
            r.cid,
            r.bid,
            csv_cell(&r.jobid),
            fmt_dt(r.ctime),
        ));
    }
    let file = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, csv.as_bytes());
    Ok(json!({
        "file": file,
        "file_name": format!("zph-coms-{}.csv", fmt_date(clock::now_ts())),
        "status": 1,
    }))
}

fn cat_kind(body: &Value) -> String {
    let k = json_str(body, "kind");
    if k.is_empty() {
        "city".into()
    } else {
        k
    }
}

fn dash_names(s: &str) -> Vec<String> {
    s.split(['-', '\n', ','])
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn cat_row_json(kind: &str, r: &cat_repo::CatPhpRow, level: i32) -> Value {
    let mut v = json!({
        "id": r.id,
        "name": r.name,
        "sort": r.sort,
        "keyid": r.keyid,
        "variable": r.variable,
    });
    if kind == "city" {
        v["e_name"] = json!(r.e_name.clone());
        v["letter"] = json!(r.letter.clone());
        v["display"] = json!(r.display.to_string());
        v["code"] = json!(r.code.clone());
        v["level"] = json!(level);
        v["hasChildren"] = json!(level < 3);
    }
    if kind == "job" {
        v["e_name"] = json!(r.e_name.clone());
        v["rec"] = json!(r.rec);
        v["content"] = json!(r.content.clone());
    }
    if kind == "introduce" || kind == "introduce_class" {
        v["content"] = json!(r.content.clone());
    }
    v
}

async fn cat_class_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let rows = cat_repo::list_php(state.db.reader(), &kind, None).await?;
    let level = 1;
    Ok(Value::Array(
        rows.iter()
            .map(|r| cat_row_json(&kind, r, level))
            .collect(),
    ))
}

async fn cat_class_children(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let keyid = json_u64(body, "keyid");
    if keyid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let level = json_i32(body, "level").max(1);
    let rows = cat_repo::list_php(state.db.reader(), &kind, Some(keyid)).await?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| cat_row_json(&kind, r, level))
        .collect();
    Ok(json!({ "list": list }))
}

async fn cat_class_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let kind = cat_kind(body);
    let id = json_u64(body, "id");
    let name_owned = json_str(body, "name");
    let content_owned = json_str(body, "content");
    if id > 0 {
        cat_repo::patch_php(
            state.db.pool(),
            &kind,
            id,
            if name_owned.is_empty() {
                None
            } else {
                Some(name_owned.as_str())
            },
            if has_flag(body, "sort") {
                Some(json_i32(body, "sort"))
            } else {
                None
            },
            None,
            if content_owned.is_empty() {
                None
            } else {
                Some(content_owned.as_str())
            },
        )
        .await?;
        return Ok(PhpOut::Message("ok"));
    }
    let names = dash_names(&name_owned);
    if names.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let vars = dash_names(&json_str(body, "str"));
    let parent = json_u64(body, "nid");
    let ctype = json_str(body, "ctype");
    let parent_id = if ctype == "2" { parent } else { 0 };
    for (i, name) in names.iter().enumerate() {
        let variable = vars.get(i).map(|s| s.as_str()).unwrap_or("");
        let nid = cat_repo::insert_php(
            state.db.pool(),
            &kind,
            parent_id,
            name,
            json_i32(body, "sort"),
            variable,
        )
        .await?;
        if !content_owned.is_empty() && nid > 0 {
            cat_repo::patch_php(
                state.db.pool(),
                &kind,
                nid,
                None,
                None,
                None,
                Some(content_owned.as_str()),
            )
            .await?;
        }
    }
    Ok(PhpOut::Message("ok"))
}

async fn cat_class_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let kind = cat_kind(body);
    let mut ids = ids_named(body, "delid");
    ids.extend(ids_of(body));
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    cat_repo::delete_php_ids(state.db.pool(), &kind, &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn cat_class_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let kind = cat_kind(body);
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let name_owned = json_str(body, "name");
    let e_name_owned = json_str(body, "e_name");
    cat_repo::patch_php(
        state.db.pool(),
        &kind,
        id,
        if name_owned.is_empty() {
            None
        } else {
            Some(name_owned.as_str())
        },
        if has_flag(body, "sort") {
            Some(json_i32(body, "sort"))
        } else {
            None
        },
        if e_name_owned.is_empty() {
            None
        } else {
            Some(e_name_owned.as_str())
        },
        None,
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn cat_class_up(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let id = json_u64(body, "id");
    let roots = cat_repo::list_php(state.db.reader(), &kind, None).await?;
    let position: Vec<Value> = roots.iter().map(|r| cat_row_json(&kind, r, 1)).collect();
    let mut class1 = Value::Null;
    let mut class2 = Value::Array(vec![]);
    if id > 0 {
        if let Some(one) = cat_repo::get_php(state.db.reader(), &kind, id).await? {
            class1 = cat_row_json(&kind, &one, 1);
            let kids = cat_repo::list_php(state.db.reader(), &kind, Some(id)).await?;
            class2 = Value::Array(kids.iter().map(|r| cat_row_json(&kind, r, 2)).collect());
        }
    }
    Ok(json!({ "class1": class1, "class2": class2, "position": position }))
}

async fn cat_class_add_single(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("admin_system_00089"));
    }
    cat_repo::insert_city(
        state.db.pool(),
        json_u64(body, "keyid"),
        &name,
        &json_str(body, "letter"),
        json_i32(body, "display"),
        json_i32(body, "sort"),
        &json_str(body, "e_name"),
        &json_str(body, "code"),
    )
    .await?;
    Ok(PhpOut::Message("admin_01367"))
}

async fn cat_class_up_single(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let name = json_str(body, "name");
    if id == 0 || name.is_empty() {
        return Err(ApiError::param_invalid("admin_system_00089"));
    }
    cat_repo::update_city(
        state.db.pool(),
        id,
        &name,
        &json_str(body, "letter"),
        json_i32(body, "display"),
        json_i32(body, "sort"),
        &json_str(body, "e_name"),
        &json_str(body, "code"),
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

async fn cat_class_upp(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id_arr = json_str(body, "id_arr");
    if id_arr.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    for part in id_arr.split(',') {
        let id: u64 = part.trim().parse().unwrap_or(0);
        if id == 0 {
            continue;
        }
        let name = json_str(body, &format!("cityname_{id}"));
        if name.is_empty() {
            continue;
        }
        cat_repo::update_city(
            state.db.pool(),
            id,
            &name,
            &json_str(body, &format!("letter_{id}")),
            json_i32(body, &format!("display_{id}")),
            json_i32(body, &format!("citysort_{id}")),
            &json_str(body, &format!("citye_name_{id}")),
            &json_str(body, &format!("citycode_{id}")),
        )
        .await?;
    }
    Ok(PhpOut::Message("admin_system_00002"))
}

async fn cat_class_clearpinyin(state: &AppState) -> AppResult<PhpOut> {
    cat_repo::city_clear_pinyin(state.db.pool()).await?;
    Ok(PhpOut::Message("admin_01369"))
}

async fn cat_class_chachong(state: &AppState, body: &Value) -> AppResult<Value> {
    let page = json_u64(body, "page").max(0);
    let limit = 50u64;
    let offset = page.saturating_mul(limit);
    let list = cat_repo::city_dup_pinyin(state.db.reader(), offset, limit).await?;
    Ok(json!({ "list": list, "page": page }))
}

async fn cat_class_one(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({}));
    }
    Ok(cat_repo::get_php(state.db.reader(), &kind, id)
        .await?
        .map(|r| cat_row_json(&kind, &r, 1))
        .unwrap_or(json!({})))
}

async fn user_gap_company_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let all = company_repo::count_admin(db, None, None).await?;
    let s0 = company_repo::count_admin(db, Some(0), None).await?;
    let s3 = company_repo::count_admin(db, Some(3), None).await?;
    let s2 = company_repo::count_admin(db, Some(2), None).await?;
    Ok(json!({
        "companyAllNum": all,
        "companyStatusNum1": s0,
        "companyStatusNum2": s3,
        "companyStatusNum3": s2,
    }))
}

async fn user_gap_reset_password(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let salt: String = Uuid::now_v7().simple().to_string().chars().take(16).collect();
    let password_hash = argon2_hash_async(format!("123456{salt}")).await?;
    user_repo::update_password_with_salt(state.db.pool(), uid, &password_hash, &salt).await?;
    Ok(PhpOut::Message("admin_model_00119"))
}

async fn user_gap_matching(state: &AppState, body: &Value) -> AppResult<Value> {
    let job_id = json_u64(body, "id");
    let comid = if job_id > 0 {
        job_repo::find_by_id(state.db.reader(), job_id)
            .await?
            .map(|j| j.uid)
            .unwrap_or(0)
    } else {
        0
    };
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let list = expect_repo::list_match_admin(
        state.db.reader(),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        offset,
        limit,
    )
    .await?;
    let total = expect_repo::count_match_admin(
        state.db.reader(),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
    )
    .await?;
    let rows: Vec<Value> = list
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "name": r.name,
                "uname": r.uname,
                "username": r.username,
                "moblie": r.moblie,
                "defaults": r.defaults,
                "integrity": r.integrity,
                "status": r.status,
                "edu": r.edu,
                "exp": r.exp,
                "lastupdate": r.lastupdate,
                "salary": if r.maxsalary > 0 {
                    format!("{}-{}", r.minsalary, r.maxsalary)
                } else {
                    r.minsalary.to_string()
                },
                "edu_n": "",
                "exp_n": "",
                "city_n": "",
                "report_n": "",
                "type_n": "",
                "citynum": 0,
                "cityall": "",
            })
        })
        .collect();
    Ok(json!({
        "list": rows,
        "total": total,
        "perPage": per,
        "pageSizes": [10, 20, 50, 100],
        "comid": comid,
        "page": page,
    }))
}

async fn user_gap_resume_audit(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    let eid = json_u64(body, "id");
    if eid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let row = expect_repo::find_admin_by_id(state.db.reader(), eid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("wap_com_00228"))?;
    let mut data = admin_longtail_service::resume_php_edit(state, user, row.uid, eid).await?;
    if let Some(obj) = data.as_object_mut() {
        let info = obj
            .get("expectData")
            .and_then(|e| e.get("expect"))
            .cloned()
            .unwrap_or_else(|| json!({ "id": eid, "uid": row.uid }));
        obj.insert("info".into(), info);
        obj.entry("snum".to_string()).or_insert(json!(0));
    }
    Ok(data)
}

fn wx_nav_apply_type(btn: &mut Value, nav: &phpyun_models::wx_nav::entity::WxNav) {
    match nav.nav_type.as_str() {
        "view" => {
            btn["type"] = json!("view");
            btn["url"] = json!(nav.url);
        }
        "click" => {
            btn["type"] = json!("click");
            btn["key"] = json!(nav.key);
        }
        "miniprogram" => {
            btn["type"] = json!("miniprogram");
            btn["url"] = json!(nav.url);
            btn["appid"] = json!(nav.appid);
            btn["pagepath"] = json!(nav.apppage);
        }
        _ => {}
    }
}

async fn wx_nav_creatnav(state: &AppState) -> AppResult<Value> {
    let navs = wx_nav_repo::list_all(state.db.reader()).await?;
    let mut buttons = Vec::new();
    for root in navs.iter().filter(|n| n.keyid == 0) {
        let kids: Vec<_> = navs.iter().filter(|n| n.keyid == root.id as i32).collect();
        let mut btn = json!({ "name": root.name });
        if kids.is_empty() {
            wx_nav_apply_type(&mut btn, root);
        } else {
            let subs: Vec<Value> = kids
                .iter()
                .map(|k| {
                    let mut s = json!({ "name": k.name });
                    wx_nav_apply_type(&mut s, k);
                    s
                })
                .collect();
            btn["sub_button"] = json!(subs);
        }
        buttons.push(btn);
    }
    if buttons.is_empty() {
        return Ok(json!({ "error": 1, "msg": "admin_tool_00053" }));
    }
    let menu = json!({ "button": buttons });
    match wechat_api_service::replace_menu(state, &menu).await {
        Ok(()) => Ok(json!({ "error": 0, "msg": "admin_01473" })),
        Err(_) => Ok(json!({ "error": 1, "msg": "admin_tool_00053" })),
    }
}

async fn email_set_ceshi(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let email = json_str(body, "ceshi_email");
    if email.is_empty() || !email.contains('@') {
        return Err(ApiError::param_invalid("email"));
    }
    let smtp = gap_extra::list_admin_email(state.db.reader()).await?;
    if smtp.is_empty() {
        return Err(ApiError::business("admin_tool_00026"));
    }
    let _ = state
        .events
        .publish_json(
            "email.verify_queued",
            &json!({
                "kind": "admin_smtp_test",
                "email": email,
                "smtp_id": json_u64(body, "id"),
                "subject": "SMTP test",
            }),
        )
        .await;
    Ok(PhpOut::Message("admin_tool_00027"))
}

async fn email_set_gettpl(state: &AppState, body: &Value) -> AppResult<Value> {
    let name = json_str(body, "name");
    let row = if name.is_empty() {
        None
    } else {
        site_page_repo::find_by_code(state.db.reader(), &name).await?
    };
    Ok(json!({
        "info": row.map(|r| json!({
            "name": r.code,
            "title": r.title,
            "content": r.content,
        })).unwrap_or(json!({})),
        "tpl_temp": {},
        "tpl_n": name,
    }))
}

async fn email_set_savetpl(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let content = json_str(body, "content").replace("amp;nbsp;", "nbsp;");
    site_page_repo::upsert_content(
        state.db.pool(),
        &name,
        &json_str(body, "title"),
        &content,
    )
    .await?;
    Ok(PhpOut::Message("ok"))
}

fn kv_obj(pairs: &[(&str, &str)]) -> Value {
    let mut m = serde_json::Map::new();
    for (k, v) in pairs {
        m.insert((*k).into(), json!(*v));
    }
    Value::Object(m)
}

fn search_item(param: &str, name: &str, value: Value) -> Value {
    json!({ "param": param, "name": name, "value": value })
}

fn class_map(dicts: &dict_service::LocalizedDicts, var: &str) -> Value {
    let mut m = serde_json::Map::new();
    for (id, name) in dicts.userclass_by_variable(var) {
        m.insert(id.to_string(), json!(name));
    }
    Value::Object(m)
}

const SOURCE_MAP: &[(&str, &str)] = &[
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

async fn user_gap_resume_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let all = expect_repo::count_admin_all(db).await?;
    let s0 = expect_repo::count_admin_state(db, 0).await?;
    let s3 = expect_repo::count_admin_state(db, 3).await?;
    let lock = expect_repo::count_admin_r_status(db, 2).await?;
    let teen_since = clock::now_ts() - 16 * 365 * 86400;
    let teen = expect_repo::count_admin_teen(db, teen_since).await?;
    Ok(json!({
        "resumeAllNum": all,
        "resumeStatusNum1": s0,
        "resumeStatusNum2": s3,
        "resumeStatusNum3": lock,
        "resumeTeenNum": teen,
    }))
}

async fn user_gap_user_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let all = resume_repo::count_admin(db, None, None).await?;
    let lock = resume_repo::count_admin(db, Some(2), None).await?;
    Ok(json!({
        "userAllNum": all,
        "userStatusNum3": lock,
    }))
}

/// PHP `msgNum::memNumV1` — Vue reads `res.data.memAllNum` / `memStatusNum3` (envelope, not raw).
async fn user_gap_mem_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let all = user_repo::count_admin_pid0(db).await?;
    let lock = user_repo::count_admin_status(db, 2).await?;
    Ok(json!({
        "memAllNum": all,
        "memStatusNum3": lock,
    }))
}

async fn user_gap_mem_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let utype = json_opt_i32(body, "utype");
    let usertype = match utype {
        Some(5) => Some(0),
        other => other.filter(|v| *v != 0),
    };
    let (t0, t1) = json_day_range(body, "times");
    let time_type = json_str(body, "time_type");
    let time_col = match time_type.as_str() {
        "adtime" if t0.is_some() && t1.is_some() => Some("reg_date"),
        "lotime" if t0.is_some() && t1.is_some() => Some("login_date"),
        _ => None,
    };
    let kw = json_str(body, "keyword");
    let f = user_repo::PhpMemberListFilter {
        usertype,
        status: json_opt_i32(body, "status"),
        source: json_opt_i32(body, "source"),
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        kw_type: json_i32(body, "type"),
        time_col,
        time_from: t0,
        time_to: t1,
    };
    let db = state.db.reader();
    let rows = user_repo::list_php_members(db, &f, offset, limit).await?;
    let total = user_repo::count_php_members(db, &f).await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "uid": r.uid,
                "username": r.username,
                "email": r.email,
                "moblie": r.moblie,
                "moblie_status": r.moblie_status,
                "reg_ip": r.reg_ip,
                "reg_date": r.reg_date,
                "reg_date_n": fmt_ts(r.reg_date, "%Y-%m-%d %H:%M:%S"),
                "login_ip": r.login_ip,
                "login_date": r.login_date,
                "login_date_n": fmt_ts(r.login_date, "%Y-%m-%d %H:%M:%S"),
                "usertype": r.usertype,
                "status": r.status,
                "lock_info": r.lock_info,
                "source": r.source,
                "did": r.did,
                "login_address": r.login_address,
                "moblie_address": r.moblie_address,
                "countname": "",
            })
        })
        .collect();
    Ok(php_data_table(data, total))
}

async fn user_gap_logout_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let db = state.db.reader();
    let rows = logout_repo::list_admin(
        db,
        json_opt_i32(body, "status"),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
        offset,
        limit,
    )
    .await?;
    let total = logout_repo::count_admin(
        db,
        json_opt_i32(body, "status"),
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_i32(body, "type"),
    )
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let usertype_name = match r.usertype {
                1 => "admin_user_00304",
                2 => "wap_user_00153",
                _ => "common_02004",
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "username": r.username,
                "tel": r.tel,
                "status": r.status,
                "ctime": r.ctime,
                "ctime_ymd": fmt_ts(r.ctime, "%Y-%m-%d %H:%M:%S"),
                "usertype": r.usertype,
                "usertype_name": usertype_name,
            })
        })
        .collect();
    Ok(php_data_table(data, total))
}

async fn user_gap_appeal_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let db = state.db.reader();
    let rows = user_repo::list_php_appeals(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_opt_i32(body, "appealstate"),
        offset,
        limit,
    )
    .await?;
    let total = user_repo::count_php_appeals(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        json_opt_i32(body, "appealstate"),
    )
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "uid": r.uid,
                "username": r.username,
                "appeal": r.appeal,
                "appealtime": r.appealtime,
                "appealtime_ymd": fmt_ts(r.appealtime, "%Y-%m-%d %H:%M:%S"),
                "appealstate": r.appealstate,
                "moblie": r.moblie,
                "email": r.email,
            })
        })
        .collect();
    let mut out = php_data_table(data, total);
    out["promiss"] = json!({ "email": 1, "moblie": 1 });
    Ok(out)
}

fn start_of_utc_day(ts: i64) -> i64 {
    ts - ts.rem_euclid(86400)
}

fn user_gap_log_usertype(body: &Value) -> i32 {
    json_opt_i32(body, "utype").filter(|v| *v > 0).unwrap_or(1)
}

fn user_gap_log_del_usertype(body: &Value) -> Option<i32> {
    match json_str(body, "del").as_str() {
        "alluser" => Some(1),
        "allcom" => Some(2),
        "alltrain" => Some(4),
        _ => None,
    }
}

/// PHP `admin_loginlog::index_action` — `{data,total,pageSizes}`.
async fn user_gap_login_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let content = json_str(body, "content");
    let kw_type = json_i32(body, "type");
    let (t0, t1) = json_day_range(body, "times");
    let mut username_like: Option<String> = None;
    let mut content_like: Option<String> = None;
    let mut uid = json_u64(body, "uid");
    if !kw.is_empty() {
        match kw_type {
            1 => username_like = Some(kw.clone()),
            2 => content_like = Some(kw.clone()),
            3 => uid = kw.parse().unwrap_or(0),
            _ => {}
        }
    }
    if !content.is_empty() {
        content_like = Some(content.clone());
    }
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = admin_msg_repo::PhpLoginLogFilter {
        usertype: user_gap_log_usertype(body),
        uid: if uid > 0 { Some(uid) } else { None },
        username_like: username_like.as_deref(),
        content_like: content_like.as_deref(),
        time_from: t0,
        time_to: t1,
        order_t: order_t.as_str(),
        order_dir: order_dir.as_str(),
    };
    let db = state.db.reader();
    let rows = admin_msg_repo::list_php_login_logs(db, &f, offset, limit).await?;
    let total = admin_msg_repo::count_php_login_logs(db, &f).await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "usertype": r.usertype,
                "content": r.content,
                "ip": r.ip,
                "ctime": r.ctime,
                "ctime_ymd": fmt_ts(r.ctime, "%Y-%m-%d %H:%M:%S"),
                "remoteport": r.remoteport,
                "username": r.username,
                "rname": r.rname,
                "eid": r.eid,
                "comname": r.comname,
                "pid": r.pid,
            })
        })
        .collect();
    Ok(php_data_table(data, total))
}

/// PHP `admin_loginlog::dellog_action`.
async fn user_gap_login_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let db = state.db.pool();
    if let Some(ut) = user_gap_log_del_usertype(body) {
        admin_msg_repo::delete_php_login_logs_by_usertype(db, ut).await?;
        let msg = match ut {
            2 => "admin_01296",
            4 => "admin_01298",
            _ => "admin_01297",
        };
        return Ok(PhpOut::Message(msg));
    }
    let ids = ids_of(body);
    if ids.is_empty() {
        return Ok(PhpOut::Message("model_00034"));
    }
    admin_msg_repo::delete_php_login_logs(db, &ids).await?;
    Ok(PhpOut::Message("admin_model_00163"))
}

/// PHP `admin_memberlog::index_action` — `{data,total,pageSizes}`.
async fn user_gap_memlog_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let content = json_str(body, "content");
    let kw_type = json_i32(body, "type");
    let (t0, t1) = json_day_range(body, "time");
    let mut username_like: Option<String> = None;
    let mut uid = json_u64(body, "uid");
    if !kw.is_empty() {
        match kw_type {
            1 => username_like = Some(kw.clone()),
            3 => uid = kw.parse().unwrap_or(0),
            _ => {}
        }
    }
    let now = clock::now_ts();
    let mut time_from = t0;
    if let Some(end) = json_opt_i32(body, "end") {
        let from_end = if end == 1 {
            start_of_utc_day(now)
        } else if end > 0 {
            now.saturating_sub(i64::from(end) * 86400)
        } else {
            0
        };
        if from_end > 0 {
            time_from = Some(time_from.map_or(from_end, |t| t.max(from_end)));
        }
    }
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gap_repo::PhpMemberLogFilter {
        usertype: user_gap_log_usertype(body),
        uid: if uid > 0 { Some(uid) } else { None },
        username_like: username_like.as_deref(),
        content_like: if content.is_empty() {
            None
        } else {
            Some(content.as_str())
        },
        opera: json_opt_i32(body, "operas"),
        log_type: json_opt_i32(body, "parrs"),
        time_from,
        time_to: t1,
        order_t: order_t.as_str(),
        order_dir: order_dir.as_str(),
    };
    let db = state.db.reader();
    let rows = gap_repo::list_php_member_logs(db, &f, offset, limit).await?;
    let total = gap_repo::count_php_member_logs(db, &f).await?;
    let base = preview_base(state);
    let data: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let com_url = if r.comname.is_empty() {
                String::new()
            } else {
                format!(
                    "{}/index.php?m=company&c=show&id={}&look=admin",
                    base.trim_end_matches('/'),
                    r.uid
                )
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "opera": r.opera,
                "type": r.r#type,
                "usertype": r.usertype,
                "content": r.content,
                "ip": r.ip,
                "ctime": r.ctime,
                "ctime_ymd": fmt_ts(r.ctime, "%Y-%m-%d %H:%M:%S"),
                "remoteport": r.remoteport,
                "username": r.username,
                "rname": r.rname,
                "eid": r.eid,
                "comname": r.comname,
                "pid": r.pid,
                "sub_n": r.sub_n,
                "com_url": com_url,
                "comp_url": "",
            })
        })
        .collect();
    Ok(php_data_table(data, total))
}

/// PHP `admin_memberlog::delLog_action`.
async fn user_gap_memlog_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let db = state.db.pool();
    if let Some(ut) = user_gap_log_del_usertype(body) {
        gap_repo::delete_php_member_logs_by_usertype(db, ut).await?;
        let msg = match ut {
            2 => "admin_01296",
            4 => "admin_01298",
            _ => "admin_01297",
        };
        return Ok(PhpOut::Message(msg));
    }
    let ids = ids_of(body);
    if ids.is_empty() {
        return Ok(PhpOut::Message("common_00740"));
    }
    gap_repo::delete_php_member_logs(db, &ids).await?;
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `admin_member::Imitate_action` — `{url: sy_weburl/member}` (no PHP cookie).
async fn user_gap_mem_imitate(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("wap_com_00228"))?;
    let web = cfg_of(state, "sy_weburl").await;
    let url = format!("{}/member", web.trim_end_matches('/'));
    Ok(json!({ "url": url }))
}

/// PHP `admin_member::lock_action` / `userinfo::lock`.
async fn user_gap_mem_lock(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    let lock_info = json_str(body, "lock_info");
    if status == 2 && lock_info.is_empty() {
        return Err(ApiError::business("common_06622"));
    }
    if status == 1 {
        if logout_repo::find_pending_by_uid(state.db.reader(), uid)
            .await?
            .is_some()
        {
            return Err(ApiError::business("common_01459"));
        }
    }
    let db = state.db.pool();
    let n = user_repo::update_lock(db, uid, status, &lock_info).await?;
    if n == 0 && user_repo::find_by_uid(db, uid).await?.is_none() {
        return Err(ApiError::business("common_01071"));
    }
    user_repo::lock_related_r_status(db, uid, status).await?;
    Ok(PhpOut::Message("common_01944"))
}

/// PHP `admin_member::editSave_action` / `userinfo::upMemberInfo`.
async fn user_gap_mem_edit(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let reader = state.db.reader();
    let mem = user_repo::find_by_uid(reader, uid)
        .await?
        .ok_or_else(|| ApiError::business("wap_com_00228"))?;
    let username = {
        let s = json_str(body, "username");
        if s.is_empty() {
            mem.username.clone()
        } else {
            s
        }
    };
    let mobile = json_str(body, "moblie");
    let email = json_str(body, "email");
    if user_repo::exists_username_except(reader, &username, Some(uid)).await? {
        return Err(ApiError::business("common_01388"));
    }
    if !mobile.is_empty() && user_repo::exists_mobile_except(reader, &mobile, Some(uid)).await? {
        return Err(ApiError::business("api_wxapp_00008"));
    }
    if !email.is_empty() && user_repo::exists_email_except(reader, &email, Some(uid)).await? {
        return Err(ApiError::business("default_00012"));
    }
    let password = json_str(body, "password");
    let hashed = if password.is_empty() {
        None
    } else {
        let salt: String = Uuid::now_v7().simple().to_string().chars().take(16).collect();
        let hash = argon2_hash_async(format!("{password}{salt}")).await?;
        Some((hash, salt))
    };
    let pw = hashed
        .as_ref()
        .map(|(hash, salt)| (hash.as_str(), salt.as_str()));
    let db = state.db.pool();
    let n = user_repo::update_php_admin_member(
        db,
        uid,
        &user_repo::PhpMemberEdit {
            username: &username,
            mobile: &mobile,
            email: &email,
            reg_ip: &json_str(body, "reg_ip"),
            did: json_u64(body, "did"),
            status: json_i32(body, "status"),
            password: pw,
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("member_user_00603"));
    }
    user_repo::sync_php_profile_contact(db, uid, &mobile, &email).await?;
    Ok(PhpOut::Message("member_user_00602"))
}

/// PHP `admin_member::del_action` / `userinfo::delMember`.
async fn user_gap_mem_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = user_repo::delete_php_members(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("common_06641"));
    }
    Ok(PhpOut::Message("common_06640"))
}

async fn php_userinfo_row(state: &AppState, uid: u64, usertype: i32) -> AppResult<Value> {
    let db = state.db.reader();
    if usertype == 2 {
        if let Some(c) = company_repo::find_by_uid(db, uid).await? {
            return Ok(json!({
                "uid": c.uid,
                "name": c.name.unwrap_or_default(),
                "moblie_status": c.moblie_status,
                "email_status": c.email_status,
                "yyzz_status": c.yyzz_status,
            }));
        }
    } else if let Some(r) = resume_repo::find_by_uid(db, uid).await? {
        return Ok(json!({
            "uid": r.uid,
            "name": r.name.unwrap_or_default(),
            "moblie_status": r.moblie_status,
            "email_status": r.email_status,
            "idcard_status": r.idcard_status,
        }));
    }
    Ok(json!({}))
}

/// PHP `admin_appeal::info_action` — `{user, info}`.
async fn user_gap_appeal_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "id");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let row = user_repo::find_php_member_detail(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("wap_com_00228"))?;
    let user = php_userinfo_row(state, row.uid, row.usertype).await?;
    let mut info = json!({
        "uid": row.uid,
        "username": row.username,
        "email": row.email,
        "moblie": row.moblie,
        "usertype": row.usertype,
        "status": row.status,
        "did": row.did,
        "reg_date": row.reg_date,
        "login_date": row.login_date,
        "login_hits": row.login_hits,
        "lock_info": row.lock_info,
        "appeal": row.appeal,
        "appealtime": row.appealtime,
        "appealstate": row.appealstate,
        "login_ip": row.login_ip,
        "reg_ip": row.reg_ip,
        "address": row.address,
        "login_date_ymd": fmt_date(row.login_date),
        "reg_date_ymd": fmt_date(row.reg_date),
    });
    // shensu.vue assigns `user = res.data.info`; copy profile fields onto info too.
    if let Some(obj) = info.as_object_mut() {
        if let Some(v) = user.get("name") {
            obj.insert("name".into(), v.clone());
        }
        for k in ["moblie_status", "email_status", "idcard_status", "yyzz_status"] {
            if let Some(v) = user.get(k) {
                obj.insert(k.into(), v.clone());
            }
        }
    }
    Ok(json!({ "user": user, "info": info }))
}

/// PHP `admin_appeal::success_action`.
async fn user_gap_appeal_success(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "id");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = user_repo::update_appeal_state(state.db.pool(), uid, 2).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00002"));
    }
    Ok(PhpOut::Message("admin_user_00001"))
}

/// PHP `admin_appeal::del_action` — clear appeal fields, do not delete member.
async fn user_gap_appeal_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "del");
    if ids.is_empty() {
        let id = json_u64(body, "id");
        if id > 0 {
            ids.push(id);
        }
    }
    if ids.is_empty() {
        return Err(ApiError::param_invalid("common_01066"));
    }
    let n = user_repo::clear_appeals(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `admin_member_logout::status_action` / `logout::status` (skip mail/SMS).
async fn user_gap_logout_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("common_01070"));
    }
    let row = logout_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("common_01383"))?;
    let n = logout_repo::approve(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("common_06534"));
    }
    if let Some(member) = user_repo::find_by_uid(state.db.reader(), row.uid).await? {
        let uname: String = Uuid::now_v7().simple().to_string().chars().take(16).collect();
        let mob = format!("out_{}", member.moblie.as_deref().unwrap_or(""));
        let mail = format!("out_{}_out", member.email.as_deref().unwrap_or(""));
        let db = state.db.pool();
        user_repo::anonymize_logout_member(db, row.uid, &uname, &mob, &mail).await?;
        user_repo::sync_php_profile_contact(db, row.uid, &mob, &mail).await?;
        user_repo::lock_related_r_status(db, row.uid, 2).await?;
    }
    Ok(PhpOut::Message("model_00208"))
}

/// PHP `admin_member_logout::del_action`.
async fn user_gap_logout_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "del");
    if ids.is_empty() {
        let id = json_u64(body, "id");
        if id > 0 {
            ids.push(id);
        }
    }
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = logout_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `logout::getListNumV1` — `{count, weishenhe}`.
async fn user_gap_logout_num(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let count = logout_repo::count_admin(db, None, None, 0).await?;
    let weishenhe = logout_repo::count_admin(db, Some(1), None, 0).await?;
    Ok(json!({ "count": count, "weishenhe": weishenhe }))
}

async fn user_gap_resume_config(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let source = kv_obj(SOURCE_MAP);
    let search_list = vec![
        search_item(
            "status",
            "wap_com_00406",
            kv_obj(&[
                ("1", "wap_user_00165"),
                ("2", "admin_user_00138"),
                ("3", "wap_user_00167"),
                ("4", "wap_user_00166"),
            ]),
        ),
        search_item("source", "admin_yunying_00139", source.clone()),
        search_item(
            "service",
            "member_com_00107",
            kv_obj(&[("1", "wap_user_00335"), ("2", "wap_01465")]),
        ),
        search_item("type", "wap_user_00012", class_map(&dicts, "user_type")),
        search_item(
            "salary",
            "member_user_00106",
            kv_obj(&[
                ("2000_4000", "2000-4000"),
                ("4000_6000", "4000-6000"),
                ("6000_8000", "6000-8000"),
                ("8000_10000", "8000-10000"),
                ("10000", "common_06590"),
            ]),
        ),
        search_item(
            "age",
            "wap_com_00302",
            kv_obj(&[
                ("16_20", "admin_user_00376"),
                ("21_30", "admin_user_00377"),
                ("31_40", "admin_user_00378"),
                ("41_50", "admin_user_00379"),
                ("50", "admin_01316"),
            ]),
        ),
        search_item(
            "sex",
            "wap_com_00303",
            kv_obj(&[("3", "不限"), ("1", "男"), ("2", "女")]),
        ),
        search_item("marriage", "wap_com_00282", class_map(&dicts, "user_marriage")),
        search_item(
            "remark",
            "admin_01317",
            kv_obj(&[("1", "是"), ("2", "否")]),
        ),
        search_item("edu", "wap_com_00283", class_map(&dicts, "user_edu")),
        search_item("exp", "wap_user_00240", class_map(&dicts, "user_word")),
        search_item("report", "wap_com_00279", class_map(&dicts, "user_report")),
        search_item(
            "integrity",
            "member_user_00151",
            kv_obj(&[
                ("1", "55%以上"),
                ("2", "65%以上"),
                ("3", "75%以上"),
                ("4", "85%以上"),
                ("55", "等于55%"),
                ("65", "等于65%"),
            ]),
        ),
    ];
    let export_type = kv_obj(&[
        ("rtype_id", "member_com_00012"),
        ("rtype_name", "member_com_00013"),
        ("rtype_uid", "admin_user_00130"),
        ("rtype_uname", "wap_00529"),
        ("rtype_sex", "wap_com_00303"),
        ("rtype_birthday", "member_com_00016"),
        ("type_marriage", "member_user_00162"),
        ("type_height", "member_user_00165"),
        ("type_nationality", "member_user_00164"),
        ("type_weight", "member_user_00160"),
        ("type_idcard", "member_com_00014"),
        ("type_telphone", "member_user_00163"),
        ("type_telhome", "member_com_00015"),
        ("type_email", "member_com_00018"),
        ("rtype_edu", "member_com_00011"),
        ("type_homepage", "member_com_00008"),
        ("type_address", "wap_01362"),
        ("rtype_exp", "wap_user_00240"),
        ("type_domicile", "common_01989"),
        ("type_living", "wap_user_00242"),
        ("type_description", "member_com_00009"),
        ("rtype_hy", "member_com_00010"),
        ("rtype_job_classid", "wap_com_00353"),
        ("rtype_city_classid", "wap_js_00083"),
        ("rtype_minsalary,maxsalary", "member_com_00017"),
        ("rtype_type", "wap_user_00012"),
        ("rtype_report", "wap_com_00279"),
        ("rtype_lastdate", "wap_00326"),
    ]);
    Ok(json!({
        "source": source,
        "search_list": search_list,
        "exportType": export_type,
    }))
}

async fn user_gap_user_config(state: &AppState) -> AppResult<Value> {
    let source = kv_obj(SOURCE_MAP);
    let search_list = vec![
        search_item("source", "admin_yunying_00139", source.clone()),
        search_item(
            "status",
            "member_user_00181",
            kv_obj(&[("1", "admin_user_00149"), ("2", "admin_user_00150")]),
        ),
        search_item(
            "def_job",
            "admin_user_company_00294",
            kv_obj(&[("1", "是"), ("2", "否")]),
        ),
    ];
    let domains = domain_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "search_list": search_list,
        "source": source,
        "domainList": domain_object(&domains),
    }))
}

fn keyword_type_map() -> Value {
    kv_obj(&[
        ("1", "wap_js_00130"),
        ("2", "wap_user_00220"),
        ("3", "wap_user_00154"),
        ("4", "default_00262"),
        ("5", "wap_com_00428"),
        ("8", "admin_01381"),
        ("9", "admin_01382"),
        ("10", "wap_user_00084"),
        ("11", "admin_user_00018"),
        ("12", "wap_user_00223"),
        ("13", "wap_js_00066"),
    ])
}

const WEB_CONFIG_KEYS: &[&str] = &[
    "sy_seo_rewrite",
    "sy_header_fix",
    "sy_footer_fix",
    "sy_linksq",
    "sy_wap_jump",
    "sy_pc_jump_wap",
    "sy_h5_share",
    "sy_advice_mobilecode",
    "sy_job_lookfx",
    "sy_wxwap_list",
    "sy_wap_comtpl",
    "sy_uni_comtpl",
    "sy_news_rewrite",
    "sy_ewm_type",
    "sy_default_userclass",
    "sy_default_comclass",
    "resume_salarytype",
    "sy_indexpage",
    "sy_datacycle",
    "sy_datacycle_job",
    "sy_datacycle_com",
    "sy_logintime",
    "sy_login_type",
    "sy_resume_visitors",
    "sy_adclick",
    "sy_recommend_day_num",
    "sy_recommend_interval",
    "sy_resumeout_day_num",
    "sy_resumeout_interval",
    "sy_zhanzhang_baidu",
    "sy_outlinks",
    "sy_shenming",
    "sy_job_hits",
    "sy_web_city_one",
    "sy_web_city_two",
    "sy_sxsjgs",
    "sy_closeOrder",
    "sy_autoref",
    "sy_autorefrand",
];

async fn settings_hash(state: &AppState) -> AppResult<HashMap<String, String>> {
    let rows = setting_repo::list_all(state.db.reader()).await?;
    Ok(rows.into_iter().map(|r| (r.key_name, r.value)).collect())
}

fn city_label_pairs(rows: &[(i32, String)]) -> Vec<Value> {
    rows.iter()
        .map(|(id, name)| json!({ "label": name, "value": *id }))
        .collect()
}

async fn web_config_index(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let mut config = serde_json::Map::new();
    for k in WEB_CONFIG_KEYS {
        config.insert((*k).into(), json!(cfg.get(*k).cloned().unwrap_or_default()));
    }
    let dicts = dict_service::get(state).await?;
    Ok(json!({
        "config": config,
        "province": city_label_pairs(&dicts.city_provinces()),
    }))
}

async fn web_config_city(state: &AppState, body: &Value) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let city_id = json_i32(body, "city_id");
    let rows = if city_id > 0 {
        dicts.city_of_parent(city_id)
    } else {
        dicts.city_provinces()
    };
    Ok(json!({ "city": city_label_pairs(&rows) }))
}

fn checkpic_url(cfg: &HashMap<String, String>, path: &str) -> String {
    let p = path.trim();
    if p.is_empty() {
        return String::new();
    }
    if p.starts_with("http://") || p.starts_with("https://") {
        return p.to_string();
    }
    let base = cfg
        .get("sy_ossurl")
        .filter(|s| !s.is_empty())
        .or_else(|| cfg.get("sy_weburl"))
        .cloned()
        .unwrap_or_default();
    if base.is_empty() {
        return p.to_string();
    }
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        p.trim_start_matches('/')
    )
}

async fn wx_nav_config(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let web = cfg.get("sy_weburl").cloned().unwrap_or_default();
    let welcom_type = cfg
        .get("wx_welcom_type")
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_else(|| "nowxcom".into());
    let htlogin = cfg
        .get("wx_author_htlogin")
        .cloned()
        .unwrap_or_else(|| "1".into());
    Ok(json!({
        "wx_name": cfg.get("wx_name").cloned().unwrap_or_default(),
        "backurl": format!("{}/weixin/index.php", web.trim_end_matches('/')),
        "wx_token": cfg.get("wx_token").cloned().unwrap_or_default(),
        "wx_appid": cfg.get("wx_appid").cloned().unwrap_or_default(),
        "wx_appsecret": cfg.get("wx_appsecret").cloned().unwrap_or_default(),
        "wx_welcom": cfg.get("wx_welcom").cloned().unwrap_or_default(),
        "wx_welcom_type": welcom_type,
        "sy_wxcom_pic": checkpic_url(&cfg, cfg.get("sy_wxcom_pic").map(String::as_str).unwrap_or("")),
        "wx_search": cfg.get("wx_search").cloned().unwrap_or_default(),
        "wx_search_no": cfg.get("wx_search_no").cloned().unwrap_or_default(),
        "sy_wx_qcode": checkpic_url(&cfg, cfg.get("sy_wx_qcode").map(String::as_str).unwrap_or("")),
        "sy_wx_logo": checkpic_url(&cfg, cfg.get("sy_wx_logo").map(String::as_str).unwrap_or("")),
        "sy_wx_sharelogo": checkpic_url(&cfg, cfg.get("sy_wx_sharelogo").map(String::as_str).unwrap_or("")),
        "wx_rz": cfg.get("wx_rz").cloned().unwrap_or_default(),
        "wx_author": cfg.get("wx_author").cloned().unwrap_or_default(),
        "wx_author_htlogin": htlogin,
        "wx_popWin": cfg.get("wx_popWin").cloned().unwrap_or_default(),
    }))
}

async fn wx_zdkeyword_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let kw_opt = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = gap_extra::count_wx_zdkeyword(db, kw_opt).await?;
    let rows = gap_extra::list_wx_zdkeyword(db, kw_opt, offset, limit).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "keyword": r.keyword,
                "content": r.content,
                "time": r.time,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn wx_zdkeyword_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("common_01066"));
    }
    gap_extra::delete_wx_zdcon_by_kids(state.db.pool(), &ids).await?;
    gap_extra::delete_wx_zdkeyword(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn wx_zdkeyword_get(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({ "row": {} }));
    }
    let Some(row) = gap_extra::get_wx_zdkeyword(state.db.reader(), id).await? else {
        return Ok(json!({ "row": {} }));
    };
    let cons = gap_extra::list_wx_zdcon(state.db.reader(), id).await?;
    let conarr: Vec<Value> = cons
        .into_iter()
        .map(|c| {
            json!({
                "id": c.id,
                "kid": c.kid,
                "msgtype": c.msgtype,
                "content": c.content,
                "media_id": c.media_id,
                "sort": c.sort,
                "time": c.time,
                "ctime_n": fmt_dt(c.time),
                "image_n": "",
                "newimage": "",
            })
        })
        .collect();
    Ok(json!({
        "row": {
            "id": row.id,
            "title": row.title,
            "keyword": row.keyword,
            "content": row.content,
            "time": row.time,
            "conarr": conarr,
        }
    }))
}

async fn wx_zdkeyword_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let title = json_str(body, "title");
    if title.is_empty() {
        return Err(ApiError::param_invalid("admin_tool_00050"));
    }
    let keyword = json_str(body, "keyword");
    if keyword.is_empty() {
        return Err(ApiError::param_invalid("admin_tool_00586"));
    }
    let now = clock::now_ts();
    let kid = gap_extra::upsert_wx_zdkeyword(
        state.db.pool(),
        json_u64(body, "id"),
        &title,
        &keyword,
        now,
    )
    .await?;
    let del_ids = ids_named(body, "del_idarr");
    if !del_ids.is_empty() {
        gap_extra::delete_wx_zdcon_ids(state.db.pool(), &del_ids, kid).await?;
    }
    let items: Vec<Value> = match body.get("content") {
        Some(Value::Array(a)) => a.clone(),
        Some(Value::String(s)) => serde_json::from_str(s).unwrap_or_default(),
        _ => Vec::new(),
    };
    for item in items {
        let msgtype = json_str(&item, "msgtype");
        let content = match item.get("content") {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => String::new(),
        };
        let media_id = json_i32(&item, "media_id");
        let sort = json_i32(&item, "sort");
        if json_i32(&item, "isadd") == 1 {
            gap_extra::insert_wx_zdcon(
                state.db.pool(),
                kid,
                &msgtype,
                &content,
                media_id,
                sort,
                now,
            )
            .await?;
        } else {
            let cid = json_u64(&item, "id");
            if cid > 0 {
                gap_extra::update_wx_zdcon(
                    state.db.pool(),
                    cid,
                    &msgtype,
                    &content,
                    media_id,
                    sort,
                    now,
                )
                .await?;
            }
        }
    }
    Ok(PhpOut::Message("wap_user_00104"))
}

async fn job_class_roots(state: &AppState) -> AppResult<Value> {
    let rows = cat_repo::list_php(state.db.reader(), "job", None).await?;
    Ok(Value::Array(
        rows.iter().map(|r| cat_row_json("job", r, 1)).collect(),
    ))
}

async fn job_class_up(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let position = job_class_roots(state).await?;
    let mut onejob = json!({});
    let mut twojob = Value::Array(vec![]);
    let mut threejob = serde_json::Map::new();
    if id > 0 {
        if let Some(one) = cat_repo::get_php(state.db.reader(), "job", id).await? {
            onejob = cat_row_json("job", &one, 1);
            let twos = cat_repo::list_php(state.db.reader(), "job", Some(id)).await?;
            let mut two_arr = Vec::new();
            for two in &twos {
                two_arr.push(cat_row_json("job", two, 2));
                let threes = cat_repo::list_php(state.db.reader(), "job", Some(two.id)).await?;
                threejob.insert(
                    two.id.to_string(),
                    Value::Array(threes.iter().map(|t| cat_row_json("job", t, 3)).collect()),
                );
            }
            twojob = Value::Array(two_arr);
        }
    }
    Ok(json!({
        "id": id,
        "onejob": onejob,
        "twojob": twojob,
        "threejob": threejob,
        "position": position,
    }))
}

async fn job_class_classadd(state: &AppState, body: &Value) -> AppResult<Value> {
    let position = job_class_roots(state).await?;
    let id = json_u64(body, "id");
    let tid = json_u64(body, "tid");
    if id > 0 {
        let info = cat_repo::get_php(state.db.reader(), "job", id)
            .await?
            .map(|r| cat_row_json("job", &r, 1))
            .unwrap_or(json!({}));
        let job_id = info.get("keyid").and_then(|v| v.as_u64()).unwrap_or(0);
        let job = if job_id > 0 {
            cat_repo::get_php(state.db.reader(), "job", job_id)
                .await?
                .map(|r| cat_row_json("job", &r, 1))
                .unwrap_or(json!({}))
        } else {
            json!({})
        };
        let class2_parent = job.get("keyid").and_then(|v| v.as_u64()).unwrap_or(0);
        let class2 = if class2_parent > 0 {
            let rows = cat_repo::list_php(state.db.reader(), "job", Some(class2_parent)).await?;
            Value::Array(rows.iter().map(|r| cat_row_json("job", r, 2)).collect())
        } else {
            Value::Array(vec![])
        };
        return Ok(json!({
            "type": "three",
            "info": info,
            "class2": class2,
            "job": job,
            "position": position,
        }));
    }
    if tid > 0 {
        let info = cat_repo::get_php(state.db.reader(), "job", tid)
            .await?
            .map(|r| cat_row_json("job", &r, 1))
            .unwrap_or(json!({}));
        return Ok(json!({
            "type": "two",
            "info": info,
            "position": position,
        }));
    }
    Ok(json!({ "position": position }))
}

async fn job_class_chachong(state: &AppState, body: &Value) -> AppResult<Value> {
    let page = json_u64(body, "page").max(0);
    let limit = 50u64;
    let offset = page.saturating_mul(limit);
    let list = cat_repo::job_dup_pinyin(state.db.reader(), offset, limit).await?;
    Ok(json!({ "list": list, "page": page }))
}

async fn job_class_move(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let pid = json_u64(body, "pid");
    if pid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let keyid = json_u64(body, "keyid");
    let nid = json_u64(body, "nid");
    let parent = if keyid > 0 { keyid } else { nid };
    cat_repo::patch_job_class_parent(state.db.pool(), pid, parent).await?;
    Ok(PhpOut::Message("ok"))
}

fn parse_intertime(body: &Value) -> i64 {
    match body.get("intertime") {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(Value::String(s)) => {
            let s = s.trim();
            if s.is_empty() {
                return 0;
            }
            if let Ok(n) = s.parse::<i64>() {
                return n;
            }
            for fmt in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"] {
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
                    return dt.and_utc().timestamp();
                }
            }
            if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                return d
                    .and_hms_opt(0, 0, 0)
                    .map(|t| t.and_utc().timestamp())
                    .unwrap_or(0);
            }
            0
        }
        _ => 0,
    }
}

/// PHP `company_interview::index_action` — 面试模板 `yqmb`，不是 userid_msg。
async fn interview_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = yqmb_repo::AdminYqmbFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
        status: json_opt_i32(body, "status"),
    };
    let db = state.db.reader();
    let total = yqmb_repo::admin_php_count(db, &f).await?;
    let rows = if total > 0 {
        yqmb_repo::admin_php_list(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "name": r.name,
                "linkman": r.linkman,
                "linktel": r.linktel,
                "address": r.address,
                "intertime": fmt_dt(r.intertime),
                "content": r.content,
                "addtime": r.addtime,
                "addtime_n": fmt_dt(r.addtime),
                "status": r.status,
                "statusbody": r.statusbody,
                "comname": r.comname,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn interview_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("common_06682"));
    }
    let name = json_str(body, "name");
    let linkman = json_str(body, "linkman");
    let linktel = json_str(body, "linktel");
    let address = json_str(body, "address");
    let content = json_str(body, "content");
    let intertime = parse_intertime(body);
    if linkman.is_empty() {
        return Err(ApiError::param_invalid("member_com_00677"));
    }
    if intertime <= 0 {
        return Err(ApiError::param_invalid("member_com_00681"));
    }
    if linktel.is_empty() {
        return Err(ApiError::param_invalid("common_06291"));
    }
    if address.is_empty() {
        return Err(ApiError::param_invalid("member_com_00680"));
    }
    let com = company_repo::find_by_uid(state.db.reader(), uid).await?;
    if com.is_none() {
        return Err(ApiError::param_invalid("common_06682"));
    }
    let tpl_name = if name.is_empty() {
        format!("{linkman}")
    } else {
        name
    };
    let id = json_u64(body, "id");
    let now = clock::now_ts();
    if id > 0 {
        yqmb_repo::admin_update(
            state.db.pool(),
            id,
            uid,
            &tpl_name,
            &linkman,
            &linktel,
            &content,
            &address,
            intertime,
            0,
        )
        .await?;
    } else {
        yqmb_repo::admin_insert(
            state.db.pool(),
            uid,
            &tpl_name,
            &linkman,
            &linktel,
            &content,
            &address,
            intertime,
            0,
            now,
        )
        .await?;
    }
    Ok(PhpOut::Message("ok"))
}

async fn interview_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    let status = json_i32(body, "status");
    if ids.is_empty() || status == 0 {
        return Err(ApiError::param_invalid("common_01716"));
    }
    let body_txt = json_str(body, "statusbody");
    yqmb_repo::admin_set_status(state.db.pool(), &ids, status, &body_txt).await?;
    Ok(PhpOut::Message("ok"))
}

async fn interview_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("common_01066"));
    }
    yqmb_repo::admin_delete_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

/// PHP `company_comlog::index_action` 职位申请记录。
async fn comlog_userid_job(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let (from, to) = json_day_range(body, "times");
    let f = gap_extra::UseridJobPhpFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: json_i32(body, "type"),
        browse: json_opt_i32(body, "browse"),
        datetime_from: from,
        datetime_to: to,
        job_id: {
            let n = json_u64(body, "job_id");
            if n > 0 { Some(n) } else { None }
        },
        com_id: {
            let n = json_u64(body, "com_id");
            if n > 0 { Some(n) } else { None }
        },
        user_id: {
            let n = json_u64(body, "user_id");
            if n > 0 { Some(n) } else { None }
        },
    };
    let db = state.db.reader();
    let total = gap_extra::count_userid_job_php(db, &f).await?;
    let rows = if total > 0 {
        gap_extra::list_userid_job_php(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let dt = if r.datetime > 0 {
                fmt_date(r.datetime)
            } else {
                String::new()
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "eid": r.eid,
                "job_name": r.job_name,
                "job_url": format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.jobid),
                "com_name": r.com_name,
                "com_url": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.comid),
                "username_n": r.username,
                "telphone": r.telphone,
                "telphone_url": r.telphone,
                "is_browse": r.is_browse,
                "datetime": r.datetime,
                "datetime_n_n": dt,
                "isdel_n": r.isdel_n,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn comlog_del_userid_job(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    gap_extra::delete_userid_job_ids(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("ok"))
}

async fn resume_rec(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let rec = json_i32(body, "rec");
    expect_repo::admin_set_rec(state.db.pool(), &ids, rec).await?;
    Ok(PhpOut::Message("ok"))
}

async fn resume_top(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let (top, topdate) = if has_flag(body, "s") {
        (0, 0)
    } else {
        let days = json_i32(body, "addday").max(0);
        let now = clock::now_ts();
        (1, now + i64::from(days) * 86400)
    };
    expect_repo::admin_set_top(state.db.pool(), &ids, top, topdate).await?;
    Ok(PhpOut::Message("ok"))
}

async fn resume_refresh(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    expect_repo::admin_refresh_ids(state.db.pool(), &ids, clock::now_ts()).await?;
    Ok(PhpOut::Message("ok"))
}

fn web_base(state: &AppState) -> String {
    state
        .config
        .web_base_url
        .as_deref()
        .unwrap_or("")
        .trim_end_matches('/')
        .to_string()
}

fn csv_city_labels(dicts: &dict_service::LocalizedDicts, csv: &str) -> (String, String, i32) {
    let mut names = Vec::new();
    for part in csv.split([',', '，']) {
        let Ok(id) = part.trim().parse::<i32>() else {
            continue;
        };
        if id <= 0 {
            continue;
        }
        let n = dicts.city(id);
        if !n.is_empty() {
            names.push(n.to_string());
        }
    }
    let city_n = names.first().cloned().unwrap_or_default();
    let citynum = i32::try_from(names.len()).unwrap_or(0);
    let cityall = names.join("、");
    (city_n, cityall, citynum)
}

fn wx_bind_msg(wxid: &str, unionid: &str) -> String {
    let zh = !matches!(i18n::current_lang(), i18n::Lang::En);
    match (wxid.is_empty(), unionid.is_empty()) {
        (true, _) if zh => "公众号未绑定".into(),
        (true, _) => "Official account is not bound".into(),
        (_, true) if zh => "公众号已绑定".into(),
        (_, true) => "Official account is bound".into(),
        _ if zh => "公众号已绑定，微信开放平台已绑定".into(),
        _ => "Official account is bound, and WeChat Open Platform is bound".into(),
    }
}

fn port_label(port: i32) -> &'static str {
    match port {
        1 => "PC",
        2 => "WAP",
        5 => "Admin",
        _ => "",
    }
}

/// PHP `company::index_action` — `{list,total,perPage,pageSizes}`.
async fn user_gap_company_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let (t0, t1) = json_day_range(body, "times");
    let time_type = json_str(body, "time_type");
    let time_col = match time_type.as_str() {
        "lotime" if t0.is_some() && t1.is_some() => Some("login_date"),
        "adtime" if t0.is_some() && t1.is_some() => Some("reg_date"),
        _ => None,
    };
    let status = json_opt_i32(body, "status").filter(|v| *v > 0);
    let r_status = status.map(|s| match s {
        4 => 0,
        5 => 4,
        other => other,
    });
    let city = json_csv(body, "city_class");
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = company_repo::PhpCompanyListFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        kw_type: json_i32(body, "type"),
        r_status,
        rating: json_opt_i32(body, "rating"),
        rec: json_opt_i32(body, "rec"),
        source: json_opt_i32(body, "source"),
        crm_uid: json_opt_i32(body, "gw"),
        has_job: json_opt_i32(body, "has_job"),
        fact_status: json_opt_i32(body, "fact_status"),
        map_status: json_opt_i32(body, "map_status"),
        city_class: if city.is_empty() {
            None
        } else {
            Some(city.as_str())
        },
        time_col,
        time_from: t0,
        time_to: t1,
        order_t: &order_t,
        order_dir: &order_dir,
    };
    let db = state.db.reader();
    let rows = company_repo::list_php_companies(db, &f, offset, limit).await?;
    let total = company_repo::count_php_companies(db, &f).await?;
    let base = web_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "uid": r.uid,
                "name": r.name,
                "shortname": r.shortname,
                "r_status": r.r_status,
                "rating": r.rating,
                "rating_name": r.rating_name,
                "oldrating_name": "",
                "vipetime": r.vipetime,
                "vip_etime_n": fmt_date(r.vipetime),
                "yyzz_status": r.yyzz_status,
                "logo": r.logo,
                "linktel": r.linktel,
                "linkphone": r.linkphone,
                "linkmail": r.linkmail,
                "crm_uid": r.crm_uid,
                "crm_name": r.crm_name,
                "fact_status": r.fact_status,
                "moblie_status": r.moblie_status,
                "email_status": r.email_status,
                "username": r.username,
                "usertype": r.usertype,
                "wxid": r.wxid,
                "wxopenid": r.wxopenid,
                "unionid": r.unionid,
                "wxBindmsg": wx_bind_msg(&r.wxid, &r.unionid),
                "lock_info": r.lock_info,
                "source": r.source,
                "login_ip": r.login_ip,
                "login_address": r.login_address,
                "moblie_address": r.moblie_address,
                "login_date": r.login_date,
                "login_date_n": fmt_dt(r.login_date),
                "reg_date": r.reg_date,
                "reg_date_n": fmt_dt(r.reg_date),
                "jobnum": r.jobnum,
                "zz_jobnum": r.zz_jobnum,
                "comUrl": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.uid),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `users_resume::index_action` — `{list,total,page_sizes,limit,page}`.
async fn user_gap_resume_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let (t0, t1) = json_day_range(body, "times");
    let time_type = json_str(body, "time_type");
    let time_col = match time_type.as_str() {
        "adtime" if t0.is_some() && t1.is_some() => Some("ctime"),
        "uptime" if t0.is_some() && t1.is_some() => Some("lastupdate"),
        _ => None,
    };
    let now = clock::now_ts();
    let job_class = json_csv(body, "job_class");
    let city_class = json_csv(body, "city_class");
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = expect_repo::PhpResumeListFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keytype: json_i32(body, "keytype"),
        status: json_opt_i32(body, "status"),
        source: json_opt_i32(body, "source"),
        r#type: json_opt_i32(body, "type"),
        edu: json_opt_i32(body, "edu"),
        exp: json_opt_i32(body, "exp"),
        service: json_opt_i32(body, "service"),
        teen: json_i32(body, "teen") == 1,
        teen_since: now - 16 * 365 * 86400,
        now,
        time_col,
        time_from: t0,
        time_to: t1,
        job_class: if job_class.is_empty() {
            None
        } else {
            Some(job_class.as_str())
        },
        city_class: if city_class.is_empty() {
            None
        } else {
            Some(city_class.as_str())
        },
        order_t: &order_t,
        order_dir: &order_dir,
    };
    let db = state.db.reader();
    let rows = expect_repo::list_php_resumes(db, &f, offset, limit).await?;
    let total = expect_repo::count_php_resumes(db, &f).await?;
    let dicts = dict_service::get(state).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let (city_n, cityall, citynum) = csv_city_labels(&dicts, &r.city_classid);
            let top_day = if r.topdate > now {
                ((r.topdate - now) as f64 / 86400.0).ceil() as i64
            } else {
                0
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "name": r.name,
                "uname": r.uname,
                "username": r.username,
                "moblie": r.moblie,
                "moblie_address": r.moblie_address,
                "lock_info": r.lock_info,
                "edu": r.edu,
                "edu_n": dicts.user_or_com(r.edu),
                "exp": r.exp,
                "exp_n": dicts.user_or_com(r.exp),
                "integrity": r.integrity,
                "status": r.status,
                "state": r.state,
                "r_status": r.r_status,
                "statusbody": r.statusbody,
                "rec_resume": r.rec_resume.to_string(),
                "top": r.top,
                "top_day": top_day,
                "defaults": r.defaults,
                "lastupdate": r.lastupdate,
                "lastupdate_n": fmt_dt(r.lastupdate),
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "source": r.source,
                "add_ip": r.add_ip,
                "ip_address": r.ip_address,
                "city_classid": r.city_classid,
                "city_n": city_n,
                "cityall": cityall,
                "citynum": citynum,
                "doc": r.doc,
                "sq_num": r.sq_num,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `company_job_refresh_log::index_action`.
async fn user_gap_job_refresh_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let r#type = json_opt_i32(body, "type");
    let ktype = json_i32(body, "ktype");
    let db = state.db.reader();
    let rows = gap_repo::list_php_refresh_logs(
        db,
        r#type,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        ktype,
        offset,
        limit,
    )
    .await?;
    let total = gap_repo::count_php_refresh_logs(
        db,
        r#type,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        ktype,
        )
        .await?;
    let base = web_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let joburl = if r.r#type == 2 {
                format!("{base}/index.php?m=part&c=show&id={}&look=admin", r.jobid)
            } else {
                format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.jobid)
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "jobid": r.jobid,
                "usertype": r.usertype,
                "type": r.r#type,
                "ip": r.ip,
                "remark": r.remark,
                "job_name": r.job_name,
                "com_name": r.com_name,
                "port_n": port_label(r.port),
                "r_time_n": fmt_dt(r.r_time),
                "joburl": joburl,
                "comurl": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.uid),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn user_gap_job_refresh_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Ok(PhpOut::Message("common_00740"));
    }
    gap_repo::delete_php_refresh_logs(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("admin_user_00187"))
}
