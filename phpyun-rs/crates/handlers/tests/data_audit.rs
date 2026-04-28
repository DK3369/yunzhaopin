//! Diagnostic: for each "empty data" endpoint flagged by `endpoint_smoke`,
//! count rows in the underlying PHP table so we can tell whether the empty
//! response is legitimate (table really is empty in this DB) or a SQL bug
//! (table has rows but the Rust query misses them).

use phpyun_core::shutdown::CancellationToken;
use phpyun_core::{AppState, Config};

#[tokio::test(flavor = "multi_thread")]
async fn audit_underlying_table_counts() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("warn,sqlx=error")
        .with_test_writer()
        .try_init();
    let config = Config::load().expect("Config::load");
    let shutdown = CancellationToken::new();
    let state = AppState::build(config.clone(), shutdown.clone())
        .await
        .expect("AppState::build");
    let db = state.db.reader();

    // (endpoint, [tables to count])
    let probes: &[(&str, &[(&str, &str)])] = &[
        ("/v1/wap/articles",            &[("phpyun_news_base", "1=1")]),
        ("/v1/wap/companies",           &[("phpyun_company", "show_status = 1")]),
        ("/v1/wap/companies/hot",       &[("phpyun_company", "show_status = 1")]),
        ("/v1/wap/parts",               &[("phpyun_partjob", "1=1")]),
        ("/v1/wap/zph",                 &[("phpyun_zhaopinhui", "1=1")]),
        ("/v1/wap/specials",            &[("phpyun_special", "1=1")]),
        ("/v1/wap/gongzhao",            &[("phpyun_gongzhao", "1=1")]),
        ("/v1/wap/hr-docs",             &[("phpyun_toolbox_doc", "1=1")]),
        ("/v1/wap/redeem/rewards",      &[("phpyun_reward", "status = 1")]),
        ("/v1/wap/eval-papers",         &[("phpyun_evaluate", "1=1")]),
        ("/v1/wap/qna/hotweek",         &[("phpyun_question", "state = 1")]),
        ("/v1/wap/qna/top-answerers",   &[("phpyun_answer", "1=1")]),
        ("/v1/wap/questions",           &[("phpyun_question", "state = 1")]),
        ("/v1/wap/site/sub-sites",      &[("phpyun_domain", "1=1")]),
        ("/v1/wap/integral/items",      &[("phpyun_reward", "1=1")]),
        ("/v1/wap/countries",           &[("phpyun_country", "status != 2")]),
        ("/v1/wap/data-show/resume-edu",     &[("phpyun_resume", "edu > 0")]),
        ("/v1/wap/data-show/resume-exp",     &[("phpyun_resume", "exp > 0")]),
        ("/v1/wap/data-show/resume-sex",     &[("phpyun_resume", "sex IN (1,2)")]),
        ("/v1/wap/data-show/resume-city",    &[("phpyun_resume", "1=1")]),
        ("/v1/wap/data-show/company-city",   &[("phpyun_company", "1=1")]),
        ("/v1/wap/data-show/company-scale",  &[("phpyun_company", "1=1")]),
        ("/v1/wap/data-show/company-property", &[("phpyun_company", "1=1")]),
        ("/v1/mcenter/eval-logs",       &[("phpyun_examinee", "uid = 1")]),
        ("/v1/mcenter/my/questions",    &[("phpyun_question", "uid = 1")]),
        ("/v1/mcenter/my/answers",      &[("phpyun_answer", "uid = 1")]),
        ("/v1/mcenter/recommend/jobs",  &[("phpyun_company_job", "state = 1 AND status = 0 AND r_status = 1")]),
        ("/v1/mcenter/interviews",      &[("phpyun_yqmb", "uid = 1")]),
        ("/v1/mcenter/my-applications", &[("phpyun_userid_job", "uid = 1")]),
        ("/v1/admin/feedback",          &[("phpyun_advice_question", "1=1")]),
        ("/v1/admin/audit-log",         &[("phpyun_rs_audit_log", "1=1")]),
        ("/v1/admin/company-certs",     &[("phpyun_company_cert", "status = 1")]),
        ("/v1/mcenter/chat/conversations", &[("phpyun_rs_chat", "1=1")]),
        ("/v1/mcenter/sessions",        &[("phpyun_user_session", "uid = 1")]),
        ("/v1/mcenter/warnings",        &[("phpyun_warning", "uid = 1")]),
        ("/v1/mcenter/fans",            &[("phpyun_fav_job", "1=1")]),
        ("/v1/mcenter/followers",       &[("phpyun_atn", "1=1")]),
    ];

    println!("\n========== UNDERLYING TABLE-COUNT AUDIT ==========");
    println!("{:<48} {}", "endpoint", "row counts in source table(s)");
    for (path, tables) in probes {
        let mut summary: Vec<String> = Vec::new();
        for (t, cond) in *tables {
            let sql = format!("SELECT COUNT(*) FROM {t} WHERE {cond}");
            let result: Result<(i64,), sqlx::Error> =
                sqlx::query_as(&sql).fetch_one(db).await;
            let cell = match result {
                Ok((n,)) => format!("{t}({cond})={n}"),
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("doesn't exist") {
                        format!("{t}=⛔missing")
                    } else {
                        format!("{t}=ERR:{}", msg.chars().take(200).collect::<String>())
                    }
                }
            };
            summary.push(cell);
        }
        println!("{:<48} {}", path, summary.join(" | "));
    }
    println!("==================================================");
}
