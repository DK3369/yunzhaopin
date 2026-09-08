//! Admin report queues — PHP `yunying/report_job`, `report_resume`,
//! `report_ask`, `report_advise`.
//!
//! All four are the same `phpyun_report` table split by `type` + `usertype`,
//! so the queue selector lives in [`ReportQueue`] and everything below keys off
//! it. PHP decorates each row with names and phone numbers looked up from
//! `member` / `company_job` / `resume` / `resume_expect` / `company` /
//! `question`, and the Vue grids bind those PHP names directly, so the payload
//! here keeps PHP's field names rather than inventing new ones.
//!
//! Column direction, which is easy to invert: `p_uid` is the reporter and
//! `c_uid` the reported party. The 简历举报 refund therefore pays `p_uid` —
//! the employer who paid to download a resume that turned out to be bogus.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::i18n;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::integral_transfer::repo as ledger_repo;
use phpyun_models::qna::repo as qna_repo;
use phpyun_models::report::entity::{AdminReportRow, ReportRefundRow};
use phpyun_models::report::refund;
use phpyun_models::report::repo::{self as report_repo, AdminReportFilter, ReportQueue};
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

/// PHP `pay_remark` keys for the two refund kinds, stored raw so the member's
/// billing history renders them in the reader's own language.
const REMARK_CASH_REFUND: &str = "admin_01423";
const REMARK_POINTS_REFUND: &str = "admin_01424";

/// The 问答举报 grid prints `is_del` and `reason` straight into the cell with
/// no `lc()` around them, so — like PHP's `yun_at()` — these have to come back
/// already translated for the caller's language.
fn tr(key: &str) -> String {
    i18n::t(&format!("messages.{key}"), i18n::current_lang())
}

/// What the admin asked us to give back, per PHP's `datafh` radio: `1` = 返还,
/// anything else = don't. PHP stores the raw value either way.
fn wants_refund(datafh: Option<i32>) -> bool {
    datafh == Some(1)
}

// ---------- 列表 ----------

#[derive(Debug, Default)]
pub struct ListQuery {
    /// PHP `ftype` — which column the keyword searches; meaning depends on the
    /// queue (see [`resolve_keyword`]).
    pub ftype: i32,
    pub keyword: String,
    pub status: Option<i32>,
    /// PHP grid sort: `t` = column, `order` = direction.
    pub order_by: Option<String>,
    pub order: Option<String>,
}

/// Turn the keyword into the narrowed filter PHP builds. Each queue searches a
/// different set of columns, and PHP pre-resolves the ones that live in other
/// tables into an `IN (...)` over `report`.
async fn resolve_keyword<'a>(
    state: &AppState,
    queue: ReportQueue,
    q: &'a ListQuery,
    f: &mut AdminReportFilter<'a>,
) -> AppResult<()> {
    let kw = q.keyword.trim();
    if kw.is_empty() {
        return Ok(());
    }
    let db = state.db.reader();
    match queue {
        // ftype 1=职位名 2=企业名 3=个人姓名
        ReportQueue::Job => match q.ftype {
            1 => {
                let (eids, _) = report_repo::admin_queue_targets(db, queue).await?;
                f.eids = Some(report_repo::job_ids_matching(db, &eids, kw).await?);
            }
            2 => f.r_name = Some(kw),
            3 => {
                let (_, uids) = report_repo::admin_queue_targets(db, queue).await?;
                f.p_uids = Some(report_repo::resume_uids_matching(db, &uids, kw).await?);
            }
            _ => {}
        },
        // ftype 1=简历名 2=个人姓名 3=企业名
        ReportQueue::Resume => match q.ftype {
            1 => {
                let (eids, _) = report_repo::admin_queue_targets(db, queue).await?;
                f.eids = Some(report_repo::expect_ids_matching(db, &eids, kw).await?);
            }
            2 => f.r_name = Some(kw),
            3 => {
                let (_, uids) = report_repo::admin_queue_targets(db, queue).await?;
                f.p_uids = Some(report_repo::company_uids_matching(db, &uids, kw).await?);
            }
            _ => {}
        },
        // ftype 1=被举报方名称，其余=举报人账号
        ReportQueue::Ask | ReportQueue::Advise => {
            if q.ftype == 1 {
                f.r_name = Some(kw);
            } else {
                f.username = Some(kw);
            }
        }
    }
    Ok(())
}

fn lookup<'a>(pairs: &'a [(u64, String)], id: u64) -> Option<&'a str> {
    pairs
        .iter()
        .find(|(k, _)| *k == id)
        .map(|(_, v)| v.as_str())
}

/// Add the per-queue display fields PHP's `getReportList` computes.
async fn decorate(
    state: &AppState,
    queue: ReportQueue,
    rows: &[AdminReportRow],
) -> AppResult<Vec<Value>> {
    let db = state.db.reader();

    let mut uids: Vec<u64> = rows.iter().flat_map(|r| [r.p_uid, r.c_uid]).collect();
    uids.retain(|v| *v > 0);
    uids.sort_unstable();
    uids.dedup();
    let mut eids: Vec<u64> = rows.iter().map(|r| r.eid).filter(|v| *v > 0).collect();
    eids.sort_unstable();
    eids.dedup();
    let mut p_uids: Vec<u64> = rows.iter().map(|r| r.p_uid).filter(|v| *v > 0).collect();
    p_uids.sort_unstable();
    p_uids.dedup();

    let mobiles = report_repo::member_mobiles(db, &uids).await?;

    // Only the queue's own lookups run; PHP branches the same way.
    let (targets, reporters) = match queue {
        ReportQueue::Job => (
            report_repo::job_names(db, &eids).await?,
            report_repo::resume_names(db, &p_uids).await?,
        ),
        ReportQueue::Resume => (
            report_repo::expect_names(db, &eids).await?,
            report_repo::company_names(db, &p_uids).await?,
        ),
        ReportQueue::Ask => (report_repo::question_titles(db, &eids).await?, Vec::new()),
        ReportQueue::Advise => (Vec::new(), Vec::new()),
    };
    let reasons = if queue == ReportQueue::Ask {
        report_repo::list_reasons(db).await?
    } else {
        Vec::new()
    };

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let mut v = serde_json::to_value(r).unwrap_or(json!({}));
        // The entity renames `type` to `kind` for Rust's sake; the grid wants
        // PHP's spelling back.
        v["type"] = json!(r.kind);
        v["inputtime_n"] = json!(fmt_dt(r.inputtime));
        v["rtime_n"] = json!(fmt_dt(r.rtime));
        v["c_mobile"] = json!(lookup(&mobiles, r.c_uid).unwrap_or_default());
        v["p_mobile"] = json!(lookup(&mobiles, r.p_uid).unwrap_or_default());

        match queue {
            ReportQueue::Job | ReportQueue::Resume => {
                let name = lookup(&targets, r.eid).unwrap_or_default();
                v["name"] = json!(name);
                v["p_name"] = json!(lookup(&reporters, r.p_uid).unwrap_or_default());
                // PHP only offers the preview link once the target still
                // resolves, so a deleted job/resume renders as plain text.
                v["url"] = json!(if name.is_empty() {
                    String::new()
                } else if queue == ReportQueue::Job {
                    format!("/job/comapply/{}?look=admin", r.eid)
                } else {
                    format!("/resume/show/{}?look=admin", r.eid)
                });
            }
            ReportQueue::Ask => {
                let title = lookup(&targets, r.eid).unwrap_or_default();
                v["title"] = json!(title);
                v["c"] = json!("add");
                // PHP marks a report whose question is already gone with
                // `is_del`, and clears it when the question still exists.
                v["is_del"] = json!(if title.is_empty() {
                    tr("common_01552")
                } else {
                    String::new()
                });
                v["url"] = json!(if title.is_empty() {
                    String::new()
                } else {
                    format!("/ask/{}", r.eid)
                });
                // `r_reason` on 问答举报 is a `phpyun_reason` id, not free text.
                // PHP reserves "原因已被删除" for the literal id `0`; an id that
                // simply no longer resolves leaves the cell blank rather than
                // claiming the reason was deleted.
                let id = r.r_reason.trim().parse::<u64>().ok();
                v["reason"] = json!(match id {
                    Some(0) if !reasons.is_empty() => tr("common_01477"),
                    Some(id) => reasons
                        .iter()
                        .find(|x| x.id == id)
                        .map(|x| x.name.clone())
                        .unwrap_or_default(),
                    None => String::new(),
                });
            }
            ReportQueue::Advise => {}
        }
        out.push(v);
    }
    Ok(out)
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    queue: ReportQueue,
    q: &ListQuery,
    page: Pagination,
) -> AppResult<Value> {
    user.require_admin()?;
    let mut f = AdminReportFilter {
        status: q.status,
        order_by: q.order_by.as_deref(),
        order_desc: q
            .order
            .as_deref()
            .map(|o| o.eq_ignore_ascii_case("desc") || o.eq_ignore_ascii_case("descending"))
            .unwrap_or(false),
        ..Default::default()
    };
    resolve_keyword(state, queue, q, &mut f).await?;

    let db = state.db.reader();
    let total = report_repo::admin_count(db, queue, &f).await?;
    let list = if total > 0 {
        let rows = report_repo::admin_list(db, queue, &f, page.limit, page.offset).await?;
        decorate(state, queue, &rows).await?
    } else {
        Vec::new()
    };
    Ok(json!({
        "list": list,
        "total": total,
        "page": page.page,
        "limit": page.page_size,
        "page_sizes": [10, 20, 50, 100],
    }))
}

// PHP `report_resume::index_base_data` already has a home in
// `admin_php_page_service` under the `report_resume_base` page key, so it is
// not repeated here.

// ---------- 处理与删除 ----------

/// PHP `saveresult` for the three queues with no refund: stamp the result and
/// mark handled.
pub async fn save_result(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    result: &str,
) -> AppResult<()> {
    user.require_admin()?;
    if id == 0 {
        return Err(ApiError::param_invalid("pid"));
    }
    let n = report_repo::save_result(
        state.db.pool(),
        &[id],
        result,
        user.uid,
        clock::now_ts(),
        None,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_01266"));
    }
    audit_write(state, user, "admin.report.saveresult", format!("report:{id}")).await;
    Ok(())
}

/// PHP `del_action`. `report_resume` additionally accepts `type = 'pldel'`,
/// which drops every report filed against the same resume rather than just the
/// row that was clicked.
///
/// One divergence: PHP widens by `eid` alone, which would also sweep up a
/// 问答举报 whose question id happened to equal the resume id. We keep the
/// widening inside the queue that asked for it.
pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    widen_to_same_target: bool,
) -> AppResult<String> {
    user.require_admin()?;
    let mut ids: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("del"));
    }
    let pool = state.db.pool();
    if widen_to_same_target {
        if let Some(one) = report_repo::find_refund_row(pool, ids[0]).await? {
            let siblings = report_repo::resume_reports_of(pool, one.eid).await?;
            for id in siblings {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
    }
    let n = report_repo::delete_by_ids(pool, &ids).await?;
    if n == 0 {
        return Err(ApiError::business("admin_user_00186"));
    }
    audit_write(
        state,
        user,
        "admin.report.delete",
        format!("report:{}", join_ids(&ids)),
    )
    .await;
    // PHP `delReport` composes "举报(ID:1,2)删除成功" from three pieces.
    Ok(format!(
        "{}(ID:{}){}",
        tr("wap_com_00350"),
        join_ids(&ids),
        tr("admin_user_00187")
    ))
}

fn join_ids(ids: &[u64]) -> String {
    ids.iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

// ---------- 简历举报：返还 ----------

/// What one upheld resume report owes its reporter. PHP calls this `fhtype`:
/// 1 = quota only, 2 = cash, 3 = points, 4 = both.
///
/// The quota branch is mutually exclusive with the other two here, which is
/// what PHP's single-report and 批量 paths do. Its 同步处理 path disagrees, but
/// only because of a typo — it collects points payers into `$noUid` while
/// testing `$notUid`, so a company that paid with points gets its points back
/// *and* a quota credit. We don't reproduce that.
#[derive(Debug, Default)]
struct Owed {
    cash: Option<f64>,
    points: Option<i64>,
    /// No payment record, but the resume was downloaded — give the quota back.
    quota: bool,
}

/// Work out what to give back for one report, without writing anything.
async fn owed_for(state: &AppState, r: &ReportRefundRow) -> AppResult<Owed> {
    let db = state.db.reader();
    // `p_uid` paid, `c_uid` owns the resume.
    if !refund::downloaded(db, r.eid, r.c_uid, r.p_uid).await? {
        return Ok(Owed::default());
    }
    let cash = refund::cash_paid(db, r.p_uid, r.eid).await?;
    let points = refund::points_paid(db, r.p_uid, r.eid).await?;
    Ok(Owed {
        quota: cash.is_none() && points.is_none(),
        cash,
        points,
    })
}

/// Pay one reporter back and append the matching ledger rows.
async fn pay_back(state: &AppState, r: &ReportRefundRow, owed: &Owed) -> AppResult<bool> {
    let pool = state.db.pool();
    let now = clock::now_ts();
    let mut paid = false;

    if let Some(amount) = owed.cash {
        statis_repo::add_packpay(pool, r.p_uid, amount).await?;
        let did = refund::member_did(pool, r.p_uid).await?;
        ledger_repo::insert_refund_row(
            pool,
            &ledger_repo::RefundLedgerRow {
                uid: r.p_uid,
                did,
                eid: r.eid,
                amount,
                remark: REMARK_CASH_REFUND,
                kind: ledger_repo::LEDGER_KIND_PACKPAY,
                usertype: 2,
            },
            now,
        )
        .await?;
        paid = true;
    }
    if let Some(points) = owed.points {
        statis_repo::add_integral(pool, r.p_uid, points).await?;
        let did = refund::member_did(pool, r.p_uid).await?;
        ledger_repo::insert_refund_row(
            pool,
            &ledger_repo::RefundLedgerRow {
                uid: r.p_uid,
                did,
                eid: r.eid,
                amount: points as f64,
                remark: REMARK_POINTS_REFUND,
                kind: ledger_repo::LEDGER_KIND_INTEGRAL,
                usertype: 2,
            },
            now,
        )
        .await?;
        paid = true;
    }
    if owed.quota {
        statis_repo::refund_down_resume(pool, r.p_uid).await?;
        paid = true;
    }
    Ok(paid)
}

/// Refund every report in `rows` that hasn't been refunded yet. Returns the
/// employer uids that actually received something, which PHP names in its
/// admin log.
pub async fn refund_unpaid_resume_reports(
    state: &AppState,
    rows: &[ReportRefundRow],
) -> AppResult<Vec<u64>> {
    refund_rows(state, rows).await
}

async fn refund_rows(
    state: &AppState,
    rows: &[ReportRefundRow],
) -> AppResult<Vec<u64>> {
    let mut credited = Vec::new();
    for r in rows {
        // PHP guards on `datafh != 1` so a second 返还 can't double-pay.
        if r.datafh == 1 {
            continue;
        }
        let owed = owed_for(state, r).await?;
        if pay_back(state, r, &owed).await? && !credited.contains(&r.p_uid) {
            credited.push(r.p_uid);
        }
    }
    Ok(credited)
}

#[derive(Debug, Default)]
pub struct ResumeResultForm {
    pub pid: u64,
    pub result: String,
    /// PHP radio: `1` = 返还, `2` = 不返还.
    pub datafh: Option<i32>,
    /// PHP `tongbu = 1` — also handle every other pending report on the same
    /// resume.
    pub tongbu: bool,
}

/// PHP `report_resume::saveresult`.
pub async fn save_result_resume(
    state: &AppState,
    user: &AuthenticatedUser,
    f: &ResumeResultForm,
) -> AppResult<()> {
    user.require_admin()?;
    if f.pid == 0 {
        return Err(ApiError::param_invalid("pid"));
    }
    let pool = state.db.pool();
    let Some(base) = report_repo::find_refund_row(pool, f.pid).await? else {
        return Err(ApiError::business("common_01266"));
    };

    // 同步处理 widens the target set to every pending report on this resume.
    let rows = if f.tongbu {
        let mut pending = report_repo::pending_resume_reports(pool, base.eid).await?;
        if !pending.iter().any(|r| r.id == base.id) {
            pending.push(base.clone());
        }
        pending
    } else {
        vec![base.clone()]
    };

    let credited = if wants_refund(f.datafh) {
        refund_rows(state, &rows).await?
    } else {
        Vec::new()
    };

    let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
    let n = report_repo::save_result(
        pool,
        &ids,
        &f.result,
        user.uid,
        clock::now_ts(),
        f.datafh,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("common_01266"));
    }
    audit_write(
        state,
        user,
        "admin.report.resume.saveresult",
        format!(
            "report:{} refunded:{}",
            join_ids(&ids),
            join_ids(&credited)
        ),
    )
    .await;
    Ok(())
}

/// PHP `report_resume::saveresultall` — the same thing over a checkbox batch.
pub async fn save_result_resume_all(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    result: &str,
    datafh: Option<i32>,
) -> AppResult<()> {
    user.require_admin()?;
    let ids: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("rid"));
    }
    let pool = state.db.pool();
    let credited = if wants_refund(datafh) {
        let rows = report_repo::find_refund_rows(pool, &ids).await?;
        refund_rows(state, &rows).await?
    } else {
        Vec::new()
    };
    let n =
        report_repo::save_result(pool, &ids, result, user.uid, clock::now_ts(), datafh).await?;
    if n == 0 {
        return Err(ApiError::business("common_01266"));
    }
    audit_write(
        state,
        user,
        "admin.report.resume.saveresult_batch",
        format!(
            "report:{} refunded:{}",
            join_ids(&ids),
            join_ids(&credited)
        ),
    )
    .await;
    Ok(())
}

// ---------- 问答举报 ----------

/// PHP `report_ask::delquestion`. PHP hard-deletes the question along with its
/// answers; we keep this codebase's soft delete for `phpyun_question` (what
/// `/v1/admin/questions/delete` already does) and clear the answers, which
/// have no soft-delete column of their own.
pub async fn delete_questions(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<String> {
    user.require_admin()?;
    let ids: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("del"));
    }
    let pool = state.db.pool();
    qna_repo::admin_delete_questions(pool, &ids).await?;
    qna_repo::admin_delete_answers_of(pool, &ids).await?;
    audit_write(
        state,
        user,
        "admin.report.ask.delete_question",
        format!("question:{}", join_ids(&ids)),
    )
    .await;
    Ok(i18n::t_args(
        "messages.admin_model_00009",
        i18n::current_lang(),
        &[("ids", &join_ids(&ids))],
    ))
}

/// Q&A categories are two levels deep; the editor picks a parent then a child.
fn qclass_json(c: &phpyun_models::qna::entity::QClass) -> Value {
    json!({ "id": c.id, "name": c.name, "pid": c.pid })
}

/// PHP `report_ask::getclass` — the children of one parent category.
pub async fn ask_classes(state: &AppState, user: &AuthenticatedUser, pid: i32) -> AppResult<Value> {
    user.require_admin()?;
    let all = qna_repo::list_qclasses(state.db.reader()).await?;
    let class: Vec<Value> = all
        .iter()
        .filter(|c| c.pid == pid)
        .map(qclass_json)
        .collect();
    Ok(json!({ "class": class }))
}

/// PHP `report_ask::edit` — the reported question plus the category pickers.
/// Returns the top-level categories, and the parent of the question's own
/// category so the editor can preselect both dropdowns.
pub async fn ask_edit(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<Value> {
    user.require_admin()?;
    let db = state.db.reader();
    let all = qna_repo::list_qclasses(db).await?;

    let mut info = json!({});
    let mut parent_pid = 0;
    if id > 0 {
        let Some(q) = qna_repo::find_question(db, id).await? else {
            return Err(ApiError::business("common_01266"));
        };
        // The Vue binds PHP's column names, and its switch reads `is_recom_n`
        // while its submit sends `is_recom`, so hand back both.
        info = json!({
            "id": q.id,
            "uid": q.uid,
            "title": q.title,
            "content": q.content,
            "cid": q.category_id,
            "visit": q.hits,
            "is_recom": q.is_recom,
            "is_recom_n": q.is_recom == 1,
            "state": q.status,
        });
        parent_pid = all
            .iter()
            .find(|c| c.id == q.category_id as u64)
            .map(|c| c.pid)
            .unwrap_or(0);
    }
    let class: Vec<Value> = all.iter().filter(|c| c.pid == 0).map(qclass_json).collect();
    Ok(json!({ "info": info, "class": class, "pid": parent_pid }))
}

#[derive(Debug, Default)]
pub struct AskSaveForm {
    pub id: u64,
    pub title: String,
    pub cid: i32,
    pub visit: u32,
    pub is_recom: i32,
    pub content: String,
}

/// PHP `report_ask::save` → `ask.model::upAskInfo`. PHP undoes one round of
/// entity escaping on the body first (`&amp;` → `&`), because the rich-text
/// editor posts already-escaped ampersands that would otherwise accumulate a
/// layer on every save.
pub async fn ask_save(
    state: &AppState,
    user: &AuthenticatedUser,
    f: &AskSaveForm,
) -> AppResult<()> {
    user.require_admin()?;
    if f.id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    if f.title.trim().is_empty() {
        return Err(ApiError::param_invalid("title"));
    }
    let content = f.content.replace("&amp;", "&");
    let n = qna_repo::admin_update_question(
        state.db.pool(),
        f.id,
        &qna_repo::AdminQuestionUpdate {
            title: f.title.trim(),
            cid: f.cid,
            visit: f.visit,
            is_recom: f.is_recom,
            content: &content,
        },
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("admin_01422"));
    }
    audit_write(
        state,
        user,
        "admin.report.ask.save",
        format!("question:{}", f.id),
    )
    .await;
    Ok(())
}
