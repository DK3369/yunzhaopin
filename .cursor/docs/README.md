# Cursor 文档（给后续对话读）

有实质业务落地后写进这里，不要只留在 chat 或 `.cursor/plans/`。  
**细节以代码为准**；这里只写现状口径、禁做项、文件入口。

硬边界仍看 [`.cursor/rules/rust-php-boundary.mdc`](../rules/rust-php-boundary.mdc)。怎么写文档看 [`.cursor/rules/cursor-docs.mdc`](../rules/cursor-docs.mdc)。

## 先读

| 主题 | 文件 |
|---|---|
| **整机地图**（进程 / nginx / 禁做 / 读什么） | [overview.md](./overview.md) |
| **加功能配方**（新接口 / 新页 / i18n） | [dev-playbook.md](./dev-playbook.md) |
| Web 前台（BFF / PC·H5 / 会员） | [web/README.md](./web/README.md) |
| Site 页面域 → API | [web/routes.md](./web/routes.md) |
| 管理后台（phpMap / unix socket） | [web/admin.md](./web/admin.md) |
| Rust 进程 / 分层 / 契约 / 编译 | [rust/README.md](./rust/README.md) |

长文拓扑：[doc/ARCHITECTURE.md](../../doc/ARCHITECTURE.md)。`doc/README.md` 有历史文阅读顺序。

## 专题

| 主题 | 文件 |
|---|---|
| 已合并 / 已摘除 deprecated 接口 | [rust/api-merged.md](./rust/api-merged.md) |
| 后端 i18n（四层：Vue / Rust msg / dict_i18n / packed） | [rust/i18n.md](./rust/i18n.md) |
| 横切（登录态 / locale 缓存 key / 媒体 URL / settings 缓存） | [rust/cross-cut.md](./rust/cross-cut.md) |
| 公开只读两级缓存（L1 moka + Redis） | [rust/cache.md](./rust/cache.md) |
| 鉴权信任链（usertype=9 / admin 回查） | [rust/security.md](./rust/security.md) |
| 会员私信（HTTP `/v1/mcenter/chat/*`，表 `phpyun_rs_chat`） | [rust/chat.md](./rust/chat.md) |
| Flutter 缺口 + 两张 RS 表（身份证实名 / 面试评价 / 栏目订阅 / 简历删除） | [rust/app-api-gaps.md](./rust/app-api-gaps.md) |
| 采集英文岗、补 JD、官网申请记原投递表 | [features/job-scrape.md](./features/job-scrape.md) |
| PC/H5 会员中心套肤（求职 `/user` · 招聘 `/com`） | [features/member-center.md](./features/member-center.md) |
| PC 页脚五列与关于我们/排行榜/订阅落地页 | [features/pc-footer.md](./features/pc-footer.md) |
| 字段编码 UTF-8 / MySQL utf8mb4 | [`.cursor/rules/utf8.mdc`](../rules/utf8.mdc) |
| API 标识符 / SQL 拼接 / 富文本清洗 | [`.cursor/rules/api-naming.mdc`](../rules/api-naming.mdc) |

## 不要当现状的

| 路径 | 怎么用 |
|---|---|
| `.cursor/plans/` | 某次执行稿；未落地的条目不要当已做 |
| 仓库根 `SCHEMA_BUGS_BACKLOG.md` | 旧盘点；`phpyun_rs_*` 以 [rust/crates.md](./rust/crates.md) 为准 |
| `PROJECT_PLAN.md`、`doc/FRONTEND_BACKEND_SPLIT.md` | 方案 /「保留 PHP」已废弃；文内「现状」过时 |
| `doc/API_GAP.md`、`doc/ADMIN_PROGRESS.md`、`doc/plans/` | 盘点 / 流水账，条数会过期。后台缺口以 `phpMap.ts` + `admin_php_content_service` 为准 |
| 会员中心点名对照 [2026-09-14-member-center-user-vs-com.md](../../doc/plans/2026-09-14-member-center-user-vs-com.md) | **现状仍以** [features/member-center.md](./features/member-center.md) **为准** |
| `ops/T14_RETIRE_PHP.md` 里 admin `:3002` | 过时；现网 admin 为 unix socket，见 [overview.md](./overview.md) |
