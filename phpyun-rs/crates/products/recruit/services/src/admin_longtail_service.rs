//! Admin company archive / resume review / finance recharge / PHP RBAC tables.

use phpyun_auth::argon2_hash_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::repo as gap_repo;
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::apply::repo as apply_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company::repo::AdminCompanyRow;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::resume::edu::Edu;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume::repo::AdminResumeRow;
use phpyun_models::resume::training::Training;
use phpyun_models::resume::work::Work;
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
