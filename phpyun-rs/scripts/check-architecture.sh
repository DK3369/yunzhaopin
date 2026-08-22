#!/usr/bin/env bash
# check-architecture.sh — third-party isolation guard
#
# Enforces the architecture rules documented in:
#   crates/products/recruit/services/src/lib.rs   (top of file)
#   crates/products/recruit/api/src/lib.rs        (top of file)
#
# Exits 0 if no NEW violations (i.e. lines tagged `// TODO(arch):` are
# grandfathered). Exits 1 otherwise.
#
# Usage:
#   scripts/check-architecture.sh           # full repo
#   scripts/check-architecture.sh services  # only services crate
#   scripts/check-architecture.sh handlers  # only handlers crate
#
# Designed to be cheap (~50ms): just a few greps over the workspace.

set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

target="${1:-all}"
violations=0

# Crate source roots. Named here so the layout move in Phase 2 is a one-line
# edit rather than a scatter of literals through the rules below.
PLATFORM="crates/platform"
PRODUCTS="crates/products"
CORE="$PLATFORM/core/src"
AUTH="$PLATFORM/auth/src"
KERNEL="$PLATFORM/kernel/src"
MODELS="$PRODUCTS/recruit/models/src"
SERVICES="$PRODUCTS/recruit/services/src"
API="$PRODUCTS/recruit/api/src"
APP="crates/apps/recruit-server/src"

# ----------------------------------------------------------------------------
# Helpers
# ----------------------------------------------------------------------------

# Grep that excludes:
#   - `crates/platform/core/`  (the wrappers themselves live here)
#   - `crates/platform/auth/`  (a foundational crate, allowed to import primitives)
#   - lines tagged `TODO(arch)` (pre-existing violations being migrated)
#   - the line `//!` doc comment in lib.rs that mentions forbidden symbols
#   - test files (`#[cfg(test)]`, `tests/`)
report() {
    local label="$1"
    local pattern="$2"
    local scope="$3"   # "services" | "handlers" | "both"

    local paths=()
    case "$scope" in
        services) paths=("$SERVICES") ;;
        handlers) paths=("$API") ;;
        both)     paths=("$SERVICES" "$API") ;;
    esac

    local hits
    # Filters (in order):
    #   1. drop pre-grandfathered lines (`// TODO(arch): ...`)
    #   2. drop ANY comment line — doc comments (`//!`), regular (`//`),
    #      block (`/*`, ` * `) — they describe rules, they don't violate them
    #   3. drop test files
    hits=$(grep -rnE "$pattern" "${paths[@]}" --include='*.rs' 2>/dev/null \
        | grep -v 'TODO(arch)' \
        | grep -vE ':[[:space:]]*(//|/\*|\*[[:space:]])' \
        | grep -v 'tests/')

    if [ -n "$hits" ]; then
        echo "❌ $label"
        echo "$hits" | sed 's/^/    /'
        echo ""
        violations=$((violations + $(echo "$hits" | wc -l)))
    fi
}

# Same filtering as `report`, but over an explicit list of source roots.
report_in() {
    local label="$1"
    local pattern="$2"
    shift 2
    local paths=("$@")

    local hits
    hits=$(grep -rnE "$pattern" "${paths[@]}" --include='*.rs' 2>/dev/null \
        | grep -v 'TODO(arch)' \
        | grep -vE ':[[:space:]]*(//|/\*|\*[[:space:]])' \
        | grep -v 'tests/')

    if [ -n "$hits" ]; then
        echo "❌ $label"
        echo "$hits" | sed 's/^/    /'
        echo ""
        violations=$((violations + $(echo "$hits" | wc -l)))
    fi
}

report_core_contract() {
    local label="$1"
    local pattern="$2"

    local paths=()
    case "$target" in
        services) paths=("$SERVICES") ;;
        handlers) paths=("$API") ;;
        all)      paths=("$APP" "$AUTH" "$API" "$MODELS" "$SERVICES") ;;
        *)        return ;;
    esac

    local hits
    hits=$(grep -rnE "$pattern" "${paths[@]}" --include='*.rs' 2>/dev/null \
        | grep -v 'TODO(arch)' \
        | grep -vE ':[[:space:]]*(//|/\*|\*[[:space:]])' \
        | grep -v 'tests/')

    if [ -n "$hits" ]; then
        echo "❌ $label"
        echo "$hits" | sed 's/^/    /'
        echo ""
        violations=$((violations + $(echo "$hits" | wc -l)))
    fi
}

# ----------------------------------------------------------------------------
# Rules
# ----------------------------------------------------------------------------

echo "→ Auditing core public error contract..."
report_core_contract "application crates must use ApiError constructors instead of ad-hoc construction" \
    '\bApiError::new'
report_core_contract "application crates may not reference phpyun_core::error::* directly" \
    '\bphpyun_core::error::'

if [ "$target" = "all" ] || [ "$target" = "services" ]; then
    echo "→ Auditing services/ for direct third-party imports..."
    # Patterns match both `use X::Y` AND `X::Y` type paths.
    report "services may not reference moka::* (use phpyun_core::cache::SimpleCache)" \
        '\bmoka::' services
    report "services may not reference redis::* (use phpyun_core::kv / events)" \
        '\bredis::' services
    report "services may not reference reqwest::* (use phpyun_core::http_client)" \
        '\breqwest::' services
    report "services may not reference jsonwebtoken::* (use phpyun_core::jwt)" \
        '\bjsonwebtoken::' services
    report "services may not reference cron::* (use phpyun_core::scheduler)" \
        '\bcron::' services
    report "services should not write raw 'sqlx::query*' (move to models/*/repo)" \
        '\bsqlx::query' services
fi

if [ "$target" = "all" ] || [ "$target" = "handlers" ]; then
    echo "→ Auditing handlers/ for direct third-party imports..."
    report "handlers may not reference sqlx::* (call services, which call repos)" \
        '\bsqlx::query' handlers
    report "handlers may not reference moka::* (cache lives in services)" \
        '\bmoka::' handlers
    report "handlers may not reference redis::* (kv access lives in services)" \
        '\bredis::' handlers
    report "handlers may not reference reqwest::* (http out lives in services)" \
        '\breqwest::' handlers
fi

if [ "$target" = "all" ] || [ "$target" = "layering" ]; then
    echo "→ Auditing the transport boundary..."

    # The whole point of the kernel is that one handler can be driven by HTTP
    # today and a queue tomorrow. That only holds while business code names no
    # transport type. `api/` is exempt: it still holds the 482 legacy axum
    # handlers, which migrate to `Operation` gradually.
    report_in "products' models/services must not name a transport type (business code stays protocol-agnostic)" \
        '\b(axum|tonic|tungstenite|hyper)::' "$MODELS" "$SERVICES"
    report_in "products must not depend on a transport crate (depend on phpyun_kernel instead)" \
        '\bphpyun_transport_' "$MODELS" "$SERVICES" "$API"

    # If the kernel ever grows an HTTP dependency, the multi-protocol design is
    # over — every future transport would inherit axum's request model.
    report_in "the kernel must not name a transport type (it is protocol-agnostic by definition)" \
        '\b(axum|tonic|tungstenite|hyper)::' "$KERNEL"
fi

# ----------------------------------------------------------------------------
# Result
# ----------------------------------------------------------------------------

if [ $violations -eq 0 ]; then
    echo "✅ No new architecture violations."
    exit 0
else
    echo "✗ $violations violation(s) found."
    echo ""
    echo "How to fix:"
    echo "  • Real fix: refactor into the proper layer (see lib.rs of each crate)."
    echo "  • Grandfather an existing line: add a trailing comment '// TODO(arch): <reason>'"
    echo "    so this script ignores it while you migrate elsewhere."
    exit 1
fi
