# Cursor 文档（给后续对话读）

有实质业务落地后写进这里，不要只留在 chat 或 `.cursor/plans/`。  
**细节以代码为准**；这里只写现状口径、禁做项、文件入口。

硬边界仍看 [`.cursor/rules/rust-php-boundary.mdc`](../rules/rust-php-boundary.mdc)。怎么写文档看 [`.cursor/rules/cursor-docs.mdc`](../rules/cursor-docs.mdc)。

## 专题

| 主题 | 文件 |
|---|---|
| Rust 进程 / 分层 / 契约 / 编译 | [rust/README.md](./rust/README.md) |
| 后端 i18n（四层：Vue / Rust msg / dict_i18n / packed） | [rust/i18n.md](./rust/i18n.md) |
| 采集英文岗、补 JD、官网申请记原投递表 | [features/job-scrape.md](./features/job-scrape.md) |
| PC/H5 会员中心套肤（求职 `/user` · 招聘 `/com`） | [features/member-center.md](./features/member-center.md) |
| 字段编码 UTF-8 / MySQL utf8mb4 | [`.cursor/rules/utf8.mdc`](../rules/utf8.mdc) |

## 不要当现状的

| 路径 | 怎么用 |
|---|---|
| `.cursor/plans/` | 某次执行稿；未落地的条目不要当已做 |
| 仓库根 `doc/plans/`、`PROJECT_PLAN.md` | 历史，条数会过期 |
