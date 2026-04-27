//! Static reachability check for the post-audit new endpoints.
//!
//! Black-box: builds the v1 OpenAPI document (no DB/Redis) and asserts every
//! newly added path is registered with the expected method + body schema. If
//! someone deletes a `#[utoipa::path]` annotation, edits a route string, or
//! forgets to register a handler in `openapi.rs`, this test fails fast.
//!
//! End-to-end testing (real DB+Redis) is intentionally out of scope here —
//! see `crates/app/tests/` (when added) for that.

use phpyun_core::json;
use phpyun_handlers::openapi::V1Doc;
use utoipa::OpenApi;

fn doc() -> utoipa::openapi::OpenApi {
    V1Doc::openapi()
}

/// Helper: assert a path exists at the given method + tag.
fn assert_endpoint(method: &str, path: &str) {
    let openapi = doc();
    let item = openapi
        .paths
        .paths
        .get(path)
        .unwrap_or_else(|| panic!("path {path} not registered in V1Doc"));
    let op = match method {
        "GET" => item.get.as_ref(),
        "POST" => item.post.as_ref(),
        "PUT" => item.put.as_ref(),
        "DELETE" => item.delete.as_ref(),
        _ => panic!("unsupported method {method}"),
    };
    assert!(op.is_some(), "no {method} handler at {path}");
}

fn assert_schema(name: &str) {
    let openapi = doc();
    let components = openapi.components.expect("components should exist");
    assert!(
        components.schemas.contains_key(name),
        "schema {name} missing from V1Doc components"
    );
}

// ==================== WAP additions ====================

#[test]
fn usertype_select_registered() {
    // setutype: PHP wap/login::setutype_action → POST /v1/wap/usertype/select
    assert_endpoint("POST", "/v1/wap/usertype/select");
    assert_schema("SelectUsertypeForm");
    assert_schema("SelectUsertypeData");
}

#[test]
fn city_domain_registered() {
    // PHP wap/index::getCityDomain_action → GET /v1/wap/regions/city-domain
    assert_endpoint("POST", "/v1/wap/regions/city-domain");
    assert_schema("CityDomainResp");
}

#[test]
fn specials_apply_registered() {
    // PHP wap/special::apply_action → POST /v1/wap/specials/{id}/apply
    assert_endpoint("POST", "/v1/wap/specials/apply");
    assert_schema("ApplyResp");
}

#[test]
fn once_pay_registered() {
    // PHP wap/once::getOrder_action → POST /v1/wap/once-jobs/{id}/pay
    assert_endpoint("POST", "/v1/wap/once-jobs/pay");
    assert_schema("PayForm");
    assert_schema("PayCreated");
}

// ==================== Mcenter additions ====================

#[test]
fn follow_routes_registered() {
    // PHP wap/ajax::atn_action / atncompany_action → /v1/mcenter/follows
    assert_endpoint("POST", "/v1/mcenter/follows");
    assert_endpoint("POST", "/v1/mcenter/follows");
    assert_endpoint("POST", "/v1/mcenter/follows/exists");
    assert_endpoint("POST", "/v1/mcenter/followers");
    assert_schema("FollowToggleForm");
    assert_schema("FollowItem");
    assert_schema("ExistsResp");
}

#[test]
fn unread_summary_registered() {
    // PHP wap/ajax::msgNum_action → GET /v1/mcenter/messages/unread-summary
    assert_endpoint("POST", "/v1/mcenter/messages/unread-summary");
    assert_schema("UnreadSummary");
}

#[test]
fn once_orders_registered() {
    // PHP wap/once::paylog_action → GET /v1/mcenter/once-jobs/orders
    assert_endpoint("POST", "/v1/mcenter/once-jobs/orders");
    // PHP wap/once::delpaylog_action → POST /v1/mcenter/once-jobs/orders/{id}/cancel
    assert_endpoint("POST", "/v1/mcenter/once-jobs/orders/cancel");
    assert_schema("OrderItem");
}

#[test]
fn year_report_registered() {
    // PHP wap/ajax::lastYearReport_action → GET /v1/mcenter/dashboard/year-report
    assert_endpoint("POST", "/v1/mcenter/dashboard/year-report");
    assert_schema("YearReportView");
}

// ==================== Sanity: total path count grew ====================

#[test]
fn v1_doc_path_count_floor() {
    // Floor — catches accidental mass-deregistration. The actual count is ~287
    // at the time of writing; bump this when more endpoints are added but
    // never raise it above the live count to keep CI green.
    let n = doc().paths.paths.len();
    assert!(
        n >= 280,
        "expected V1Doc to register at least 280 paths, got {n}"
    );
}

// ==================== Methods are unique on shared paths ====================

#[test]
#[test]
fn follows_routes_post_only() {
    let openapi = doc();
    let item = openapi.paths.paths.get("/v1/mcenter/follows").expect("/follows");
    assert!(item.post.is_some(), "POST /follows should exist");
    assert!(item.get.is_none(), "GET /follows should be removed (all POST)");
    let listit = openapi.paths.paths.get("/v1/mcenter/follows/list").expect("/follows/list");
    assert!(listit.post.is_some(), "POST /follows/list should exist");
}

// ==================== Schema body sanity ====================
//
// Spot-check that the schema body for new endpoints actually serialises:
// catches the case where a `#[derive(ToSchema)]` field is renamed but the
// OpenAPI registration still points at the old name.

#[test]
fn select_usertype_form_has_usertype_field() {
    let openapi = doc();
    let comp = openapi.components.expect("components");
    let s = comp
        .schemas
        .get("SelectUsertypeForm")
        .expect("SelectUsertypeForm schema");
    let v = json::to_value(s).expect("schema serialises");
    let blob = v.to_string();
    assert!(
        blob.contains("usertype"),
        "SelectUsertypeForm schema must mention `usertype` (got {blob})"
    );
}

#[test]
fn unread_summary_has_total_field() {
    let openapi = doc();
    let comp = openapi.components.expect("components");
    let s = comp
        .schemas
        .get("UnreadSummary")
        .expect("UnreadSummary schema");
    let blob = json::to_value(s).expect("ok").to_string();
    for field in ["messages", "chat", "broadcasts", "warnings", "total"] {
        assert!(
            blob.contains(field),
            "UnreadSummary missing field `{field}` (got {blob})"
        );
    }
}

#[test]
fn year_report_view_has_login_days() {
    let openapi = doc();
    let comp = openapi.components.expect("components");
    let s = comp
        .schemas
        .get("YearReportView")
        .expect("YearReportView schema");
    let blob = json::to_value(s).expect("ok").to_string();
    for field in [
        "login_days",
        "job_count",
        "view_count",
        "received_resumes",
        "company_name",
        "linkman",
    ] {
        assert!(
            blob.contains(field),
            "YearReportView missing field `{field}` (got {blob})"
        );
    }
}

#[test]
fn follow_item_has_sc_uid() {
    let openapi = doc();
    let comp = openapi.components.expect("components");
    let s = comp.schemas.get("FollowItem").expect("FollowItem schema");
    let blob = json::to_value(s).expect("ok").to_string();
    for field in ["uid", "sc_uid", "sc_usertype", "time"] {
        assert!(
            blob.contains(field),
            "FollowItem missing field `{field}` (got {blob})"
        );
    }
}

#[test]
fn pay_form_has_paytype_and_gear() {
    let openapi = doc();
    let comp = openapi.components.expect("components");
    let s = comp.schemas.get("PayForm").expect("PayForm schema");
    let blob = json::to_value(s).expect("ok").to_string();
    for field in ["password", "paytype", "oncepricegear"] {
        assert!(
            blob.contains(field),
            "PayForm missing field `{field}` (got {blob})"
        );
    }
}

// ==================== Round 3: job-page Q&A + non-id-token OAuth ====================

#[test]
fn job_messages_routes_registered() {
    // PHP wap/ajax::pl_action → POST /v1/wap/jobs/{id}/messages
    assert_endpoint("POST", "/v1/wap/jobs/messages");
    assert_endpoint("POST", "/v1/wap/jobs/messages");
    assert_endpoint("POST", "/v1/wap/jobs/messages/hide");
    assert_endpoint("POST", "/v1/mcenter/job-messages");
    assert_endpoint("POST", "/v1/mcenter/job-messages/reply");
    assert_endpoint("POST", "/v1/mcenter/job-messages/hide");
    assert_schema("CreateMessageForm");
    assert_schema("JobMsgView");
    assert_schema("EmployerMsgItem");
}

#[test]
fn qq_oauth_routes_registered() {
    assert_endpoint("POST", "/v1/wap/oauth/qq/authorize-url");
    assert_endpoint("POST", "/v1/wap/oauth/qq/code-login");
    assert_schema("QqAuthorizeData");
    assert_schema("CodeLoginForm");
}

#[test]
fn weibo_oauth_routes_registered() {
    assert_endpoint("POST", "/v1/wap/oauth/weibo/authorize-url");
    assert_endpoint("POST", "/v1/wap/oauth/weibo/code-login");
    assert_schema("WeiboAuthorizeData");
}

// ==================== Round 4: email forgetpw + manual appeal ====================

#[test]
fn email_forgetpw_routes_registered() {
    // PHP forgetpw/index::sendCode_action (sendtype=email) → POST /v1/wap/forgetpw/send-email
    assert_endpoint("POST", "/v1/wap/forgetpw/send-email");
    // Plus reset using the emailed code
    assert_endpoint("POST", "/v1/wap/forgetpw/reset-by-email");
    assert_schema("SendEmailForm");
    assert_schema("ResetByEmailForm");
}

#[test]
fn appeal_route_registered() {
    // PHP forgetpw/index::checklink_action → POST /v1/wap/forgetpw/appeal
    assert_endpoint("POST", "/v1/wap/forgetpw/appeal");
    assert_schema("AppealForm");
    assert_schema("AppealResponse");
}

// ==================== Round 5: hot-companies / zph com-status / share-text / recommend ====================

#[test]
fn hot_companies_route_registered() {
    // PHP wap/index::getmq_action → GET /v1/wap/companies/hot
    assert_endpoint("POST", "/v1/wap/companies/hot");
    assert_schema("HotCompanyView");
}

#[test]
fn zph_com_status_route_registered() {
    // PHP wap/ajax::ajaxComjob_action → GET /v1/mcenter/zph/{id}/com-status
    assert_endpoint("POST", "/v1/mcenter/zph/com-status");
    assert_schema("ComStatusView");
    assert_schema("OwnJobBrief");
}

#[test]
fn job_share_text_route_registered() {
    // PHP wap/job::getJobWb_action → GET /v1/wap/jobs/{id}/share-text
    assert_endpoint("POST", "/v1/wap/jobs/share-text");
    assert_schema("JobShareText");
}

#[test]
fn email_recommend_route_registered() {
    // PHP wap/resume/resumeshare::index_action → POST /v1/mcenter/recommend/email/{kind}/{id}
    assert_endpoint("POST", "/v1/mcenter/recommend/email");
    assert_schema("EmailRecommendForm");
    assert_schema("EmailRecommendResp");
}

// ==================== Round 6: Q&A leaderboard ====================

#[test]
fn qna_top_answerers_route_registered() {
    // PHP ask::getAnswersList(groupby:uid) → GET /v1/wap/qna/top-answerers
    assert_endpoint("POST", "/v1/wap/qna/top-answerers");
    assert_schema("TopAnswererItem");
}

// ==================== Round 12: recommended categories ====================

#[test]
fn recommended_categories_route_registered() {
    // PHP wap/special::hotjobclass_action / category::getHotJobClass(rec=1)
    //   → GET /v1/wap/categories/{kind}/recommended
    assert_endpoint("POST", "/v1/wap/categories/recommended");
}

// ==================== Admin auth invariants ====================
//
// Every `/v1/admin/*` route MUST declare `security(("bearer" = []))` in its
// `#[utoipa::path]` so OpenAPI consumers (and CI) can see at a glance that
// the route is admin-gated. Combined with `phpyun_core::admin_guard::layer`
// at the router boundary and per-handler `user.require_admin()` calls, this
// is the third leg of the defense-in-depth stack — a missing security
// marker fails the test even if the runtime guards still hold.

#[test]
fn every_admin_route_declares_bearer_security() {
    let openapi = doc();
    let mut missing: Vec<String> = Vec::new();

    for (path, item) in openapi.paths.paths.iter() {
        if !path.starts_with("/v1/admin/") {
            continue;
        }
        for (method, op) in [
            ("GET", item.get.as_ref()),
            ("POST", item.post.as_ref()),
            ("PUT", item.put.as_ref()),
            ("PATCH", item.patch.as_ref()),
            ("DELETE", item.delete.as_ref()),
        ] {
            let Some(op) = op else { continue };
            let secured = op
                .security
                .as_ref()
                .map(|sec| {
                    sec.iter().any(|req| {
                        // utoipa serialises SecurityRequirement as a map of
                        // scheme-name → required scopes; we only need to see
                        // the `bearer` key — the scope list is empty for our
                        // bearer scheme.
                        let v = json::to_value(req).unwrap_or_default();
                        v.as_object()
                            .map(|m| m.contains_key("bearer"))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if !secured {
                missing.push(format!("{method} {path}"));
            }
        }
    }

    assert!(
        missing.is_empty(),
        "admin routes missing `security((\"bearer\" = []))`:\n  {}",
        missing.join("\n  ")
    );
}

// ==================== Validation invariant: Path<String> handlers must validate ====================
//
// `Path<String>` (or `Path<(String, _)>`) feeds the captured segment into
// service / DB layers — sqlx parameter binding rules out injection but not
// memory exhaustion or oversized identifiers. Every such handler must call
// one of `phpyun_core::validators::ensure_path_token / ensure_path_key /
// ensure_path_hex_token` on the String binding before any service call.
//
// This test scans handler files: any function signature with `Path<String>`
// (or tuple form) whose body doesn't reference the matching ensure_path_*
// helper for that binding fails.

const PATH_VALIDATION_ALLOWLIST: &[&str] = &[
    // (none currently)
];

#[test]
fn path_string_handlers_validate_input() {
    use std::path::PathBuf;
    let v1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/v1");
    let mut violations: Vec<String> = Vec::new();

    walk(&v1, &mut |path, src| {
        if PATH_VALIDATION_ALLOWLIST
            .iter()
            .any(|allow| path.to_string_lossy().ends_with(allow))
        {
            return;
        }
        // For each handler function, find the Path<String> bindings and
        // confirm an ensure_path_* call references each name.
        for cap in regex_iter(
            r"pub\s+async\s+fn\s+(\w+)\s*\(([^)]*)\)\s*[^{]*\{([^}]*?)\n\s*\w",
            src,
        ) {
            let _name = &cap[1];
            let sig = &cap[2];
            let body_head = &cap[3];

            let bindings = path_string_bindings(sig);
            for var in bindings {
                let pattern = format!("ensure_path_token(&{var})");
                let key_pattern = format!("ensure_path_key(&{var})");
                let hex_pattern = format!("ensure_path_hex_token(&{var})");
                if !body_head.contains(&pattern)
                    && !body_head.contains(&key_pattern)
                    && !body_head.contains(&hex_pattern)
                {
                    violations.push(format!(
                        "{}: handler `{_name}` has Path<String> binding `{var}` but body does not call ensure_path_*",
                        path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                            .unwrap_or(path)
                            .display(),
                    ));
                }
            }
        }
    });
    assert!(
        violations.is_empty(),
        "Path<String> handlers missing input validation:\n  {}",
        violations.join("\n  ")
    );
}

/// Pull String-typed bindings out of a function signature.
fn path_string_bindings(sig: &str) -> Vec<String> {
    let mut out = Vec::new();
    // Single Path<String>:  Path(name): Path<String>
    let single = regex::Regex::new(r"Path\((\w+)\):\s*Path<\s*String\s*>").unwrap();
    for m in single.captures_iter(sig) {
        out.push(m[1].to_string());
    }
    // Tuple Path<(String, T)> / Path<(String, T, U)>:  Path((a, b, c)): Path<(String, ...)>
    let tup = regex::Regex::new(
        r"Path\(\(([^)]+)\)\):\s*Path<\(\s*String([^>]*)\)>",
    )
    .unwrap();
    for m in tup.captures_iter(sig) {
        let names: Vec<&str> = m[1].split(',').map(|s| s.trim()).collect();
        // Walk the type list; only keep names whose corresponding type is `String`
        let types: Vec<&str> = std::iter::once("String")
            .chain(m[2].trim_start_matches(',').split(','))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for (n, t) in names.iter().zip(types.iter()) {
            if *t == "String" {
                out.push(n.to_string());
            }
        }
    }
    out
}

fn regex_iter<'a>(pat: &str, src: &'a str) -> impl Iterator<Item = regex::Captures<'a>> {
    let re = regex::Regex::new(pat).unwrap();
    re.captures_iter(src).collect::<Vec<_>>().into_iter()
}

// ==================== Validation invariant: NO raw axum::Json in handlers ====================
//
// Body extractors must be `ValidatedJson<T>` (or `ValidatedForm<T>`) so the
// `validator::Validate` impl runs before any DB code touches the data. A raw
// `axum::Json<...>` in a handler is a regression — the handler can choose to
// run `value.validate()` manually, but a future maintainer might forget.
//
// The two legacy polymorphic delete-or-update spots in `wap/once.rs` and
// `wap/tiny.rs` have been split into dedicated routes (`/{id}/delete`) with
// `ValidatedJson<PasswordBody>` so this rule is now hold-the-line.

#[test]
fn no_raw_json_extractor_in_handlers() {
    use std::path::PathBuf;
    let v1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/v1");
    let mut violations: Vec<String> = Vec::new();
    walk(&v1, &mut |p, src| {
        for (lineno, line) in src.lines().enumerate() {
            let trim = line.trim_start();
            if trim.starts_with("//") {
                continue;
            }
            // `axum::Json(` (extractor) — NOT `axum::Json<` (return type) and
            // NOT `Json::<` turbofish.
            if line.contains("axum::Json(") && !line.contains("ApiJson") {
                violations.push(format!(
                    "{}:{}: {}",
                    p.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                        .unwrap_or(p)
                        .display(),
                    lineno + 1,
                    line.trim()
                ));
            }
        }
    });
    assert!(
        violations.is_empty(),
        "raw `axum::Json(...)` extractor — use `ValidatedJson<T>`:\n  {}",
        violations.join("\n  ")
    );
}

// ==================== Validation invariant: NO raw Query<T> in handlers ====================
//
// Every URL-query extractor across `/v1/wap`, `/v1/mcenter`, `/v1/admin` must
// be `ValidatedQuery<T>` (so `validator::Validate` runs before any DB code).
// `Query<T>` deserialises but does **not** validate, so a regression would
// let unbounded strings / out-of-range numbers reach SQL.
//
// The test scans source files — OpenAPI doesn't surface the extractor type.
// To opt out (e.g. proxy / pass-through endpoints with no DB touch), add the
// path to `EXTRACTOR_ALLOWLIST` below with a comment justifying the carve-out.

const EXTRACTOR_ALLOWLIST: &[&str] = &[
    // (none currently)
];

#[test]
fn no_raw_query_extractor_in_handlers() {
    use std::path::PathBuf;
    let v1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/v1");
    let mut violations: Vec<String> = Vec::new();
    walk(&v1, &mut |p, src| {
        if EXTRACTOR_ALLOWLIST
            .iter()
            .any(|allow| p.to_string_lossy().ends_with(allow))
        {
            return;
        }
        for (lineno, line) in src.lines().enumerate() {
            // Match `Query(...): Query<X>` or `axum::extract::Query(`.
            // Skip `ValidatedQuery` and comments.
            let trim = line.trim_start();
            if trim.starts_with("//") || trim.starts_with("///") {
                continue;
            }
            let bad = (line.contains("Query(") && line.contains(": Query<")
                && !line.contains("ValidatedQuery"))
                || (line.contains("axum::extract::Query(")
                    && !line.contains("ValidatedQuery"));
            if bad {
                violations.push(format!(
                    "{}:{}: {}",
                    p.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                        .unwrap_or(p)
                        .display(),
                    lineno + 1,
                    line.trim()
                ));
            }
        }
    });
    assert!(
        violations.is_empty(),
        "raw `Query<T>` extractor found — use `ValidatedQuery<T>` so validator::Validate runs:\n  {}",
        violations.join("\n  ")
    );
}

// ==================== Zero GET routes outside WeChat callback (Round 25) ====================
//
// `/v1/*` is POST-only; the WeChat callback (`wap/wechat.rs`) is the only
// allow-listed exception. This test scans every handler file for
// `#[utoipa::path(get, ...)]` annotations and `.route(...).get(...)` chains,
// and fails if anything matches outside the allow-list. Catches both single-
// line (`.route("/x", get(fn))`) and multi-line route blocks.

#[test]
fn no_get_routes_in_v1_outside_wechat() {
    use std::path::PathBuf;
    let v1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/v1");
    let mut violations: Vec<String> = Vec::new();
    walk(&v1, &mut |p, src| {
        let rel = p.to_string_lossy().to_string();
        if WECHAT_CALLBACK_FILES.iter().any(|w| rel.ends_with(w)) {
            return;
        }
        for m in regex::Regex::new(r"#\[utoipa::path\s*\(\s*get\b")
            .unwrap()
            .find_iter(src)
        {
            let lineno = src[..m.start()].matches('\n').count() + 1;
            violations.push(format!("{rel}:{lineno}: utoipa::path(get, ...)"));
        }
        // Multi-line tolerant: any `.route(...get(...))` chain.
        for m in regex::Regex::new(r"(?s)\.route\([^)]*?get\(")
            .unwrap()
            .find_iter(src)
        {
            let lineno = src[..m.start()].matches('\n').count() + 1;
            violations.push(format!("{rel}:{lineno}: .route(...).get(...) chain"));
        }
    });
    assert!(
        violations.is_empty(),
        "GET method is not allowed on /v1/* (POST-only). WeChat callback is allow-listed.\n  {}",
        violations.join("\n  ")
    );
}

// ==================== URL contains zero path params (Round 24) ====================
//
// All `/v1/*` endpoints take their parameters in the JSON body — never in
// the URL path. This test scans:
//   1. `#[utoipa::path(... path = "/v1/...{...}/...")]` annotations
//   2. `.route("/...{...}", ...)` calls
//   3. `Path<T>` / `Path<(...)>` extractors in handler signatures
//
// Any remaining `{...}` in URL strings, or any handler still consuming a
// `Path<T>` extractor, fails CI. The lone exception is the WeChat callback
// query-string protocol, allow-listed via `WECHAT_CALLBACK_FILES`.

const WECHAT_CALLBACK_FILES: &[&str] = &["wap/wechat.rs"];

#[test]
fn no_path_params_in_v1_routes() {
    use std::path::PathBuf;
    let v1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/v1");
    let mut violations: Vec<String> = Vec::new();
    walk(&v1, &mut |p, src| {
        let rel = p.to_string_lossy().to_string();

        // 1) `path = "..."` strings inside `#[utoipa::path(...)]`
        for caps in regex::Regex::new(r#"path\s*=\s*"(/v1/[^"]+)""#)
            .unwrap()
            .captures_iter(src)
        {
            let url = &caps[1];
            if url.contains('{') {
                violations.push(format!("{rel}: utoipa path = {url:?}"));
            }
        }

        // 2) `.route("...", ...)` URL strings
        for caps in regex::Regex::new(r#"\.route\(\s*"([^"]+)""#)
            .unwrap()
            .captures_iter(src)
        {
            let url = &caps[1];
            if url.contains('{') {
                violations.push(format!("{rel}: route = {url:?}"));
            }
        }

        // 3) `Path<T>` extractors in handler signatures
        if WECHAT_CALLBACK_FILES.iter().any(|w| rel.ends_with(w)) {
            return;
        }
        for (lineno, line) in src.lines().enumerate() {
            let trim = line.trim_start();
            if trim.starts_with("//") || trim.starts_with("///") {
                continue;
            }
            if regex::Regex::new(r"\bPath\s*\(\s*(?:\w+|\([^)]+\))\s*\)\s*:\s*Path<")
                .unwrap()
                .is_match(line)
            {
                violations.push(format!("{rel}:{}: {}", lineno + 1, line.trim()));
            }
        }
    });
    assert!(
        violations.is_empty(),
        "URL must not carry path params — every parameter goes in the JSON body:\n  {}",
        violations.join("\n  ")
    );
}

fn walk(dir: &std::path::Path, visit: &mut dyn FnMut(&std::path::Path, &str)) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, visit);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Ok(src) = std::fs::read_to_string(&path) {
                    visit(&path, &src);
                }
            }
        }
    }
}

// ==================== Validation invariant: high-risk search handlers ====================
//
// Search/list handlers that accept free-text `keyword` / `kw` / `name`
// fragments must use `ValidatedQuery` (so length/range/charset rules from
// `validator::Validate` run before SQL). A regression here would let an
// unbounded multi-MB string flow into a `LIKE %?%` query — DoS vector even
// when sqlx parameterisation rules out injection. The test checks the
// **source files** because OpenAPI doesn't surface the extractor type.

#[test]
fn high_risk_search_handlers_use_validated_query() {
    use std::path::PathBuf;
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let targets: &[(&str, &[&str])] = &[
        ("src/v1/wap/jobs.rs", &["JobListQuery", "RecQuery"]),
        ("src/v1/wap/companies.rs", &["CompanyListQuery", "CompanyAutoQuery", "HotCompaniesQuery"]),
        ("src/v1/wap/resumes.rs", &["ResumeListQuery"]),
        ("src/v1/wap/articles.rs", &["ArticleListQuery"]),
        ("src/v1/wap/qna.rs", &["QListQuery", "HotweekQuery", "TopAnswerersQuery"]),
        ("src/v1/wap/search.rs", &["SearchQuery"]),
        ("src/v1/wap/ads.rs", &["AdQuery"]),
    ];
    let mut violations: Vec<String> = Vec::new();
    for (rel, structs) in targets {
        let path = crate_root.join(rel);
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {rel}: {e}"));
        for s in *structs {
            // Forbid `Query<S>` (would be the unvalidated extractor) — must be
            // `ValidatedQuery<S>`.
            let needle_bad = format!("Query<{s}>");
            let needle_good = format!("ValidatedQuery<{s}>");
            for (lineno, line) in src.lines().enumerate() {
                if line.contains(&needle_bad) && !line.contains(&needle_good) {
                    violations.push(format!("{rel}:{}: raw `{needle_bad}` (use `{needle_good}`)", lineno + 1));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "high-risk search handlers must use ValidatedQuery:\n  {}",
        violations.join("\n  ")
    );
}

// ==================== Round 13: company-side fans ====================

#[test]
fn fans_route_registered() {
    // PHP wap/com::attention_me_action → GET /v1/mcenter/fans
    assert_endpoint("POST", "/v1/mcenter/fans");
    assert_schema("FanItem");
}

#[test]
fn job_contact_route_registered() {
    // PHP app/job/comapply::getJobLink_action → GET /v1/wap/jobs/{id}/contact
    assert_endpoint("POST", "/v1/wap/jobs/contact");
    assert_schema("JobContactView");
}

// ==================== Round 14: resume hits + showuid ====================

#[test]
fn resume_expect_hits_route_registered() {
    // PHP app/resume/show::GetHits_action → POST /v1/wap/resumes/expects/{eid}/hits
    assert_endpoint("POST", "/v1/wap/resumes/expects/hits");
    assert_schema("ResumeHitsResp");
}

#[test]
fn resume_default_expect_route_registered() {
    // PHP wap/resume/index::showuid_action → GET /v1/wap/resumes/by-uid/{uid}/default-expect
    assert_endpoint("POST", "/v1/wap/resumes/default-expect");
    assert_schema("DefaultExpectResp");
}

// ==================== Round 15: zph jobs ====================

#[test]
fn zph_jobs_route_registered() {
    // PHP app/zph/index::getJobList_action → GET /v1/wap/zph/{id}/jobs
    assert_endpoint("POST", "/v1/wap/zph/jobs");
}

// ==================== Round 11: recommend quota / eval log / recent examinees ====================

#[test]
fn recommend_quota_route_registered() {
    // PHP ajax::ajax_recommend_interval_action → GET /v1/mcenter/recommend/email/quota
    assert_endpoint("POST", "/v1/mcenter/recommend/email/quota");
    assert_schema("QuotaView");
}

#[test]
fn eval_log_detail_route_registered() {
    // PHP evaluate/exampaper::gradeshow_action → GET /v1/mcenter/eval-logs/{id}
    assert_endpoint("POST", "/v1/mcenter/eval-logs");
}

#[test]
fn eval_recent_examinees_route_registered() {
    // PHP evaluate.model.php::getEvaluateLogList(groupby:uid) → GET /v1/wap/eval-papers/{id}/recent-examinees
    assert_endpoint("POST", "/v1/wap/eval-papers/recent-examinees");
    assert_schema("ExamineeItem");
}

// ==================== Round 10: ad click tracking ====================

#[test]
fn ad_click_route_registered() {
    // PHP index/index::clickhits_action → POST /v1/wap/ads/{id}/click
    assert_endpoint("POST", "/v1/wap/ads/click");
    assert_schema("AdClickResp");
}

// ==================== Round 9: map config / eval paper messages ====================

#[test]
fn map_config_route_registered() {
    // PHP ajax::mapconfig_action → GET /v1/wap/site/map-config
    assert_endpoint("POST", "/v1/wap/site/map-config");
    assert_schema("MapConfigView");
}

#[test]
fn eval_paper_messages_routes_registered() {
    // PHP evaluate.model.php::getMessageList → GET /v1/wap/eval-papers/{id}/messages
    assert_endpoint("POST", "/v1/wap/eval-papers/messages");
    assert_schema("PaperMessageItem");
    // PHP evaluate/exampaper::message_action → POST /v1/mcenter/eval-papers/{id}/messages
    assert_endpoint("POST", "/v1/mcenter/eval-papers/messages");
    assert_schema("PaperMessageForm");
}

// ==================== Round 8: company autocomplete / VIP quote / HR download ====================

#[test]
fn company_autocomplete_route_registered() {
    // PHP ajax::getComBySearch_action → GET /v1/wap/companies/autocomplete
    assert_endpoint("POST", "/v1/wap/companies/autocomplete");
    assert_schema("CompanyAutoItem");
}

#[test]
fn vip_quote_route_registered() {
    // PHP ajax::getPackPrice_action / getVipPrice_action → GET /v1/mcenter/vip/quote/{kind}/{id}
    assert_endpoint("POST", "/v1/mcenter/vip/quote");
    assert_schema("PriceQuoteView");
}

#[test]
fn hr_download_route_registered() {
    // PHP hr/index::ajax_action → POST /v1/wap/hr-docs/{id}/download
    assert_endpoint("POST", "/v1/wap/hr-docs/download");
    assert_schema("HrDownloadResp");
}

// ==================== Round 7: domains / hits / legal slug ====================

#[test]
fn sub_sites_routes_registered() {
    // PHP wap/site::cache_action / domain_action → /v1/wap/site/sub-sites*
    assert_endpoint("POST", "/v1/wap/site/sub-sites");
    assert_endpoint("POST", "/v1/wap/site/sub-sites/match");
    assert_schema("SubSiteView");
}

#[test]
fn article_hits_route_registered() {
    // PHP wap/article::GetHits_action → POST /v1/wap/articles/{id}/hits
    assert_endpoint("POST", "/v1/wap/articles/hits");
    assert_schema("ArticleHitsResp");
}

#[test]
fn job_hits_route_registered() {
    // PHP wap/job::GetHits_action → POST /v1/wap/jobs/{id}/hits
    assert_endpoint("POST", "/v1/wap/jobs/hits");
    assert_schema("JobHitsResp");
}

#[test]
fn legal_slug_routes_registered() {
    // PHP wap/index::about/contact/privacy/protocol → GET /v1/wap/legal/{slug}
    assert_endpoint("POST", "/v1/wap/legal");
    // and the by-name lookup that PHP getDes(name=...) maps to
    assert_endpoint("POST", "/v1/wap/descriptions/by-name");
}

// ==================== Deprecation marker propagated ====================
//
// `#[deprecated]` on the handler function should not by itself flip the
// OpenAPI `deprecated` flag (utoipa requires the `deprecated` attr on the
// `#[utoipa::path]` macro for that). We don't enforce that here — we only
// assert the legacy `/v1/wap/dict/cities` path is still routable so existing
// clients don't 404 during the migration window.
#[test]
fn legacy_dict_cities_still_present() {
    assert_endpoint("POST", "/v1/wap/dict/cities");
    assert_endpoint("POST", "/v1/wap/dict/cities");
}
