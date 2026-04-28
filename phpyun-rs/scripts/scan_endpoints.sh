#!/usr/bin/env bash
# Automated endpoint scan against a running phpyun-rs server.
#
# Reads the live OpenAPI spec, logs in to obtain a fresh access token, then
# POSTs `{}` to every documented POST path with that Bearer token. Buckets
# every response by status code, prints a 5xx report (real bugs), and exits
# non-zero if any 5xx is found.
#
# Usage:
#   ./scripts/scan_endpoints.sh                                  # default host + creds
#   HOST=http://127.0.0.1:3000 USER=duncan11 PASS=dd112211 ./scripts/scan_endpoints.sh
#   TOKEN=eyJ0eXA... ./scripts/scan_endpoints.sh                 # bring your own token
#
# Requires: curl + jq (sudo apt-get install jq if missing).

set -u -o pipefail

HOST="${HOST:-http://127.0.0.1:3000}"
USER="${USER:-duncan11}"
PASS="${PASS:-dd112211}"
LOGIN_PATH="${LOGIN_PATH:-/v1/wap/login}"
SPEC_PATH="${SPEC_PATH:-/api-docs/v1/openapi.json}"
BODY="${BODY:-{}}"

# Where to dump artifacts.
TMPDIR="$(mktemp -d -t phpyun-scan-XXXXXX)"
trap 'rm -rf "$TMPDIR"' EXIT
SPEC="$TMPDIR/openapi.json"
REPORT="$TMPDIR/report.tsv"

echo "scan target : $HOST"
echo "tmpdir      : $TMPDIR"

# ---------- 1. Token ----------
if [[ -n "${TOKEN:-}" ]]; then
  echo "auth        : using TOKEN from env"
else
  LOGIN_RESP=$(curl -s -X POST "$HOST$LOGIN_PATH" \
    -H 'content-type: application/json' \
    -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}")
  TOKEN=$(printf '%s' "$LOGIN_RESP" | jq -r '.data.access_token // empty')
  if [[ -z "$TOKEN" ]]; then
    echo "❌ login failed:"
    printf '%s\n' "$LOGIN_RESP" | head -c 400
    echo
    exit 2
  fi
  USERTYPE=$(printf '%s' "$LOGIN_RESP" | jq -r '.data.usertype // empty')
  UID_=$(printf '%s' "$LOGIN_RESP" | jq -r '.data.uid // empty')
  echo "auth        : login ok, uid=$UID_, usertype=$USERTYPE"
fi

# ---------- 2. Spec ----------
HTTP=$(curl -s -o "$SPEC" -w '%{http_code}' "$HOST$SPEC_PATH")
if [[ "$HTTP" != "200" ]]; then
  echo "❌ openapi spec fetch failed (HTTP $HTTP)"; exit 3
fi
TOTAL_PATHS=$(jq '.paths | length' "$SPEC")
echo "openapi     : $TOTAL_PATHS path(s) discovered"

# ---------- 3. Probe ----------
> "$REPORT"
mapfile -t PATHS < <(jq -r '.paths | to_entries[] | select(.value.post) | .key' "$SPEC" | sort -u)

declare -A BUCKETS=()
SERVER_ERRORS=()

i=0
for P in "${PATHS[@]}"; do
  i=$((i+1))
  RESP_FILE="$TMPDIR/resp.$i"
  STATUS=$(curl -s -o "$RESP_FILE" -w '%{http_code}' \
    -X POST "$HOST$P" \
    -H 'content-type: application/json' \
    -H "authorization: Bearer $TOKEN" \
    --data-raw "$BODY")
  printf '%s\t%s\n' "$STATUS" "$P" >> "$REPORT"
  BUCKETS["$STATUS"]=$((${BUCKETS["$STATUS"]:-0}+1))
  if [[ "$STATUS" =~ ^5 ]]; then
    BODY_SNIPPET=$(head -c 280 "$RESP_FILE" | tr '\n' ' ')
    SERVER_ERRORS+=("[$STATUS] $P :: $BODY_SNIPPET")
  fi
done

# ---------- 4. Report ----------
echo
echo "========== ENDPOINT SCAN REPORT =========="
echo "Total POST endpoints probed: ${#PATHS[@]}"
for K in $(printf '%s\n' "${!BUCKETS[@]}" | sort -n); do
  printf '  HTTP %-3s : %3d endpoint(s)\n' "$K" "${BUCKETS[$K]}"
done
echo
if (( ${#SERVER_ERRORS[@]} > 0 )); then
  echo "❌ 5xx endpoints (real bugs):"
  for line in "${SERVER_ERRORS[@]}"; do
    echo "  $line"
  done
  echo "==========================================="
  exit 1
fi
echo "✅ No 5xx responses across ${#PATHS[@]} endpoints"
echo "==========================================="
