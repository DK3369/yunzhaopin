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

/// Paths under `/v1/wap` that are exempt from the POST-only rule, collected
/// from the modules that own them. Add to the owning module's
/// `GET_ALLOWED_PATHS`, not to a list in the middleware.
pub fn get_allowed_paths() -> Vec<&'static str> {
    let mut v = wechat::GET_ALLOWED_PATHS.to_vec();
    v.extend_from_slice(jobs::GET_ALLOWED_PATHS);
    v.extend_from_slice(companies::GET_ALLOWED_PATHS);
    v.extend_from_slice(home::GET_ALLOWED_PATHS);
    v.extend_from_slice(articles::GET_ALLOWED_PATHS);
    v.extend_from_slice(announcements::GET_ALLOWED_PATHS);
    v.extend_from_slice(search::GET_ALLOWED_PATHS);
    v.extend_from_slice(dict::GET_ALLOWED_PATHS);
    v.extend_from_slice(regions::GET_ALLOWED_PATHS);
    v.extend_from_slice(resumes::GET_ALLOWED_PATHS);
    v.extend_from_slice(zph::GET_ALLOWED_PATHS);
    v.extend_from_slice(part::GET_ALLOWED_PATHS);
    v.extend_from_slice(once::GET_ALLOWED_PATHS);
    v.extend_from_slice(tiny::GET_ALLOWED_PATHS);
    v.extend_from_slice(ads::GET_ALLOWED_PATHS);
    v.extend_from_slice(nav::GET_ALLOWED_PATHS);
    v.extend_from_slice(links::GET_ALLOWED_PATHS);
    v.extend_from_slice(site::GET_ALLOWED_PATHS);
    v.extend_from_slice(specials::GET_ALLOWED_PATHS);
    v.extend_from_slice(gongzhao::GET_ALLOWED_PATHS);
    v.extend_from_slice(qna::GET_ALLOWED_PATHS);
    v.extend_from_slice(redeem::GET_ALLOWED_PATHS);
    v.extend_from_slice(hr_docs::GET_ALLOWED_PATHS);
    v.extend_from_slice(map::GET_ALLOWED_PATHS);
    v.extend_from_slice(eval::GET_ALLOWED_PATHS);
    v.extend_from_slice(categories::GET_ALLOWED_PATHS);
    v.extend_from_slice(countries::GET_ALLOWED_PATHS);
    v.extend_from_slice(hot_searches::GET_ALLOWED_PATHS);
    v
}

#[cfg(test)]
mod get_alias_tests {
    use super::get_allowed_paths;

    #[test]
    fn public_read_get_aliases_are_registered() {
        let v = get_allowed_paths();
        for p in [
            "/v1/wap/jobs",
            "/v1/wap/jobs/detail",
            "/v1/wap/home",
            "/v1/wap/companies",
            "/v1/wap/search",
            "/v1/wap/articles",
            "/v1/wap/announcements",
            "/v1/wap/wechat/callback",
            "/v1/wap/eval-papers",
        ] {
            assert!(v.contains(&p), "missing GET alias {p}");
        }
        let mut sorted = v.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), v.len(), "duplicate GET_ALLOWED_PATHS");
    }
}
