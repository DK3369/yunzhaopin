# 2026-08-30 实施稿：公招 / 广告 / 财务 + 伪删除

对应 Cursor plan：`.cursor/plans/admin_下一批迁移.plan.md`。

## 范围

1. `php-content` 公招具名 action + 公告 `getGroup`/`checksitedid`。
2. 修广告 `del`→create、`ad_class` 错表；补广告位/分类具名 action。
3. 财务订单/消费/后台充值按 PHP 包一层字段。
4. 招聘会/专题主表加 `deleted` 并改 del 为标记删除。

## 约定

- 不进 AdminDoc（快照仍 297）
- 只写 jobs；不碰 systemd `:3000`
- 每小块 commit + push
- 凭证上传若无 storage 则明确业务错误，不假装成功
- `xls` 出 CSV

## 本波之后仍排队

微聘 once/tiny、兼职 show/audit、名企 save 映射、简历 skill/project/other、单页、分类树 ajax、微信菜单、招聘会 xls/图。
