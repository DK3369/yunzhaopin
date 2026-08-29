//! Admin company archive / resume review / finance recharge / PHP RBAC tables.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_rbac::repo as rbac_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::company::repo::AdminCompanyRow;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::resume::edu::Edu;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume::repo::AdminResumeRow;
use phpyun_models::resume::training::Training;
use phpyun_models::resume::work::Work;
use phpyun_models::user::repo as user_repo;
use serde::Serialize;
use serde_json::{json, Value};

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
    let username = username.trim();
    if username.is_empty() {
        return Ok(());
    }
    if !check_reg_user(username) && !check_reg_email(username) {
        return Err(ApiError::business("wap_00205"));
    }
    if username.eq_ignore_ascii_case("admin") {
        return Err(ApiError::business("common_01147"));
    }
    if user_repo::exists_username(state.db.reader(), username).await? {
        return Err(ApiError::business("common_01388"));
    }
    Ok(())
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
