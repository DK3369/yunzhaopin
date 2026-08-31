# 文档怎么读

**先读 [ARCHITECTURE.md](./ARCHITECTURE.md)。** 文首「完成面一览」是路径 / 端口 / URL；后文是运行拓扑与契约。不是改造任务清单。

| 文件 | 是什么 | 怎么用 |
|---|---|---|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | **现状**：进程、目录、调用链、契约、硬约束 | 改代码前读 |
| [ADMIN_PHP_TO_NUXT.md](./ADMIN_PHP_TO_NUXT.md) | 后台工作约定（`uploads/` 只读、不造页、无万能 invoke） | 仍有效 |
| [FRONTEND_BACKEND_SPLIT.md](./FRONTEND_BACKEND_SPLIT.md) | 2026-08 前后端分离 **方案**（T0–T14） | 历史；文内「现状」段落已过时 |
| [ADMIN_PROGRESS.md](./ADMIN_PROGRESS.md) | 后台实施流水账 | 看「做过什么」；条数会过期 |
| [API_GAP.md](./API_GAP.md) | 2026-08-26 PHP vs Rust 盘点 | 历史；admin 条数已过时 |
| [SMOKE_BASELINE.md](./SMOKE_BASELINE.md) | T2 冒烟记录 | 测试基线 |
| [ops/T14_RETIRE_PHP.md](../ops/T14_RETIRE_PHP.md) | 切站 nginx / systemd | 运维 |
| `snapshots/` | OpenAPI 路径快照（契约测试锁定） | 改公开路由才动 |
| `plans/` | 某日执行稿 | **不是架构**；接完即过期，勿当未映射名单 |

不要再往 `doc/plans/` 堆「下一轮提纲」当总图。缺口以 `web/apps/admin/app/utils/phpMap.ts` 和 `admin_php_content_service.rs` 的 `dispatch` 为准。
