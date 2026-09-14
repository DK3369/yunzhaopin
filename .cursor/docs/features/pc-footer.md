# PC 页脚与单页

PC 深色页脚五列来自后台「单页」分类 + `is_nav=1` 的描述：[`AppFooter.vue`](../../web/layers/ui/app/components/AppFooter.vue) 调 `/v1/wap/descriptions/classes` 与 `/v1/wap/descriptions`，文案用 [`useSiteChrome.ts`](../../web/layers/ui/app/composables/useSiteChrome.ts) 的 `footerNav`。对照 PHP `default/footer.htm` 的 `{yun:}desc{/yun}`。

跳转走 [`descHref`](../../web/layers/ui/app/utils/site.ts)：

- `about/*.html`（`is_type=1` CMS）→ `/get/:id`，皮是 [`AboutShell.vue`](../../web/layers/ui/app/components/AboutShell.vue)（PC `make.htm` 左栏+正文；H5 只出正文）
- `index.php?c=top` → `/top`（排行榜，`index/top.htm` + `top.css`）
- `index.php?m=subscribe` → `/subscribe`（只做皮和跳转：已登录去 `/user/searches`，未登录去 `/login?next=/user/searches`。不接游客邮箱投稿接口）
- `m=link` → `/links`；`/zph/` `/once/` `/tiny/` `/evaluate/` `/map/` 走已有模块路由

`/pages/:code`：legal slug（about/contact/privacy/protocol）走 `/v1/wap/legal`；否则 `descriptions/by-name`（文件名映射含 `jyxkz` / `rlzy`）再 `site/pages`。页脚备案/人资证仍链 `/pages/jyxkz` `/pages/rlzy`。

这些路由不要套 `.site-inner`（会裁掉 `about_left` float）。H5 底栏五项不变。前台不要再给页脚加 `/data-show`。

只改 `web/`（`by-name` 允许中文 `name` 的校验在 `phpyun-rs` `validators::desc_page_name`）。不改 `uploads/` PHP。
