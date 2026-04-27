//! Poster (whb) service — emits the metadata required for composition; the client composes the PNG via Canvas.
//!
//! Mirrors PHPYun `whb.model::{getJobHb, getInviteRegHb, getGongzhaoHb, getComHb}`.
//! Difference: the PHP server uses GD to compose and serve the PNG directly; the Rust version returns a JSON composition spec.
//!
//! Composition spec `PosterSpec`:
//! - `template`: template metadata (background image URL + config_pos JSON)
//! - `qr_scene`: result of `wechat_api_service::scene_str_for` — the client uses it to call
//!   `/v1/wap/wechat/qr/{kind}/{id}` to obtain the real QR code
//! - `fields`: business fields to render (job name / company name / salary / inviter uid, etc.)

use phpyun_core::{AppError, AppResult, AppState, InfraError};
use phpyun_models::poster_template::entity::{PosterKind, PosterTemplate};
use phpyun_models::poster_template::repo as tpl_repo;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct PosterSpec {
    pub template: PosterTemplateView,
    /// QR-code scene_str (the client calls the qr endpoint to exchange it for a ticket)
    pub qr_scene: String,
    /// Flat map of business fields — the front end looks them up by the keys in config_pos
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PosterTemplateView {
    pub id: u64,
    pub title: String,
    pub pic: Option<String>,
    pub config_pos: Option<String>,
}

impl From<PosterTemplate> for PosterTemplateView {
    fn from(t: PosterTemplate) -> Self {
        Self {
            id: t.id,
            title: t.title,
            pic: t.pic,
            config_pos: t.config_pos,
        }
    }
}

pub async fn list_templates(
    state: &AppState,
    kind_str: &str,
) -> AppResult<Vec<PosterTemplateView>> {
    let kind = parse_kind(kind_str)?;
    let list = tpl_repo::list_by_kind(state.db.reader(), kind).await?;
    Ok(list.into_iter().map(PosterTemplateView::from).collect())
}

fn parse_kind(s: &str) -> AppResult<PosterKind> {
    PosterKind::parse(s)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam(format!("poster_kind={s}"))))
}

async fn pick_template(
    state: &AppState,
    kind: PosterKind,
    hb: Option<u64>,
) -> AppResult<PosterTemplate> {
    let tpl = match hb {
        Some(id) if id > 0 => tpl_repo::find_by_id(state.db.reader(), id).await?,
        _ => tpl_repo::default_for_kind(state.db.reader(), kind).await?,
    };
    tpl.ok_or_else(|| {
        AppError::new(InfraError::InvalidParam("poster_template_not_found".into()))
    })
}

// ==================== Job poster ====================

/// Mirrors PHPYun `whb.model::getJobHb`
pub async fn job_poster_spec(
    state: &AppState,
    hb: Option<u64>,
    job_id: u64,
) -> AppResult<PosterSpec> {
    let tpl = pick_template(state, PosterKind::Job, hb).await?;

    // Fetch the job data
    let job = phpyun_models::job::repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("job_not_found".into())))?;

    let mut fields = BTreeMap::new();
    fields.insert("job_name".into(), job.name);
    fields.insert("com_name".into(), job.com_name.unwrap_or_default());
    fields.insert(
        "salary".into(),
        ((job.minsalary + job.maxsalary) / 2).to_string(),
    );
    fields.insert("min_salary".into(), job.minsalary.to_string());
    fields.insert("max_salary".into(), job.maxsalary.to_string());
    fields.insert("exp".into(), job.exp.to_string());
    fields.insert("edu".into(), job.edu.to_string());
    fields.insert("province_id".into(), job.provinceid.to_string());
    fields.insert("city_id".into(), job.cityid.to_string());

    let _ = tpl_repo::incr_num(state.db.pool(), tpl.id).await;

    Ok(PosterSpec {
        template: PosterTemplateView::from(tpl),
        qr_scene: format!("weixin_jobid_{job_id}"),
        fields,
    })
}

// ==================== Company poster ====================

pub async fn company_poster_spec(
    state: &AppState,
    hb: Option<u64>,
    com_uid: u64,
) -> AppResult<PosterSpec> {
    let tpl = pick_template(state, PosterKind::Company, hb).await?;

    let (name, logo) = fetch_company_brief(state, com_uid).await?;

    let mut fields = BTreeMap::new();
    fields.insert("company_name".into(), name);
    fields.insert("company_logo".into(), logo);

    let _ = tpl_repo::incr_num(state.db.pool(), tpl.id).await;

    Ok(PosterSpec {
        template: PosterTemplateView::from(tpl),
        qr_scene: format!("weixin_companyid_{com_uid}"),
        fields,
    })
}

// ==================== Invite-to-register poster ====================

pub async fn invite_reg_poster_spec(
    state: &AppState,
    hb: Option<u64>,
    inviter_uid: u64,
) -> AppResult<PosterSpec> {
    let tpl = pick_template(state, PosterKind::InviteReg, hb).await?;

    let mut fields = BTreeMap::new();
    fields.insert("inviter_uid".into(), inviter_uid.to_string());
    fields.insert(
        "reg_url".into(),
        state
            .config
            .web_base_url
            .as_deref()
            .map(|b| format!("{b}/register?uid={inviter_uid}"))
            .unwrap_or_default(),
    );

    let _ = tpl_repo::incr_num(state.db.pool(), tpl.id).await;

    Ok(PosterSpec {
        template: PosterTemplateView::from(tpl),
        qr_scene: format!("weixin_ruid_{inviter_uid}"),
        fields,
    })
}

// ==================== Public hiring (gongzhao) poster ====================

pub async fn gongzhao_poster_spec(
    state: &AppState,
    hb: Option<u64>,
    gongzhao_id: u64,
) -> AppResult<PosterSpec> {
    let tpl = pick_template(state, PosterKind::Gongzhao, hb).await?;

    let mut fields = BTreeMap::new();
    fields.insert("gongzhao_id".into(), gongzhao_id.to_string());

    let _ = tpl_repo::incr_num(state.db.pool(), tpl.id).await;

    Ok(PosterSpec {
        template: PosterTemplateView::from(tpl),
        qr_scene: format!("weixin_gongzhaoid_{gongzhao_id}"),
        fields,
    })
}

// ========== helpers ==========

async fn fetch_company_brief(
    state: &AppState,
    com_uid: u64,
) -> AppResult<(String, String)> {
    let row: Option<(Option<String>, Option<String>)> = sqlx::query_as( // TODO(arch): inline sqlx pending repo lift
        "SELECT name, logo FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(com_uid)
    .fetch_optional(state.db.reader())
    .await?;
    let (name, logo) = row
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("company_not_found".into())))?;
    Ok((name.unwrap_or_default(), logo.unwrap_or_default()))
}
