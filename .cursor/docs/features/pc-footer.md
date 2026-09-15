# PC 页脚与单页

PC 深色页脚五列来自后台「单页」分类 + `is_nav=1` 的描述：[`AppFooter.vue`](../../web/layers/ui/app/components/AppFooter.vue) 调 `/v1/wap/descriptions/classes` 与 `/v1/wap/descriptions`，文案用 [`useSiteChrome.ts`](../../web/layers/ui/app/composables/useSiteChrome.ts) 的 `footerNav`。对照 PHP `default/footer.htm` 的 `{yun:}desc{/yun}`。

跳转走 [`descHref`](../../web/layers/ui/app/utils/site.ts)：

- `about/*.html`（`is_type=1` CMS）→ `/get/:id`，皮是 [`AboutShell.vue`](../../web/layers/ui/app/components/AboutShell.vue)（PC `make.htm` 左栏+正文；H5 只出正文）
- `index.php?c=top` → `/top`：一次 `GET /v1/wap/rankings`（七块：推荐职位 / 最新企业 / 最新职位 / 最新人才 / 热词 / 资讯 hits / 紧急职位）。热词 `check=1` 跨 type，链用返回的 `to` + `type_name` + `num`。PHP 只有 PC 页；现网 PC/H5 共用 [`top.vue`](../../web/apps/site/app/pages/top.vue)（皮写在页面里，H5 不依赖 `/legacy/pc.css` 里的 `top.css`）。H5 底栏五项不含排行榜，入口仍是页脚。空块照样出卡片。
- `index.php?m=subscribe` → `/subscribe`：游客或登录投稿写 `phpyun_subscribe` 离散列（`POST /v1/wap/subscribe`，meta 走 `GET /v1/wap/subscribe/meta`）。**不是**会员搜索器 `phpyun_finder`（那是 `/user/searches`）
- `m=link` → `/links`：`sy_linksq=1` 才出申请块；申请可加 `link_type` / `pic`。`/zph/` `/once/` `/tiny/` `/evaluate/` `/map/` 走已有模块路由

`/pages/:code`：legal slug（about/contact/privacy/protocol）走 `/v1/wap/legal`；否则 `descriptions/by-name`（文件名映射含 `jyxkz` / `rlzy`）再 `site/pages`。页脚备案/人资证仍链 `/pages/jyxkz` `/pages/rlzy`。

`/get/:id` 与页脚走请求语言：公开接口读 [`phpyun-rs/cms-pages/{lang}/`](../../phpyun-rs/cms-pages/)（`meta.json` 给页脚/左栏，`{id}.html` 给正文，对齐 PHP `desc.cache.php`）。请求头是 `Accept-Language: zh-CN|en`（cookie 仍是 `zh`/`en`）。中文底稿仍在 `phpyun_description`；后台首次保存生成、改完再覆盖。请求只读、不写盘、不灌进程内存。英文缺文件回退 `zh-CN`。页脚 `FOOTER_NAME_KEY` 同时认中文和英文 CMS 名，再 `$t` 成当前 Vue 语言，避免接口英文、皮中文时列名混排。**不要**把单页 HTML 打进 Vue 大 JSON，也不要另开翻译表。

这些路由不要套 `.site-inner`（会裁掉 `about_left` float）。H5 底栏五项不变。前台不要再给页脚加 `/data-show`。

**不做**：周期扫表发匹配职位（`phpyun_subscriberecord` / cron）。

只改 `web/` 与 `phpyun-rs/`。不改 `uploads/` PHP。
