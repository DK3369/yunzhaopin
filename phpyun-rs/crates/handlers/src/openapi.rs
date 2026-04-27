//! OpenAPI schema aggregation — **one per version**.
//!
//! - `/api-docs/v1/openapi.json` — V1 spec
//! - `/api-docs/v2/openapi.json` — V2 spec
//! - `/docs` — Swagger UI top-left dropdown to choose v1 / v2
//!
//! When adding v3: define `V3Doc` and add one more `.url(...)` line in `swagger_ui()`.

use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{v1, v2};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

/// Inject the operation count into each tag's description so Swagger UI shows
/// `<tag> [N 个接口]` in the section header. Swagger UI doesn't compute counts
/// itself; we walk every operation, tally its tags, then prefix each existing
/// tag description with the total. Tags absent from `tags(...)` are auto-added.
pub struct TagCounts;

impl Modify for TagCounts {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use std::collections::BTreeMap;
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for (_path, item) in openapi.paths.paths.iter() {
            let mut count_op = |op: Option<&utoipa::openapi::path::Operation>| {
                if let Some(op) = op {
                    if let Some(tags) = op.tags.as_ref() {
                        for t in tags {
                            *counts.entry(t.clone()).or_insert(0) += 1;
                        }
                    }
                }
            };
            count_op(item.get.as_ref());
            count_op(item.post.as_ref());
            count_op(item.put.as_ref());
            count_op(item.delete.as_ref());
            count_op(item.patch.as_ref());
            count_op(item.head.as_ref());
            count_op(item.options.as_ref());
            count_op(item.trace.as_ref());
        }

        // Update existing tag entries.
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        if let Some(tags) = openapi.tags.as_mut() {
            for t in tags.iter_mut() {
                seen.insert(t.name.clone());
                let n = counts.get(&t.name).copied().unwrap_or(0);
                let base = t.description.clone().unwrap_or_default();
                t.description = Some(format!("【{n} 个接口】 {base}").trim().to_string());
            }
        }
        // Add any missing tags (operations using a tag not declared in `tags(...)`).
        let extra: Vec<_> = counts
            .iter()
            .filter(|(name, _)| !seen.contains(*name))
            .collect();
        if !extra.is_empty() {
            let v = openapi.tags.get_or_insert_with(Vec::new);
            for (name, n) in extra {
                let mut t = utoipa::openapi::tag::TagBuilder::new()
                    .name(name)
                    .description(Some(format!("【{n} 个接口】")))
                    .build();
                let _ = &mut t;
                v.push(t);
            }
        }
    }
}

/// Derive a globally unique `operationId` for each operation (`{method}_{path_slug}`).
///
/// utoipa uses Rust function names as operationId by default; same-named `fn list`
/// / `fn create` / `fn issue` across handler modules all collide, violating the
/// OpenAPI spec (which requires global uniqueness), causing Swagger UI deep-links
/// to jump around and YApi imports to overwrite each other.
pub struct UniqueOperationId;

impl Modify for UniqueOperationId {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        fn slug(path: &str) -> String {
            let mut s: String = path
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            while s.contains("__") {
                s = s.replace("__", "_");
            }
            s.trim_matches('_').to_string()
        }
        for (path, item) in openapi.paths.paths.iter_mut() {
            let base = slug(path);
            let set = |m: &str, op: &mut utoipa::openapi::path::Operation| {
                op.operation_id = Some(format!("{m}_{base}"));
            };
            if let Some(op) = item.get.as_mut() { set("get", op); }
            if let Some(op) = item.post.as_mut() { set("post", op); }
            if let Some(op) = item.put.as_mut() { set("put", op); }
            if let Some(op) = item.delete.as_mut() { set("delete", op); }
            if let Some(op) = item.patch.as_mut() { set("patch", op); }
            if let Some(op) = item.head.as_mut() { set("head", op); }
            if let Some(op) = item.options.as_mut() { set("options", op); }
            if let Some(op) = item.trace.as_mut() { set("trace", op); }
        }
    }
}

// ==================== V1 ====================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PHPYun API v1",
        version = "1.0.0",
        description = "PHPYun WAP API v1 (stable)"
    ),
    servers(
        (url = "/yapi", description = "Nginx proxy (dev.test/yapi → 127.0.0.1:3000)"),
        (url = "/", description = "Direct (local app port 3000)"),
    ),
    paths(
        // auth
        v1::wap::login::mlogin,
        v1::wap::login::login_sms,
        v1::wap::auth::logout,
        v1::wap::auth::refresh,
        v1::wap::auth::me,
        v1::wap::register::register,
        v1::wap::forgetpw::send_sms,
        v1::wap::forgetpw::reset,
        v1::wap::sms::send,
        v1::wap::captcha::issue,
        v1::wap::oauth::oauth_login,
        v1::wap::oauth::oauth_bind,
        // upload
        v1::wap::upload::upload_avatar,
        v1::wap::upload::upload_company_logo,
        v1::wap::upload::upload_resume_photo,
        v1::wap::upload::upload_cert,
        v1::wap::upload::upload_attachment,
        // wap public: jobs
        v1::wap::jobs::list_jobs,
        v1::wap::jobs::job_detail,
        v1::wap::jobs::similar_jobs,
        v1::wap::jobs::same_company_jobs,
        v1::wap::jobs::company_jobs,
        // wap public: companies
        v1::wap::companies::list_companies,
        v1::wap::companies::company_detail,
        // wap public: resumes (employer only)
        v1::wap::resumes::list_resumes,
        v1::wap::resumes::resume_detail,
        // mcenter: account
        v1::mcenter::profile::get_profile,
        v1::mcenter::profile::update_profile,
        v1::mcenter::password::change_password,
        v1::mcenter::oauth_bindings::list_bindings,
        v1::mcenter::oauth_bindings::unbind,
        // mcenter: resume (jobseeker)
        v1::mcenter::resume::get_mine,
        v1::mcenter::resume::update_mine,
        v1::mcenter::resume::update_status,
        // mcenter: company (employer)
        v1::mcenter::company::get_mine,
        v1::mcenter::company::update_mine,
        // mcenter: jobs (employer CRUD)
        v1::mcenter::jobs::list_mine,
        v1::mcenter::jobs::create,
        v1::mcenter::jobs::detail,
        v1::mcenter::jobs::update,
        v1::mcenter::jobs::set_status,
        v1::mcenter::jobs::refresh,
        // v1::mcenter::jobs::delete_job,  // merged into update_or_delete
        v1::mcenter::jobs::batch_refresh,
        v1::mcenter::jobs::batch_close,
        v1::mcenter::jobs::batch_delete,
        // mcenter: apply (jobseeker)
        v1::mcenter::apply::apply_to_job,
        v1::mcenter::apply::list_mine,
        v1::mcenter::apply::withdraw,
        // mcenter: applications (employer view)
        v1::mcenter::applications::list_received,
        v1::mcenter::applications::mark_browsed,
        v1::mcenter::applications::invite,
        // mcenter: favorites
        v1::mcenter::favorites::add,
        v1::mcenter::favorites::remove,
        v1::mcenter::favorites::list,
        v1::mcenter::favorites::exists,
        // mcenter: resume children (jobseeker)
        v1::mcenter::resume_expect::list,
        v1::mcenter::resume_expect::create,
        v1::mcenter::resume_expect::update,
        // v1::mcenter::resume_expect::remove, // removed
        v1::mcenter::resume_edu::list,
        v1::mcenter::resume_edu::create,
        v1::mcenter::resume_edu::update,
        // v1::mcenter::resume_edu::remove, // removed
        v1::mcenter::resume_work::list,
        v1::mcenter::resume_work::create,
        v1::mcenter::resume_work::update,
        // v1::mcenter::resume_work::remove, // removed
        // mcenter: views (visit trail)
        v1::mcenter::views::list_my_views,
        v1::mcenter::views::list_profile_views,
        // mcenter: interviews
        v1::mcenter::interviews::list_mine,
        v1::mcenter::interviews::accept,
        v1::mcenter::interviews::reject,
        v1::mcenter::interviews::create,
        v1::mcenter::interviews::list_by_company,
        v1::mcenter::interviews::cancel,
        // mcenter: messages
        v1::mcenter::messages::list,
        v1::mcenter::messages::mark_read,
        v1::mcenter::messages::mark_all_read,
        v1::mcenter::messages::remove,
        // mcenter: resume_downloads
        v1::mcenter::resume_downloads::download,
        v1::mcenter::resume_downloads::list_outbox,
        v1::mcenter::resume_downloads::list_inbox,
        // mcenter: resume sub-tables (project / skill / language)
        v1::mcenter::resume_project::list,
        v1::mcenter::resume_project::create,
        v1::mcenter::resume_project::update,
        // v1::mcenter::resume_project::remove, // removed
        v1::mcenter::resume_skill::list,
        v1::mcenter::resume_skill::create,
        v1::mcenter::resume_skill::update,
        // v1::mcenter::resume_skill::remove, // removed
        v1::mcenter::resume_language::list,
        v1::mcenter::resume_language::create,
        v1::mcenter::resume_language::update,
        // v1::mcenter::resume_language::remove, // removed
        // wap: dict
        v1::wap::dict::cities,
        v1::wap::dict::cities_of_province,
        v1::wap::dict::industries,
        v1::wap::dict::job_categories,
        v1::wap::dict::educations,
        v1::wap::dict::experiences,
        v1::wap::dict::salaries,
        v1::wap::dict::job_types,
        // wap: articles
        v1::wap::articles::list_articles,
        v1::wap::articles::article_detail,
        // mcenter: feedback / reports / chat
        v1::mcenter::feedback::submit,
        v1::mcenter::feedback::list_mine,
        v1::mcenter::reports::submit,
        v1::mcenter::reports::list_mine,
        v1::mcenter::chat::send,
        v1::mcenter::chat::list_with,
        v1::mcenter::chat::list_conversations,
        v1::mcenter::chat::mark_read,
        v1::mcenter::chat::unread_count,
        // mcenter: VIP
        v1::mcenter::vip::list_packages,
        v1::mcenter::vip::get_current,
        v1::mcenter::vip::create_order,
        v1::mcenter::vip::list_orders,
        v1::mcenter::vip::mock_paid,
        // wap: hot searches
        v1::wap::hot_searches::list,
        // wap: announcements
        v1::wap::announcements::list,
        v1::wap::announcements::detail,
        // wap: zph
        v1::wap::zph::list,
        v1::wap::zph::detail,
        v1::wap::zph::list_companies,
        // wap: qna
        v1::wap::qna::list_questions,
        v1::wap::qna::question_detail,
        v1::wap::qna::list_answers,
        v1::wap::qna::list_categories,
        v1::wap::qna::list_hotweek,
        v1::wap::qna::list_comments,
        // mcenter: zph
        v1::mcenter::zph::reserve,
        v1::mcenter::zph::my_reservation,
        // wap: integral mall
        v1::wap::integral::list_items,
        v1::wap::integral::item_detail,
        // wap: site pages + claim
        v1::wap::site::get_page,
        v1::wap::claim::claim,
        // wap: home + links + search + advice
        v1::wap::home::home,
        v1::wap::links::list,
        v1::wap::search::search,
        v1::wap::advice::submit,
        // mcenter: integral
        v1::mcenter::integral::balance,
        v1::mcenter::integral::exchange,
        v1::mcenter::integral::history,
        v1::mcenter::integral::transfer,
        v1::mcenter::integral::list_transfers,
        // mcenter: invite
        v1::mcenter::invite::send,
        // mcenter: sign-in + dashboard
        v1::mcenter::sign::sign,
        v1::mcenter::sign::status,
        v1::mcenter::dashboard::counts,
        // wap: company sub (products + news)
        v1::wap::company_sub::list_products,
        v1::wap::company_sub::product_detail,
        v1::wap::company_sub::list_news,
        v1::wap::company_sub::news_detail,
        // wap: stats
        v1::wap::stats::overview,
        // mcenter: company sub CRUD
        v1::mcenter::company_sub::list_products,
        v1::mcenter::company_sub::create_product,
        v1::mcenter::company_sub::update_product,
        // delete_product merged into update_product (status:2 soft delete)
        v1::mcenter::company_sub::list_news,
        v1::mcenter::company_sub::create_news,
        v1::mcenter::company_sub::update_news,
        // delete_news merged into update_news (status:2 soft delete)
        // mcenter: interview templates
        v1::mcenter::interview_tpl::list,
        v1::mcenter::interview_tpl::create,
        v1::mcenter::interview_tpl::update,
        // delete merged into update (status:2 soft delete)
        // wap: resume share
        v1::wap::resume_share::view,
        // wap: geo map
        v1::wap::map::jobs_near,
        v1::wap::map::companies_near,
        // wap: share url
        v1::wap::share::job_share,
        v1::wap::share::company_share,
        v1::wap::share::resume_share,
        // wap: career evaluation
        v1::wap::eval::list_papers,
        v1::wap::eval::paper_detail,
        // mcenter: eval
        v1::mcenter::eval::submit,
        v1::mcenter::eval::list_logs,
        // mcenter: company cert
        v1::mcenter::company_cert::get_mine,
        v1::mcenter::company_cert::submit,
        // mcenter: resume completion score
        v1::mcenter::resume_score::completion,
        // mcenter: contact cert (mobile + email)
        v1::mcenter::contact_cert::mobile_send,
        v1::mcenter::contact_cert::mobile_verify,
        v1::mcenter::contact_cert::email_send,
        // wap: email verify callback
        v1::wap::email_verify::verify,
        // wap: specials (special recruitment events)
        v1::wap::specials::list,
        v1::wap::specials::detail,
        v1::wap::specials::companies,
        v1::wap::specials::jobs,
        // wap: parts (public part-time)
        v1::wap::part::list_parts,
        v1::wap::part::part_detail,
        // wap: gongzhao (joint recruitment)
        v1::wap::gongzhao::list,
        v1::wap::gongzhao::detail,
        // wap: ads public
        v1::wap::ads::list,
        // mcenter: blacklist
        v1::mcenter::blacklist::list,
        v1::mcenter::blacklist::add,
        v1::mcenter::blacklist::remove,
        // mcenter: remarks
        v1::mcenter::remarks::list,
        v1::mcenter::remarks::upsert,
        v1::mcenter::remarks::get_one,
        v1::mcenter::remarks::remove,
        // admin: ads
        v1::admin::ads::list,
        v1::admin::ads::create,
        v1::admin::ads::update,
        // remove merged into update (status:2 soft delete)
        // admin: warnings + audit-log
        v1::admin::warnings::list,
        v1::admin::warnings::issue,
        v1::admin::audit_log::list,
        // mcenter: my warnings
        v1::mcenter::warnings::list,
        v1::mcenter::warnings::unread,
        v1::mcenter::warnings::mark_read,
        // admin: broadcasts
        v1::admin::broadcasts::list,
        v1::admin::broadcasts::create,
        v1::admin::broadcasts::remove,
        // mcenter: my broadcasts
        v1::mcenter::broadcasts::list,
        v1::mcenter::broadcasts::unread,
        v1::mcenter::broadcasts::mark_read,
        // mcenter: entrust (jobseeker ↔ headhunter binding)
        v1::mcenter::entrust::list,
        v1::mcenter::entrust::bind,
        v1::mcenter::entrust::unbind,
        v1::mcenter::entrust_search::list_for_headhunter,
        // wap: categories
        v1::wap::categories::list,
        v1::wap::categories::children,
        // admin: categories
        v1::admin::categories::list,
        v1::admin::categories::create,
        v1::admin::categories::update,
        // remove merged into update (status:2 soft delete)
        // mcenter: recommendations
        v1::mcenter::recommend::jobs,
        v1::mcenter::recommend::resumes,
        // wap: app version check
        v1::wap::app_version::latest,
        // admin: app versions
        v1::admin::app_versions::list,
        v1::admin::app_versions::create,
        v1::admin::app_versions::remove,
        // mcenter: my activity log
        v1::mcenter::activity::list,
        // mcenter: saved searches
        v1::mcenter::saved_searches::list,
        v1::mcenter::saved_searches::create,
        v1::mcenter::saved_searches::set_notify,
        v1::mcenter::saved_searches::remove,
        // wap: ratings (public read)
        v1::wap::ratings::list,
        v1::wap::ratings::summary,
        // mcenter: ratings (write)
        v1::mcenter::ratings::rate,
        v1::mcenter::ratings::get_mine,
        v1::mcenter::ratings::unrate,
        // wap: nav + hr docs
        v1::wap::nav::list,
        v1::wap::hr_docs::list,
        v1::wap::hr_docs::detail,
        // admin: nav
        v1::admin::nav::list,
        v1::admin::nav::create,
        v1::admin::nav::update,
        // remove merged into update (status:2 soft delete)
        // mcenter: company HR multi-account
        v1::mcenter::company_hr::create_code,
        v1::mcenter::company_hr::list_codes,
        v1::mcenter::company_hr::revoke_code,
        v1::mcenter::company_hr::list_hrs,
        v1::mcenter::company_hr::remove_hr,
        v1::mcenter::company_hr::join,
        v1::mcenter::company_hr::my_companies,
        // mcenter: resume timeline
        v1::mcenter::resume_timeline::list,
        // admin: dashboard
        v1::admin::dashboard::overview,
        v1::admin::dashboard::recent_signups,
        // admin: orders
        v1::admin::orders::list,
        v1::admin::orders::set_status,
        // mcenter: referrals
        v1::mcenter::referrals::list,
        v1::mcenter::referrals::summary,
        // mcenter: search history
        v1::mcenter::search_history::list,
        v1::mcenter::search_history::clear,
        v1::mcenter::search_history::remove,
        // wap: payment callback
        v1::wap::pay_callback::callback,
        // admin: recycle bin
        v1::admin::recycle::list,
        v1::admin::recycle::detail,
        v1::admin::recycle::purge,
        // wap: redeem mall (public)
        v1::wap::redeem::list_classes,
        v1::wap::redeem::list_rewards,
        v1::wap::redeem::get_reward,
        // mcenter: redeem orders
        v1::mcenter::redeem::redeem,
        v1::mcenter::redeem::list_mine,
        v1::mcenter::redeem::cancel_mine,
        // admin: redeem catalog + orders
        v1::admin::redeem::list_classes,
        v1::admin::redeem::create_class,
        v1::admin::redeem::delete_class,
        v1::admin::redeem::list_rewards,
        v1::admin::redeem::create_reward,
        v1::admin::redeem::delete_reward,
        v1::admin::redeem::set_reward_status,
        v1::admin::redeem::set_reward_flags,
        v1::admin::redeem::list_orders,
        v1::admin::redeem::approve_order,
        v1::admin::redeem::reject_order,
        // wap: descriptions (public cms pages)
        v1::wap::descriptions::list_classes,
        v1::wap::descriptions::list,
        v1::wap::descriptions::get_one,
        // admin: descriptions
        v1::admin::descriptions::list_classes,
        v1::admin::descriptions::create_class,
        v1::admin::descriptions::update_class,
        // delete_class merged into update_class (status:2 soft delete)
        v1::admin::descriptions::list,
        v1::admin::descriptions::upsert,
        v1::admin::descriptions::delete_one,
        // wap: site settings (public)
        v1::wap::site_settings::list,
        v1::wap::site_settings::get_one,
        // admin: site settings manage
        v1::admin::site_settings::list,
        v1::admin::site_settings::upsert,
        v1::admin::site_settings::remove,
        // admin: company cert review
        v1::admin::company_cert::list_pending,
        v1::admin::company_cert::review,
        // mcenter: resume share tokens
        v1::mcenter::resume_share::create,
        v1::mcenter::resume_share::list_mine,
        v1::mcenter::resume_share::revoke,
        // mcenter: qna
        v1::mcenter::qna::ask,
        v1::mcenter::qna::remove,
        v1::mcenter::qna::answer,
        v1::mcenter::qna::accept,
        v1::mcenter::qna::toggle_attention,
        v1::mcenter::qna::support_question,
        v1::mcenter::qna::support_answer,
        v1::mcenter::qna::my_questions,
        v1::mcenter::qna::my_answers,
        v1::mcenter::qna::attended,
        v1::mcenter::qna::post_comment,
        v1::mcenter::qna::remove_comment,
        // admin
        v1::admin::users::list,
        v1::admin::users::set_status,
        v1::admin::reports::list,
        v1::admin::reports::set_status,
        v1::admin::feedback::list,
        v1::admin::feedback::set_status,
        v1::admin::jobs::list,
        v1::admin::jobs::set_state,
        v1::admin::jobs::batch_set_state,
        v1::admin::reports::batch_set_status,
        v1::admin::feedback::batch_set_status,
        // New endpoints (post-audit additions)
        v1::wap::auth::select_usertype,
        v1::wap::regions::city_domain,
        v1::wap::specials::apply,
        v1::wap::once::pay,
        v1::mcenter::atn::toggle,
        v1::mcenter::atn::list_following,
        v1::mcenter::atn::list_followers,
        v1::mcenter::atn::exists,
        v1::mcenter::messages::unread_summary,
        v1::mcenter::once_orders::list_pending,
        v1::mcenter::once_orders::cancel,
        v1::mcenter::dashboard::year_report,
        // Round 3: job-page Q&A + non-id-token OAuth (QQ / Weibo)
        v1::wap::job_messages::list,
        v1::wap::job_messages::create,
        v1::wap::job_messages::hide,
        v1::mcenter::job_messages::list,
        v1::mcenter::job_messages::reply,
        v1::mcenter::job_messages::hide,
        v1::wap::oauth::qq_authorize_url,
        v1::wap::oauth::qq_code_login,
        v1::wap::oauth::weibo_authorize_url,
        v1::wap::oauth::weibo_code_login,
        // Email-based forgot-password channel + manual appeal
        v1::wap::forgetpw::send_email,
        v1::wap::forgetpw::reset_by_email,
        v1::wap::forgetpw::submit_appeal,
        // Round 5: hot companies, zph com-status, job share-text, email recommend
        v1::wap::companies::hot_companies,
        v1::wap::jobs::share_text,
        v1::mcenter::zph::com_status,
        v1::mcenter::recommend::send_email,
        // Round 6: Q&A leaderboard
        v1::wap::qna::list_top_answerers,
        // Round 7: multi-site domains, hits counters, legal slug
        v1::wap::site::list_sub_sites,
        v1::wap::site::match_sub_site,
        v1::wap::articles::bump_hits,
        v1::wap::jobs::bump_jobhits,
        v1::wap::descriptions::get_by_name,
        v1::wap::descriptions::get_legal_page,
        // Round 8: company autocomplete, VIP quote, HR download
        v1::wap::companies::autocomplete,
        v1::mcenter::vip::quote_price,
        v1::wap::hr_docs::track_download,
        // Round 9: map config, eval paper messages
        v1::wap::site::map_config,
        v1::wap::eval::list_messages,
        v1::mcenter::eval::post_message,
        // Round 10: ad click tracking
        v1::wap::ads::track_click,
        // Round 11: recommend quota preflight, eval log detail, recent examinees
        v1::mcenter::recommend::quota,
        v1::mcenter::eval::get_log,
        v1::wap::eval::list_recent_examinees,
        // Round 12: recommended categories
        v1::wap::categories::recommended,
        // Round 13: fans of my company (com-side "对我感兴趣")
        v1::mcenter::fans::list_mine,
        // Round 13: job contact reveal (PHP `getJobLink_action`)
        v1::wap::jobs::job_contact,
        // Round 14: resume hits counter + showuid redirect helper
        v1::wap::resumes::bump_expect_hits,
        v1::wap::resumes::default_expect_by_uid,
        // Round 15: zph jobs (PHP app/zph/index::getJobList_action)
        v1::wap::zph::list_jobs,
    ),
    components(
        schemas(
            v1::wap::login::LoginForm,
            v1::wap::login::LoginSmsForm,
            v1::wap::login::LoginData,
            v1::wap::auth::RefreshForm,
            v1::wap::auth::RefreshData,
            v1::wap::auth::LogoutData,
            v1::wap::auth::MeData,
            v1::wap::register::RegisterForm,
            v1::wap::register::RegisterData,
            v1::wap::forgetpw::SendSmsForm,
            v1::wap::forgetpw::ResetForm,
            v1::wap::sms::SmsSendForm,
            v1::wap::captcha::CaptchaData,
            v1::wap::oauth::OAuthLoginForm,
            v1::wap::oauth::OAuthLoginData,
            v1::wap::oauth::OAuthBindData,
            v1::wap::upload::UploadResult,
            v1::mcenter::profile::ProfileData,
            v1::mcenter::profile::UpdateProfileForm,
            v1::mcenter::password::ChangePasswordForm,
            v1::mcenter::oauth_bindings::BindingsData,
            v1::mcenter::resume::ResumeData,
            v1::mcenter::resume::UpdateResumeForm,
            v1::mcenter::resume::UpdateStatusForm,
            v1::mcenter::company::CompanyData,
            v1::mcenter::company::UpdateCompanyForm,
            v1::wap::jobs::JobSummary,
            // JobDetail is now dynamic nested JSON; no longer has a fixed schema
            v1::mcenter::jobs::CreateJobForm,
            v1::mcenter::jobs::CreateJobData,
            v1::mcenter::jobs::UpdateJobForm,
            v1::mcenter::jobs::SetStatusForm,
            v1::mcenter::jobs::MyJobSummary,
            v1::mcenter::jobs::BatchIdsForm,
            v1::mcenter::jobs::BatchResult,
            v1::mcenter::apply::ApplyForm,
            v1::mcenter::apply::ApplyCreated,
            v1::mcenter::apply::MyApplySummary,
            v1::mcenter::applications::ApplicantSummary,
            v1::mcenter::favorites::AddFavoriteForm,
            v1::mcenter::favorites::ToggleResp,
            v1::mcenter::favorites::ExistsResp,
            v1::mcenter::fans::FanItem,
            v1::wap::companies::CompanySummary,
            v1::wap::companies::CompanyDetail,
            v1::wap::resumes::ResumeSummary,
            v1::wap::resumes::ResumeDetail,
            v1::wap::resumes::ResumeExpectItem,
            v1::wap::resumes::ResumeEduItem,
            v1::wap::resumes::ResumeWorkItem,
            v1::mcenter::resume_expect::ExpectItem,
            v1::mcenter::resume_expect::ExpectForm,
            v1::mcenter::resume_edu::EduItem,
            v1::mcenter::resume_edu::EduForm,
            v1::mcenter::resume_work::WorkItem,
            v1::mcenter::resume_work::WorkForm,
            v1::mcenter::views::ViewItem,
            v1::mcenter::interviews::InterviewItem,
            v1::mcenter::interviews::CreateInterviewForm,
            v1::mcenter::messages::MessageItem,
            v1::mcenter::resume_downloads::DownloadForm,
            v1::mcenter::resume_downloads::DownloadItem,
            v1::mcenter::resume_project::ProjectItem,
            v1::mcenter::resume_project::ProjectForm,
            v1::mcenter::resume_skill::SkillItem,
            v1::mcenter::resume_skill::SkillForm,
            v1::mcenter::resume_language::LanguageItem,
            v1::mcenter::resume_language::LanguageForm,
            v1::wap::dict::DictItem,
            v1::wap::articles::ArticleSummary,
            v1::wap::articles::ArticleDetail,
            v1::mcenter::feedback::FeedbackForm,
            v1::mcenter::feedback::FeedbackItem,
            v1::mcenter::reports::ReportForm,
            v1::mcenter::reports::ReportItem,
            v1::mcenter::chat::SendForm,
            v1::mcenter::chat::SentMessage,
            v1::mcenter::chat::ChatItem,
            v1::mcenter::chat::UnreadCount,
            v1::mcenter::vip::PackageItem,
            v1::mcenter::vip::CurrentVip,
            v1::mcenter::vip::CreateOrderForm,
            v1::mcenter::vip::OrderCreated,
            v1::mcenter::vip::OrderItem,
            v1::wap::hot_searches::HotItem,
            v1::wap::announcements::AnnouncementSummary,
            v1::wap::announcements::AnnouncementDetail,
            v1::wap::zph::ZphSummary,
            v1::wap::zph::ZphDetail,
            v1::wap::zph::ZphCompanyItem,
            v1::wap::qna::QuestionSummary,
            v1::wap::qna::QuestionDetail,
            v1::wap::qna::AnswerItem,
            v1::wap::qna::AskerInfo,
            v1::wap::qna::ViewerInfo,
            v1::wap::qna::CategoryItem,
            v1::wap::qna::CommentItem,
            v1::mcenter::zph::ReserveForm,
            v1::mcenter::zph::ReservedId,
            v1::mcenter::zph::MyReservation,
            v1::mcenter::qna::AskForm,
            v1::mcenter::qna::AnswerBody,
            v1::mcenter::qna::CommentBody,
            v1::mcenter::qna::AcceptBody,
            v1::mcenter::qna::Toggled,
            v1::mcenter::qna::MyQuestion,
            v1::mcenter::qna::MyAnswer,
            v1::wap::integral::IntegralItemView,
            v1::wap::site::SitePageView,
            v1::wap::claim::ClaimForm,
            v1::wap::claim::ClaimResult,
            v1::mcenter::integral::BalanceView,
            v1::mcenter::integral::ExchangedId,
            v1::mcenter::integral::ExchangeItemView,
            v1::mcenter::integral::TransferForm,
            v1::mcenter::integral::TransferResult,
            v1::mcenter::integral::TransferItem,
            v1::wap::home::HomeData,
            v1::wap::home::AnnouncementSummary,
            v1::wap::home::HotKeyword,
            v1::wap::links::LinkItem,
            v1::wap::search::SearchData,
            // SearchData reuses per-domain Summary directly; no separate SearchJob/SearchCompany etc.
            v1::wap::advice::AdviceForm,
            v1::mcenter::invite::InviteForm,
            v1::mcenter::invite::Sent,
            v1::mcenter::sign::SignResp,
            v1::mcenter::sign::StatusResp,
            v1::mcenter::dashboard::DashboardView,
            v1::wap::company_sub::ProductSummary,
            v1::wap::company_sub::ProductDetail,
            v1::wap::company_sub::NewsSummary,
            v1::wap::company_sub::NewsDetail,
            v1::mcenter::company_sub::OwnProduct,
            v1::mcenter::company_sub::OwnNews,
            v1::mcenter::company_sub::ProductForm,
            v1::mcenter::company_sub::ProductPatch,
            v1::mcenter::company_sub::NewsForm,
            v1::mcenter::company_sub::NewsPatch,
            v1::mcenter::interview_tpl::TplItem,
            v1::mcenter::interview_tpl::TplForm,
            v1::mcenter::interview_tpl::TplPatchForm,
            v1::wap::stats::OverviewView,
            v1::wap::resume_share::SharedResume,
            v1::wap::map::NearJob,
            v1::wap::map::NearCompany,
            v1::wap::share::ShareUrl,
            v1::wap::eval::PaperSummary,
            v1::wap::eval::PaperDetail,
            v1::wap::eval::QuestionView,
            v1::mcenter::eval::SubmitForm,
            v1::mcenter::eval::SubmitResult,
            v1::mcenter::eval::LogItem,
            v1::mcenter::company_cert::CertView,
            v1::mcenter::company_cert::SubmitForm,
            v1::admin::company_cert::CertItem,
            v1::admin::company_cert::ReviewForm,
            v1::mcenter::resume_score::Completion,
            v1::wap::site_settings::SettingView,
            v1::admin::site_settings::SettingItem,
            v1::admin::site_settings::UpsertForm,
            v1::mcenter::contact_cert::MobileSendForm,
            v1::mcenter::contact_cert::MobileVerifyForm,
            v1::mcenter::contact_cert::EmailSendForm,
            v1::mcenter::contact_cert::EmailSent,
            v1::wap::specials::SpecialSummary,
            v1::wap::specials::SpecialDetail,
            v1::wap::specials::SpecialCompanyItem,
            v1::wap::specials::SpecialJob,
            v1::wap::part::PartSummary,
            v1::wap::part::PartDetail,
            v1::wap::gongzhao::GzSummary,
            v1::wap::gongzhao::GzDetail,
            v1::wap::ads::AdView,
            v1::mcenter::blacklist::BlackItem,
            v1::mcenter::blacklist::AddForm,
            v1::mcenter::remarks::RemarkView,
            v1::mcenter::remarks::UpsertForm,
            v1::admin::ads::AdItem,
            v1::admin::ads::AdForm,
            v1::admin::ads::AdPatchForm,
            v1::admin::warnings::WarningItem,
            v1::admin::warnings::WarnForm,
            v1::admin::audit_log::AuditItem,
            v1::mcenter::warnings::MyWarning,
            v1::mcenter::warnings::UnreadCount,
            v1::admin::broadcasts::BroadcastItem,
            v1::admin::broadcasts::CreateForm,
            v1::mcenter::broadcasts::BcItem,
            v1::mcenter::broadcasts::UnreadCount,
            v1::mcenter::entrust::EntrustItem,
            v1::mcenter::entrust::BindForm,
            v1::mcenter::entrust::BindResp,
            v1::mcenter::entrust::UnbindForm,
            v1::mcenter::entrust_search::EntrustedSeekerItem,
            v1::wap::categories::CatNode,
            v1::admin::categories::CatItem,
            v1::admin::categories::CatForm,
            v1::admin::categories::CatPatchForm,
            v1::mcenter::recommend::RecJob,
            v1::mcenter::recommend::RecResume,
            v1::wap::app_version::VersionView,
            v1::admin::app_versions::VersionItem,
            v1::admin::app_versions::CreateForm,
            v1::mcenter::activity::ActivityItem,
            v1::mcenter::saved_searches::SavedItem,
            v1::mcenter::saved_searches::CreateForm,
            v1::mcenter::saved_searches::NotifyForm,
            v1::wap::ratings::RatingItem,
            v1::wap::ratings::RatingSummary,
            v1::mcenter::ratings::RateForm,
            v1::mcenter::ratings::MyRating,
            v1::mcenter::company_hr::CodeView,
            v1::mcenter::company_hr::CodeForm,
            v1::mcenter::company_hr::HrView,
            v1::mcenter::company_hr::JoinForm,
            v1::mcenter::company_hr::JoinedResult,
            v1::mcenter::company_hr::MyCompany,
            v1::wap::nav::NavItem,
            v1::wap::hr_docs::HrSummary,
            v1::wap::hr_docs::HrDetail,
            v1::admin::nav::NavItem,
            v1::admin::nav::NavForm,
            v1::admin::nav::NavPatchForm,
            v1::mcenter::resume_timeline::TimelineItem,
            v1::admin::dashboard::OverviewView,
            v1::admin::dashboard::RecentUser,
            v1::mcenter::referrals::ReferralItem,
            v1::mcenter::referrals::SummaryView,
            v1::admin::orders::OrderItem,
            v1::admin::orders::SetStatusForm,
            v1::mcenter::search_history::HistoryItem,
            v1::mcenter::search_history::ClearResult,
            v1::wap::pay_callback::CallbackForm,
            v1::wap::pay_callback::CallbackResult,
            v1::mcenter::resume_share::CreateForm,
            v1::mcenter::resume_share::ShareTokenView,
            v1::admin::users::AdminUserItem,
            v1::admin::users::SetStatusForm,
            v1::admin::reports::AdminReportItem,
            v1::admin::reports::SetReportStatusForm,
            v1::admin::feedback::AdminFeedbackItem,
            v1::admin::feedback::SetFeedbackStatusForm,
            v1::admin::jobs::AdminJobItem,
            v1::admin::jobs::SetJobStateForm,
            v1::admin::jobs::BatchStateForm,
            v1::admin::jobs::BatchResult,
            v1::admin::reports::BatchStatusForm,
            v1::admin::reports::BatchResult,
            v1::admin::feedback::BatchStatusForm,
            v1::admin::feedback::BatchResult,
            // New endpoint schemas (post-audit additions)
            v1::wap::auth::SelectUsertypeForm,
            v1::wap::auth::SelectUsertypeData,
            v1::wap::regions::CityDomainResp,
            v1::wap::specials::ApplyResp,
            v1::wap::once::PayForm,
            v1::wap::once::PayCreated,
            v1::mcenter::atn::FollowToggleForm,
            v1::mcenter::atn::ToggleResp,
            v1::mcenter::atn::FollowItem,
            v1::mcenter::atn::ExistsResp,
            v1::mcenter::messages::UnreadSummary,
            v1::mcenter::once_orders::OrderItem,
            v1::mcenter::dashboard::YearReportView,
            // Round 3 schemas
            v1::wap::job_messages::JobMsgView,
            v1::wap::job_messages::CreateMessageForm,
            v1::wap::job_messages::CreateMessageData,
            v1::mcenter::job_messages::EmployerMsgItem,
            v1::mcenter::job_messages::ReplyForm,
            v1::wap::oauth::QqAuthorizeData,
            v1::wap::oauth::WeiboAuthorizeData,
            v1::wap::oauth::CodeLoginForm,
            v1::wap::forgetpw::SendEmailForm,
            v1::wap::forgetpw::ResetByEmailForm,
            v1::wap::forgetpw::AppealForm,
            v1::wap::forgetpw::AppealResponse,
            v1::wap::companies::HotCompanyView,
            v1::wap::jobs::JobShareText,
            v1::mcenter::zph::ComStatusView,
            v1::mcenter::zph::OwnJobBrief,
            v1::mcenter::recommend::EmailRecommendForm,
            v1::mcenter::recommend::EmailRecommendResp,
            v1::wap::qna::TopAnswererItem,
            v1::wap::site::SubSiteView,
            v1::wap::articles::ArticleHitsResp,
            v1::wap::jobs::JobHitsResp,
            v1::wap::jobs::JobContactView,
            v1::wap::resumes::ResumeHitsResp,
            v1::wap::resumes::DefaultExpectResp,
            v1::wap::companies::CompanyAutoItem,
            v1::mcenter::vip::PriceQuoteView,
            v1::wap::hr_docs::HrDownloadResp,
            v1::wap::site::MapConfigView,
            v1::wap::eval::PaperMessageItem,
            v1::mcenter::eval::PaperMessageForm,
            v1::mcenter::eval::PaperMessageCreated,
            v1::wap::ads::AdClickResp,
            v1::mcenter::recommend::QuotaView,
            v1::wap::eval::ExamineeItem,
            // ==== Phase A: shared DTOs (deduped from per-handler files) ====
            phpyun_core::dto::IdBody,
            phpyun_core::dto::UidBody,
            phpyun_core::dto::EidBody,
            phpyun_core::dto::AidBody,
            phpyun_core::dto::MidBody,
            phpyun_core::dto::PeerBody,
            phpyun_core::dto::TokenBody,
            phpyun_core::dto::OrderNoBody,
            phpyun_core::dto::ProviderBody,
            phpyun_core::dto::UidIdBody,
            phpyun_core::dto::KindTargetIdBody,
            phpyun_core::dto::KindTargetUidBody,
            phpyun_core::dto::CreatedId,
            phpyun_core::dto::Toggled,
        ),
    ),
    modifiers(&SecurityAddon, &UniqueOperationId, &TagCounts),
    tags(
        (name = "auth", description = "Login / Register / Refresh / Logout / Captcha / OAuth"),
        (name = "upload", description = "File upload"),
        (name = "mcenter", description = "Member center"),
        (name = "wap", description = "Public browsing endpoints"),
        (name = "admin", description = "Admin"),
    )
)]
pub struct V1Doc;

// ==================== V2 ====================

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PHPYun API v2",
        version = "2.0.0",
        description = "PHPYun WAP API v2: login response time uses RFC3339; other endpoints match v1"
    ),
    servers(
        (url = "/yapi", description = "Nginx proxy (dev.test/yapi → 127.0.0.1:3000)"),
        (url = "/", description = "Direct (local app port 3000)"),
    ),
    paths(
        v2::wap::login::mlogin,
        // /v2/wap/{logout,refresh,me} routes actually serve the v1 handler (nested router
        // does the path rewrite); OpenAPI still references the v1 path descriptions to avoid duplication
        v1::wap::auth::logout,
        v1::wap::auth::refresh,
        v1::wap::auth::me,
    ),
    components(
        schemas(
            v2::wap::login::LoginForm,
            v2::wap::login::LoginData,
            v1::wap::auth::RefreshForm,
            v1::wap::auth::RefreshData,
            v1::wap::auth::LogoutData,
            v1::wap::auth::MeData,
        ),
    ),
    modifiers(&SecurityAddon, &UniqueOperationId, &TagCounts),
    tags(
        (name = "auth", description = "Login / Refresh / Logout / Current user"),
    )
)]
pub struct V2Doc;

/// Swagger UI single URL with a top-left dropdown to switch between versions.
pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs")
        .url("/api-docs/v1/openapi.json", V1Doc::openapi())
        .url("/api-docs/v2/openapi.json", V2Doc::openapi())
}
