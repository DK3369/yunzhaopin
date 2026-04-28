#!/usr/bin/env python3
"""Automated endpoint scan against a running phpyun-rs server.

Reads the live OpenAPI spec, logs in to obtain a fresh access token, then
POSTs `{}` to every documented POST path with that Bearer token. Buckets
every response by status code, prints a 5xx report (real bugs), and exits
non-zero if any 5xx is found.

Usage:
  scripts/scan_endpoints.py
  HOST=http://127.0.0.1:3000 LOGIN_USER=duncan11 LOGIN_PASS=dd112211 scripts/scan_endpoints.py
  TOKEN=eyJ... scripts/scan_endpoints.py    # bring your own token

Stdlib-only — no jq / requests / etc required.
"""
from __future__ import annotations

import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import Tuple

# tower_governor returns plain-text bodies like "Too Many Requests! Wait for 41s"
# when the per-IP bucket is empty. Parse the cooldown so we sleep just enough.
_WAIT_RE = re.compile(rb"Wait for (\d+)s")

HOST = os.environ.get("HOST", "http://127.0.0.1:3000").rstrip("/")
# Note: don't read `USER` — it's a standard shell env var (= the unix user
# running the script). Use the namespaced `LOGIN_USER` instead.
LOGIN_USER = os.environ.get("LOGIN_USER", "duncan11")
LOGIN_PASS = os.environ.get("LOGIN_PASS", "dd112211")
LOGIN_PATH = os.environ.get("LOGIN_PATH", "/v1/wap/login")
SPEC_PATH = os.environ.get("SPEC_PATH", "/api-docs/v1/openapi.json")
BODY = os.environ.get("BODY", "{}").encode()
PARALLEL = int(os.environ.get("PARALLEL", "4"))
TIMEOUT = float(os.environ.get("TIMEOUT", "10"))
# Per-request delay (seconds) — prevents the per-IP rate limiter from
# triaging real responses as 429. The default works against the dev rate
# limiter; tune via `DELAY=0.05 PARALLEL=2 ...` for tighter loops.
DELAY = float(os.environ.get("DELAY", "0.15"))
# How many times to retry a 429 with linear backoff before giving up.
RATE_RETRIES = int(os.environ.get("RATE_RETRIES", "3"))


# Some middlewares (UA-based bot guard) block default `Python-urllib`. Pose
# as a regular browser so we exercise the actual handler stack.
DEFAULT_UA = (
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) "
    "phpyun-rs/scan-endpoints"
)


def post(path: str, data: bytes, token: str | None) -> Tuple[int, bytes]:
    headers = {
        "content-type": "application/json",
        "user-agent": DEFAULT_UA,
    }
    if token:
        headers["authorization"] = f"Bearer {token}"
    req = urllib.request.Request(
        url=HOST + path, data=data, headers=headers, method="POST"
    )
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            return resp.status, resp.read()
    except urllib.error.HTTPError as e:
        return e.code, e.read() or b""
    except Exception as e:
        return 0, f"<exception: {e}>".encode()


def get(path: str) -> Tuple[int, bytes]:
    req = urllib.request.Request(
        url=HOST + path, method="GET", headers={"user-agent": DEFAULT_UA}
    )
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            return resp.status, resp.read()
    except urllib.error.HTTPError as e:
        return e.code, e.read() or b""
    except Exception as e:
        return 0, f"<exception: {e}>".encode()


def with_rate_retry(call, *args, retries: int = 6) -> Tuple[int, bytes]:
    """Wrap any GET/POST so it transparently waits out a 429 cooldown.

    Used for the spec fetch and login — both must succeed before the scan
    proper can start, so a single rate-limit hit shouldn't abort the run.
    """
    for attempt in range(retries + 1):
        st, b = call(*args)
        if st != 429 or attempt == retries:
            return st, b
        m = _WAIT_RE.search(b)
        wait = min(int(m.group(1)) + 0.2, 90.0) if m else 1.0 * (attempt + 1)
        print(
            f"  rate-limited (HTTP 429), waiting {wait:.1f}s "
            f"(attempt {attempt + 1}/{retries})"
        )
        time.sleep(wait)
    return st, b


def get_token() -> str:
    if os.environ.get("TOKEN"):
        print(f"auth        : using TOKEN from env")
        return os.environ["TOKEN"]
    payload = json.dumps({"username": LOGIN_USER, "password": LOGIN_PASS}).encode()
    status, body = with_rate_retry(post, LOGIN_PATH, payload, None)
    if status != 200:
        print(f"❌ login failed (HTTP {status}): {body[:400]!r}")
        sys.exit(2)
    j = json.loads(body)
    tok = (j.get("data") or {}).get("access_token")
    if not tok:
        print(f"❌ login response had no token: {body[:400]!r}")
        sys.exit(2)
    print(
        f"auth        : login ok, uid={j['data'].get('uid')}, "
        f"usertype={j['data'].get('usertype')}"
    )
    return tok


def main() -> int:
    print(f"scan target : {HOST}")
    token = get_token()

    status, spec_bytes = with_rate_retry(get, SPEC_PATH)
    if status != 200:
        print(f"❌ openapi spec fetch failed (HTTP {status})")
        return 3
    spec = json.loads(spec_bytes)
    paths = sorted(p for p, item in spec.get("paths", {}).items() if "post" in item)
    print(f"openapi     : {len(paths)} POST path(s) discovered")
    print()

    by_status: dict[int, list[str]] = defaultdict(list)
    server_errors: list[tuple[str, int, bytes]] = []

    def probe(p: str) -> tuple[str, int, bytes]:
        # On 429, parse the limiter's `Wait for Ns` hint and sleep that long
        # before retrying. Falls back to linear backoff if the hint isn't
        # present (e.g. a different limiter mid-stack).
        for attempt in range(RATE_RETRIES + 1):
            st, b = post(p, BODY, token)
            if st != 429 or attempt == RATE_RETRIES:
                return p, st, b
            m = _WAIT_RE.search(b)
            if m:
                # +0.2s safety margin; cap at 90s so a wedged limiter can't
                # stall the entire scan.
                wait = min(int(m.group(1)) + 0.2, 90.0)
            else:
                wait = 0.5 * (attempt + 1)
            time.sleep(wait)
        return p, st, b

    if PARALLEL <= 1:
        # Serial mode: pace requests with DELAY to keep the IP rate limit
        # from kicking in, which produces a cleaner per-endpoint reading.
        for p in paths:
            _, st, b = probe(p)
            by_status[st].append(p)
            if 500 <= st < 600:
                server_errors.append((p, st, b))
            if DELAY > 0:
                time.sleep(DELAY)
    else:
        with ThreadPoolExecutor(max_workers=PARALLEL) as pool:
            futures = [pool.submit(probe, p) for p in paths]
            for f in as_completed(futures):
                p, st, b = f.result()
                by_status[st].append(p)
                if 500 <= st < 600:
                    server_errors.append((p, st, b))

    print("========== ENDPOINT SCAN REPORT ==========")
    print(f"Total POST endpoints probed: {len(paths)}")
    for st in sorted(by_status):
        print(f"  HTTP {st:<3} : {len(by_status[st]):>3} endpoint(s)")
    print()
    if os.environ.get("VERBOSE"):
        for st in sorted(by_status):
            print(f"---- HTTP {st} ----")
            for p in sorted(by_status[st]):
                print(f"  {p}")
        print()
    if server_errors:
        print("❌ 5xx endpoints (real bugs):")
        for p, st, body in sorted(server_errors):
            snippet = body[:280].decode("utf-8", "replace").replace("\n", " ")
            print(f"  [{st}] {p}\n    body: {snippet}")
        print("===========================================")
        return 1
    print(f"✅ No 5xx responses across {len(paths)} endpoints")
    print("===========================================")
    return 0


if __name__ == "__main__":
    sys.exit(main())
