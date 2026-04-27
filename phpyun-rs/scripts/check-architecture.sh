#!/usr/bin/env bash
# check-architecture.sh — third-party isolation guard
#
# Enforces the architecture rules documented in:
#   crates/services/src/lib.rs   (top of file)
#   crates/handlers/src/lib.rs   (top of file)
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

# ----------------------------------------------------------------------------
# Helpers
# ----------------------------------------------------------------------------

# Grep that excludes:
#   - `crates/core/`           (the wrappers themselves live here)
#   - `crates/auth/`           (a foundational crate, allowed to import primitives)
#   - lines tagged `TODO(arch)` (pre-existing violations being migrated)
#   - the line `//!` doc comment in lib.rs that mentions forbidden symbols
#   - test files (`#[cfg(test)]`, `tests/`)
report() {
    local label="$1"
    local pattern="$2"
    local scope="$3"   # "services" | "handlers" | "both"

    local paths=()
    case "$scope" in
        services) paths=(crates/services/src) ;;
        handlers) paths=(crates/handlers/src) ;;
        both)     paths=(crates/services/src crates/handlers/src) ;;
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

# ----------------------------------------------------------------------------
# Rules
# ----------------------------------------------------------------------------

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
