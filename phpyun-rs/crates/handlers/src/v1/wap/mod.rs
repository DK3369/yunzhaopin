pub mod ads;
pub mod advice;
pub mod announcements;
pub mod app_version;
pub mod articles;
pub mod auth;
pub mod captcha;
pub mod categories;
pub mod claim;
pub mod companies;
pub mod company_sub;
pub mod countries;
pub mod data_show;
pub mod descriptions;
pub mod dict;
pub mod email_verify;
pub mod eval;
pub mod forgetpw;
pub mod gongzhao;
pub mod home;
pub mod hot_searches;
pub mod hr_docs;
pub mod integral;
pub mod job_messages;
pub mod jobs;
pub mod links;
pub mod login;
pub mod map;
pub mod nav;
pub mod oauth;
pub mod once;
pub mod part;
pub mod pay_callback;
pub mod poster;
pub mod qna;
pub mod ratings;
pub mod redeem;
pub mod regions;
pub mod register;
pub mod resume_share;
pub mod resumes;
pub mod search;
pub mod share;
pub mod site;
pub mod site_settings;
pub mod sms;
pub mod specials;
pub mod stats;
pub mod tiny;
pub mod upload;
pub mod wechat;
pub mod zph;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(login::routes())
        .merge(auth::routes())
        .merge(upload::routes())
        .merge(register::routes())
        .merge(forgetpw::routes())
        .merge(sms::routes())
        .merge(captcha::routes())
        .merge(oauth::routes())
        .merge(jobs::routes())
        .merge(job_messages::routes())
        .merge(companies::routes())
        .merge(resumes::routes())
        .merge(dict::routes())
        .merge(regions::routes())
        .merge(countries::routes())
        .merge(articles::routes())
        .merge(hot_searches::routes())
        .merge(announcements::routes())
        .merge(zph::routes())
        .merge(qna::routes())
        .merge(integral::routes())
        .merge(site::routes())
        .merge(claim::routes())
        .merge(home::routes())
        .merge(links::routes())
        .merge(search::routes())
        .merge(advice::routes())
        .merge(company_sub::routes())
        .merge(stats::routes())
        .merge(resume_share::routes())
        .merge(map::routes())
        .merge(share::routes())
        .merge(eval::routes())
        .merge(site_settings::routes())
        .merge(email_verify::routes())
        .merge(specials::routes())
        .merge(gongzhao::routes())
        .merge(ads::routes())
        .merge(categories::routes())
        .merge(app_version::routes())
        .merge(ratings::routes())
        .merge(nav::routes())
        .merge(hr_docs::routes())
        .merge(pay_callback::routes())
        .merge(redeem::routes())
        .merge(part::routes())
        .merge(descriptions::routes())
        .merge(tiny::routes())
        .merge(wechat::routes())
        .merge(data_show::routes())
        .merge(once::routes())
        .merge(poster::routes())
}
