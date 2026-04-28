//! Endpoint smoke test — hit every POST `/v1/*` route in-process and
//! classify the response status. The point is not to assert business
//! correctness (an empty `{}` body fails most validators with 400, which
//! is healthy) but to catch endpoints that **5xx** at runtime — those are
//! the actual bugs (missing tables, bad SQL, panicking handlers).
//!
//! Status code semantics:
//! - **2xx / 4xx (≠ 500)**: handler reachable, validator/auth/biz layer
//!   working as designed.
//! - **5xx**: regression, schema mismatch, or panic — must investigate.
//!
//! The test boots a real `AppState` (DB + Redis as configured by `.env`),
//! mints an admin JWT so admin routes don't 403 the smoke probe, then
//! POSTs `{}` to every path discovered from the OpenAPI spec.

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Method, Request};
use phpyun_core::shutdown::CancellationToken;
use phpyun_core::{AppState, Config};
use phpyun_handlers::{build_router_with_state, V1Doc};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use tower::util::ServiceExt;
use utoipa::OpenApi;

const TEST_ADMIN_UID: u64 = 1; // Conventional super-admin uid in PHPYun installs.

/// Issue a minimal access token + register the corresponding session row so
/// the `AuthenticatedUser` extractor's session-presence check passes. The
/// extractor enforces that every JWT corresponds to a live row in
/// `phpyun_user_session`; we stamp one in here using the same code path the
/// real login flow uses.
async fn issue_token_with_session(state: &AppState, cfg: &Config, usertype: u8) -> String {
    let issued = phpyun_core::jwt::issue_pair(cfg, TEST_ADMIN_UID, usertype, 0)
        .expect("issue_pair");
    phpyun_services::user_session_service::record_login(
        state,
        phpyun_services::user_session_service::LoginRecord {
            uid: TEST_ADMIN_UID,
            usertype,
            jti_access: &issued.jti_access,
            jti_refresh: &issued.jti_refresh,
            access_exp: issued.access_exp,
            refresh_exp: issued.refresh_exp,
            ip: "127.0.0.1",
            ua: "smoke-test",
        },
    )
    .await
    .expect("record_login for test session");
    issued.access
}

#[tokio::test(flavor = "multi_thread")]
async fn smoke_every_v1_post_endpoint() {
    // Capture sqlx + handler-layer tracing so 5xx bodies can include the real
    // error rather than just the generic `"db"` envelope. Filter is permissive
    // when RUST_LOG isn't set so the report is informative even on plain
    // `cargo test`.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,sqlx=error,phpyun_core=warn,phpyun_handlers=warn".into()),
        )
        .with_test_writer()
        .try_init();
    // Config::load reads .env (the same file the running server uses) before
    // pulling values from the environment.
    let config = Config::load().expect("Config::load (is .env present?)");
    let shutdown = CancellationToken::new();
    let state = AppState::build(config.clone(), shutdown.clone())
        .await
        .expect("AppState::build (DB / Redis reachable?)");
    let router = build_router_with_state(&config, state.clone()).with_state(state.clone());

    let admin_token = issue_token_with_session(&state, &config, 3).await;
    let user_token = issue_token_with_session(&state, &config, 1).await;

    // Enumerate POST paths from the OpenAPI spec.
    let api = V1Doc::openapi();
    let mut paths: Vec<(String, bool /* admin */, bool /* needs auth */)> = Vec::new();
    for (path, item) in api.paths.paths.iter() {
        let Some(op) = &item.post else { continue };
        let needs_auth = op
            .security
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        let is_admin = path.starts_with("/v1/admin/");
        paths.push((path.clone(), is_admin, needs_auth));
    }
    paths.sort();

    // Bucket results by status code.
    let mut by_status: BTreeMap<u16, Vec<String>> = BTreeMap::new();
    let mut server_errors: Vec<(String, u16, String)> = Vec::new();
    // Detailed inspection for 200 responses: shape + emptiness flags so we
    // can spot endpoints that "succeed" but return null/empty/malformed data.
    let mut bad_envelope: Vec<(String, String)> = Vec::new();
    let mut data_null: Vec<String> = Vec::new();
    let mut data_empty_list: Vec<String> = Vec::new();
    let mut ok_with_data: usize = 0;

    for (idx, (path, is_admin, needs_auth)) in paths.iter().enumerate() {
        let token = if *is_admin { &admin_token } else { &user_token };
        let mut req = Request::builder()
            .method(Method::POST)
            .uri(path.as_str())
            .header("content-type", "application/json");
        if *needs_auth {
            req = req.header("authorization", format!("Bearer {token}"));
        }
        let mut req = req.body(Body::from("{}")).unwrap();
        let octet_a = (idx >> 8) as u8;
        let octet_b = (idx & 0xff) as u8;
        let peer: SocketAddr =
            format!("10.99.{octet_a}.{octet_b}:65535").parse().unwrap();
        req.extensions_mut().insert(ConnectInfo(peer));

        let resp = router.clone().oneshot(req).await.expect("router oneshot");
        let status = resp.status();
        by_status
            .entry(status.as_u16())
            .or_default()
            .push(path.clone());

        // Read body once; even non-5xx may carry diagnostic info.
        let bytes = axum::body::to_bytes(resp.into_body(), 64 * 1024)
            .await
            .unwrap_or_default();
        let body_str = String::from_utf8_lossy(&bytes).into_owned();

        if status.is_server_error() {
            server_errors.push((path.clone(), status.as_u16(), body_str.clone()));
            continue;
        }

        // For 200 responses, validate the standard `{code, msg, data}`
        // envelope and inspect the `data` payload.
        if status.as_u16() == 200 {
            let v: serde_json::Value = match serde_json::from_str(&body_str) {
                Ok(v) => v,
                Err(_) => {
                    bad_envelope.push((path.clone(), "non-JSON body".into()));
                    continue;
                }
            };
            // Standard envelope check.
            let code_ok = v.get("code").and_then(|x| x.as_u64()) == Some(200);
            let msg_present = v.get("msg").and_then(|x| x.as_str()).is_some();
            if !code_ok || !msg_present {
                bad_envelope.push((
                    path.clone(),
                    format!("code_ok={code_ok} msg_present={msg_present}"),
                ));
                continue;
            }
            // `data` may legitimately be absent for action endpoints that
            // return `ApiOk` (Option::is_none skips serialization). Treat
            // missing data field as Null for classification.
            let owned_null = serde_json::Value::Null;
            let data = v.get("data").unwrap_or(&owned_null);
            // Classify data shape.
            match data {
                serde_json::Value::Null => data_null.push(path.clone()),
                serde_json::Value::Array(arr) if arr.is_empty() => {
                    data_empty_list.push(path.clone());
                }
                serde_json::Value::Object(obj) => {
                    // Paged-style {list, total, page, page_size} — flag empty list.
                    if let Some(list) = obj.get("list").and_then(|x| x.as_array()) {
                        if list.is_empty() {
                            data_empty_list.push(path.clone());
                        } else {
                            ok_with_data += 1;
                        }
                    } else {
                        ok_with_data += 1;
                    }
                }
                _ => {
                    ok_with_data += 1;
                }
            }
        }
    }

    // ---------- Report ----------
    println!("\n========== ENDPOINT SMOKE REPORT ==========");
    println!("Total POST endpoints probed: {}", paths.len());
    for (status, list) in &by_status {
        println!("  HTTP {} : {} endpoint(s)", status, list.len());
    }
    println!();
    let two_hundred_count = by_status.get(&200).map(|l| l.len()).unwrap_or(0);
    println!(
        "200-response data audit ({}/{} carry non-empty data):",
        ok_with_data, two_hundred_count
    );
    if !bad_envelope.is_empty() {
        println!("  ❌ malformed envelope ({}):", bad_envelope.len());
        for (p, why) in &bad_envelope {
            println!("       {p}  ({why})");
        }
    }
    if !data_null.is_empty() {
        println!("  ⚠ data=null ({}):", data_null.len());
        for p in &data_null {
            println!("       {p}");
        }
    }
    if !data_empty_list.is_empty() {
        println!("  ⚠ data is empty list ({}):", data_empty_list.len());
        for p in &data_empty_list {
            println!("       {p}");
        }
    }
    println!();
    if !server_errors.is_empty() {
        println!("❌ 5xx endpoints (real bugs):");
        for (path, code, body) in &server_errors {
            let snippet: String = body.chars().take(280).collect();
            println!("  [{code}] {path}\n    body: {snippet}");
        }
    } else {
        println!("✅ No 5xx responses across {} endpoints", paths.len());
    }
    println!("===========================================\n");

    // The test only fails when a 5xx slips through — that's the real bug class.
    assert!(
        server_errors.is_empty(),
        "{} endpoint(s) returned 5xx — see report above",
        server_errors.len()
    );
}
