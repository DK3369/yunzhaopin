//! Services layer: orchestration of repos + side-effects + caching.
//!
//! ## Architecture rules
//!
//! 1. **No direct third-party imports for caching / HTTP / Redis / JWT / Cron.**
//!    Reach for the `phpyun_core` facade instead:
//!    | concern   | use this                                  | DO NOT use         |
//!    |-----------|-------------------------------------------|--------------------|
//!    | caching   | `phpyun_core::cache::SimpleCache`         | `moka::*`          |
//!    |           | `phpyun_core::cache::get_or_load` (L1+L2) |                    |
//!    | redis     | `phpyun_core::kv::Kv` / `events`          | `redis::*`         |
//!    | http out  | `phpyun_core::http_client`                | `reqwest::*`       |
//!    | jwt       | `phpyun_core::jwt`                        | `jsonwebtoken::*`  |
//!    | cron      | `phpyun_core::scheduler`                  | `cron::*`          |
//!
//! 2. **SQL belongs in `phpyun_models::*::repo`.** Services orchestrate calls
//!    to repos and other services; raw `sqlx::query*` here means a missing repo
//!    method — add the method to the repo and call it.
//!
//! 3. **CI guard:** `scripts/check-architecture.sh` greps for violations and
//!    fails the build. Run it locally before committing.
//!
//! Pre-existing violations are tagged `// TODO(arch):` and migrated opportunistically.

pub mod ad_service;
pub mod admin_dashboard_service;
pub mod admin_service;
pub mod announcement_service;
pub mod app_version_service;
pub mod apply_service;
pub mod article_service;
pub mod atn_service;
pub mod audit_log_service;
pub mod blacklist_service;
pub mod broadcast_service;
pub mod captcha_service;
pub mod category_service;
pub mod chat_service;
pub mod claim_service;
pub mod collect_service;
pub mod company_address_service;
pub mod company_banner_service;
pub mod company_cert_service;
pub mod company_content_service;
pub mod company_hr_service;
pub mod company_tpl_service;
pub mod company_service;
pub mod company_sub_service;
pub mod contact_cert_service;
pub mod country_service;
pub mod dashboard_service;
pub mod data_show_service;
pub mod description_service;
pub mod dict_service;
pub mod domain_errors;
pub mod entrust_service;
pub mod eval_service;
pub mod fan_service;
pub mod feedback_service;
pub mod friend_link_service;
pub mod gallery_service;
pub mod gongzhao_service;
pub mod home_service;
pub mod hot_search_service;
pub mod hr_doc_service;
pub mod integral_service;
pub mod interview_service;
pub mod interview_template_service;
pub mod invite_service;
pub mod job_mgmt_service;
pub mod job_msg_service;
pub mod job_service;
pub mod maintenance;
pub mod map_service;
pub mod mcenter_service;
pub mod member_logout_service;
pub mod message_service;
pub mod nav_menu_service;
pub mod notification_consumers;
pub mod oauth_service;
pub mod once_service;
pub mod part_service;
pub mod password_reset_service;
pub mod poster_service;
pub mod qna_service;
pub mod rating_service;
pub mod recommend_email_service;
pub mod recommend_service;
pub mod recycle_bin_service;
pub mod redeem_service;
pub mod referral_service;
pub mod region_service;
pub mod registration_service;
pub mod remark_service;
pub mod report_service;
pub mod resume_children_service;
pub mod resume_download_service;
pub mod resume_out_service;
pub mod resume_score_service;
pub mod resume_service;
pub mod resume_share_service;
pub mod resume_timeline_service;
pub mod resume_tpl_service;
pub mod saved_search_service;
pub mod search_history_service;
pub mod search_service;
pub mod sign_service;
pub mod site_page_service;
pub mod site_setting_service;
pub mod sms_service;
pub mod special_service;
pub mod stats_service;
pub mod sysmsg_service;
pub mod talent_pool_service;
pub mod tiny_service;
pub mod transfer_service;
pub mod user_error;
pub mod user_service;
pub mod user_session_service;
pub mod collect_cache;
pub mod usertype_change_service;
pub mod view_service;
pub mod vip_service;
pub mod warning_service;
pub mod wechat_api_service;
pub mod wechat_service;
pub mod zph_service;

pub use domain_errors::{ApplyError, CollectError, CompanyError, JobError, ResumeError};
pub use user_error::UserError;
