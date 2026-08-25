//! App-contract OpenAPI snapshot: `/v1/wap` + `/v1/mcenter` must never contain admin.
//! Path list is locked to the T2 dump in `doc/snapshots/v1_paths.txt`.

use phpyun_handlers::v1_openapi;
use std::path::PathBuf;

fn t2_v1_paths() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../../doc/snapshots/v1_paths.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

fn current_paths() -> Vec<String> {
    let mut v: Vec<_> = v1_openapi().paths.paths.keys().cloned().collect();
    v.sort();
    v
}

#[test]
fn v1_paths_match_t2_snapshot() {
    assert_eq!(
        current_paths(),
        t2_v1_paths(),
        "v1 OpenAPI paths drifted from T2 snapshot (doc/snapshots/v1_paths.txt)"
    );
}

#[test]
fn v1_openapi_has_no_admin_paths() {
    let api = v1_openapi();
    let mut admin = Vec::new();
    for path in api.paths.paths.keys() {
        if path.starts_with("/v1/admin") {
            admin.push(path.clone());
        }
    }
    assert!(
        admin.is_empty(),
        "v1 spec leaked admin paths: {admin:?}"
    );
}

#[test]
fn v1_openapi_covers_wap_and_mcenter() {
    let api = v1_openapi();
    assert!(api.paths.paths.contains_key("/v1/wap/jobs"));
    assert!(api.paths.paths.contains_key("/v1/wap/login"));
    assert!(api.paths.paths.contains_key("/v1/mcenter/apply"));
    assert!(
        api.paths.paths.len() >= 350,
        "v1 path count unexpectedly low: {}",
        api.paths.paths.len()
    );
}

#[test]
fn v1_operation_ids_are_unique() {
    let api = v1_openapi();
    let mut ids = Vec::new();
    for item in api.paths.paths.values() {
        for op in [
            item.get.as_ref(),
            item.post.as_ref(),
            item.put.as_ref(),
            item.delete.as_ref(),
            item.patch.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(id) = op.operation_id.as_ref() {
                ids.push(id.clone());
            }
        }
    }
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate operationId in v1 spec");
}
