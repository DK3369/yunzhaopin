#!/usr/bin/env python3
"""Static check: every `#[utoipa::path(path = "...")]` annotation must match
the `.route("...", post(handler))` string that mounts the same handler.

Background: utoipa's `path` is a free-form string maintained alongside the
axum router. A typo (or a copy-paste from `list` to `detail`) silently
desynchronises Swagger UI from the real API — Swagger documents an endpoint
that nobody serves while the real endpoint is undocumented. This script
walks `crates/handlers/src/v1/**/*.rs`, extracts both, and reports every
disagreement.

Usage:
  scripts/check_path_consistency.py                                # report
  scripts/check_path_consistency.py --strict                       # exit 1 on mismatch

The router prefix per file is inferred from its directory:
  v1/wap/foo.rs       → /v1/wap/<route_str>
  v1/mcenter/foo.rs   → /v1/mcenter/<route_str>
  v1/admin/foo.rs     → /v1/admin/<route_str>
This matches `v1/mod.rs`'s `nest("/wap", …)` etc.
"""
from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parent.parent / "crates" / "handlers" / "src"
V1_DIR = ROOT / "v1"

# Match `.route("/path", post(handler))` — `\s` covers newlines, and the
# trailing `,?` accepts the multi-line style:
#     .route(
#         "/foo",
#         post(handler),
#     )
ROUTE_RE = re.compile(
    r'\.route\(\s*"([^"]+)"\s*,\s*post\(\s*(\w+)\s*\)\s*,?\s*\)'
)

# Match `#[utoipa::path(...)]` followed by `pub async fn <name>(`. Single capture
# group on the function name; the attribute body is in the prior segment.
ATTR_FN_RE = re.compile(
    r'#\[utoipa::path\((?P<attr>.*?)\)\]\s*pub\s+async\s+fn\s+(?P<fn>\w+)\b',
    re.DOTALL,
)
PATH_FIELD_RE = re.compile(r'\bpath\s*=\s*"([^"]+)"')


def prefix_for(rs_path: Path) -> str:
    """Translate `v1/<area>/<file>.rs` into the URL prefix axum nests this at."""
    rel = rs_path.relative_to(V1_DIR)
    parts = list(rel.parts)
    parts[-1] = ""  # drop the file name itself
    # crates/handlers/src/v1/<area>/<file>.rs → /v1/<area>
    return "/v1/" + "/".join(p for p in parts if p)


def join(prefix: str, route: str) -> str:
    """Concatenate the area prefix + handler-local `.route("/foo")` path."""
    if not route.startswith("/"):
        route = "/" + route
    return prefix.rstrip("/") + route


def scan_file(rs: Path) -> tuple[dict[str, str], dict[str, str]]:
    """Return (router_paths_by_handler, openapi_paths_by_handler) for one file."""
    src = rs.read_text(encoding="utf-8", errors="replace")
    prefix = prefix_for(rs)

    routes: dict[str, str] = {}
    for m in ROUTE_RE.finditer(src):
        path, handler = m.group(1), m.group(2)
        routes[handler] = join(prefix, path)

    openapi: dict[str, str] = {}
    for m in ATTR_FN_RE.finditer(src):
        fn = m.group("fn")
        path_m = PATH_FIELD_RE.search(m.group("attr"))
        if path_m:
            openapi[fn] = path_m.group(1)

    return routes, openapi


def iter_handler_files() -> Iterable[Path]:
    for dirpath, _, filenames in os.walk(V1_DIR):
        for f in filenames:
            if not f.endswith(".rs"):
                continue
            if f in ("mod.rs",):
                continue
            yield Path(dirpath) / f


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strict", action="store_true",
                    help="exit 1 if any mismatch found")
    args = ap.parse_args()

    mismatches: list[tuple[Path, str, str, str]] = []  # (file, fn, router, openapi)
    undocumented: list[tuple[Path, str, str]] = []     # (file, fn, router)
    orphan_docs: list[tuple[Path, str, str]] = []      # (file, fn, openapi)
    duplicate_openapi: list[tuple[str, list[tuple[Path, str]]]] = []

    by_path: dict[str, list[tuple[Path, str]]] = {}

    for rs in sorted(iter_handler_files()):
        routes, openapi = scan_file(rs)
        for fn, route_path in routes.items():
            decl = openapi.get(fn)
            if decl is None:
                undocumented.append((rs, fn, route_path))
            elif decl != route_path:
                mismatches.append((rs, fn, route_path, decl))
        for fn, decl in openapi.items():
            if fn not in routes:
                orphan_docs.append((rs, fn, decl))
            by_path.setdefault(decl, []).append((rs, fn))

    for path, owners in by_path.items():
        if len(owners) > 1:
            duplicate_openapi.append((path, owners))

    print(f"== path consistency check ==")
    print(f"  scanned files     : {len(list(iter_handler_files()))}")
    print(f"  mismatches        : {len(mismatches)}")
    print(f"  undocumented      : {len(undocumented)}")
    print(f"  orphan-doc        : {len(orphan_docs)}")
    print(f"  duplicate openapi : {len(duplicate_openapi)}")
    print()

    if mismatches:
        print("❌ OpenAPI path != router path:")
        for f, fn, r, d in sorted(mismatches):
            rel = f.relative_to(ROOT.parent.parent.parent)
            print(f"  {rel}::{fn}")
            print(f"    router  : {r}")
            print(f"    openapi : {d}")
        print()

    if duplicate_openapi:
        print("⚠ duplicate OpenAPI paths (utoipa will merge / overwrite):")
        for p, owners in sorted(duplicate_openapi):
            print(f"  {p}")
            for f, fn in owners:
                rel = f.relative_to(ROOT.parent.parent.parent)
                print(f"    - {rel}::{fn}")
        print()

    if undocumented:
        print(f"ℹ {len(undocumented)} handler(s) without #[utoipa::path] (Swagger won't list them):")
        for f, fn, r in sorted(undocumented)[:25]:
            rel = f.relative_to(ROOT.parent.parent.parent)
            print(f"  {rel}::{fn}  →  {r}")
        if len(undocumented) > 25:
            print(f"  ... and {len(undocumented) - 25} more")
        print()

    if orphan_docs:
        print(
            f"⚠ {len(orphan_docs)} #[utoipa::path] decoration(s) on functions "
            "not registered in `routes()` (dead doc):"
        )
        for f, fn, d in sorted(orphan_docs):
            rel = f.relative_to(ROOT.parent.parent.parent)
            print(f"  {rel}::{fn}  doc → {d}")
        print()

    failed = bool(mismatches or duplicate_openapi)
    if failed:
        print("❌ FAIL")
        return 1 if args.strict else 0
    print("✅ all router paths match their OpenAPI annotations")
    return 0


if __name__ == "__main__":
    sys.exit(main())
