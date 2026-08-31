# 2026-08-30 实施稿：Admin 现状盘点落入 doc + git

> **当日备忘，不是架构。** 现状读 [ARCHITECTURE.md](../ARCHITECTURE.md)。

对应 Cursor plan：`.cursor/plans/admin_现状盘点.plan.md`。

## 范围

1. 把后台 Nuxt + Rust API + 伪删除的现状写入 `doc/ADMIN_PROGRESS.md`。
2. 仓库 `.cursor/plans/` 提交 git（与 `.cursor/rules/` 一样）。
3. `doc/FRONTEND_BACKEND_SPLIT.md`、`doc/ADMIN_PHP_TO_NUXT.md` 加链接。
4. 接着按 PHP 补招聘会/新闻/问答/专题具名 action；再做前台 OAuth 与支付真单。

## 约定

- 每次 plan 双写：`.cursor/plans/*.plan.md` + `doc/plans/YYYY-MM-DD-短名.md`
- 现状只改 `doc/ADMIN_PROGRESS.md`
- `.gitignore` 不要忽略 `.cursor/`
- 不改 `~/.cursor/plans/` 当唯一存档
- 业务只写 jobs 库；不碰 systemd `:3000`

## 验收

- `git ls-files .cursor/plans/` 能看到本 plan
- `doc/ADMIN_PROGRESS.md` 含端口表（3000/9090 与 3003/9091）和下一项
