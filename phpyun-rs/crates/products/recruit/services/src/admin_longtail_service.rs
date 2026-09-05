//! Admin company archive / resume review / finance recharge / PHP RBAC tables.

use phpyun_auth::argon2_hash_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::repo as gap_repo;
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company::repo::AdminCompanyRow;
use phpyun_models::company_cert::repo as cert_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::member_statis::repo as member_statis_repo;
use phpyun_models::resume::edu::{self as edu_repo, Edu};
use phpyun_models::resume::entity::Resume;
use phpyun_models::resume::expect::{self as expect_repo, ExpectInput};
use phpyun_models::resume::other::{self as other_repo, Other};
use phpyun_models::resume::project::{self as project_repo, Project};
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume::repo::AdminResumeRow;
use phpyun_models::resume::skill::{self as skill_repo, Skill};
use phpyun_models::resume::training::{self as training_repo, Training};
use phpyun_models::resume::work::{self as work_repo, Work};
use phpyun_models::user::entity::Member;
use phpyun_models::user::repo as user_repo;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

async fn audit_write(
    state: &AppState,
    actor: &AuthenticatedUser,
    action: &'static str,
    target: String,
) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

fn csv_cell(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct CsvExport {
    pub filename: String,
    pub csv: String,
}

pub async fn list_companies(
    state: &AppState,
    r_status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<AdminCompanyRow>> {
    let db = state.db.reader();
    let list = company_repo::list_admin(db, r_status, keyword, page.offset, page.limit).await?;
    let total = company_repo::count_admin(db, r_status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_company_r_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    r_status: i32,
) -> AppResult<()> {
    let n = company_repo::set_r_status(state.db.pool(), uid, r_status).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("company_not_found"));
    }
    audit_write(state, actor, "admin.company.r_status", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn list_company_ratings(
    state: &AppState,
) -> AppResult<Vec<phpyun_models::company::repo::CompanyRatingOpt>> {
    Ok(company_repo::list_rating_options(state.db.reader()).await?)
}

pub async fn set_company_rating(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    rating: i32,
) -> AppResult<()> {
    let opts = company_repo::list_rating_options(state.db.reader()).await?;
    let name = opts
        .iter()
        .find(|r| r.id == rating)
        .map(|r| r.name.clone())
        .ok_or_else(|| ApiError::param_invalid("rating"))?;
    let n = company_repo::set_rating(state.db.pool(), uid, rating, &name).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("company_not_found"));
    }
    let _ = statis_repo::set_rating(state.db.pool(), uid, rating).await;
    audit_write(state, actor, "admin.company.rating", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn export_companies_csv(
    state: &AppState,
    r_status: Option<i32>,
    keyword: Option<&str>,
) -> AppResult<CsvExport> {
    let db = state.db.reader();
    let list = company_repo::list_admin(db, r_status, keyword, 0, 5_000).await?;
    let mut csv = String::from("uid,name,r_status,hy,cityid,hits\n");
    for r in list {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            r.uid,
            csv_cell(&r.name),
            r.r_status,
            r.hy,
            r.cityid,
            r.hits
        ));
    }
    Ok(CsvExport {
        filename: "companies.csv".into(),
        csv,
    })
}

pub async fn list_resumes(
    state: &AppState,
    r_status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<AdminResumeRow>> {
    let db = state.db.reader();
    let list = resume_repo::list_admin(db, r_status, keyword, page.offset, page.limit).await?;
    let total = resume_repo::count_admin(db, r_status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_resume_r_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    r_status: i32,
) -> AppResult<()> {
    let n = resume_repo::update_r_status(state.db.pool(), uid, r_status).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("resume_not_found"));
    }
    audit_write(state, actor, "admin.resume.r_status", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn list_resume_works(state: &AppState, uid: u64) -> AppResult<Vec<Work>> {
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    Ok(phpyun_models::resume::work::list_by_uid(state.db.reader(), uid).await?)
}

pub async fn list_resume_edus(state: &AppState, uid: u64) -> AppResult<Vec<Edu>> {
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    Ok(phpyun_models::resume::edu::list_by_uid(state.db.reader(), uid).await?)
}

pub async fn list_resume_trainings(state: &AppState, uid: u64) -> AppResult<Vec<Training>> {
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    Ok(phpyun_models::resume::training::list_by_uid(state.db.reader(), uid).await?)
}

pub async fn export_resumes_csv(
    state: &AppState,
    r_status: Option<i32>,
    keyword: Option<&str>,
) -> AppResult<CsvExport> {
    let db = state.db.reader();
    let list = resume_repo::list_admin(db, r_status, keyword, 0, 5_000).await?;
    let mut csv = String::from("uid,name,r_status,status,lastupdate\n");
    for r in list {
        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            r.uid,
            csv_cell(&r.name),
            r.r_status,
            r.status,
            r.lastupdate
        ));
    }
    Ok(CsvExport {
        filename: "resumes.csv".into(),
        csv,
    })
}

pub async fn finance_recharge(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    kind: &str,
    amount: i64,
) -> AppResult<i64> {
    if uid == 0 || amount <= 0 {
        return Err(ApiError::param_invalid("amount"));
    }
    let value = match kind {
        "integral" => statis_repo::add_integral(state.db.pool(), uid, amount).await?,
        "vip_days" => {
            statis_repo::extend_vip_days(state.db.pool(), uid, amount, clock::now_ts()).await?
        }
        _ => return Err(ApiError::param_invalid("kind")),
    };
    audit_write(
        state,
        actor,
        "admin.finance.recharge",
        format!("uid:{uid}:{kind}:{amount}"),
    )
    .await;
    Ok(value)
}

pub async fn list_rbac_users(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<rbac_repo::AdminRbacUser>> {
    let db = state.db.reader();
    let list = rbac_repo::list_users(db, page.offset, page.limit).await?;
    let total = rbac_repo::count_users(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_rbac_groups(state: &AppState) -> AppResult<Vec<rbac_repo::AdminRbacGroup>> {
    Ok(rbac_repo::list_groups(state.db.reader()).await?)
}

pub async fn set_rbac_user_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    status: i32,
) -> AppResult<()> {
    let n = rbac_repo::set_user_status(state.db.pool(), uid, status).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("admin_user_not_found"));
    }
    audit_write(state, actor, "admin.rbac.status", format!("uid:{uid}")).await;
    Ok(())
}

fn shanghai_today_bounds(now_utc: i64) -> (i64, i64) {
    const OFFSET: i64 = 8 * 3600;
    let local = now_utc + OFFSET;
    let day_start_local = local - local.rem_euclid(86_400);
    let today = day_start_local - OFFSET;
    (today, today + 86_400 - 1)
}

async fn cfg(state: &AppState, key: &str) -> AppResult<String> {
    Ok(phpyun_models::site_setting::repo::find(state.db.reader(), key)
        .await?
        .map(|s| s.value)
        .unwrap_or_default())
}

/// PHP `company::getCache_action`.
pub async fn company_php_cache(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<serde_json::Value> {
    user.require_admin()?;
    let db = state.db.reader();
    let ratings = company_repo::list_rating_options(db).await?;
    let mut ratingarr = serde_json::Map::new();
    for r in &ratings {
        ratingarr.insert(r.id.to_string(), serde_json::Value::String(r.name.clone()));
    }
    let advisors = rbac_repo::list_crm_advisors(db).await?;
    let mut isgw = serde_json::Map::new();
    isgw.insert("-1".into(), serde_json::Value::String("admin_01303".into()));
    isgw.insert("-2".into(), serde_json::Value::String("admin_user_company_00153".into()));
    for a in &advisors {
        let label = if a.name.is_empty() {
            a.username.clone()
        } else {
            a.name.clone()
        };
        isgw.insert(a.uid.to_string(), serde_json::Value::String(label));
    }
    let domains = phpyun_models::domain::repo::list_all(db).await?;
    let mut dname = serde_json::Map::new();
    for d in domains {
        dname.insert(d.id.to_string(), serde_json::Value::String(d.title));
    }
    let weburl = cfg(state, "sy_weburl").await?;
    let map_key = cfg(state, "map_key").await?;
    let map_secret = cfg(state, "map_secret").await?;
    let hb_bg: Vec<String> = (1..=10)
        .map(|i| format!("{weburl}/data/upload/whb/logo/{i}.png"))
        .collect();
    let (today, today_etime) = shanghai_today_bounds(clock::now_ts());
    let mut payload = serde_json::json!({
        "gwinfo": advisors,
        "source": {},
        "ratingarr": ratingarr.clone(),
        "search_list": {
            "rating": { "name": "admin_user_company_00018", "value": ratingarr },
            "time": { "name": "admin_user_company_00052", "value": {
                "1": "admin_tool_00622", "2": "common_01659", "3": "common_01897",
                "4": "common_01875", "5": "wap_com_00319", "6": "common_01985"
            }},
            "status": { "name": "wap_com_00406", "value": {
                "1": "wap_user_00165", "2": "admin_user_00138", "3": "wap_user_00167",
                "4": "wap_user_00166", "5": "admin_user_00184"
            }},
            "source": { "name": "admin_yunying_00139", "value": {} },
            "rec": { "name": "admin_user_company_00145", "value": { "1": "是", "2": "否", "3": "wap_com_00319" } },
            "gw": { "name": "admin_01231", "value": isgw },
            "has_job": { "name": "admin_user_00045", "value": { "1": "是", "2": "否" } },
            "fact_status": { "name": "wap_00274", "value": { "1": "是", "2": "否" } },
            "map_status": { "name": "member_com_00204", "value": { "1": "是", "2": "否" } }
        },
        "hbBgA": hb_bg,
        "config": {
            "com_social_credit": cfg(state, "com_social_credit").await?,
            "com_cert_owner": cfg(state, "com_cert_owner").await?,
            "com_cert_wt": cfg(state, "com_cert_wt").await?,
            "com_cert_other": cfg(state, "com_cert_other").await?,
            "com_free_status": cfg(state, "com_free_status").await?,
            "pricename": cfg(state, "integral_pricename").await?,
            "today_etime": today_etime,
            "today": today
        },
        "dname": dname,
    });
    crate::admin_dashboard_service::attach_amap(&mut payload, &map_key, &map_secret);
    Ok(payload)
}

/// PHP `CheckRegUser`.
fn check_reg_user(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| {
            matches!(
                c,
                'A'..='Z'
                    | 'a'..='z'
                    | '0'..='9'
                    | '-'
                    | '@'
                    | '#'
                    | '.'
                    | '$'
                    | '_'
                    | '!'
            ) || ('\u{4e00}'..='\u{9fa5}').contains(&c)
        })
}

/// PHP `CheckRegEmail` (simplified; admin checkUsername accepts email-shaped names).
fn check_reg_email(s: &str) -> bool {
    let s = s.trim();
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

/// PHP `userinfo::addMemberCheck` for username only (`checkUsername_action`).
pub async fn check_member_username(
    state: &AppState,
    user: &AuthenticatedUser,
    username: &str,
) -> AppResult<()> {
    user.require_admin()?;
    add_member_check(state, username, "", "", "", None).await
}

/// PHP `CheckMobile`.
fn check_mobile(s: &str) -> bool {
    let s = s.trim();
    let b = s.as_bytes();
    b.len() == 11
        && b[0] == b'1'
        && (b'3'..=b'9').contains(&b[1])
        && b.iter().all(|c| c.is_ascii_digit())
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
        _ => 0,
    }
}

fn gen_salt() -> String {
    Uuid::now_v7().simple().to_string().chars().take(16).collect()
}

/// PHP `userinfo::addMemberCheck` (username / companyName / mobile / email).
async fn add_member_check(
    state: &AppState,
    username: &str,
    company_name: &str,
    mobile: &str,
    email: &str,
    except_uid: Option<u64>,
) -> AppResult<()> {
    let db = state.db.reader();
    if !username.is_empty() {
        if !check_reg_user(username) && !check_reg_email(username) {
            return Err(ApiError::business("wap_00205"));
        }
        if username.eq_ignore_ascii_case("admin") {
            return Err(ApiError::business("common_01147"));
        }
        if user_repo::exists_username_except(db, username, except_uid).await? {
            return Err(ApiError::business("common_01388"));
        }
    }
    if !company_name.is_empty() && company_repo::find_uid_by_name(db, company_name).await?.is_some()
    {
        return Err(ApiError::business("admin_user_00021"));
    }
    if !mobile.is_empty() {
        if !check_mobile(mobile) {
            return Err(ApiError::business("wap_js_00117"));
        }
        if user_repo::exists_mobile_or_username(db, mobile).await? {
            return Err(ApiError::business("api_wxapp_00008"));
        }
    }
    if !email.is_empty() {
        if !check_reg_email(email) {
            return Err(ApiError::business("wap_js_00120"));
        }
        if user_repo::exists_email_or_username(db, email).await? {
            return Err(ApiError::business("default_00012"));
        }
    }
    Ok(())
}

/// PHP `company::add_action` POST (`submit`).
pub async fn create_admin_company(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<u64> {
    user.require_admin()?;
    let username = json_str(body, "username");
    let password = json_str(body, "password");
    let name = json_str(body, "name");
    let mobile = json_str(body, "moblie");
    let email = json_str(body, "email");
    if username.chars().count() < 2 || username.chars().count() > 16 {
        return Err(ApiError::business("admin_user_00084"));
    }
    if password.chars().count() < 6 || password.chars().count() > 20 {
        return Err(ApiError::business("admin_user_00085"));
    }
    add_member_check(state, &username, &name, &mobile, &email, None).await?;

    let mut phone = String::new();
    let areacode = json_str(body, "areacode");
    let telphone = json_str(body, "telphone");
    if !areacode.is_empty() && !telphone.is_empty() {
        phone = format!("{areacode}-{telphone}");
        let exten = json_str(body, "exten");
        if !exten.is_empty() {
            phone.push('-');
            phone.push_str(&exten);
        }
    }

    let now = clock::now_ts();
    let salt = gen_salt();
    let password_hash = argon2_hash_async(format!("{password}{salt}")).await?;
    let shortname = json_str(body, "shortname");
    let address = json_str(body, "address");
    let x = json_str(body, "x");
    let y = json_str(body, "y");
    let linkman = json_str(body, "linkman");
    let content = json_str(body, "content");
    let hy = json_i32(body, "hy");
    let pr = json_i32(body, "pr");
    let mun = json_i32(body, "mun");
    let provinceid = json_i32(body, "provinceid");
    let cityid = json_i32(body, "cityid");
    let three_cityid = json_i32(body, "three_cityid");
    let extra_integral = i64::from(json_i32(body, "integral").max(0));
    let mut rating_id = json_i32(body, "rating_name");
    if rating_id <= 0 {
        rating_id = cfg(state, "com_rating")
            .await?
            .parse::<i32>()
            .unwrap_or(0);
    }
    let pkg = if rating_id > 0 {
        gap_repo::find_rating_package(state.db.reader(), rating_id as u64).await?
    } else {
        None
    };
    let rating_name = pkg.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let rating_type = pkg.as_ref().map(|p| p.r#type).unwrap_or(0);
    let job_num = pkg.as_ref().map(|p| p.job_num).unwrap_or(0);
    let down_resume = pkg.as_ref().map(|p| p.resume).unwrap_or(0);
    let breakjob_num = pkg.as_ref().map(|p| p.breakjob_num).unwrap_or(0);
    let invite_resume = pkg.as_ref().map(|p| p.interview).unwrap_or(0);
    let zph_num = pkg.as_ref().map(|p| p.zph_num).unwrap_or(0);
    let top_num = pkg.as_ref().map(|p| p.top_num).unwrap_or(0);
    let urgent_num = pkg.as_ref().map(|p| p.urgent_num).unwrap_or(0);
    let rec_num = pkg.as_ref().map(|p| p.rec_num).unwrap_or(0);
    let service_time = pkg.as_ref().map(|p| p.service_time).unwrap_or(0);
    let integral_buy: i64 = pkg
        .as_ref()
        .and_then(|p| p.integral_buy.trim().parse().ok())
        .unwrap_or(0);
    let integral = integral_buy + extra_integral;
    let vip_etime = if service_time > 0 {
        now.saturating_add(i64::from(service_time).saturating_mul(86400))
    } else {
        0
    };
    let lastupdate = now.to_string();
    let username_c = username.clone();
    let hash_c = password_hash.clone();
    let salt_c = salt.clone();
    let mobile_c = mobile.clone();
    let email_c = email.clone();
    let address_c = address.clone();
    let name_c = name.clone();
    let short_c = shortname.clone();
    let x_c = x.clone();
    let y_c = y.clone();
    let linkman_c = linkman.clone();
    let phone_c = phone.clone();
    let content_c = content.clone();
    let last_c = lastupdate.clone();
    let rname_c = rating_name.clone();

    let uid = state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                let uid = user_repo::create_member(
                    &mut **tx,
                    &username_c,
                    &hash_c,
                    &salt_c,
                    Some(&mobile_c),
                    if email_c.is_empty() {
                        None
                    } else {
                        Some(email_c.as_str())
                    },
                    2,
                    0,
                    "0.0.0.0",
                    now,
                )
                .await?;
                // MyISAM 不支持事务回滚，后续失败要手工清 member/company。
                if let Err(e) = user_repo::set_address(&mut **tx, uid, &address_c).await {
                    let _ = user_repo::delete_member(&mut **tx, uid).await;
                    return Err(e.into());
                }
                if let Err(e) = company_repo::insert_admin_created(
                    &mut **tx,
                    company_repo::AdminCompanyInsert {
                        uid,
                        name: &name_c,
                        shortname: &short_c,
                        hy,
                        pr,
                        mun,
                        provinceid,
                        cityid,
                        three_cityid,
                        address: &address_c,
                        x: &x_c,
                        y: &y_c,
                        linkman: &linkman_c,
                        linktel: &mobile_c,
                        linkphone: &phone_c,
                        linkmail: &email_c,
                        content: &content_c,
                        lastupdate: &last_c,
                        rating: rating_id,
                        rating_name: &rname_c,
                        vipstime: now,
                        vipetime: vip_etime,
                    },
                )
                .await
                {
                    let _ = user_repo::delete_member(&mut **tx, uid).await;
                    return Err(e.into());
                }
                if let Err(e) = statis_repo::insert_admin_created(
                    &mut **tx,
                    uid,
                    rating_id,
                    &rname_c,
                    rating_type,
                    job_num,
                    down_resume,
                    breakjob_num,
                    invite_resume,
                    zph_num,
                    top_num,
                    urgent_num,
                    rec_num,
                    integral,
                    now,
                    vip_etime,
                )
                .await
                {
                    let _ = statis_repo::delete_by_uid(&mut **tx, uid).await;
                    let _ = company_repo::delete_by_uid(&mut **tx, uid).await;
                    let _ = user_repo::delete_member(&mut **tx, uid).await;
                    return Err(e.into());
                }
                Ok(uid)
            })
        })
        .await?;

    audit_write(state, user, "admin.company.create", format!("uid:{uid}")).await;
    Ok(uid)
}

/// PHP `company::checkComName_action`.
pub async fn check_com_name(
    state: &AppState,
    user: &AuthenticatedUser,
    company_name: &str,
) -> AppResult<Value> {
    user.require_admin()?;
    let name = company_name.trim();
    if name.is_empty() {
        return Ok(json!([]));
    }
    let rows = company_repo::list_kh_by_name(state.db.reader(), name, 50).await?;
    if rows.is_empty() {
        return Ok(json!([]));
    }
    let mut out = vec![json!({ "value": "admin_user_00028" })];
    for r in rows {
        let crm = if r.crm_uid > 0 && !r.crm_name.is_empty() {
            r.crm_name
        } else {
            "admin_user_company_00153".into()
        };
        out.push(json!({ "value": format!("{} ({})", r.name, crm) }));
    }
    Ok(Value::Array(out))
}

async fn company_editor_cache(state: &AppState) -> AppResult<Value> {
    let dicts = crate::dict_service::get(state).await?;
    let cities = crate::category_service::list(state, "city").await?;
    let city_nodes: Vec<(u64, u64, String)> = cities
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let jobs = crate::category_service::list(state, "job").await?;
    let job_nodes: Vec<(u64, u64, String)> = jobs
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let mut payload = crate::admin_dashboard_service::php_cache_payload(
        &job_nodes,
        &city_nodes,
        &dicts.comclass_by_variable("job_edu"),
        &dicts.comclass_by_variable("job_exp"),
    );
    let pr = dicts.comclass_by_variable("job_pr");
    let mun = dicts.comclass_by_variable("job_mun");
    let mut comclass_name = serde_json::Map::new();
    let mut job_pr = Vec::new();
    for (id, name) in &pr {
        job_pr.push(*id);
        comclass_name.insert(id.to_string(), Value::String(name.clone()));
    }
    let mut job_mun = Vec::new();
    for (id, name) in &mun {
        job_mun.push(*id);
        comclass_name.insert(id.to_string(), Value::String(name.clone()));
    }
    let mut job_welfare = Vec::new();
    for (id, name) in dicts.comclass_by_variable("job_welfare") {
        job_welfare.push(id);
        comclass_name.insert(id.to_string(), Value::String(name));
    }
    let hy = dicts.industry_all();
    let industry_index: Vec<i32> = hy.iter().map(|(id, _)| *id).collect();
    let mut industry_name = serde_json::Map::new();
    for (id, name) in hy {
        industry_name.insert(id.to_string(), Value::String(name));
    }
    let cities_v = payload.get("city_types").cloned().unwrap_or(json!([]));
    payload["cache"] = json!({
        "cities": cities_v,
        "industry_index": industry_index,
        "industry_name": industry_name,
        "comdata": { "job_pr": job_pr, "job_mun": job_mun, "job_welfare": job_welfare },
        "comclass_name": comclass_name,
    });
    Ok(payload)
}

/// PHP `company::edit_action`.
pub async fn company_php_edit(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    if uid == 0 {
        return Ok(json!({}));
    }
    let mut payload = company_editor_cache(state).await?;
    let dicts = crate::dict_service::get(state).await?;
    let Some(c) = company_repo::find_by_uid(state.db.reader(), uid).await? else {
        return Err(ApiError::business("admin_user_company_00104"));
    };
    let mut row = serde_json::to_value(&c).unwrap_or(json!({}));
    let welfare: Vec<String> = c
        .welfare
        .clone()
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut all_welfare = welfare.clone();
    for (_, name) in dicts.comclass_by_variable("job_welfare") {
        if !all_welfare.contains(&name) {
            all_welfare.push(name);
        }
    }
    row["arraywelfare"] = json!(welfare);
    row["all_welfare"] = json!(all_welfare);
    payload["row"] = row;
    payload["statis"] = serde_json::to_value(
        statis_repo::find_admin(state.db.reader(), uid)
            .await?
            .unwrap_or(statis_repo::AdminStatisRow {
                rating: 0,
                rating_name: String::new(),
                job_num: 0,
                down_resume: 0,
                breakjob_num: 0,
                invite_resume: 0,
                zph_num: 0,
                top_num: 0,
                urgent_num: 0,
                rec_num: 0,
                vip_stime: 0,
                vip_etime: 0,
                integral: String::new(),
                rating_type: 0,
                suspend_num: 0,
                max_time: 0,
            }),
    )
    .unwrap_or(json!({}));
    if let Some(m) = user_repo::find_by_uid(state.db.reader(), uid).await? {
        payload["com_info"] = json!({
            "uid": m.uid,
            "username": m.username,
            "email": m.email,
            "moblie": m.moblie,
            "status": m.status,
            "usertype": m.usertype,
            "reg_date": m.reg_date,
            "login_date": m.login_date,
        });
    } else {
        payload["com_info"] = json!({});
    }
    let ratings = company_repo::list_rating_options(state.db.reader()).await?;
    payload["rating_list"] = serde_json::to_value(ratings).unwrap_or(json!([]));
    payload["city_name"] = payload
        .get("cache")
        .and_then(|c| c.get("city_name"))
        .cloned()
        .unwrap_or(json!({}));
    Ok(payload)
}

/// PHP `company::comeditsave_action`.
pub async fn company_comeditsave(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let name = json_str(body, "name");
    if name.is_empty() {
        return Err(ApiError::business("admin_user_00021"));
    }
    let content = json_str(body, "content")
        .replace("&amp;", "&")
        .replace("background-color:#ffffff", "background-color:")
        .replace("background-color:#fff", "background-color:")
        .replace("white-space:nowrap;", "white-space:");
    let lastupdate = clock::now_ts().to_string();
    let r_status = if body.get("r_status").is_some() {
        Some(json_i32(body, "r_status"))
    } else {
        None
    };
    let infostatus = if body.get("infostatus").is_some() {
        Some(json_i32(body, "infostatus"))
    } else {
        None
    };
    let sdate_s = json_str(body, "sdate");
    let sdate = if body.get("sdate").is_some() {
        Some(sdate_s.as_str())
    } else {
        None
    };
    let linkjob_s = json_str(body, "linkjob");
    let linkjob = if body.get("linkjob").is_some() {
        Some(linkjob_s.as_str())
    } else {
        None
    };
    let n = company_repo::update_admin_profile(
        state.db.pool(),
        uid,
        company_repo::AdminCompanyProfile {
            name: &name,
            shortname: &json_str(body, "shortname"),
            hy: json_i32(body, "hy"),
            pr: json_i32(body, "pr"),
            mun: json_i32(body, "mun"),
            linkman: &json_str(body, "linkman"),
            linktel: &json_str(body, "linktel"),
            linkphone: &json_str(body, "linkphone"),
            linkmail: &json_str(body, "linkmail"),
            address: &json_str(body, "address"),
            moneytype: json_i32(body, "moneytype"),
            money: json_i32(body, "money"),
            linkqq: &json_str(body, "linkqq"),
            website: &json_str(body, "website"),
            provinceid: json_i32(body, "provinceid"),
            cityid: json_i32(body, "cityid"),
            three_cityid: json_i32(body, "three_cityid"),
            content: &content,
            busstops: &json_str(body, "busstops"),
            welfare: &json_str(body, "checked_welfare"),
            lastupdate: &lastupdate,
            x: &json_str(body, "x"),
            y: &json_str(body, "y"),
            r_status,
            infostatus,
            sdate,
            linkjob,
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_01304"));
    }
    user_repo::update_contact(
        state.db.pool(),
        uid,
        &json_str(body, "linkmail"),
        &json_str(body, "linktel"),
        &json_str(body, "address"),
    )
    .await?;
    audit_write(state, user, "admin.company.edit", format!("uid:{uid}")).await;
    Ok(())
}

/// PHP `company::getinfo_action`.
pub async fn company_php_getinfo(
    state: &AppState,
    user: &AuthenticatedUser,
    comid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    if comid == 0 {
        return Err(ApiError::param_invalid("comid"));
    }
    let Some(c) = company_repo::find_by_uid(state.db.reader(), comid).await? else {
        return Err(ApiError::business("admin_user_company_00104"));
    };
    let mut info = serde_json::to_value(&c).unwrap_or(json!({}));
    let mem = user_repo::find_admin_extras(state.db.reader(), comid).await?;
    if let Some(m) = &mem {
        info["username"] = json!(m.username);
        info["reg_ip"] = json!(m.reg_ip);
        info["status"] = json!(m.status);
        info["lock_info"] = json!(m.lock_info);
        info["wxid"] = json!(m.wxid);
        info["wxopenid"] = json!(m.wxopenid);
        info["reg_date_n"] = if m.reg_date > 0 {
            json!(fmt_ts(m.reg_date))
        } else {
            json!("")
        };
        info["source_n"] = json!("");
    }
    let login_date = c.login_date;
    info["login_date_n"] = if login_date > 0 {
        json!(fmt_ts(login_date))
    } else {
        json!("")
    };
    info["adviser"] = Value::Null;
    info["rating"] = json!(statis_repo::read_rating(state.db.reader(), comid).await?);
    info["phone"] = if c.linktel.as_deref().unwrap_or("").is_empty() {
        json!(c.linkphone)
    } else {
        json!(c.linktel)
    };
    info["vipetime_n"] = json!(phpyun_core::utils::fmt_date(c.vipetime));
    info["package"] = json!(c
        .welfare
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|_| Vec::<String>::new())
        .unwrap_or_default());
    // PHP `package` is a CSV of purchased extras; leave empty when column missing on entity.
    info["did"] = json!(c.did.to_string());
    info["did_name"] = json!("");
    info["yyzzurl"] = json!("");
    info["logo_n"] = json!(c.logo.clone().unwrap_or_default());
    info["zt_days"] = json!(0);
    let db = state.db.reader();
    info["jobNum"] = json!(job_repo::count_by_uid(db, comid).await?);
    info["applyNum"] = json!(
        apply_repo::count_by_com(
            db,
            comid,
            apply_repo::ApplyFilter {
                unread_only: None,
                invited_only: None,
                browse_state: None,
            },
        )
        .await?
    );
    info["integralNum"] = json!(0);
    info["orderNum"] = json!(0);
    info["downNum"] = json!(0);
    info["inviteNum"] = json!(0);
    info["showNum"] = json!(0);
    Ok(info)
}

/// PHP `company::saveUser_action`.
pub async fn company_save_user(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    let username = json_str(body, "username");
    if uid == 0 || username.is_empty() || body.get("status").is_none() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let status = json_i32(body, "status");
    add_member_check(state, &username, "", "", "", Some(uid)).await?;
    let password = json_str(body, "password");
    let hash = if password.is_empty() {
        None
    } else {
        let salt = gen_salt();
        let h = argon2_hash_async(format!("{password}{salt}")).await?;
        Some((h, salt))
    };
    let n = if let Some((ref h, ref salt)) = hash {
        user_repo::update_admin_account(
            state.db.pool(),
            uid,
            &username,
            status,
            &json_str(body, "lock_info"),
            Some((h.as_str(), salt.as_str())),
        )
        .await?
    } else {
        user_repo::update_admin_account(
            state.db.pool(),
            uid,
            &username,
            status,
            &json_str(body, "lock_info"),
            None,
        )
        .await?
    };
    if n == 0 {
        return Err(ApiError::business("admin_user_00082"));
    }
    audit_write(state, user, "admin.company.save_user", format!("uid:{uid}")).await;
    Ok(())
}

fn is_vip(etime: i64, now: i64) -> bool {
    etime == 0 || etime > now
}

fn parse_end_date(s: &str) -> i64 {
    let t = s.trim();
    if t.is_empty() || t == "common_01936" {
        return 0;
    }
    if let Ok(n) = t.parse::<i64>() {
        return if n > 1_000_000_000 { n } else { 0 };
    }
    chrono::NaiveDate::parse_from_str(t, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(23, 59, 59))
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or(0)
}

fn rating_dates(etime: i64, max_time: i64) -> (Value, Value, Value, Value) {
    let vipetime = if etime > 0 {
        json!(phpyun_core::utils::fmt_date(etime))
    } else {
        json!("common_01936")
    };
    let max_n = if max_time > 0 {
        json!(phpyun_core::utils::fmt_date(max_time))
    } else {
        json!("common_01936")
    };
    (json!(etime), vipetime, json!(max_time), max_n)
}

/// PHP `company::getrating_action` / `changeRatingInfo`.
pub async fn company_getrating(
    state: &AppState,
    user: &AuthenticatedUser,
    rating_id: i32,
    uid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    let mut id = rating_id;
    if id <= 0 {
        id = cfg(state, "com_rating").await?.parse().unwrap_or(0);
    }
    let pkg = gap_repo::find_rating_package(state.db.reader(), id as u64)
        .await?
        .ok_or_else(|| ApiError::business("admin_01305"))?;
    let st = statis_repo::find_admin(state.db.reader(), uid)
        .await?
        .unwrap_or(statis_repo::AdminStatisRow {
            rating: 0,
            rating_name: String::new(),
            job_num: 0,
            down_resume: 0,
            breakjob_num: 0,
            invite_resume: 0,
            zph_num: 0,
            top_num: 0,
            urgent_num: 0,
            rec_num: 0,
            vip_stime: 0,
            vip_etime: 0,
            integral: String::new(),
            rating_type: 0,
            suspend_num: 0,
            max_time: 0,
        });
    let now = clock::now_ts();
    let add_on = st.rating_type == pkg.r#type && pkg.r#type == 1 && is_vip(st.vip_etime, now);
    let vip_etime = if pkg.service_time > 0 {
        if add_on && is_vip(st.vip_etime, now) {
            st.vip_etime.saturating_add(i64::from(pkg.service_time) * 86400)
        } else {
            now.saturating_add(i64::from(pkg.service_time) * 86400)
        }
    } else {
        0
    };
    let integral_buy: i64 = pkg.integral_buy.trim().parse().unwrap_or(0);
    let cur_int: i64 = st.integral.trim().parse().unwrap_or(0);
    let (job_num, breakjob_num, down_resume, invite_resume, zph_num, urgent_num, rec_num, top_num, integral) =
        if add_on {
            (
                pkg.job_num,
                st.breakjob_num + pkg.breakjob_num,
                st.down_resume + pkg.resume,
                st.invite_resume + pkg.interview,
                st.zph_num + pkg.zph_num,
                st.urgent_num + pkg.urgent_num,
                st.rec_num + pkg.rec_num,
                st.top_num + pkg.top_num,
                cur_int + integral_buy,
            )
        } else {
            (
                pkg.job_num,
                pkg.breakjob_num,
                pkg.resume,
                pkg.interview,
                pkg.zph_num,
                pkg.urgent_num,
                pkg.rec_num,
                pkg.top_num,
                integral_buy,
            )
        };
    let max_time = if pkg.max_time > 0 {
        now.saturating_add(i64::from(pkg.max_time) * 86400)
    } else {
        0
    };
    let (oldetime, vipetime, max_time_v, max_time_n) = rating_dates(vip_etime, max_time);
    Ok(json!({
        "rating": id,
        "rating_name": pkg.name,
        "rating_type": pkg.r#type,
        "suspend_num": pkg.suspend_num,
        "max_time": max_time_v,
        "max_time_n": max_time_n,
        "job_num": job_num,
        "breakjob_num": breakjob_num,
        "down_resume": down_resume,
        "invite_resume": invite_resume,
        "zph_num": zph_num,
        "urgent_num": urgent_num,
        "rec_num": rec_num,
        "top_num": top_num,
        "integral": integral,
        "vip_etime": vip_etime,
        "vip_stime": now,
        "oldetime": oldetime,
        "vipetime": vipetime,
        "hotjob": 0,
    }))
}

/// PHP `company::getstatis_action`.
pub async fn company_getstatis(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    let st = statis_repo::find_admin(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("admin_user_company_00097"))?;
    let (oldetime, vipetime, max_time_v, max_time_n) = rating_dates(st.vip_etime, st.max_time);
    Ok(json!({
        "rating": st.rating,
        "rating_name": st.rating_name,
        "rating_type": st.rating_type,
        "suspend_num": st.suspend_num,
        "max_time": max_time_v,
        "max_time_n": max_time_n,
        "job_num": st.job_num,
        "breakjob_num": st.breakjob_num,
        "down_resume": st.down_resume,
        "invite_resume": st.invite_resume,
        "zph_num": st.zph_num,
        "urgent_num": st.urgent_num,
        "rec_num": st.rec_num,
        "top_num": st.top_num,
        "integral": st.integral,
        "vip_etime": st.vip_etime,
        "vip_stime": st.vip_stime,
        "oldetime": oldetime,
        "vipetime": vipetime,
        "hotjob": 0,
    }))
}

/// PHP `company::uprating_action`.
pub async fn company_uprating(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let rid = json_i32(body, "rating");
    let uid = json_u64(body, "ratuid");
    if rid <= 0 {
        return Err(ApiError::business("admin_01305"));
    }
    if uid == 0 {
        return Err(ApiError::param_invalid("ratuid"));
    }
    let com = company_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("admin_user_00086"))?;
    if com.r_status == 4 {
        return Err(ApiError::business("admin_user_00024"));
    }
    let pkg = gap_repo::find_rating_package(state.db.reader(), rid as u64)
        .await?
        .ok_or_else(|| ApiError::business("admin_01305"))?;
    let vip_etime = if pkg.service_time == 0 {
        0
    } else {
        parse_end_date(&json_str(body, "vipetime"))
    };
    let max_time = if json_str(body, "max_time").is_empty() || vip_etime == 0 {
        0
    } else {
        parse_end_date(&json_str(body, "max_time"))
    };
    if vip_etime > 0 && max_time > 0 && max_time < vip_etime {
        return Err(ApiError::business("admin_user_company_00081"));
    }
    let now = clock::now_ts();
    let row = statis_repo::AdminStatisRow {
        rating: rid,
        rating_name: pkg.name.clone(),
        job_num: json_i32(body, "job_num"),
        down_resume: json_i32(body, "down_resume"),
        breakjob_num: json_i32(body, "breakjob_num"),
        invite_resume: json_i32(body, "invite_resume"),
        zph_num: json_i32(body, "zph_num"),
        top_num: json_i32(body, "top_num"),
        urgent_num: json_i32(body, "urgent_num"),
        rec_num: json_i32(body, "rec_num"),
        vip_stime: now,
        vip_etime,
        integral: json_str(body, "integral"),
        rating_type: pkg.r#type,
        suspend_num: json_i32(body, "suspend_num"),
        max_time,
    };
    statis_repo::update_admin_quotas(state.db.pool(), uid, &row).await?;
    company_repo::set_rating(state.db.pool(), uid, rid, &pkg.name).await?;
    company_repo::set_vip_times(state.db.pool(), uid, now, vip_etime).await?;
    audit_write(state, user, "admin.company.uprating", format!("uid:{uid}")).await;
    Ok(())
}

/// PHP `company::Imitate_action`：返回会员中心 URL 字符串。
pub async fn company_imitate(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
    typ: &str,
) -> AppResult<String> {
    user.require_admin()?;
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let _ = user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("admin_user_00086"))?;
    let web = cfg(state, "sy_weburl").await?;
    let mut url = format!("{}/member/", web.trim_end_matches('/'));
    if !typ.is_empty() {
        url.push_str(&format!("index.php?c={typ}"));
    }
    audit_write(state, user, "admin.company.imitate", format!("uid:{uid}")).await;
    Ok(url)
}

/// PHP `company::companyAudit_action`.
pub async fn company_php_audit(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<Value> {
    let edit = company_php_edit(state, user, uid).await?;
    let mut info = edit.get("row").cloned().unwrap_or(json!({}));
    if let Some(m) = user_repo::find_admin_extras(state.db.reader(), uid).await? {
        info["statusbody"] = json!(m.lock_info);
        info["member_status"] = json!(m.status);
        info["login_ip"] = json!(m.reg_ip);
        info["login_address"] = json!("");
        info["moblie_address"] = json!("");
        info["reg_date_n"] = json!(fmt_ts(m.reg_date));
        info["login_date_n"] = json!(fmt_ts(m.login_date));
    }
    let snum = company_repo::count_r_status_except(state.db.reader(), 0, uid).await?;
    Ok(json!({
        "Info": info,
        "snum": snum,
        "cache": edit.get("cache").cloned().unwrap_or(json!({})),
        "statis": edit.get("statis").cloned().unwrap_or(json!({})),
        "rating_list": edit.get("rating_list").cloned().unwrap_or(json!([])),
    }))
}

/// PHP `company::suspend_action`.
pub async fn company_suspend(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<()> {
    user.require_admin()?;
    if uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    company_repo::set_r_status(state.db.pool(), uid, 4).await?;
    user_repo::admin_set_status(state.db.pool(), uid, 4).await?;
    audit_write(state, user, "admin.company.suspend", format!("uid:{uid}")).await;
    Ok(())
}

/// PHP `company::comcert_action` 分发。
pub async fn company_comcert(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    user.require_admin()?;
    if body.get("sbody").is_some() {
        let uid = json_u64(body, "uid");
        let sbody = cert_repo::find_type3_note(state.db.reader(), uid).await?;
        return Ok(json!({ "sbody": sbody }));
    }
    if body.get("acwxbind").is_some() {
        return Err(ApiError::business("common_01335"));
    }
    if body.get("comemail").is_some() {
        let email = json_str(body, "comemail");
        if email.is_empty() {
            return Err(ApiError::business("wap_01119"));
        }
        if !check_reg_email(&email) {
            return Err(ApiError::business("wap_js_00120"));
        }
        let uid = json_u64(body, "uid");
        company_repo::set_email_lock(state.db.pool(), uid, &email, json_i32(body, "estatus")).await?;
        return Ok(json!({}));
    }
    if body.get("comlinktel").is_some() {
        let mobile = json_str(body, "comlinktel");
        if mobile.is_empty() {
            return Err(ApiError::business("wap_user_00274"));
        }
        if !check_mobile(&mobile) {
            return Err(ApiError::business("wap_user_00039"));
        }
        let uid = json_u64(body, "uid");
        company_repo::set_mobile_lock(state.db.pool(), uid, &mobile, json_i32(body, "mstatus"))
            .await?;
        return Ok(json!({}));
    }
    if body.get("batchfirm").is_some() {
        let uids = json_str(body, "uid");
        for part in uids.split(',') {
            let uid: u64 = part.trim().parse().unwrap_or(0);
            if uid == 0 {
                continue;
            }
            if body.get("comname_yyzz").is_some() {
                company_repo::set_yyzz(state.db.pool(), uid, 1, None).await?;
                let _ = cert_repo::update_admin_type3(state.db.pool(), uid, 1, "").await;
            }
        }
        return Ok(json!({}));
    }
    let r_status = json_i32(body, "r_status");
    if r_status == 0 {
        return Err(ApiError::business("admin_user_00037"));
    }
    let uid = json_u64(body, "uid");
    let yyzz = if r_status == 1 { 1 } else { 2 };
    let name = json_str(body, "name");
    company_repo::set_yyzz(
        state.db.pool(),
        uid,
        yyzz,
        if name.is_empty() { None } else { Some(&name) },
    )
    .await?;
    cert_repo::update_admin_type3(state.db.pool(), uid, r_status, &json_str(body, "statusbody"))
        .await?;
    Ok(json!({}))
}

async fn create_jobseeker(
    state: &AppState,
    username: &str,
    password: &str,
    mobile: &str,
    email: &str,
    resume_name: &str,
    sex: i32,
    birthday: &str,
    living: &str,
    edu: i32,
    exp: i32,
    description: &str,
) -> AppResult<u64> {
    add_member_check(state, username, "", mobile, email, None).await?;
    let now = clock::now_ts();
    let salt = gen_salt();
    let password_hash = argon2_hash_async(format!("{password}{salt}")).await?;
    let username_c = username.to_string();
    let hash_c = password_hash.clone();
    let salt_c = salt.clone();
    let mobile_c = mobile.to_string();
    let email_c = email.to_string();
    let uid = state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                let uid = user_repo::create_member(
                    &mut **tx,
                    &username_c,
                    &hash_c,
                    &salt_c,
                    Some(&mobile_c),
                    if email_c.is_empty() {
                        None
                    } else {
                        Some(email_c.as_str())
                    },
                    1,
                    0,
                    "0.0.0.0",
                    now,
                )
                .await?;
                if let Err(e) = resume_repo::ensure_row_in_tx(&mut **tx, uid, 0, now).await {
                    let _ = user_repo::delete_member(&mut **tx, uid).await;
                    return Err(e.into());
                }
                Ok(uid)
            })
        })
        .await?;
    if let Err(e) = member_statis_repo::ensure_row(state.db.pool(), uid).await {
        let _ = resume_repo::delete_by_uid(state.db.pool(), uid).await;
        let _ = user_repo::delete_member(state.db.pool(), uid).await;
        return Err(e.into());
    }
    let n = resume_repo::update_admin_basic(
        state.db.pool(),
        uid,
        resume_name,
        sex,
        birthday,
        living,
        edu,
        exp,
        mobile,
        email,
        description,
        now,
    )
    .await?;
    if n == 0 {
        let _ = member_statis_repo::ensure_row(state.db.pool(), uid).await;
        return Err(ApiError::business("common_00978"));
    }
    Ok(uid)
}

/// PHP `users_resume::add_action`：`add=1` 空成功；`add=2` 表单；否则有 uid 更新 / 无 uid 注册。
pub async fn resume_php_add(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    user.require_admin()?;
    let add = json_i32(body, "add");
    if add == 1 {
        return Ok(json!({}));
    }
    if add == 2 {
        let uid = json_u64(body, "uid");
        let mut cache = crate::admin_php_page_service::resume_member_cache(state).await?;
        let resume = if uid > 0 {
            match resume_repo::find_by_uid(state.db.reader(), uid).await? {
                Some(r) => resume_php(&r),
                None => json!(""),
            }
        } else {
            json!("")
        };
        cache
            .as_object_mut()
            .map(|m| m.insert("resume".into(), resume));
        return Ok(cache);
    }
    let uid = json_u64(body, "uid");
    let email = json_str(body, "email");
    let mobile = json_str(body, "moblie");
    let name = json_str(body, "resume_name");
    let sex = json_i32(body, "sex");
    let birthday = birthday_norm(&json_str(body, "birthday"));
    let living = json_str(body, "living");
    let edu = json_i32(body, "edu");
    let exp = json_i32(body, "exp");
    let description = json_str(body, "description");
    let now = clock::now_ts();
    if uid > 0 {
        resume_repo::ensure_row(state.db.pool(), uid, 0, now).await?;
        let n = resume_repo::update_admin_basic(
            state.db.pool(),
            uid,
            &name,
            sex,
            &birthday,
            &living,
            edu,
            exp,
            &mobile,
            &email,
            &description,
            now,
        )
        .await?;
        if n == 0 {
            return Err(ApiError::business("admin_user_00096"));
        }
        user_repo::update_contact(state.db.pool(), uid, &email, &mobile, "").await?;
        audit_write(state, user, "admin.resume.add_update", format!("uid:{uid}")).await;
        return Ok(json!({ "uid": uid }));
    }
    let username = json_str(body, "username");
    let password = json_str(body, "password");
    let nid = create_jobseeker(
        state,
        &username,
        &password,
        &mobile,
        &email,
        &name,
        sex,
        &birthday,
        &living,
        edu,
        exp,
        &description,
    )
    .await?;
    audit_write(state, user, "admin.resume.create", format!("uid:{nid}")).await;
    Ok(json!({ "uid": nid }))
}

/// PHP `users_member::add_action`：`add` 或空用户名为打开表单；否则注册求职者。
pub async fn member_php_add(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    user.require_admin()?;
    let username = json_str(body, "username");
    if json_truthy(body, "add") || username.is_empty() {
        return Ok(json!({}));
    }
    if username.chars().count() < 2 || username.chars().count() > 16 {
        return Err(ApiError::business("admin_user_00084"));
    }
    let password = json_str(body, "password");
    if password.chars().count() < 6 || password.chars().count() > 20 {
        return Err(ApiError::business("admin_user_00085"));
    }
    let mobile = json_str(body, "moblie");
    if mobile.is_empty() {
        return Err(ApiError::business("admin_01285"));
    }
    let email = json_str(body, "email");
    let nid = create_jobseeker(
        state, &username, &password, &mobile, &email, "", 0, "", "", 0, 0, "",
    )
    .await?;
    audit_write(state, user, "admin.member.create", format!("uid:{nid}")).await;
    Ok(json!({ "id": nid, "uid": nid }))
}

async fn expect_bundle(state: &AppState, uid: u64, eid: u64) -> AppResult<Value> {
    let db = state.db.reader();
    let row = if eid > 0 {
        expect_repo::find_admin_by_id(db, eid).await?
    } else {
        expect_repo::find_admin_by_uid(db, uid).await?
    };
    let (job_name, city_name) = name_maps(state).await?;
    Ok(json!({
        "uid": uid,
        "expect": decorate_expect(row.as_ref(), &job_name, &city_name),
        "edu": edus_php(edu_repo::list_by_uid(db, uid).await?),
        "work": works_php(work_repo::list_by_uid(db, uid).await?),
        "training": trainings_php(training_repo::list_by_uid(db, uid).await?),
        "skill": skills_php(skill_repo::list_by_uid(db, uid).await?),
        "project": projects_php(project_repo::list_by_uid(db, uid).await?),
        "other": others_php(other_repo::list_by_uid(db, uid).await?),
        "salary": salary_list(),
    }))
}

/// PHP `users_member::edit_action`.
pub async fn member_php_edit(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let member = user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("wap_com_00228"))?;
    let resume = resume_repo::find_by_uid(state.db.reader(), uid).await?;
    let eid = resume.as_ref().map(|r| r.def_job as u64).unwrap_or(0);
    let mut out = crate::admin_php_page_service::resume_member_cache(state).await?;
    if let Some(obj) = out.as_object_mut() {
        obj.insert("member".into(), member_php(&member));
        obj.insert(
            "resume".into(),
            resume.as_ref().map(resume_php).unwrap_or(json!("")),
        );
        obj.insert("expectData".into(), expect_bundle(state, uid, eid).await?);
    }
    Ok(out)
}

/// PHP `users_resume::editResume_action`.
pub async fn resume_php_edit(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
    eid: u64,
) -> AppResult<Value> {
    user.require_admin()?;
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let resume = resume_repo::find_by_uid(state.db.reader(), uid).await?;
    let use_eid = if eid > 0 {
        eid
    } else {
        resume.as_ref().map(|r| r.def_job as u64).unwrap_or(0)
    };
    let mut out = crate::admin_php_page_service::resume_member_cache(state).await?;
    if let Some(obj) = out.as_object_mut() {
        obj.insert(
            "resume".into(),
            resume.as_ref().map(resume_php).unwrap_or(json!("")),
        );
        obj.insert(
            "expectData".into(),
            expect_bundle(state, uid, use_eid).await?,
        );
        obj.insert("snum".into(), json!(0));
    }
    Ok(out)
}

/// PHP `users_member::editSave_action`（跳过微信二维码上传）。
pub async fn member_edit_save(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let now = clock::now_ts();
    let email = json_str(body, "email");
    let telphone = json_str(body, "telphone");
    let mobile = if telphone.is_empty() {
        json_str(body, "moblie")
    } else {
        telphone.clone()
    };
    resume_repo::ensure_row(state.db.pool(), uid, 0, now).await?;
    let n = resume_repo::update_admin_profile(
        state.db.pool(),
        uid,
        &json_str(body, "name"),
        json_i32(body, "sex"),
        &birthday_norm(&json_str(body, "birthday")),
        json_i32(body, "exp"),
        json_i32(body, "edu"),
        &if telphone.is_empty() {
            mobile.clone()
        } else {
            telphone
        },
        &email,
        &json_str(body, "domicile"),
        &json_str(body, "living"),
        json_i32(body, "marriage"),
        &json_str(body, "height"),
        &json_str(body, "nationality"),
        &json_str(body, "weight"),
        &json_str(body, "idcard"),
        &json_str(body, "address"),
        &json_str(body, "homepage"),
        &json_str(body, "qq"),
        &json_str(body, "description"),
        now,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00096"));
    }
    user_repo::update_contact(state.db.pool(), uid, &email, &mobile, "").await?;
    audit_write(state, user, "admin.member.edit_save", format!("uid:{uid}")).await;
    Ok(())
}

/// PHP `users_resume::saveExpect_action`：新建 `state=1`，更新不打回 `state=0`。
pub async fn resume_save_expect(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<Value> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    let eid = json_u64(body, "eid");
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let now = clock::now_ts();
    let name = json_str(body, "name");
    let job_classid = json_str(body, "job_classid");
    let city_classid = json_str(body, "city_classid");
    let minsalary = json_i32(body, "minsalary");
    let maxsalary = json_i32(body, "maxsalary");
    let input = ExpectInput {
        name: if name.is_empty() { None } else { Some(name.as_str()) },
        job_classid: 0,
        city_classid: 0,
        salary: 0,
        minsalary,
        maxsalary: if maxsalary > 0 { Some(maxsalary) } else { None },
        r#type: json_i32(body, "type"),
        report: json_i32(body, "report"),
        jobstatus: json_i32(body, "jobstatus"),
        hy: json_i32(body, "hy"),
    };
    if eid == 0 {
        let resume = resume_repo::find_by_uid(state.db.reader(), uid)
            .await?
            .ok_or_else(|| ApiError::business("admin_model_00134"))?;
        let nid = expect_repo::create_admin(
            state.db.pool(),
            uid,
            &input,
            &job_classid,
            &city_classid,
            resume.r_status,
            resume.name.as_deref().unwrap_or(""),
            resume.education,
            resume.exp,
            resume.sex,
            resume.birthday.as_deref().unwrap_or(""),
            now,
        )
        .await?;
        if nid == 0 {
            return Err(ApiError::business("admin_model_00134"));
        }
        resume_repo::set_def_job(state.db.pool(), uid, nid).await?;
        audit_write(state, user, "admin.resume.save_expect", format!("eid:{nid}")).await;
        return Ok(json!({ "eid": nid }));
    }
    let n = expect_repo::update_admin(
        state.db.pool(),
        eid,
        uid,
        &input,
        &job_classid,
        &city_classid,
        now,
    )
    .await?;
    if n == 0 && expect_repo::find_admin_by_id(state.db.reader(), eid).await?.is_none() {
        return Err(ApiError::business("admin_model_00136"));
    }
    audit_write(state, user, "admin.resume.save_expect", format!("eid:{eid}")).await;
    Ok(json!({ "eid": eid }))
}

/// PHP `users_resume::saveTag_action`。
pub async fn resume_save_tag(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let uid = json_u64(body, "uid");
    if uid == 0 {
        return Err(ApiError::business("wap_com_00228"));
    }
    let tags = json_tag_list(body);
    if tags.len() > 5 {
        return Err(ApiError::business("admin_user_00206"));
    }
    let description = json_str(body, "description");
    if description.is_empty() {
        return Err(ApiError::business("admin_01319"));
    }
    let mut kept = Vec::new();
    for t in tags {
        let n = t.chars().count();
        if n >= 2 && n <= 8 && !kept.iter().any(|x: &String| x == &t) {
            kept.push(t);
        }
        if kept.len() >= 5 {
            break;
        }
    }
    let now = clock::now_ts();
    resume_repo::ensure_row(state.db.pool(), uid, 0, now).await?;
    let n = resume_repo::update_tag_desc(state.db.pool(), uid, &kept.join(","), &description, now)
        .await?;
    if n == 0 {
        return Err(ApiError::business("admin_01320"));
    }
    audit_write(state, user, "admin.resume.save_tag", format!("uid:{uid}")).await;
    Ok(())
}

fn json_u64(v: &Value, key: &str) -> u64 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn fmt_ts(ts: i64) -> String {
    phpyun_core::utils::fmt_dt(ts)
}

fn json_truthy(v: &Value, key: &str) -> bool {
    match v.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        Some(Value::String(s)) => {
            let t = s.trim();
            !t.is_empty() && t != "0" && !t.eq_ignore_ascii_case("false")
        }
        _ => false,
    }
}

fn json_tag_list(v: &Value) -> Vec<String> {
    match v.get("tag") {
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|x| match x {
                Value::String(s) => Some(s.trim().to_string()),
                Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
            .filter(|s| !s.is_empty())
            .collect(),
        Some(Value::String(s)) => s
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

fn birthday_norm(s: &str) -> String {
    let t = s.trim();
    if t.is_empty() {
        return String::new();
    }
    if let Some(i) = t.find('T') {
        return t[..i].to_string();
    }
    t.chars().take(10).collect()
}

fn salary_list() -> Vec<i32> {
    let mut v = Vec::new();
    let mut i = 1000;
    while i <= 50000 {
        v.push(i);
        i += if i >= 20000 { 5000 } else { 1000 };
    }
    v
}

fn member_php(m: &Member) -> Value {
    json!({
        "uid": m.uid,
        "username": m.username,
        "email": m.email,
        "moblie": m.moblie,
        "usertype": m.usertype,
        "status": m.status,
        "did": m.did,
        "reg_date": m.reg_date,
        "login_date": m.login_date,
    })
}

fn resume_php(r: &Resume) -> Value {
    let tags: Vec<String> = r
        .tag
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut v = serde_json::to_value(r).unwrap_or(json!({}));
    if let Some(obj) = v.as_object_mut() {
        obj.insert("edu".into(), json!(r.education));
        obj.insert("arrayTag".into(), json!(tags));
        obj.insert(
            "wxewm_n".into(),
            json!(r.wxewm.clone().unwrap_or_default()),
        );
    }
    v
}

fn date_n(ts: i64) -> String {
    if ts > 0 {
        phpyun_core::utils::fmt_date(ts)
    } else {
        String::new()
    }
}

fn decorate_expect(
    row: Option<&expect_repo::AdminExpectRow>,
    job_name: &serde_json::Map<String, Value>,
    city_name: &serde_json::Map<String, Value>,
) -> Value {
    let Some(e) = row else {
        return json!({
            "id": 0,
            "name": "",
            "job_classid": "",
            "city_classid": "",
            "minsalary": 0,
            "maxsalary": 0,
            "hy": 0,
            "report": 0,
            "type": 0,
            "jobstatus": 0,
            "jobnameArr": {},
            "citynameArr": {},
            "jobArr": [],
            "cityArr": [],
        });
    };
    let mut jobname_arr = serde_json::Map::new();
    let mut job_arr = Vec::new();
    for id in e.job_classid.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(n) = job_name.get(id) {
            jobname_arr.insert(id.to_string(), n.clone());
            job_arr.push(json!(id));
        }
    }
    let mut cityname_arr = serde_json::Map::new();
    let mut city_arr = Vec::new();
    for id in e.city_classid.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(n) = city_name.get(id) {
            cityname_arr.insert(id.to_string(), n.clone());
            city_arr.push(json!(id));
        }
    }
    let job_classname = jobname_arr
        .values()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let city_classname = cityname_arr
        .values()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    json!({
        "id": e.id,
        "uid": e.uid,
        "name": e.name,
        "hy": e.hy,
        "job_classid": e.job_classid,
        "city_classid": e.city_classid,
        "minsalary": e.minsalary,
        "maxsalary": e.maxsalary,
        "type": e.r#type,
        "report": e.report,
        "jobstatus": e.jobstatus,
        "state": e.state,
        "lastupdate": e.lastupdate,
        "jobnameArr": jobname_arr,
        "citynameArr": cityname_arr,
        "jobArr": job_arr,
        "cityArr": city_arr,
        "job_classname": job_classname,
        "city_classname": city_classname,
    })
}

fn works_php(rows: Vec<Work>) -> Vec<Value> {
    rows.into_iter()
        .map(|w| {
            json!({
                "id": w.id,
                "uid": w.uid,
                "eid": w.eid,
                "name": w.name,
                "sdate": w.sdate,
                "edate": w.edate,
                "sdate_n": date_n(w.sdate),
                "edate_n": if w.edate > 0 { date_n(w.edate) } else { "wap_js_00170".into() },
                "department": w.department,
                "title": w.title,
                "content": w.content,
            })
        })
        .collect()
}

fn edus_php(rows: Vec<Edu>) -> Vec<Value> {
    rows.into_iter()
        .map(|e| {
            json!({
                "id": e.id,
                "uid": e.uid,
                "eid": e.eid,
                "name": e.name,
                "sdate": e.sdate,
                "edate": e.edate,
                "sdate_n": date_n(e.sdate),
                "edate_n": date_n(e.edate),
                "specialty": e.specialty,
                "education": e.education,
            })
        })
        .collect()
}

fn trainings_php(rows: Vec<Training>) -> Vec<Value> {
    rows.into_iter()
        .map(|t| {
            json!({
                "id": t.id,
                "uid": t.uid,
                "eid": t.eid,
                "name": t.name,
                "sdate": t.sdate,
                "edate": t.edate,
                "sdate_n": date_n(t.sdate),
                "edate_n": if t.edate > 0 { date_n(t.edate) } else { "wap_js_00170".into() },
                "title": t.title,
                "content": t.content,
            })
        })
        .collect()
}

fn skills_php(rows: Vec<Skill>) -> Vec<Value> {
    rows.into_iter()
        .map(|s| {
            json!({
                "id": s.id,
                "uid": s.uid,
                "eid": s.eid,
                "name": s.name,
                "longtime": s.years,
                "ing": 0,
                "pic": "",
                "skill": s.level,
            })
        })
        .collect()
}

fn projects_php(rows: Vec<Project>) -> Vec<Value> {
    rows.into_iter()
        .map(|p| {
            json!({
                "id": p.id,
                "uid": p.uid,
                "eid": p.eid,
                "name": p.name,
                "sdate": p.sdate,
                "edate": p.edate,
                "sdate_n": date_n(p.sdate),
                "edate_n": if p.edate > 0 { date_n(p.edate) } else { "wap_js_00170".into() },
                "title": p.role,
                "content": p.content,
            })
        })
        .collect()
}

fn others_php(rows: Vec<Other>) -> Vec<Value> {
    rows.into_iter()
        .map(|o| {
            json!({
                "id": o.id,
                "uid": o.uid,
                "eid": o.eid,
                "name": o.name,
                "content": o.content,
            })
        })
        .collect()
}

async fn name_maps(
    state: &AppState,
) -> AppResult<(serde_json::Map<String, Value>, serde_json::Map<String, Value>)> {
    let jobs = crate::category_service::list(state, "job").await?;
    let cities = crate::category_service::list(state, "city").await?;
    let mut job_name = serde_json::Map::new();
    for c in jobs.iter() {
        job_name.insert(c.id.to_string(), json!(c.name));
    }
    let mut city_name = serde_json::Map::new();
    for c in cities.iter() {
        city_name.insert(c.id.to_string(), json!(c.name));
    }
    Ok((job_name, city_name))
}
