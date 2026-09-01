# Rust 文档索引（给 Cursor / 改 `phpyun-rs` 时读）

长文仍在仓库原处，**不要复制一份当第二真相**。这里只分类、标明「现状 / 历史 / 去哪查」。

硬边界见 [`.cursor/rules/rust-php-boundary.mdc`](../../rules/rust-php-boundary.mdc)（always apply）。改 Rust 代码时再读 [`.cursor/rules/rust-code.mdc`](../../rules/rust-code.mdc)。

## 先读（现状）

| 主题 | 本目录精炼 | 原文（细节 / 数字以原文或代码为准） |
|---|---|---|
| 进程、端口、URL、信封 | [run.md](./run.md) | [doc/ARCHITECTURE.md](../../../doc/ARCHITECTURE.md) |
| crate 分层、新接口放哪 | [crates.md](./crates.md) | [phpyun-rs/docs/CRATE_LAYERING.md](../../../phpyun-rs/docs/CRATE_LAYERING.md) |
| 契约、快照、PHP 字符串 400 | [api.md](./api.md) | `doc/snapshots/`；OpenAPI JSON 现网 `:3003/api-docs/` |
| 后端 i18n | [i18n.md](./i18n.md) | [phpyun-rs/docs/INTERNATIONALIZATION.md](../../../phpyun-rs/docs/INTERNATIONALIZATION.md) |
| 依赖怎么数、别乱砍 | [deps.md](./deps.md) | [doc/RUST_DEPENDENCIES.md](../../../doc/RUST_DEPENDENCIES.md) |
| 本机怎么起 | [run.md](./run.md) | [`ops/restart.sh`](../../../ops/restart.sh)、[phpyun-rs/README.md](../../../phpyun-rs/README.md) |

## 历史 / 条数会过期（不要当未做清单）

| 文件 | 怎么用 |
|---|---|
| [doc/FRONTEND_BACKEND_SPLIT.md](../../../doc/FRONTEND_BACKEND_SPLIT.md) | 2026-08 方案 T0–T14；文内「没有 main.rs / admin 72 条」等**已作废** |
| [PROJECT_PLAN.md](../../../PROJECT_PLAN.md) | 「保留 PHP 后台 + 灰度」**已废弃** |
| [phpyun-rs/docs/API_V1_SUMMARY.md](../../../phpyun-rs/docs/API_V1_SUMMARY.md) | 旧口径（文内 305 条）；路径以 `doc/snapshots/` 为准 |
| [phpyun-rs/docs/API_SHAPE_ANSWERS.md](../../../phpyun-rs/docs/API_SHAPE_ANSWERS.md) | 前端问过的形状；是否已做以代码为准 |
| [doc/API_GAP.md](../../../doc/API_GAP.md)、[doc/ADMIN_PROGRESS.md](../../../doc/ADMIN_PROGRESS.md) | 盘点 / 流水账，条数过期 |
| [doc/plans/](../../../doc/plans/) | 某日执行稿，不是架构 |

后台缺口以 `web/apps/admin/app/utils/phpMap.ts` 和 `admin_php_content_service.rs` 的 `dispatch` 为准，不要从这些历史文反推。

## 运维材料（本机 job1/job2，不是通用安装包）

| 文件 | 怎么用 |
|---|---|
| [`ops/restart.sh`](../../../ops/restart.sh) | 现网重启 rust `:3003` + site `:3001`（PC/H5 + `/admin`） |
| [`ops/systemd/test-jobs-phpyun-rs-3003.service`](../../../ops/systemd/test-jobs-phpyun-rs-3003.service) | 现网 unit；binary 是 `target/debug/phpyun-rs` |
| [`ops/systemd/test-jobs-phpyun-rs.service`](../../../ops/systemd/test-jobs-phpyun-rs.service) | **已停用** 的 `:3000` |
| [phpyun-rs/deploy/INSTALL.md](../../../phpyun-rs/deploy/INSTALL.md) | **release tar 安装说明**，不是这台机的 systemd |
| [ops/T14_RETIRE_PHP.md](../../../ops/T14_RETIRE_PHP.md) | 切站 nginx 手册 |

## 契约快照（改公开路由才动）

| 文件 | 内容 |
|---|---|
| `doc/snapshots/v1_paths.txt` | App `/v1/wap` + `/v1/mcenter` 等 |
| `doc/snapshots/admin_paths.txt` | `/v1/admin` 具名路径（**不含** php-content） |
| `doc/snapshots/v1.openapi.json` / `admin.openapi.json` | 生成 TS 类型用 |

测试：`cargo test -p phpyun-handlers --test openapi_snapshot`（全量可能链接 OOM，优先 `--lib`）。
