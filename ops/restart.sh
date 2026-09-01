#!/usr/bin/env bash
# 现网前后端重启（本机 job1/job2）。
# 只管 :3003（Rust）+ :3001（site Nitro：PC/H5 + /admin）。
# admin Nitro 走 unix socket，不占 TCP。
# 绝不 start/enable 旧 :3000（test-jobs-phpyun-rs）。
# 不要再开 :3002 / :3004 / :3005；占用 3001 就杀掉再绑。
set -Eeuo pipefail

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(CDPATH= cd -- "${SCRIPT_DIR}/.." && pwd)"
UNIT_DIR="${ROOT}/ops/systemd"
RS_DIR="${ROOT}/phpyun-rs"
WEB_DIR="${ROOT}/web"

UNIT_RS="test-jobs-phpyun-rs-3003"
UNIT_SITE="test-jobs-phpyun-site"
UNIT_ADMIN="test-jobs-phpyun-admin"
UNIT_ADMIN_EDGE="test-jobs-phpyun-admin-edge"
RETIRED_RS="test-jobs-phpyun-rs"

PORT_RS=3003
PORT_SITE=3001
ADMIN_SOCK="${ADMIN_SOCK:-/var/tmp/phpyun-admin.sock}"
RETIRED_WEB_PORTS=(3002 3004 3005)

CARGO_TMP="${CARGO_TMP:-/var/tmp/cargo-tmp}"
NPM_TMP="${NPM_TMP:-/var/tmp/npm-tmp}"
ADMIN_N_PREV="${ADMIN_N_PREV:-/var/tmp/phpyun-admin-n-prev}"
NODE_BIN="${NODE_BIN:-}"

DO_BUILD=0
CARGO_OFFLINE=1
DO_VERIFY=1
TARGET="all"

if [[ "${EUID}" -eq 0 ]]; then
  SUDO=()
else
  SUDO=(sudo)
fi

fail() { printf 'error: %s\n' "$*" >&2; exit 1; }
log() { printf '==> %s\n' "$*"; }

usage() {
  cat <<'EOF'
用法:
  ops/restart.sh                 重启 rust + site + admin（不编译）
  ops/restart.sh all --build     先编译再重启全部
  ops/restart.sh rust            只重启 API :3003
  ops/restart.sh rust --build    cargo build 后再重启 :3003
  ops/restart.sh site            只重启前台 :3001（先确保 admin unix socket）
  ops/restart.sh admin           只重启后台 Nitro（unix socket，TCP 仍 :3001）
  ops/restart.sh frontend        重启 site + admin
  ops/restart.sh frontend --build
  ops/restart.sh status          看 systemd / 端口 / HTTP

选项:
  --build      重启前编译（rust: cargo；nuxt: pnpm build）
  --online     cargo 允许联网（默认 --offline）
  --no-verify  跳过 HTTP 探活

不会动旧 :3000（test-jobs-phpyun-rs）。改 PHP 原项目请不要用本脚本当借口。
Web TCP 只有 :3001，API 只有 :3003。占用 3001 就杀掉再绑，不要新建 TCP 端口。
admin 不占 3002/3005。
EOF
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "找不到命令: $1"
}

resolve_node_bin() {
  if [[ -n "${NODE_BIN}" && -x "${NODE_BIN}/node" ]]; then
    return 0
  fi
  local d
  for d in \
    /var/tmp/node-dist/node-v22.22.1-linux-arm64/bin \
    /var/tmp/node-dist/node-v22*-linux-*/bin
  do
    if [[ -x "${d}/node" ]]; then
      NODE_BIN="${d}"
      return 0
    fi
  done
  fail "找不到 Node 22 目录（/var/tmp/node-dist/node-v22*-linux-*/bin）"
}

sys() {
  "${SUDO[@]}" systemctl "$@"
}

http_code() {
  local url="$1"
  curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "${url}" 2>/dev/null || printf '000'
}

http_ok() {
  case "$1" in
    200|301|302|304) return 0 ;;
    *) return 1 ;;
  esac
}

listen_pids() {
  local port="$1"
  ss -H -ltnp "sport = :${port}" 2>/dev/null \
    | grep -oE 'pid=[0-9]+' \
    | cut -d= -f2 \
    | sort -u \
    || true
}

unit_main_pid() {
  local pid
  pid="$(sys show -p MainPID --value "$1" 2>/dev/null || printf '0')"
  [[ -n "${pid}" ]] || pid=0
  printf '%s' "${pid}"
}

kill_pid() {
  local pid="$1"
  [[ "${pid}" =~ ^[1-9][0-9]*$ ]] || return 0
  [[ "${pid}" == "1" ]] && return 0
  kill "${pid}" 2>/dev/null || true
  sleep 0.4
  if kill -0 "${pid}" 2>/dev/null; then
    kill -KILL "${pid}" 2>/dev/null || true
  fi
}

free_port() {
  local port="$1"
  local pid
  local left
  for pid in $(listen_pids "${port}"); do
    log "清端口 :${port} 残留 pid=${pid}"
    kill_pid "${pid}"
  done
  left="$(listen_pids "${port}")"
  if [[ -n "${left}" ]]; then
    log "sudo fuser -k ${port}/tcp"
    "${SUDO[@]}" fuser -k "${port}/tcp" >/dev/null 2>&1 || true
    sleep 0.3
  fi
  left="$(listen_pids "${port}")"
  if [[ -n "${left}" ]]; then
    fail "端口 :${port} 仍被占用: ${left}"
  fi
}

install_unit() {
  local name="$1"
  local src="${UNIT_DIR}/${name}.service"
  local dst="/etc/systemd/system/${name}.service"
  [[ -f "${src}" ]] || fail "缺少 unit 文件: ${src}"
  if [[ ! -f "${dst}" ]] || ! cmp -s "${src}" "${dst}"; then
    log "安装 systemd unit ${name}"
    "${SUDO[@]}" cp "${src}" "${dst}"
    NEED_DAEMON_RELOAD=1
  fi
}

ensure_units() {
  NEED_DAEMON_RELOAD=0
  install_unit "${UNIT_RS}"
  install_unit "${UNIT_SITE}"
  install_unit "${UNIT_ADMIN}"
  if [[ "${NEED_DAEMON_RELOAD}" -eq 1 ]]; then
    log "systemctl daemon-reload"
    sys daemon-reload
  fi
  sys enable "${UNIT_RS}" "${UNIT_SITE}" "${UNIT_ADMIN}" >/dev/null
  if sys is-enabled --quiet "${UNIT_ADMIN_EDGE}" 2>/dev/null; then
    log "disable ${UNIT_ADMIN_EDGE}"
    sys disable "${UNIT_ADMIN_EDGE}" >/dev/null || true
  fi
  if sys is-enabled --quiet "${RETIRED_RS}" 2>/dev/null; then
    log "旧 ${RETIRED_RS} 仍是 enabled，正在 disable（不 stop，避免误伤回退机）"
    sys disable "${RETIRED_RS}" >/dev/null || true
  fi
  if sys is-active --quiet "${RETIRED_RS}" 2>/dev/null; then
    printf 'warn: 旧 :3000（%s）正在跑。本脚本不会 stop/start 它。API 应走 :3003。\n' "${RETIRED_RS}" >&2
  fi
}

# 停掉曾占 TCP 的 edge / 旧端口。admin 改 unix socket，不要杀它的 unit。
retire_extra_web() {
  local port
  if sys is-enabled --quiet "${UNIT_ADMIN_EDGE}" 2>/dev/null; then
    log "disable ${UNIT_ADMIN_EDGE}"
    sys disable "${UNIT_ADMIN_EDGE}" >/dev/null || true
  fi
  if sys is-active --quiet "${UNIT_ADMIN_EDGE}" 2>/dev/null; then
    log "停止 ${UNIT_ADMIN_EDGE}"
    sys stop "${UNIT_ADMIN_EDGE}" || true
  fi
  for port in "${RETIRED_WEB_PORTS[@]}"; do
    if [[ -n "$(listen_pids "${port}")" ]]; then
      log "释放旧端口 :${port}"
      free_port "${port}"
    fi
  done
}

bounce_admin_sock() {
  log "停止 ${UNIT_ADMIN}"
  sys stop "${UNIT_ADMIN}" || true
  sleep 0.3
  rm -f "${ADMIN_SOCK}"
  log "启动 ${UNIT_ADMIN}（unix ${ADMIN_SOCK}）"
  sys start "${UNIT_ADMIN}"
  local i
  for i in $(seq 1 20); do
    if sys is-active --quiet "${UNIT_ADMIN}" && [[ -S "${ADMIN_SOCK}" ]]; then
      return 0
    fi
    sleep 0.4
  done
  sys status --no-pager -l "${UNIT_ADMIN}" >&2 || true
  fail "${UNIT_ADMIN} 未能监听 ${ADMIN_SOCK}"
}

ensure_admin_sock() {
  if sys is-active --quiet "${UNIT_ADMIN}" 2>/dev/null && [[ -S "${ADMIN_SOCK}" ]]; then
    return 0
  fi
  bounce_admin_sock
}

bounce_unit() {
  local unit="$1"
  local port="$2"
  log "停止 ${unit}"
  sys stop "${unit}" || true
  sleep 0.4
  free_port "${port}"
  log "启动 ${unit}"
  sys start "${unit}"
  local i
  for i in $(seq 1 20); do
    if sys is-active --quiet "${unit}"; then
      break
    fi
    sleep 0.5
  done
  if ! sys is-active --quiet "${unit}"; then
    sys status --no-pager -l "${unit}" >&2 || true
    fail "${unit} 未能进入 active"
  fi
  for i in $(seq 1 20); do
    if [[ -n "$(listen_pids "${port}")" ]]; then
      return 0
    fi
    sleep 0.3
  done
  log "warn: ${unit} active 但 :${port} 尚未监听"
}

wait_http() {
  local name="$1"
  local url="$2"
  local i code
  for i in $(seq 1 30); do
    code="$(http_code "${url}")"
    if http_ok "${code}"; then
      log "${name} ${url} → ${code}"
      return 0
    fi
    sleep 1
  done
  fail "${name} 探活失败: ${url} → ${code:-000}"
}

build_rust() {
  need_cmd cargo
  mkdir -p "${CARGO_TMP}" "${RS_DIR}/target"
  export TMPDIR="${CARGO_TMP}"
  export CARGO_TARGET_DIR="${RS_DIR}/target"
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
  local extra=()
  if [[ "${CARGO_OFFLINE}" -eq 1 ]]; then
    extra+=(--offline)
  fi
  log "cargo build -p phpyun-rs ${extra[*]:-} -j ${CARGO_BUILD_JOBS}"
  (
    cd "${RS_DIR}"
    cargo build -p phpyun-rs "${extra[@]}" -j "${CARGO_BUILD_JOBS}"
  )
}

build_nuxt() {
  local filter="$1"
  local tag
  local pub d name
  resolve_node_bin
  mkdir -p "${NPM_TMP}"
  export PATH="${NODE_BIN}:${HOME}/.cargo/bin:${PATH}"
  export TMPDIR="${NPM_TMP}"
  need_cmd pnpm
  tag="$(git -C "${ROOT}" rev-parse --short HEAD)"
  pub="${WEB_DIR}/apps/admin/.output/public/_n"
  if [[ "${filter}" == "@phpyun/admin" && -d "${pub}" ]]; then
    rm -rf "${ADMIN_N_PREV}"
    cp -a "${pub}" "${ADMIN_N_PREV}"
  fi
  log "pnpm --filter ${filter} build  (ADMIN_ASSET_TAG=${tag})"
  (
    cd "${WEB_DIR}"
    ADMIN_ASSET_TAG="${tag}" pnpm --filter "${filter}" build
  )
  if [[ "${filter}" == "@phpyun/admin" && -d "${ADMIN_N_PREV}" ]]; then
    mkdir -p "${pub}"
    for d in "${ADMIN_N_PREV}"/*; do
      [[ -d "${d}" ]] || continue
      name="$(basename "${d}")"
      if [[ ! -d "${pub}/${name}" ]]; then
        log "保留上一版 hashed 资源 _n/${name}"
        cp -a "${d}" "${pub}/${name}"
      fi
    done
  fi
}

verify_admin() {
  wait_http "admin" "http://127.0.0.1:${PORT_SITE}/admin/login"
  local href code ctype
  href="$(curl -sS --max-time 5 "http://127.0.0.1:${PORT_SITE}/admin/login" | grep -oE '/admin/_n/[^" ]+\.css' | head -1 || true)"
  if [[ -z "${href}" ]]; then
    return 0
  fi
  code="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "http://127.0.0.1:${PORT_SITE}${href}" || printf '000')"
  ctype="$(curl -sS -o /dev/null -w '%{content_type}' --max-time 5 "http://127.0.0.1:${PORT_SITE}${href}" || true)"
  log "admin css ${href} → ${code} ${ctype}"
  if [[ "${code}" != "200" || "${ctype}" != text/css* ]]; then
    fail "admin hashed css 未就绪: ${href} → ${code} ${ctype}"
  fi
}

do_status() {
  local unit port st pids
  printf '%-32s %-10s %-8s %s\n' "unit" "active" "port" "listen"
  for unit in "${UNIT_RS}" "${UNIT_SITE}"; do
    st="inactive"
    pids="-"
    case "${unit}" in
      "${UNIT_RS}") port="${PORT_RS}" ;;
      "${UNIT_SITE}") port="${PORT_SITE}" ;;
    esac
    if sys is-active --quiet "${unit}"; then
      st="active"
    fi
    pids="$(listen_pids "${port}")"
    [[ -n "${pids}" ]] || pids="-"
    printf '%-32s %-10s :%-7s %s\n' "${unit}" "${st}" "${port}" "${pids}"
  done
  st="inactive"
  if sys is-active --quiet "${UNIT_ADMIN}" 2>/dev/null; then
    st="active"
  fi
  if [[ -S "${ADMIN_SOCK}" ]]; then
    printf '%-32s %-10s %-8s %s\n' "${UNIT_ADMIN}" "${st}" "unix" "${ADMIN_SOCK}"
  else
    printf '%-32s %-10s %-8s %s\n' "${UNIT_ADMIN}" "${st}" "unix" "（socket 未就绪）"
  fi
  if sys is-active --quiet "${UNIT_ADMIN_EDGE}" 2>/dev/null; then
    printf '%-32s %-10s %-8s %s\n' "${UNIT_ADMIN_EDGE}" "active" "-" "（应已停）"
  fi
  if sys is-active --quiet "${RETIRED_RS}" 2>/dev/null; then
    printf '%-32s %-10s :%-7s %s\n' "${RETIRED_RS}" "active" "3000" "（旧栈，不应在跑）"
  fi
  printf '\n'
  printf 'HTTP\n'
  printf '  rust   /health          %s\n' "$(http_code "http://127.0.0.1:${PORT_RS}/health")"
  printf '  web    /                %s\n' "$(http_code "http://127.0.0.1:${PORT_SITE}/")"
  printf '  admin  /admin/login     %s\n' "$(http_code "http://127.0.0.1:${PORT_SITE}/admin/login")"
}

verify_rust() { wait_http "rust" "http://127.0.0.1:${PORT_RS}/health"; }
verify_site() { wait_http "site" "http://127.0.0.1:${PORT_SITE}/"; }

restart_rust() {
  if [[ "${DO_BUILD}" -eq 1 ]]; then
    build_rust
  fi
  bounce_unit "${UNIT_RS}" "${PORT_RS}"
  if [[ "${DO_VERIFY}" -eq 1 ]]; then
    verify_rust
  fi
}

restart_site() {
  if [[ "${DO_BUILD}" -eq 1 ]]; then
    build_nuxt "@phpyun/site"
  fi
  retire_extra_web
  ensure_admin_sock
  bounce_unit "${UNIT_SITE}" "${PORT_SITE}"
  if [[ "${DO_VERIFY}" -eq 1 ]]; then
    verify_site
    verify_admin
  fi
}

restart_admin() {
  if [[ "${DO_BUILD}" -eq 1 ]]; then
    build_nuxt "@phpyun/admin"
  fi
  bounce_admin_sock
  if [[ "${DO_VERIFY}" -eq 1 ]]; then
    verify_admin
  fi
}

# --- args ---
while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help|help)
      usage
      exit 0
      ;;
    --build)
      DO_BUILD=1
      shift
      ;;
    --online)
      CARGO_OFFLINE=0
      shift
      ;;
    --offline)
      CARGO_OFFLINE=1
      shift
      ;;
    --no-verify)
      DO_VERIFY=0
      shift
      ;;
    all|rust|site|admin|frontend|status)
      TARGET="$1"
      shift
      ;;
    3000|:3000|test-jobs-phpyun-rs)
      fail "禁止操作旧 :3000（${RETIRED_RS}）"
      ;;
    -*)
      fail "未知选项: $1（见 --help）"
      ;;
    *)
      fail "未知目标: $1（all|rust|site|admin|frontend|status）"
      ;;
  esac
done

need_cmd curl
need_cmd ss
need_cmd systemctl

if [[ "${TARGET}" == "status" ]]; then
  do_status
  exit 0
fi

ensure_units

case "${TARGET}" in
  rust)
    restart_rust
    ;;
  site)
    restart_site
    ;;
  admin)
    restart_admin
    ;;
  frontend)
    if [[ "${DO_BUILD}" -eq 1 ]]; then
      build_nuxt "@phpyun/site"
      build_nuxt "@phpyun/admin"
      DO_BUILD=0
    fi
    retire_extra_web
    bounce_admin_sock
    bounce_unit "${UNIT_SITE}" "${PORT_SITE}"
    if [[ "${DO_VERIFY}" -eq 1 ]]; then
      verify_site
      verify_admin
    fi
    ;;
  all)
    if [[ "${DO_BUILD}" -eq 1 ]]; then
      build_rust
      build_nuxt "@phpyun/site"
      build_nuxt "@phpyun/admin"
      DO_BUILD=0
    fi
    bounce_unit "${UNIT_RS}" "${PORT_RS}"
    restart_site
    if [[ "${DO_VERIFY}" -eq 1 ]]; then
      verify_rust
    fi
    ;;
  *)
    fail "未知目标: ${TARGET}"
    ;;
esac

log "完成"
do_status
