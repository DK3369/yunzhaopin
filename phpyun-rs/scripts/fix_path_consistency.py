#!/usr/bin/env python3
"""Auto-fix `#[utoipa::path(... path = "...")]` annotations so they match the
`.route("...", post(handler))` strings registered in `routes()`.

For every handler with a router-vs-openapi mismatch, find the
`#[utoipa::path(...)]` attribute that precedes `pub async fn <handler>(`,
and rewrite the `path = "..."` field inside that attribute to match the
router-effective path.

Run after `check_path_consistency.py` has identified the discrepancies; this
script is idempotent — running it again on a clean tree is a no-op.

Usage:
  scripts/fix_path_consistency.py             # apply edits in place
  scripts/fix_path_consistency.py --dry-run   # show edits without writing
"""
from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "crates" / "handlers" / "src"
V1_DIR = ROOT / "v1"

ROUTE_RE = re.compile(
    r'\.route\(\s*"([^"]+)"\s*,\s*post\(\s*(\w+)\s*\)\s*,?\s*\)'
)


def prefix_for(rs_path: Path) -> str:
    rel = rs_path.relative_to(V1_DIR)
    parts = list(rel.parts)
    parts[-1] = ""
    return "/v1/" + "/".join(p for p in parts if p)


def join(prefix: str, route: str) -> str:
    if not route.startswith("/"):
        route = "/" + route
    return prefix.rstrip("/") + route


def find_attr_block(src: str, fn_match_start: int) -> tuple[int, int] | None:
    """Walk backwards from a `pub async fn <name>(` start position to locate
    the `#[utoipa::path(...)]` immediately preceding it. Returns (start, end)
    of the attribute (start = `#`, end = the matching `]` index + 1).
    """
    # Search backwards for `#[utoipa::path(`.
    head = src.rfind("#[utoipa::path(", 0, fn_match_start)
    if head == -1:
        return None
    # Find matching `)]`. Walk forward counting parens.
    open_paren = head + len("#[utoipa::path")
    if src[open_paren] != "(":
        return None
    depth = 1
    i = open_paren + 1
    while i < len(src) and depth > 0:
        c = src[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        i += 1
    # i now sits just after the matching `)`. Expect `]` next.
    if i < len(src) and src[i] == "]":
        return head, i + 1
    return None


def rewrite_attr_path(attr_text: str, new_path: str) -> str:
    """Replace the value of `path = "..."` inside an attribute body."""
    return re.sub(
        r'(\bpath\s*=\s*")[^"]+(")',
        lambda m: f"{m.group(1)}{new_path}{m.group(2)}",
        attr_text,
        count=1,
    )


def fix_file(rs: Path, dry_run: bool) -> list[tuple[str, str, str]]:
    """Returns list of (handler_fn, old_path_in_attr, new_path_in_attr) edits."""
    src = rs.read_text(encoding="utf-8", errors="replace")
    prefix = prefix_for(rs)

    routes = {h: join(prefix, p) for p, h in [
        (m.group(1), m.group(2)) for m in ROUTE_RE.finditer(src)
    ]}
    if not routes:
        return []

    fn_re = re.compile(r'pub\s+async\s+fn\s+(\w+)\b')
    edits: list[tuple[str, str, str]] = []
    new_src = src
    # Walk handlers in reverse so positions stay valid as we splice strings.
    matches = list(fn_re.finditer(src))
    matches.reverse()
    for m in matches:
        fn = m.group(1)
        if fn not in routes:
            continue
        block = find_attr_block(new_src, m.start())
        if block is None:
            continue
        attr_text = new_src[block[0]:block[1]]
        path_m = re.search(r'\bpath\s*=\s*"([^"]+)"', attr_text)
        if not path_m:
            continue
        declared = path_m.group(1)
        target = routes[fn]
        if declared == target:
            continue
        new_attr = rewrite_attr_path(attr_text, target)
        new_src = new_src[:block[0]] + new_attr + new_src[block[1]:]
        edits.append((fn, declared, target))

    if edits and not dry_run:
        rs.write_text(new_src, encoding="utf-8")
    return edits


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    total = 0
    files = []
    for dirpath, _, filenames in os.walk(V1_DIR):
        for f in filenames:
            if f.endswith(".rs") and f != "mod.rs":
                files.append(Path(dirpath) / f)
    for rs in sorted(files):
        edits = fix_file(rs, dry_run=args.dry_run)
        if edits:
            rel = rs.relative_to(ROOT.parent.parent.parent)
            print(f"{rel}:")
            for fn, old, new in edits:
                print(f"  {fn}:  {old}  →  {new}")
            total += len(edits)

    if total == 0:
        print("✅ no edits needed")
        return 0

    verb = "would apply" if args.dry_run else "applied"
    print(f"\n{verb} {total} edits across {sum(1 for _ in files)} scanned files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
