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
use serde::Serialize;

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
