//! Company qualification (`phpyun_company_cert` type=3).
//!
//! PHP: `member/com/model/binding.class.php::savecert_action` +
//! `api/wxapp/member/com/set.class.php::getCert_action` / `saveCert_action`.
//! Status: `0` pending / `1` approved / `2` rejected. Display: if
//! `company.yyzz_status==1` then show approved even without a cert row.

use std::collections::HashMap;

use phpyun_core::{
    audit, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_cert::{
    entity::{CompanyCert, STATUS_APPROVED, STATUS_REJECTED},
    repo as cert_repo,
    repo::CertWrite,
};
use phpyun_models::job::repo as job_repo;
use phpyun_models::member_log::repo as member_log_repo;
use phpyun_models::site_setting::repo as setting_repo;

const SETTING_KEYS: &[&str] = &[
    "com_social_credit",
    "com_cert_owner",
    "com_cert_wt",
    "com_cert_other",
    "com_cert_status",
    "exa_cert_wt",
    "pic_type",
    "file_maxsize",
    "sy_comwebtel",
    "sy_freewebtel",
];

fn cfg_on(map: &HashMap<String, String>, key: &str) -> bool {
    map.get(key).map(|v| v.trim() == "1").unwrap_or(false)
}

fn cfg_flag(map: &HashMap<String, String>, key: &str) -> i32 {
    i32::from(cfg_on(map, key))
}

fn cfg_str(map: &HashMap<String, String>, key: &str) -> String {
    map.get(key).cloned().unwrap_or_default()
}

fn nonempty<'a>(s: &'a str) -> Option<&'a str> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}

pub struct CertMine {
    pub uid: u64,
    pub status: i32,
    pub statusbody: String,
    pub company_name: String,
    pub social_credit: String,
    pub check: String,
    pub owner_cert: String,
    pub wt_cert: String,
    pub other_cert: String,
    pub yyzz_status: i32,
    pub ctime: i64,
    pub com_social_credit: i32,
    pub com_cert_owner: i32,
    pub com_cert_wt: i32,
    pub com_cert_other: i32,
    pub com_cert_status: i32,
    pub exa_cert_wt: String,
    pub pic_type: String,
    pub file_maxsize: String,
    pub review_tel: String,
}

pub struct SubmitInput<'a> {
    pub company_name: &'a str,
    pub social_credit: &'a str,
    pub check: &'a str,
    pub owner_cert: &'a str,
    pub wt_cert: &'a str,
    pub other_cert: &'a str,
}

pub async fn get_mine(state: &AppState, user: &AuthenticatedUser) -> AppResult<CertMine> {
    user.require_employer()?;
    let db = state.db.reader();
    let (company, cert, settings) = tokio::try_join!(
        company_repo::find_by_uid(db, user.uid),
        cert_repo::find(db, user.uid),
        setting_repo::find_many(db, SETTING_KEYS),
    )?;
    let yyzz_status = company.as_ref().map(|c| c.yyzz_status).unwrap_or(0);
    let company_name = company
        .as_ref()
        .and_then(|c| c.name.clone())
        .unwrap_or_default();
    let status = if yyzz_status == 1 {
        STATUS_APPROVED
    } else {
        cert.as_ref().map(|c| c.status).unwrap_or(-1)
    };
    let tel = {
        let com = cfg_str(&settings, "sy_comwebtel");
        if com.trim().is_empty() {
            cfg_str(&settings, "sy_freewebtel")
        } else {
            com
        }
    };
    Ok(CertMine {
        uid: user.uid,
        status,
        statusbody: cert
            .as_ref()
            .map(|c| c.statusbody.clone())
            .unwrap_or_default(),
        company_name,
        social_credit: cert
            .as_ref()
            .map(|c| c.social_credit.clone())
            .unwrap_or_default(),
        check: cert.as_ref().map(|c| c.check.clone()).unwrap_or_default(),
        owner_cert: cert
            .as_ref()
            .map(|c| c.owner_cert.clone())
            .unwrap_or_default(),
        wt_cert: cert.as_ref().map(|c| c.wt_cert.clone()).unwrap_or_default(),
        other_cert: cert
            .as_ref()
            .map(|c| c.other_cert.clone())
            .unwrap_or_default(),
        yyzz_status,
        ctime: cert.as_ref().map(|c| c.ctime).unwrap_or(0),
        com_social_credit: cfg_flag(&settings, "com_social_credit"),
        com_cert_owner: cfg_flag(&settings, "com_cert_owner"),
        com_cert_wt: cfg_flag(&settings, "com_cert_wt"),
        com_cert_other: cfg_flag(&settings, "com_cert_other"),
        com_cert_status: cfg_flag(&settings, "com_cert_status"),
        exa_cert_wt: cfg_str(&settings, "exa_cert_wt"),
        pic_type: cfg_str(&settings, "pic_type"),
        file_maxsize: cfg_str(&settings, "file_maxsize"),
        review_tel: tel,
    })
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    input: SubmitInput<'_>,
    client_ip: &str,
) -> AppResult<&'static str> {
    user.require_employer()?;
    let name = input.company_name.trim();
    if name.is_empty() {
        return Err(ApiError::business("common_06399"));
    }
    let db = state.db.pool();
    company_repo::ensure_row(db, user.uid, user.did).await?;
    let company = company_repo::find_by_uid(db, user.uid).await?;
    let cert = cert_repo::find(db, user.uid).await?;
    let settings = setting_repo::find_many(state.db.reader(), SETTING_KEYS).await?;

    if company_repo::count_name_except(db, name, user.uid).await? > 0 {
        return Err(ApiError::business("wap_01269"));
    }

    let social = input.social_credit.trim();
    if cfg_on(&settings, "com_social_credit") && social.is_empty() {
        return Err(ApiError::business("common_05983"));
    }

    let check = nonempty(input.check).or_else(|| cert.as_ref().and_then(|c| nonempty(&c.check)));
    if check.is_none() {
        return Err(ApiError::business("member_com_00182"));
    }
    let owner = nonempty(input.owner_cert)
        .or_else(|| cert.as_ref().and_then(|c| nonempty(&c.owner_cert)));
    if cfg_on(&settings, "com_cert_owner") && owner.is_none() {
        return Err(ApiError::business("member_com_00176"));
    }
    let wt =
        nonempty(input.wt_cert).or_else(|| cert.as_ref().and_then(|c| nonempty(&c.wt_cert)));
    if cfg_on(&settings, "com_cert_wt") && wt.is_none() {
        return Err(ApiError::business("member_com_00173"));
    }
    let r_status = company.as_ref().map(|c| c.r_status).unwrap_or(0);
    let status = if r_status == 0 {
        0
    } else if cfg_on(&settings, "com_cert_status") {
        0
    } else {
        1
    };

    let is_new = cert.is_none();
    let now = clock::now_ts();
    cert_repo::upsert_type3(
        db,
        &CertWrite {
            uid: user.uid,
            did: user.did,
            status,
            social_credit: social,
            check,
            owner_cert: nonempty(input.owner_cert),
            wt_cert: nonempty(input.wt_cert),
            other_cert: nonempty(input.other_cert),
            now,
        },
    )
    .await?;
    company_repo::set_yyzz(db, user.uid, status, Some(name)).await?;
    let _ = job_repo::set_yyzz_by_uid(db, user.uid, status, Some(name)).await?;
    let _ = member_log_repo::insert(
        db,
        member_log_repo::InsertInput {
            uid: user.uid,
            opera: 12,
            type_: if is_new { 1 } else { 2 },
            usertype: 2,
            content: if is_new {
                "common_06385"
            } else {
                "common_06384"
            },
            ip: client_ip,
            ctime: now,
            did: user.did,
        },
    )
    .await;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company_cert.submit", audit::Actor::uid(user.uid)),
    )
    .await;

    let need_review = cfg_on(&settings, "com_cert_status") || r_status == 0;
    Ok(if is_new {
        if need_review {
            "common_00965"
        } else {
            "common_05610"
        }
    } else if need_review {
        "common_01020"
    } else {
        "admin_system_00064"
    })
}

// ---------- admin ----------

pub async fn list_pending(state: &AppState, page: Pagination) -> AppResult<Paged<CompanyCert>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        cert_repo::list_pending(db, page.offset, page.limit),
        cert_repo::count_pending(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn review(
    state: &AppState,
    admin: &AuthenticatedUser,
    target_uid: u64,
    approve: bool,
    note: &str,
) -> AppResult<()> {
    admin.require_admin()?;
    let status = if approve {
        STATUS_APPROVED
    } else {
        STATUS_REJECTED
    };
    let affected = cert_repo::review(state.db.pool(), target_uid, status, note).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("cert_not_pending"));
    }
    let yyzz = if approve { 1 } else { 2 };
    let _ = company_repo::set_yyzz(state.db.pool(), target_uid, yyzz, None).await?;
    let _ = job_repo::set_yyzz_by_uid(state.db.pool(), target_uid, yyzz, None).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.company_cert.review", audit::Actor::uid(admin.uid))
            .target(format!("uid:{target_uid}"))
            .meta(&serde_json::json!({ "approve": approve, "note": note })),
    )
    .await;
    Ok(())
}
