//! Cross-crate OpenAPI contract after splitting `api-admin`.

use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;

fn t2_admin_paths() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../doc/snapshots/admin_paths.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

#[test]
fn admin_paths_match_t2_snapshot() {
    let mut actual: Vec<_> = phpyun_api_admin::openapi()
        .paths
        .paths
        .keys()
        .cloned()
        .collect();
    actual.sort();
    assert_eq!(
        actual,
        t2_admin_paths(),
        "admin OpenAPI paths drifted from T2 snapshot (doc/snapshots/admin_paths.txt)"
    );
    assert_eq!(actual.len(), 307, "admin path count is {}", actual.len());
}

fn collect_ops(doc: &utoipa::openapi::OpenApi) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for (path, item) in &doc.paths.paths {
        let mut push = |method: &str, op: &utoipa::openapi::path::Operation| {
            out.push((
                method.to_string(),
                path.clone(),
                op.operation_id.clone().unwrap_or_default(),
            ));
        };
        if let Some(op) = &item.get {
            push("GET", op);
        }
        if let Some(op) = &item.post {
            push("POST", op);
        }
        if let Some(op) = &item.put {
            push("PUT", op);
        }
        if let Some(op) = &item.delete {
            push("DELETE", op);
        }
        if let Some(op) = &item.patch {
            push("PATCH", op);
        }
    }
    out
}

#[test]
fn admin_spec_is_admin_only_and_nonempty() {
    let admin = phpyun_api_admin::openapi();
    assert!(
        !admin.paths.paths.is_empty(),
        "admin OpenAPI has no paths"
    );
    for path in admin.paths.paths.keys() {
        assert!(
            path.starts_with("/v1/admin"),
            "admin spec leaked non-admin path {path}"
        );
    }
}

#[test]
fn merged_specs_have_unique_method_path_and_operation_id() {
    let v1 = phpyun_handlers::v1_openapi();
    let admin = phpyun_api_admin::openapi();
    let mut pairs = HashSet::new();
    let mut ids = HashSet::new();
    for (method, path, id) in collect_ops(&v1)
        .into_iter()
        .chain(collect_ops(&admin))
    {
        assert!(
            pairs.insert((method.clone(), path.clone())),
            "duplicate ({method}, {path})"
        );
        if !id.is_empty() {
            assert!(ids.insert(id.clone()), "duplicate operationId {id}");
        }
    }
}

#[test]
fn v1_wap_and_mcenter_baseline_paths_still_exist() {
    let v1 = phpyun_handlers::v1_openapi();
    let keys: BTreeSet<_> = v1.paths.paths.keys().cloned().collect();
    for required in [
        "/v1/wap/jobs",
        "/v1/wap/jobs/detail",
        "/v1/wap/companies",
        "/v1/wap/home",
        "/v1/wap/login",
        "/v1/wap/refresh",
        "/v1/mcenter/apply",
        "/v1/mcenter/jobs",
        "/v1/mcenter/resume",
    ] {
        assert!(keys.contains(required), "missing baseline path {required}");
    }
    assert!(!keys.iter().any(|p| p.starts_with("/v1/admin")));
}
