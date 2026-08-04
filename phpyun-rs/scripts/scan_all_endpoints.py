#!/usr/bin/env python3
"""Safe all-method endpoint contract scan against a running app.

GET endpoints and validation/auth failures are probed by default. Mutating
operations are skipped unless SAFE_WRITES=1; payment, account deletion,
outbound messaging and OAuth operations remain skipped unless
ALLOW_DANGEROUS=1 is explicitly set.
"""
from __future__ import annotations

import json
import os
import sys
import time
import urllib.error
import urllib.request
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from scan_endpoints import (  # noqa: E402
    DEFAULT_UA,
    HOST,
    LOGIN_PATH,
    LOGIN_PASS,
    LOGIN_USER,
    SPEC_PATH,
    SYNTH_BODY,
    TIMEOUT,
    get_token,
    synthesize_body,
    validate_response,
    with_rate_retry,
)

SAFE_WRITES = os.environ.get("SAFE_WRITES", "0").lower() not in {"0", "false", "no"}
ALLOW_DANGEROUS = os.environ.get("ALLOW_DANGEROUS", "0").lower() not in {"0", "false", "no"}
METHODS = ("get", "post", "put", "patch", "delete")
DANGEROUS_WORDS = (
    "pay", "payment", "top-up", "topup", "withdraw", "logout", "delete-account",
    "unbind", "oauth", "email", "sms", "upload", "revoke", "purge",
)


def call(method: str, path: str, body: bytes, token: str | None) -> tuple[int, bytes]:
    headers = {"user-agent": DEFAULT_UA, "accept": "application/json"}
    if token:
        headers["authorization"] = f"Bearer {token}"
    if method != "get":
        headers["content-type"] = "application/json"
    req = urllib.request.Request(HOST + path, data=None if method == "get" else body,
                                 headers=headers, method=method.upper())
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            return resp.status, resp.read()
    except urllib.error.HTTPError as exc:
        return exc.code, exc.read() or b""
    except Exception as exc:
        return 0, f"<exception: {exc}>".encode()


def dangerous(path: str) -> bool:
    lowered = path.lower()
    return any(word in lowered for word in DANGEROUS_WORDS)


def main() -> int:
    token = get_token()
    status, raw = with_rate_retry(lambda: call("get", SPEC_PATH, b"", None))
    if status != 200:
        print(f"spec fetch failed: HTTP {status}")
        return 2
    spec = json.loads(raw)
    ops = []
    skipped = []
    for path, item in sorted(spec.get("paths", {}).items()):
        for method in METHODS:
            op = item.get(method)
            if not op:
                continue
            is_write = method != "get"
            if is_write and not SAFE_WRITES:
                skipped.append((method.upper(), path, "write disabled"))
                continue
            if is_write and dangerous(path) and not ALLOW_DANGEROUS:
                skipped.append((method.upper(), path, "dangerous disabled"))
                continue
            ops.append((method, path, op))

    statuses = Counter()
    failures = []
    violations = []
    def probe(item):
        method, path, op = item
        body = synthesize_body(spec, op) if method != "get" and SYNTH_BODY else b"{}"
        # A single request per endpoint is intentional: repeated retries would
        # make a full scan wait behind the application's IP limiter.
        return item, call(method, path, body, token)

    parallel = max(1, int(os.environ.get("PARALLEL", "8")))
    results = []
    with ThreadPoolExecutor(max_workers=parallel) as pool:
        futures = [pool.submit(probe, item) for item in ops]
        for future in as_completed(futures):
            results.append(future.result())

    for (method, path, op), (st, response) in results:
        statuses[st] += 1
        if st >= 500 or st == 0:
            failures.append((method.upper(), path, st, response[:280]))
        issues = validate_response(spec, op, st, response)
        if issues:
            violations.append((method.upper(), path, st, issues[:3]))

    print("========== ALL ENDPOINT SCAN ==========")
    print(f"target: {HOST}")
    print(f"probed: {len(ops)}  skipped: {len(skipped)}")
    print("statuses: " + ", ".join(f"{k}={v}" for k, v in sorted(statuses.items())))
    if skipped:
        print("skip reasons: " + ", ".join(sorted({reason for _, _, reason in skipped})))
    if failures:
        print("5xx/network failures:")
        for method, path, st, body in failures:
            print(f"  {method} {path} -> {st}: {body.decode('utf-8', 'replace')}")
    if violations:
        print("contract violations:")
        for method, path, st, issues in violations:
            print(f"  {method} {path} -> {st}: {'; '.join(issues)}")
    if not failures and not violations:
        print("OK: no 5xx/network failures or response contract violations")
    print("=======================================")
    return 1 if failures or violations else 0


if __name__ == "__main__":
    sys.exit(main())
