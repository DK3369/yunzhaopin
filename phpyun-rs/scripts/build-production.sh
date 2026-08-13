#!/usr/bin/env bash
set -Eeuo pipefail

TARGET="x86_64-unknown-linux-gnu"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(CDPATH= cd -- "${SCRIPT_DIR}/.." && pwd)"
DIST_DIR="${ROOT_DIR}/dist"

fail() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

for command_name in cargo rustc git tar sha256sum file awk find sort xargs install grep; do
    command -v "${command_name}" >/dev/null 2>&1 \
        || fail "required command not found: ${command_name}"
done

[[ "$(uname -s)" == "Linux" ]] || fail "production packages must be built on Linux"
[[ "$(uname -m)" == "x86_64" ]] \
    || fail "native x86_64 builder required (current architecture: $(uname -m))"

RUST_HOST="$(rustc -vV | awk '/^host:/ { print $2 }')"
[[ "${RUST_HOST}" == "${TARGET}" ]] \
    || fail "Rust host must be ${TARGET} (current host: ${RUST_HOST:-unknown})"
git -C "${ROOT_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1 \
    || fail "${ROOT_DIR} is not inside a Git worktree"

DIRTY_STATUS="$(git -C "${ROOT_DIR}" status --porcelain -- .)"
if [[ -n "${DIRTY_STATUS}" && "${ALLOW_DIRTY:-0}" != "1" ]]; then
    printf '%s\n' "${DIRTY_STATUS}" >&2
    fail "tracked or untracked project changes found; commit them or set ALLOW_DIRTY=1"
fi

VERSION="$(awk -F '"' '/^version = "/ { print $2; exit }' "${ROOT_DIR}/Cargo.toml")"
[[ -n "${VERSION}" ]] || fail "unable to read workspace version from Cargo.toml"
COMMIT="$(git -C "${ROOT_DIR}" rev-parse --short=12 HEAD)"
BUILD_REF="${COMMIT}"
if [[ -n "${DIRTY_STATUS}" ]]; then
    BUILD_REF="${BUILD_REF}-dirty"
fi

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
    case "${CARGO_TARGET_DIR}" in
        /*) TARGET_DIR="${CARGO_TARGET_DIR}" ;;
        *) TARGET_DIR="${ROOT_DIR}/${CARGO_TARGET_DIR}" ;;
    esac
else
    TARGET_DIR="${ROOT_DIR}/target"
fi
export CARGO_TARGET_DIR="${TARGET_DIR}"

printf '==> Checking formatting\n'
cargo fmt --all -- --check

printf '==> Running workspace library tests\n'
cargo test --workspace --lib --locked

printf '==> Building %s release binary\n' "${TARGET}"
cargo build --release --locked --target "${TARGET}" -p phpyun-app --bin app

BINARY="${TARGET_DIR}/${TARGET}/release/app"
[[ -x "${BINARY}" ]] || fail "release binary not found: ${BINARY}"
BINARY_INFO="$(file -b "${BINARY}")"
printf '%s\n' "${BINARY_INFO}" | grep -Eq 'ELF 64-bit.*x86-64' \
    || fail "unexpected release binary format: ${BINARY_INFO}"

PACKAGE_NAME="phpyun-rs-${VERSION}-${BUILD_REF}-${TARGET}"
ARCHIVE_NAME="${PACKAGE_NAME}.tar.gz"
STAGE_DIR="$(mktemp -d /tmp/phpyun-rs-package.XXXXXX)"
cleanup() {
    case "${STAGE_DIR}" in
        /tmp/phpyun-rs-package.*) rm -rf -- "${STAGE_DIR}" ;;
    esac
}
trap cleanup EXIT

PACKAGE_DIR="${STAGE_DIR}/${PACKAGE_NAME}"
install -d "${PACKAGE_DIR}/bin" "${PACKAGE_DIR}/config" \
    "${PACKAGE_DIR}/migrations" "${PACKAGE_DIR}/systemd"
install -m 0755 "${BINARY}" "${PACKAGE_DIR}/bin/phpyun-rs"
install -m 0644 "${ROOT_DIR}/.env.pro.example" \
    "${PACKAGE_DIR}/config/.env.pro.example"
cp -a "${ROOT_DIR}/migrations/sqlx" "${PACKAGE_DIR}/migrations/"
install -m 0644 "${ROOT_DIR}/deploy/systemd/phpyun-rs.service" \
    "${PACKAGE_DIR}/systemd/phpyun-rs.service"
install -m 0644 "${ROOT_DIR}/deploy/INSTALL.md" "${PACKAGE_DIR}/INSTALL.md"

BUILD_TIME="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
RUSTC_VERSION="$(rustc --version)"
{
    printf 'name=%s\n' "phpyun-rs"
    printf 'version=%s\n' "${VERSION}"
    printf 'commit=%s\n' "${COMMIT}"
    printf 'dirty=%s\n' "$([[ -n "${DIRTY_STATUS}" ]] && printf true || printf false)"
    printf 'target=%s\n' "${TARGET}"
    printf 'built_at=%s\n' "${BUILD_TIME}"
    printf 'rustc=%s\n' "${RUSTC_VERSION}"
    printf 'binary=%s\n' "${BINARY_INFO}"
} > "${PACKAGE_DIR}/BUILD-MANIFEST.txt"

(
    cd "${PACKAGE_DIR}"
    find . -type f ! -name SHA256SUMS -print0 \
        | LC_ALL=C sort -z \
        | xargs -0 sha256sum > SHA256SUMS
)

mkdir -p "${DIST_DIR}"
tar -C "${STAGE_DIR}" -czf "${DIST_DIR}/${ARCHIVE_NAME}" "${PACKAGE_NAME}"
(
    cd "${DIST_DIR}"
    sha256sum "${ARCHIVE_NAME}" > "${ARCHIVE_NAME}.sha256"
)

printf '==> Package ready\n%s\n%s\n' \
    "${DIST_DIR}/${ARCHIVE_NAME}" "${DIST_DIR}/${ARCHIVE_NAME}.sha256"
