//! Cross-crate OpenAPI contract after splitting `api-admin`.

use std::collections::{BTreeSet, HashSet};

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
