#!/usr/bin/env python3
"""Automated endpoint scan against a running phpyun-rs server.

Two passes against every documented `POST /v1/*` endpoint:

  1. **Health pass** — POST `{}` with the logged-in user's Bearer token, bucket
     responses by HTTP status, fail on any 5xx. This is the equivalent of the
     in-process smoke test but exercises the real network/middleware/DB stack.

  2. **Schema pass** — for every endpoint that returned 200, validate the
     response envelope and the `data` payload against the OpenAPI schema:

     - envelope: `{code, msg, data}` — `code` matches HTTP status, `msg` is a
       non-empty string, `data` is present.
     - `data` shape: resolve the response schema's `$ref`, then check the
       object has every `required` field, the right top-level type, and (one
       level deep) field types. Extra fields are tolerated; nested objects are
       structurally checked but not exhaustively validated.

Usage:
  scripts/scan_endpoints.py
  HOST=http://127.0.0.1:3000 LOGIN_USER=duncan11 LOGIN_PASS=dd112211 scripts/scan_endpoints.py
  TOKEN=eyJ... scripts/scan_endpoints.py    # bring your own token
  VERBOSE=1 scripts/scan_endpoints.py       # print the per-bucket endpoint list

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
from typing import Any, Tuple

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
DELAY = float(os.environ.get("DELAY", "0"))
RATE_RETRIES = int(os.environ.get("RATE_RETRIES", "3"))
VALIDATE = os.environ.get("VALIDATE", "1") not in ("0", "false", "no", "")
# When 1, build a minimal valid request body from each endpoint's OpenAPI
# schema and send that instead of `{}`. Forces requests through the parser
# and into handler logic — surfaces 5xx that validation 400s would mask.
SYNTH_BODY = os.environ.get("SYNTH_BODY", "0") not in ("0", "false", "no", "")

# tower_governor returns plain-text bodies like "Too Many Requests! Wait for 41s"
# when the per-IP bucket is empty. Parse the cooldown so we sleep just enough.
_WAIT_RE = re.compile(rb"Wait for (\d+)s")

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
        print("auth        : using TOKEN from env")
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


# =====================================================================
# OpenAPI schema validation
# =====================================================================
#
# We don't ship a full JSON-schema validator (overkill, and stdlib-only is a
# soft constraint). Instead we cover the failure modes that actually matter
# for "is the API returning sensible data":
#
#   1. envelope shape: `{code, msg, data}` always present, `code` matches the
#      HTTP status, `msg` is a non-empty string.
#   2. `data` against the response schema: required fields present, top-level
#      type matches, nested object/array shape sanity-checked one level deep.
#
# Every check returns a list of human-readable issue strings. Empty list = OK.


def resolve_ref(spec: dict, schema: dict, max_depth: int = 8) -> dict:
    """Follow `$ref` chains in a schema until we land on an inline object."""
    seen = 0
    while isinstance(schema, dict) and "$ref" in schema and seen < max_depth:
        ref = schema["$ref"]
        # Only handle local refs of the form "#/components/schemas/Name"
        if not ref.startswith("#/"):
            return schema
        node: Any = spec
        for part in ref[2:].split("/"):
            if not isinstance(node, dict) or part not in node:
                return schema
            node = node[part]
        schema = node
        seen += 1
    return schema if isinstance(schema, dict) else {}


def _matches_type(value: Any, declared: Any) -> bool:
    """OpenAPI 3.1 `type` may be a string or a list (e.g. `["string","null"]`)."""
    if declared is None:
        return True
    types = declared if isinstance(declared, list) else [declared]
    for t in types:
        if t == "null" and value is None:
            return True
        if t == "string" and isinstance(value, str):
            return True
        if t == "boolean" and isinstance(value, bool):
            return True
        if t == "integer" and isinstance(value, int) and not isinstance(value, bool):
            return True
        if t == "number" and isinstance(value, (int, float)) and not isinstance(value, bool):
            return True
        if t == "array" and isinstance(value, list):
            return True
        if t == "object" and isinstance(value, dict):
            return True
    return False


def _validate_object(spec: dict, schema: dict, value: Any, path: str) -> list[str]:
    issues: list[str] = []
    if not isinstance(value, dict):
        issues.append(f"{path}: expected object, got {type(value).__name__}")
        return issues
    for req in schema.get("required", []) or []:
        if req not in value:
            issues.append(f"{path}.{req}: required field missing")
    for k, prop_schema in (schema.get("properties") or {}).items():
        if k not in value:
            continue
        prop_schema = resolve_ref(spec, prop_schema)
        sub = _validate_value(spec, prop_schema, value[k], f"{path}.{k}", depth=1)
        issues.extend(sub)
    return issues


def _validate_value(
    spec: dict, schema: dict, value: Any, path: str, depth: int
) -> list[str]:
    """Validate a single value against a (resolved) schema, depth-limited."""
    schema = resolve_ref(spec, schema)
    if not schema:
        return []
    declared = schema.get("type")
    if declared and not _matches_type(value, declared):
        return [
            f"{path}: expected type {declared}, got "
            f"{type(value).__name__} (value sample: {repr(value)[:80]})"
        ]
    if depth >= 3:
        return []  # don't recurse forever; one or two levels is plenty
    if isinstance(value, dict):
        return _validate_object(spec, schema, value, path)
    if isinstance(value, list) and "items" in schema and value:
        # Sample the first item; if it's malformed every item probably is.
        return _validate_value(
            spec, resolve_ref(spec, schema["items"]), value[0], f"{path}[0]", depth + 1
        )
    return []


# =====================================================================
# Request-body synthesis (`SYNTH_BODY=1`)
# =====================================================================
#
# Build a minimal valid JSON body from the operation's `requestBody` schema.
# The point isn't realism — it's to satisfy server-side validation so the
# request reaches handler logic, where real bugs (panics, bad SQL,
# unhandled domain errors) actually live. With `body = {}` we usually stop
# at validator 400 and never exercise the handler.

# Heuristic stub values per JSON-schema type. Strings deliberately default
# to non-empty so `min_length=1` validators don't bounce them; integer IDs
# default to a small positive value so foreign-key lookups have a chance to
# return either Some(row) or NotFound (both non-5xx) rather than reject 0.
ID_LIKE = re.compile(r"(^|_)(id|uid|cid|did|sid|tid|aid|gid|qid|pid|bid)$|_id$|^id$")


def stub_value(spec: dict, schema: dict, name: str = "", depth: int = 0) -> object:
    schema = resolve_ref(spec, schema)
    if not schema or depth > 4:
        return None
    declared = schema.get("type")
    if isinstance(declared, list):
        # `["string","null"]` → pick the non-null variant.
        for t in declared:
            if t != "null":
                declared = t
                break
        else:
            return None
    fmt = schema.get("format", "")
    enum = schema.get("enum")
    if enum:
        return enum[0]
    if declared == "string":
        if "min_length" in schema or schema.get("minLength", 0) > 0 or fmt:
            return "x"
        return "x"
    if declared == "integer":
        # ID-shaped fields → 1; otherwise the schema's `minimum` (or 0).
        if ID_LIKE.search(name):
            return 1
        if "minimum" in schema and schema["minimum"] > 0:
            return int(schema["minimum"])
        return 0
    if declared == "number":
        return 0
    if declared == "boolean":
        return False
    if declared == "array":
        items = schema.get("items")
        if items:
            return [stub_value(spec, items, name + "[0]", depth + 1)]
        return []
    if declared == "object" or "properties" in schema:
        out: dict[str, object] = {}
        required = set(schema.get("required") or [])
        for k, prop in (schema.get("properties") or {}).items():
            if k not in required:
                continue
            out[k] = stub_value(spec, prop, k, depth + 1)
        return out
    return None


def synthesize_body(spec: dict, op: dict) -> bytes:
    """Return a minimal JSON body satisfying the op's request schema.

    Falls back to `{}` if no requestBody is declared.
    """
    schema_node = (
        op.get("requestBody", {})
        .get("content", {})
        .get("application/json", {})
        .get("schema")
    )
    if not schema_node:
        return b"{}"
    body = stub_value(spec, schema_node)
    if not isinstance(body, dict):
        body = {}
    return json.dumps(body).encode()


def validate_response(spec: dict, op: dict, status: int, body: bytes) -> list[str]:
    """Top-level validation: envelope + data shape."""
    issues: list[str] = []
    try:
        env = json.loads(body)
    except Exception as e:
        return [f"body: not valid JSON ({e})"]
    if not isinstance(env, dict):
        return [f"body: expected JSON object envelope, got {type(env).__name__}"]
    # Envelope sanity.
    if "code" not in env:
        issues.append("envelope: missing `code`")
    elif env["code"] != status:
        issues.append(f"envelope: code={env['code']} but HTTP status={status}")
    if "msg" not in env:
        issues.append("envelope: missing `msg`")
    elif not isinstance(env["msg"], str) or not env["msg"]:
        issues.append(f"envelope: msg should be non-empty string, got {env['msg']!r}")
    # `data` schema check only applies to 200s — error responses are nullable.
    if status != 200:
        return issues
    data = env.get("data")
    schema_node = (
        op.get("responses", {})
        .get(str(status), op.get("responses", {}).get("default", {}))
        .get("content", {})
        .get("application/json", {})
        .get("schema")
    )
    if not schema_node:
        return issues  # No schema declared for this response; nothing to check.
    schema = resolve_ref(spec, schema_node)
    issues.extend(_validate_value(spec, schema, data, "data", depth=0))
    return issues


# =====================================================================
# Main
# =====================================================================


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
    print(f"validate    : {'ON (envelope + data schema)' if VALIDATE else 'OFF'}")
    print()

    by_status: dict[int, list[str]] = defaultdict(list)
    server_errors: list[tuple[str, int, bytes]] = []
    schema_violations: list[tuple[str, int, list[str]]] = []

    def probe(p: str) -> tuple[str, int, bytes]:
        op = spec["paths"][p]["post"]
        body = synthesize_body(spec, op) if SYNTH_BODY else BODY
        for attempt in range(RATE_RETRIES + 1):
            st, b = post(p, body, token)
            if st != 429 or attempt == RATE_RETRIES:
                return p, st, b
            m = _WAIT_RE.search(b)
            wait = min(int(m.group(1)) + 0.2, 90.0) if m else 0.5 * (attempt + 1)
            time.sleep(wait)
        return p, st, b

    def record(p: str, st: int, b: bytes) -> None:
        by_status[st].append(p)
        if 500 <= st < 600:
            server_errors.append((p, st, b))
        if VALIDATE:
            op = spec["paths"][p]["post"]
            v = validate_response(spec, op, st, b)
            if v:
                schema_violations.append((p, st, v))

    if PARALLEL <= 1:
        for p in paths:
            _, st, b = probe(p)
            record(p, st, b)
            if DELAY > 0:
                time.sleep(DELAY)
    else:
        with ThreadPoolExecutor(max_workers=PARALLEL) as pool:
            futures = [pool.submit(probe, p) for p in paths]
            for f in as_completed(futures):
                p, st, b = f.result()
                record(p, st, b)

    # ---------- Report ----------
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

    failed = False
    if server_errors:
        failed = True
        print("❌ 5xx endpoints (real bugs):")
        for p, st, body in sorted(server_errors):
            snippet = body[:280].decode("utf-8", "replace").replace("\n", " ")
            print(f"  [{st}] {p}\n    body: {snippet}")
        print()
    else:
        print(f"✅ No 5xx responses across {len(paths)} endpoints")
        print()

    if VALIDATE:
        if schema_violations:
            failed = True
            print(
                f"❌ {len(schema_violations)} endpoint(s) returned data not matching "
                "the declared schema:"
            )
            for p, st, issues in sorted(schema_violations):
                print(f"  [{st}] {p}")
                for issue in issues[:5]:
                    print(f"    - {issue}")
                if len(issues) > 5:
                    print(f"    ... and {len(issues) - 5} more")
            print()
        else:
            valid_count = len(by_status.get(200, []))
            print(
                f"✅ All {valid_count} 200-response payload(s) match their "
                "declared OpenAPI schema"
            )
            print()

    print("===========================================")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
