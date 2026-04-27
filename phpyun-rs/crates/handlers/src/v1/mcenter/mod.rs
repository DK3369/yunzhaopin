//! Member center (/v1/mcenter/*).

pub mod account_logout;
pub mod activity;
pub mod applications;
pub mod apply;
pub mod atn;
pub mod blacklist;
pub mod broadcasts;
pub mod chat;
pub mod company;
pub mod company_cert;
pub mod company_content;
pub mod company_hr;
pub mod company_skin;
pub mod company_sub;
pub mod contact_cert;
pub mod dashboard;
pub mod entrust;
pub mod entrust_search;
pub mod eval;
pub mod fans;
pub mod favorites;
pub mod feedback;
pub mod integral;
pub mod interview_tpl;
pub mod interviews;
pub mod invite;
pub mod job_messages;
pub mod jobs;
pub mod messages;
pub mod oauth_bindings;
pub mod once_orders;
pub mod part;
pub mod password;
pub mod profile;
pub mod qna;
pub mod ratings;
pub mod recommend;
pub mod redeem;
pub mod referrals;
pub mod remarks;
pub mod reports;
pub mod resume;
pub mod resume_downloads;
pub mod resume_edu;
pub mod resume_expect;
pub mod resume_language;
pub mod resume_out;
pub mod resume_project;
pub mod resume_score;
pub mod resume_share;
pub mod resume_skill;
pub mod resume_timeline;
pub mod resume_tpl;
pub mod resume_work;
pub mod saved_searches;
pub mod search_history;
pub mod sessions;
pub mod sign;
pub mod sysmsg;
pub mod talent_pool;
pub mod transfer;
pub mod username;
pub mod usertype_change;
pub mod views;
pub mod vip;
pub mod warnings;
pub mod zph;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(profile::routes())
        .merge(password::routes())
        .merge(oauth_bindings::routes())
        .merge(resume::routes())
        .merge(resume_expect::routes())
        .merge(resume_edu::routes())
        .merge(resume_work::routes())
        .merge(resume_project::routes())
        .merge(resume_skill::routes())
        .merge(resume_language::routes())
        .merge(company::routes())
        .merge(jobs::routes())
        .merge(job_messages::routes())
        .merge(apply::routes())
        .merge(applications::routes())
        .merge(favorites::routes())
        .merge(fans::routes())
        .merge(views::routes())
        .merge(interviews::routes())
        .merge(messages::routes())
        .merge(resume_downloads::routes())
        .merge(feedback::routes())
        .merge(reports::routes())
        .merge(chat::routes())
        .merge(vip::routes())
        .merge(zph::routes())
        .merge(qna::routes())
        .merge(integral::routes())
        .merge(invite::routes())
        .merge(sign::routes())
        .merge(dashboard::routes())
        .merge(company_sub::routes())
        .merge(interview_tpl::routes())
        .merge(resume_share::routes())
        .merge(eval::routes())
        .merge(company_cert::routes())
        .merge(resume_score::routes())
        .merge(contact_cert::routes())
        .merge(blacklist::routes())
        .merge(remarks::routes())
        .merge(warnings::routes())
        .merge(broadcasts::routes())
        .merge(entrust::routes())
        .merge(entrust_search::routes())
        .merge(recommend::routes())
        .merge(activity::routes())
        .merge(saved_searches::routes())
        .merge(ratings::routes())
        .merge(company_hr::routes())
        .merge(resume_timeline::routes())
        .merge(referrals::routes())
        .merge(search_history::routes())
        .merge(sessions::routes())
        .merge(redeem::routes())
        .merge(part::routes())
        .merge(talent_pool::routes())
        .merge(transfer::routes())
        .merge(resume_out::routes())
        .merge(company_skin::routes())
        .merge(sysmsg::routes())
        .merge(account_logout::routes())
        .merge(atn::routes())
        .merge(once_orders::routes())
        .merge(company_content::routes())
        .merge(resume_tpl::routes())
        .merge(username::routes())
        .merge(usertype_change::routes())
}
