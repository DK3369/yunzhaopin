//! PHP admin named actions for 招聘会 / 新闻 / 问答 / 专题.
//! SQL stays in repos. Routes are `php-*` and stay out of AdminDoc.

use std::collections::{HashMap, HashSet};

use chrono::{Datelike, TimeZone};
use phpyun_core::i18n;
use phpyun_core::utils::{fmt_date, fmt_dt, fmt_ts};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::ad::repo as ad_repo;
use phpyun_models::admin_gap::datacall as gap_datacall;
use phpyun_models::admin_gap::extra as gap_extra;
use phpyun_models::admin_gap::repo as gap_repo;
use phpyun_models::admin_gap::tongji as gap_tongji;
use phpyun_models::admin_rbac::php as rbac_php;
use phpyun_models::admin_rbac::php_power;
use phpyun_models::admin_rbac::repo as admin_user_repo;
use phpyun_models::nav_menu::php as nav_php;
use phpyun_models::seo as seo_repo;
use phpyun_models::company_tpl::repo as company_tpl_repo;
use phpyun_models::admin_msg::repo as admin_msg_repo;
use phpyun_models::company_statis::repo as cstatis_repo;
use phpyun_models::integral_transfer::repo as pay_repo;
use phpyun_models::member_statis::repo as mstatis_repo;
use phpyun_models::vip::repo as vip_repo;
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::announcement::repo as announcement_repo;
use phpyun_models::article::repo::{self as article_repo, ArticleFilter};
use phpyun_auth::{argon2_hash_async, md5_hex};
use phpyun_models::category::repo as cat_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::description::repo as desc_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::email_msg::repo as email_msg_repo;
use phpyun_models::friend_link::repo as friend_link_repo;
use phpyun_models::gongzhao::repo as gongzhao_repo;
use phpyun_models::moblie_msg::repo as moblie_msg_repo;
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
use phpyun_models::recycle_bin::php_repo as recycle_php;
use phpyun_models::resume::skill as skill_repo;
use phpyun_models::resume::work as work_repo;
use phpyun_models::resume::edu as edu_repo;
use phpyun_models::resume::training as training_repo;
use phpyun_models::company_address::repo as address_repo;
use phpyun_models::site_page::repo as site_page_repo;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::special::repo as special_repo;
use phpyun_models::tiny::repo as tiny_repo;
use phpyun_models::redeem::repo as redeem_repo;
use phpyun_models::user::repo as user_repo;
use phpyun_models::warning::repo as warning_repo;
use phpyun_models::wx_nav::repo as wx_nav_repo;
use phpyun_models::zph::repo as zph_repo;
use serde_json::{json, Value};

use crate::ad_service;
use crate::admin_cms_service;
use crate::admin_dashboard_service;
use crate::admin_longtail_service;
use crate::admin_report_service;
use crate::description_service;
use crate::dict_service;
use crate::friend_link_service;
use crate::home_service;
use crate::mail_service;
use crate::redeem_service;
use crate::job_scrape_service;
use crate::site_setting_service;
use crate::wechat_api_service;
use phpyun_models::entrust_record;
use phpyun_models::report::repo as report_repo;
use uuid::Uuid;

pub enum PhpOut {
    Data(Value),
    Message(&'static str),
    /// Already-translated message, for PHP strings with runtime placeholders.
    /// The key still travels so clients can branch on it.
    Text(&'static str, String),
    /// Success message plus payload (e.g. generate-page loop stopper `{type:ok}`).
    MessageData(&'static str, Value),
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
        ("fairs", "upload") => Ok(PhpOut::Data(fairs_upload(state, body).await?)),
        ("fairs", "uploadsave") => fairs_uploadsave(state, body).await,
        ("fairs", "setthemb") => fairs_setthemb(state, body).await,
        ("fairs", "delpic") => fairs_delpic(state, body).await,
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
        ("news", "ajax_menu") => Ok(PhpOut::Data(news_ajax_menu(state, body).await?)),
        ("news", "set_menu") => news_set_menu(state, body).await,
        ("news", "make_cache") => {
            home_service::invalidate_all().await;
            Ok(PhpOut::Message("admin_system_00064"))
        },
        ("gongzhao", "index") => Ok(PhpOut::Data(gongzhao_index(state, body).await?)),
        ("gongzhao", "getGroup") => Ok(PhpOut::Data(gongzhao_get_group(state).await?)),
        ("gongzhao", "add") => gongzhao_add(state, user, body).await,
        ("gongzhao", "delete") => gongzhao_del(state, body).await,
        ("gongzhao", "checksitedid") => gongzhao_checksitedid(state, body).await,
        ("gongzhao", "setRec") => gongzhao_set_rec(state, body).await,
        ("gongzhao", "whb") => Ok(PhpOut::Data(gongzhao_whb(state).await?)),
        ("announce", "getGroup") => Ok(PhpOut::Data(announce_get_group(state).await?)),
        ("announce", "checksitedid") => announce_checksitedid(state, body).await,
        ("announce", "index") => Ok(PhpOut::Data(announce_index(state, body).await?)),
        ("announce", "delete") => announce_del(state, user, body).await,
        ("ads", "index") => Ok(PhpOut::Data(ads_index(state, body).await?)),
        ("ads", "get_base_data") => Ok(PhpOut::Data(ads_get_base(state).await?)),
        ("ads", "info") => Ok(PhpOut::Data(ads_info(state, body).await?)),
        ("ads", "ad_saveadd") => ads_saveadd(state, body).await,
        ("ads", "delete") => ads_del(state, body).await,
        ("ads", "preview") => Ok(PhpOut::Data(ads_preview(state, body).await?)),
        ("ads", "check") => ads_check(state, body).await,
        ("ads", "cache_ad") => {
            ad_service::invalidate_all();
            Ok(PhpOut::Message("admin_01172"))
        }
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
        ("finance-order", "delete") => finance_order_del(state, user, body).await,
        ("finance-order", "xls") => Ok(PhpOut::Data(finance_order_xls(state, body).await?)),
        ("finance-order", "upload") => Ok(PhpOut::Data(finance_order_upload(state, body).await?)),
        ("finance-order", "multiupload") => Ok(PhpOut::Data(finance_order_multiupload(body))),
        ("finance-order", "uploadsave") => finance_order_uploadsave(state, body).await,
        ("finance-order", "htpic_del") => finance_order_htpic_del(state, body).await,
        ("comset", "logo") => comset_logo(state, user, body).await,
        ("comset", "rating") => Ok(PhpOut::Data(comset_rating(state, body).await?)),
        ("comset", "comspend") => Ok(PhpOut::Data(comset_comspend(state).await?)),
        ("userset", "userspend") => Ok(PhpOut::Data(userset_userspend(state).await?)),
        ("userset", "saveLogo") => userset_save_logo(state, user, body).await,
        ("rating", "ajax") => rating_ajax(state, body).await,
        ("rating", "zzData") => Ok(PhpOut::Data(rating_zz_data(state).await?)),
        ("rating", "edittc") => Ok(PhpOut::Data(rating_edittc(state, body).await?)),
        ("rating", "del") => rating_del_detail(state, body).await,
        ("email-log", "index") => Ok(PhpOut::Data(email_log_index(state, body).await?)),
        ("email-log", "delete") => email_log_del(state, user, body).await,
        ("email-log", "repeat") => email_log_repeat(state, body).await,
        ("sms-log", "index") => Ok(PhpOut::Data(sms_log_index(state, body).await?)),
        ("sms-log", "delete") => sms_log_del(state, user, body).await,
        ("sms-log", "repeat") => sms_log_repeat(state, body).await,
        ("warning", "index") => Ok(PhpOut::Data(warning_index(state, body).await?)),
        ("warning", "delete") => warning_del(state, user, body).await,
        ("warning", "getWarningConfig") => Ok(PhpOut::Data(warning_get_config(state).await?)),
        ("warning", "config") => warning_config_save(state, user, body).await,
        ("cron-log", "index") => Ok(PhpOut::Data(cron_log_index(state, body).await?)),
        ("cron-log", "delete") => cron_log_del(state, user, body).await,
        ("cron", "index") => Ok(PhpOut::Data(cron_index(state, body).await?)),
        ("shop-reward", "index") => Ok(PhpOut::Data(shop_reward_index(state, body).await?)),
        ("shop-reward", "rec") => shop_reward_flag(state, user, body, true).await,
        ("shop-reward", "hot") => shop_reward_flag(state, user, body, false).await,
        ("shop-reward", "getclass") => Ok(PhpOut::Data(shop_reward_getclass(state, body).await?)),
        ("shop-reward", "add") => shop_reward_add(state, user, body).await,
        ("shop-reward", "status") => shop_reward_status(state, user, body).await,
        ("shop-reward", "delete") => shop_reward_del(state, user, body).await,
        ("shop-class", "index") => Ok(PhpOut::Data(shop_class_index(state).await?)),
        ("shop-class", "up") => Ok(PhpOut::Data(shop_class_up(state, body).await?)),
        ("shop-class", "ajax") => shop_class_ajax(state, user, body).await,
        ("shop-class", "save") => shop_class_save(state, user, body).await,
        ("shop-class", "delete") => shop_class_del(state, user, body).await,
        ("shop-list", "index") => Ok(PhpOut::Data(shop_list_index(state, body).await?)),
        ("shop-list", "status") => shop_list_status(state, user, body).await,
        ("shop-list", "delete") => shop_list_del(state, user, body).await,
        ("friend-link", "getInfo") => Ok(PhpOut::Data(friend_link_get_info(state, body).await?)),
        ("friend-link", "sitedid") => friend_link_sitedid(state, body).await,
        ("friend-link", "index") => Ok(PhpOut::Data(friend_link_index(state, body).await?)),
        ("friend-link", "status") => friend_link_status(state, body).await,
        ("friend-link", "delete") => friend_link_del(state, user, body).await,
        ("friend-link", "save") => friend_link_save(state, body).await,
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
        ("special", "savespecial") => special_savespecial(state, body).await,
        ("special", "mutiAddCom") => special_muti_add_com(state, body).await,
        ("special", "comxls") => Ok(PhpOut::Data(special_comxls(state, body).await?)),
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
        ("hotjob", "index") => Ok(PhpOut::Data(hotjob_index(state, body).await?)),
        ("hotjob", "getComList") => Ok(PhpOut::Data(hotjob_com_list(state, body).await?)),
        ("hotjob", "gethotjob") => Ok(PhpOut::Data(hotjob_get(state, body).await?)),
        ("hotjob", "hotjobinfo") => Ok(PhpOut::Data(hotjob_info(state, body).await?)),
        ("hotjob", "hotNum") => Ok(PhpOut::Data(hotjob_num(state).await?)),
        ("hotjob", "delete") => hotjob_del(state, user, body).await,
        ("resume", "skill") => resume_skill(state, body).await,
        ("resume", "project") => resume_project(state, body).await,
        ("resume", "other") => resume_other(state, body).await,
        ("resume", "rec") => resume_rec(state, body).await,
        ("resume", "top") => resume_top(state, body).await,
        ("resume", "refresh") => resume_refresh(state, body).await,
        ("resume", "delResume") => resume_del(state, user, body).await,
        ("resume", "delResumeFb") => resume_del_fb(state, body).await,
        ("resume", "label") => resume_label(state, body).await,
        ("resume", "resumePreview") => Ok(PhpOut::Data(resume_preview(state, body).await?)),
        ("resume", "export_check") => resume_export_check(state, body).await,
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
        ("cat-class", "clearpinyin") => cat_class_clearpinyin(state, body).await,
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
        ("user-gap", "writtenoff-index") => {
            Ok(PhpOut::Data(user_gap_writtenoff_index(state, body).await?))
        }
        ("user-gap", "writtenoff-del") => user_gap_writtenoff_del(state, body).await,
        ("user-gap", "apply-log") => Ok(PhpOut::Data(user_gap_apply_log(state, body).await?)),
        ("user-gap", "invite-log") => Ok(PhpOut::Data(user_gap_invite_log(state, body).await?)),
        ("user-gap", "pay-log") => Ok(PhpOut::Data(user_gap_pay_log(state, body).await?)),
        ("user-gap", "search-com") => Ok(PhpOut::Data(user_gap_search_com(state, body).await?)),
        ("user-gap", "member-activity") => {
            Ok(PhpOut::Data(user_gap_member_activity(state, body).await?))
        }
        ("user-gap", "member-activity-del") => user_gap_member_activity_del(state, body).await,
        ("user-gap", "usercert") => user_gap_usercert(state, body).await,
        ("user-gap", "member-checksitedid") => user_gap_member_checksitedid(state, body).await,
        ("user-gap", "company-checksitedid") => user_gap_company_checksitedid(state, body).await,

        ("company-job", "depower") => company_job_depower(state, body).await,
        ("company-job", "setlinkopen") => company_job_setlinkopen(state, body).await,
        ("company-job", "upjobhits") => company_job_upjobhits(state, body).await,
        ("company-job", "reserve-index") => {
            Ok(PhpOut::Data(company_job_reserve_index(state, body).await?))
        }
        ("company-job", "get-refresh") => {
            Ok(PhpOut::Data(company_job_get_refresh(state, body).await?))
        }
        ("company-job", "close-reserve") => company_job_close_reserve(state, body).await,
        ("company-job", "close-stale-reserve") => company_job_close_stale_reserve(state).await,
        ("company-job", "up-reserve") => company_job_up_reserve(state, body).await,
        ("company-job", "saveAddress") => company_job_save_address(state, body).await,
        ("company-job", "saveclass") => company_job_saveclass(state, body).await,
        ("company-job", "getJobHtml") => Ok(PhpOut::Data(company_job_get_html(state, body).await?)),
        ("company-job", "addTuiWenTask") => company_job_add_tuiwen(state, user, body).await,
        ("company-job", "whb") => Ok(PhpOut::Data(company_whb(state, 1).await?)),
        ("company-job", "getHbData") => Ok(PhpOut::Data(company_job_hb_data(state).await?)),
        ("company-job", "xls") => Ok(PhpOut::Data(company_job_xls(state, body).await?)),
        ("company-job", "status") => company_job_status(state, body).await,
        ("company-job", "cjobstatus") => company_job_cjobstatus(state, body).await,
        ("company-job", "jobAudit") => Ok(PhpOut::Data(company_job_audit(state, body).await?)),
        ("company-job", "applyJob") => company_job_apply(state, body).await,
        ("resume", "status") => resume_php_status(state, body).await,
        ("resume", "resumestatus") => resume_php_resumestatus(state, body).await,
        ("part", "status") => part_php_status(state, body).await,
        ("part", "tbStatus") => part_php_tb_status(state, body).await,
        ("company-cert", "index") => Ok(PhpOut::Data(company_cert_index(state, body).await?)),
        ("company-cert", "status") => company_cert_status(state, body).await,
        ("company-cert", "getCertStatist") => {
            Ok(PhpOut::Data(company_cert_statist(state).await?))
        }
        ("company-cert", "sbody") => Ok(PhpOut::Data(company_cert_sbody(state, body).await?)),
        ("set-integral", "index") => Ok(PhpOut::Data(set_integral_index(state).await?)),
        ("set-integral", "save") => set_integral_save(state, user, body).await,
        ("set-integral", "saveSet") => set_integral_save(state, user, body).await,
        ("set-integral", "comjifen") => set_integral_save(state, user, body).await,
        ("set-integral", "class") => Ok(PhpOut::Data(set_integral_class(state).await?)),
        ("set-integral", "ajax") => set_integral_ajax(state, body).await,
        ("set-integral", "del") => set_integral_del(state, body).await,
        ("set-config", "save_logo") => set_config_save_logo(body),
        ("set-config", "settplcache") => Ok(PhpOut::Data(set_config_settplcache(state).await?)),
        ("set-config", "savetplcache") => set_config_savetplcache(state, user, body).await,
        ("index", "mapconfig") => Ok(PhpOut::Data(index_mapconfig(state).await?)),
        ("cache", "getPriceName") => Ok(PhpOut::Data(cache_get_price_name(state).await?)),

        ("company", "bind-package") => company_bind_package(state, body).await,
        ("company", "set-logo") => company_set_logo(state, body).await,
        ("company", "check-guwen") => company_check_guwen(state, body).await,
        ("company", "statis-detail") => {
            Ok(PhpOut::Data(company_statis_detail(state, body).await?))
        }
        ("company", "del-statis-detail") => company_del_statis_detail(state, body).await,
        ("company", "mcomtpl") => Ok(PhpOut::Data(company_mcomtpl(state, body).await?)),
        ("company", "msettpl") => company_msettpl(state, body).await,
        ("company", "add-tuiwen-task") => company_add_tuiwen_task(state, user, body).await,
        ("company", "savefact") => company_savefact(state, user, body).await,
        ("company", "getacbindstatus") => company_getacbindstatus(state, body).await,
        ("company", "export_check") => company_export_check(state, body).await,
        ("company", "mwhb") => Ok(PhpOut::Data(company_whb(state, 2).await?)),
        ("company", "adminLogoHb") => company_admin_logo_hb(body),
        ("user-gap", "mem-imitate") => Ok(PhpOut::Data(user_gap_mem_imitate(state, body).await?)),
        ("user-gap", "mem-lock") => user_gap_mem_lock(state, body).await,
        ("user-gap", "mem-edit") => user_gap_mem_edit(state, body).await,
        ("user-gap", "mem-del") => user_gap_mem_del(state, body).await,
        ("user-gap", "company-del") => user_gap_company_del(state, body).await,
        ("user-gap", "user-del") => user_gap_user_del(state, body).await,
        ("user-gap", "company-status") => user_gap_company_status(state, body).await,
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
        ("user-gap", "user-index") => Ok(PhpOut::Data(user_gap_user_index(state, body).await?)),
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
        ("email-set", "delconfig") => email_set_delconfig(state, user, body).await,
        ("email-set", "tplswitch") => Ok(PhpOut::Data(tplswitch_data(state, "email").await?)),
        ("email-set", "savetplconfig") => email_set_savetplconfig(state, user, body).await,
        ("message-set", "tplswitch") => Ok(PhpOut::Data(tplswitch_data(state, "msg").await?)),
        ("message-set", "gettpl") => Ok(PhpOut::Data(email_set_gettpl(state, body).await?)),
        ("message-set", "savetpl") => message_set_savetpl(state, body).await,
        ("admin-uc", "index") => Ok(PhpOut::Data(admin_uc_index(state).await?)),
        ("admin-uc", "ucsave") => admin_uc_save(state, user, body, false).await,
        ("admin-uc", "pwsave") => admin_uc_save(state, user, body, true).await,
        ("admin-member", "send") => admin_member_send_email(state, body).await,
        ("admin-member", "msgsave") => admin_member_send_sms(state, body).await,
        ("weixinrecord", "clearwx") => weixinrecord_clearwx(state).await,
        ("weixinrecord", "index") => Ok(PhpOut::Data(weixinrecord_index(state, body).await?)),
        ("weixinrecord", "userbd") => Ok(PhpOut::Data(weixinrecord_userbd(state, body).await?)),
        ("weixinrecord", "deluser") => weixinrecord_deluser(state, body).await,
        ("weixinrecord", "keyword") => Ok(PhpOut::Data(weixinrecord_keyword(state, body).await?)),
        ("weixinrecord", "delkeyword") => weixinrecord_delkeyword(state, user, body).await,
        ("role-user", "index") => Ok(PhpOut::Data(role_user_index(state, body).await?)),
        ("role-user", "save") => role_user_save(state, body).await,
        ("role-user", "delete") => role_user_del(state, user, body).await,
        ("role-ugroup", "index") => Ok(PhpOut::Data(role_ugroup_index(state, body).await?)),
        ("role-ugroup", "info") => Ok(PhpOut::Data(role_ugroup_info(state, body).await?)),
        ("role-ugroup", "save") => role_ugroup_save(state, body).await,
        ("role-ugroup", "delete") => role_ugroup_del(state, user, body).await,
        ("role-myuser", "index") => Ok(PhpOut::Data(role_myuser_index(state, user).await?)),
        ("role-myuser", "delAdminQyUserId") => role_myuser_del_qy(state, user).await,
        ("role-myuser", "qwcallback") => Ok(PhpOut::Message("admin_system_00023")),
        ("admin-nav", "index") => Ok(PhpOut::Data(admin_nav_index(state, body).await?)),
        ("admin-nav", "info") => Ok(PhpOut::Data(admin_nav_info(state, body).await?)),
        ("admin-nav", "add") => admin_nav_add(state, body).await,
        ("admin-nav", "delete") => admin_nav_del(state, user, body).await,
        ("admin-nav", "changeDisplay") => admin_nav_change_display(state, body).await,
        ("admin-nav", "version") => Ok(PhpOut::Data(admin_nav_version(state).await?)),
        ("front-nav", "index") => Ok(PhpOut::Data(front_nav_index(state, body).await?)),
        ("front-nav", "add") => Ok(PhpOut::Data(front_nav_add(state, body).await?)),
        ("front-nav", "save") => front_nav_save(state, body).await,
        ("front-nav", "delete") => front_nav_del(state, user, body).await,
        ("front-nav", "navset") => front_nav_navset(state, body).await,
        ("front-nav", "navsort") => front_nav_navsort(state, body).await,
        ("front-nav", "type") => Ok(PhpOut::Data(front_nav_type(state).await?)),
        ("front-nav", "typeadd") => front_nav_typeadd(state, body).await,
        ("front-nav", "typename") => front_nav_typename(state, body).await,
        ("front-nav", "typedel") => front_nav_typedel(state, user, body).await,
        ("navmap", "index") => Ok(PhpOut::Data(navmap_index(state, body).await?)),
        ("navmap", "getTypes") => Ok(PhpOut::Data(navmap_get_types(state).await?)),
        ("navmap", "save") => navmap_save(state, body).await,
        ("navmap", "delete") => navmap_del(state, user, body).await,
        ("navmap", "nav_xianshi") => navmap_xianshi(state, body).await,
        ("set-module", "index") => Ok(PhpOut::Data(set_module_index(state).await?)),
        ("set-module", "save") => set_module_save(state, user, body).await,
        ("set-module", "navset") => Ok(PhpOut::Data(set_module_navset(state, body).await?)),
        ("set-module", "navsetSave") => set_module_navset_save(state, body).await,
        ("set-module", "getseo") => Ok(PhpOut::Data(set_module_getseo(state, body).await?)),
        ("set-module", "seoshezhi") => set_module_seoshezhi(state, body).await,
        ("sysmsg", "index") => Ok(PhpOut::Data(sysmsg_index(state, body).await?)),
        ("sysmsg", "delete") => sysmsg_del(state, user, body).await,
        ("sysmsg", "sendSys") => sysmsg_send(state, body).await,
        ("error-log", "index") => Ok(PhpOut::Data(error_log_index(state, body).await?)),
        ("error-log", "delete") => error_log_del(state, user, body).await,
        ("admin-log", "index") => Ok(PhpOut::Data(admin_log_index(state, body).await?)),
        ("domain-group", "groupList") => Ok(PhpOut::Data(domain_group_list(state, body).await?)),
        ("domain-group", "groupInfo") => Ok(PhpOut::Data(domain_group_info(state, body).await?)),
        ("domain-group", "saveGroup") => domain_group_save(state, body).await,
        ("domain-group", "delGroup") => domain_group_del(state, user, body).await,
        ("domain-group", "adminList") => Ok(PhpOut::Data(domain_admin_list(state, body).await?)),
        ("domain-group", "adminInfo") => Ok(PhpOut::Data(domain_admin_info(state, body).await?)),
        ("domain-list", "index") => Ok(PhpOut::Data(domain_list_index(state, body).await?)),
        ("domain-list", "changeDomainType") => domain_list_change_type(state, body).await,
        ("domain-list", "configSave") => domain_list_config_save(state, user, body).await,
        ("domain-list", "getDomainCache") => Ok(PhpOut::Data(domain_list_get_cache(state).await?)),
        ("zph-space", "ajax") => zph_space_ajax(state, body).await,
        ("zph-space", "ajaxspace") => Ok(PhpOut::Data(zph_space_ajaxspace(state, body).await?)),
        ("zph-space", "up") => Ok(PhpOut::Data(zph_space_up(state, body).await?)),
        ("zph-space", "index") => Ok(PhpOut::Data(zph_space_index(state, body).await?)),
        ("zph-space", "add") => zph_space_add(state, body).await,
        ("zph-space", "delete") => zph_space_del(state, body).await,
        ("report-resume", "delresume") => report_delresume(state, user, body).await,
        ("report-resume", "delresumeall") => report_delresumeall(state, user, body).await,
        ("fabutool", "index") => Ok(PhpOut::Data(fabutool_index(state, body).await?)),
        ("fabutool", "wxPubTemp") => Ok(PhpOut::Data(fabutool_wx_pub_temp(state, body).await?)),
        ("fabutool", "wxPubTempSave") => fabutool_wx_pub_temp_save(state, body).await,
        ("fabutool", "wxPubTempDel") => fabutool_wx_pub_temp_del(state, user, body).await,
        ("fabutool", "pubtool") => Ok(PhpOut::Data(fabutool_pubtool(state).await?)),
        ("fabutool", "getWxpubJob") => Ok(PhpOut::Data(fabutool_get_wxpub_job(state, body).await?)),
        ("fabutool", "getComBySearch") => Ok(PhpOut::Data(fabutool_get_com_search(state, body).await?)),
        ("fabutool", "Getpubtool") => Ok(PhpOut::Data(Value::String(fabutool_get_pubtool(state, body).await?))),
        ("fabutool", "getTW") => Ok(PhpOut::Data(Value::String(fabutool_get_tw(state, body).await?))),
        ("fabutool", "getComTW") => Ok(PhpOut::Data(Value::String(fabutool_get_com_tw(state, body).await?))),
        ("fabutool", "twTask") => Ok(PhpOut::Data(fabutool_tw_task(state, body, 1).await?)),
        ("fabutool", "comtwTask") => Ok(PhpOut::Data(fabutool_tw_task(state, body, 2).await?)),
        ("fabutool", "twTask_base_data") => Ok(PhpOut::Data(fabutool_tw_base(state, true).await?)),
        ("fabutool", "comtwTask_base_data") => Ok(PhpOut::Data(fabutool_tw_base(state, false).await?)),
        ("fabutool", "delTwTask") => fabutool_del_tw(state, user, body).await,
        ("fabutool", "taskFinish") => fabutool_task_finish(state, body).await,
        ("tuiguang", "index") => Ok(PhpOut::Data(tuiguang_index(state, false).await?)),
        ("tuiguang", "msgtg") => Ok(PhpOut::Data(tuiguang_index(state, true).await?)),
        ("tuiguang", "getBirthday") => Ok(PhpOut::Data(tuiguang_birthday(state, body).await?)),
        ("tuiguang", "getcom") => Ok(PhpOut::Data(json!(tuiguang_getcom(state, body).await?))),
        ("tuiguang", "getuser") => Ok(PhpOut::Data(json!(tuiguang_getuser(state, body).await?))),
        ("tuiguang", "getjob") => Ok(PhpOut::Data(json!(tuiguang_getjob(state, body).await?))),
        ("tuiguang", "sendresume") => Ok(PhpOut::Data(tuiguang_sendresume(state, body).await?)),
        ("tuiguang", "sendjob") => Ok(PhpOut::Data(tuiguang_sendjob(state, body).await?)),
        ("tplset", "index") => Ok(PhpOut::Data(tplset_index(state).await?)),
        ("tplset", "stylesave") => tplset_stylesave(body),
        ("tplset", "check_style") => tplset_check_style(state, body).await,
        ("tplset", "comtpl") => Ok(PhpOut::Data(tplset_comtpl(state).await?)),
        ("tplset", "comptplsave") => tplset_com_save(state, body).await,
        ("tplset", "comtpldel") => tplset_com_del(state, user, body).await,
        ("tplset", "resumetpl") => Ok(PhpOut::Data(tplset_resume_list(state).await?)),
        ("tplset", "resumetplsave") => tplset_resume_save(state, body).await,
        ("tplset", "resumetpldel") => tplset_resume_del(state, user, body).await,
        ("tplset", "pcindextpl") => Ok(PhpOut::Data(tplset_index_list(state).await?)),
        ("tplset", "indextplsave") => tplset_index_save(state, body).await,
        ("tplset", "indextpldel") => tplset_index_del(state, user, body).await,
        ("database", "getOptTable") => Ok(PhpOut::Data(db_opt_table(state).await?)),
        ("database", "optimizeTable") => db_optimize(state, body).await,
        ("database", "clearData") => Ok(PhpOut::Data(db_clear(state, body).await?)),
        ("database", "getDbTable") => Ok(PhpOut::Data(db_table_names(state).await?)),
        ("database", "getBackFile") => Ok(PhpOut::Data(json!([]))),
        ("database", "backUp") => Err(ApiError::business("admin_tool_00510")),
        ("database", "delBack") => Err(ApiError::business("admin_tool_00510")),
        ("database", "backIn") => Err(ApiError::business("admin_tool_00510")),
        ("generate-page", "baseData") => Ok(PhpOut::Data(gen_page_base(state).await?)),
        ("generate-page", "index") => gen_page_index(state, user, body).await,
        ("generate-page", "news") => gen_page_news(state, user, body).await,
        ("generate-page", "archive") => Ok(PhpOut::MessageData("admin_ssr_no_static", gen_ssr_ok())),
        ("generate-page", "once") => Ok(PhpOut::Message("admin_ssr_no_static")),
        ("generate-page", "newsclass") => Ok(PhpOut::MessageData("admin_ssr_no_static", gen_ssr_ok())),
        ("generate-page", "all") => Ok(PhpOut::MessageData("admin_ssr_no_static", gen_ssr_ok())),
        ("generate-cache", "index") => Ok(PhpOut::Data(gen_cache_index())),
        ("generate-cache", "cache") => gen_cache_run(state, user, body).await,
        ("generate-xml", "archive") => Ok(PhpOut::Message("admin_ssr_no_static")),
        ("gap-skip", "crm") => Err(ApiError::business("admin_crm_unavailable")),
        ("shop-set", "index") => Ok(PhpOut::Data(shop_set_index(state).await?)),
        ("shop-set", "saveset") => shop_set_saveset(state, user, body).await,
        ("shop-set", "get_redeem_option") => Ok(PhpOut::Data(shop_set_redeem_option(state, body).await?)),
        ("hbconfig", "saveWhb") => hbconfig_save_whb(state, user, body).await,
        ("hbconfig", "delWhb") => hbconfig_del_whb(state, user, body).await,
        ("hrlog", "editsave") => hrlog_editsave(state, body).await,
        ("hrlog", "rehrlog") => hrlog_rehrlog(state, body).await,
        ("hrlog", "set") => Ok(PhpOut::Data(hrlog_set(state).await?)),
        ("hrlog", "setSave") => hrlog_set_save(state, user, body).await,
        ("hrlog", "getHb") => Ok(PhpOut::Data(hrlog_get_hb(state, body).await?)),
        ("trust", "recom") => Ok(PhpOut::Data(trust_recom(state, body).await?)),
        ("trust", "directrecom") => trust_directrecom(state, body).await,
        ("data-board", "index") => Ok(PhpOut::Data(data_board_index(state, body).await?)),
        ("data-board", "class") => Ok(PhpOut::Data(data_board_class(state, body).await?)),
        ("data-board", "fenxiabiao") => Ok(PhpOut::Data(data_board_fenxiabiao(state, body).await?)),
        ("data-board", "getAuth") => Ok(PhpOut::Data(data_board_get_auth(state, user, body).await?)),
        ("data-call", "getPreviewData") => Ok(PhpOut::Data(data_call_preview(state, body).await?)),
        ("data-collection", "getRating") => Ok(PhpOut::Data(data_collection_rating(state).await?)),
        ("data-collection", "index") => Ok(PhpOut::Data(data_collection_index(state).await?)),
        ("data-collection", "scrapeGet") => Ok(PhpOut::Data(job_scrape_service::admin_get(state).await?)),
        ("data-collection", "scrapeSave") => {
            job_scrape_service::admin_save(state, body).await?;
            Ok(PhpOut::Message("api_wxapp_00007"))
        }
        ("data-collection", "scrapeRun") => {
            Ok(PhpOut::MessageData(
                "job_scrape_done",
                job_scrape_service::admin_run(state).await?,
            ))
        }
        ("index", "getIpAddress") => index_get_ip_address(state, body).await,
        ("index", "getMobileAddress") => index_get_mobile_address(state, body).await,
        ("index", "wxbind") => Ok(PhpOut::Data(index_wxbind(state, user).await?)),
        ("index", "getwxbindstatus") => index_wxbind_status(state, user).await,
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

fn json_str_list(v: &Value, key: &str) -> Vec<String> {
    match v.get(key) {
        Some(Value::Array(a)) => a
            .iter()
            .map(|x| match x {
                Value::String(s) => s.trim().to_string(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            })
            .filter(|s| !s.is_empty())
            .collect(),
        Some(Value::String(s)) => s
            .split([',', ';'])
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect(),
        _ => Vec::new(),
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

fn json_i64(v: &Value, key: &str) -> i64 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn json_ts(v: &Value, key: &str) -> i64 {
    match v.get(key) {
        Some(Value::Number(n)) => n
            .as_i64()
            .or_else(|| n.as_f64().map(|f| f as i64))
            .unwrap_or(0),
        Some(Value::String(s)) => {
            let s = s.trim();
            s.parse::<i64>()
                .ok()
                .or_else(|| s.parse::<f64>().ok().map(|f| f as i64))
                .unwrap_or(0)
        }
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

fn json_ms_day_range(body: &Value, key: &str) -> (Option<i64>, Option<i64>) {
    let arr = match body.get(key) {
        Some(Value::Array(a)) if a.len() >= 2 => a,
        _ => return (None, None),
    };
    let mut a = match &arr[0] {
        Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_f64().map(|f| f as i64))
            .unwrap_or(0),
        Value::String(s) => s.trim().parse().unwrap_or(0),
        _ => 0,
    };
    let mut b = match &arr[1] {
        Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_f64().map(|f| f as i64))
            .unwrap_or(0),
        Value::String(s) => s.trim().parse().unwrap_or(0),
        _ => 0,
    };
    if a <= 0 || b <= 0 {
        return (None, None);
    }
    if a > 10_000_000_000 {
        a /= 1000;
    }
    if b > 10_000_000_000 {
        b /= 1000;
    }
    (
        Some(clock::start_of_day(a)),
        Some(clock::start_of_day(b) + 86_400 - 1),
    )
}

fn php_preset_days(code: i32) -> (Option<i64>, Option<i64>) {
    if code == 0 {
        return (None, None);
    }
    let now = clock::now_ts();
    let today = clock::start_of_day(now);
    match code {
        -1 => (Some(today - 86_400), Some(today)),
        1 => (Some(today), Some(now)),
        2 => (Some(now - 7 * 86_400), Some(now)),
        3 => (Some(now - 30 * 86_400), Some(now)),
        4 => (Some(now - 180 * 86_400), Some(now)),
        5 => (Some(now - 365 * 86_400), Some(now)),
        _ => (None, None),
    }
}

fn merge_ts_range(
    a: (Option<i64>, Option<i64>),
    b: (Option<i64>, Option<i64>),
) -> (Option<i64>, Option<i64>) {
    let from = match (a.0, b.0) {
        (Some(x), Some(y)) => Some(x.max(y)),
        (x, y) => x.or(y),
    };
    let to = match (a.1, b.1) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (x, y) => x.or(y),
    };
    (from, to)
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

async fn fairs_upload(state: &AppState, body: &Value) -> AppResult<Value> {
    let zid = json_u64(body, "id");
    let base = preview_base(state);
    let rows = zph_repo::list_pics(state.db.reader(), zid).await?;
    let mut pics: Vec<String> = Vec::new();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let pic_n = pic_url(&base, &r.pic);
            pics.push(pic_n.clone());
            json!({
                "id": r.id,
                "title": r.title,
                "pic": r.pic,
                "pic_n": pic_n,
                "sort": r.sort,
                "zid": r.zid,
                "is_themb": if r.is_themb == "1" { "1" } else { "0" },
                "did": r.did,
            })
        })
        .collect();
    let row = zph_repo::find_admin_form(state.db.reader(), zid)
        .await?
        .map(|r| json!({ "id": r.id, "title": r.title, "did": r.did }))
        .unwrap_or_else(|| json!({}));
    Ok(json!({ "row": row, "list": list, "pics": pics }))
}

fn media_path_of(cfg: &HashMap<String, String>, raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty()
        || s.starts_with("data:")
        || s.starts_with("blob:")
        || s.contains('\0')
    {
        return String::new();
    }
    strip_site_url(cfg, s)
}

async fn fairs_uploadsave(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let cfg = settings_hash(state).await?;
    let pic = media_path_of(&cfg, &json_str(body, "pic"));
    let title = json_str(body, "title");
    if title.is_empty() {
        return Err(ApiError::business("admin_vue_00073"));
    }
    let id = json_u64(body, "id");
    if id == 0 {
        if pic.is_empty() {
            return Err(ApiError::business("wap_01412"));
        }
        let zid = json_u64(body, "zph_id");
        if zid == 0 {
            return Err(ApiError::param_invalid("wap_com_00228"));
        }
        let did = zph_repo::find_admin_form(state.db.reader(), zid)
            .await?
            .map(|r| r.did)
            .unwrap_or(0);
        let nid = zph_repo::insert_pic(
            state.db.pool(),
            &title,
            &pic,
            json_i32(body, "sort"),
            zid,
            did,
        )
        .await?;
        if nid == 0 {
            return Err(ApiError::business("admin_system_00137"));
        }
        Ok(PhpOut::Message("ok"))
    } else {
        let n = zph_repo::update_pic(state.db.pool(), id, &title, &pic, json_i32(body, "sort")).await?;
        if n == 0 {
            return Err(ApiError::business("member_user_00603"));
        }
        Ok(PhpOut::Message("ok"))
    }
}

async fn fairs_setthemb(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if zph_repo::set_pic_themb(state.db.pool(), id).await? == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("wap_com_00240"))
}

async fn fairs_delpic(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "del");
    let id = if id == 0 { json_u64(body, "id") } else { id };
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if zph_repo::delete_pic(state.db.pool(), id).await? == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("ok"))
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
    if days <= 1 {
        return clock::start_of_today();
    }
    clock::now_ts() - i64::from(days) * 86_400
}

/// A `YYYY-MM-DD` filter bound from an admin form. Site-timezone midnight, so
/// picking one day covers that day in Beijing time rather than being 8 hours
/// off like a UTC parse would be.
fn parse_date_ts(s: &str) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    if let Ok(n) = s.parse::<i64>() {
        return n;
    }
    clock::parse_site_date(s).unwrap_or(0)
}

fn pic_url(base: &str, pic: &str) -> String {
    let pic = pic.trim();
    if pic.is_empty() || pic == "undefined" || pic == "null" {
        String::new()
    } else if pic.starts_with("http") {
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

/// PHP `special_special::savespecial_action` — admin add, skip self-signup checks.
async fn special_savespecial(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let sid = json_u64(body, "sid");
    let uid = json_u64(body, "uid");
    if sid == 0 || uid == 0 {
        return Err(ApiError::business("admin_01448"));
    }
    if special_repo::already_applied(state.db.reader(), sid, uid).await? {
        return Err(ApiError::business("admin_yunying_00015"));
    }
    special_repo::insert_admin_com(state.db.pool(), sid, uid, 1, clock::now_ts()).await?;
    Ok(PhpOut::Message("admin_model_00061"))
}

/// PHP `special.model::addSpecialMutiCom` — skip unapproved companies.
async fn special_muti_add_com(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let sid = json_u64(body, "sid");
    let uids = ids_named(body, "uid");
    if sid == 0 || uids.is_empty() {
        return Err(ApiError::business("wap_00556"));
    }
    let existing = special_repo::list_uids_by_sid(state.db.reader(), sid).await?;
    let companies = company_repo::list_by_uids(state.db.reader(), &uids).await?;
    let now = clock::now_ts();
    for uid in uids {
        if existing.iter().any(|u| *u == uid) {
            continue;
        }
        let ok = companies
            .iter()
            .any(|c| c.uid == uid && c.r_status == 1);
        if !ok {
            continue;
        }
        special_repo::insert_admin_com(state.db.pool(), sid, uid, 1, now).await?;
    }
    Ok(PhpOut::Message("api_wxapp_00013"))
}

fn csv_cell(s: &str) -> String {
    let t = s.replace('"', "\"\"");
    format!("\"{t}\"")
}

/// CSV export of special participants (no Excel/GD, no uploads write).
async fn special_comxls(state: &AppState, body: &Value) -> AppResult<Value> {
    let sid = json_u64(body, "zid").max(json_u64(body, "sid"));
    let ids = ids_named(body, "cid");
    let rows = gap_extra::php_list_special_coms_export(state.db.reader(), sid, &ids).await?;
    let mut csv = String::from("\u{FEFF}id,uid,name,status,time\n");
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            r.id,
            r.uid,
            csv_cell(&r.name),
            r.status,
            r.created_at
        ));
    }
    Ok(json!({
        "csv": csv,
        "filename": "special_com.csv",
        "total": rows.len() as u64,
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
    ad_service::invalidate_all();
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
    ad_service::invalidate_all();
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
    ad_service::invalidate_all();
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
    ad_service::invalidate_all();
    Ok(PhpOut::Message("ok"))
}

async fn ads_upsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("common_01716"));
    }
    ad_repo::set_sort(state.db.pool(), id, json_i32(body, "sort")).await?;
    ad_service::invalidate_all();
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
        "order_time_n": fmt_dt(r.order_time),
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
    let mut typezf = json_str(body, "typezf");
    if typezf.is_empty() {
        typezf = json_str(body, "order_type");
    }
    let order_id_kw = json_str(body, "keyword");
    let typeca = if body.get("typeca").is_some() {
        json_i32(body, "typeca")
    } else {
        json_i32(body, "type")
    };
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
    let mut time_min = if t0.is_empty() {
        None
    } else {
        Some(parse_date_ts(&t0))
    };
    let mut time_max = if t1.is_empty() {
        None
    } else {
        Some(parse_date_ts(&t1) + 86_400)
    };
    let time_days = json_i32(body, "time");
    if time_min.is_none() && time_days > 0 {
        time_min = Some(days_ago_ts(time_days));
    }
    let start1 = json_str(body, "time_start1");
    if !start1.is_empty() {
        time_min = Some(parse_date_ts(&start1));
    }
    let end1 = json_str(body, "time_end1");
    if !end1.is_empty() {
        time_max = Some(parse_date_ts(&end1) + 86_400);
    }
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
        sort: json_str(body, "t"),
        dir: json_str(body, "order"),
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
    sort: String,
    dir: String,
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
        sort: if q.sort.is_empty() {
            None
        } else {
            Some(q.sort.as_str())
        },
        dir: if q.dir.is_empty() {
            None
        } else {
            Some(q.dir.as_str())
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
    let pricename = setting_repo::find(db, "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    Ok(json!({
        "data": list,
        "list": list,
        "total": total,
        "pageSizes": [10, 20, 50, 100],
        "perPage": per,
        "page": page,
        "integral_pricename": pricename,
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
    let (htpics, preview_pics) = order_ht_pics(state, r.id, 0, 500).await?;
    let row = finance_order_json(&r);
    Ok(json!({
        "detail": row.clone(),
        "row": row,
        "htpics": htpics,
        "preview_pics": preview_pics,
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

async fn finance_order_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let joined = ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let ident = md5_hex(&format!("company_order{joined}"));
    let username = recycle_php::admin_username(state.db.pool(), user.uid)
        .await
        .unwrap_or_default();
    if let Err(e) = recycle_php::archive(
        state.db.pool(),
        "company_order",
        &ids,
        user.uid,
        &username,
        &ident,
        "/v1/admin/php-content/finance-order/delete",
    )
    .await
    {
        tracing::warn!(error = %e, "recycle snapshot skipped for company_order");
    }
    let n = vip_repo::php_delete_orders(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    let lang = i18n::current_lang();
    let msg = format!(
        "{}{}{}",
        i18n::t("messages.model_00021", lang),
        joined,
        i18n::t("messages.model_00112", lang),
    );
    Ok(PhpOut::Text("admin_user_00187", msg))
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

fn csv_arr(s: &str) -> Vec<String> {
    s.split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn rating_opt_json(id: i32, name: &str) -> Value {
    json!({ "id": id.to_string(), "name": name })
}

async fn order_ht_pics(
    state: &AppState,
    order_id: u64,
    offset: u64,
    limit: u64,
) -> AppResult<(Vec<Value>, Vec<String>)> {
    let cfg = settings_hash(state).await?;
    let rows = vip_repo::list_order_ht_pics(state.db.reader(), order_id, offset, limit).await?;
    let mut list = Vec::new();
    let mut pics = Vec::new();
    for r in rows {
        let pic_n = checkpic_url(&cfg, &r.picurl);
        if !pic_n.is_empty() {
            pics.push(pic_n.clone());
        }
        list.push(json!({
            "id": r.id,
            "order_id": r.order_id,
            "picurl": r.picurl,
            "pic_n": pic_n,
            "ctime": r.ctime,
        }));
    }
    Ok((list, pics))
}

async fn finance_order_upload(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let (page, per, offset, limit) = page_of(body);
    let row = match vip_repo::php_find_order(state.db.reader(), id).await? {
        Some(r) => finance_order_json(&r),
        None => json!({}),
    };
    let total = vip_repo::count_order_ht_pics(state.db.reader(), id).await?;
    let (list, pics) = order_ht_pics(state, id, offset, limit).await?;
    Ok(json!({
        "row": row,
        "list": list,
        "pics": pics,
        "total": total,
        "perPage": per,
        "page": page,
        "pageSizes": [10, 20, 50, 100],
    }))
}

fn finance_order_multiupload(body: &Value) -> Value {
    let url = json_str(body, "picurl");
    let url = if url.is_empty() { json_str(body, "url") } else { url };
    json!({ "error": 0, "picurl": url })
}

async fn finance_order_uploadsave(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let order_id = json_u64(body, "order_id");
    if order_id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let pics = json_str_list(body, "picurl");
    if pics.is_empty() {
        return Err(ApiError::business("admin_system_00137"));
    }
    let nid = vip_repo::insert_order_ht_pics(state.db.pool(), order_id, &pics, clock::now_ts()).await?;
    if nid == 0 {
        return Err(ApiError::business("admin_system_00137"));
    }
    let lang = i18n::current_lang();
    let id_s = nid.to_string();
    Ok(PhpOut::Text(
        "admin_model_00001",
        i18n::t_args("messages.admin_model_00001", lang, &[("id", &id_s)]),
    ))
}

async fn finance_order_htpic_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "delid");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = vip_repo::delete_order_ht_pic(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    let lang = i18n::current_lang();
    let id_s = id.to_string();
    Ok(PhpOut::Text(
        "admin_model_00002",
        i18n::t_args("messages.admin_model_00002", lang, &[("id", &id_s)]),
    ))
}

fn cfg_pick(cfg: &HashMap<String, String>, key: &str) -> String {
    cfg.get(key).cloned().unwrap_or_default()
}

fn strip_site_url(cfg: &HashMap<String, String>, url: &str) -> String {
    let mut s = url.trim().to_string();
    for k in ["sy_ossurl", "sy_weburl"] {
        let Some(raw) = cfg.get(k).map(|x| x.trim()).filter(|x| !x.is_empty()) else {
            continue;
        };
        let base = raw.trim_end_matches('/');
        if s.starts_with(base) {
            s = s[base.len()..].trim_start_matches('/').to_string();
        }
    }
    s
}

async fn comset_logo(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let cfg = settings_hash(state).await?;
    if body.get("submit").is_some() || body.get("sy_unit_icon").is_some() {
        for key in ["sy_unit_icon", "sy_guwen", "sy_banner", "sy_member_ewm"] {
            if body.get(key).is_some() {
                let val = strip_site_url(&cfg, &json_str(body, key));
                upsert_cfg(state, user, key, &val).await?;
            }
        }
        return Ok(PhpOut::Message("admin_user_00055"));
    }
    Ok(PhpOut::Data(json!({
        "config": {
            "sy_unit_icon": checkpic_url(&cfg, &cfg_pick(&cfg, "sy_unit_icon")),
            "sy_guwen": checkpic_url(&cfg, &cfg_pick(&cfg, "sy_guwen")),
            "sy_banner": checkpic_url(&cfg, &cfg_pick(&cfg, "sy_banner")),
            "sy_member_ewm": checkpic_url(&cfg, &cfg_pick(&cfg, "sy_member_ewm")),
        }
    })))
}

async fn comset_rating(state: &AppState, body: &Value) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let need = json_str(body, "need");
    let mut config = json!({});
    if need == "all" || need.is_empty() {
        config = json!({
            "integral_pricename": cfg_pick(&cfg, "integral_pricename"),
            "com_vip_type": cfg_pick(&cfg, "com_vip_type"),
            "com_integral_online": cfg_pick(&cfg, "com_integral_online"),
            "com_single_can": csv_arr(&cfg_pick(&cfg, "com_single_can")),
            "sy_only_price": csv_arr(&cfg_pick(&cfg, "sy_only_price")),
            "tg_back": cfg_pick(&cfg, "tg_back"),
            "rating_add": csv_arr(&cfg_pick(&cfg, "rating_add")),
            "job_ms_rating": csv_arr(&cfg_pick(&cfg, "job_ms_rating")),
            "com_package_open": csv_arr(&cfg_pick(&cfg, "com_package_open")),
            "com_rating": cfg_pick(&cfg, "com_rating"),
            "com_vip_done": cfg_pick(&cfg, "com_vip_done"),
        });
    }
    let rows = company_repo::list_rating_options(state.db.reader()).await?;
    let qy_rows: Vec<Value> = rows
        .iter()
        .map(|r| rating_opt_json(r.id, &r.name))
        .collect();
    let mut tc_package = qy_rows.clone();
    tc_package.push(json!({ "id": "999", "name": "admin_user_company_00297" }));
    Ok(json!({
        "config": config,
        "qy_rows": qy_rows,
        "tcPackage": tc_package,
    }))
}

async fn comset_comspend(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let mut data = Vec::new();
    for part in cfg_pick(&cfg, "integral_down_resume_dayprice").split(':') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let mut it = part.splitn(2, '_');
        let days = it.next().unwrap_or("").trim();
        let price = it.next().unwrap_or("").trim();
        if days.is_empty() {
            continue;
        }
        data.push(json!({ "days": days, "price": price }));
    }
    Ok(json!({
        "config": {
            "integral_pricename": cfg_pick(&cfg, "integral_pricename"),
            "integral_proportion": cfg_pick(&cfg, "integral_proportion"),
            "integral_priceunit": cfg_pick(&cfg, "integral_priceunit"),
            "integral_job": cfg_pick(&cfg, "integral_job"),
            "integral_down_resume": cfg_pick(&cfg, "integral_down_resume"),
            "integral_interview": cfg_pick(&cfg, "integral_interview"),
            "integral_jobefresh": cfg_pick(&cfg, "integral_jobefresh"),
            "com_urgent": cfg_pick(&cfg, "com_urgent"),
            "integral_job_top": cfg_pick(&cfg, "integral_job_top"),
            "com_recjob": cfg_pick(&cfg, "com_recjob"),
            "job_auto": cfg_pick(&cfg, "job_auto"),
        },
        "data": data,
    }))
}

async fn userset_userspend(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    Ok(json!({
        "config": {
            "integral_resume_top": cfg_pick(&cfg, "integral_resume_top"),
            "pay_trust_resume": cfg_pick(&cfg, "pay_trust_resume"),
        }
    }))
}

async fn rating_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let name = json_str(body, "name");
    if id == 0 || name.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = gap_extra::count_rating_service_name(state.db.reader(), &name, id).await?;
    if n > 0 {
        return Err(ApiError::business("wap_js_00054"));
    }
    gap_extra::rename_rating_service(state.db.pool(), id, &name).await?;
    let lang = i18n::current_lang();
    let id_s = id.to_string();
    Ok(PhpOut::Text(
        "admin_model_00161",
        i18n::t_args("messages.admin_model_00161", lang, &[("id", &id_s)]),
    ))
}

async fn rating_zz_data(state: &AppState) -> AppResult<Value> {
    let list = gap_extra::list_rating_services(state.db.reader()).await?;
    let zzlist: Vec<Value> = list
        .into_iter()
        .map(|s| json!({ "id": s.id, "name": s.name, "display": s.display, "sort": s.sort }))
        .collect();
    Ok(json!({ "config": {}, "zzlist": zzlist }))
}

async fn rating_edittc(state: &AppState, body: &Value) -> AppResult<Value> {
    let tid = json_u64(body, "tid");
    if tid == 0 {
        return Ok(json!({}));
    }
    match gap_extra::find_rating_detail(state.db.reader(), tid).await? {
        Some(r) => Ok(serde_json::to_value(r).unwrap_or(json!({}))),
        None => Ok(json!({})),
    }
}

async fn rating_del_detail(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = gap_extra::delete_rating_details(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
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
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
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
        sort: &sort,
        dir: &dir,
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
                "pay_time_n": fmt_dt(r.pay_time),
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
        "list": list,
        "total": total,
        "pageSizes": [10, 20, 50, 100],
        "perPage": per,
        "page": page,
        "integral_pricename": pricename,
    }))
}

async fn finance_pay_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01164"));
    }
    let n = pay_repo::php_delete_pay(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    let lang = i18n::current_lang();
    let joined = ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let msg = format!(
        "{}{}{}",
        i18n::t("messages.model_00216", lang),
        joined,
        i18n::t("messages.model_00112", lang),
    );
    Ok(PhpOut::Text("admin_user_00187", msg))
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
    let cfg = settings_hash(state).await?;
    let pic = media_path_of(&cfg, &json_str(body, "pic"));
    let yyzz = media_path_of(&cfg, &json_str(body, "yyzz"));
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
            pic: &pic,
            yyzz: &yyzz,
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
    if has_flag(body, "mqlogo") && json_str(body, "hot_pic").is_empty() {
        return Err(ApiError::business("admin_system_00137"));
    }
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let existing = company_repo::hotjob_find_by_uid(state.db.reader(), uid).await?;
    let cfg = settings_hash(state).await?;
    let mut hot_pic = media_path_of(&cfg, &json_str(body, "hot_pic"));
    if hot_pic.is_empty() {
        hot_pic = media_path_of(&cfg, &json_str(body, "hot_pic_n"));
    }
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
        v["hasChildren"] = json!(false);
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

async fn cat_rows_json(
    state: &AppState,
    kind: &str,
    rows: &[cat_repo::CatPhpRow],
    level: i32,
) -> AppResult<Vec<Value>> {
    let has = if kind == "city" && !rows.is_empty() {
        let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        cat_repo::ids_with_children(state.db.reader(), kind, &ids).await?
    } else {
        HashSet::new()
    };
    Ok(rows
        .iter()
        .map(|r| {
            let mut v = cat_row_json(kind, r, level);
            if kind == "city" {
                v["hasChildren"] = json!(has.contains(&r.id));
            }
            v
        })
        .collect())
}

async fn cat_class_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let rows = cat_repo::list_php(state.db.reader(), &kind, None).await?;
    Ok(Value::Array(cat_rows_json(state, &kind, &rows, 1).await?))
}

async fn cat_class_children(state: &AppState, body: &Value) -> AppResult<Value> {
    let kind = cat_kind(body);
    let keyid = json_u64(body, "keyid");
    if keyid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let level = json_i32(body, "level").max(1);
    let rows = cat_repo::list_php(state.db.reader(), &kind, Some(keyid)).await?;
    let list = cat_rows_json(state, &kind, &rows, level).await?;
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
        dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
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
    dict_service::reload(state).await?;
    Ok(PhpOut::Message("admin_system_00002"))
}

async fn cat_class_clearpinyin(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let kind = cat_kind(body);
    if kind == "job" {
        cat_repo::job_clear_pinyin(state.db.pool()).await?;
        return Ok(PhpOut::Message("admin_01370"));
    }
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
    let (tpl_n, tpl_temp) = tpl_meta_of(&name);
    Ok(json!({
        "info": row.map(|r| json!({
            "name": r.code,
            "title": r.title,
            "content": r.content,
        })).unwrap_or(json!({})),
        "tpl_temp": tpl_temp,
        "tpl_n": tpl_n,
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
    Ok(PhpOut::Message("admin_01462"))
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
        content_like_any: &[],
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


/// PHP `users_member::writtenOffLog` narrows opera 12 to these three keys, so
/// the unbind tab does not show every account-security log in the bucket.
const WRITTENOFF_CONTENT_KEYS: &[&str] = &["wap_user_00138", "wap_js_00065", "common_02028"];
const WRITTENOFF_OPERA: i32 = 12;

/// PHP `users_member::writtenOffLog_action` — 解绑日志.
async fn user_gap_writtenoff_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (_, _, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let kw_type = json_i32(body, "type");
    let mut username_like: Option<String> = None;
    let mut content_like: Option<String> = None;
    if !kw.is_empty() {
        match kw_type {
            1 => username_like = Some(kw.clone()),
            2 => content_like = Some(kw.clone()),
            _ => {}
        }
    }
    // PHP: strtotime(time_start) .. strtotime(time_end . ' 23:59:59').
    let t0 = Some(parse_date_ts(&json_str(body, "time_start"))).filter(|n| *n > 0);
    let t1 = Some(parse_date_ts(&json_str(body, "time_end")))
        .filter(|n| *n > 0)
        .map(|n| n + 86_399);
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gap_repo::PhpMemberLogFilter {
        usertype: user_gap_log_usertype(body),
        uid: None,
        username_like: username_like.as_deref(),
        content_like: content_like.as_deref(),
        content_like_any: if content_like.is_some() {
            &[]
        } else {
            WRITTENOFF_CONTENT_KEYS
        },
        opera: Some(WRITTENOFF_OPERA),
        log_type: None,
        time_from: t0,
        time_to: t1,
        order_t: order_t.as_str(),
        order_dir: order_dir.as_str(),
    };
    let db = state.db.reader();
    let total = gap_repo::count_php_member_logs(db, &f).await?;
    let rows = if total > 0 {
        gap_repo::list_php_member_logs(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let (page, per, _, _) = page_of(body);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "opera": r.opera,
                "usertype": r.usertype,
                "content": r.content,
                "ip": r.ip,
                "ctime": r.ctime,
                "ctime_n": fmt_ts(r.ctime, "%Y-%m-%d %H:%M:%S"),
                "username": r.username,
                "rname": r.rname,
                "sub_n": r.sub_n,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `users_member::delwflog_action`. `del=all` clears only opera 12, and only
/// for the `utype` the tab is showing — PHP hardcodes `usertype = 1` here even in
/// the company copy, which would let the company tab wipe personal unbind logs.
async fn user_gap_writtenoff_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let db = state.db.pool();
    if json_str(body, "del").trim() == "all" {
        let ut = user_gap_log_usertype(body);
        gap_repo::delete_php_member_logs_by_usertype_opera(db, ut, WRITTENOFF_OPERA).await?;
        return Ok(PhpOut::Message("admin_01290"));
    }
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    gap_repo::delete_php_member_logs(db, &ids).await?;
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `job.model::subSqListInfo` isdel text for `userid_job`.
fn apply_isdel_text(isdel: i32) -> String {
    let lang = i18n::current_lang();
    i18n::t(
        match isdel {
            1 => "messages.common_06284",
            2 => "messages.common_06285",
            3 => "messages.common_01522",
            _ => "messages.admin_user_00149",
        },
        lang,
    )
}

/// PHP `job.model::subYqmsListInfo` isdel text for `userid_msg`.
fn invite_isdel_text(isdel: i32) -> String {
    let lang = i18n::current_lang();
    i18n::t(
        match isdel {
            1 => "messages.common_06294",
            2 => "messages.common_06285",
            _ => "messages.admin_user_00149",
        },
        lang,
    )
}

/// PHP `users_member::jobSqLog_action` — 职位申请记录.
async fn user_gap_apply_log(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let db = state.db.reader();
    let total = gap_extra::count_member_applies(db, uid).await?;
    let rows = if total > 0 {
        gap_extra::list_member_applies(db, uid, offset, limit).await?
    } else {
        Vec::new()
    };
    let base = preview_base(state);
    let base = base.trim_end_matches('/');
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "job_name": r.job_name,
                "com_name": r.com_name,
                "job_comapply": format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.job_id),
                "company_show": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.com_id),
                "datetime": r.datetime,
                "datetime_n_n": fmt_dt(r.datetime),
                "is_browse": r.is_browse,
                "isdel_n": apply_isdel_text(r.isdel),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `users_member::yqmsLog_action` — 面试邀请记录.
async fn user_gap_invite_log(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let db = state.db.reader();
    let total = gap_extra::count_member_invites(db, uid).await?;
    let rows = if total > 0 {
        gap_extra::list_member_invites(db, uid, offset, limit).await?
    } else {
        Vec::new()
    };
    let base = preview_base(state);
    let base = base.trim_end_matches('/');
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "fname": r.fname,
                "jobname": r.jobname,
                "title": r.title,
                "content": r.content,
                "job_comapply": format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.jobid),
                "company_show": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.fid),
                "datetime": r.datetime,
                "datetime_n": fmt_dt(r.datetime),
                "is_browse": r.is_browse,
                "isdel_n": invite_isdel_text(r.isdel),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `db.data.php` `paystate` after `strip_tags`.
fn pay_state_text(state: i32) -> &'static str {
    match state {
        0 => "支付失败",
        1 => "等待付款",
        2 => "支付成功",
        3 => "等待确认",
        4 => "交易关闭",
        _ => "",
    }
}

/// PHP `users_member::payLog_action` — 消费记录（`company_pay.com_id = uid`）.
async fn user_gap_pay_log(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let f = pay_repo::PhpPayFilter {
        com_id: Some(uid),
        usertype: None,
        order_id_kw: None,
        remark_kw: None,
        uid_in: None,
        pay_state: None,
        time_min: None,
        sort: "id",
        dir: "desc",
    };
    let db = state.db.reader();
    let total = pay_repo::php_count_pay(db, &f).await?;
    let rows = if total > 0 {
        pay_repo::php_list_pay(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    // PHP `getPayList` default branch: integral orders carry the点数 unit, the rest 元.
    let unit = setting_repo::find(db, "integral_priceunit")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let pricename = setting_repo::find(db, "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let price = r.order_price.replace(".00", "");
            let tag = if r.r#type == 1 {
                format!("{unit}{pricename}")
            } else {
                "元".to_string()
            };
            json!({
                "id": r.id,
                "order_id": r.order_id,
                "consume_id": r.order_id,
                "consume_price_n": format!("{price}{tag}"),
                "consume_remark": r.pay_remark,
                "consume_state_n": pay_state_text(r.pay_state),
                "pay_time_n": fmt_ts(r.pay_time, "%Y-%m-%d %H:%M:%S"),
                "consume_time_n": fmt_ts(r.pay_time, "%Y-%m-%d %H:%M:%S"),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP `company_job::depower_action` — 职位降权 / 取消降权.
async fn company_job_depower(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let is_depower = json_i32(body, "is_depower");
    if id == 0 || is_depower == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let lang = i18n::current_lang();
    let verb = i18n::t(
        if is_depower == 1 {
            "messages.admin_user_company_00374"
        } else {
            "messages.admin_user_company_00367"
        },
        lang,
    );
    let id_s = id.to_string();
    if job_repo::admin_set_depower(state.db.pool(), id, is_depower).await? == 0 {
        return Err(ApiError::business("admin_model_00132"));
    }
    Ok(PhpOut::Text(
        "admin_model_00131",
        i18n::t_args(
            "messages.admin_model_00131",
            lang,
            &[("action", &verb), ("id", &id_s)],
        ),
    ))
}

/// PHP `company_job::setlinkopen_action` — 外链投递开关.
async fn company_job_setlinkopen(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "linkjobid");
    let linkopen = json_i32(body, "linkopen");
    if id == 0 || linkopen == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    if job_repo::admin_set_linkopen(state.db.pool(), id, linkopen).await? == 0 {
        return Err(ApiError::business("wap_01715"));
    }
    Ok(PhpOut::Message("model_00011"))
}

/// PHP `company_job::upjobhits_action` — 浏览量 / 曝光量改绝对值.
async fn company_job_upjobhits(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "pid");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_01298"));
    }
    let hits = json_i64(body, "jobhits");
    let expoure = json_i64(body, "jobexpoure");
    if job_repo::admin_set_hits(state.db.pool(), id, hits, expoure).await? == 0 {
        return Err(ApiError::business("member_user_00603"));
    }
    Ok(PhpOut::Message("member_user_00602"))
}

fn php_audit_next_job(next: Option<u64>, single: bool, atype: i32) -> PhpOut {
    if single && atype != 1 {
        if let Some(id) = next {
            return PhpOut::Data(json!({ "job": { "id": id } }));
        }
    }
    PhpOut::Message("common_01944")
}

fn php_audit_next_resume(next: Option<u64>, single: bool, atype: i32) -> PhpOut {
    if single && atype != 1 {
        if let Some(id) = next {
            return PhpOut::Data(json!({ "next_id": id }));
        }
    }
    PhpOut::Message("common_01944")
}

/// PHP `company_job::status_action` — `pid` CSV + `statusbody` + optional next job.
async fn company_job_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "pid");
    let ids = if ids.is_empty() { ids_of(body) } else { ids };
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    if status == 0 {
        return Err(ApiError::param_invalid("admin_01311"));
    }
    let n = gap_extra::php_status_jobs(
        state.db.pool(),
        &ids,
        status,
        &json_str(body, "statusbody"),
        json_i32(body, "lock_status"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_job(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_job(next, single, json_i32(body, "atype")))
}

/// PHP `company_job::cjobstatus_action` — also reinstates the company on pass.
async fn company_job_cjobstatus(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "pid");
    let uid = json_u64(body, "uid");
    let status = json_i32(body, "status");
    if id == 0 || uid == 0 || status == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = gap_extra::php_cjobstatus(
        state.db.pool(),
        id,
        uid,
        status,
        &json_str(body, "statusbody"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    if status == 1 {
        let _ = user_repo::admin_set_status(state.db.pool(), uid, 1).await?;
        let _ = gap_extra::reinstate_company(state.db.pool(), uid).await?;
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_job(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_job(next, single, json_i32(body, "atype")))
}

/// PHP `company_job::jobAudit_action`.
async fn company_job_audit(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let db = state.db.reader();
    let row = gap_extra::php_job_audit(db, id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let mut tel = row.linktel.clone();
    let mut phone = row.linkphone.clone();
    let mut address = row.address.clone();
    let mut linkman = row.linkman.clone();
    if row.is_link == 2 {
        if let Some(link) = gap_extra::php_job_link(db, row.link_id).await? {
            tel = link.link_moblie;
            phone = link.link_phone;
            address = link.link_address;
            linkman = link.link_man;
        }
    }
    let crm_salesman = gap_extra::php_admin_user_name(db, row.crm_uid).await?;
    let snum = gap_extra::php_job_pending_except(db, id).await?;
    Ok(json!({
        "info": {
            "id": row.id,
            "uid": row.uid,
            "name": row.name,
            "jobname": row.name,
            "state": row.state,
            "status": row.status,
            "r_status": row.r_status,
            "statusbody": row.lock_info.trim(),
            "c_status": row.c_status,
            "reg_date_n": fmt_ts(row.reg_date, "%Y-%m-%d %H:%M:%S"),
            "login_date_n": if row.login_date > 0 {
                fmt_ts(row.login_date, "%Y-%m-%d %H:%M:%S")
            } else {
                String::new()
            },
            "tel": tel,
            "phone": phone,
            "address": address,
            "linkman": linkman,
            "linkmail": row.linkmail,
            "crm_salesman": crm_salesman,
            "rating_name": row.rating_name,
        },
        "snum": snum,
    }))
}

/// PHP `job.model::applyJobByAdmin` core insert (no email/sms/weixin).
async fn company_job_apply(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let eid = json_u64(body, "eid");
    let uid = json_u64(body, "uid");
    let job_id = json_u64(body, "id").max(json_u64(body, "job_id"));
    let com_id = json_u64(body, "comid").max(json_u64(body, "com_id"));
    if eid == 0 || job_id == 0 {
        return Err(ApiError::business("common_01170"));
    }
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| com_id == 0 || j.uid == com_id);
    let Some(job) = job else {
        return Err(ApiError::business("common_01538"));
    };
    if gap_extra::php_userid_job_exists(state.db.reader(), eid, uid, job.id, job.uid).await? {
        return Err(ApiError::business("common_01538"));
    }
    let nid = apply_repo::create(
        state.db.pool(),
        apply_repo::ApplyCreate {
            uid,
            job_id: job.id,
            job_name: &job.name,
            apply_url: "",
            com_id: job.uid,
            com_name: job.com_name.as_deref().unwrap_or(""),
            eid,
            now: clock::now_ts(),
            is_browse: 1,
        },
    )
    .await?;
    if nid == 0 {
        return Err(ApiError::business("common_01394"));
    }
    let _ = gap_extra::php_inc_company_sq_job(state.db.pool(), job.uid).await;
    let _ = gap_extra::php_inc_member_sq_jobnum(state.db.pool(), uid).await;
    Ok(PhpOut::Message("model_00010"))
}

/// PHP `users_resume::status_action`.
async fn resume_php_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "id");
    if ids.is_empty() {
        ids = ids_of(body);
    }
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    if status == 0 {
        return Err(ApiError::param_invalid("admin_01311"));
    }
    let content = json_str(body, "content");
    if !content.is_empty() {
        let _ = resume_label(state, body).await;
    }
    let n = gap_extra::php_status_resumes(
        state.db.pool(),
        &ids,
        status,
        &json_str(body, "statusbody"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_resume(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_resume(next, single, json_i32(body, "atype")))
}

/// PHP `users_resume::resumestatus_action` — unlock user then review.
async fn resume_php_resumestatus(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let uid = json_u64(body, "uid");
    let status = json_i32(body, "status");
    if id == 0 || uid == 0 || status == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let lock_status = json_i32(body, "lock_status");
    if lock_status == 1 {
        user_repo::update_lock(state.db.pool(), uid, 1, "").await?;
        user_repo::lock_related_r_status(state.db.pool(), uid, 1).await?;
    } else if lock_status != 0 && lock_status != 1 {
        return Err(ApiError::param_invalid("wap_01298"));
    }
    let content = json_str(body, "content");
    if !content.is_empty() {
        let _ = resume_label(state, body).await;
    }
    let n = gap_extra::php_resume_status_one(
        state.db.pool(),
        id,
        uid,
        status,
        &json_str(body, "statusbody"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_resume(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_resume(next, single, json_i32(body, "atype")))
}

/// PHP `partjob::status_action`.
async fn part_php_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "pid");
    let ids = if ids.is_empty() { ids_of(body) } else { ids };
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    if status == 0 {
        return Err(ApiError::param_invalid("admin_01311"));
    }
    let n = gap_extra::php_status_parts(
        state.db.pool(),
        &ids,
        status,
        &json_str(body, "statusbody"),
        json_i32(body, "lock_status"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_part(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_job(next, single, json_i32(body, "atype")))
}

/// PHP `partjob::tbStatus_action`.
async fn part_php_tb_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "pid");
    let uid = json_u64(body, "uid");
    let status = json_i32(body, "status");
    if id == 0 || uid == 0 || status == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = gap_extra::php_tb_status_part(
        state.db.pool(),
        id,
        uid,
        status,
        &json_str(body, "statusbody"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("model_00115"));
    }
    if status == 1 {
        let _ = user_repo::admin_set_status(state.db.pool(), uid, 1).await?;
        let _ = gap_extra::reinstate_company(state.db.pool(), uid).await?;
    }
    let single = has_flag(body, "single");
    let next = if single && json_i32(body, "atype") != 1 {
        gap_extra::php_next_pending_part(state.db.reader()).await?
    } else {
        None
    };
    Ok(php_audit_next_job(next, single, json_i32(body, "atype")))
}

const INTEGRAL_INDEX_KEYS: &[&str] = &[
    "integral_pricename",
    "integral_priceunit",
    "integral_min_recharge",
    "money_min_recharge",
    "paypack_max_recharge",
    "packprice_min_recharge",
    "integral_proportion",
    "integral_signin",
    "integral_reg",
    "integral_login",
    "integral_userinfo",
    "integral_emailcert",
    "integral_mobliecert",
    "integral_avatar",
    "integral_question",
    "integral_answer",
    "integral_answerpl",
    "integral_invite_reg",
    "integral_bind_wx",
    "integral_add_resume",
    "integral_identity",
    "integral_comcert",
    "integral_banner",
    "integral_map",
    "integral_ltcert",
    "integral_px_banner",
];

async fn company_cert_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let uids = if kw.trim().is_empty() {
        None
    } else {
        Some(gap_extra::php_search_company_uids(state.db.reader(), &kw).await?)
    };
    if uids.as_ref().is_some_and(|v| v.is_empty()) {
        return Ok(paged(json!([]), 0, page, per));
    }
    let status_raw = json_str(body, "status");
    let status = if status_raw.trim().is_empty() {
        None
    } else {
        let s = json_i32(body, "status");
        Some(if s == 3 { 0 } else { s })
    };
    let end = json_i32(body, "end");
    let ctime_min = match end {
        0 => None,
        1 => Some(clock::start_of_today()),
        n => Some(clock::now_ts() - i64::from(n) * 86_400),
    };
    let (rows, total) = gap_extra::php_cert_list(
        state.db.reader(),
        gap_extra::PhpCertListFilter {
            status,
            uids: uids.as_deref(),
            ctime_min,
            order_col: &json_str(body, "t"),
            order_dir: &json_str(body, "order"),
        },
        offset,
        limit,
    )
    .await?;
    let cfg = setting_repo::find_many(state.db.reader(), &["sy_ossurl", "sy_weburl"]).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "status": r.status,
                "check": checkpic_url(&cfg, &r.check),
                "owner_cert": checkpic_url(&cfg, &r.owner_cert),
                "wt_cert": checkpic_url(&cfg, &r.wt_cert),
                "other_cert": checkpic_url(&cfg, &r.other_cert),
                "social_credit": r.social_credit,
                "statusbody": r.statusbody,
                "ctime": r.ctime,
                "ctime_n": fmt_ts(r.ctime, "%Y-%m-%d %H:%M"),
                "name": r.name,
                "logo": checkpic_url(&cfg, &r.logo),
            })
        })
        .collect();
    Ok(paged(json!(list), total, page, per))
}

async fn company_cert_statist(state: &AppState) -> AppResult<Value> {
    let s = gap_extra::com_cert_stat(state.db.reader()).await?;
    Ok(json!({
        "comCertAll": s.com_cert_all,
        "comCert1": s.com_cert1,
        "comCert2": s.com_cert2,
    }))
}

async fn company_cert_sbody(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let body_txt = gap_extra::com_cert_statusbody(state.db.reader(), uid).await?;
    Ok(Value::String(body_txt.trim().to_string()))
}

async fn company_cert_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let status_raw = json_str(body, "status");
    if status_raw.trim().is_empty() {
        return Err(ApiError::param_invalid("admin_01311"));
    }
    let status = json_i32(body, "status");
    let mut uids = ids_named(body, "uid");
    if uids.is_empty() {
        uids = ids_of(body);
    }
    if uids.is_empty() {
        return Err(ApiError::param_invalid("model_00001"));
    }
    let name = json_str(body, "name");
    if !name.is_empty() && uids.len() == 1 {
        if gap_extra::php_cert_name_taken(state.db.reader(), &name, uids[0]).await? {
            return Err(ApiError::business("admin_user_00021"));
        }
    }
    let n = gap_extra::php_cert_review(
        state.db.pool(),
        &uids,
        status,
        &json_str(body, "statusbody"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("wap_01715"));
    }
    let yyzz = if status == 1 { 1 } else { 2 };
    let free = cfg_of(state, "com_free_status").await;
    let job_status = json_i32(body, "job_status") == 1 || json_str(body, "job_status") == "1";
    if free == "1" && job_status && yyzz == 1 {
        let _ = gap_extra::php_jobs_approve_pending_for_uids(state.db.pool(), &uids).await?;
        let _ = gap_extra::php_company_set_r_status_uids(state.db.pool(), &uids, 1).await?;
    }
    for uid in &uids {
        let nm = if uids.len() == 1 && !name.is_empty() {
            Some(name.as_str())
        } else {
            None
        };
        let _ = company_repo::set_yyzz(state.db.pool(), *uid, yyzz, nm).await?;
    }
    let _ = gap_extra::php_jobs_set_yyzz(state.db.pool(), &uids, yyzz).await?;
    Ok(PhpOut::Message("admin_model_00159"))
}

async fn set_integral_index(state: &AppState) -> AppResult<Value> {
    let map = setting_repo::find_many(state.db.reader(), INTEGRAL_INDEX_KEYS).await?;
    let mut out = serde_json::Map::new();
    for k in INTEGRAL_INDEX_KEYS {
        out.insert((*k).into(), json!(map.get(*k).cloned().unwrap_or_default()));
    }
    Ok(Value::Object(out))
}

async fn set_integral_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let obj = body.as_object().ok_or_else(|| ApiError::param_invalid("wap_com_00228"))?;
    let skip = ["pytoken", "m", "c", "a", "waterconfig"];
    for (k, v) in obj {
        if skip.contains(&k.as_str()) || k.is_empty() {
            continue;
        }
        let val = match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => if *b { "1".into() } else { "0".into() },
            Value::Array(a) => a
                .iter()
                .filter_map(|x| x.as_str().map(str::to_string).or_else(|| x.as_i64().map(|n| n.to_string())))
                .collect::<Vec<_>>()
                .join(","),
            Value::Null => continue,
            _ => continue,
        };
        upsert_cfg(state, user, k, &val).await?;
    }
    Ok(PhpOut::Message("admin_01384"))
}

async fn set_integral_class(state: &AppState) -> AppResult<Value> {
    let rows = gap_extra::php_intclass_list(state.db.reader()).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "integral": r.integral,
                "discount": r.discount,
                "state": r.state,
                "status": r.state == 1,
                "isEditjifen": false,
                "isEditdiscount": false,
            })
        })
        .collect();
    Ok(json!(list))
}

async fn set_integral_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("wap_js_00141"));
    }
    let mut n = 0u64;
    let typ = json_str(body, "type");
    if !typ.is_empty() {
        let rec = json_i32(body, "rec");
        let flag = if rec == 1 { 1 } else { 2 };
        n = gap_extra::php_intclass_set_field(state.db.pool(), id, &typ, flag).await?;
    }
    if body.get("integral").is_some() {
        let integral = json_i32(body, "integral");
        if gap_extra::php_intclass_integral_taken(state.db.reader(), integral, id).await? {
            return Err(ApiError::business("wap_js_00141"));
        }
        n = gap_extra::php_intclass_set_field(state.db.pool(), id, "integral", integral).await?;
    }
    if body.get("discount").is_some() {
        n = gap_extra::php_intclass_set_field(state.db.pool(), id, "discount", json_i32(body, "discount")).await?;
    }
    if n == 0 {
        return Err(ApiError::business("wap_js_00141"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

async fn set_integral_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "delid");
    ids.extend(ids_named(body, "del"));
    ids.extend(ids_of(body));
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let n = gap_extra::php_intclass_del(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_model_00206"))
}

/// PHP `set_config::save_logo_action` — logo already written by upload.
fn set_config_save_logo(body: &Value) -> AppResult<PhpOut> {
    if !has_flag(body, "waterconfig") {
        return Err(ApiError::business("common_01237"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

/// Keys from PHP `uploads/config/db.data.php` `modelconfig` (read-only copy).
const TPL_CACHE_MODELS: &[(&str, &str)] = &[
    ("job", "找工作"),
    ("resume", "找人才"),
    ("part", "兼职"),
    ("company", "找企业"),
    ("wap", "手机端"),
    ("article", "资讯"),
    ("announcement", "公告"),
    ("hr", "工具箱"),
    ("zph", "招聘会"),
    ("ask", "问答"),
    ("evaluate", "测评"),
    ("once", "店铺招聘"),
    ("tiny", "普工简历"),
    ("redeem", "商城"),
    ("map", "地图"),
    ("special", "专题招聘"),
    ("login", "登录"),
    ("register", "注册"),
    ("gongzhao", "公招"),
    ("error", "错误提醒"),
];

fn tpl_cache_key(model: &str) -> String {
    format!("sy_{model}_cache")
}

async fn set_config_settplcache(state: &AppState) -> AppResult<Value> {
    let mut keys: Vec<&str> = vec!["sy_index_cache"];
    let owned: Vec<String> = TPL_CACHE_MODELS.iter().map(|(k, _)| tpl_cache_key(k)).collect();
    keys.extend(owned.iter().map(|s| s.as_str()));
    let map = setting_repo::find_many(state.db.reader(), &keys).await?;
    let mut new_model = serde_json::Map::new();
    for (k, label) in TPL_CACHE_MODELS {
        let cache = map.get(&tpl_cache_key(k)).cloned().unwrap_or_default();
        new_model.insert(
            (*k).to_string(),
            json!({ "value": *label, "cache": cache }),
        );
    }
    Ok(json!({
        "newModel": Value::Object(new_model),
        "sy_index_cache": map.get("sy_index_cache").cloned().unwrap_or_default(),
    }))
}

async fn set_config_savetplcache(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let mut allowed = vec!["sy_index_cache".to_string()];
    allowed.extend(TPL_CACHE_MODELS.iter().map(|(k, _)| tpl_cache_key(k)));
    let obj = match body {
        Value::Object(m) => m,
        _ => return Err(ApiError::param_invalid("wap_com_00228")),
    };
    for (k, v) in obj {
        if !allowed.iter().any(|a| a == k) {
            continue;
        }
        let val = match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => (if *b { "1" } else { "0" }).into(),
            _ => continue,
        };
        upsert_cfg(state, user, k, &val).await?;
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

async fn index_mapconfig(state: &AppState) -> AppResult<Value> {
    let keys = [
        "map_x",
        "map_y",
        "map_rating",
        "map_control",
        "map_control_anchor",
        "map_control_type",
        "map_control_xb",
        "map_control_scale",
    ];
    let map = setting_repo::find_many(state.db.reader(), &keys).await?;
    let mut out = serde_json::Map::new();
    for k in keys {
        out.insert(k.into(), json!(map.get(k).cloned().unwrap_or_default()));
    }
    Ok(Value::Object(out))
}

async fn cache_get_price_name(state: &AppState) -> AppResult<Value> {
    Ok(json!({
        "integral_pricename": cfg_of(state, "integral_pricename").await,
    }))
}

/// PHP `job.model::subReserveJob` renders the daily window as `s - e`, filling
/// in the open end, and says "不限" when neither bound is set.
fn reserve_window_text(s_time: &str, e_time: &str) -> String {
    match (s_time.is_empty(), e_time.is_empty()) {
        (false, false) => format!("{s_time} - {e_time}"),
        (false, true) => format!("{s_time} - 24:00"),
        (true, false) => format!("00:00 - {e_time}"),
        (true, true) => i18n::t("messages.common_01936", i18n::current_lang()),
    }
}

/// PHP `company_job::reserveJob_action` — 预约刷新列表.
async fn company_job_reserve_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let keyword = json_str(body, "keyword");
    let order_col = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gap_extra::ReserveJobFilter {
        keyword: if keyword.trim().is_empty() {
            None
        } else {
            Some(keyword.as_str())
        },
        keyword_type: json_i32(body, "type"),
        uid: Some(json_u64(body, "uid")).filter(|v| *v > 0),
        order_col: order_col.as_str(),
        order_desc: !order_dir.eq_ignore_ascii_case("asc"),
    };
    let db = state.db.reader();
    let total = gap_extra::count_reserve_jobs(db, &f).await?;
    let rows = if total > 0 {
        gap_extra::list_reserve_jobs(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let base = preview_base(state);
    let base = base.trim_end_matches('/');
    let unlimited = i18n::t("messages.common_01936", i18n::current_lang());
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "name": r.name,
                "com_name": r.com_name,
                "joburl": format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.id),
                "comurl": format!("{base}/index.php?m=company&c=show&id={}&look=admin", r.uid),
                "reserve_status": r.reserve_status.to_string(),
                "reserve_interval": r.reserve_interval.to_string(),
                "reserve_start": fmt_ts(r.start_time, "%Y-%m-%d %H:%M:%S"),
                "reserve_end": if r.end_time > 0 { fmt_ts(r.end_time, "%Y-%m-%d") } else { unlimited.clone() },
                "s_time": r.s_time,
                "e_time": r.e_time,
                "sx_time_n": reserve_window_text(&r.s_time, &r.e_time),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

/// PHP member-side `reserveInfo`, called by the admin dialog to pre-fill a job
/// that is not queued yet. `refreshStatus` 0 means "never scheduled".
async fn company_job_get_refresh(state: &AppState, body: &Value) -> AppResult<Value> {
    let job_id = json_u64(body, "job_id");
    if job_id == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let row = gap_extra::find_reserve_schedule(state.db.reader(), job_id).await?;
    Ok(match row {
        Some(r) => json!({
            "refreshStatus": r.status,
            "interval": r.interval,
            "s_time": r.s_time,
            "e_time": r.e_time,
            "end_time": r.end_time,
        }),
        None => json!({
            "refreshStatus": 0,
            "interval": 0,
            "s_time": "",
            "e_time": "",
            "end_time": 0,
        }),
    })
}

/// PHP `company_job::closeReserve_action` — 关闭选中职位的预约刷新.
async fn company_job_close_reserve(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "ids");
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    gap_extra::close_reserve_jobs(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("model_00009"))
}

/// PHP `company_job::ajaxCloseReserve_action` — 页面挂载时的清扫，无入参.
async fn company_job_close_stale_reserve(state: &AppState) -> AppResult<PhpOut> {
    gap_extra::close_stale_reserve_jobs(state.db.pool()).await?;
    Ok(PhpOut::Message("model_00009"))
}

/// PHP `company_job::upReserveJob_action` → `job.model::reserveUpJob`.
async fn company_job_up_reserve(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let job_ids = ids_named(body, "job_id");
    if uid == 0 || job_ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    crate::job_mgmt_service::up_reserve(
        state,
        uid,
        &job_ids,
        status,
        &json_str(body, "end_time"),
        json_i32(body, "interval"),
        &json_str(body, "s_time"),
        &json_str(body, "e_time"),
    )
    .await?;
    Ok(PhpOut::Message("common_01047"))
}

/// PHP `users_member::log_action` maps each `operas` bucket to the content
/// keywords its log lines carry, instead of filtering on `member_log.opera`.
fn member_activity_content_keys(operas: i32) -> &'static [&'static str] {
    match operas {
        1 => &["wap_user_00154"],
        2 => &["common_01951", "wap_com_00428", "common_02021"],
        3 => &["wap_00070"],
        4 => &["common_02040"],
        5 => &["wap_00379", "common_01949", "member_user_00242"],
        6 => &["wap_00574", "common_01991", "common_01982", "common_01971"],
        7 => &["wap_00456"],
        8 => &["member_user_00226"],
        9 => &["wap_user_00220"],
        11 => &["admin_user_00140", "common_02035"],
        12 => &[
            "member_com_00093",
            "common_02028",
            "member_user_00234",
            "member_user_00236",
            "common_02034",
            "admin_user_00171",
            "member_user_00235",
        ],
        14 => &["member_com_00293", "common_01937"],
        15 => &["wap_00317", "common_01954"],
        16 => &[
            "wap_js_00081",
            "member_user_00161",
            "LOGO",
            "common_02012",
            "default_00092",
            "admin_tool_00428",
            "common_01886",
            "member_com_00077",
        ],
        17 => &["admin_yunying_00117", "wap_user_00008"],
        18 => &["common_01967", "common_01965", "common_02015", "wap_user_00363"],
        19 => &["wap_user_00223"],
        22 => &["admin_tool_00428"],
        23 => &["wap_com_00350"],
        25 => &["wap_com_00357", "admin_user_company_00379"],
        26 => &["wap_user_00221", "member_user_00044"],
        29 => &["common_02046"],
        88 => &["common_02029"],
        _ => &[],
    }
}

/// PHP `log_action` labels each row's day: today gets its own word, other days
/// get the weekday name, `date('w')` being 0 = Sunday.
fn activity_day_label(ctime: i64, today: &str) -> String {
    const WEEKDAYS: [&str; 7] = [
        "wap_00014", "wap_00008", "wap_00010", "wap_00009", "wap_00013", "wap_00011", "wap_00012",
    ];
    let day = fmt_ts(ctime, "%Y-%m-%d");
    let key = if day == today {
        "common_01940"
    } else {
        let w = fmt_ts(ctime, "%w").parse::<usize>().unwrap_or(0);
        WEEKDAYS[w.min(6)]
    };
    let lang = i18n::current_lang();
    i18n::t(&format!("messages.{key}"), lang)
}

/// PHP `users_member::log_action` — 会员活动日志（个人日志 tab 与日志列表页共用）.
async fn user_gap_member_activity(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let kw_type = json_i32(body, "type");
    let mut uid = json_u64(body, "uid");
    let mut username_like: Option<String> = None;
    if !kw.is_empty() {
        match kw_type {
            1 => username_like = Some(kw.clone()),
            3 => uid = kw.trim().parse().unwrap_or(uid),
            _ => {}
        }
    }
    let content = json_str(body, "content");
    // PHP: end=1 means "since midnight", any other N means "last N days".
    let mut time_from = Some(parse_date_ts(&json_str(body, "time_start"))).filter(|n| *n > 0);
    let end = json_i32(body, "end");
    if end > 0 {
        let now = clock::now_ts();
        let from_end = if end == 1 {
            now - now.rem_euclid(86_400)
        } else {
            now - i64::from(end) * 86_400
        };
        time_from = Some(time_from.map_or(from_end, |t| t.max(from_end)));
    }
    let time_to = Some(parse_date_ts(&json_str(body, "time_end")))
        .filter(|n| *n > 0)
        .map(|n| n + 86_399);
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
        content_like_any: member_activity_content_keys(json_i32(body, "operas")),
        opera: None,
        log_type: json_opt_i32(body, "parrs"),
        time_from,
        time_to,
        order_t: order_t.as_str(),
        order_dir: order_dir.as_str(),
    };
    let db = state.db.reader();
    let total = gap_repo::count_php_member_logs(db, &f).await?;
    let rows = if total > 0 {
        gap_repo::list_php_member_logs(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let today = fmt_ts(clock::now_ts(), "%Y-%m-%d");
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "opera": r.opera,
                "type": r.r#type,
                "usertype": r.usertype,
                "content": r.content,
                "ip": r.ip,
                "ctime": r.ctime,
                "ctime_n": fmt_ts(r.ctime, "%Y-%m-%d %H:%M:%S"),
                "date_n": fmt_ts(r.ctime, "%Y-%m-%d"),
                "time_n": fmt_ts(r.ctime, "%H:%M"),
                "week": activity_day_label(r.ctime, &today),
                "username": r.username,
                "rname": r.rname,
                "eid": r.eid,
                "sub_n": r.sub_n,
            })
        })
        .collect();
    let mut out = paged(Value::Array(list), total, page, per);
    let last_page = total.div_ceil(u64::from(per)).max(1);
    out["last_page"] = json!(last_page);
    Ok(out)
}

/// PHP `users_member::logDel_action`. `del=all` clears the whole log for the
/// account type the page is showing.
async fn user_gap_member_activity_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let db = state.db.pool();
    if json_str(body, "del").trim() == "all" {
        let ut = user_gap_log_usertype(body);
        gap_repo::delete_php_member_logs_by_usertype(db, ut).await?;
        return Ok(PhpOut::Message("admin_01293"));
    }
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    gap_repo::delete_php_member_logs(db, &ids).await?;
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `users_member::usercert_action` — one action, four branches picked by
/// which field the dialog sent.
async fn user_gap_usercert(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if body.get("batchfirm").is_some() {
        return user_gap_usercert_batch(state, body).await;
    }
    if body.get("email").is_some() {
        return user_gap_usercert_email(state, body).await;
    }
    if body.get("moblie").is_some() {
        return user_gap_usercert_mobile(state, body).await;
    }
    user_gap_usercert_idcard(state, body).await
}

/// PHP `users_member::batchfirm` — `type` is a checkbox list, `status` the flag.
async fn user_gap_usercert_batch(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let types: Vec<String> = match body.get("type") {
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        Some(Value::String(s)) => s.split(',').map(|p| p.trim().to_string()).collect(),
        _ => Vec::new(),
    };
    if types.is_empty() {
        return Err(ApiError::param_invalid("admin_01288"));
    }
    if body.get("status").is_none() {
        return Err(ApiError::param_invalid("admin_01289"));
    }
    let uids = ids_named(body, "uid");
    if uids.is_empty() {
        return Err(ApiError::param_invalid("member_com_00320"));
    }
    let status = json_i32(body, "status");
    let db = state.db.pool();
    for t in &types {
        let flag = match t.as_str() {
            "email" => gap_extra::CertFlag::Email,
            "moblie" => gap_extra::CertFlag::Mobile,
            "idcard" => gap_extra::CertFlag::Idcard,
            _ => continue,
        };
        gap_extra::set_cert_flag_bulk(db, &uids, flag, status).await?;
    }
    Ok(PhpOut::Message("admin_model_00110"))
}

/// PHP `users_member::emailstatus`.
async fn user_gap_usercert_email(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let email = json_str(body, "email");
    let email = email.trim();
    if email.is_empty() {
        return Err(ApiError::param_invalid("wap_01119"));
    }
    if !email.contains('@') || !email.rsplit('@').next().is_some_and(|d| d.contains('.')) {
        return Err(ApiError::param_invalid("wap_js_00120"));
    }
    let db = state.db.pool();
    let before = gap_extra::resume_email_state(db, uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_user_00086"))?;
    if before.0 == email && before.1 == 1 {
        return Ok(PhpOut::Message("admin_user_00080"));
    }
    let status = json_i32(body, "estatus");
    if gap_extra::rebind_member_email(db, uid, email, status).await? == 0 {
        return Err(ApiError::business("admin_user_00089"));
    }
    Ok(PhpOut::Message("admin_01286"))
}

/// PHP `users_member::mobliestatus`.
async fn user_gap_usercert_mobile(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let phone = json_str(body, "moblie");
    let phone = phone.trim();
    if phone.is_empty() {
        return Err(ApiError::param_invalid("wap_user_00274"));
    }
    if phone.len() != 11 || !phone.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::param_invalid("wap_user_00039"));
    }
    let db = state.db.pool();
    let before = gap_extra::resume_mobile_state(db, uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("admin_user_00086"))?;
    if before.0 == phone && before.1 == 1 {
        return Ok(PhpOut::Message("admin_user_00078"));
    }
    let status = json_i32(body, "mstatus");
    if !gap_extra::rebind_member_mobile(db, uid, phone, status).await? {
        return Err(ApiError::business("admin_user_00087"));
    }
    Ok(PhpOut::Message("admin_01287"))
}

/// PHP `users_member::userStatus` → `resume.model::statusCert`.
async fn user_gap_usercert_idcard(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uids = ids_named(body, "uid");
    if uids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "r_status");
    let statusbody = json_str(body, "statusbody");
    let n =
        gap_extra::set_idcard_review_many(state.db.pool(), &uids, status, statusbody.trim()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("admin_user_00086"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `users_member::checksitedid_action` — 个人账号迁分站.
async fn user_gap_member_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uids = ids_named(body, "uid");
    if uids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    gap_extra::set_member_did(state.db.pool(), &uids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00111"))
}

/// PHP `company::checksitedid_action` — 企业账号迁分站.
async fn user_gap_company_checksitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uids = ids_named(body, "uid");
    if uids.is_empty() {
        return Err(ApiError::param_invalid("common_01236"));
    }
    gap_extra::set_company_did(state.db.pool(), &uids, json_i32(body, "did")).await?;
    Ok(PhpOut::Message("admin_model_00118"))
}

/// PHP `users_member::searchCom_action` — `{companyList:[{uid,name}]}`.
async fn user_gap_search_com(state: &AppState, body: &Value) -> AppResult<Value> {
    let name = json_str(body, "com_name");
    let name = name.trim();
    if name.is_empty() {
        return Ok(json!({ "companyList": [] }));
    }
    let rows = company_repo::search_admin_brief(state.db.reader(), name, 50).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|(uid, name)| json!({ "uid": uid, "name": name }))
        .collect();
    Ok(json!({ "companyList": list }))
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

/// PHP `users_member::index_action` — `{list,total,page_sizes,limit,page}`.
async fn user_gap_user_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let time_type = json_str(body, "time_type");
    let times = json_day_range(body, "times");
    let mut login = php_preset_days(json_i32(body, "login_days"));
    login = merge_ts_range(login, json_ms_day_range(body, "login_time"));
    let mut reg = php_preset_days(json_i32(body, "reg_days"));
    reg = merge_ts_range(reg, json_ms_day_range(body, "reg_time"));
    match time_type.as_str() {
        "adtime" => {
            reg = merge_ts_range(reg, times);
        }
        "lotime" => {
            login = merge_ts_range(login, times);
        }
        _ => {}
    }
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gap_extra::PhpUserMemberFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        kw_type: json_i32(body, "type"),
        r_status: json_opt_i32(body, "status"),
        source: json_opt_i32(body, "source"),
        def_job: json_opt_i32(body, "def_job"),
        login_from: login.0,
        login_to: login.1,
        reg_from: reg.0,
        reg_to: reg.1,
        order_t: &order_t,
        order_dir: &order_dir,
    };
    let db = state.db.reader();
    let total = gap_extra::php_count_user_members(db, &f).await?;
    let rows = if total > 0 {
        gap_extra::php_list_user_members(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "uid": r.uid,
                "username": r.username,
                "username_n": r.username_n,
                "email": r.email,
                "telphone": r.telphone,
                "moblie": r.moblie,
                "moblie_status": r.moblie_status,
                "idcard_status": r.idcard_status,
                "email_status": r.email_status,
                "email_status_n": r.email_status,
                "r_status": r.r_status.to_string(),
                "def_job": r.def_job.to_string(),
                "usertype": r.usertype,
                "status": r.status,
                "source": r.source,
                "wxid": r.wxid,
                "wxopenid": r.wxopenid,
                "unionid": r.unionid,
                "wxBindmsg": wx_bind_msg(&r.wxid, &r.unionid),
                "login_ip": r.login_ip,
                "login_address": r.login_address,
                "moblie_address": r.moblie_address,
                "login_date": r.login_date,
                "login_date_n": if r.login_date > 0 { fmt_ts(r.login_date, "%Y-%m-%d %H:%M") } else { String::new() },
                "reg_date": r.reg_date,
                "reg_date_n": if r.reg_date > 0 { fmt_ts(r.reg_date, "%Y-%m-%d %H:%M") } else { String::new() },
                "sq_num": r.sq_num,
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

// ==================== 企业管理长尾 (PHP `user/company`) ====================

/// PHP `company::bindPackage_action` — bind extra rating packages to a company.
async fn company_bind_package(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_js_00141"));
    }
    // PHP `pylode(',', $_POST['package'])`: the checkbox group posts an array,
    // and clearing every box legitimately stores an empty string.
    let package = json_csv(body, "package");
    if gap_extra::set_company_package(state.db.pool(), uid, &package).await? == 0 {
        return Err(ApiError::business("wap_js_00141"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

/// PHP `company::setLogo_action` — admin overwrites a company logo.
async fn company_set_logo(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let logo = json_str(body, "logo");
    if uid == 0 || logo.is_empty() {
        return Err(ApiError::param_invalid("api_wxapp_00016"));
    }
    if gap_extra::set_company_logo_by_admin(state.db.pool(), uid, &logo).await? == 0 {
        return Err(ApiError::business("api_wxapp_00016"));
    }
    let lang = i18n::current_lang();
    let uid_s = uid.to_string();
    Ok(PhpOut::Text(
        "admin_model_00123",
        i18n::t_args("messages.admin_model_00123", lang, &[("uid", &uid_s)]),
    ))
}

/// PHP `company::checkguwen_action` — assign a CRM advisor to one or many
/// companies and notify each of them.
async fn company_check_guwen(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let gid = json_u64(body, "gid");
    let uids = gap_extra::parse_id_csv(&json_csv(body, "comid"));
    if gid == 0 || uids.is_empty() {
        return Err(ApiError::param_invalid("admin_01307"));
    }
    let Some(advisor) = gap_extra::admin_user_name(state.db.pool(), gid).await? else {
        return Err(ApiError::business("admin_01307"));
    };
    let now = clock::now_ts();
    if gap_extra::set_company_advisor(state.db.pool(), &uids, gid, now).await? == 0 {
        return Err(ApiError::business("common_06402"));
    }
    let lang = i18n::current_lang();
    let notice = format!("{}{}", i18n::t("messages.common_00802", lang), advisor);
    for uid in &uids {
        gap_repo::insert_sysmsg(state.db.pool(), *uid, 2, &notice, now).await?;
    }
    Ok(PhpOut::Message("common_06401"))
}

/// The ledger kinds behind `company_statis_detail.type` (PHP
/// `statis.model::$typeN`). Kind 1 is a literal in PHP, not a lang key.
const STATIS_DETAIL_TYPES: &[(i32, &str)] = &[
    (1, "上架|发布 职位"),
    (2, "wap_com_00029"),
    (3, "wap_00451"),
    (4, "resume_00029"),
    (5, "wap_com_00237"),
    (6, "member_com_00613"),
    (7, "wap_com_00238"),
    (8, "wap_com_00039"),
    (10, "admin_user_00019"),
    (11, "wap_00788"),
];

fn statis_detail_type_name(kind: i32) -> &'static str {
    STATIS_DETAIL_TYPES
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, n)| *n)
        .unwrap_or("")
}

/// PHP `company::statisDetail_action` — one company's package ledger.
/// `type_n` stays a raw lang key, matching PHP; the admin table runs it
/// through `lc()`.
async fn company_statis_detail(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_00203"));
    }
    let (page, per, offset, limit) = page_of(body);
    let filter = gap_extra::StatisDetailFilter {
        uid,
        kind: json_i32(body, "type"),
    };
    let db = state.db.reader();
    let total = gap_extra::count_company_statis_details(db, &filter).await?;
    let rows = if total > 0 {
        gap_extra::list_company_statis_details(db, &filter, limit, offset).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "type": r.kind,
                "type_n": statis_detail_type_name(r.kind),
                "num": r.num,
                "detail": r.detail,
                "time": r.time,
                "time_n": fmt_dt(r.time),
                "uri": r.uri,
                "ip": r.ip,
            })
        })
        .collect();
    // PHP only offers kinds 1..8 in the filter dropdown even though `typeN`
    // also names 10 and 11.
    let options: serde_json::Map<String, Value> = STATIS_DETAIL_TYPES
        .iter()
        .filter(|(k, _)| *k <= 8)
        .map(|(k, n)| (k.to_string(), Value::String((*n).to_string())))
        .collect();
    let mut out = paged(Value::Array(list), total, page, per);
    if let Some(obj) = out.as_object_mut() {
        obj.insert(
            "search_list".into(),
            json!([{
                "param": "type",
                "name": "admin_user_company_00051",
                "value": Value::Object(options),
            }]),
        );
    }
    Ok(out)
}

/// PHP `company::delStatisDetail_action`.
async fn company_del_statis_detail(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("common_06585"));
    }
    if gap_extra::delete_company_statis_details(state.db.pool(), &ids).await? == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}

/// PHP `company::mcomtpl_action` — skins this company may pick from, plus the
/// one currently applied.
async fn company_mcomtpl(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "comid");
    if uid == 0 {
        return Err(ApiError::param_invalid("wap_00203"));
    }
    let db = state.db.reader();
    let rows = gap_extra::list_company_tpls_for(db, uid).await?;
    let applied = company_tpl_repo::fetch_applied_tpl(db, uid).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.id,
                "name": r.name,
                "url": r.url,
                "pic": r.pic,
                "pic_n": pic_url(&base, &r.pic),
                "price": r.price,
                "status": r.status,
                "preview_url": format!("{base}/companies/{uid}?style={}", r.url),
            })
        })
        .collect();
    // The drawer reads `comtplstatis.comtpl` straight away, so this has to stay
    // an object even when the company has no `company_statis` row.
    Ok(json!({ "list": list, "statis": { "comtpl": applied } }))
}

/// PHP `company::msettpl_action` — admin applies a skin on the company's behalf.
async fn company_msettpl(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "comid");
    let id = json_u64(body, "id");
    if uid == 0 || id == 0 {
        return Err(ApiError::param_invalid("wap_01715"));
    }
    let Some(url) = gap_extra::company_tpl_url(state.db.pool(), id).await? else {
        return Err(ApiError::business("wap_01715"));
    };
    if company_tpl_repo::set_applied_tpl(state.db.pool(), uid, &url).await? == 0 {
        return Err(ApiError::business("wap_01715"));
    }
    let lang = i18n::current_lang();
    let notice = format!(
        "{}<a href=\"comtpl,{uid}\">{url}</a>",
        i18n::t("messages.admin_user_company_00405", lang)
    );
    gap_repo::insert_sysmsg(state.db.pool(), uid, 2, &notice, clock::now_ts()).await?;
    Ok(PhpOut::Message("model_00011"))
}

/// PHP `company::addTuiWenTask_action` — queue a 推文 task per company.
async fn company_add_tuiwen_task(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let uids = gap_extra::parse_id_csv(&json_csv(body, "twtask_uid"));
    let content = json_str(body, "twtask_content");
    if uids.is_empty() || user.uid == 0 {
        return Err(ApiError::param_invalid("common_01238"));
    }
    let companies = gap_extra::tuiwen_companies(state.db.reader(), &uids).await?;
    if companies.is_empty() {
        return Err(ApiError::business("common_06677"));
    }
    let rows: Vec<gap_extra::TuiWenTaskIn> = companies
        .iter()
        .map(|c| gap_extra::TuiWenTaskIn {
            cuid: c.uid,
            comname: c.name.clone(),
            jobsdate: c.lastupdate.trim().parse().unwrap_or(0),
            auid: user.uid,
            content: content.clone(),
            urgent: json_i32(body, "twtask_urgent"),
            wcmoments: json_i32(body, "twtask_wcmoments"),
            gzh: json_i32(body, "twtask_gzh"),
            jobid: 0,
            jobname: String::new(),
            kind: 2,
        })
        .collect();
    if gap_extra::insert_tuiwen_tasks(state.db.pool(), &rows, clock::now_ts()).await? == 0 {
        return Err(ApiError::business("common_06677"));
    }
    Ok(PhpOut::Message("common_06676"))
}

fn msg_t(key: &str) -> String {
    let lang = i18n::current_lang();
    for prefix in ["messages.", "errors."] {
        let prefixed = format!("{prefix}{key}");
        let t = i18n::t(&prefixed, lang);
        if t != prefixed {
            return t;
        }
    }
    let t = i18n::t(key, lang);
    if t != key {
        t
    } else {
        key.to_string()
    }
}

fn is_system_sender(kw: &str) -> bool {
    let kw = kw.trim();
    kw == "common_02020" || (!kw.is_empty() && kw == msg_t("common_02020"))
}

fn php_unix_range(body: &Value) -> (Option<i64>, Option<i64>) {
    let a = json_ts(body, "date1");
    let b = json_ts(body, "date2");
    if a > 0 && b > 0 {
        (Some(a), Some(b + 86_399))
    } else {
        (None, None)
    }
}

async fn recycle_ids(
    state: &AppState,
    user: &AuthenticatedUser,
    table: &str,
    ids: &[u64],
    uri: &str,
) {
    let joined = ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let ident = md5_hex(&format!("{table}{joined}"));
    let username = recycle_php::admin_username(state.db.pool(), user.uid)
        .await
        .unwrap_or_default();
    if let Err(e) = recycle_php::archive(
        state.db.pool(),
        table,
        ids,
        user.uid,
        &username,
        &ident,
        uri,
    )
    .await
    {
        tracing::warn!(error = %e, table, "recycle snapshot skipped");
    }
}

fn del_ids_msg(prefix_key: &str, ids: &[u64]) -> String {
    format!(
        "{}{}{}",
        msg_t(prefix_key),
        ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
        msg_t("model_00112"),
    )
}

async fn resolve_name_uids(state: &AppState, kw: &str) -> AppResult<Vec<u64>> {
    Ok(gap_extra::find_display_uids_like(state.db.reader(), kw).await?)
}

async fn email_log_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ty = json_i32(body, "type");
    let mut email_kw = None;
    let mut smtp_kw = None;
    let mut cuid_zero = false;
    let mut uid_zero = false;
    let cuid_buf = if !kw.is_empty() && ty == 2 && !is_system_sender(&kw) {
        resolve_name_uids(state, &kw).await?
    } else {
        Vec::new()
    };
    let uid_buf = if !kw.is_empty() && ty == 3 && !is_system_sender(&kw) {
        resolve_name_uids(state, &kw).await?
    } else {
        Vec::new()
    };
    let mut cuid_in = None;
    let mut uid_in = None;
    if !kw.is_empty() {
        match ty {
            2 => {
                if is_system_sender(&kw) {
                    cuid_zero = true;
                } else {
                    cuid_in = Some(cuid_buf.as_slice());
                }
            }
            3 => {
                if is_system_sender(&kw) {
                    uid_zero = true;
                } else {
                    uid_in = Some(uid_buf.as_slice());
                }
            }
            4 => smtp_kw = Some(kw.as_str()),
            _ => email_kw = Some(kw.as_str()),
        }
    }
    let (time_min, time_max) = php_unix_range(body);
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
    let f = email_msg_repo::PhpEmailFilter {
        email_kw,
        smtp_kw,
        cuid_zero,
        uid_zero,
        cuid_in,
        uid_in,
        time_min,
        time_max,
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let rows = email_msg_repo::php_list(db, &f, offset, limit).await?;
    let total = email_msg_repo::php_count(db, &f).await?;
    let mut uids = Vec::new();
    for r in &rows {
        if r.cuid > 0 {
            uids.push(r.cuid as u64);
        }
        if r.uid > 0 {
            uids.push(r.uid as u64);
        }
    }
    uids.sort_unstable();
    uids.dedup();
    let names = gap_extra::display_names_by_uids(db, &uids).await?;
    let sys = msg_t("common_02020");
    let admin = msg_t("wap_user_00361");
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let fname = if r.cuid > 0 {
                names
                    .get(&(r.cuid as u64))
                    .cloned()
                    .unwrap_or_else(|| r.cname.clone())
            } else {
                sys.clone()
            };
            let sname = if r.uid > 0 {
                names.get(&(r.uid as u64)).cloned().unwrap_or_default()
            } else if r.uid < 0 {
                admin.clone()
            } else {
                String::new()
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "cuid": r.cuid,
                "email": r.email,
                "title": r.title,
                "content": r.content,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "state": r.state,
                "smtpserver": r.smtpserver,
                "fname": fname,
                "sname": sname,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn email_log_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01066"));
    }
    recycle_ids(
        state,
        user,
        "email_msg",
        &ids,
        "/v1/admin/php-content/email-log/delete",
    )
    .await;
    let n = email_msg_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Text("admin_user_00187", del_ids_msg("model_00244", &ids)))
}

async fn email_log_repeat(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("admin_tool_00021"));
    }
    let rows = email_msg_repo::get_by_ids(state.db.reader(), &ids).await?;
    let failed: Vec<_> = rows
        .into_iter()
        .filter(|r| r.state != 1 && r.del != 1 && r.email.contains('@'))
        .collect();
    if failed.is_empty() {
        return Ok(PhpOut::Message("common_01031"));
    }
    let mut ok = 0u32;
    let mut bad = 0u32;
    for row in &failed {
        match state
            .events
            .publish_json(
                "email.verify_queued",
                &json!({
                    "kind": "admin_email_repeat",
                    "id": row.id,
                    "email": row.email,
                    "subject": row.title,
                    "content": row.content,
                    "repeat": 1,
                }),
            )
            .await
        {
            Ok(_) => ok += 1,
            Err(_) => bad += 1,
        }
    }
    let mut msg = format!("{}{ok}条", msg_t("common_01132"));
    if bad > 0 {
        msg.push_str(&format!("，失败：{bad}条"));
    }
    Ok(PhpOut::Text("common_01132", msg))
}

async fn sms_log_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ty = json_i32(body, "type");
    let mut moblie_kw = None;
    let mut content_kw = None;
    let mut cuid_zero = false;
    let mut uid_zero = false;
    let cuid_buf = if !kw.is_empty() && ty == 2 && !is_system_sender(&kw) {
        resolve_name_uids(state, &kw).await?
    } else {
        Vec::new()
    };
    let uid_buf = if !kw.is_empty() && ty == 3 && !is_system_sender(&kw) {
        resolve_name_uids(state, &kw).await?
    } else {
        Vec::new()
    };
    let mut cuid_in = None;
    let mut uid_in = None;
    if !kw.is_empty() {
        match ty {
            2 => {
                if is_system_sender(&kw) {
                    cuid_zero = true;
                } else {
                    cuid_in = Some(cuid_buf.as_slice());
                }
            }
            3 => {
                if is_system_sender(&kw) {
                    uid_zero = true;
                } else {
                    uid_in = Some(uid_buf.as_slice());
                }
            }
            4 => content_kw = Some(kw.as_str()),
            _ => moblie_kw = Some(kw.as_str()),
        }
    }
    let time_days = json_i32(body, "time");
    let (d1, d2) = php_unix_range(body);
    let (time_min, time_max) = if time_days > 0 {
        (
            Some(if time_days == 1 {
                clock::start_of_today()
            } else {
                days_ago_ts(time_days)
            }),
            None,
        )
    } else {
        (d1, d2)
    };
    let state_f = match json_i32(body, "state") {
        1 => Some(0),
        2 => Some(2),
        _ => None,
    };
    let port = json_present_i32(body, "port");
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
    let f = moblie_msg_repo::PhpSmsFilter {
        moblie_kw,
        content_kw,
        cuid_zero,
        uid_zero,
        cuid_in,
        uid_in,
        time_min,
        time_max,
        state: state_f,
        port,
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let rows = moblie_msg_repo::php_list(db, &f, offset, limit).await?;
    let total = moblie_msg_repo::php_count(db, &f).await?;
    let mut uids = Vec::new();
    for r in &rows {
        if r.cuid > 0 {
            uids.push(r.cuid as u64);
        }
        if r.uid > 0 {
            uids.push(r.uid as u64);
        }
    }
    uids.sort_unstable();
    uids.dedup();
    let names = gap_extra::display_names_by_uids(db, &uids).await?;
    let sys = msg_t("common_02020");
    let admin = msg_t("wap_user_00361");
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let fname = if r.cuid > 0 {
                names
                    .get(&(r.cuid as u64))
                    .cloned()
                    .unwrap_or_else(|| r.cname.clone())
            } else {
                sys.clone()
            };
            let sname = if r.uid > 0 {
                names.get(&(r.uid as u64)).cloned().unwrap_or_default()
            } else if r.uid < 0 {
                admin.clone()
            } else {
                String::new()
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "cuid": r.cuid,
                "moblie": r.moblie,
                "content": r.content,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "state": r.state,
                "ip": r.ip,
                "port": r.port,
                "port_n": sms_port_n(r.port),
                "location": r.location,
                "result": sms_result_n(r.state),
                "fname": fname,
                "sname": sname,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

fn sms_port_n(port: i32) -> String {
    match port {
        1 => msg_t("member_user_00094"),
        2 => "WAP".into(),
        5 => msg_t("wap_js_00101"),
        7 => msg_t("ajax_00010"),
        8 => msg_t("wap_00121"),
        _ => String::new(),
    }
}

fn sms_result_n(state: i32) -> String {
    match state {
        0 => String::new(),
        401 => "手机号为空".into(),
        402 => "短信内容为空".into(),
        403 => "appKey为空".into(),
        404 => "appSecret为空".into(),
        405 => "手机号码格式错误".into(),
        406 => "禁用手机号".into(),
        407 => "短信内容含有敏感字词".into(),
        410 => "短信秘钥认证错误".into(),
        411 => "网站无有效短信签名".into(),
        412 => "短信余额不足".into(),
        413 => "短信发送失败".into(),
        501 => "检测是空号".into(),
        502 => "空号检测归属地失败".into(),
        n => n.to_string(),
    }
}

async fn sms_log_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01066"));
    }
    recycle_ids(
        state,
        user,
        "moblie_msg",
        &ids,
        "/v1/admin/php-content/sms-log/delete",
    )
    .await;
    let n = moblie_msg_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Text("admin_user_00187", del_ids_msg("model_00243", &ids)))
}

async fn sms_log_repeat(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("admin_tool_00021"));
    }
    let rows = moblie_msg_repo::get_by_ids(state.db.reader(), &ids).await?;
    let failed: Vec<_> = rows
        .into_iter()
        .filter(|r| r.state != 0 && r.del != 1 && !r.moblie.is_empty())
        .collect();
    if failed.is_empty() {
        return Ok(PhpOut::Message("common_01031"));
    }
    let mut ok = 0u32;
    let mut bad = 0u32;
    for row in &failed {
        match state
            .events
            .publish_json(
                "sms.send_queued",
                &json!({
                    "kind": "admin_sms_repeat",
                    "id": row.id,
                    "phone": row.moblie,
                    "content": row.content,
                }),
            )
            .await
        {
            Ok(_) => ok += 1,
            Err(_) => bad += 1,
        }
    }
    let mut msg = format!("{}{ok}条", msg_t("common_01131"));
    if bad > 0 {
        msg.push_str(&format!("，失败：{bad}条"));
    }
    Ok(PhpOut::Text("common_01131", msg))
}

fn warn_type_n(kind: i32) -> String {
    let key = match kind {
        1 => "wap_com_00028",
        2 => "wap_00451",
        3 => "wap_user_00111",
        4 => "common_01686",
        5 => "admin_user_00166",
        6 => "common_06469",
        7 => "wap_com_00355",
        8 => "member_com_00032",
        9 => "common_01800",
        12 => "common_01239",
        _ => return String::new(),
    };
    msg_t(key)
}

async fn warning_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let (time_min, time_max) = php_unix_range(body);
    let f = warning_repo::PhpWarningFilter {
        status: json_present_i32(body, "status").filter(|n| *n > 0),
        time_min,
        time_max,
    };
    let db = state.db.reader();
    let rows = warning_repo::php_list(db, &f, offset, limit).await?;
    let total = warning_repo::php_count(db, &f).await?;
    let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
    let _ = warning_repo::mark_admin_seen(state.db.pool(), &ids).await;
    let tip = msg_t("common_01546");
    let seeker = msg_t("admin_user_00122");
    let company = msg_t("admin_user_00124");
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let type_n = warn_type_n(r.warn_type);
            let content = if r.warn_type == 15 {
                r.content
            } else {
                format!("{type_n}{tip}")
            };
            let usertype_n = match r.usertype {
                1 => seeker.clone(),
                2 => company.clone(),
                _ => String::new(),
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "type": r.warn_type,
                "type_n": type_n,
                "status": r.status,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "content": content,
                "usertype": r.usertype,
                "usertype_n": usertype_n,
                "username": r.username,
                "name_n": r.name_n,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn warning_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("member_com_00084"));
    }
    recycle_ids(
        state,
        user,
        "warning",
        &ids,
        "/v1/admin/php-content/warning/delete",
    )
    .await;
    let n = warning_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    let msg = format!(
        "{}{}{}{}",
        msg_t("model_00215"),
        ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
        msg_t("model_00130"),
        msg_t("admin_user_00187"),
    );
    Ok(PhpOut::Text("admin_user_00187", msg))
}

const WARNING_CFG_KEYS: &[&str] = &[
    "warning_addjob",
    "warning_addjob_type",
    "warning_downresume",
    "warning_downresume_type",
    "warning_addresume",
    "warning_addresume_type",
    "warning_recharge",
    "warning_recharge_type",
    "sy_hour_msgnum",
    "warning_closemsg_type",
    "warning_lookresume",
    "warning_lookresume_type",
    "warning_lookjob",
    "warning_lookjob_type",
    "warning_teljob",
    "warning_teljob_type",
    "warning_reg_ip",
    "warning_reg_ip_type",
    "warning_exchange_link",
    "warning_exchange_link_type",
    "warning_sendresume",
    "warning_sendresume_type",
    "warning_sendresume_tips",
    "warning_sendresume_tipss",
    "warning_sqjob",
    "warning_sqjob_type",
    "warning_sqjob_tips",
    "warning_sqjob_tipss",
];

async fn warning_get_config(state: &AppState) -> AppResult<Value> {
    let map = setting_repo::find_many(state.db.reader(), WARNING_CFG_KEYS).await?;
    let mut out = serde_json::Map::new();
    for k in WARNING_CFG_KEYS {
        out.insert(
            (*k).to_string(),
            json!(map.get(*k).cloned().unwrap_or_default()),
        );
    }
    Ok(Value::Object(out))
}

async fn warning_config_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    for key in WARNING_CFG_KEYS {
        if body.get(*key).is_some() {
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
    Ok(PhpOut::Message("admin_01362"))
}

fn cron_log_time_range(body: &Value) -> (Option<i64>, Option<i64>) {
    let (a, b) = json_day_range(body, "time");
    if a.is_some() {
        return (a, b);
    }
    let s = json_str(body, "time");
    if let Some((l, r)) = s.split_once('~') {
        let from = parse_date_ts(l);
        let mut to = parse_date_ts(r);
        if to > 0 {
            to += 86_399;
        }
        return (
            if from > 0 { Some(from) } else { None },
            if to > 0 { Some(to) } else { None },
        );
    }
    (None, None)
}

async fn cron_log_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let (time_min, time_max) = cron_log_time_range(body);
    let mut sort = json_str(body, "t");
    if sort == "ctime_n" || sort == "cron_name" {
        sort = if sort == "cron_name" {
            "id".into()
        } else {
            "ctime".into()
        };
    }
    let dir = json_str(body, "order");
    let f = gap_extra::PhpCronLogFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        time_min,
        time_max,
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let mut rows = gap_extra::php_list_cron_logs(db, &f, offset, limit).await?;
    let total = gap_extra::php_count_cron_logs(db, &f).await?;
    for r in &mut rows {
        r.ctime_n = if r.ctime > 0 {
            fmt_dt(r.ctime)
        } else {
            String::new()
        };
    }
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "cid": r.cid,
                "ctime": r.ctime,
                "ctime_n": r.ctime_n,
                "name": r.name,
                "cron_name": r.name,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn cron_log_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_00597"));
    }
    recycle_ids(
        state,
        user,
        "cron_log",
        &ids,
        "/v1/admin/php-content/cron-log/delete",
    )
    .await;
    let n = gap_extra::delete_cron_logs(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("common_01536"))
}

async fn shop_reward_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ctype = json_i32(body, "ctype");
    let mut name_kw = None;
    let mut integral = None;
    if !kw.is_empty() {
        if ctype == 2 {
            integral = Some(json_i32(body, "keyword"));
        } else {
            name_kw = Some(kw.as_str());
        }
    }
    let status = match json_i32(body, "status") {
        0 => None,
        2 => Some(0),
        n => Some(n),
    };
    let rec = match json_i32(body, "rec") {
        0 => None,
        2 => Some(0),
        n => Some(n),
    };
    let hot = match json_i32(body, "hot") {
        0 => None,
        2 => Some(0),
        n => Some(n),
    };
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
    let f = redeem_repo::PhpRewardFilter {
        name_kw,
        integral,
        nid: Some(json_u64(body, "nid")).filter(|n| *n > 0),
        status,
        rec,
        hot,
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let rows = redeem_repo::php_list_rewards(db, &f, offset, limit).await?;
    let total = redeem_repo::php_count_rewards(db, &f).await?;
    let classes = redeem_repo::list_classes(db, None).await?;
    let class_name: HashMap<u64, String> = classes.into_iter().map(|c| (c.id, c.name)).collect();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let mut classname = class_name.get(&r.nid).cloned().unwrap_or_default();
            if let Some(tn) = class_name.get(&r.tnid) {
                classname = if classname.is_empty() {
                    tn.clone()
                } else {
                    format!("{classname}-{tn}")
                };
            }
            json!({
                "id": r.id,
                "name": r.name,
                "nid": r.nid,
                "tnid": r.tnid,
                "integral": r.integral,
                "restriction": r.restriction,
                "stock": r.stock,
                "sort": r.sort,
                "status": r.status,
                "status_n": r.status == 1,
                "rec": r.rec,
                "rec_n": r.rec == 1,
                "hot": r.hot,
                "hot_n": r.hot == 1,
                "pic": r.pic,
                "classname": classname,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn shop_reward_flag(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
    is_rec: bool,
) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    let flag = if is_rec {
        json_i32(body, "rec")
    } else {
        json_i32(body, "hot")
    }
    .clamp(0, 1);
    let (rec, hot) = if is_rec {
        (Some(flag), None)
    } else {
        (None, Some(flag))
    };
    redeem_service::set_reward_flags(state, user, id, rec, hot).await?;
    Ok(PhpOut::Message(if is_rec {
        "admin_01434"
    } else {
        "admin_01435"
    }))
}

async fn shop_reward_getclass(state: &AppState, body: &Value) -> AppResult<Value> {
    let nid = json_u64(body, "nid");
    if nid == 0 {
        return Err(ApiError::business("common_01237"));
    }
    let classes = redeem_repo::list_classes(state.db.reader(), Some(nid)).await?;
    let class: Vec<Value> = classes.iter().map(redeem_class_json).collect();
    Ok(json!({ "class": class }))
}

fn redeem_class_json(c: &phpyun_models::redeem::entity::RedeemClass) -> Value {
    json!({
        "id": c.id,
        "name": c.name,
        "keyid": c.parent_id,
        "sort": c.sort,
    })
}

async fn shop_class_index(state: &AppState) -> AppResult<Value> {
    let rows = redeem_repo::list_classes(state.db.reader(), Some(0)).await?;
    let list: Vec<Value> = rows.iter().map(redeem_class_json).collect();
    Ok(json!({ "list": list }))
}

async fn shop_class_up(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({ "list": [] }));
    }
    let db = state.db.reader();
    let mut list = Vec::new();
    if let Some(one) = redeem_repo::find_class(db, id).await? {
        list.push(redeem_class_json(&one));
        let kids = redeem_repo::list_classes(db, Some(id)).await?;
        list.extend(kids.iter().map(redeem_class_json));
    }
    Ok(json!({ "list": list }))
}

async fn shop_class_ajax(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    let name = json_str(body, "name");
    let sort = if body.get("sort").is_some() {
        Some(json_i32(body, "sort"))
    } else {
        None
    };
    let name_opt = if name.is_empty() { None } else { Some(name.as_str()) };
    redeem_service::update_class_fields(state, user, id, name_opt, sort).await?;
    Ok(PhpOut::Message("admin_user_company_00208"))
}

async fn friend_link_get_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({}));
    }
    match friend_link_repo::php_find(state.db.reader(), id).await? {
        Some(r) => {
            let pic = clean_stored_pic(&r.pic);
            let pic_n = pic_url(&preview_base(state), &pic);
            Ok(json!({
                "id": r.id,
                "link_name": r.link_name,
                "link_url": r.link_url,
                "pic": pic,
                "pic_n": pic_n,
                "link_type": r.link_type,
                "link_sorting": r.link_sorting.to_string(),
                "did": r.did,
                "tem_type": r.tem_type,
                "img_type": r.img_type,
            }))
        }
        None => Ok(json!({})),
    }
}

async fn friend_link_sitedid(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01236"));
    }
    let n = friend_link_repo::set_did(state.db.pool(), &ids, json_i32(body, "did")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00030"));
    }
    friend_link_service::invalidate_all().await;
    let msg = format!(
        "{}{}{}",
        msg_t("model_00211"),
        ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
        msg_t("model_00212"),
    );
    Ok(PhpOut::Text("model_00212", msg))
}

fn nest<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.get(key).unwrap_or(&Value::Null)
}

fn php_address(body: &str) -> String {
    let chars: Vec<char> = body.trim().chars().collect();
    if chars.len() <= 6 {
        return String::new();
    }
    chars[5..chars.len() - 1].iter().collect()
}

fn stored_pic(raw: &str) -> Option<String> {
    let s = clean_stored_pic(raw);
    if s.is_empty() || s.starts_with("blob:") {
        None
    } else {
        Some(s)
    }
}

fn clean_stored_pic(pic: &str) -> String {
    let p = pic.trim();
    if p.is_empty() || p.eq_ignore_ascii_case("undefined") || p.eq_ignore_ascii_case("null") {
        String::new()
    } else {
        p.to_string()
    }
}

async fn credit_integral(
    state: &AppState,
    uid: u64,
    usertype: i32,
    amount: i32,
    remark: &str,
) -> AppResult<()> {
    if uid == 0 || amount <= 0 {
        return Ok(());
    }
    let now = clock::now_ts();
    let delta = i64::from(amount);
    if usertype == 2 {
        cstatis_repo::adjust_integral(state.db.pool(), uid, delta).await?;
    } else {
        mstatis_repo::add_balance(state.db.pool(), uid, delta, now).await?;
    }
    let oid = new_dingdan(now);
    pay_repo::php_insert_pay_typed(
        state.db.pool(),
        &oid,
        &amount.to_string(),
        now,
        uid,
        remark,
        1,
        usertype,
        24,
    )
    .await?;
    Ok(())
}

async fn shop_reward_add(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    if has_flag(body, "add") {
        let id = json_u64(body, "id");
        let db = state.db.reader();
        let class = redeem_repo::list_classes(db, Some(0)).await?;
        let class: Vec<Value> = class.iter().map(redeem_class_json).collect();
        let integral_pricename = setting_repo::find(db, "integral_pricename")
            .await?
            .map(|s| s.value)
            .unwrap_or_else(|| "积分".into());
        let info = if id > 0 {
            match redeem_repo::php_get_reward(db, id).await? {
                Some(r) => {
                    let nid = if r.nid > 0 {
                        json!(r.nid.to_string())
                    } else {
                        json!("")
                    };
                    let tnid = if r.tnid > 0 {
                        json!(r.tnid.to_string())
                    } else {
                        json!("")
                    };
                    json!({
                        "id": r.id,
                        "name": r.name,
                        "nid": nid,
                        "tnid": tnid,
                        "integral": r.integral.to_string(),
                        "restriction": r.restriction.to_string(),
                        "stock": r.stock.to_string(),
                        "sort": r.sort.to_string(),
                        "status": if r.status == 0 { "1".into() } else { r.status.to_string() },
                        "pic": pic_url(&preview_base(state), &r.pic),
                        "content": r.content,
                        "content_n": r.content,
                    })
                }
                None => json!({}),
            }
        } else {
            json!({})
        };
        return Ok(PhpOut::Data(json!({
            "info": info,
            "class": class,
            "integral_pricename": integral_pricename,
        })));
    }
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::business("admin_vue_00084"));
    }
    let id = json_u64(body, "id");
    let _ = user;
    let n = redeem_repo::php_save_reward(
        state.db.pool(),
        &redeem_repo::PhpRewardSave {
            id: if id > 0 { Some(id) } else { None },
            name: &name,
            pic: stored_pic(&json_str(body, "pic")).as_deref(),
            content: &amp(&json_str(body, "content")),
            integral: json_i32(body, "integral"),
            stock: json_i32(body, "stock"),
            restriction: json_i32(body, "restriction"),
            nid: json_u64(body, "nid"),
            tnid: json_u64(body, "tnid"),
            status: {
                let s = json_i32(body, "status");
                if s == 0 { 1 } else { s }
            },
            sort: json_i32(body, "sort"),
            now: clock::now_ts(),
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business(if id > 0 {
            "admin_01422"
        } else {
            "api_wxapp_00012"
        }));
    }
    Ok(PhpOut::Message(if id > 0 {
        "admin_01431"
    } else {
        "admin_01432"
    }))
}

async fn shop_reward_status(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    redeem_service::set_reward_status(state, user, id, json_i32(body, "status")).await?;
    Ok(PhpOut::Message("admin_01433"))
}

async fn shop_reward_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01162"));
    }
    recycle_ids(
        state,
        user,
        "reward",
        &ids,
        "/v1/admin/php-content/shop-reward/delete",
    )
    .await;
    let n = redeem_repo::php_delete_rewards(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("wap_user_00146"));
    }
    Ok(PhpOut::Message("admin_01436"))
}

async fn shop_class_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let names: Vec<String> = json_str(body, "name")
        .split('-')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    if names.is_empty() {
        return Err(ApiError::business("admin_01200"));
    }
    if redeem_repo::count_class_names(state.db.reader(), &names).await? > 0 {
        return Err(ApiError::business("admin_system_00050"));
    }
    let parent = if json_i32(body, "ctype") == 1 {
        0
    } else {
        json_u64(body, "nid")
    };
    let mut last = 0u64;
    for name in &names {
        last = redeem_service::create_class(state, user, parent, name, 0).await?;
    }
    if last == 0 {
        return Err(ApiError::business("api_wxapp_00012"));
    }
    Ok(PhpOut::Message("admin_01426"))
}

async fn shop_class_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01162"));
    }
    recycle_ids(
        state,
        user,
        "redeem_class",
        &ids,
        "/v1/admin/php-content/shop-class/delete",
    )
    .await;
    let n = redeem_repo::php_delete_classes(state.db.pool(), &ids).await?;
    redeem_service::invalidate_classes_cache().await;
    if n == 0 {
        return Err(ApiError::business("wap_user_00146"));
    }
    Ok(PhpOut::Message("admin_01427"))
}

async fn shop_list_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let ty = json_i32(body, "type");
    let mut name_kw = None;
    let mut username_kw = None;
    if !kw.is_empty() {
        if ty == 2 {
            username_kw = Some(kw.as_str());
        } else {
            name_kw = Some(kw.as_str());
        }
    }
    let status = match json_i32(body, "status") {
        0 => None,
        -1 => Some(0),
        n => Some(n),
    };
    let change = json_i32(body, "change");
    let time_min = if change > 0 {
        Some(if change == 1 {
            clock::start_of_today()
        } else {
            days_ago_ts(change)
        })
    } else {
        None
    };
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
    let f = redeem_repo::PhpChangeFilter {
        name_kw,
        username_kw,
        status,
        time_min,
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let rows = redeem_repo::php_list_changes(db, &f, offset, limit).await?;
    let total = redeem_repo::php_count_changes(db, &f).await?;
    let gids: Vec<u64> = {
        let mut v: Vec<u64> = rows.iter().map(|r| r.gid).filter(|g| *g > 0).collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let mut pics: HashMap<u64, String> = HashMap::new();
    let base = preview_base(state);
    for gid in gids {
        if let Some(r) = redeem_repo::php_get_reward(db, gid).await? {
            pics.insert(gid, pic_url(&base, &r.pic));
        }
    }
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "username": r.username,
                "usertype": r.usertype,
                "name": r.name,
                "gid": r.gid,
                "integral": r.integral,
                "num": r.num,
                "linkman": r.linkman,
                "linktel": r.linktel,
                "body": r.body,
                "address": php_address(&r.body),
                "status": r.status,
                "statusbody": r.statusbody,
                "express": r.express,
                "expnum": r.expnum,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "pic": pics.get(&r.gid).cloned().unwrap_or_default(),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn shop_list_status(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("member_com_00320"));
    }
    let new_status = json_i32(body, "status");
    if new_status <= 0 {
        return Err(ApiError::business("admin_user_weipin_00015"));
    }
    let row = redeem_repo::php_get_change(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("api_wxapp_00016"))?;
    let mut express = json_str(body, "express");
    let mut expnum = json_str(body, "expnum");
    if row.status == 0 && new_status > 0 {
        if new_status == 2 {
            express.clear();
            expnum.clear();
            if row.num != 0 {
                let _ = redeem_repo::php_adjust_reward_stock(state.db.pool(), row.gid, row.num)
                    .await?;
            }
            let already =
                pay_repo::count_by_remark(state.db.reader(), row.uid, "admin_01428").await?;
            if already == 0 && row.integral > 0 {
                credit_integral(state, row.uid, row.usertype, row.integral, "admin_01428")
                    .await?;
            }
        }
    }
    let n = redeem_repo::php_review_change(
        state.db.pool(),
        id,
        &redeem_repo::PhpChangeReview {
            status: new_status,
            linkman: &json_str(body, "linkman"),
            linktel: &json_str(body, "linktel"),
            statusbody: &json_str(body, "statusbody"),
            express: &express,
            expnum: &expnum,
        },
    )
    .await?;
    let _ = user;
    if n == 0 {
        return Err(ApiError::business("api_wxapp_00016"));
    }
    Ok(PhpOut::Message("admin_01429"))
}

async fn shop_list_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01162"));
    }
    let rows = redeem_repo::php_get_changes(state.db.reader(), &ids).await?;
    for r in &rows {
        if r.status == 0 {
            if r.num != 0 {
                let _ = redeem_repo::php_adjust_reward_stock(state.db.pool(), r.gid, r.num).await?;
            }
            if r.integral > 0 {
                credit_integral(state, r.uid, r.usertype, r.integral, "wap_user_00003").await?;
            }
        }
    }
    recycle_ids(
        state,
        user,
        "change",
        &ids,
        "/v1/admin/php-content/shop-list/delete",
    )
    .await;
    let n = redeem_repo::php_delete_changes(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("wap_user_00146"));
    }
    Ok(PhpOut::Message("admin_01430"))
}

fn friend_link_page(body: &Value) -> (u32, u32, u64, u64) {
    let p = nest(body, "pagination");
    if !p.is_null() {
        let page = json_u64(p, "page").max(1) as u32;
        let mut per = json_u64(p, "pageSize");
        if per == 0 {
            per = json_u64(p, "page_size");
        }
        if per == 0 {
            per = 20;
        }
        let per = per.clamp(1, 100) as u32;
        let offset = u64::from(page.saturating_sub(1)) * u64::from(per);
        return (page, per, offset, u64::from(per));
    }
    page_of(body)
}

fn friend_link_time_min(opt: &Value) -> Option<i64> {
    let n = json_i32(opt, "ctime");
    if n <= 0 {
        None
    } else if n == 1 {
        Some(clock::start_of_today())
    } else {
        Some(days_ago_ts(n))
    }
}

async fn friend_link_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = friend_link_page(body);
    let opt = nest(body, "searchOption");
    let kw = json_str(opt, "keyword");
    let ty = json_str(opt, "type");
    let did = json_i32(opt, "did");
    let state_f = json_present_i32(opt, "state");
    let sort = json_str(body, "t");
    let dir = json_str(body, "order");
    let f = friend_link_repo::PhpLinkFilter {
        name_kw: if kw.is_empty() { None } else { Some(kw.as_str()) },
        link_type: if ty.is_empty() { None } else { Some(ty.as_str()) },
        did: if did > 0 { Some(did) } else { None },
        state: state_f,
        time_min: friend_link_time_min(opt),
        sort: &sort,
        dir: &dir,
    };
    let db = state.db.reader();
    let rows = friend_link_repo::php_list(db, &f, offset, limit).await?;
    let total = friend_link_repo::php_count(db, &f).await?;
    let domains = domain_repo::list_all(db).await?;
    let mut did_name: HashMap<i32, String> = HashMap::new();
    for d in &domains {
        did_name.insert(d.id as i32, d.title.clone());
    }
    let type_txt = msg_t("admin_01013");
    let type_img = msg_t("admin_00100");
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let ts = r.link_time.trim().parse::<i64>().unwrap_or(0);
            let pic = clean_stored_pic(&r.pic);
            json!({
                "id": r.id,
                "link_name": r.link_name,
                "link_url": r.link_url,
                "pic": pic,
                "pic_n": pic_url(&base, &pic),
                "link_type": r.link_type,
                "link_type_n": if r.link_type == "1" { type_txt.clone() } else { type_img.clone() },
                "link_sorting": r.link_sorting,
                "link_state": r.link_state,
                "statusbody": r.statusbody,
                "did": r.did,
                "did_n": did_name.get(&r.did).cloned().unwrap_or_default(),
                "ctime_n": if ts > 0 { fmt_date(ts) } else { String::new() },
                "tem_type": r.tem_type,
                "img_type": r.img_type,
            })
        })
        .collect();
    let domain: Vec<Value> = domains
        .iter()
        .map(|d| json!({ "label": d.title, "value": d.id.to_string() }))
        .collect();
    let mut out = paged(Value::Array(list), total, page, per);
    if let Some(m) = out.as_object_mut() {
        m.insert("domain".into(), Value::Array(domain));
        m.insert("pageSize".into(), json!(per));
        m.insert("pageSizes".into(), json!([10, 20, 50, 100]));
    }
    Ok(out)
}

async fn friend_link_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let form = nest(body, "formdata");
    let id = json_u64(form, "id");
    if id == 0 {
        return Err(ApiError::business("common_06518"));
    }
    let n = friend_link_repo::php_set_status(
        state.db.pool(),
        id,
        json_i32(form, "status"),
        &json_str(form, "content"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_06517"));
    }
    friend_link_service::invalidate_all().await;
    Ok(PhpOut::Message("common_06516"))
}

async fn friend_link_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    recycle_ids(
        state,
        user,
        "admin_link",
        &ids,
        "/v1/admin/php-content/friend-link/delete",
    )
    .await;
    let n = friend_link_repo::php_delete_ids(state.db.pool(), &ids).await?;
    friend_link_service::invalidate_all().await;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    let msg = format!(
        "{}{}{}{}",
        msg_t("model_00211"),
        ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
        msg_t("model_00130"),
        msg_t("admin_user_00187"),
    );
    Ok(PhpOut::Text("admin_user_00187", msg))
}

async fn friend_link_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let sorting = json_str(body, "sorting");
    if !sorting.is_empty()
        && sorting
            .chars()
            .any(|c| !c.is_ascii_digit() && c != '-' && c != '.' && c != ',' && c != ' ')
    {
        return Err(ApiError::business("common_00811"));
    }
    let id = json_u64(body, "id");
    let phototype = json_i32(body, "phototype");
    let pic = if phototype == 1 {
        None
    } else {
        stored_pic(&json_str(body, "uplocadpic"))
    };
    let n = friend_link_repo::php_save(
        state.db.pool(),
        &friend_link_repo::PhpLinkSave {
            id: if id > 0 { Some(id) } else { None },
            link_name: &json_str(body, "title"),
            link_url: &json_str(body, "url"),
            pic: pic.as_deref(),
            link_type: &json_str(body, "type"),
            link_sorting: json_i32(body, "sorting"),
            did: json_i32(body, "did"),
            tem_type: json_i32(body, "tem_type"),
            img_type: phototype,
            now: clock::now_ts(),
        },
    )
    .await?;
    friend_link_service::invalidate_all().await;
    let verb = if id > 0 {
        msg_t("wap_js_00073")
    } else {
        msg_t("wap_js_00091")
    };
    let msg = format!(
        "{}{n}{}{verb}{}",
        msg_t("model_00211"),
        msg_t("model_00130"),
        msg_t("wap_js_00104"),
    );
    Ok(PhpOut::Text("wap_js_00104", msg))
}

fn php_flag_filter(body: &Value, key: &str) -> Option<i32> {
    match body.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.trim().is_empty() => None,
        _ => {
            let n = json_i32(body, key);
            if n == 2 {
                Some(0)
            } else if n > 0 {
                Some(n)
            } else {
                None
            }
        }
    }
}

fn php_days_ago(body: &Value, key: &str) -> Option<i64> {
    let n = json_i32(body, key);
    if n <= 0 {
        return None;
    }
    let now = clock::now_ts();
    if n == 1 {
        Some(now - (now % 86_400))
    } else {
        Some(now - i64::from(n) * 86_400)
    }
}

fn strip_a_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        if rest[start..].to_ascii_lowercase().starts_with("<a") {
            if let Some(gt) = rest[start..].find('>') {
                rest = &rest[start + gt + 1..];
                if let Some(end) = rest.to_ascii_lowercase().find("</a>") {
                    out.push_str(&rest[..end]);
                    rest = &rest[end + 4..];
                    continue;
                }
            }
        }
        if let Some(gt) = rest[start..].find('>') {
            rest = &rest[start + gt + 1..];
        } else {
            rest = &rest[start + 1..];
        }
    }
    out.push_str(rest);
    out.replace("\r\n", "<br/>")
        .replace('\n', "<br/>")
        .replace('\r', "<br/>")
}

fn admin_nav_json(r: &rbac_php::PhpAdminNavRow, has_children: bool) -> Value {
    json!({
        "id": r.id,
        "keyid": r.keyid,
        "name": r.name,
        "url": r.url,
        "path": r.path,
        "classname": r.classname,
        "menu": r.menu.to_string(),
        "sort": r.sort.to_string(),
        "display": r.display.to_string(),
        "dids": r.dids.to_string(),
        "hasChildren": has_children,
    })
}

fn admin_nav_tree(rows: &[rbac_php::PhpAdminNavRow], keyid: i64) -> Vec<Value> {
    rows.iter()
        .filter(|r| r.keyid == keyid)
        .map(|r| {
            let children = admin_nav_tree(rows, r.id);
            let mut v = admin_nav_json(r, !children.is_empty());
            if !children.is_empty() {
                v["children"] = Value::Array(children);
            }
            v
        })
        .collect()
}

const PHP_MODULE_KEYS: &[(&str, &str)] = &[
    ("job", "找工作"),
    ("resume", "找人才"),
    ("part", "兼职"),
    ("company", "找企业"),
    ("wap", "手机端"),
    ("article", "资讯"),
    ("announcement", "公告"),
    ("hr", "工具箱"),
    ("zph", "招聘会"),
    ("ask", "问答"),
    ("evaluate", "测评"),
    ("once", "店铺招聘"),
    ("tiny", "普工简历"),
    ("redeem", "商城"),
    ("map", "地图"),
    ("special", "专题招聘"),
    ("login", "登录"),
    ("register", "注册"),
    ("gongzhao", "公招"),
];

async fn role_user_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let m_id = json_present_i32(body, "m_id").filter(|n| *n > 0);
    let db = state.db.reader();
    let rows = rbac_php::php_list_users(db, if kw.is_empty() { None } else { Some(&kw) }, m_id, offset, limit).await?;
    let total = rbac_php::php_count_users(db, if kw.is_empty() { None } else { Some(&kw) }, m_id).await?;
    let cfg = settings_hash(state).await.unwrap_or_default();
    let groups = rbac_php::php_list_groups(db, 0, 500).await?;
    let group: Vec<Value> = groups
        .into_iter()
        .map(|g| json!({ "id": g.id, "group_name": g.group_name, "group_type": g.group_type }))
        .collect();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let parts: Vec<&str> = r.control_login.split(" - ").collect();
            json!({
                "uid": r.uid,
                "username": r.username,
                "group_name": r.group_name,
                "name": r.name,
                "mobile": r.mobile,
                "weixin": r.weixin,
                "qq": r.qq,
                "num": r.num,
                "call_num": r.call_num,
                "tuoxin_num": r.tuoxin_num,
                "follow_num": r.follow_num,
                "deal_num": r.deal_num,
                "month_deal_num": r.month_deal_num,
                "jobtai_ranking": r.jobtai_ranking,
                "is_crm": r.is_crm.to_string(),
                "isdid": r.isdid.to_string(),
                "m_id": r.m_id.to_string(),
                "crm_city": if r.crm_city.is_empty() { json!([]) } else { json!(r.crm_city.split(',').collect::<Vec<_>>()) },
                "crm_duty": if r.crm_duty.is_empty() { json!([]) } else { json!(r.crm_duty.split(',').collect::<Vec<_>>()) },
                "control_login": if r.control_login.is_empty() { json!("") } else { json!(parts) },
                "login_start": parts.first().copied().unwrap_or(""),
                "login_end": parts.get(1).copied().unwrap_or(""),
                "index_lookstatistc": r.index_lookstatistc.to_string(),
                "photo": checkpic_url(&cfg, &r.photo),
                "ewm": checkpic_url(&cfg, &r.ewm),
            })
        })
        .collect();
    let mut data = paged(Value::Array(list), total, page, per);
    data["group"] = Value::Array(group);
    data["week"] = json!({
        "1": "admin_system_00031",
        "2": "admin_system_00033",
        "3": "admin_system_00032",
        "4": "admin_system_00036",
        "5": "admin_system_00034",
        "6": "admin_system_00035",
        "7": "admin_system_00037",
    });
    Ok(data)
}

async fn role_user_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let username = json_str(body, "username");
    if username.is_empty() {
        return Err(ApiError::business("admin_system_00010"));
    }
    let uid = json_u64(body, "uid");
    if rbac_php::php_username_taken(state.db.pool(), &username, uid).await? {
        return Err(ApiError::business("admin_system_00027"));
    }
    let password = json_str(body, "password");
    let password_hash = if password.is_empty() {
        None
    } else {
        Some(md5_hex(&md5_hex(&password)))
    };
    let login_start = json_str(body, "login_start");
    let login_end = json_str(body, "login_end");
    let control_login = if !login_start.is_empty() && !login_end.is_empty() {
        format!("{login_start} - {login_end}")
    } else {
        json_str(body, "control_login")
    };
    let photo = json_str(body, "photo");
    let ewm = json_str(body, "ewm");
    let n = rbac_php::php_save_user(
        state.db.pool(),
        rbac_php::PhpAdminUserSave {
            uid: if uid > 0 { Some(uid) } else { None },
            username: &username,
            name: &json_str(body, "name"),
            m_id: json_i32(body, "m_id"),
            mobile: &json_str(body, "mobile"),
            weixin: &json_str(body, "weixin"),
            qq: &json_str(body, "qq"),
            is_crm: json_i32(body, "is_crm"),
            num: &json_str(body, "num"),
            call_num: &json_str(body, "call_num"),
            tuoxin_num: &json_str(body, "tuoxin_num"),
            follow_num: &json_str(body, "follow_num"),
            deal_num: &json_str(body, "deal_num"),
            month_deal_num: &json_str(body, "month_deal_num"),
            jobtai_ranking: json_i32(body, "jobtai_ranking"),
            crm_duty: &json_csv(body, "crm_duty"),
            crm_city: &json_csv(body, "crm_city"),
            photo: if photo.is_empty() { None } else { Some(photo.as_str()) },
            ewm: if ewm.is_empty() { None } else { Some(ewm.as_str()) },
            control_login: &control_login,
            index_lookstatistc: json_i32(body, "index_lookstatistc"),
            isdid: json_i32(body, "isdid"),
            password_hash: password_hash.as_deref(),
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_06357"));
    }
    Ok(PhpOut::Message(if uid > 0 { "wap_js_00073" } else { "wap_js_00091" }))
}

async fn role_user_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "del").max(json_u64(body, "id"));
    if uid == 0 {
        return Err(ApiError::business("common_01237"));
    }
    if uid == user.uid {
        return Err(ApiError::business("common_00747"));
    }
    let n = rbac_php::php_delete_user(state.db.pool(), uid).await?;
    if n == 0 {
        return Err(ApiError::business("model_00137"));
    }
    Ok(PhpOut::Message("model_00112"))
}

async fn role_ugroup_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let db = state.db.reader();
    let rows = rbac_php::php_list_groups(db, offset, limit).await?;
    let total = rbac_php::php_count_groups(db).await?;
    let type1 = msg_t("admin_system_00029");
    let type2 = msg_t("admin_system_00028");
    let list: Vec<Value> = rows
        .into_iter()
        .map(|g| {
            json!({
                "id": g.id,
                "group_name": g.group_name,
                "group_type": g.group_type,
                "group_type_n": if g.group_type == 1 { type1.clone() } else { type2.clone() },
                "num": g.num,
                "did": g.did,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn role_ugroup_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let rows = rbac_php::php_list_admin_nav(state.db.reader(), None).await?;
    let mut navigation = Vec::new();
    let mut one_menu = serde_json::Map::new();
    let mut two_menu = serde_json::Map::new();
    let mut three_menu = serde_json::Map::new();
    let mut one_children = serde_json::Map::new();
    let mut two_children = serde_json::Map::new();
    let top: Vec<i64> = rows.iter().filter(|r| r.keyid == 0).map(|r| r.id).collect();
    for r in &rows {
        if r.keyid == 0 {
            navigation.push(json!({
                "id": r.id, "name": r.name, "classname": r.classname, "sort": r.sort
            }));
        }
    }
    let one_ids: Vec<i64> = rows
        .iter()
        .filter(|r| top.contains(&r.keyid))
        .map(|r| r.id)
        .collect();
    for r in &rows {
        if top.contains(&r.keyid) {
            one_menu
                .entry(r.keyid.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!({"id": r.id, "keyid": r.keyid, "name": r.name, "url": r.url, "path": r.path}));
            one_children
                .entry(r.keyid.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!(r.id));
        }
    }
    let two_ids: Vec<i64> = rows
        .iter()
        .filter(|r| one_ids.contains(&r.keyid))
        .map(|r| r.id)
        .collect();
    for r in &rows {
        if one_ids.contains(&r.keyid) {
            two_menu
                .entry(r.keyid.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!({"id": r.id, "keyid": r.keyid, "name": r.name, "url": r.url, "path": r.path}));
            two_children
                .entry(r.keyid.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!(r.id));
        }
        if two_ids.contains(&r.keyid) {
            three_menu
                .entry(r.keyid.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!({"id": r.id, "keyid": r.keyid, "name": r.name, "url": r.url, "path": r.path}));
        }
    }
    let mut power = Vec::new();
    let mut group = json!({});
    if id > 0 {
        if let Some(g) = rbac_php::php_get_group(state.db.reader(), id).await? {
            power = php_power::parse_group_power(&g.group_power);
            group = json!({ "id": g.id, "group_name": g.group_name, "group_type": g.group_type, "did": g.did });
        }
    }
    let power_set: std::collections::HashSet<i64> = power.iter().copied().collect();
    let mut checked_three = Vec::new();
    let mut checked_four = Vec::new();
    for r in &rows {
        if one_ids.contains(&r.keyid) && power_set.contains(&r.id) {
            checked_three.push(r.id);
        }
        if two_ids.contains(&r.keyid) && power_set.contains(&r.id) {
            checked_four.push(r.id);
        }
    }
    Ok(json!({
        "one_menu": one_menu,
        "two_menu": two_menu,
        "three_menu": three_menu,
        "navigation": navigation,
        "one_children_ids": one_children,
        "two_children_ids": two_children,
        "checked_three_ids": checked_three,
        "checked_four_ids": checked_four,
        "admin_group": group,
        "power": power,
    }))
}

async fn role_ugroup_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "group_name");
    if name.is_empty() {
        return Err(ApiError::business("admin_system_00010"));
    }
    let mut ids = ids_named(body, "one_ids");
    ids.extend(ids_named(body, "two_ids"));
    ids.extend(ids_named(body, "three_ids"));
    ids.extend(ids_named(body, "four_ids"));
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::business("admin_system_00009"));
    }
    let gid = json_u64(body, "groupid");
    if rbac_php::php_group_name_taken(state.db.pool(), &name, gid).await? {
        return Err(ApiError::business("admin_system_00027"));
    }
    let power_i: Vec<i64> = ids.iter().map(|n| *n as i64).collect();
    let ser = php_power::serialize_group_power(&power_i);
    let n = rbac_php::php_save_group(state.db.pool(), if gid > 0 { Some(gid) } else { None }, &name, &ser, 0, 1).await?;
    if n == 0 {
        return Err(ApiError::business("common_06357"));
    }
    if name == "admin_system_00011" || name == msg_t("admin_system_00011") {
        let _ = rbac_php::php_mark_admin_nav_dids(state.db.pool(), &power_i, 1).await;
    }
    Ok(PhpOut::Message("common_06360"))
}

async fn role_ugroup_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("common_01237"));
    }
    if rbac_php::php_count_group_users(state.db.pool(), id).await? > 0 {
        return Err(ApiError::business("common_00307"));
    }
    recycle_ids(state, user, "admin_user_group", &[id], "/v1/admin/php-content/role-ugroup/delete").await;
    let n = rbac_php::php_delete_group(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("model_00137"));
    }
    Ok(PhpOut::Message("model_00112"))
}

async fn role_myuser_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    let row = rbac_php::php_get_user(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    let group = rbac_php::php_get_group(state.db.reader(), row.m_id as u64)
        .await?
        .map(|g| g.group_name)
        .unwrap_or_default();
    let last = phpyun_models::admin_rbac::repo::user_lasttime(state.db.reader(), user.uid).await?;
    let cfg = settings_hash(state).await.unwrap_or_default();
    let qy = rbac_php::php_qy_userid(state.db.reader(), user.uid).await?;
    let agent = {
        let a = cfg_pick(&cfg, "wx_photo_agentId");
        if a.is_empty() {
            cfg_pick(&cfg, "wx_qy_agentid")
        } else {
            a
        }
    };
    let qw_state = "weblogin@phpyun";
    Ok(json!({
        "username": row.username,
        "mobile": row.mobile,
        "real_name": row.name,
        "wxid": row.wxid,
        "qy_wxid": qy,
        "last_login": if last > 0 { fmt_dt(last) } else { String::new() },
        "group_name": group,
        "qy_app_id": cfg_pick(&cfg, "wx_qy_corpid"),
        "agent_id": agent,
        "redirect_uri": "",
        "state": qw_state,
    }))
}

async fn role_myuser_del_qy(state: &AppState, user: &AuthenticatedUser) -> AppResult<PhpOut> {
    let qy = rbac_php::php_qy_userid(state.db.pool(), user.uid).await?;
    if qy.is_empty() {
        return Err(ApiError::business("admin_system_00023"));
    }
    let n = rbac_php::php_clear_qy(state.db.pool(), user.uid).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01380"));
    }
    Ok(PhpOut::Message("admin_01379"))
}

async fn admin_nav_index(state: &AppState, body: &Value) -> AppResult<Value> {
    if body.get("keyid").is_some() {
        let keyid = json_i64(body, "keyid");
        let rows = rbac_php::php_list_admin_nav(state.db.reader(), Some(keyid)).await?;
        let ids: Vec<i64> = rows.iter().map(|r| r.id).collect();
        let kids = rbac_php::php_admin_nav_child_keyids(state.db.reader(), &ids).await?;
        let set: std::collections::HashSet<i64> = kids.into_iter().collect();
        let list: Vec<Value> = rows
            .iter()
            .map(|r| admin_nav_json(r, set.contains(&r.id)))
            .collect();
        return Ok(json!({ "list": list }));
    }
    let rows = rbac_php::php_list_admin_nav(state.db.reader(), None).await?;
    Ok(json!({ "list": admin_nav_tree(&rows, 0) }))
}

async fn admin_nav_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_i64(body, "id");
    let info = rbac_php::php_get_admin_nav(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    Ok(json!({ "info": admin_nav_json(&info, false) }))
}

async fn admin_nav_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_i64(body, "id");
    let n = rbac_php::php_save_admin_nav(
        state.db.pool(),
        rbac_php::PhpAdminNavSave {
            id: if id > 0 { Some(id) } else { None },
            keyid: json_i64(body, "keyid"),
            name: &json_str(body, "name"),
            url: &json_str(body, "url"),
            path: &json_str(body, "path"),
            classname: &json_str(body, "classname"),
            display: json_i32(body, "display"),
            dids: json_i32(body, "dids"),
            sort: json_i32(body, "sort"),
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business(if id > 0 { "admin_01364" } else { "admin_01366" }));
    }
    Ok(PhpOut::Message(if id > 0 { "admin_01363" } else { "admin_01365" }))
}

async fn admin_nav_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let id = json_i64(body, "id");
    if id == 0 {
        return Err(ApiError::business("common_01237"));
    }
    if rbac_php::php_count_admin_nav_children(state.db.pool(), id).await? > 0 {
        return Err(ApiError::business("model_00031"));
    }
    recycle_ids(state, user, "admin_navigation", &[id as u64], "/v1/admin/php-content/admin-nav/delete").await;
    let n = rbac_php::php_delete_admin_nav(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Message("model_00032"))
}

async fn admin_nav_change_display(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_i64(body, "id");
    let field = json_str(body, "field");
    let n = rbac_php::php_set_admin_nav_field(state.db.pool(), id, &field, json_i32(body, "status")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_model_00084"));
    }
    Ok(PhpOut::Message("admin_model_00083"))
}

async fn admin_nav_version(state: &AppState) -> AppResult<Value> {
    let rows = rbac_php::php_list_versions(state.db.reader()).await.unwrap_or_default();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "version": r.version,
                "code": r.code,
                "ctime": r.ctime,
                "ctime_n": if r.ctime > 0 { fmt_dt(r.ctime) } else { String::new() },
            })
        })
        .collect();
    Ok(json!({ "list": list }))
}

async fn front_nav_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let f = nav_php::PhpNavFilter {
        type_eq: json_present_i32(body, "type"),
        eject: php_flag_filter(body, "eject"),
        display: php_flag_filter(body, "display"),
        nid: json_present_i32(body, "nid"),
        keyword: {
            let s = json_str(body, "keyword");
            if s.is_empty() { None } else { Some(s) }
        },
    };
    let order = json_str(body, "order");
    let t = json_str(body, "t");
    let db = state.db.reader();
    let rows = nav_php::php_list_nav(db, &f, offset, limit, &t, &order).await?;
    let total = nav_php::php_count_nav(db, &f).await?;
    let types = nav_php::php_list_nav_types(db).await?;
    let mut nclass = serde_json::Map::new();
    let mut type_map = std::collections::HashMap::new();
    for ty in &types {
        nclass.insert(ty.id.to_string(), json!(ty.typename));
        type_map.insert(ty.id as i32, ty.typename.clone());
    }
    let type1 = msg_t("admin_00198");
    let type2 = msg_t("admin_system_00663");
    let nav: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "nid": r.nid.to_string(),
                "name": r.name,
                "url": r.url,
                "furl": r.furl,
                "sort": r.sort.to_string(),
                "display": r.display.to_string(),
                "eject": r.eject.to_string(),
                "type": r.r#type.to_string(),
                "type_n": if r.r#type == 1 { type1.clone() } else { type2.clone() },
                "typename": type_map.get(&r.nid).cloned().unwrap_or_default(),
                "color": r.color,
                "model": r.model,
                "bold": r.bold.to_string(),
                "pic": r.pic,
                "config": r.config,
            })
        })
        .collect();
    let mut data = paged(Value::Array(nav.clone()), total, page, per);
    data["nav"] = Value::Array(nav);
    data["nclass"] = Value::Object(nclass);
    Ok(data)
}

async fn front_nav_add(state: &AppState, body: &Value) -> AppResult<Value> {
    let types = nav_php::php_list_nav_types(state.db.reader()).await?;
    let type_v: Vec<Value> = types
        .into_iter()
        .map(|t| json!({ "id": t.id, "typename": t.typename }))
        .collect();
    let mut info = json!({});
    let id = json_u64(body, "id");
    if id > 0 {
        if let Some(r) = nav_php::php_get_nav(state.db.reader(), id).await? {
            info = json!({
                "id": r.id, "nid": r.nid.to_string(), "name": r.name, "url": r.url, "furl": r.furl,
                "sort": r.sort.to_string(), "display": r.display.to_string(), "eject": r.eject.to_string(),
                "type": r.r#type.to_string(), "color": r.color, "model": r.model, "bold": r.bold.to_string(),
                "pic": r.pic, "config": r.config,
            });
        }
    }
    Ok(json!({ "type": type_v, "info": info, "picMaxSize": 5, "picType": "jpg,png,jpeg,bmp,gif" }))
}

async fn front_nav_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let name = json_str(body, "name");
    let nid = json_i32(body, "nid");
    if id == 0 && nav_php::php_nav_name_taken(state.db.pool(), &name, nid).await? {
        return Err(ApiError::business("admin_neirong_00021"));
    }
    let url = json_str(body, "url").replace("amp;", "");
    let pic = json_str(body, "pic");
    let n = nav_php::php_save_nav(
        state.db.pool(),
        nav_php::PhpNavSave {
            id: if id > 0 { Some(id) } else { None },
            nid,
            eject: json_i32(body, "eject"),
            display: json_i32(body, "display"),
            name: &name,
            url: &url,
            furl: &json_str(body, "furl"),
            sort: json_i32(body, "sort"),
            color: &json_str(body, "color"),
            model: &json_str(body, "model"),
            bold: json_i32(body, "bold"),
            r#type: json_i32(body, "type"),
            pic: if pic.is_empty() { None } else { Some(pic.as_str()) },
            config: &json_str(body, "config"),
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business(if id > 0 { "admin_01389" } else { "admin_01390" }));
    }
    Ok(PhpOut::Message(if id > 0 { "admin_model_00085" } else { "admin_model_00086" }))
}

async fn front_nav_clear_links(state: &AppState, rows: &[nav_php::PhpNavRow]) {
    let desc_ids: Vec<u64> = rows.iter().filter_map(|r| r.desc.parse().ok()).filter(|n: &u64| *n > 0).collect();
    let news_ids: Vec<u64> = rows.iter().filter_map(|r| r.news.parse().ok()).filter(|n: &u64| *n > 0).collect();
    let _ = nav_php::php_clear_desc_menu(state.db.pool(), &desc_ids).await;
    for id in news_ids {
        let _ = article_repo::set_group_is_menu(state.db.pool(), id, 0).await;
    }
}

async fn front_nav_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01063"));
    }
    let rows = nav_php::php_navs_with_links(state.db.pool(), &ids).await.unwrap_or_default();
    front_nav_clear_links(state, &rows).await;
    recycle_ids(state, user, "navigation", &ids, "/v1/admin/php-content/front-nav/delete").await;
    let n = nav_php::php_delete_navs(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Message("admin_model_00087"))
}

async fn front_nav_navset(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let field = json_str(body, "type");
    let n = nav_php::php_set_nav_field(state.db.pool(), id, &field, json_i32(body, "rec")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01388"));
    }
    Ok(PhpOut::Message(if field == "display" { "admin_model_00088" } else { "admin_model_00089" }))
}

async fn front_nav_navsort(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let n = nav_php::php_set_nav_field(state.db.pool(), id, "sort", json_i32(body, "sort")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01392"));
    }
    Ok(PhpOut::Message("admin_model_00090"))
}

async fn front_nav_type(state: &AppState) -> AppResult<Value> {
    let list = nav_php::php_list_nav_types(state.db.reader()).await?;
    Ok(json!({ "list": list }))
}

async fn front_nav_typeadd(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "typename");
    if name.is_empty() {
        return Err(ApiError::business("admin_01393"));
    }
    if nav_php::php_nav_type_taken(state.db.pool(), &name).await? {
        return Err(ApiError::business("admin_system_00049"));
    }
    let n = nav_php::php_add_nav_type(state.db.pool(), &name).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01394"));
    }
    Ok(PhpOut::Message("admin_model_00091"))
}

async fn front_nav_typename(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "typename");
    if name.is_empty() {
        return Err(ApiError::business("admin_01393"));
    }
    let id = json_u64(body, "id");
    let n = nav_php::php_up_nav_type(state.db.pool(), id, &name).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01395"));
    }
    Ok(PhpOut::Message("admin_model_00092"))
}

async fn front_nav_typedel(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("member_com_00320"));
    }
    let rows = nav_php::php_navs_by_nid_with_links(state.db.pool(), id as i32).await.unwrap_or_default();
    front_nav_clear_links(state, &rows).await;
    recycle_ids(state, user, "navigation_type", &[id], "/v1/admin/php-content/front-nav/typedel").await;
    let _ = nav_php::php_delete_navs_by_nid(state.db.pool(), id as i32).await;
    let n = nav_php::php_delete_nav_type(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01396"));
    }
    Ok(PhpOut::Message("admin_model_00093"))
}

async fn navmap_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let f = gap_repo::PhpNavmapFilter {
        keyword: {
            let s = json_str(body, "keyword");
            if s.is_empty() { None } else { Some(s) }
        },
        ktype: json_present_i32(body, "type"),
        eject: php_flag_filter(body, "eject"),
        display: php_flag_filter(body, "display"),
        ctype: json_present_i32(body, "ctype"),
    };
    let db = state.db.reader();
    let rows = gap_repo::php_list_navmap(db, &f, offset, limit).await?;
    let total = gap_repo::php_count_navmap(db, &f).await?;
    let mut names: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    for r in &rows {
        names.insert(r.id as i32, r.name.clone());
    }
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "nid": r.nid,
                "name": r.name,
                "url": r.url,
                "furl": r.furl,
                "sort": r.sort,
                "display": r.display,
                "display_n": r.display == 1,
                "eject": r.eject,
                "type": r.r#type,
                "typename": names.get(&r.nid).cloned().unwrap_or_default(),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn navmap_get_types(state: &AppState) -> AppResult<Value> {
    let rows = gap_repo::php_list_navmap_types(state.db.reader()).await?;
    let type_v: Vec<Value> = rows
        .into_iter()
        .map(|(id, name)| json!({ "label": name, "value": id }))
        .collect();
    Ok(json!({ "type": type_v }))
}

async fn navmap_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let n = gap_repo::upsert_navmap(
        state.db.pool(),
        if id > 0 { Some(id) } else { None },
        json_i32(body, "nid"),
        &json_str(body, "name"),
        &json_str(body, "url").replace("amp;", ""),
        json_i32(body, "sort"),
        json_i32(body, "display"),
        json_i32(body, "eject"),
        json_i32(body, "type"),
        &json_str(body, "furl"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_model_00097"));
    }
    Ok(PhpOut::Message(if id > 0 { "admin_model_00094" } else { "admin_model_00095" }))
}

async fn navmap_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    recycle_ids(state, user, "navmap", &ids, "/v1/admin/php-content/navmap/delete").await;
    let n = gap_repo::delete_navmap(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Message("admin_model_00087"))
}

async fn navmap_xianshi(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let n = gap_repo::php_set_navmap_field(state.db.pool(), id, &json_str(body, "type"), json_i32(body, "rec")).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01388"));
    }
    Ok(PhpOut::Message("ok"))
}

async fn set_module_index(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let mut module = serde_json::Map::new();
    for (key, label) in PHP_MODULE_KEYS {
        module.insert(
            (*key).to_string(),
            json!({
                "value": label,
                "web": cfg.get(&format!("sy_{key}_web")).cloned().unwrap_or_default(),
                "ssl": cfg.get(&format!("sy_{key}ssl")).cloned().unwrap_or_default(),
                "domain": cfg.get(&format!("sy_{key}domain")).cloned().unwrap_or_default(),
                "dir": cfg.get(&format!("sy_{key}dir")).cloned().unwrap_or_default(),
            }),
        );
    }
    Ok(json!({ "module": module }))
}

async fn set_module_save(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let obj = match body {
        Value::Object(m) => m,
        _ => return Err(ApiError::param_invalid("param_invalid")),
    };
    for (key, _) in PHP_MODULE_KEYS {
        let Some(item) = obj.get(*key) else { continue };
        let web = json_i32(item, "web");
        let ssl = json_i32(item, "ssl");
        let domain = json_str(item, "domain");
        let dir = json_str(item, "dir");
        let display = if web == 1 { 1 } else { 0 };
        let _ = nav_php::php_set_nav_display_by_config(state.db.pool(), key, display).await;
        let ssl_s = if ssl == 0 || domain.is_empty() { "0" } else { "1" };
        for (k, v) in [
            (format!("sy_{key}_web"), web.to_string()),
            (format!("sy_{key}ssl"), ssl_s.to_string()),
            (format!("sy_{key}domain"), domain.clone()),
            (format!("sy_{key}dir"), dir),
        ] {
            setting_repo::upsert(state.db.pool(), &k, &v, "", true, clock::now_ts()).await?;
        }
        let _ = user;
    }
    Ok(PhpOut::Message("admin_01386"))
}

async fn set_module_navset(state: &AppState, body: &Value) -> AppResult<Value> {
    let config = json_str(body, "config");
    let types = nav_php::php_list_nav_types(state.db.reader()).await?;
    let type_v: Vec<Value> = types
        .into_iter()
        .map(|t| json!({ "id": t.id, "typename": t.typename }))
        .collect();
    let nav = if let Some(r) = nav_php::php_get_nav_by_config(state.db.reader(), &config).await? {
        json!({
            "id": r.id, "nid": r.nid.to_string(), "name": r.name, "url": r.url,
            "sort": r.sort.to_string(), "display": r.display.to_string(), "eject": r.eject.to_string(),
            "model": r.model, "bold": r.bold.to_string(), "config": r.config,
        })
    } else {
        json!({ "name": json_str(body, "name"), "config": config, "nid": "1" })
    };
    Ok(json!({ "type": type_v, "nav": nav }))
}

async fn set_module_navset_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let config = json_str(body, "config");
    let cfg = settings_hash(state).await.unwrap_or_default();
    let url = cfg.get(&format!("sy_{config}dir")).cloned().unwrap_or_default();
    let id = json_u64(body, "id");
    let n = nav_php::php_save_nav(
        state.db.pool(),
        nav_php::PhpNavSave {
            id: if id > 0 { Some(id) } else { None },
            nid: json_i32(body, "nid"),
            eject: json_i32(body, "eject"),
            display: json_i32(body, "display"),
            name: &json_str(body, "name"),
            url: &url,
            furl: "",
            sort: json_i32(body, "sort"),
            color: "",
            model: &json_str(body, "model"),
            bold: json_i32(body, "bold"),
            r#type: 1,
            pic: None,
            config: &config,
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_01388"));
    }
    Ok(PhpOut::Message("admin_01387"))
}

async fn set_module_getseo(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let seo = seo_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("not_found"))?;
    Ok(json!({
        "seoname": seo.seoname,
        "ident": seo.ident,
        "rewrite_url": seo.rewrite_url,
        "php_url": seo.php_url,
        "title": seo.title,
        "keywords": seo.keywords,
        "description": seo.description,
        "did": seo.did,
        "php_wap_url": seo.php_wap_url,
        "rewrite_wap_url": seo.rewrite_wap_url,
    }))
}

async fn set_module_seoshezhi(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id > 0 && body.get("title").is_some() {
        seo_repo::upsert(
            state.db.pool(),
            id,
            &json_str(body, "seoname"),
            &json_str(body, "ident"),
            &json_str(body, "seomodel"),
            &json_str(body, "title"),
            &json_str(body, "keywords"),
            &json_str(body, "php_url"),
            &json_str(body, "rewrite_url"),
            &json_str(body, "php_wap_url"),
            &json_str(body, "rewrite_wap_url"),
            &json_str(body, "description"),
            json_i32(body, "did"),
            clock::now_ts(),
        )
        .await?;
        return Ok(PhpOut::Message("admin_model_00217"));
    }
    let config = json_str(body, "config");
    let seo = seo_repo::list_by_model(state.db.reader(), &config).await?;
    let domains = phpyun_models::domain::repo::list_all(state.db.reader()).await.unwrap_or_default();
    Ok(PhpOut::Data(json!({
        "seo": seo,
        "Dname": domain_object(&domains),
        "seoconfig": { "public": {
            "webname": "网站名称", "webkeyword": "网站关键字", "webdesc": "网站描述", "weburl": "网址"
        }}
    })))
}

async fn sysmsg_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = gap_repo::PhpSysmsgFilter {
        keyword: if kw.is_empty() { None } else { Some(kw) },
        ktype: json_present_i32(body, "type"),
        ctime_from: php_days_ago(body, "end").or_else(|| php_days_ago(body, "ectime")),
    };
    let db = state.db.reader();
    let rows = gap_repo::php_list_sysmsgs(db, &f, offset, limit).await?;
    let total = gap_repo::php_count_sysmsgs(db, &f).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "fa_uid": r.fa_uid,
                "username": r.username,
                "content": r.content,
                "content_all": strip_a_tags(&r.content),
                "usertype": r.usertype,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn sysmsg_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    recycle_ids(state, user, "sysmsg", &ids, "/v1/admin/php-content/sysmsg/delete").await;
    let n = gap_repo::php_delete_sysmsgs(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Message("model_00112"))
}

async fn sysmsg_send(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let utype = json_i32(body, "utype");
    if utype == 0 {
        return Err(ApiError::business("admin_system_00210"));
    }
    let content = json_str(body, "content");
    if content.is_empty() {
        return Err(ApiError::business("admin_system_00016"));
    }
    let now = clock::now_ts();
    let members = if utype == 5 {
        let names: Vec<String> = json_str(body, "userarr")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        gap_repo::php_members_by_usernames(state.db.pool(), &names).await?
    } else {
        let page = json_u64(body, "page").max(1);
        let offset = (page - 1) * 1000;
        gap_repo::php_list_members_by_usertype(state.db.pool(), utype, offset, 1000).await?
    };
    if members.is_empty() {
        return Err(ApiError::business("wap_js_00060"));
    }
    for (uid, username, ut) in &members {
        gap_repo::php_insert_sysmsg(state.db.pool(), *uid, username, *ut, &content, now).await?;
    }
    if utype != 5 {
        let count = gap_repo::php_count_members_by_usertype(state.db.pool(), utype).await?;
        let page = json_u64(body, "page").max(1);
        let size = 1000u64;
        if count > page * size {
            return Ok(PhpOut::Text(
                "admin_system_00017",
                format!("{}{count}{}{}{}", msg_t("admin_system_00017"), page * size, msg_t("ok"), ""),
            ));
        }
    }
    Ok(PhpOut::Message("admin_system_00018"))
}

fn error_type_n(t: i32) -> String {
    let key = match t {
        1 => "wap_00242",
        2 => "admin_user_00296",
        3 => "wap_00794",
        4 => "wap_00322",
        5 => "common_06479",
        6 => "wap_00451",
        7 => "resume_00029",
        8 => "admin_user_00166",
        9 => "admin_user_00167",
        _ => return String::new(),
    };
    msg_t(key)
}

async fn error_log_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = gap_repo::PhpErrorLogFilter {
        keyword: if kw.is_empty() { None } else { Some(kw) },
        ktype: json_present_i32(body, "type"),
        logtype: json_present_i32(body, "logtype"),
        ctime_from: php_days_ago(body, "ctime"),
    };
    let db = state.db.reader();
    let rows = gap_repo::php_list_error_logs(db, &f, offset, limit).await?;
    let total = gap_repo::php_count_error_logs(db, &f).await?;
    let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
    let _ = gap_repo::php_mark_error_read(state.db.pool(), &ids).await;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "uid": r.uid,
                "type": r.r#type,
                "type_n": error_type_n(r.r#type),
                "content": r.content,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
                "isread": r.isread,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn error_log_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    if json_str(body, "id") == "all" {
        let n = gap_repo::delete_error_logs(state.db.pool(), &[]).await?;
        let _ = n;
        return Ok(PhpOut::Message("admin_01376"));
    }
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01237"));
    }
    recycle_ids(state, user, "error_log", &ids, "/v1/admin/php-content/error-log/delete").await;
    let n = gap_repo::delete_error_logs(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Message("admin_01376"))
}

async fn admin_log_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let mut ctime_from = php_days_ago(body, "end");
    let mut ctime_to = None;
    let time = json_str(body, "time");
    if !time.is_empty() {
        let parts: Vec<&str> = time.split('~').collect();
        if parts.len() == 2 {
            let a = clock::parse_site_date(parts[0].trim()).unwrap_or(0);
            let b = clock::parse_site_date(parts[1].trim()).unwrap_or(0);
            if a > 0 {
                ctime_from = Some(a);
            }
            if b > 0 {
                ctime_to = Some(b + 86_399);
            }
        }
    }
    let uk = json_str(body, "ukeyword");
    let kw = json_str(body, "keyword");
    let f = admin_msg_repo::PhpAdminLogFilter {
        ukeyword: if uk.is_empty() { None } else { Some(uk) },
        keyword: if kw.is_empty() { None } else { Some(kw) },
        ctime_from,
        ctime_to,
    };
    let db = state.db.reader();
    let rows = admin_msg_repo::php_list_admin_logs(db, &f, offset, limit).await?;
    let total = admin_msg_repo::php_count_admin_logs(db, &f).await?;
    let domains = phpyun_models::domain::repo::list_all(db).await.unwrap_or_default();
    let dmap = domain_object(&domains);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let did_name = dmap.get(&r.did.to_string()).cloned().unwrap_or(json!(""));
            json!({
                "id": r.id,
                "uid": r.uid,
                "username": r.username,
                "content": r.content,
                "ip": r.ip,
                "did": r.did,
                "did_name": did_name,
                "ctime": r.ctime,
                "ctime_n": fmt_dt(r.ctime),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

fn php_serialize_strs(items: &[String]) -> String {
    let mut inner = String::new();
    for (i, s) in items.iter().enumerate() {
        inner.push_str(&format!("i:{i};s:{}:\"{s}\";", s.len()));
    }
    format!("a:{}:{{{inner}}}", items.len())
}

fn parse_lastwork(s: &str) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    parse_date_ts(s.get(..10).unwrap_or(s))
}

fn cron_type_n(t: i32) -> String {
    msg_t(match t {
        1 => "admin_system_00268",
        2 => "admin_system_00270",
        3 => "admin_system_00269",
        4 => "admin_00894",
        5 => "admin_system_00261",
        _ => "",
    })
}

fn job_salary_n(min: i32, max: i32) -> String {
    if max > 0 {
        format!("{min}-{max}")
    } else if min > 0 {
        min.to_string()
    } else {
        String::new()
    }
}

async fn news_ajax_menu(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({}));
    }
    let Some(row) = article_repo::get_group_admin(state.db.reader(), id).await? else {
        return Ok(json!({}));
    };
    if row.is_menu == 1 {
        if let Some(nav) = nav_php::php_get_nav_by_news(state.db.reader(), id as i64).await? {
            let types = nav_php::php_list_nav_types(state.db.reader()).await?;
            let typename = types
                .iter()
                .find(|t| t.id == nav.nid as u64)
                .map(|t| t.typename.clone())
                .unwrap_or_default();
            return Ok(json!({
                "id": nav.id,
                "nid": nav.nid,
                "name": nav.name,
                "typename": typename,
                "color": nav.color,
                "url": nav.url,
                "furl": nav.furl,
                "type": nav.r#type,
                "sort": nav.sort,
                "eject": nav.eject,
                "model": nav.model,
                "bold": nav.bold,
                "display": nav.display,
            }));
        }
    }
    Ok(json!({
        "name": row.name,
        "url": format!("news/{}/", row.id),
        "furl": format!("article/c_list-nid_{}.html", row.id),
    }))
}

async fn news_set_menu(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if !has_flag(body, "submit") {
        return Err(ApiError::business("wap_01298"));
    }
    let name = json_str(body, "name");
    let nid = json_i32(body, "nid");
    let nav_id = json_u64(body, "id");
    let did = json_u64(body, "did");
    if name.is_empty() {
        return Err(ApiError::business("wap_01298"));
    }
    if nav_id == 0 && nav_php::php_nav_name_taken(state.db.pool(), &name, nid).await? {
        return Err(ApiError::business("admin_neirong_00018"));
    }
    let url = json_str(body, "url").replace("amp;", "");
    let pic = json_str(body, "pic");
    let saved = nav_php::php_save_nav(
        state.db.pool(),
        nav_php::PhpNavSave {
            id: if nav_id > 0 { Some(nav_id) } else { None },
            nid,
            eject: json_i32(body, "eject"),
            display: json_i32(body, "display"),
            name: &name,
            url: &url,
            furl: &json_str(body, "furl"),
            sort: json_i32(body, "sort"),
            color: &json_str(body, "color"),
            model: &json_str(body, "model"),
            bold: json_i32(body, "bold"),
            r#type: json_i32(body, "type"),
            pic: if pic.is_empty() { None } else { Some(pic.as_str()) },
            config: &json_str(body, "config"),
        },
    )
    .await?;
    if saved == 0 {
        return Err(ApiError::business(if nav_id > 0 {
            "admin_01389"
        } else {
            "admin_01390"
        }));
    }
    if did > 0 {
        nav_php::php_set_nav_news(state.db.pool(), saved, did as i64).await?;
        if nav_id == 0 {
            article_repo::set_group_is_menu(state.db.pool(), did, 1).await?;
        }
    }
    let key = if nav_id > 0 {
        "admin_01337"
    } else {
        "admin_01338"
    };
    Ok(PhpOut::Text(key, format!("{}{}", msg_t(key), msg_t("wap_js_00104"))))
}

async fn announce_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = announcement_repo::PhpAnnounceFilter {
        keyword: if kw.is_empty() { None } else { Some(kw) },
        since: php_days_ago(body, "end"),
    };
    let db = state.db.reader();
    let rows = announcement_repo::php_list_admin(db, &f, offset, limit).await?;
    let total = announcement_repo::php_count_admin(db, &f).await?;
    let domains = domain_repo::list_all(db).await.unwrap_or_default();
    let dmap = domain_object(&domains);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let dname = dmap.get(&r.did.to_string()).cloned().unwrap_or(json!(""));
            json!({
                "id": r.id,
                "title": r.title,
                "keyword": r.keyword,
                "description": r.description,
                "content": r.content,
                "view_num": r.view_num,
                "datetime": r.datetime,
                "datetime_n": fmt_dt(r.datetime),
                "startime": r.startime.to_string(),
                "startime_n": fmt_dt(r.startime),
                "endtime": r.endtime.to_string(),
                "endtime_n": fmt_dt(r.endtime),
                "did": r.did,
                "dname": dname,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn hotjob_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let rating_s = json_str(body, "rating");
    let rating_n = json_i32(body, "rating");
    let f = company_repo::PhpHotJobFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        ctype: json_i32(body, "ctype"),
        rating: if rating_n > 0 { Some(rating_n) } else { None },
        rating_name: if rating_s.parse::<i32>().is_ok() {
            None
        } else if rating_s.is_empty() {
            None
        } else {
            Some(rating_s.as_str())
        },
        time_mode: json_i32(body, "time"),
        now: clock::now_ts(),
    };
    let db = state.db.reader();
    let rows = company_repo::php_hotjob_list(db, &f, offset, limit).await?;
    let total = company_repo::php_hotjob_count(db, &f).await?;
    let cfg = settings_hash(state).await.unwrap_or_default();
    let now = clock::now_ts();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let expired = r.time_end > 0 && r.time_end < now;
            json!({
                "id": r.id,
                "uid": r.uid,
                "name": r.username,
                "username": r.username,
                "rating": r.rating,
                "rating_id": r.rating_id,
                "hot_pic": checkpic_url(&cfg, &r.hot_pic),
                "service_price": r.service_price,
                "time_start": r.time_start,
                "time_start_n": fmt_dt(r.time_start),
                "time_end": r.time_end,
                "time_end_n": if expired { "wap_com_00319".to_string() } else { fmt_dt(r.time_end) },
                "sort": r.sort,
                "beizhu": r.beizhu,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn cron_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let db = state.db.reader();
    let rows = gap_repo::list_cron(db, offset, limit).await?;
    let total = gap_repo::count_cron(db).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let src = format!("{base}/index.php?m=cron&id={}", r.id);
            json!({
                "id": r.id,
                "name": r.name,
                "dir": r.dir,
                "type": r.r#type,
                "type_n": cron_type_n(r.r#type),
                "week": r.week,
                "month": r.month,
                "hour": r.hour,
                "minute": r.minute,
                "display": r.display,
                "display_n": msg_t(if r.display == 1 { "common_02085" } else { "common_02063" }),
                "nowtime": r.nowtime,
                "nowtime_n": if r.nowtime > 0 { fmt_dt(r.nowtime) } else { "-".into() },
                "nexttime": r.nexttime,
                "nexttime_n": if r.nexttime > 0 { fmt_dt(r.nexttime) } else { "-".into() },
                "src": src,
                "waibu": src,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn zph_space_ajax(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    if body.get("sort").is_some() {
        zph_repo::patch_space_field(state.db.pool(), id, "sort", &json_str(body, "sort")).await?;
    }
    if body.get("name").is_some() {
        zph_repo::patch_space_field(state.db.pool(), id, "name", &json_str(body, "name")).await?;
    }
    if body.get("price").is_some() {
        zph_repo::patch_space_field(state.db.pool(), id, "price", &json_str(body, "price")).await?;
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

async fn zph_space_ajaxspace(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_i32(body, "id");
    if id == 0 {
        return Ok(json!([]));
    }
    let rows = zph_repo::list_spaces(state.db.reader(), Some(i64::from(id)), None).await?;
    Ok(serde_json::to_value(rows).unwrap_or(json!([])))
}

async fn zph_space_up(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_i32(body, "id");
    if id == 0 {
        return Ok(json!([]));
    }
    let Some(one) = zph_repo::find_space_by_id(state.db.reader(), id).await? else {
        return Ok(json!([]));
    };
    let two = zph_repo::list_spaces(state.db.reader(), Some(i64::from(id)), None).await?;
    let mut children = Vec::new();
    for row in two {
        let mut v = serde_json::to_value(&row).unwrap_or(json!({}));
        let grand = zph_repo::list_spaces(state.db.reader(), Some(row.id as i64), None).await?;
        if !grand.is_empty() {
            v["children"] = serde_json::to_value(grand).unwrap_or(json!([]));
        }
        children.push(v);
    }
    let mut one_v = serde_json::to_value(&one).unwrap_or(json!({}));
    one_v["children"] = Value::Array(children);
    Ok(json!([one_v]))
}

async fn report_delresume(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let eid = json_u64(body, "eid");
    let rid = json_u64(body, "id");
    if eid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let expect = expect_repo::find_by_id(state.db.reader(), eid).await?;
    let uid = expect
        .as_ref()
        .map(|e| e.uid)
        .filter(|u| *u > 0)
        .unwrap_or_else(|| json_u64(body, "uid"));
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    recycle_ids(
        state,
        user,
        "resume_expect",
        &[eid],
        "/v1/admin/php-content/report-resume/delresume",
    )
    .await;
    let n = expect_repo::delete(state.db.pool(), eid, uid).await?;
    if n == 0 {
        let msg = format!("{}{}{}", msg_t("admin_00387"), eid, msg_t("admin_01425"));
        return Ok(PhpOut::Text("admin_01425", msg));
    }
    if let Some(row) = report_repo::find_refund_row(state.db.reader(), rid).await? {
        if row.datafh != 1 {
            let _ = admin_report_service::refund_unpaid_resume_reports(state, &[row]).await;
        }
    }
    let msg = format!("{}{}{}", msg_t("admin_00387"), eid, msg_t("admin_01291"));
    Ok(PhpOut::Text("admin_01291", msg))
}

async fn report_delresumeall(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let rids = ids_named(body, "rid");
    if rids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let rows = report_repo::find_refund_rows(state.db.reader(), &rids).await?;
    let mut eids: Vec<u64> = rows.iter().map(|r| r.eid).filter(|e| *e > 0).collect();
    eids.sort_unstable();
    eids.dedup();
    if !eids.is_empty() {
        recycle_ids(
            state,
            user,
            "resume_expect",
            &eids,
            "/v1/admin/php-content/report-resume/delresumeall",
        )
        .await;
    }
    for eid in &eids {
        if let Some(ex) = expect_repo::find_by_id(state.db.reader(), *eid).await? {
            let _ = expect_repo::delete(state.db.pool(), *eid, ex.uid).await;
        }
    }
    let unpaid: Vec<_> = rows.into_iter().filter(|r| r.datafh != 1).collect();
    if !unpaid.is_empty() {
        let _ = admin_report_service::refund_unpaid_resume_reports(state, &unpaid).await;
    }
    let joined = eids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let msg = format!("{}{}{}", msg_t("admin_00387"), joined, msg_t("admin_01291"));
    Ok(PhpOut::Text("admin_01291", msg))
}

async fn shop_set_index(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let pic = cfg.get("sy_imgsc_mr").cloned().unwrap_or_default();
    Ok(json!({ "sy_imgsc_mr": checkpic_url(&cfg, &pic) }))
}

async fn shop_set_saveset(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let path = json_str(body, "sy_imgsc_mr");
    let url = json_str(body, "url");
    let picurl = json_str(body, "picurl");
    let val = [path, url, picurl]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or_default();
    if val.is_empty() {
        return Err(ApiError::business("ajax_00015"));
    }
    upsert_cfg(state, user, "sy_imgsc_mr", &val).await?;
    Ok(PhpOut::Message("admin_01437"))
}

async fn shop_set_redeem_option(state: &AppState, body: &Value) -> AppResult<Value> {
    let tnid = json_u64(body, "tnid");
    let mut html = format!("<option value=\"\">{}</option>", msg_t("admin_model_00228"));
    if tnid > 0 {
        let rows = redeem_repo::list_classes(state.db.reader(), Some(tnid)).await?;
        for c in rows {
            html.push_str(&format!("<option value='{}'>{}</option>", c.id, c.name));
        }
    }
    Ok(json!({ "html": html }))
}

async fn hbconfig_save_whb(
    state: &AppState,
    _user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::business("wap_01298"));
    }
    let pic = json_str(body, "pic");
    let id = json_u64(body, "id");
    let n = whb_repo::upsert_whb(
        state.db.pool(),
        whb_repo::WhbSave {
            id: if id > 0 { Some(id) } else { None },
            name: &name,
            pic: if pic.is_empty() { None } else { Some(pic.as_str()) },
            sort: json_i32(body, "sort"),
            isopen: json_i32(body, "isopen"),
            r#type: json_i32(body, "type"),
            num: json_i32(body, "num"),
            style: json_i32(body, "style"),
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_06357"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

async fn hbconfig_del_whb(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    recycle_ids(
        state,
        user,
        "admin_jobwhb",
        &ids,
        "/v1/admin/php-content/hbconfig/delWhb",
    )
    .await;
    let mut n = 0u64;
    for id in &ids {
        n += whb_repo::delete_whb(state.db.pool(), *id).await?;
    }
    if n == 0 {
        return Err(ApiError::business("model_00033"));
    }
    Ok(PhpOut::Text(
        "admin_model_00242",
        format!(
            "{}{}{}",
            msg_t("admin_model_00242").split('(').next().unwrap_or(""),
            ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","),
            msg_t("admin_01291")
        ),
    ))
}

async fn hrlog_editsave(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let mut fields: Vec<(&str, i64)> = Vec::new();
    for key in [
        "job", "lookjob", "lookresume", "sqjob", "yq", "login", "nightwork", "chatnum", "chatuser",
    ] {
        if body.get(key).is_some() {
            fields.push((key, i64::from(json_i32(body, key))));
        }
    }
    if body.get("lastwork").is_some() {
        fields.push(("lastwork", parse_lastwork(&json_str(body, "lastwork"))));
    }
    let n = gap_repo::php_update_hr_log(state.db.pool(), id, &fields, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01453"));
    }
    Ok(PhpOut::Text(
        "admin_model_00003",
        format!("{}{}", msg_t("admin_model_00003"), ""),
    ))
}

async fn hrlog_rehrlog(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    if gap_repo::php_get_hr_log(state.db.reader(), id).await?.is_none() {
        return Err(ApiError::business("admin_01453"));
    }
    let n = gap_repo::php_update_hr_log(state.db.pool(), id, &[], clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01453"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

async fn hrlog_set(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await?;
    let isopen = cfg.get("sy_yearreport_isopen").cloned().unwrap_or_else(|| "0".into());
    Ok(json!({
        "set": {
            "sy_yearreport_isopen": if isopen.is_empty() { "0".into() } else { isopen },
            "sy_yearreport_ewmtype": cfg.get("sy_yearreport_ewmtype").cloned().unwrap_or_default(),
            "sy_yearreport_tip_n": checkpic_url(&cfg, cfg.get("sy_yearreport_tip").map(|s| s.as_str()).unwrap_or("")),
            "sy_yearreport_pic_n": checkpic_url(&cfg, cfg.get("sy_yearreport_pic").map(|s| s.as_str()).unwrap_or("")),
        }
    }))
}

async fn hrlog_set_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let isopen = if has_flag(body, "sy_yearreport_isopen") { "1" } else { "0" };
    upsert_cfg(state, user, "sy_yearreport_isopen", isopen).await?;
    if body.get("sy_yearreport_ewmtype").is_some() {
        upsert_cfg(
            state,
            user,
            "sy_yearreport_ewmtype",
            &json_str(body, "sy_yearreport_ewmtype"),
        )
        .await?;
    }
    let pic = json_str(body, "sy_yearreport_pic");
    if !pic.is_empty() {
        upsert_cfg(state, user, "sy_yearreport_pic", &pic).await?;
    }
    let tip = json_str(body, "sy_yearreport_tip");
    if !tip.is_empty() {
        upsert_cfg(state, user, "sy_yearreport_tip", &tip).await?;
    }
    Ok(PhpOut::Message("admin_01452"))
}

async fn hrlog_get_hb(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    let base = preview_base(state);
    let hb_url = format!("{base}/index.php?c=ajax&a=lastYearReport&uid={uid}");
    Ok(json!({ "hbUrl": hb_url }))
}

async fn trust_recom(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let f = job_repo::AdminJobFilter {
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        keyword_type: Some(json_i32(body, "type")).filter(|n| *n > 0),
        state: Some(1),
        status: None,
        ..Default::default()
    };
    let db = state.db.reader();
    let rows = job_repo::admin_list_filtered(db, &f, offset, limit).await?;
    let total = job_repo::admin_count_filtered(db, &f).await?;
    let dicts = dict_service::get(state).await?;
    let base = preview_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let job_three = {
                let a = dicts.job(r.job_post);
                if !a.is_empty() {
                    a.to_string()
                } else {
                    let b = dicts.job(r.job1_son);
                    if !b.is_empty() {
                        b.to_string()
                    } else {
                        dicts.job(r.job1).to_string()
                    }
                }
            };
            json!({
                "id": r.id,
                "uid": r.uid,
                "com_name": r.com_name.clone().unwrap_or_default(),
                "name": r.name,
                "job_comapply": format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", r.id),
                "job_city_one": dicts.city(r.provinceid),
                "job_city_two": dicts.city(r.cityid),
                "job_three_n": job_three,
                "job_salary": job_salary_n(r.minsalary, r.maxsalary),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn trust_directrecom(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let eid = json_u64(body, "eid");
    let jobid = json_u64(body, "jobid").max(json_u64(body, "id"));
    let comid = json_u64(body, "comid");
    if eid == 0 || jobid == 0 || comid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    if entrust_record::exists_record(state.db.reader(), eid, jobid, comid).await? {
        return Err(ApiError::business("admin_01339"));
    }
    let uid = match expect_repo::find_by_id(state.db.reader(), eid).await? {
        Some(ex) => ex.uid,
        None => json_u64(body, "uid"),
    };
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    entrust_record::insert_record(state.db.pool(), uid, eid, jobid, comid, clock::now_ts()).await?;
    Ok(PhpOut::Message("wap_01720"))
}

async fn userset_save_logo(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    if !has_flag(body, "submit") {
        return Err(ApiError::business("wap_01298"));
    }
    let mut man = json_str_list(body, "manicon_sys");
    man.extend(json_str_list(body, "man_files"));
    man.truncate(6);
    let mut woman = json_str_list(body, "womanicon_sys");
    woman.extend(json_str_list(body, "woman_files"));
    woman.truncate(6);
    if man.is_empty() || woman.is_empty() {
        return Err(ApiError::business("admin_user_00100"));
    }
    upsert_cfg(state, user, "sy_member_icon_arr", &php_serialize_strs(&man)).await?;
    upsert_cfg(state, user, "sy_member_icon", man.first().map(|s| s.as_str()).unwrap_or("")).await?;
    upsert_cfg(state, user, "sy_member_iconv_arr", &php_serialize_strs(&woman)).await?;
    upsert_cfg(
        state,
        user,
        "sy_member_iconv",
        woman.first().map(|s| s.as_str()).unwrap_or(""),
    )
    .await?;
    Ok(PhpOut::Message("admin_user_00098"))
}

async fn domain_group_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = rbac_php::php_count_groups_of_type(db, 2, keyword).await?;
    let rows = if total > 0 {
        rbac_php::php_list_groups_of_type(db, 2, keyword, offset, limit).await?
    } else {
        Vec::new()
    };
    let titles: HashMap<i32, String> = domain_repo::list_all(db)
        .await?
        .into_iter()
        .map(|d| (d.id as i32, d.title))
        .collect();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|g| {
            json!({
                "id": g.id,
                "group_name": g.group_name,
                "did": g.did,
                "num": g.num,
                "domain_name": titles.get(&g.did).cloned().unwrap_or_else(|| "--".to_string()),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn domain_group_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let mut data = role_ugroup_info(state, body).await?;
    let group = data.get("admin_group").cloned().unwrap_or(json!({}));
    data["groupInfo"] = group;
    let domains = domain_repo::list_all(state.db.reader()).await?;
    data["domain"] = Value::Array(
        domains
            .into_iter()
            .map(|d| json!({ "id": d.id, "title": d.title }))
            .collect(),
    );
    Ok(data)
}

async fn domain_group_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "group_name");
    if name.is_empty() {
        return Err(ApiError::business("admin_system_00010"));
    }
    let three = ids_named(body, "three_ids");
    if three.is_empty() {
        return Err(ApiError::business("admin_system_00009"));
    }
    let mut ids = ids_named(body, "one_ids");
    ids.extend(ids_named(body, "two_ids"));
    ids.extend(three);
    ids.extend(ids_named(body, "four_ids"));
    ids.sort_unstable();
    ids.dedup();
    let gid = json_u64(body, "groupid");
    if rbac_php::php_group_name_taken(state.db.pool(), &name, gid).await? {
        return Err(ApiError::business("admin_system_00027"));
    }
    let power_i: Vec<i64> = ids.iter().map(|n| *n as i64).collect();
    let ser = php_power::serialize_group_power(&power_i);
    let did = json_i32(body, "did");
    let n = rbac_php::php_save_group(
        state.db.pool(),
        if gid > 0 { Some(gid) } else { None },
        &name,
        &ser,
        did,
        2,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_06357"));
    }
    let _ = rbac_php::php_mark_admin_nav_dids(state.db.pool(), &power_i, 1).await;
    Ok(PhpOut::Message("common_06360"))
}

async fn domain_group_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "id");
    if ids.is_empty() {
        return Err(ApiError::business("wap_00203"));
    }
    for id in &ids {
        if rbac_php::php_count_group_users(state.db.pool(), *id).await? > 0 {
            return Err(ApiError::business("common_00307"));
        }
    }
    recycle_ids(
        state,
        user,
        "admin_user_group",
        &ids,
        "/v1/admin/php-content/domain-group/delGroup",
    )
    .await;
    for id in ids {
        let n = rbac_php::php_delete_group(state.db.pool(), id).await?;
        if n == 0 {
            return Err(ApiError::business("model_00137"));
        }
    }
    Ok(PhpOut::Message("model_00112"))
}

async fn domain_admin_list(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = rbac_php::php_count_site_admins(db, keyword).await?;
    let rows = if total > 0 {
        rbac_php::php_list_site_admins(db, keyword, offset, limit).await?
    } else {
        Vec::new()
    };
    let titles: HashMap<i32, String> = domain_repo::list_all(db)
        .await?
        .into_iter()
        .map(|d| (d.id as i32, d.title))
        .collect();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|u| {
            json!({
                "uid": u.uid,
                "username": u.username,
                "name": u.name,
                "m_id": u.m_id,
                "did": u.did,
                "group_name": u.group_name,
                "domain_name": titles.get(&u.did).cloned().unwrap_or_else(|| "--".to_string()),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn domain_admin_info(state: &AppState, body: &Value) -> AppResult<Value> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Ok(json!({}));
    }
    Ok(match rbac_php::php_get_user(state.db.reader(), uid).await? {
        Some(u) => json!({
            "uid": u.uid,
            "username": u.username,
            "name": u.name,
            "m_id": u.m_id,
            "did": u.did,
        }),
        None => json!({}),
    })
}

fn php_domain_city_label(
    dicts: &dict_service::LocalizedDicts,
    province: i32,
    cityid: i32,
    three: i32,
) -> String {
    let pair = |a: i32, b: i32| -> String {
        let left = dicts.city(a);
        let right = dicts.city(b);
        if left.is_empty() && right.is_empty() {
            "--".to_string()
        } else if left.is_empty() {
            right.to_string()
        } else if right.is_empty() {
            left.to_string()
        } else {
            format!("{left} - {right}")
        }
    };
    if three > 0 {
        pair(cityid, three)
    } else if cityid > 0 {
        pair(province, cityid)
    } else if province > 0 {
        let n = dicts.city(province);
        if n.is_empty() {
            "--".to_string()
        } else {
            n.to_string()
        }
    } else {
        "--".to_string()
    }
}

async fn domain_list_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = gap_repo::count_domains(db, keyword).await?;
    let rows = if total > 0 {
        gap_repo::list_domains(db, keyword, offset, limit).await?
    } else {
        Vec::new()
    };
    let dicts = dict_service::get(state).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let name = if r.mode == 2 {
                r.indexdir.clone()
            } else {
                r.domain.clone()
            };
            let (city, hy_n) = if r.fz_type == 1 {
                (
                    php_domain_city_label(&dicts, r.province, r.cityid, r.three_cityid),
                    "--".to_string(),
                )
            } else if r.fz_type == 2 {
                let hy = dicts.industry(r.hy);
                (
                    "--".to_string(),
                    if hy.is_empty() {
                        "--".to_string()
                    } else {
                        hy.to_string()
                    },
                )
            } else {
                ("--".to_string(), "--".to_string())
            };
            json!({
                "id": r.id,
                "title": r.title,
                "name": name,
                "domain": r.domain,
                "indexdir": r.indexdir,
                "city": city,
                "hy_n": hy_n,
                "style": r.style,
                "type": r.r#type,
                "typeStatus": r.r#type == 1,
                "fz_type": r.fz_type,
                "mode": r.mode,
                "hy": r.hy,
                "cityid": r.cityid,
                "province": r.province,
                "three_cityid": r.three_cityid,
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn domain_list_change_type(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    let typ = json_i32(body, "type");
    if id == 0 || typ == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    let n = gap_repo::php_set_domain_type(state.db.pool(), id, typ).await?;
    if n == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    home_service::invalidate_all().await;
    if typ == 1 {
        Ok(PhpOut::Message("admin_01372"))
    } else {
        Ok(PhpOut::Message("admin_01373"))
    }
}

async fn domain_list_config_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    if json_i32(body, "domainConfig") != 1 {
        return Err(ApiError::business("wap_00203"));
    }
    let cfg = settings_hash(state).await.unwrap_or_default();
    let mut indexdomain = json_str(body, "sy_indexdomain");
    if !indexdomain.is_empty() {
        let lower = indexdomain.to_ascii_lowercase();
        if !lower.contains("http") {
            let weburl = cfg_pick(&cfg, "sy_weburl");
            let proto = if weburl.to_ascii_lowercase().contains("https://") {
                "https://"
            } else {
                "http://"
            };
            indexdomain = format!("{proto}{indexdomain}");
        }
    }
    upsert_cfg(state, user, "sy_web_site", &json_str(body, "sy_web_site")).await?;
    upsert_cfg(state, user, "sy_gotocity", &json_str(body, "sy_gotocity")).await?;
    upsert_cfg(state, user, "sy_indexcity", &json_str(body, "sy_indexcity")).await?;
    upsert_cfg(state, user, "sy_indexdomain", &indexdomain).await?;
    upsert_cfg(state, user, "sy_onedomain", &json_str(body, "sy_onedomain")).await?;
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("admin_01371"))
}

const SKIP_STYLE_DIRS: &[&str] = &[
    "admin", "ask", "chat", "company", "lietou", "member", "promoter", "resume", "school",
    "shop", "siteadmin", "train", "im", "wap", "wapadmin", "com", "indextpl",
];

fn style_info_row(root: &std::path::Path, dir: &str) -> Value {
    let text = std::fs::read_to_string(root.join(dir).join("info.txt")).unwrap_or_default();
    let parts: Vec<&str> = text.split("||").collect();
    let name = parts
        .first()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or(dir);
    let author = parts.get(1).map(|s| s.trim()).unwrap_or("");
    let dir_n = parts
        .get(2)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or(dir);
    let img = parts.get(3).map(|s| s.trim()).unwrap_or("");
    json!({
        "name": name,
        "author": author,
        "dir": dir_n,
        "img": img,
    })
}

fn collect_style_dirs(root: &std::path::Path) -> Vec<String> {
    let Ok(rd) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    let mut names: Vec<String> = rd
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('.'))
        .collect();
    names.sort();
    names
}

fn domain_style_list() -> Vec<Value> {
    let php = std::path::Path::new("/www/wwwroot/zzzz.com/uploads/app/template");
    let web = std::path::Path::new("/www/wwwroot/zzzz.com/web/apps/site/public/skins");
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for dir in collect_style_dirs(php) {
        if SKIP_STYLE_DIRS.contains(&dir.as_str()) {
            continue;
        }
        if seen.insert(dir.clone()) {
            out.push(style_info_row(php, &dir));
        }
    }
    for dir in collect_style_dirs(web) {
        if SKIP_STYLE_DIRS.contains(&dir.as_str()) {
            continue;
        }
        if seen.insert(dir.clone()) {
            out.push(style_info_row(web, &dir));
        }
    }
    out
}

fn tpl_preview_url(cfg: &HashMap<String, String>, img: &str) -> String {
    if img.starts_with("/skins/") {
        img.to_string()
    } else {
        checkpic_url(cfg, img)
    }
}

async fn domain_list_get_cache(state: &AppState) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let mut province_arr = Vec::new();
    let mut city_arr = Vec::new();
    for (pid, pname) in dicts.city_provinces() {
        province_arr.push(json!({ "id": pid, "name": pname }));
        for (cid, cname) in dicts.city_of_parent(pid) {
            city_arr.push(json!({ "id": cid, "pid": pid, "name": cname }));
            for (tid, tname) in dicts.city_of_parent(cid) {
                city_arr.push(json!({ "id": tid, "pid": cid, "name": tname }));
            }
        }
    }
    let industry_arr: Vec<Value> = dicts
        .industry_all()
        .into_iter()
        .map(|(id, name)| json!({ "id": id, "name": name }))
        .collect();
    let cfg = settings_hash(state).await.unwrap_or_default();
    let pic_max = cfg_pick(&cfg, "pic_maxsize");
    let pic_maxsize = if pic_max.is_empty() {
        json!(5)
    } else {
        pic_max
            .parse::<i64>()
            .map(Value::from)
            .unwrap_or_else(|_| json!(pic_max))
    };
    let pic_type = cfg_pick(&cfg, "pic_type");
    Ok(json!({
        "styleList": domain_style_list(),
        "industryArr": industry_arr,
        "provinceArr": province_arr,
        "cityArr": city_arr,
        "picMaxSize": pic_maxsize,
        "picType": if pic_type.is_empty() { "jpg,png,jpeg,bmp,gif".to_string() } else { pic_type },
    }))
}

struct PhpTplDef {
    key: String,
    name: String,
    kind: String,
    config: String,
    cate: String,
    vars: serde_json::Map<String, Value>,
}

fn php_squote_after(block: &str, key: &str) -> String {
    let needle = format!("'{key}'");
    let Some(pos) = block.find(&needle) else {
        return String::new();
    };
    let rest = block[pos + needle.len()..].trim_start();
    let rest = rest.strip_prefix("=>").unwrap_or(rest).trim_start();
    let rest = rest.strip_prefix('\'').unwrap_or(rest);
    rest.find('\'').map(|i| rest[..i].to_string()).unwrap_or_default()
}

fn php_tpl_vars(block: &str) -> serde_json::Map<String, Value> {
    let mut m = serde_json::Map::new();
    let mut s = block;
    while let Some(i) = s.find("'{") {
        let s2 = &s[i + 1..];
        let Some(endk) = s2.find('\'') else {
            break;
        };
        let key = &s2[..endk];
        let rest = s2[endk + 1..].trim_start();
        let rest = rest.strip_prefix("=>").unwrap_or(rest).trim_start();
        let rest = rest.strip_prefix('\'').unwrap_or(rest);
        let Some(j) = rest.find('\'') else {
            break;
        };
        m.insert(key.to_string(), json!(rest[..j]));
        s = &rest[j + 1..];
    }
    m
}

fn last_php_ident_key(s: &str) -> String {
    let mut i = s.len();
    while i > 0 {
        let Some(p) = s[..i].rfind('\'') else {
            break;
        };
        let Some(p0) = s[..p].rfind('\'') else {
            break;
        };
        let k = &s[p0 + 1..p];
        if !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return k.to_string();
        }
        i = p0;
    }
    String::new()
}

fn split_paren_block(s: &str) -> (&str, &str) {
    let mut depth = 1i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return (&s[..i], &s[i + 1..]);
                }
            }
            _ => {}
        }
    }
    (s, "")
}

fn parse_arr_tpl() -> Vec<PhpTplDef> {
    let text = std::fs::read_to_string("/www/wwwroot/zzzz.com/uploads/config/db.tpl.php").unwrap_or_default();
    let mut out = Vec::new();
    let mut search = text.as_str();
    while let Some(arr) = search.find("=> array") {
        let key = last_php_ident_key(&search[..arr]);
        let after = search[arr + "=> array".len()..].trim_start();
        let after = after.strip_prefix('(').unwrap_or(after);
        let (block, rest) = split_paren_block(after);
        if !key.is_empty() {
            let mut vars = php_tpl_vars(block);
            vars.remove("name");
            vars.remove("type");
            vars.remove("config");
            vars.remove("cate");
            out.push(PhpTplDef {
                key,
                name: php_squote_after(block, "name"),
                kind: php_squote_after(block, "type"),
                config: php_squote_after(block, "config"),
                cate: php_squote_after(block, "cate"),
                vars,
            });
        }
        search = rest;
    }
    out
}

fn tpl_meta_of(name: &str) -> (String, Value) {
    for t in parse_arr_tpl() {
        if t.key == name {
            return (t.name, Value::Object(t.vars));
        }
    }
    (name.to_string(), json!({}))
}

async fn tplswitch_data(state: &AppState, kind: &str) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let (user_key, com_key) = if kind == "email" {
        ("admin_01459", "admin_01460")
    } else {
        ("admin_01469", "admin_01470")
    };
    let mut public = Vec::new();
    let mut user = Vec::new();
    let mut com = Vec::new();
    for t in parse_arr_tpl() {
        if t.kind != kind {
            continue;
        }
        let item = json!({
            "name": t.name,
            "tpl": t.key,
            "config_name": t.config,
            "config_val": cfg_pick(&cfg, &t.config),
        });
        match t.cate.as_str() {
            "user" => user.push(item),
            "com" => com.push(item),
            _ => public.push(item),
        }
    }
    Ok(json!({
        "public": { "name": msg_t("admin_tool_00029"), "configarr": public },
        "user": { "name": msg_t(user_key), "configarr": user },
        "com": { "name": msg_t(com_key), "configarr": com },
    }))
}

async fn email_set_savetplconfig(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let allowed: std::collections::HashSet<String> = parse_arr_tpl()
        .into_iter()
        .filter(|t| t.kind == "email")
        .map(|t| t.config)
        .collect();
    if let Some(obj) = body.as_object() {
        for (k, v) in obj {
            if !allowed.contains(k) {
                continue;
            }
            let val = match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(true) => "1".into(),
                _ => "2".into(),
            };
            upsert_cfg(state, user, k, &val).await?;
        }
    }
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("admin_01461"))
}

async fn email_set_delconfig(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    let row = gap_extra::php_get_admin_email(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::business("wap_00203"))?;
    if row.default_flag == 1 && gap_extra::php_count_default_smtp(state.db.reader()).await? < 2 {
        return Err(ApiError::business("admin_tool_00024"));
    }
    recycle_ids(
        state,
        user,
        "admin_email",
        &[id],
        "/v1/admin/php-content/email-set/delconfig",
    )
    .await;
    let n = gap_extra::php_delete_admin_email(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::business("model_00137"));
    }
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("model_00112"))
}

async fn message_set_savetpl(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let content = json_str(body, "content").replace("amp;nbsp;", "nbsp;");
    let title = {
        let t = json_str(body, "title");
        if t.is_empty() {
            site_page_repo::find_by_code(state.db.reader(), &name)
                .await?
                .map(|r| r.title)
                .unwrap_or_default()
        } else {
            t
        }
    };
    site_page_repo::upsert_content(state.db.pool(), &name, &title, &content).await?;
    Ok(PhpOut::Message("admin_01471"))
}

fn json_to_uc_map(body: &Value) -> serde_json::Map<String, Value> {
    let mut m = serde_json::Map::new();
    if let Some(obj) = body.as_object() {
        for (k, v) in obj {
            if k == "sy_uc_type" || k == "sy_pw_type" || k == "m" || k == "c" || k == "a" {
                continue;
            }
            if k.starts_with("UC_") {
                m.insert(k.clone(), json!(match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => String::new(),
                }));
            }
        }
    }
    m
}

async fn admin_uc_index(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let ucinfo = serde_json::from_str::<Value>(&cfg_pick(&cfg, "sy_uc_info")).unwrap_or(json!({}));
    let pw_ucinfo = serde_json::from_str::<Value>(&cfg_pick(&cfg, "sy_pw_info")).unwrap_or(json!({}));
    Ok(json!({
        "ucinfo": ucinfo,
        "pw_ucinfo": pw_ucinfo,
        "config": {
            "sy_uc_type": cfg_pick(&cfg, "sy_uc_type"),
            "sy_pw_type": cfg_pick(&cfg, "sy_pw_type"),
        }
    }))
}

async fn admin_uc_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
    pw: bool,
) -> AppResult<PhpOut> {
    let info = Value::Object(json_to_uc_map(body));
    if pw {
        upsert_cfg(state, user, "sy_pw_type", &json_str(body, "sy_pw_type")).await?;
        upsert_cfg(state, user, "sy_pw_info", &info.to_string()).await?;
        upsert_cfg(state, user, "sy_uc_type", "").await?;
    } else {
        upsert_cfg(state, user, "sy_uc_type", &json_str(body, "sy_uc_type")).await?;
        upsert_cfg(state, user, "sy_uc_info", &info.to_string()).await?;
    }
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("wap_user_00104"))
}

fn split_csv(raw: &str) -> Vec<String> {
    raw.split([',', ';', '\n'])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

async fn admin_member_send_email(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let title = json_str(body, "email_title");
    let content = json_str(body, "content");
    if title.is_empty() || content.is_empty() {
        return Err(ApiError::business("admin_user_00007"));
    }
    let utype = json_i32(body, "utype");
    let rows = if utype == 5 {
        let emails = split_csv(&json_str(body, "email_user"));
        gap_extra::php_list_members_by_emails(state.db.reader(), &emails).await?
    } else if utype > 0 {
        gap_extra::php_list_members_by_usertype(state.db.reader(), utype).await?
    } else {
        Vec::new()
    };
    let mut targets: Vec<(u64, String)> = rows
        .into_iter()
        .filter(|r| r.email.contains('@'))
        .map(|r| (r.uid, r.email))
        .collect();
    targets.sort_by(|a, b| a.0.cmp(&b.0));
    targets.dedup_by(|a, b| a.0 == b.0);
    if targets.is_empty() {
        return Err(ApiError::business("admin_user_00003"));
    }
    let limit = json_u64(body, "pagelimit").clamp(1, 100) as usize;
    let start = json_u64(body, "value") as usize * limit;
    let now = clock::now_ts();
    let mut sent = 0u64;
    for (uid, email) in targets.iter().skip(start).take(limit) {
        gap_repo::insert_email_log(state.db.pool(), *uid, email, &title, &content, now, 1).await?;
        let _ = state
            .events
            .publish_json(
                "email.verify_queued",
                &json!({
                    "kind": "admin_member_mail",
                    "uid": uid,
                    "email": email,
                    "subject": title,
                }),
            )
            .await;
        sent += 1;
    }
    let _ = sent;
    Ok(PhpOut::Message("admin_tool_00495"))
}

async fn admin_member_send_sms(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let open = cfg_pick(&cfg, "sy_msg_isopen");
    if open != "1" {
        return Err(ApiError::business("admin_user_00010"));
    }
    let content = json_str(body, "content");
    if content.is_empty() {
        return Err(ApiError::business("admin_01299"));
    }
    let utype = json_i32(body, "utype");
    if utype == 5 && json_str(body, "userarr").is_empty() {
        return Err(ApiError::business("wap_00399"));
    }
    let rows = if utype == 5 {
        let mobiles = split_csv(&json_str(body, "userarr"));
        gap_extra::php_list_members_by_mobiles(state.db.reader(), &mobiles).await?
    } else if utype > 0 {
        gap_extra::php_list_members_by_usertype(state.db.reader(), utype).await?
    } else {
        Vec::new()
    };
    let mut targets: Vec<(u64, String)> = rows
        .into_iter()
        .filter(|r| !r.moblie.trim().is_empty())
        .map(|r| (r.uid, r.moblie))
        .collect();
    if utype == 5 {
        let known: std::collections::HashSet<String> = targets.iter().map(|(_, m)| m.clone()).collect();
        for m in split_csv(&json_str(body, "userarr")) {
            if !known.contains(&m) && m.chars().any(|c| c.is_ascii_digit()) {
                targets.push((0, m));
            }
        }
    }
    if targets.is_empty() {
        return Err(ApiError::business("admin_user_00004"));
    }
    let limit = json_u64(body, "pagelimit").clamp(1, 100) as usize;
    let start = json_u64(body, "value") as usize * limit;
    let now = clock::now_ts();
    for (uid, mobile) in targets.iter().skip(start).take(limit) {
        gap_repo::insert_sms_log(state.db.pool(), *uid, mobile, &content, now, 1).await?;
        let _ = state
            .events
            .publish_json(
                "sms.admin_queued",
                &json!({
                    "kind": "admin_member_sms",
                    "uid": uid,
                    "mobile": mobile,
                }),
            )
            .await;
    }
    Ok(PhpOut::Message("admin_tool_00495"))
}

async fn weixinrecord_clearwx(state: &AppState) -> AppResult<PhpOut> {
    let before = clock::now_ts() - 3 * 86400;
    let n = gap_extra::php_delete_old_wxqrcodes(state.db.pool(), before).await?;
    if n > 0 {
        Ok(PhpOut::Message("admin_tool_00058"))
    } else {
        Ok(PhpOut::Message("admin_user_00186"))
    }
}

async fn weixinrecord_userbd(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = gap_extra::php_count_wx_bound(db, keyword).await?;
    let rows = if total > 0 {
        gap_extra::php_list_wx_bound(db, keyword, offset, limit).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "uid": r.uid,
                "username": r.username,
                "wxid": r.wxid,
                "wxbindtime": r.wxbindtime,
                "wxbindtime_n": if r.wxbindtime > 0 { fmt_dt(r.wxbindtime) } else { String::new() },
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn weixinrecord_deluser(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::business("admin_tool_00055"));
    }
    let n = gap_extra::php_clear_member_wxids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_tool_00055"));
    }
    Ok(PhpOut::Message("admin_01379"))
}

async fn weixinrecord_keyword(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let db = state.db.reader();
    let total = gap_extra::php_count_wx_hot_keys(db, keyword).await?;
    let rows = if total > 0 {
        gap_extra::php_list_wx_hot_keys(db, keyword, offset, limit).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "key_name": r.key_name,
                "num": r.num,
                "wxtime": r.wxtime,
                "wxtime_n": if r.wxtime > 0 { fmt_dt(r.wxtime) } else { String::new() },
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn weixinrecord_delkeyword(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::business("wap_00203"));
    }
    recycle_ids(
        state,
        user,
        "hot_key",
        &ids,
        "/v1/admin/php-content/weixinrecord/delkeyword",
    )
    .await;
    let n = gap_repo::delete_hot_keys(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("model_00137"));
    }
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("model_00112"))
}

fn strip_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for c in raw.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&nbsp;", " ").replace("&quot;", "\"").replace("&amp;", "&")
}

fn resume_age(birthday: Option<&str>) -> i32 {
    let s = birthday.unwrap_or("");
    if s.len() < 4 {
        return 0;
    }
    let y: i32 = s[..4].parse().unwrap_or(0);
    if y < 1920 {
        return 0;
    }
    let cy = 1970 + (clock::now_ts() / 31_557_600) as i32;
    (cy - y).max(0)
}

fn range_n(sdate: i64, edate: i64) -> (String, String) {
    let start = if sdate > 0 { fmt_date(sdate) } else { String::new() };
    let end = if edate > 0 {
        fmt_date(edate)
    } else if sdate > 0 {
        String::from("至今")
    } else {
        String::new()
    };
    (start, end)
}

async fn company_job_save_address(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    let man = json_str(body, "link_man");
    let mobile = json_str(body, "link_moblie");
    let phone = json_str(body, "link_phone");
    let email = json_str(body, "email");
    let address = json_str(body, "link_address");
    let x = json_str(body, "x");
    let y = json_str(body, "y");
    let f = address_repo::AddressFields {
        link_man: &man,
        link_moblie: &mobile,
        link_phone: &phone,
        email: &email,
        link_address: &address,
        provinceid: json_i32(body, "provinceid"),
        cityid: json_i32(body, "cityid"),
        three_cityid: json_i32(body, "three_cityid"),
        x: &x,
        y: &y,
    };
    let link_id = if json_u64(body, "id") > 0 {
        let n = address_repo::update(state.db.pool(), json_u64(body, "id"), uid, &f).await?;
        if n == 0 {
            return Err(ApiError::business("api_wxapp_00006"));
        }
        json_u64(body, "id")
    } else {
        address_repo::create(state.db.pool(), uid, &f).await?
    };
    if json_i32(body, "is_link") == 2 {
        let list = address_repo::list_by_uid(state.db.reader(), uid, 0, 100).await?;
        return Ok(PhpOut::Data(json!({
            "addressList": list,
            "link_id": link_id,
        })));
    }
    Ok(PhpOut::Message("api_wxapp_00007"))
}

async fn company_job_saveclass(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if json_str(body, "hy").is_empty() {
        return Err(ApiError::business("admin_01283"));
    }
    if json_str(body, "job1").is_empty() {
        return Err(ApiError::business("admin_user_company_00023"));
    }
    let ids = gap_extra::parse_id_csv(&json_csv(body, "jobid"));
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = gap_extra::php_update_job_class(
        state.db.pool(),
        &ids,
        json_i32(body, "hy"),
        json_i32(body, "job1"),
        json_i32(body, "job1_son"),
        json_i32(body, "job_post"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("member_user_00603"));
    }
    let jobs = job_repo::list_by_ids(state.db.reader(), &ids).await?;
    let now = clock::now_ts();
    for j in &jobs {
        let notice = format!("{}{}", msg_t("admin_model_00127"), j.name);
        gap_repo::insert_sysmsg(state.db.pool(), j.uid, 2, &notice, now).await?;
    }
    Ok(PhpOut::Text(
        "admin_model_00128",
        format!("{}{}", msg_t("admin_model_00128"), json_csv(body, "jobid")),
    ))
}

async fn company_job_get_html(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let Some(job) = job_repo::find_by_id(state.db.reader(), id).await? else {
        return Err(ApiError::business("model_00008"));
    };
    let com = company_repo::find_by_uid(state.db.reader(), job.uid).await?;
    let phone = com
        .as_ref()
        .and_then(|c| c.linktel.clone().filter(|s| !s.is_empty()).or(c.linkphone.clone()))
        .unwrap_or_default();
    let addr = com.as_ref().and_then(|c| c.address.clone()).unwrap_or_default();
    let com_name = job
        .com_name
        .clone()
        .or_else(|| com.as_ref().and_then(|c| c.name.clone()))
        .unwrap_or_default();
    let desc = strip_html(job.description.as_deref().unwrap_or(""));
    let salary = if job.maxsalary > 0 {
        format!("{}-{}", job.minsalary, job.maxsalary)
    } else if job.minsalary > 0 {
        job.minsalary.to_string()
    } else {
        String::from("面议")
    };
    Ok(Value::String(format!(
        "<div><p><b>{}</b> · {}</p><p>薪资：{}</p><p>{}</p><p>电话：{}</p><p>地址：{}</p></div>",
        job.name, com_name, salary, desc, phone, addr
    )))
}

async fn company_job_add_tuiwen(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = gap_extra::parse_id_csv(&json_csv(body, "twtask_jobid"));
    let content = json_str(body, "twtask_content");
    if ids.is_empty() || user.uid == 0 {
        return Err(ApiError::param_invalid("common_01238"));
    }
    let jobs = gap_extra::tuiwen_jobs(state.db.reader(), &ids).await?;
    if jobs.is_empty() {
        return Err(ApiError::business("common_06677"));
    }
    let rows: Vec<gap_extra::TuiWenTaskIn> = jobs
        .iter()
        .map(|j| gap_extra::TuiWenTaskIn {
            cuid: j.uid,
            comname: j.com_name.clone(),
            jobsdate: j.sdate,
            auid: user.uid,
            content: content.clone(),
            urgent: json_i32(body, "twtask_urgent"),
            wcmoments: json_i32(body, "twtask_wcmoments"),
            gzh: json_i32(body, "twtask_gzh"),
            jobid: j.id,
            jobname: j.name.clone(),
            kind: 1,
        })
        .collect();
    if gap_extra::insert_tuiwen_tasks(state.db.pool(), &rows, clock::now_ts()).await? == 0 {
        return Err(ApiError::business("common_06677"));
    }
    Ok(PhpOut::Message("common_06676"))
}

async fn company_whb(state: &AppState, typ: i32) -> AppResult<Value> {
    let base = preview_base(state);
    let rows = whb_repo::list_admin_by_type(state.db.reader(), typ).await?;
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
                "style": r.style,
            })
        })
        .collect();
    let hburl = if typ == 2 {
        format!("{base}/index.php?m=ajax&c=getComHb")
    } else {
        format!("{base}/index.php?m=ajax&c=getJobHb")
    };
    Ok(json!({ "comHb": list, "hburl": hburl }))
}

async fn company_job_xls(state: &AppState, body: &Value) -> AppResult<Value> {
    let ids = ids_of(body);
    let limit = json_u64(body, "limit");
    let rows = gap_extra::php_list_jobs_export(state.db.reader(), &ids, limit).await?;
    if rows.is_empty() {
        return Err(ApiError::business("admin_01308"));
    }
    let mut csv = String::from("id,uid,name,com_name,minsalary,maxsalary,lastupdate\n");
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            r.id,
            r.uid,
            csv_cell(&r.name),
            csv_cell(&r.com_name),
            r.minsalary,
            r.maxsalary,
            fmt_dt(r.lastupdate),
        ));
    }
    let file = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, csv.as_bytes());
    Ok(json!({
        "file": file,
        "file_name": format!("jobs-{}.csv", fmt_date(clock::now_ts())),
        "status": 1,
    }))
}

async fn resume_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    let ids = if ids.is_empty() { ids_of(body) } else { ids };
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    recycle_ids(
        state,
        user,
        "resume_expect",
        &ids,
        "/v1/admin/php-content/resume/delResume",
    )
    .await;
    gap_extra::php_delete_expect_children(state.db.pool(), &ids).await?;
    let n = gap_extra::php_delete_expects(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Text(
        "admin_user_00187",
        format!("{}{}{}", msg_t("common_06284"), json_csv(body, "del"), msg_t("model_00112")),
    ))
}

async fn resume_del_fb(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let table = json_str(body, "table");
    let id = json_u64(body, "id");
    let uid = json_u64(body, "uid");
    if id == 0 || uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = match table.as_str() {
        "skill" => skill_repo::delete(state.db.pool(), id, uid).await?,
        "work" => work_repo::delete(state.db.pool(), id, uid).await?,
        "project" => project_repo::delete(state.db.pool(), id, uid).await?,
        "edu" => edu_repo::delete(state.db.pool(), id, uid).await?,
        "training" => training_repo::delete(state.db.pool(), id, uid).await?,
        "other" => other_repo::delete(state.db.pool(), id, uid).await?,
        _ => return Err(ApiError::business("admin_01321")),
    };
    if n == 0 {
        return Err(ApiError::business("admin_01321"));
    }
    Ok(PhpOut::Message("wap_user_00147"))
}

async fn resume_label(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let n = gap_extra::php_set_expect_label(
        state.db.pool(),
        id,
        &json_str(body, "label"),
        &json_str(body, "content"),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("wap_01715"));
    }
    Ok(PhpOut::Message("model_00011"))
}

async fn resume_preview(state: &AppState, body: &Value) -> AppResult<Value> {
    let mut eid = json_u64(body, "id");
    let mut uid = json_u64(body, "uid");
    let db = state.db.reader();
    if eid == 0 && uid > 0 {
        if let Some(r) = resume_repo::find_by_uid(db, uid).await? {
            eid = u64::try_from(r.def_job.max(0)).unwrap_or(0);
            uid = r.uid;
        }
    }
    let mut expect = if eid > 0 {
        expect_repo::find_by_id(db, eid).await?
    } else {
        None
    };
    if expect.is_none() && uid > 0 {
        expect = expect_repo::list_by_uid(db, uid).await?.into_iter().next();
    }
    if let Some(e) = expect.as_ref() {
        uid = e.uid;
        eid = e.id;
    }
    let resumeinfo = if uid > 0 {
        resume_repo::find_by_uid(db, uid).await?
    } else {
        None
    };
    let dict = dict_service::get(state).await?;
    let expect_json = if let Some(e) = expect.as_ref() {
        let job_name = dict.job(e.job_classid as i32).to_string();
        json!({
            "id": e.id,
            "uid": e.uid,
            "name": e.name,
            "hy": e.hy,
            "hy_n": dict.industry(e.hy),
            "job_classid": e.job_classid,
            "city_classid": e.city_classid,
            "city_classname": dict.city(e.city_classid as i32),
            "report": e.report,
            "report_n": dict.user_or_com(e.report),
            "type": e.r#type,
            "type_n": dict.user_or_com(e.r#type),
            "jobstatus": e.jobstatus,
            "jobstatus_n": dict.user_or_com(e.jobstatus),
            "expectjob": if job_name.is_empty() { Vec::<String>::new() } else { vec![job_name] },
            "add_ip": "",
        })
    } else {
        json!({})
    };
    let map_dates = |sdate: i64, edate: i64, extra: Value| {
        let (sdate_n, edate_n) = range_n(sdate, edate);
        let mut o = extra;
        if let Some(m) = o.as_object_mut() {
            m.insert("sdate_n".into(), json!(sdate_n));
            m.insert("edate_n".into(), json!(edate_n));
        }
        o
    };
    let work: Vec<Value> = if uid > 0 {
        work_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| {
                map_dates(
                    w.sdate,
                    w.edate,
                    json!({
                        "id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name,
                        "title": w.title, "content": w.content, "department": w.department,
                    }),
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let edu: Vec<Value> = if uid > 0 {
        edu_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| {
                map_dates(
                    w.sdate,
                    w.edate,
                    json!({
                        "id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name,
                        "specialty": w.specialty, "education": w.education,
                    }),
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let training: Vec<Value> = if uid > 0 {
        training_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| {
                map_dates(
                    w.sdate,
                    w.edate,
                    json!({
                        "id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name,
                        "title": w.title, "content": w.content,
                    }),
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let skill: Vec<Value> = if uid > 0 {
        skill_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| {
                json!({
                    "id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name,
                    "longtime": w.years, "pic": "",
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    let project: Vec<Value> = if uid > 0 {
        project_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| {
                map_dates(
                    w.sdate,
                    w.edate,
                    json!({
                        "id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name,
                        "title": w.role, "content": w.content,
                    }),
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let other: Vec<Value> = if uid > 0 {
        other_repo::list_by_uid(db, uid)
            .await?
            .into_iter()
            .filter(|w| eid == 0 || w.eid == eid)
            .map(|w| json!({"id": w.id, "uid": w.uid, "eid": w.eid, "name": w.name, "content": w.content}))
            .collect()
    } else {
        Vec::new()
    };
    let base = preview_base(state);
    let resume_json = if let Some(r) = resumeinfo.as_ref() {
        json!({
            "uid": r.uid,
            "name": r.name,
            "telphone": r.telphone,
            "email": r.email,
            "description": r.description,
            "photo": pic_url(&base, r.photo.as_deref().unwrap_or("")),
            "exp_n": dict.user_or_com(r.exp),
            "edu_n": dict.user_or_com(r.education),
            "age": resume_age(r.birthday.as_deref()),
            "def_job": r.def_job,
        })
    } else {
        json!({})
    };
    Ok(json!({
        "Info": expect_json,
        "expect": expect_json,
        "edu": edu,
        "other": other,
        "project": project,
        "skill": skill,
        "training": training,
        "work": work,
        "resumeinfo": resume_json,
        "cionly": 0,
        "jionly": 0,
    }))
}

async fn resume_export_check(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if json_str_list(body, "type").is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let ids = ids_named(body, "ids");
    let ids = if ids.is_empty() { ids_of(body) } else { ids };
    let n = gap_extra::php_count_expects(state.db.reader(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00095"));
    }
    Ok(PhpOut::Data(json!({ "field": "id" })))
}

async fn company_savefact(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    let del_ids = gap_extra::parse_id_csv(&json_csv(body, "fact_delid"));
    if !del_ids.is_empty() {
        recycle_ids(
            state,
            user,
            "company_fact",
            &del_ids,
            "/v1/admin/php-content/company/savefact",
        )
        .await;
        gap_extra::php_delete_fact_pics(state.db.pool(), &del_ids).await?;
    }
    let now = clock::now_ts();
    for pic in json_str_list(body, "newpic") {
        if !pic.is_empty() {
            gap_extra::php_insert_fact_pic(state.db.pool(), uid, &pic, now).await?;
        }
    }
    gap_extra::php_set_fact_status(state.db.pool(), uid, json_i32(body, "fact_status")).await?;
    Ok(PhpOut::Message("admin_user_00033"))
}

async fn company_getacbindstatus(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "comid");
    if uid == 0 {
        return Err(ApiError::business("wap_00203"));
    }
    let wxid = gap_extra::php_member_wxid(state.db.reader(), uid).await?;
    if wxid.is_empty() {
        return Err(ApiError::business("admin_user_00040"));
    }
    Ok(PhpOut::Data(json!({ "wxid": wxid })))
}

async fn company_export_check(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if json_str_list(body, "type").is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let uids = gap_extra::parse_id_csv(&json_csv(body, "uid"));
    let n = gap_extra::php_count_companies(state.db.reader(), &uids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_01308"));
    }
    Ok(PhpOut::Data(json!({ "field": "uid" })))
}

fn company_admin_logo_hb(body: &Value) -> AppResult<PhpOut> {
    let _ = json_str(body, "name");
    Err(ApiError::business("admin_user_00035"))
}

fn strip_yun_tags(mut s: String) -> String {
    loop {
        let Some(a) = s.find("{yun:}") else {
            break;
        };
        let Some(rel) = s[a..].find("{/yun}") else {
            break;
        };
        s.replace_range(a..a + rel + 6, "");
    }
    s
}

fn column_form(items: &[(&str, &str, &str, &str)]) -> Vec<Value> {
    items
        .iter()
        .map(|(key, a, b, c)| {
            json!({
                "key": key,
                "data": [a, b, c],
            })
        })
        .collect()
}

fn map_pairs(items: &[(&str, &str)]) -> Vec<Value> {
    items
        .iter()
        .map(|(search, replace)| json!({ "search": search, "replace": replace }))
        .collect()
}

async fn fabutool_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let temptype = json_opt_i32(body, "temptype");
    let db = state.db.reader();
    let total = gap_repo::count_wxpub_temps(
        db,
        if kw.is_empty() { None } else { Some(kw.as_str()) },
        temptype,
    )
    .await?;
    let rows = if total > 0 {
        gap_extra::php_list_wxpub_temps_php(
            db,
            if kw.is_empty() { None } else { Some(kw.as_str()) },
            temptype,
            offset,
            limit,
        )
        .await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "type": r.r#type,
                "temptype": r.temptype.to_string(),
                "time": r.time,
                "time_n": if r.time > 0 { fmt_dt(r.time) } else { String::new() },
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn fabutool_wx_pub_temp(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    let mut info = json!({});
    let mut temptype = json_str(body, "temptype");
    if id > 0 {
        if let Some(t) = gap_extra::php_find_wxpub_temp(state.db.reader(), id).await? {
            let ty = if t.r#type == "onejob" {
                "job".into()
            } else {
                t.r#type.clone()
            };
            temptype = t.temptype.to_string();
            let base = web_base(state);
            let style = format!("{base}/app/template/admin");
            info = json!({
                "id": t.id,
                "title": t.title,
                "header": t.header.replace("{admin_style}", &style),
                "body": t.body.replace("{admin_style}", &style),
                "footer": t.footer.replace("{admin_style}", &style),
                "type": ty,
                "temptype": t.temptype.to_string(),
            });
        }
    }
    let job_cols = column_form(&[
        ("jobcolumn_name", "wap_com_00288", "{职位名称}", "job_column"),
        ("jobcolumn_jobwapurl", "common_06663", "{职位网址}", "job_column"),
        ("jobcolumn_comname", "wap_com_00157", "{企业名称}", "job_column"),
        ("jobcolumn_comdesc", "common_01338", "{str|企业描述|length:200}", "job_column"),
        ("jobcolumn_comwapurl", "wap_com_00162", "{企业网址}", "job_column"),
        ("jobcolumn_salary", "wap_com_00290", "{薪资待遇}", "job_column"),
        ("jobcolumn_number", "wap_com_00333", "{招聘人数}", "job_column"),
        ("jobcolumn_age", "wap_com_00284", "{年龄要求}", "job_column"),
        ("jobcolumn_sex", "wap_com_00332", "{性别要求}", "job_column"),
        ("jobcolumn_exp", "wap_com_00287", "{经验要求}", "job_column"),
        ("jobcolumn_edu", "wap_com_00283", "{学历要求}", "job_column"),
        ("jobcolumn_city", "wap_js_00082", "{一级城市}{二级城市}{三级城市}", "job_column"),
        ("jobcolumn_address", "admin_system_00690", "{工作地点}", "job_column"),
        ("jobcolumn_phone", "wap_user_00265", "{联系电话}", "job_column"),
        ("jobcolumn_welfare", "wap_00286", "{职位福利}", "job_column"),
        ("jobcolumn_description", "common_01395", "{str|职位描述|length:200}", "job_column"),
    ]);
    let resume_cols = column_form(&[
        ("resumecolumn_username", "admin_00429", "{姓名}", "resume_column"),
        ("resumecolumn_age", "wap_com_00302", "{年龄}", "resume_column"),
        ("resumecolumn_exp", "wap_01424", "{经验}", "resume_column"),
        ("resumecolumn_edu", "wap_com_00301", "{学历}", "resume_column"),
        ("resumecolumn_name", "wap_user_00015", "{期望职位}", "resume_column"),
        ("resumecolumn_wapurl", "common_06666", "{简历网址}", "resume_column"),
        ("resumecolumn_salary", "wap_user_00016", "{期望薪资}", "resume_column"),
    ]);
    let company_cols = column_form(&[
        ("companycolumn_name", "wap_com_00157", "{企业名称}", "company_column"),
        ("companycolumn_desc", "common_01338", "{str|企业描述|length:200}", "company_column"),
        ("companycolumn_comwapurl", "wap_com_00162", "{企业网址}", "company_column"),
        ("companycolumn_linkman", "wap_js_00058", "{企业联系人}", "company_column"),
        ("companycolumn_linktel", "common_06667", "{企业联系电话}", "company_column"),
        ("companycolumn_jobname", "wap_com_00288", "{职位名称}", "company_column"),
        ("companycolumn_jobwapurl", "common_06663", "{职位网址}", "company_column"),
        ("companycolumn_jobsalary", "common_06669", "{职位薪资}", "company_column"),
    ]);
    let public_cols = column_form(&[
        ("wapewm", "common_06673", "{移动端二维码}", "public_column"),
        ("xcxurl", "common_01600", "{小程序外链}", "public_column"),
    ]);
    let total_cols = column_form(&[
        ("webname", "admin_system_00331", "{网站名称}", "total_column"),
        ("weburl", "admin_01014", "{网站地址}", "total_column"),
        ("datetime", "member_com_00309", "{当前日期}", "total_column"),
    ]);
    let mut typecolumn = job_cols.clone();
    typecolumn.extend(resume_cols);
    typecolumn.extend(company_cols);
    typecolumn.extend(public_cols.clone());
    typecolumn.extend(total_cols.clone());
    if temptype == "1" {
        typecolumn.retain(|v| {
            let data = v.get("data").and_then(|x| x.as_array());
            !data
                .and_then(|a| a.get(1))
                .and_then(|x| x.as_str())
                .map(|s| s.contains("{img") || s.contains("H5xcx_"))
                .unwrap_or(false)
        });
    }
    Ok(json!({
        "info": info,
        "temptype": temptype,
        "typecolumn": typecolumn,
        "totalcolumn": total_cols,
        "job_map": map_pairs(&[
            ("{职位名称}", "xx职位"),
            ("{职位网址}", "#"),
            ("{企业名称}", "xx企业"),
            ("{企业描述}", "企业简介"),
            ("{企业网址}", "#"),
            ("{薪资待遇}", "10000-15000"),
            ("{招聘人数}", "若干"),
            ("{年龄要求}", "不限"),
            ("{性别要求}", "不限"),
            ("{经验要求}", "不限"),
            ("{学历要求}", "不限"),
            ("{一级城市}", "省"),
            ("{二级城市}", "市"),
            ("{三级城市}", "区"),
            ("{联系电话}", "0527-83698666"),
            ("{工作地点}", "地址"),
            ("{职位福利}", "五险一金"),
            ("{职位描述}", "职位描述"),
        ]),
        "resume_map": map_pairs(&[
            ("{期望职位}", "xx职位"),
            ("{简历网址}", "#"),
            ("{姓名}", "张三"),
            ("{年龄}", "25"),
            ("{经验}", "3年"),
            ("{学历}", "本科"),
            ("{期望薪资}", "10000-18000"),
        ]),
        "company_map": map_pairs(&[
            ("{企业名称}", "xx企业"),
            ("{企业描述}", "企业简介"),
            ("{企业网址}", "#"),
            ("{职位名称}", "xx职位"),
            ("{职位网址}", "#"),
            ("{职位薪资}", "15000-25000"),
            ("{企业联系人}", "联系人"),
            ("{企业联系电话}", "18888888888"),
        ]),
        "public_map": map_pairs(&[
            ("{移动端二维码}", "#"),
            ("{小程序外链}", "https://wxaurl.cn/xxx"),
        ]),
        "total_map": map_pairs(&[
            ("{网站名称}", "网站名称"),
            ("{网站地址}", "https://zzzz.com"),
            ("{当前日期}", "2026-01-01"),
        ]),
    }))
}

fn replace_admin_style(s: &str, weburl: &str) -> String {
    let style = format!("{weburl}/app/template/admin");
    s.replace(&style, "{admin_style}")
        .replace("http://www.yunjob.com/app/template/admin", "{admin_style}")
}

async fn rewrite_mmbiz(state: &AppState, html: &str) -> String {
    if !html.contains("mmbiz.qpic.cn") || html.contains("mmbiz_svg") {
        return html.to_string();
    }
    let mut urls = Vec::new();
    let mut i = 0;
    while let Some(p) = html[i..].find("mmbiz.qpic.cn") {
        let abs = i + p;
        let start = html[..abs].rfind("http").unwrap_or(abs);
        let slice = &html[start..];
        let end = slice
            .find(|c: char| matches!(c, '"' | '\'' | ' ' | '>' | ')'))
            .unwrap_or(slice.len());
        let raw = &slice[..end];
        let clean = raw.split('?').next().unwrap_or(raw).to_string();
        if !clean.is_empty() && !urls.iter().any(|(a, _)| a == &clean) {
            urls.push((clean, raw.to_string()));
        }
        i = start + end;
        if i <= abs {
            i = abs + 1;
        }
    }
    let mut out = html.to_string();
    for (clean, raw) in urls {
        match state.http.get_bytes(&clean).await {
            Ok(bytes) => {
                let key = format!("wx/{}/{}", clock::now_ts(), Uuid::now_v7());
                if let Ok(stored) = state.storage.put(&key, "image/jpeg", bytes).await {
                    out = out.replace(&raw, &stored);
                }
            }
            Err(e) => tracing::warn!(error = %e, url = %clean, "mmbiz image fetch skipped"),
        }
    }
    out
}

async fn fabutool_wx_pub_temp_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let title = json_str(body, "title");
    if title.is_empty() {
        return Err(ApiError::business("wap_00203"));
    }
    let id = json_u64(body, "id");
    if gap_extra::php_count_wxpub_title(state.db.reader(), &title, id).await? > 0 {
        return Err(ApiError::business("admin_tool_00031"));
    }
    let web = web_base(state);
    let mut header = rewrite_mmbiz(state, &json_str(body, "header")).await;
    let mut body_html = rewrite_mmbiz(state, &json_str(body, "body")).await;
    let mut footer = rewrite_mmbiz(state, &json_str(body, "footer")).await;
    header = strip_yun_tags(replace_admin_style(&header, &web));
    body_html = strip_yun_tags(replace_admin_style(&body_html, &web));
    footer = strip_yun_tags(replace_admin_style(&footer, &web));
    let mut ty = json_str(body, "type");
    if id > 0 {
        if let Some(old) = gap_extra::php_find_wxpub_temp(state.db.reader(), id).await? {
            if old.r#type == "onejob" {
                ty = "onejob".into();
            }
        }
    }
    if ty.is_empty() {
        ty = "job".into();
    }
    gap_repo::upsert_wxpub_temp(
        state.db.pool(),
        if id > 0 { Some(id) } else { None },
        &title,
        &header,
        &body_html,
        &footer,
        &ty,
        json_i32(body, "temptype"),
        clock::now_ts(),
    )
    .await?;
    Ok(PhpOut::Message("api_wxapp_00007"))
}

async fn fabutool_wx_pub_temp_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::business("common_01066"));
    }
    recycle_ids(
        state,
        user,
        "wxpub_temps",
        &ids,
        "/v1/admin/php-content/fabutool/wxPubTempDel",
    )
    .await;
    let n = gap_repo::delete_wxpub_temps(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Text(
        "admin_user_00187",
        del_ids_msg("admin_model_00241", &ids),
    ))
}

async fn fabutool_pubtool(state: &AppState) -> AppResult<Value> {
    let db = state.db.reader();
    let temps = gap_extra::php_list_wxpub_temps_php(db, None, None, 0, 500).await?;
    let temps: Vec<Value> = temps
        .into_iter()
        .map(|t| {
            let ty = if t.r#type == "onejob" {
                "job".to_string()
            } else {
                t.r#type
            };
            json!({
                "id": t.id,
                "title": t.title,
                "type": ty,
                "temptype": t.temptype.to_string(),
            })
        })
        .collect();
    let rating: Vec<Value> = gap_extra::php_list_rating_names(db)
        .await?
        .into_iter()
        .map(|r| json!({ "id": r.id, "name": r.name }))
        .collect();
    let domains = domain_repo::list_all(db).await?;
    Ok(json!({
        "temps": temps,
        "rating": rating,
        "domain": domain_object(&domains),
    }))
}

async fn fabutool_get_wxpub_job(state: &AppState, body: &Value) -> AppResult<Value> {
    let kw = json_str(body, "keyword");
    let rows = gap_extra::php_search_wxpub_jobs(state.db.reader(), &kw).await?;
    Ok(Value::Array(
        rows.into_iter()
            .map(|r| json!({ "name": r.name, "value": r.value, "upname": r.upname }))
            .collect(),
    ))
}

async fn fabutool_get_com_search(state: &AppState, body: &Value) -> AppResult<Value> {
    let kw = json_str(body, "keyword");
    let rows = gap_extra::php_search_wxpub_coms(state.db.reader(), &kw).await?;
    Ok(Value::Array(
        rows.into_iter()
            .map(|r| json!({ "name": r.name, "value": r.value }))
            .collect(),
    ))
}

fn days_gt(code: i32) -> Option<i64> {
    if code <= 0 {
        return None;
    }
    let now = clock::now_ts();
    if code == 1 {
        Some(clock::start_of_day(now))
    } else {
        Some(now - i64::from(code) * 86_400)
    }
}

fn job_salary(min: i32, max: i32) -> String {
    if min <= 0 && max <= 0 {
        "面议".into()
    } else if max <= 0 || max == min {
        min.to_string()
    } else {
        format!("{min}-{max}")
    }
}

fn clip(s: &str, n: usize) -> String {
    let t = s.chars().take(n).collect::<String>();
    if s.chars().count() > n {
        format!("{t}...")
    } else {
        t
    }
}

fn apply_tokens(mut html: String, map: &[(&str, String)]) -> String {
    for (k, v) in map {
        html = html.replace(k, v);
    }
    html
}

async fn site_name(state: &AppState) -> String {
    setting_repo::find(state.db.reader(), "sy_webname")
        .await
        .ok()
        .flatten()
        .map(|s| s.value)
        .unwrap_or_else(|| "zzzz".into())
}

fn job_body_html(
    j: &gap_extra::PhpPubJobRow,
    dicts: &dict_service::LocalizedDicts,
    web: &str,
    tpl_body: &str,
) -> String {
    let phone = if j.linktel.is_empty() {
        j.linkphone.clone()
    } else {
        j.linktel.clone()
    };
    let desc = clip(&html_plain(&j.description), 200);
    let com_desc = clip(&html_plain(&j.content), 200);
    let welfare = dicts.welfare_labels(&j.welfare).join(" ");
    let sex = match j.sex {
        1 => "男",
        2 => "女",
        _ => "不限",
    };
    let map = vec![
        ("{职位名称}", j.name.clone()),
        (
            "{职位网址}",
            format!("{web}/index.php?m=wap&c=job&a=comapply&id={}", j.id),
        ),
        ("{企业名称}", j.com_name.clone()),
        ("{企业描述}", com_desc.clone()),
        ("{str|企业描述|length:200}", com_desc),
        (
            "{企业网址}",
            format!("{web}/index.php?m=wap&c=company&a=show&id={}", j.uid),
        ),
        ("{薪资待遇}", job_salary(j.minsalary, j.maxsalary)),
        ("common_01436", job_salary(j.minsalary, j.maxsalary)),
        ("{招聘人数}", if j.number > 0 { j.number.to_string() } else { "若干".into() }),
        ("{年龄要求}", if j.age.is_empty() { "不限".into() } else { j.age.clone() }),
        ("{性别要求}", sex.to_string()),
        ("{经验要求}", dicts.user_or_com(j.exp).to_string()),
        ("{学历要求}", dicts.user_or_com(j.edu).to_string()),
        ("{一级城市}", dicts.city(j.provinceid).to_string()),
        ("{二级城市}", dicts.city(j.cityid).to_string()),
        ("{三级城市}", dicts.city(j.three_cityid).to_string()),
        ("{联系电话}", phone),
        ("{工作地点}", j.address.clone()),
        ("common_01435", j.address.clone()),
        ("{职位福利}", welfare),
        ("{职位描述}", desc.clone()),
        ("{str|职位描述|length:200}", desc),
    ];
    strip_yun_tags(apply_tokens(tpl_body.to_string(), &map))
}

fn html_plain(s: &str) -> String {
    let mut out = String::new();
    let mut skip = false;
    for c in s.chars() {
        match c {
            '<' => skip = true,
            '>' => skip = false,
            _ if !skip => out.push(c),
            _ => {}
        }
    }
    out.replace("&nbsp;", " ").replace("&quot;", "\"")
}

async fn fabutool_get_pubtool(state: &AppState, body: &Value) -> AppResult<String> {
    let kind = json_str(body, "type");
    let tpl_id = json_u64(body, "tpl");
    if tpl_id == 0 {
        return Ok(msg_t("common_02409"));
    }
    let temp = match gap_extra::php_find_wxpub_temp(state.db.reader(), tpl_id).await? {
        Some(t) => t,
        None => return Ok(msg_t("common_02409")),
    };
    let dicts = dict_service::get(state).await?;
    let web = web_base(state);
    let num = json_u64(body, "num");
    let mut bodies = Vec::new();
    match kind.as_str() {
        "job" => {
            let param = json_str(body, "param");
            let ids = gap_extra::parse_id_csv(&json_str(body, "jobcopos").replace('，', ","));
            let rating = json_str(body, "rating");
            let keyword = json_str(body, "keyword");
            let welfare = json_str(body, "welfare");
            let f = gap_extra::PhpPubJobFilter {
                ids: if ids.is_empty() { None } else { Some(&ids) },
                rating: if rating.is_empty() { None } else { Some(rating.as_str()) },
                keyword: if keyword.is_empty() { None } else { Some(keyword.as_str()) },
                provinceid: json_opt_i32(body, "provinceid"),
                cityid: json_opt_i32(body, "cityid"),
                three_cityid: json_opt_i32(body, "three_cityid"),
                job1: json_opt_i32(body, "job1"),
                job1_son: json_opt_i32(body, "job1_son"),
                job_post: json_opt_i32(body, "job_post"),
                lastupdate_gt: days_gt(json_i32(body, "times")),
                sdate_gt: days_gt(json_i32(body, "ftimes")),
                xsdate: param.split(',').any(|x| x.trim() == "0"),
                urgent: param.split(',').any(|x| x.trim() == "1"),
                rec: param.split(',').any(|x| x.trim() == "2"),
                minsalary: json_opt_i32(body, "minsalary"),
                maxsalary: json_opt_i32(body, "maxsalary"),
                welfare: if welfare.is_empty() { None } else { Some(welfare.as_str()) },
                now: clock::now_ts(),
            };
            let rows = gap_extra::php_list_pubtool_jobs(state.db.reader(), &f, num).await?;
            for j in &rows {
                bodies.push(job_body_html(j, &dicts, &web, &temp.body));
            }
        }
        "resume" => {
            let rows = gap_extra::php_list_pubtool_resumes(
                state.db.reader(),
                num,
                days_gt(json_i32(body, "rltimes")),
                days_gt(json_i32(body, "rtimes")),
                json_opt_i32(body, "whole"),
            )
            .await?;
            for r in &rows {
                let age = age_of(&r.birthday);
                let map = vec![
                    ("{期望职位}", r.name.clone()),
                    (
                        "{简历网址}",
                        format!("{web}/index.php?m=wap&c=resume&a=show&id={}", r.id),
                    ),
                    ("{姓名}", r.uname.clone()),
                    ("{年龄}", age),
                    ("{经验}", dicts.user_or_com(r.exp).to_string()),
                    ("{学历}", dicts.user_or_com(r.edu).to_string()),
                    ("{期望薪资}", r.salary.clone()),
                    ("{头像}", r.photo.clone()),
                ];
                bodies.push(strip_yun_tags(apply_tokens(temp.body.clone(), &map)));
            }
        }
        "company" => {
            let cuids = gap_extra::parse_id_csv(
                &json_str(body, "copos").replace('，', ","),
            );
            let rating = json_str(body, "rating");
            let rows = gap_extra::php_list_pubtool_companies(
                state.db.reader(),
                &cuids,
                if rating.is_empty() { None } else { Some(rating.as_str()) },
                num,
            )
            .await?;
            let rule = json_u64(body, "rule");
            for c in &rows {
                let jobs = gap_extra::php_list_pubtool_jobs_by_uid(
                    state.db.reader(),
                    c.uid,
                    if rule == 0 { 5 } else { rule },
                )
                .await?;
                let desc = clip(&html_plain(&c.content), 200);
                let mut job_html = String::new();
                for j in &jobs {
                    job_html.push_str(&format!(
                        "{} {} {}\n",
                        j.name,
                        job_salary(j.minsalary, j.maxsalary),
                        format!("{web}/index.php?m=wap&c=job&a=comapply&id={}", j.id)
                    ));
                }
                let map = vec![
                    ("{企业名称}", c.name.clone()),
                    ("{企业描述}", desc.clone()),
                    ("{str|企业描述|length:200}", desc),
                    (
                        "{企业网址}",
                        format!("{web}/index.php?m=wap&c=company&a=show&id={}", c.uid),
                    ),
                    ("{企业联系人}", c.linkman.clone()),
                    ("{企业联系电话}", c.linktel.clone()),
                    ("{工作地点}", c.address.clone()),
                    ("common_01197", c.address.clone()),
                    ("{职位名称}", jobs.first().map(|j| j.name.clone()).unwrap_or_default()),
                    ("{职位网址}", jobs.first().map(|j| format!("{web}/index.php?m=wap&c=job&a=comapply&id={}", j.id)).unwrap_or_default()),
                    ("{职位薪资}", jobs.first().map(|j| job_salary(j.minsalary, j.maxsalary)).unwrap_or_default()),
                    ("{职位描述}", job_html),
                ];
                bodies.push(strip_yun_tags(apply_tokens(temp.body.clone(), &map)));
            }
        }
        _ => return Ok(msg_t("common_02409")),
    }
    render_wxpub_assembled(state, &temp, &bodies).await
}

fn age_of(birthday: &str) -> String {
    let y = birthday.get(0..4).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    let now_y = chrono::Local::now().year();
    if y > 1900 && y <= now_y {
        (now_y - y).to_string()
    } else {
        String::new()
    }
}

async fn render_wxpub_assembled(
    state: &AppState,
    temp: &phpyun_models::admin_gap::entity::WxpubTempRow,
    bodies: &[String],
) -> AppResult<String> {
    let web = web_base(state);
    let name = site_name(state).await;
    let today = fmt_ts(clock::now_ts(), "%Y-%m-%d");
    let style = format!("{web}/app/template/admin");
    let globals = vec![
        ("{admin_style}", style),
        ("{网站名称}", name),
        ("{网站地址}", web),
        ("{当前日期}", today.clone()),
        ("common_01432", today),
    ];
    let header = strip_yun_tags(apply_tokens(temp.header.clone(), &globals));
    let footer = strip_yun_tags(apply_tokens(temp.footer.clone(), &globals));
    let mut html = header;
    let enter = if temp.temptype == 1 { "\r\n" } else { "" };
    for b in bodies {
        html.push_str(b);
        html.push_str(enter);
    }
    html.push_str(&footer);
    if temp.temptype == 1 {
        html = html.replace('\n', "</br>");
    }
    Ok(html)
}

async fn fabutool_get_tw(state: &AppState, body: &Value) -> AppResult<String> {
    let tpl = json_u64(body, "tpl");
    let ids = gap_extra::parse_id_csv(&json_str(body, "jobids"));
    if tpl == 0 || ids.is_empty() {
        return Ok(String::new());
    }
    let temp = match gap_extra::php_find_wxpub_temp(state.db.reader(), tpl).await? {
        Some(t) => t,
        None => return Ok(String::new()),
    };
    let dicts = dict_service::get(state).await?;
    let web = web_base(state);
    let f = gap_extra::PhpPubJobFilter {
        ids: Some(&ids),
        rating: None,
        keyword: None,
        provinceid: None,
        cityid: None,
        three_cityid: None,
        job1: None,
        job1_son: None,
        job_post: None,
        lastupdate_gt: None,
        sdate_gt: None,
        xsdate: false,
        urgent: false,
        rec: false,
        minsalary: None,
        maxsalary: None,
        welfare: None,
        now: clock::now_ts(),
    };
    let rows = gap_extra::php_list_pubtool_jobs(state.db.reader(), &f, 200).await?;
    let mut by_id = std::collections::HashMap::new();
    for j in rows {
        by_id.insert(j.id, j);
    }
    let mut bodies = Vec::new();
    for id in ids {
        if let Some(j) = by_id.get(&id) {
            bodies.push(job_body_html(j, &dicts, &web, &temp.body));
        }
    }
    render_wxpub_assembled(state, &temp, &bodies).await
}

async fn fabutool_get_com_tw(state: &AppState, body: &Value) -> AppResult<String> {
    let tpl = json_u64(body, "tpl");
    let cuids = gap_extra::parse_id_csv(&json_str(body, "cuids").replace('，', ","));
    if tpl == 0 || cuids.is_empty() {
        return Ok(String::new());
    }
    let mut b = body.clone();
    if let Some(obj) = b.as_object_mut() {
        obj.insert("type".into(), json!("company"));
        obj.insert("copos".into(), json!(cuids.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")));
        obj.insert("tpl".into(), json!(tpl));
    }
    fabutool_get_pubtool(state, &b).await
}

async fn fabutool_tw_task(state: &AppState, body: &Value, kind: i32) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let welfare = json_str(body, "welfarekeyword");
    let status_in = json_i32(body, "status");
    let status = match status_in {
        1 => Some(1),
        2 => Some(0),
        _ => None,
    };
    let order_t = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let f = gap_extra::PhpTwTaskFilter {
        kind,
        keyword: if kw.is_empty() { None } else { Some(kw.as_str()) },
        welfare: if welfare.is_empty() { None } else { Some(welfare.as_str()) },
        auid: {
            let n = json_u64(body, "auid");
            if n > 0 { Some(n) } else { None }
        },
        status,
        urgent: json_opt_i32(body, "urgent"),
        wcmoments: json_opt_i32(body, "wcmoments"),
        gzh: json_opt_i32(body, "gzh"),
        order_t: &order_t,
        order_dir: &order_dir,
    };
    let db = state.db.reader();
    let total = gap_extra::php_count_twtasks(db, &f).await?;
    let rows = if total > 0 {
        gap_extra::php_list_twtasks(db, &f, offset, limit).await?
    } else {
        Vec::new()
    };
    let base = web_base(state);
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "jobid": r.jobid,
                "cuid": r.cuid,
                "jobname": r.jobname,
                "comname": r.comname,
                "jobsdate": r.jobsdate,
                "jobsdate_n": if r.jobsdate > 0 { fmt_ts(r.jobsdate, "%Y-%m-%d %H:%M") } else { String::new() },
                "auid": r.auid,
                "content": r.content,
                "urgent": r.urgent.to_string(),
                "wcmoments": r.wcmoments.to_string(),
                "gzh": r.gzh.to_string(),
                "status": r.status.to_string(),
                "ctime": r.ctime,
                "ctime_n": if r.ctime > 0 { fmt_ts(r.ctime, "%Y-%m-%d %H:%M") } else { String::new() },
                "type": r.r#type,
                "etime": r.etime,
                "jobstatus": r.job_off.to_string(),
                "comstatus": if r.com_r_status == 1 { 1 } else { 2 },
                "admin_username": r.admin_username,
                "comurl": format!("{base}/index.php?m=company&c=show&id={}", r.cuid),
                "joburl": format!("{base}/index.php?m=job&c=comapply&look=admin&id={}", r.jobid),
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

async fn fabutool_tw_base(state: &AppState, job: bool) -> AppResult<Value> {
    let db = state.db.reader();
    let types: &[&str] = if job { &["job", "onejob"] } else { &["company"] };
    let temps = gap_extra::php_list_wxpub_temp_titles(db, 1, types).await?;
    let temps2 = gap_extra::php_list_wxpub_temp_titles(db, 0, if job { &["job"] } else { &["company"] }).await?;
    let admins = gap_extra::php_list_admin_names(db).await?;
    Ok(json!({
        "temps": temps.into_iter().map(|t| json!({ "id": t.id, "title": t.name })).collect::<Vec<_>>(),
        "temps2": temps2.into_iter().map(|t| json!({ "id": t.id, "title": t.name })).collect::<Vec<_>>(),
        "adminList": admins.into_iter().map(|a| json!({ "uid": a.uid, "username": a.username, "name": a.name })).collect::<Vec<_>>(),
    }))
}

async fn fabutool_del_tw(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_named(body, "del");
    if ids.is_empty() {
        return Err(ApiError::business("common_01066"));
    }
    recycle_ids(
        state,
        user,
        "wxpub_twtask",
        &ids,
        "/v1/admin/php-content/fabutool/delTwTask",
    )
    .await;
    let n = gap_extra::php_delete_twtasks(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("wap_user_00147"))
}

async fn fabutool_task_finish(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("wap_00556"));
    }
    let n = gap_extra::php_finish_twtasks(state.db.pool(), &ids, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::business("admin_system_00397"));
    }
    Ok(PhpOut::Message("wap_user_00264"))
}

fn tuiguang_needles(keys: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for key in keys {
        if !out.iter().any(|x| x == key) {
            out.push((*key).to_string());
        }
        for lang in [i18n::Lang::ZhCN, i18n::Lang::En, i18n::Lang::ZhTW] {
            let t = i18n::t(&format!("messages.{key}"), lang);
            if !t.is_empty() && t != format!("messages.{key}") && !out.iter().any(|x| x == &t) {
                out.push(t);
            }
        }
    }
    out
}

fn tuiguang_slot(ctime: Option<(String, i64)>) -> Value {
    match ctime {
        Some((title, ts)) if ts > 0 => json!({ "ctime": ts, "ctime_n": fmt_dt(ts), "title": title }),
        _ => json!({}),
    }
}

async fn tuiguang_index(state: &AppState, sms: bool) -> AppResult<Value> {
    let pool = state.db.reader();
    let slots: [(&str, &[&str]); 7] = if sms {
        [
            ("anniversary", &["admin_yunying_00045"]),
            ("todaydue", &["admin_yunying_00034"]),
            ("sevendue", &["admin_yunying_00035"]),
            ("useradd", &["admin_yunying_00040"]),
            ("userup", &["admin_yunying_00038"]),
            ("addjob", &["admin_yunying_00222"]),
            ("upjob", &["admin_yunying_00039"]),
        ]
    } else {
        [
            ("anniversary", &["admin_yunying_00044"]),
            ("todaydue", &["admin_yunying_00023"]),
            ("sevendue", &["admin_yunying_00036", "admin_yunying_00041"]),
            ("useradd", &["admin_yunying_00040"]),
            ("userup", &["admin_01403"]),
            ("addjob", &["admin_yunying_00222"]),
            ("upjob", &["admin_yunying_00039"]),
        ]
    };
    let mut out = serde_json::Map::new();
    for (name, keys) in slots {
        let likes = tuiguang_needles(keys);
        let hit = if sms {
            moblie_msg_repo::latest_ctime_content_likes(pool, &likes).await?
        } else {
            email_msg_repo::latest_ctime_title_likes(pool, &likes).await?
        };
        out.insert(name.to_string(), tuiguang_slot(hit));
    }
    Ok(Value::Object(out))
}

async fn tuiguang_birthday(state: &AppState, body: &Value) -> AppResult<Value> {
    let now = clock::now_ts();
    let day7 = clock::start_of_day(now - 7 * 86_400);
    let email = json_str(body, "type") != "moblie";
    let n = gap_extra::php_promo_birthday(state.db.reader(), email, now, day7, day7 + 86_400 - 1)
        .await?;
    if email {
        Ok(json!({
            "anniversary_e": n.anniversary,
            "todaydue_e": n.todaydue,
            "sevendue_e": n.sevendue,
            "useradd_e": n.useradd,
            "userup_e": n.userup,
            "addjob_e": n.addjob,
            "upjob_e": n.upjob,
        }))
    } else {
        Ok(json!({
            "anniversary_m": n.anniversary,
            "todaydue_m": n.todaydue,
            "sevendue_m": n.sevendue,
            "useradd_m": n.useradd,
            "userup_m": n.userup,
            "addjob_m": n.addjob,
            "upjob_m": n.upjob,
        }))
    }
}

async fn tuiguang_getcom(state: &AppState, body: &Value) -> AppResult<u64> {
    let email = json_i32(body, "msgType") != 2;
    Ok(gap_extra::php_count_promo_com(
        state.db.reader(),
        json_i32(body, "com"),
        email,
        clock::now_ts(),
    )
    .await?)
}

async fn tuiguang_getuser(state: &AppState, body: &Value) -> AppResult<u64> {
    let email = json_i32(body, "msgType") == 1;
    Ok(gap_extra::php_count_promo_user(
        state.db.reader(),
        json_i32(body, "user"),
        email,
        clock::now_ts(),
    )
    .await?)
}

async fn tuiguang_getjob(state: &AppState, body: &Value) -> AppResult<u64> {
    Ok(gap_extra::php_count_promo_job(state.db.reader(), json_i32(body, "job"), clock::now_ts()).await?)
}

fn promo_progress(total: usize, pagesize: usize, value: usize, sendok: u64, sendno: u64, email: bool) -> Value {
    let page = if value == 0 { 1 } else { value + 1 };
    if total > pagesize * page {
        let spage = page * pagesize + 1;
        let topage = (page + 1) * pagesize;
        let unit = if email {
            msg_t("admin_01409")
        } else {
            msg_t("admin_01410")
        };
        json!({
            "error": 3,
            "msg": format!("{}{spage}-{topage}{unit}", msg_t("admin_user_00374")),
            "data": { "value": page, "sendok": sendok, "sendno": sendno }
        })
    } else {
        json!({
            "error": 0,
            "msg": format!(
                "{}{}{}{}",
                msg_t("admin_user_00013"),
                sendok,
                msg_t("admin_user_00014"),
                sendno
            ),
            "data": { "value": 0, "sendok": sendok, "sendno": sendno }
        })
    }
}

fn city_ids_of(raw: &str) -> Vec<i32> {
    raw.split(|c: char| c == ',' || c == '，' || c.is_whitespace())
        .filter_map(|p| p.trim().parse::<i32>().ok())
        .filter(|n| *n > 0)
        .collect()
}

async fn tuiguang_sendresume(state: &AppState, body: &Value) -> AppResult<Value> {
    let stype = json_i32(body, "stype");
    let email = stype == 1;
    if !email {
        let cfg = settings_hash(state).await.unwrap_or_default();
        if cfg_pick(&cfg, "sy_msg_isopen") != "1" {
            return Err(ApiError::business("admin_user_00011"));
        }
    }
    let now = clock::now_ts();
    let sendnum = json_u64(body, "sendnum").clamp(1, 500);
    let coms = gap_extra::php_list_promo_coms(
        state.db.reader(),
        json_i32(body, "com"),
        sendnum,
        email,
        now,
    )
    .await?;
    if coms.is_empty() {
        return Err(ApiError::business(if email {
            "admin_user_00003"
        } else {
            "admin_user_00004"
        }));
    }
    let pagesize = json_u64(body, "pagelimit").clamp(1, 100) as usize;
    let value = json_u64(body, "value") as usize;
    let mut sendok = json_u64(body, "sendok");
    let mut sendno = json_u64(body, "sendno");
    let start = if value == 0 { 0 } else { value * pagesize };
    let title = json_str(body, "email_title");
    let content = json_str(body, "content");
    let resume_kind = json_i32(body, "resume");
    let num = json_u64(body, "num");
    let dicts = dict_service::get(state).await?;
    let web = web_base(state);
    let tel = setting_repo::find(state.db.reader(), "sy_freewebtel")
        .await
        .ok()
        .flatten()
        .map(|s| s.value)
        .unwrap_or_default();
    for c in coms.iter().skip(start).take(pagesize) {
        if email {
            let expects =
                gap_extra::php_list_expects_by_hy(state.db.reader(), c.hy, resume_kind, num).await?;
            let html = resume_promo_html(&expects, &dicts, &web, &tel);
            if html.is_empty() {
                continue;
            }
            let ok = mail_service::send_text(state, &c.linkmail, &title, &html)
                .await
                .is_ok();
            let _ = gap_repo::insert_email_log(
                state.db.pool(),
                c.uid,
                &c.linkmail,
                &title,
                &html,
                now,
                if ok { 1 } else { 0 },
            )
            .await;
            if ok {
                sendok += 1;
            } else {
                sendno += 1;
            }
        } else {
            let _ = gap_repo::insert_sms_log(state.db.pool(), c.uid, &c.linktel, &content, now, 1)
                .await;
            let _ = state
                .events
                .publish_json(
                    "sms.admin_queued",
                    &json!({"kind": "tuiguang_resume", "uid": c.uid, "mobile": c.linktel}),
                )
                .await;
            sendok += 1;
        }
    }
    Ok(promo_progress(coms.len(), pagesize, value, sendok, sendno, email))
}

async fn tuiguang_sendjob(state: &AppState, body: &Value) -> AppResult<Value> {
    let stype = json_i32(body, "stype");
    let email = stype == 1;
    if !email {
        let cfg = settings_hash(state).await.unwrap_or_default();
        if cfg_pick(&cfg, "sy_msg_isopen") != "1" {
            return Err(ApiError::business("admin_user_00011"));
        }
    }
    let now = clock::now_ts();
    let sendnum = json_u64(body, "sendnum").clamp(1, 500);
    let users = gap_extra::php_list_promo_users(
        state.db.reader(),
        json_i32(body, "user"),
        sendnum,
        email,
        now,
    )
    .await?;
    if users.is_empty() {
        return Err(ApiError::business(if email {
            "admin_user_00003"
        } else {
            "admin_user_00004"
        }));
    }
    let pagesize = json_u64(body, "pagelimit").clamp(1, 100) as usize;
    let value = json_u64(body, "value") as usize;
    let mut sendok = json_u64(body, "sendok");
    let mut sendno = json_u64(body, "sendno");
    let start = if value == 0 { 0 } else { value * pagesize };
    let title = json_str(body, "email_title");
    let content = json_str(body, "content");
    let job_kind = json_i32(body, "job");
    let num = json_u64(body, "num");
    let dicts = dict_service::get(state).await?;
    let web = web_base(state);
    let tel = setting_repo::find(state.db.reader(), "sy_freewebtel")
        .await
        .ok()
        .flatten()
        .map(|s| s.value)
        .unwrap_or_default();
    let uids: Vec<u64> = users.iter().map(|u| u.uid).collect();
    let prefs = gap_extra::php_expect_city_hy(state.db.reader(), &uids).await?;
    let mut hy_city = std::collections::HashMap::new();
    for (uid, hy, city) in prefs {
        hy_city.insert(uid, (hy, city));
    }
    for u in users.iter().skip(start).take(pagesize) {
        if email {
            let (hy, city) = hy_city.get(&u.uid).cloned().unwrap_or((0, String::new()));
            let cities = city_ids_of(&city);
            let jobs =
                gap_extra::php_list_jobs_by_hy_city(state.db.reader(), hy, &cities, job_kind, num, now)
                    .await?;
            let html = job_promo_html(&jobs, &dicts, &web, &tel);
            if html.is_empty() {
                continue;
            }
            let ok = mail_service::send_text(state, &u.email, &title, &html)
                .await
                .is_ok();
            let _ = gap_repo::insert_email_log(
                state.db.pool(),
                u.uid,
                &u.email,
                &title,
                &html,
                now,
                if ok { 1 } else { 0 },
            )
            .await;
            if ok {
                sendok += 1;
            } else {
                sendno += 1;
            }
        } else {
            let _ = gap_repo::insert_sms_log(state.db.pool(), u.uid, &u.telphone, &content, now, 1)
                .await;
            let _ = state
                .events
                .publish_json(
                    "sms.admin_queued",
                    &json!({"kind": "tuiguang_job", "uid": u.uid, "mobile": u.telphone}),
                )
                .await;
            sendok += 1;
        }
    }
    Ok(promo_progress(users.len(), pagesize, value, sendok, sendno, email))
}

fn resume_promo_html(
    rows: &[gap_extra::PhpPromoExpectRow],
    dicts: &dict_service::LocalizedDicts,
    web: &str,
    tel: &str,
) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let mut html = format!(
        "<table width=\"800\" border=\"0\" style=\"border:1px solid #ddd\" cellpadding=\"5\" cellspacing=\"0\">\
         <tr><td colspan=\"6\">{web} {tel}</td></tr>\
         <tr style=\"background:#f8f8f8;font-weight:bold\"><td>姓名</td><td>年龄</td><td>学历</td><td>经验</td><td>性别</td><td>操作</td></tr>"
    );
    for r in rows {
        let sex = match r.sex {
            1 => "男",
            2 => "女",
            _ => "不限",
        };
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{sex}</td>\
             <td><a href=\"{web}/index.php?m=resume&c=show&id={}\">查看</a></td></tr>",
            r.uname,
            age_of(&r.birthday),
            dicts.user_or_com(r.edu),
            dicts.user_or_com(r.exp),
            r.id
        ));
    }
    html.push_str("</table>");
    html
}

fn job_promo_html(
    rows: &[gap_extra::PhpPromoJobMini],
    dicts: &dict_service::LocalizedDicts,
    web: &str,
    tel: &str,
) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let mut html = format!(
        "<table width=\"800\" border=\"0\" style=\"border:1px solid #ddd\" cellpadding=\"5\" cellspacing=\"0\">\
         <tr><td colspan=\"7\">{web} {tel}</td></tr>\
         <tr style=\"background:#f8f8f8;font-weight:bold\"><td>职位</td><td>地点</td><td>薪资</td><td>学历</td><td>经验</td><td>性别</td><td>操作</td></tr>"
    );
    for j in rows {
        let sex = match j.sex {
            1 => "男",
            2 => "女",
            _ => "不限",
        };
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{sex}</td>\
             <td><a href=\"{web}/index.php?m=job&c=comapply&id={}\">查看</a></td></tr>",
            clip(&j.name, 12),
            dicts.city(j.cityid),
            job_salary(j.minsalary, j.maxsalary),
            dicts.user_or_com(j.edu),
            dicts.user_or_com(j.exp),
            j.id
        ));
    }
    html.push_str("</table>");
    html
}

fn is_alnum_dir(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn pic_opt(body: &Value) -> Option<String> {
    let p = json_str(body, "pic");
    if p.is_empty() {
        None
    } else {
        Some(p)
    }
}

fn parse_ymd_start(s: &str) -> i64 {
    chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or(0)
}

async fn tplset_index(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let sy_style = cfg_pick(&cfg, "style");
    let mut list = Vec::new();
    let mut imgarr = Vec::new();
    for mut row in domain_style_list() {
        let dir = row
            .get("dir")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let img = row
            .get("img")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let img = if img.is_empty() {
            format!("../app/template/{dir}/images/preview.jpg")
        } else {
            img
        };
        let img_n = tpl_preview_url(&cfg, &img);
        if let Some(obj) = row.as_object_mut() {
            obj.insert("img".into(), json!(img_n));
        }
        imgarr.push(json!(img_n));
        list.push(row);
    }
    Ok(json!({
        "list": list,
        "sy_style": sy_style,
        "imgarr": imgarr,
    }))
}

fn tplset_stylesave(body: &Value) -> AppResult<PhpOut> {
    let dir = json_str(body, "dir");
    if !is_alnum_dir(&dir) {
        return Err(ApiError::business("admin_system_00055"));
    }
    let root = std::path::Path::new("/www/wwwroot/zzzz.com/web/apps/site/public/skins").join(&dir);
    if root.is_dir() {
        let name = json_str(body, "name");
        let author = json_str(body, "author");
        let img = format!("/skins/{dir}/preview.svg");
        let text = format!("{name}||{author}||{dir}||{img}");
        std::fs::write(root.join("info.txt"), text).map_err(ApiError::internal)?;
    }
    Ok(PhpOut::Message("admin_01399"))
}

async fn tplset_check_style(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let dir = json_str(body, "dir");
    if dir.is_empty() {
        return Err(ApiError::business("admin_system_00057"));
    }
    if !is_alnum_dir(&dir) {
        return Err(ApiError::business("admin_system_00055"));
    }
    setting_repo::upsert(state.db.pool(), "style", &dir, "", true, clock::now_ts()).await?;
    home_service::invalidate_all().await;
    Ok(PhpOut::Message("admin_system_00056"))
}

fn tpl_row_json(cfg: &HashMap<String, String>, r: &gap_extra::PhpAdminTplRow, index: bool) -> Value {
    let pic_n = tpl_preview_url(cfg, &r.pic);
    let mut out = json!({
        "id": r.id,
        "name": r.name,
        "url": r.url,
        "pic": r.pic,
        "pic_n": pic_n,
        "status": r.status,
        "status_n": if r.status == 1 { "开启" } else { "关闭" },
        "price": r.price,
        "service_uid": r.service_uid,
    });
    if index {
        let st = if r.stime > 0 { fmt_date(r.stime) } else { String::new() };
        let et = if r.etime > 0 { fmt_date(r.etime) } else { String::new() };
        out["height"] = json!(r.height);
        out["se"] = json!(r.se);
        out["stime"] = json!(r.stime);
        out["etime"] = json!(r.etime);
        out["strtimes"] = if st.is_empty() {
            json!([])
        } else {
            json!([st, et])
        };
    }
    out
}

async fn tplset_comtpl(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let rows = gap_extra::php_list_company_tpls(state.db.reader()).await?;
    let list: Vec<Value> = rows.iter().map(|r| tpl_row_json(&cfg, r, false)).collect();
    let imgarr: Vec<Value> = list.iter().map(|v| v["pic_n"].clone()).collect();
    Ok(json!({ "list": list, "imgarr": imgarr }))
}

async fn tplset_resume_list(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let rows = gap_extra::php_list_resume_tpls(state.db.reader()).await?;
    let list: Vec<Value> = rows.iter().map(|r| tpl_row_json(&cfg, r, false)).collect();
    let imgarr: Vec<Value> = list.iter().map(|v| v["pic_n"].clone()).collect();
    Ok(json!({ "list": list, "imgarr": imgarr }))
}

async fn tplset_index_list(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let rows = gap_extra::php_list_index_tpls(state.db.reader()).await?;
    let list: Vec<Value> = rows.iter().map(|r| tpl_row_json(&cfg, r, true)).collect();
    let imgarr: Vec<Value> = list.iter().map(|v| v["pic_n"].clone()).collect();
    Ok(json!({ "list": list, "imgarr": imgarr }))
}

async fn tplset_com_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let url = json_str(body, "url");
    if !is_alnum_dir(&url) {
        return Err(ApiError::business("admin_system_00055"));
    }
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let pic = pic_opt(body);
    gap_extra::php_upsert_company_tpl(
        state.db.pool(),
        json_u64(body, "id"),
        &name,
        &url,
        pic.as_deref(),
        json_i32(body, "status"),
        &json_str(body, "price"),
        &json_str(body, "service_uid"),
    )
    .await?;
    Ok(PhpOut::Message("api_wxapp_00007"))
}

async fn tplset_resume_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let url = json_str(body, "url");
    if !is_alnum_dir(&url) {
        return Err(ApiError::business("admin_system_00055"));
    }
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let pic = pic_opt(body);
    gap_extra::php_upsert_resume_tpl(
        state.db.pool(),
        json_u64(body, "id"),
        &name,
        &url,
        pic.as_deref(),
        json_i32(body, "status"),
        &json_str(body, "price"),
        &json_str(body, "service_uid"),
    )
    .await?;
    Ok(PhpOut::Message("api_wxapp_00007"))
}

async fn tplset_index_save(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let times = json_str_list(body, "time");
    let stime = times.first().map(|s| parse_ymd_start(s)).unwrap_or(0);
    let etime = times
        .get(1)
        .or(times.first())
        .map(|s| parse_ymd_start(s))
        .unwrap_or(0);
    let etime = if etime > 0 { etime + 86399 } else { 0 };
    let pic = pic_opt(body);
    gap_extra::php_upsert_index_tpl(
        state.db.pool(),
        json_u64(body, "id"),
        &name,
        pic.as_deref(),
        json_i32(body, "status"),
        json_i32(body, "height"),
        json_i32(body, "se"),
        stime,
        etime,
    )
    .await?;
    Ok(PhpOut::Message("api_wxapp_00007"))
}

async fn tplset_com_del(state: &AppState, user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01843"));
    }
    recycle_ids(state, user, "company_tpl", &ids, "/admin/api/php-admin").await;
    gap_extra::php_delete_company_tpls(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("wap_user_00147"))
}

async fn tplset_resume_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01843"));
    }
    recycle_ids(state, user, "resumetpl", &ids, "/admin/api/php-admin").await;
    gap_extra::php_delete_resume_tpls(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("wap_user_00147"))
}

async fn tplset_index_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::business("common_01843"));
    }
    recycle_ids(state, user, "tplindex", &ids, "/admin/api/php-admin").await;
    gap_extra::php_delete_index_tpls(state.db.pool(), &ids).await?;
    Ok(PhpOut::Message("wap_user_00147"))
}

async fn db_opt_table(state: &AppState) -> AppResult<Value> {
    let rows = gap_extra::php_show_table_status(state.db.reader()).await?;
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "name": r.name,
                "type": r.engine,
                "num": r.rows,
                "size": format!(" {:.2} KB", r.data_length as f64 / 1024.0),
                "rec_index": r.index_length,
                "chip": r.data_free,
                "status": "",
                "charset": r.collation,
            })
        })
        .collect();
    Ok(json!(list))
}

async fn db_table_names(state: &AppState) -> AppResult<Value> {
    let rows = gap_extra::php_show_table_status(state.db.reader()).await?;
    let names: Vec<Value> = rows.into_iter().map(|r| json!({ "name": r.name })).collect();
    let db_length = names.len();
    let db_table: Vec<Value> = names.chunks(4).map(|c| json!(c)).collect();
    Ok(json!({ "dbTable": db_table, "dbLength": db_length }))
}

async fn db_optimize(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let name = json_str(body, "name");
    if !gap_extra::is_safe_phpyun_table(&name) {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let ty = json_i32(body, "type");
    let repair = ty == 2;
    if ty != 2 && ty != 3 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = gap_extra::php_optimize_table(state.db.pool(), &name, repair).await?;
    let prefix = if repair {
        "admin_tool_00008"
    } else {
        "admin_tool_00007"
    };
    let suffix = if n > 0 {
        "admin_tool_00502"
    } else {
        "admin_tool_00501"
    };
    let msg = format!("{}{}{}", msg_t(prefix), name, msg_t(suffix));
    if n > 0 {
        Ok(PhpOut::Text(prefix, msg))
    } else {
        Err(ApiError::business(suffix))
    }
}

async fn db_clear(state: &AppState, body: &Value) -> AppResult<Value> {
    let table = json_str(body, "clearTable");
    let days = json_i64(body, "clearTime");
    if table.is_empty() || days <= 0 {
        return Ok(json!({
            "error": 1,
            "msg": msg_t("admin_tool_00002"),
        }));
    }
    let before = clock::now_ts() - days * 86400;
    let (deleted, total) = gap_extra::php_clear_old(state.db.pool(), &table, before, 1000).await?;
    let label = format!(
        "{}{}{}",
        msg_t("admin_tool_00012"),
        table,
        if deleted == 0 {
            msg_t("admin_tool_00002")
        } else {
            msg_t("admin_tool_00013")
        }
    );
    if deleted == 0 {
        Ok(json!({ "error": 1, "msg": label }))
    } else if total.saturating_sub(1000) > 0 {
        Ok(json!({ "error": 2, "msg": label }))
    } else {
        Ok(json!({ "error": 0, "msg": label }))
    }
}

async fn gen_page_base(state: &AppState) -> AppResult<Value> {
    let cfg = settings_hash(state).await.unwrap_or_default();
    let groups = article_repo::list_groups(state.db.reader()).await?;
    let news_group_list: Vec<Value> = groups
        .into_iter()
        .map(|g| json!({ "id": g.id, "name": g.name, "keyid": g.keyid }))
        .collect();
    let descs = gap_extra::php_list_desc_names(state.db.reader()).await?;
    let description_list: Vec<Value> = descs
        .into_iter()
        .map(|(id, name)| json!({ "id": id, "name": name }))
        .collect();
    Ok(json!({
        "config": {
            "make_index_url": cfg_pick(&cfg, "make_index_url"),
            "make_new_url": cfg_pick(&cfg, "make_new_url"),
        },
        "news_group_list": news_group_list,
        "description_list": description_list,
    }))
}

async fn gen_page_index(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let url = json_str(body, "make_index_url");
    if !url.is_empty() {
        upsert_cfg(state, user, "make_index_url", &url).await?;
    }
    Ok(PhpOut::Message("admin_01465"))
}

async fn gen_page_news(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let url = json_str(body, "make_new_url");
    if !url.is_empty() {
        upsert_cfg(state, user, "make_new_url", &url).await?;
    }
    Ok(PhpOut::Message("admin_01465"))
}

fn gen_ssr_ok() -> Value {
    json!({ "type": "ok", "value": 0 })
}

fn gen_cache_index() -> Value {
    json!([
        {"id": "1", "name": "区域分类"},
        {"id": "2", "name": "行业分类"},
        {"id": "3", "name": "职位分类"},
        {"id": "4", "name": "个人分类"},
        {"id": "5", "name": "企业分类"},
        {"id": "6", "name": "分站缓存"},
        {"id": "7", "name": "网站缓存"},
        {"id": "8", "name": "SEO设置"},
        {"id": "9", "name": "网站导航"},
        {"id": "10", "name": "兼职分类"},
        {"id": "11", "name": "友情链接"},
        {"id": "12", "name": "新闻分类"},
        {"id": "13", "name": "商品分类"},
        {"id": "14", "name": "广告缓存"},
        {"id": "15", "name": "举报原因"},
        {"id": "16", "name": "积分优惠"},
        {"id": "18", "name": "自定义WAP导航"},
        {"id": "19", "name": "网站地图"},
        {"id": "20", "name": "问答分类"},
        {"id": "23", "name": "自我介绍"},
        {"id": "24", "name": "关键字"},
        {"id": "25", "name": "单页面分类"},
        {"id": "26", "name": "数据库"},
        {"id": "27", "name": "邮件服务器"},
        {"id": "29", "name": "计划任务"}
    ])
}

async fn gen_cache_run(
    state: &AppState,
    user: &AuthenticatedUser,
    _body: &Value,
) -> AppResult<PhpOut> {
    admin_dashboard_service::clear_site_caches(state, user).await?;
    Ok(PhpOut::Message("admin_system_00064"))
}

fn json_ms_pair(body: &Value) -> Option<(i64, i64)> {
    match body.get("time") {
        Some(Value::Array(a)) if a.len() >= 2 => {
            let n = |v: &Value| {
                v.as_i64()
                    .or_else(|| v.as_f64().map(|f| f as i64))
                    .or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))
                    .unwrap_or(0)
            };
            let a0 = n(&a[0]);
            let a1 = n(&a[1]);
            if a0 > 0 && a1 > 0 {
                Some((a0, a1))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn board_win(body: &Value) -> gap_tongji::TjWindow {
    gap_tongji::tj_window(json_i32(body, "days"), json_ms_pair(body), clock::now_ts())
}

fn points_json(s: &gap_tongji::TjSeries) -> Value {
    Value::Array(
        s.list
            .iter()
            .map(|p| json!({"tjtime": p.tjtime, "date": p.date, "count": p.count}))
            .collect(),
    )
}

fn named_list(name_key: &str, s: &gap_tongji::TjSeries) -> Value {
    json!({"name": msg_t(name_key), "list": points_json(s)})
}

async fn get_tj(
    state: &AppState,
    table: &str,
    field: &str,
    win: &gap_tongji::TjWindow,
    extra: &[gap_tongji::Extra],
    sum_price: bool,
) -> AppResult<gap_tongji::TjSeries> {
    Ok(gap_tongji::get_tj(state.db.reader(), table, field, win, extra, sum_price).await?)
}

fn source_name(id: i32) -> String {
    match id {
        1 => "网页".into(),
        2 => "手机".into(),
        4 => "微信".into(),
        6 => "采集".into(),
        8 => "QQ登录".into(),
        9 => "微信扫一扫".into(),
        10 => "微博".into(),
        11 => "PC快速投递".into(),
        12 => "WAP快速投递".into(),
        21 => "账户分离".into(),
        26 => "预留信息".into(),
        _ => String::new(),
    }
}

fn sex_name(id: i32) -> String {
    match id {
        1 => "男".into(),
        2 => "女".into(),
        _ => String::new(),
    }
}

fn salary_bucket(min: i32, max: i32) -> (String, String) {
    let v = if max > 0 { max } else { min };
    if v <= 2000 {
        ("common_06589".into(), msg_t("common_06589"))
    } else if v <= 4000 {
        ("2000-4000".into(), "2000-4000".into())
    } else if v <= 6000 {
        ("4000-6000".into(), "4000-6000".into())
    } else if v <= 8000 {
        ("6000-8000".into(), "6000-8000".into())
    } else if v <= 10000 {
        ("8000-10000".into(), "8000-10000".into())
    } else {
        ("common_06590".into(), msg_t("common_06590"))
    }
}

fn bump(map: &mut serde_json::Map<String, Value>, group: &str, id: &str, name: &str) {
    if id.is_empty() {
        return;
    }
    let slot = map
        .entry(group.to_string())
        .or_insert_with(|| json!({}));
    if let Some(obj) = slot.as_object_mut() {
        let cur = obj
            .get(id)
            .and_then(|v| v.get("count"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0)
            + 1.0;
        obj.insert(id.to_string(), json!({"name": name, "count": cur}));
    }
}

fn pie_muti(v: &Value) -> Value {
    let mut items: Vec<Value> = match v {
        Value::Object(m) => m.values().cloned().collect(),
        Value::Array(a) => a.clone(),
        _ => return json!({}),
    };
    if items.is_empty() {
        return json!({});
    }
    items.sort_by(|a, b| {
        let ca = a.get("count").and_then(Value::as_f64).unwrap_or(0.0);
        let cb = b.get("count").and_then(Value::as_f64).unwrap_or(0.0);
        cb.partial_cmp(&ca).unwrap_or(std::cmp::Ordering::Equal)
    });
    if items.len() > 10 {
        let rest: f64 = items[10..]
            .iter()
            .map(|x| x.get("count").and_then(Value::as_f64).unwrap_or(0.0))
            .sum();
        items.truncate(10);
        items.push(json!({"name": msg_t("member_com_00038"), "count": rest}));
    }
    Value::Array(items)
}

fn empty_job_tj() -> Value {
    json!({
        "job1": {},
        "provinceid": {},
        "salary": {},
        "edu": {},
        "exp": {},
    })
}

fn csv_i32(s: &str) -> Vec<i32> {
    s.split(',')
        .filter_map(|x| x.trim().parse().ok())
        .filter(|n: &i32| *n > 0)
        .collect()
}

fn csv_names(s: &str, lookup: impl Fn(i32) -> String) -> String {
    csv_i32(s)
        .into_iter()
        .map(&lookup)
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>()
        .join(",")
}

async fn data_tj_job(state: &AppState, ids: &[u64]) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let rows = gap_tongji::job_slices(state.db.reader(), ids).await?;
    let mut map = serde_json::Map::new();
    for r in rows {
        bump(&mut map, "edu", &r.edu.to_string(), dicts.comclass(r.edu));
        bump(&mut map, "exp", &r.exp.to_string(), dicts.comclass(r.exp));
        bump(&mut map, "job1", &r.job1.to_string(), dicts.job(r.job1));
        bump(
            &mut map,
            "provinceid",
            &r.provinceid.to_string(),
            dicts.city(r.provinceid),
        );
        let (k, n) = salary_bucket(r.minsalary, r.maxsalary);
        bump(&mut map, "salary", &k, &n);
    }
    if !map.contains_key("job1") {
        return Ok(empty_job_tj());
    }
    for k in ["job1", "provinceid", "salary", "edu", "exp"] {
        map.entry(k.to_string()).or_insert_with(|| json!({}));
    }
    Ok(Value::Object(map))
}

async fn data_tj_expect(state: &AppState, ids: &[u64]) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let parents = gap_tongji::job_parents(state.db.reader()).await?;
    let rows = gap_tongji::expect_slices(state.db.reader(), ids).await?;
    let cities = gap_tongji::expect_cities(state.db.reader(), ids).await?;
    let mut city_map = HashMap::new();
    for c in cities {
        city_map.insert(c.eid, c.provinceid);
    }
    let mut map = serde_json::Map::new();
    for r in rows {
        bump(&mut map, "sex", &r.sex.to_string(), &sex_name(r.sex));
        bump(&mut map, "source", &r.source.to_string(), &source_name(r.source));
        bump(&mut map, "edu", &r.edu.to_string(), dicts.user_or_com(r.edu));
        bump(&mut map, "exp", &r.exp.to_string(), dicts.user_or_com(r.exp));
        for cid in csv_i32(&r.job_classid) {
            let root = parents.get(&cid).copied().unwrap_or(cid);
            if root > 0 {
                bump(&mut map, "job1", &root.to_string(), dicts.job(root));
            }
        }
        if let Some(pid) = city_map.get(&r.id) {
            bump(&mut map, "provinceid", &pid.to_string(), dicts.city(*pid));
        }
        let (k, n) = salary_bucket(r.minsalary, r.maxsalary);
        bump(&mut map, "salary", &k, &n);
    }
    for k in ["job1", "provinceid", "salary", "edu", "exp"] {
        map.entry(k.to_string()).or_insert_with(|| json!({}));
    }
    Ok(Value::Object(map))
}

async fn data_tj_reg(state: &AppState, ids: &[u64]) -> AppResult<Value> {
    let rows = gap_tongji::source_counts(state.db.reader(), ids).await?;
    let mut source = serde_json::Map::new();
    for r in rows {
        source.insert(
            r.source.to_string(),
            json!({"name": source_name(r.source), "count": r.count}),
        );
    }
    Ok(json!({"source": Value::Object(source)}))
}

async fn data_tj_order(state: &AppState, ids: &[u64]) -> AppResult<Value> {
    let rows = gap_tongji::order_slices(state.db.reader(), ids).await?;
    let mut map = serde_json::Map::new();
    for r in rows {
        let ot = if r.order_type.is_empty() {
            "0".to_string()
        } else {
            r.order_type.clone()
        };
        let on = pay_name(&ot);
        let on = if on.is_empty() {
            msg_t("member_com_00038")
        } else {
            on.to_string()
        };
        bump(&mut map, "ordertype", &ot, &on);
        let tn = order_kind_name(r.kind);
        let tn = if tn.is_empty() {
            msg_t("member_com_00038")
        } else {
            tn.to_string()
        };
        bump(&mut map, "type", &r.kind.to_string(), &tn);
    }
    map.entry("type".to_string()).or_insert_with(|| json!({}));
    map.entry("ordertype".to_string()).or_insert_with(|| json!({}));
    Ok(Value::Object(map))
}

async fn data_tj_company(state: &AppState, ids: &[u64]) -> AppResult<Value> {
    let dicts = dict_service::get(state).await?;
    let coms = gap_tongji::com_slices(state.db.reader(), ids).await?;
    let ratings = gap_tongji::com_ratings(state.db.reader(), ids).await?;
    let mut map = serde_json::Map::new();
    for r in ratings {
        let name = if r.rating_name.is_empty() {
            r.rating.to_string()
        } else {
            r.rating_name
        };
        bump(&mut map, "rating", &r.rating.to_string(), &name);
    }
    for r in coms {
        if r.hy <= 0 {
            bump(&mut map, "hy", "0", &msg_t("member_com_00038"));
            bump(&mut map, "is", "0", &msg_t("common_01680"));
        } else {
            bump(&mut map, "hy", &r.hy.to_string(), dicts.industry(r.hy));
            bump(&mut map, "is", "1", &msg_t("admin_tool_00124"));
        }
    }
    for k in ["hy", "rating", "is"] {
        map.entry(k.to_string()).or_insert_with(|| json!({}));
    }
    Ok(Value::Object(map))
}

async fn top_list(
    state: &AppState,
    table: &str,
    time_field: &str,
    group_field: &str,
    win: &gap_tongji::TjWindow,
    extra: &[gap_tongji::Extra],
    kind: &str,
    sum_price: bool,
) -> AppResult<Value> {
    let pairs =
        gap_tongji::group_top(state.db.reader(), table, time_field, group_field, win, extra, 10, sum_price)
            .await?;
    let ids: Vec<u64> = pairs.iter().map(|p| p.id).collect();
    let count_of = |id: u64| pairs.iter().find(|p| p.id == id).map(|p| p.count).unwrap_or(0.0);
    let dicts = dict_service::get(state).await?;
    let list: Vec<Value> = match kind {
        "job" => {
            let rows = gap_tongji::jobs_by_ids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.id, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({"id": r.id, "uid": r.uid, "name": r.name, "count": count_of(*id)}))
                })
                .collect()
        }
        "company" => {
            let rows = gap_tongji::companies_by_uids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.uid, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({"uid": r.uid, "name": r.name, "count": count_of(*id)}))
                })
                .collect()
        }
        "expect" => {
            let rows = gap_tongji::expects_by_ids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.id, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({
                        "id": r.id,
                        "uid": r.uid,
                        "uname": r.uname,
                        "jobclassname": csv_names(&r.job_classid, |i| dicts.job(i).to_string()),
                        "cityclassname": csv_names(&r.city_classid, |i| dicts.city(i).to_string()),
                        "eduname": dicts.user_or_com(r.edu),
                        "expname": dicts.user_or_com(r.exp),
                        "count": count_of(*id),
                    }))
                })
                .collect()
        }
        "resume" => {
            let rows = gap_tongji::resumes_by_uids(state.db.reader(), &ids).await?;
            let eids = gap_tongji::default_eids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.uid, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({
                        "uid": r.uid,
                        "name": r.name,
                        "count": count_of(*id),
                        "eid": eids.get(id).copied().unwrap_or(0),
                    }))
                })
                .collect()
        }
        "order" => {
            let rows = gap_tongji::members_by_uids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.uid, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({
                        "uid": r.uid,
                        "username": r.username,
                        "usertype": r.usertype,
                        "count": count_of(*id),
                    }))
                })
                .collect()
        }
        "ad" => {
            let rows = gap_tongji::ads_by_ids(state.db.reader(), &ids).await?;
            let mut by = HashMap::new();
            for r in rows {
                by.insert(r.id, r);
            }
            ids.iter()
                .filter_map(|id| {
                    let r = by.get(id)?;
                    Some(json!({"aid": r.id, "name": r.ad_name, "count": count_of(*id)}))
                })
                .collect()
        }
        _ => Vec::new(),
    };
    Ok(Value::Array(list))
}

fn merge_series(a: &gap_tongji::TjSeries, b: &gap_tongji::TjSeries) -> gap_tongji::TjSeries {
    let mut map: HashMap<String, f64> = HashMap::new();
    for p in &a.list {
        map.insert(p.tjtime.clone(), p.count);
    }
    for p in &b.list {
        *map.entry(p.tjtime.clone()).or_insert(0.0) += p.count;
    }
    gap_tongji::TjSeries {
        allnum: a.allnum + b.allnum,
        list: a
            .list
            .iter()
            .map(|p| gap_tongji::TjPoint {
                tjtime: p.tjtime.clone(),
                date: p.date.clone(),
                count: map.get(&p.tjtime).copied().unwrap_or(0.0),
            })
            .collect(),
    }
}

fn merge_top(a: Value, b: Value) -> Value {
    let mut map: HashMap<String, Value> = HashMap::new();
    let take = |v: Value, map: &mut HashMap<String, Value>| {
        if let Value::Array(arr) = v {
            for item in arr {
                let uid = item
                    .get("uid")
                    .and_then(Value::as_u64)
                    .or_else(|| item.get("id").and_then(Value::as_u64))
                    .unwrap_or(0);
                if uid == 0 {
                    continue;
                }
                let key = uid.to_string();
                if let Some(old) = map.get(&key) {
                    let c = old.get("count").and_then(Value::as_f64).unwrap_or(0.0)
                        + item.get("count").and_then(Value::as_f64).unwrap_or(0.0);
                    let mut n = old.clone();
                    if let Some(obj) = n.as_object_mut() {
                        obj.insert("count".into(), json!(c));
                    }
                    map.insert(key, n);
                } else {
                    map.insert(key, item);
                }
            }
        }
    };
    take(a, &mut map);
    take(b, &mut map);
    let mut items: Vec<Value> = map.into_values().collect();
    items.sort_by(|x, y| {
        let cx = x.get("count").and_then(Value::as_f64).unwrap_or(0.0);
        let cy = y.get("count").and_then(Value::as_f64).unwrap_or(0.0);
        cb_cmp(cy, cx)
    });
    items.truncate(10);
    Value::Array(items)
}

fn cb_cmp(a: f64, b: f64) -> std::cmp::Ordering {
    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
}

async fn data_board_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let win = board_win(body);
    let pool = state.db.reader();
    let specs: [(&str, &str, &str, &str, Option<i32>); 9] = [
        ("adduser", "member", "reg_date", "admin_user_00305", Some(1)),
        ("addexpect", "resume_expect", "ctime", "admin_user_00193", None),
        ("resumeDelivery", "userid_job", "datetime", "member_com_00152", None),
        ("resumeRefresh", "resume_refresh_log", "r_time", "admin_tool_00176", None),
        ("addcom", "member", "reg_date", "admin_user_company_00162", Some(2)),
        ("addjob", "company_job", "sdate", "member_com_00250", None),
        ("downResume", "down_resume", "downtime", "wap_com_00042", None),
        ("jobRefresh", "job_refresh_log", "r_time", "wap_com_00045", None),
        ("inviteInterview", "userid_msg", "datetime", "resume_00029", None),
    ];
    let mut all_num = serde_json::Map::new();
    let mut list = serde_json::Map::new();
    for (key, table, field, name, usertype) in specs {
        let extra = match usertype {
            Some(u) => vec![gap_tongji::Extra::Eq("usertype", u)],
            None => Vec::new(),
        };
        let s = gap_tongji::get_tj(pool, table, field, &win, &extra, false).await?;
        all_num.insert(key.to_string(), json!(s.allnum));
        list.insert(key.to_string(), named_list(name, &s));
    }
    Ok(json!({"allNum": Value::Object(all_num), "list": Value::Object(list)}))
}

async fn data_board_class(state: &AppState, body: &Value) -> AppResult<Value> {
    let win = board_win(body);
    let t = json_i32(body, "type");
    let t = if t == 0 { 1 } else { t };
    let pool = state.db.reader();
    let mut all_num = serde_json::Map::new();
    let mut list = serde_json::Map::new();
    let mut top = serde_json::Map::new();
    let count_tj: Value;
    match t {
        2 => {
            let job = get_tj(state, "look_job", "datetime", &win, &[], false).await?;
            all_num.insert("job".into(), json!(job.allnum));
            list.insert("job".into(), named_list("admin_01455", &job));
            top.insert(
                "jobList".into(),
                top_list(state, "look_job", "datetime", "jobid", &win, &[], "job", false).await?,
            );
            top.insert(
                "jobComList".into(),
                top_list(state, "look_job", "datetime", "com_id", &win, &[], "company", false).await?,
            );
            let resume = get_tj(state, "look_resume", "datetime", &win, &[], false).await?;
            all_num.insert("resume".into(), json!(resume.allnum));
            list.insert("resume".into(), named_list("weixin_00010", &resume));
            top.insert(
                "resumeList".into(),
                top_list(state, "look_resume", "datetime", "resume_id", &win, &[], "expect", false).await?,
            );
            top.insert(
                "resumeComList".into(),
                top_list(state, "look_resume", "datetime", "com_id", &win, &[], "company", false).await?,
            );
            count_tj = json!({});
        }
        3 => {
            let invite = get_tj(state, "userid_msg", "datetime", &win, &[], false).await?;
            all_num.insert("invite".into(), json!(invite.allnum));
            list.insert("invite".into(), named_list("resume_00029", &invite));
            top.insert(
                "inviteCom".into(),
                top_list(state, "userid_msg", "datetime", "fid", &win, &[], "company", false).await?,
            );
            top.insert(
                "inviteResume".into(),
                top_list(state, "userid_msg", "datetime", "uid", &win, &[], "resume", false).await?,
            );
            let invite_ids =
                gap_tongji::list_ids(pool, "userid_msg", "jobid", "datetime", &win, &[]).await?;
            let mut invite_tj = data_tj_job(state, &invite_ids).await?;
            if let Some(obj) = invite_tj.as_object_mut() {
                if let Some(j) = obj.get("job1").cloned() {
                    obj.insert("job1".into(), pie_muti(&j));
                }
            }
            let down = get_tj(state, "down_resume", "downtime", &win, &[], false).await?;
            let free = get_tj(state, "freedown_resume", "downtime", &win, &[], false).await?;
            let merged = merge_series(&down, &free);
            all_num.insert("down".into(), json!(merged.allnum));
            list.insert("down".into(), named_list("wap_com_00042", &merged));
            let down_com = top_list(state, "down_resume", "downtime", "comid", &win, &[], "company", false).await?;
            let free_com =
                top_list(state, "freedown_resume", "downtime", "comid", &win, &[], "company", false).await?;
            top.insert("downCom".into(), merge_top(down_com, free_com));
            let down_re = top_list(state, "down_resume", "downtime", "uid", &win, &[], "resume", false).await?;
            let free_re =
                top_list(state, "freedown_resume", "downtime", "uid", &win, &[], "resume", false).await?;
            top.insert("downResume".into(), merge_top(down_re, free_re));
            let down_ids =
                gap_tongji::list_ids(pool, "down_resume", "eid", "downtime", &win, &[]).await?;
            let mut down_tj = data_tj_expect(state, &down_ids).await?;
            if let Some(obj) = down_tj.as_object_mut() {
                if let Some(j) = obj.get("job1").cloned() {
                    obj.insert("job1".into(), pie_muti(&j));
                }
            }
            count_tj = json!([invite_tj, down_tj]);
        }
        4 => {
            let extra = vec![gap_tongji::Extra::Eq("order_state", 2)];
            let order = get_tj(state, "company_order", "order_time", &win, &extra, true).await?;
            let all = (order.allnum * 100.0).round() / 100.0;
            all_num.insert("order".into(), json!(all));
            list.insert("order".into(), named_list("wap_user_00312", &order));
            top.insert(
                "orderCom".into(),
                top_list(
                    state,
                    "company_order",
                    "order_time",
                    "uid",
                    &win,
                    &extra,
                    "order",
                    true,
                )
                .await?,
            );
            let ids = gap_tongji::list_ids(pool, "company_order", "id", "order_time", &win, &extra).await?;
            let tj = data_tj_order(state, &ids).await?;
            let ad = get_tj(state, "adclick", "addtime", &win, &[], false).await?;
            all_num.insert("ad".into(), json!(ad.allnum));
            list.insert("ad".into(), named_list("admin_yunying_00049", &ad));
            top.insert(
                "adClick".into(),
                top_list(state, "adclick", "addtime", "aid", &win, &[], "ad", false).await?,
            );
            count_tj = tj;
        }
        5 => {
            let add_job = get_tj(state, "company_job", "sdate", &win, &[], false).await?;
            all_num.insert("addJob".into(), json!(add_job.allnum));
            list.insert("addJob".into(), named_list("wap_00322", &add_job));
            let up_job = get_tj(state, "job_refresh_log", "r_time", &win, &[], false).await?;
            all_num.insert("upJob".into(), json!(up_job.allnum));
            list.insert("upJob".into(), named_list("wap_com_00029", &up_job));
            top.insert(
                "addJobCom".into(),
                top_list(state, "company_job", "sdate", "uid", &win, &[], "company", false).await?,
            );
            let job_ids = gap_tongji::list_ids(pool, "company_job", "id", "sdate", &win, &[]).await?;
            let mut job_tj = data_tj_job(state, &job_ids).await?;
            if let Some(obj) = job_tj.as_object_mut() {
                if let Some(j) = obj.get("job1").cloned() {
                    obj.insert("job1".into(), pie_muti(&j));
                }
            }
            let add_re = get_tj(state, "resume_expect", "ctime", &win, &[], false).await?;
            all_num.insert("addResume".into(), json!(add_re.allnum));
            list.insert("addResume".into(), named_list("admin_tool_00016", &add_re));
            let up_re = get_tj(state, "resume_refresh_log", "r_time", &win, &[], false).await?;
            all_num.insert("upResume".into(), json!(up_re.allnum));
            list.insert("upResume".into(), named_list("admin_tool_00176", &up_re));
            let re_ids = gap_tongji::list_ids(pool, "resume_expect", "id", "ctime", &win, &[]).await?;
            let mut re_tj = data_tj_expect(state, &re_ids).await?;
            if let Some(obj) = re_tj.as_object_mut() {
                if let Some(j) = obj.get("job1").cloned() {
                    obj.insert("job1".into(), pie_muti(&j));
                }
            }
            count_tj = json!([job_tj, re_tj]);
        }
        6 => {
            let extra2 = vec![gap_tongji::Extra::Eq("usertype", 2)];
            let com = get_tj(state, "member", "reg_date", &win, &extra2, false).await?;
            all_num.insert("comOne".into(), json!(com.allnum));
            list.insert("comOne".into(), named_list("admin_tool_00123", &com));
            let extra_st = vec![
                gap_tongji::Extra::Eq("usertype", 2),
                gap_tongji::Extra::Eq("status", 0),
            ];
            let com2 = get_tj(state, "member", "reg_date", &win, &extra_st, false).await?;
            all_num.insert("comTwo".into(), json!(com2.allnum));
            list.insert("comTwo".into(), named_list("admin_00316", &com2));
            let uids = gap_tongji::list_ids(pool, "member", "uid", "reg_date", &win, &extra2).await?;
            let mut com_tj = data_tj_company(state, &uids).await?;
            if let Some(obj) = com_tj.as_object_mut() {
                if let Some(j) = obj.get("hy").cloned() {
                    obj.insert("hy".into(), pie_muti(&j));
                }
            }
            let apply = get_tj(state, "userid_job", "datetime", &win, &[], false).await?;
            all_num.insert("apply".into(), json!(apply.allnum));
            list.insert("apply".into(), named_list("member_com_00152", &apply));
            top.insert(
                "applyCom".into(),
                top_list(state, "userid_job", "datetime", "com_id", &win, &[], "company", false).await?,
            );
            top.insert(
                "applyResume".into(),
                top_list(state, "userid_job", "datetime", "eid", &win, &[], "expect", false).await?,
            );
            let eids = gap_tongji::list_ids(pool, "userid_job", "eid", "datetime", &win, &[]).await?;
            let mut re_tj = data_tj_expect(state, &eids).await?;
            if let Some(obj) = re_tj.as_object_mut() {
                if let Some(j) = obj.get("job1").cloned() {
                    obj.insert("job1".into(), pie_muti(&j));
                }
            }
            count_tj = json!([com_tj, re_tj]);
        }
        _ => {
            let all = get_tj(state, "member", "reg_date", &win, &[], false).await?;
            all_num.insert("allReg".into(), json!(all.allnum));
            list.insert("allReg".into(), named_list("admin_tool_00015", &all));
            let extra2 = vec![gap_tongji::Extra::Eq("usertype", 2)];
            let com = get_tj(state, "member", "reg_date", &win, &extra2, false).await?;
            all_num.insert("comReg".into(), json!(com.allnum));
            list.insert("comReg".into(), named_list("admin_user_company_00281", &com));
            let extra1 = vec![gap_tongji::Extra::Eq("usertype", 1)];
            let user = get_tj(state, "member", "reg_date", &win, &extra1, false).await?;
            all_num.insert("userReg".into(), json!(user.allnum));
            list.insert("userReg".into(), named_list("admin_system_00129", &user));
            let uids = gap_tongji::list_ids(pool, "member", "uid", "reg_date", &win, &[]).await?;
            let tj = data_tj_reg(state, &uids).await?;
            let login_in = vec![gap_tongji::Extra::In("usertype", vec![1, 2])];
            let login = get_tj(state, "login_log", "ctime", &win, &login_in, false).await?;
            all_num.insert("allLogin".into(), json!(login.allnum));
            list.insert("allLogin".into(), named_list("admin_tool_00015", &login));
            let com_l = get_tj(state, "login_log", "ctime", &win, &extra2, false).await?;
            all_num.insert("comLogin".into(), json!(com_l.allnum));
            list.insert("comLogin".into(), named_list("admin_user_company_00281", &com_l));
            let user_l = get_tj(state, "login_log", "ctime", &win, &extra1, false).await?;
            all_num.insert("userLogin".into(), json!(user_l.allnum));
            list.insert("userLogin".into(), named_list("admin_system_00129", &user_l));
            count_tj = tj;
        }
    }
    Ok(json!({
        "AllNum": Value::Object(all_num),
        "List": Value::Object(list),
        "topList": Value::Object(top),
        "CountTj": count_tj,
    }))
}

async fn data_board_fenxiabiao(state: &AppState, body: &Value) -> AppResult<Value> {
    let start = json_str(body, "startTime");
    let end = json_str(body, "endTime");
    if start.is_empty() || end.is_empty() {
        return Err(ApiError::business("admin_tool_00014"));
    }
    let fx = json_i32(body, "type");
    let fx = if fx == 0 { 1 } else { fx };
    let (s1, e1, month) = gap_tongji::fenxiabiao_range(fx, &start)
        .ok_or_else(|| ApiError::business("admin_tool_00014"))?;
    let (s2, e2, _) = gap_tongji::fenxiabiao_range(fx, &end)
        .ok_or_else(|| ApiError::business("admin_tool_00014"))?;
    let first = gap_tongji::fenxiabiao_month(state.db.reader(), s1, e1).await?;
    let second = gap_tongji::fenxiabiao_month(state.db.reader(), s2, e2).await?;
    let keys = [
        ("member", "gerezce"),
        ("login_log", "login_log"),
        ("resume_expect", "jilizce"),
        ("company", "comzce"),
        ("company_login_log", "company_login_log"),
        ("company_job", "fabuzhw"),
        ("userid_job", "jilitod"),
        ("chat_log", "liaotan"),
        ("userid_msg", "yaoqms"),
        ("down_resume", "jilixza"),
    ];
    let steps = if month > 1 { month + 1 } else { month };
    let mut first_rows = Vec::new();
    let mut second_rows = Vec::new();
    for i in 0..steps {
        let last = i == steps - 1;
        let on1 = if last {
            "sum".to_string()
        } else {
            gap_tongji::ym_of(gap_tongji::add_months(s1, i))
        };
        let on2 = if last {
            "sum".to_string()
        } else {
            gap_tongji::ym_of(gap_tongji::add_months(s2, i))
        };
        let years = if last {
            msg_t("member_com_00348")
        } else {
            on1.clone()
        };
        let years2 = if last {
            msg_t("member_com_00348")
        } else {
            on2.clone()
        };
        let mut r1 = serde_json::Map::new();
        let mut r2 = serde_json::Map::new();
        r1.insert("years".into(), json!(years));
        r2.insert("years".into(), json!(years2));
        for (src, dst) in keys {
            let mut n1 = first.get(src).and_then(|m| m.get(&on1)).copied().unwrap_or(0);
            let mut n2 = second.get(src).and_then(|m| m.get(&on2)).copied().unwrap_or(0);
            if src == "down_resume" {
                n1 += first.get("freedown_resume").and_then(|m| m.get(&on1)).copied().unwrap_or(0);
                n2 += second.get("freedown_resume").and_then(|m| m.get(&on2)).copied().unwrap_or(0);
            }
            r1.insert(dst.to_string(), json!(n1));
            r2.insert(dst.to_string(), json!(n2));
            let pct = if n1 == 0 {
                0.0
            } else {
                ((n2 as f64 - n1 as f64) / n1 as f64 * 100.0 * 100.0).round() / 100.0
            };
            r2.insert(format!("{dst}_percent"), json!(pct));
        }
        first_rows.push(Value::Object(r1));
        second_rows.push(Value::Object(r2));
    }
    Ok(json!({"firstResult": first_rows, "secondResult": second_rows}))
}

async fn data_board_get_auth(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    let navi = json_i64(body, "navi_id");
    if navi == 0 {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let row = rbac_php::php_get_user(state.db.reader(), user.uid).await?;
    let status = if let Some(u) = row {
        let powers = phpyun_models::admin_rbac::repo::group_power_ids(state.db.reader(), u.m_id).await?;
        powers.iter().any(|p| *p == navi)
    } else {
        false
    };
    Ok(json!({"status": status}))
}

fn php_date(ts: i64, fmt: &str) -> String {
    if ts <= 0 {
        return String::new();
    }
    let Some(dt) = clock::tz().timestamp_opt(ts, 0).single() else {
        return String::new();
    };
    let pat = if fmt.trim().is_empty() { "Y-m-d" } else { fmt };
    let mut out = String::new();
    for c in pat.chars() {
        match c {
            'Y' => out.push_str(&dt.format("%Y").to_string()),
            'm' => out.push_str(&dt.format("%m").to_string()),
            'd' => out.push_str(&dt.format("%d").to_string()),
            'H' => out.push_str(&dt.format("%H").to_string()),
            'i' => out.push_str(&dt.format("%M").to_string()),
            's' => out.push_str(&dt.format("%S").to_string()),
            other => out.push(other),
        }
    }
    out
}

fn site_url(web: &str, path: &str) -> String {
    format!(
        "{}/{}",
        web.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

fn render_loop(code: &str, items: &[HashMap<String, String>], urltype: i32) -> String {
    let code = code
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");
    let lower = code.to_ascii_lowercase();
    let Some(a) = lower.find("<loop>") else {
        return String::new();
    };
    let Some(rel) = lower[a + 6..].find("</loop>") else {
        return String::new();
    };
    let body_start = a + 6;
    let body_end = body_start + rel;
    let prefix = &code[..a];
    let tpl = &code[body_start..body_end];
    let suffix = &code[body_end + 7..];
    let target = if urltype == 1 {
        " target=\"_blank\""
    } else {
        ""
    };
    let mut mid = String::new();
    for item in items {
        let mut row = tpl.to_string();
        for (k, v) in item {
            row = row.replace(&format!("{{{k}}}"), v);
        }
        row = row.replace("{target}", target);
        mid.push_str(&row);
    }
    let mut out = format!("{prefix}{mid}{suffix}");
    out = out.replace("<!--循环开始-->", "");
    out = out.replace("<!--循环结束-->", "");
    out = out.replace('\n', "").replace('\r', "");
    out
}

async fn data_call_preview(state: &AppState, body: &Value) -> AppResult<Value> {
    let id = json_u64(body, "id");
    if id == 0 {
        return Ok(json!({"list": ""}));
    }
    let Some(row) = gap_datacall::find_outside(state.db.reader(), id).await? else {
        return Ok(json!({"list": ""}));
    };
    let dicts = dict_service::get(state).await?;
    let cfg = settings_hash(state).await.unwrap_or_default();
    let web = cfg.get("sy_weburl").cloned().unwrap_or_default();
    let n = if row.num > 0 { row.num } else { 10 };
    let title = if row.titlelen > 0 { row.titlelen as usize } else { 0 };
    let info = if row.infolen > 0 { row.infolen as usize } else { 0 };
    let tf = row.timetype.clone();
    let mut items: Vec<HashMap<String, String>> = Vec::new();
    match row.r#type.as_str() {
        "resume" => {
            let rows = gap_datacall::list_resume(state.db.reader(), &row.byorder, n).await?;
            let icon = cfg.get("sy_member_icon").cloned().unwrap_or_default();
            for r in rows {
                let mut m = HashMap::new();
                m.insert("resumename".into(), trunc_chars(&r.resumename, title));
                m.insert("name".into(), trunc_chars(&r.name, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=resume&c=show&id={}", r.id)));
                let y = r.birthday.chars().take(4).collect::<String>().parse::<i32>().unwrap_or(0);
                let age = if y > 0 {
                    (i32::from(clock::now_year()) - y).max(0).to_string()
                } else {
                    String::new()
                };
                m.insert("birthday".into(), age);
                m.insert("edu".into(), dicts.user_or_com(r.edu).to_string());
                m.insert("lastedit".into(), php_date(r.lastupdate, &tf));
                m.insert("hits".into(), r.hits.to_string());
                let pic = if r.photo.is_empty() { icon.clone() } else { r.photo.clone() };
                m.insert("big_pic".into(), checkpic_url(&cfg, &pic));
                m.insert("small_pic".into(), checkpic_url(&cfg, &pic));
                m.insert("email".into(), r.email);
                m.insert("tel".into(), r.telhome);
                m.insert("moblie".into(), r.telphone);
                m.insert("hy".into(), dicts.industry(r.hy).to_string());
                m.insert("hyurl".into(), site_url(&web, &format!("index.php?m=company&hy={}", r.hy)));
                m.insert("job_classid".into(), csv_names(&r.job_classid, |i| dicts.job(i).to_string()));
                m.insert("report".into(), dicts.user_or_com(r.report).to_string());
                m.insert("salary".into(), dicts.user_or_com(r.salary).to_string());
                m.insert("type".into(), dicts.user_or_com(r.kind).to_string());
                m.insert(
                    "gz_city".into(),
                    format!("{}-{}", dicts.city(r.qw_provinceid), dicts.city(r.qw_cityid)),
                );
                m.insert("domicile".into(), r.domicile);
                m.insert("living".into(), r.living);
                m.insert("exp".into(), dicts.user_or_com(r.exp).to_string());
                m.insert("address".into(), r.address);
                m.insert("description".into(), trunc_chars(&r.description, info));
                m.insert("idcard".into(), r.idcard);
                m.insert("homepage".into(), r.homepage);
                items.push(m);
            }
        }
        "company" => {
            let rows = gap_datacall::list_company(state.db.reader(), &row.byorder, n).await?;
            let uids: Vec<u64> = rows.iter().map(|r| r.uid).collect();
            let jobs = gap_datacall::job_counts(state.db.reader(), &uids).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("uid".into(), r.uid.to_string());
                m.insert("companyname".into(), trunc_chars(&r.name, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=company&c=show&id={}", r.uid)));
                m.insert("hy".into(), dicts.industry(r.hy).to_string());
                m.insert("hy_url".into(), site_url(&web, &format!("index.php?m=company&hy={}", r.hy)));
                m.insert("pr".into(), dicts.comclass(r.pr).to_string());
                m.insert("city".into(), format!("{}-{}", dicts.city(r.provinceid), dicts.city(r.cityid)));
                m.insert("mun".into(), dicts.comclass(r.mun).to_string());
                m.insert("address".into(), r.address);
                m.insert("linkphone".into(), r.linkphone);
                m.insert("linkmail".into(), r.linkmail);
                m.insert("sdate".into(), r.sdate);
                m.insert("money".into(), r.money);
                m.insert("zip".into(), r.zip);
                m.insert("linkman".into(), r.linkman);
                m.insert("job_num".into(), jobs.get(&r.uid).copied().unwrap_or(0).to_string());
                m.insert("linkqq".into(), r.linkqq);
                m.insert("linktel".into(), r.linktel);
                m.insert("website".into(), r.website);
                m.insert("logo".into(), checkpic_url(&cfg, &r.logo));
                items.push(m);
            }
        }
        "job" => {
            let rows = gap_datacall::list_job(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("jobname".into(), trunc_chars(&r.name, title));
                m.insert("companyname".into(), trunc_chars(&r.com_name, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=job&c=comapply&id={}", r.id)));
                m.insert("com_url".into(), site_url(&web, &format!("index.php?m=company&c=show&id={}", r.uid)));
                m.insert("hy".into(), dicts.industry(r.hy).to_string());
                m.insert("hy_url".into(), site_url(&web, &format!("index.php?m=company&hy={}", r.hy)));
                m.insert("city".into(), format!("{}-{}", dicts.city(r.provinceid), dicts.city(r.cityid)));
                m.insert("num".into(), dicts.comclass(r.number).to_string());
                m.insert("jobtype".into(), format!("{}-{}", dicts.job(r.job1_son), dicts.job(r.job_post)));
                m.insert("edu".into(), dicts.comclass(r.edu).to_string());
                m.insert("age".into(), dicts.comclass(r.age).to_string());
                m.insert("report".into(), dicts.comclass(r.report).to_string());
                m.insert("exp".into(), dicts.comclass(r.exp).to_string());
                let salary = if r.minsalary > 0 && r.maxsalary > 0 {
                    format!("{}-{}", r.minsalary, r.maxsalary)
                } else if r.minsalary > 0 {
                    format!("{}{}", r.minsalary, msg_t("common_01942"))
                } else {
                    msg_t("common_02045")
                };
                m.insert("salary".into(), salary);
                m.insert(
                    "lang".into(),
                    csv_names(&r.lang, |i| dicts.comclass(i).to_string()),
                );
                m.insert("welfare".into(), r.welfare);
                m.insert("time".into(), php_date(r.lastupdate, &tf));
                items.push(m);
            }
        }
        "zph" => {
            let rows = gap_datacall::list_zph(state.db.reader(), &row.byorder, n).await?;
            let zids: Vec<u64> = rows.iter().map(|r| r.id).collect();
            let nums = gap_datacall::zph_com_counts(state.db.reader(), &zids).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("id".into(), r.id.to_string());
                m.insert("title".into(), trunc_chars(&r.title, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=zph&c=show&id={}", r.id)));
                m.insert("organizers".into(), r.organizers);
                let time = if r.start_at > 0 {
                    php_date(r.start_at, &tf)
                } else {
                    r.starttime.clone()
                };
                m.insert("time".into(), time);
                m.insert("address".into(), r.address);
                m.insert("phone".into(), r.phone);
                m.insert("linkman".into(), r.user);
                m.insert("website".into(), r.weburl);
                m.insert("logo".into(), checkpic_url(&cfg, &r.pic));
                m.insert("com_num".into(), nums.get(&r.id).copied().unwrap_or(0).to_string());
                items.push(m);
            }
        }
        "news" => {
            let rows = gap_datacall::list_news(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("title".into(), trunc_chars(&r.title, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=news&c=show&id={}", r.id)));
                m.insert("keyword".into(), r.keyword);
                m.insert("author".into(), r.author);
                m.insert("time".into(), php_date(r.datetime, &tf));
                m.insert("hits".into(), r.hits.to_string());
                m.insert("description".into(), trunc_chars(&r.description, info));
                m.insert("thumb".into(), checkpic_url(&cfg, &r.s_thumb));
                m.insert("source".into(), r.source);
                items.push(m);
            }
        }
        "ask" => {
            let rows = gap_datacall::list_ask(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("title".into(), trunc_chars(&r.title, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=ask&c=content&id={}", r.id)));
                m.insert("content".into(), r.content);
                m.insert("name".into(), r.nickname);
                m.insert("time".into(), php_date(r.add_time, &tf));
                m.insert("answer_num".into(), r.answer_num.to_string());
                items.push(m);
            }
        }
        "link" => {
            let rows = gap_datacall::list_link(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("link_name".into(), trunc_chars(&r.link_name, title));
                m.insert("link_url".into(), r.link_url);
                m.insert("link_src".into(), checkpic_url(&cfg, &r.pic));
                items.push(m);
            }
        }
        "once" => {
            let rows = gap_datacall::list_once(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("jobname".into(), trunc_chars(&r.title, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=once&c=show&id={}", r.id)));
                m.insert("companyname".into(), trunc_chars(&r.companyname, title));
                m.insert("mans".into(), r.mans);
                m.insert("require".into(), r.require);
                m.insert("phone".into(), r.phone);
                m.insert("linkman".into(), r.linkman);
                m.insert("address".into(), r.address);
                m.insert("time".into(), php_date(r.ctime, &tf));
                items.push(m);
            }
        }
        "tiny" => {
            let rows = gap_datacall::list_tiny(state.db.reader(), &row.byorder, n).await?;
            for r in rows {
                let mut m = HashMap::new();
                m.insert("name".into(), trunc_chars(&r.username, title));
                m.insert("url".into(), site_url(&web, &format!("index.php?m=tiny&c=show&id={}", r.id)));
                m.insert("sex".into(), sex_name(r.sex));
                m.insert("exp".into(), dicts.user_or_com(r.exp).to_string());
                m.insert("job".into(), r.job);
                m.insert("mobile".into(), r.mobile);
                m.insert("describe".into(), trunc_chars(&r.production, info));
                m.insert("time".into(), php_date(r.time, &tf));
                items.push(m);
            }
        }
        _ => {}
    }
    let html = render_loop(&row.code, &items, row.urltype);
    Ok(json!({"list": html}))
}

async fn data_collection_rating(state: &AppState, ) -> AppResult<Value> {
    let rows = company_repo::list_rating_options(state.db.reader()).await?;
    let rating_arr: Vec<Value> = rows
        .into_iter()
        .map(|r| json!({"id": r.id, "name": r.name}))
        .collect();
    Ok(json!({"ratingArr": rating_arr}))
}

const LOCOY_KEYS: &[&str] = &[
    "locoy_online",
    "locoy_key",
    "locoy_rate",
    "locoy_keyword",
    "locoy_rand",
    "locoy_sort",
    "locoy_length",
    "locoy_name",
    "locoy_pwd",
    "locoy_user_status",
    "locoy_rating",
    "locoy_resume_status",
];

fn locoy_php_defaults() -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    for (k, v) in [
        ("locoy_online", "2"),
        ("locoy_key", ""),
        ("locoy_rate", ""),
        ("locoy_keyword", "2"),
        ("locoy_rand", ""),
        ("locoy_sort", ""),
        ("locoy_length", ""),
        ("locoy_name", ""),
        ("locoy_pwd", ""),
        ("locoy_user_status", "2"),
        ("locoy_rating", "1"),
        ("locoy_resume_status", "1"),
    ] {
        m.insert(k.into(), v.into());
    }
    if let Ok(text) =
        std::fs::read_to_string("/www/wwwroot/zzzz.com/uploads/data/api/locoy/locoy_config.php")
    {
        for part in text.split(',') {
            let Some((k, v)) = part.split_once("=>") else {
                continue;
            };
            let k = k.trim().trim_matches(|c| c == '"' || c == '\'' || c == '{' || c == '(');
            let k = k.trim_start_matches('$').trim();
            if !k.starts_with("locoy_") {
                continue;
            }
            let v = v
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == ')' || c == ';' || c == '}');
            m.insert(k.to_string(), v.to_string());
        }
    }
    m
}

async fn data_collection_index(state: &AppState) -> AppResult<Value> {
    let mut out = locoy_php_defaults();
    if let Ok(rows) = setting_repo::find_many(state.db.reader(), LOCOY_KEYS).await {
        for (k, v) in rows {
            if !v.is_empty() {
                out.insert(k, v);
            }
        }
    }
    let mut map = serde_json::Map::new();
    for k in LOCOY_KEYS {
        map.insert(
            (*k).into(),
            Value::String(out.get(*k).cloned().unwrap_or_default()),
        );
    }
    Ok(Value::Object(map))
}

async fn user_gap_del_info(
    state: &AppState,
    body: &Value,
    usertype: i32,
) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "del");
    ids.extend(ids_of(body));
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let del_account = json_str(body, "delAccount") == "1" || json_i32(body, "delAccount") == 1;
    let pool = state.db.pool();
    let n = if del_account {
        gap_extra::php_del_member_account(pool, &ids).await?
    } else if usertype == 2 {
        let n = gap_extra::php_del_com(pool, &ids).await?;
        if n > 0 {
            let _ = gap_extra::php_clear_member_usertype(pool, &ids).await?;
        }
        n
    } else {
        let n = gap_extra::php_del_user(pool, &ids).await?;
        if n > 0 {
            let _ = gap_extra::php_clear_member_usertype(pool, &ids).await?;
        }
        n
    };
    if n == 0 {
        return Err(ApiError::business("common_06641"));
    }
    if del_account {
        Ok(PhpOut::Message("common_06640"))
    } else {
        Ok(PhpOut::Message("common_01459"))
    }
}

async fn user_gap_company_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    user_gap_del_info(state, body, 2).await
}

async fn user_gap_user_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    user_gap_del_info(state, body, 1).await
}

async fn user_gap_company_status(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let mut ids = ids_named(body, "uid");
    ids.extend(ids_of(body));
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    let lock_info = json_str(body, "statusbody");
    let pool = state.db.pool();
    for uid in &ids {
        let n = company_repo::set_r_status(pool, *uid, status).await?;
        if n == 0 {
            return Err(ApiError::business("common_01071"));
        }
        let _ = user_repo::update_lock_info_only(pool, *uid, &lock_info).await?;
        user_repo::lock_related_r_status(pool, *uid, status).await?;
    }
    let single = json_i32(body, "single") == 1 || json_str(body, "single") == "1";
    let atype = json_i32(body, "atype");
    if single && atype != 1 {
        if let Some(next) = company_repo::next_r_status_uid(pool, 0, ids[0]).await? {
            return Ok(PhpOut::Data(json!({ "uid": next })));
        }
    }
    Ok(PhpOut::Message("common_01944"))
}

async fn company_job_hb_data(state: &AppState) -> AppResult<Value> {
    let rows = whb_repo::list_admin_by_type(state.db.reader(), 1).await?;
    let hb_num = rows.iter().filter(|r| r.isopen == 1).count();
    let hb_isopen = cfg_of(state, "sy_haibao_isopen").await;
    Ok(json!({
        "hbNum": hb_num,
        "hb_isopen": if hb_isopen.is_empty() { "0".into() } else { hb_isopen },
    }))
}

async fn weixinrecord_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let (page, per, offset, limit) = page_of(body);
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let mut status = json_opt_i32(body, "status");
    if status == Some(2) {
        status = Some(0);
    }
    let since = match json_i32(body, "time") {
        0 => None,
        1 => Some(clock::start_of_today()),
        n if n > 0 => Some(clock::now_ts() - i64::from(n) * 86_400),
        _ => None,
    };
    let db = state.db.reader();
    let total = gap_extra::php_count_wxqrcodes_admin(db, status, keyword, since).await?;
    let rows = if total > 0 {
        gap_extra::php_list_wxqrcodes_admin(db, status, keyword, since, offset, limit).await?
    } else {
        Vec::new()
    };
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "wxloginid": r.wxloginid,
                "ticket": r.ticket,
                "time": r.time,
                "status": r.status,
                "wxid": r.wxid,
                "uid": r.uid,
                "username": r.username,
                "usertype": r.usertype,
                "time_n": if r.time > 0 { fmt_dt(r.time) } else { String::new() },
            })
        })
        .collect();
    Ok(paged(Value::Array(list), total, page, per))
}

#[derive(serde::Deserialize)]
struct Ov6Resp {
    #[serde(default)]
    code: i32,
    #[serde(default)]
    data: Option<Ov6Data>,
}

#[derive(serde::Deserialize, Default)]
struct Ov6Data {
    #[serde(default)]
    addr: Option<String>,
    #[serde(default)]
    location: Option<String>,
}

async fn index_get_ip_address(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let ip = json_str(body, "ip");
    if uid == 0 || ip.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let on = cfg_of(state, "sy_ip").await;
    if on != "1" {
        let key = if on == "2" {
            "admin_user_00008"
        } else {
            "admin_user_00016"
        };
        return Err(ApiError::business(key));
    }
    let appkey = cfg_of(state, "sy_ip_appkey").await;
    let secret = cfg_of(state, "sy_ip_appsecret").await;
    let url = format!(
        "https://u.ov6.com/ip2addr?appSecret={secret}&appKey={appkey}&ip={ip}"
    );
    let addr = match state.http.get_json::<Ov6Resp>(&url).await {
        Ok(res) if res.code == 200 => res
            .data
            .and_then(|d| d.addr)
            .unwrap_or_default(),
        _ => String::new(),
    };
    let _ = user_repo::update_login_address(state.db.pool(), uid, &addr).await?;
    if addr.is_empty() {
        return Err(ApiError::business("admin_user_00016"));
    }
    Ok(PhpOut::Text("ok", addr))
}

async fn index_get_mobile_address(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let uid = json_u64(body, "uid");
    let phone = json_str(body, "moblie");
    if uid == 0 || phone.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let on = cfg_of(state, "sy_mobile").await;
    if on != "1" {
        return Err(ApiError::business("admin_user_00016"));
    }
    let appkey = cfg_of(state, "sy_mobile_appkey").await;
    let secret = cfg_of(state, "sy_mobile_appsecret").await;
    let url = format!(
        "https://u.ov6.com/mlocation?appSecret={secret}&appKey={appkey}&phone={phone}"
    );
    let loc = match state.http.get_json::<Ov6Resp>(&url).await {
        Ok(res) if res.code == 200 => res
            .data
            .and_then(|d| d.location)
            .unwrap_or_default(),
        _ => String::new(),
    };
    let _ = user_repo::update_moblie_address(state.db.pool(), uid, &loc).await?;
    if loc.is_empty() {
        return Err(ApiError::business("admin_user_00016"));
    }
    Ok(PhpOut::Text("ok", loc))
}

async fn index_wxbind(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    let author = cfg_of(state, "wx_author").await;
    if author != "1" {
        return Err(ApiError::business("common_01335"));
    }
    let login_id = format!("{}{:04}", clock::now_ts(), (uuid::Uuid::now_v7().as_u128() % 10_000));
    let qr = wechat_api_service::create_qr_scene(state, &login_id, 86_400).await?;
    gap_extra::php_insert_wxqrcode(
        state.db.pool(),
        &login_id,
        &qr.ticket,
        clock::now_ts(),
        user.uid,
        0,
    )
    .await?;
    Ok(json!({ "code_url": qr.show_url }))
}

async fn index_wxbind_status(state: &AppState, user: &AuthenticatedUser) -> AppResult<PhpOut> {
    match admin_user_repo::admin_wxid(state.db.reader(), user.uid).await? {
        Some(wxid) => Ok(PhpOut::Data(json!({ "wxid": wxid }))),
        None => Err(ApiError::business("admin_system_00225")),
    }
}

async fn hotjob_del(state: &AppState, _user: &AuthenticatedUser, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let n = company_repo::hotjob_delete_by_uids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_model_00150"))
}

async fn announce_del(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let mut n = 0u64;
    for id in ids {
        if admin_cms_service::delete_announcement(state, user, id)
            .await
            .is_ok()
        {
            n += 1;
        }
    }
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}

async fn zph_space_index(state: &AppState, body: &Value) -> AppResult<Value> {
    let kw = json_str(body, "keyword");
    let keyword = if kw.is_empty() { None } else { Some(kw.as_str()) };
    let rows = zph_repo::list_spaces(state.db.reader(), Some(0), keyword).await?;
    let mut pics = Vec::new();
    let list: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let pic_n = if r.pic.is_empty() {
                String::new()
            } else {
                r.pic.clone()
            };
            if !pic_n.is_empty() {
                pics.push(json!(pic_n));
            }
            json!({
                "id": r.id,
                "name": r.name,
                "sort": r.sort,
                "keyid": r.keyid,
                "pic": r.pic,
                "pic_n": pic_n,
                "content": r.content,
                "price": r.price,
            })
        })
        .collect();
    Ok(json!({ "list": list, "pics": pics }))
}

async fn zph_space_add(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    if body.get("add").is_some() && json_u64(body, "id") == 0 && json_str(body, "name").is_empty() {
        return Ok(PhpOut::Message("ok"));
    }
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::business("admin_01357"));
    }
    let id = json_u64(body, "id");
    let keyid = json_i64(body, "keyid");
    let sort = json_i32(body, "sort");
    let price = json_i32(body, "price");
    let pic = json_str(body, "pic");
    let content = json_str(body, "content").replace("&amp;", "&");
    if id > 0 {
        zph_repo::upsert_space(
            state.db.pool(),
            zph_repo::SpaceUpsert {
                id: Some(id),
                name: &name,
                sort,
                keyid,
                pic: &pic,
                content: &content,
                price,
            },
        )
        .await?;
        return Ok(PhpOut::Message("wap_00225"));
    }
    for part in name.split([',', '，']) {
        let n = part.trim();
        if n.is_empty() {
            continue;
        }
        zph_repo::upsert_space(
            state.db.pool(),
            zph_repo::SpaceUpsert {
                id: None,
                name: n,
                sort,
                keyid,
                pic: &pic,
                content: &content,
                price,
            },
        )
        .await?;
    }
    Ok(PhpOut::Message("wap_js_00091"))
}

async fn zph_space_del(state: &AppState, body: &Value) -> AppResult<PhpOut> {
    let ids = ids_of(body);
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let mut n = 0u64;
    for id in ids {
        n += zph_repo::delete_space(state.db.pool(), id).await?;
    }
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    Ok(PhpOut::Message("admin_user_00187"))
}
