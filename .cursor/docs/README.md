# Cursor 文档（给后续对话读）

有实质业务落地后写进这里，不要只留在 chat 或 `.cursor/plans/`。  
**细节以代码为准**；这里只写现状口径、禁做项、文件入口。

硬边界仍看 [`.cursor/rules/rust-php-boundary.mdc`](../rules/rust-php-boundary.mdc)。怎么写文档看 [`.cursor/rules/cursor-docs.mdc`](../rules/cursor-docs.mdc)。

## 专题

| 主题 | 文件 |
|---|---|
| Rust 进程 / 分层 / 契约 / 编译 | [rust/README.md](./rust/README.md) |
| 已合并 / 即将失效接口（App OpenAPI `deprecated`） | [rust/api-merged.md](./rust/api-merged.md) |
| 后端 i18n（四层：Vue / Rust msg / dict_i18n / packed） | [rust/i18n.md](./rust/i18n.md) |
| 横切（登录态 / locale 缓存 key / 媒体 URL / settings 缓存） | [rust/cross-cut.md](./rust/cross-cut.md) |
| 公开只读两级缓存（L1 moka + Redis） | [rust/cache.md](./rust/cache.md) |
| 鉴权信任链（usertype=9 / admin 回查） | [rust/security.md](./rust/security.md) |
| 会员私信（HTTP `/v1/mcenter/chat/*`，表 `phpyun_rs_chat`） | [rust/chat.md](./rust/chat.md) |
| App 缺口（身份证实名 / 面试评价 / 栏目订阅） | [rust/app-api-gaps.md](./rust/app-api-gaps.md) |
| 采集英文岗、补 JD、官网申请记原投递表 | [features/job-scrape.md](./features/job-scrape.md) |
| PC/H5 会员中心套肤（求职 `/user` · 招聘 `/com`） | [features/member-center.md](./features/member-center.md) |
| PC 页脚五列与关于我们/排行榜/订阅落地页 | [features/pc-footer.md](./features/pc-footer.md) |
| 字段编码 UTF-8 / MySQL utf8mb4 | [`.cursor/rules/utf8.mdc`](../rules/utf8.mdc) |

## 不要当现状的

| 路径 | 怎么用 |
|---|---|
| `.cursor/plans/` | 某次执行稿；未落地的条目不要当已做 |
| 仓库根 `doc/plans/`、`PROJECT_PLAN.md` | 历史，条数会过期。会员中心点名对照见 [2026-09-14-member-center-user-vs-com.md](../../doc/plans/2026-09-14-member-center-user-vs-com.md)，**现状仍以** [features/member-center.md](./features/member-center.md) **为准** |
