# 横切口径（语言之外）

语言合同见 [i18n.md](./i18n.md)。这里是同一类「一条通道、漏了就会混」的收口。不要为干净再拆 PC/H5、GET+POST、PhpOut。

## 登录态

- 前台「我是谁」只走 [`useAuthMe`](../../../web/layers/base/app/composables/useAuthMe.ts)，key 固定 `auth-me`。
- 浏览器打 `/api/auth/me`（BFF 拆信封后是 `/v1/wap/me`）。会员页、中间件、顶栏共用这一份，不要再各打 `/v1/wap/me`。
- SSR：`useRequestFetch()` + [`ssrCookieHeaders`](../../../web/layers/base/app/utils/ssrFetch.ts) 转发文档请求的 Cookie。`useApi` 同样带 Cookie 和 `Accept-Language: zh-CN|en`。
- 会员头栏与首页 dash 共用 `user-dash` / `com-dash` 等 key；另一身份必须用 `hdr-skip-*`，避免把共享缓存写成 `null`。
- `/v1/mcenter/*` 必须登录。凭证是 `Authorization: Bearer` **或** Cookie `token=`：JWT 签名、`exp`、黑名单、`phpyun_user_session`。[`member_guard`](../../../phpyun-rs/crates/platform/core/src/member_guard.rs) 挂在 `/v1/mcenter` nest 上，没有 token **进不了 handler**，HTTP 401 `unauth`。BFF [`/api/proxy`](../../../web/layers/base/server/routes/api/proxy/[...path].ts) 无 `token` cookie 也不转发。公开意见反馈走 `/v1/wap/advice`。
- **互不信任**：前台 `ensureLogin(me)` / 中间件跳登录 **不是授权**。谁能看数据、以谁的身份写库，只认 Rust [`AuthenticatedUser`](../../../phpyun-rs/crates/platform/core/src/extractors.rs)。BFF 有 cookie 只负责转发，垃圾 token 仍由后端 401。
- 职位 / 简历 / 企业 **详情**（含 contact、share-text，以及 `/v1/wap/jobs?id=` 全文旁路）必须登录。**列表**给游客；公开列表 `page_size` 上限 20，Redis `rl:ip:{ip}:wap-list` 60 次/分钟，详情 `rl:uid:{uid}:wap-detail` 30 次/分钟。Governor 按 `X-Forwarded-For`（`SmartIpKeyExtractor`），`APP_ENV=test` 才关掉。
- 公开页要登录的动作（收藏/关注/举报/提问/报名/兑换/留言/看电话等）：跟顶栏同一份 `me`，**没登录先 `ensureLogin(me, 当前页)`**，不要裸 `navigateTo('/login')`（求职者登录后会甩首页）。已登录才 POST；后端再验 Cookie。会话过期 **401** 再 `goLogin(next)`。游客不打 `exists` / `unread-summary`。已登录误进 `/login` 时回 `next` 或 `history.back()`。`unwrapEnvelope` 以 `code === 200` 为准。

## `useAsyncData` key 带 locale

切语言会整页 reload，但 SSR payload 仍按 key 复用。随语言变的接口用 [`localeAsyncKey('site-nav')`](../../../web/layers/base/app/composables/useLocaleAsyncKey.ts)，禁止再抄 `'site-nav-1'`。

已带 locale：顶栏 settings/nav、广告 `useAdsBundle`、`usePublicDicts`、职位/兼职分类 `useJobCats` / `usePartCats`、首页/职位/简历/企业/兼职列表。`localeAsyncKey` 在 setup 里算出**字符串**（切语言会整页 reload）。不要用会 watch locale 的 getter，否则 i18n 就绪时会把导航/配置再打一遍。

会员 `/user` `/com` 与登录页 **不要**打前台 `nav` / `descriptions` / `hot-searches`。settings 全站一份（[`useSiteSettings`](../../../web/layers/ui/app/composables/useSiteSettings.ts)）。头栏 / 左栏 / 首页 dash 共用 `user-dash` / `com-dash`，另一身份用 `hdr-skip-*`。

## 媒体 URL

Rust 拼图只走 [`media_url` / `media_url_from_cfg`](../../../phpyun-rs/crates/platform/core/src/utils.rs)：已是 `http(s)` 原样；相对路径前缀 `sy_ossurl`，空则 `sy_weburl`。后台 `checkpic` 别再各写一份。

前台卡片继续 [`mediaUrl`](../../../web/layers/ui/app/utils/site.ts)（相对路径变站内 `/…`，给 nginx 静态）。两套场景不同，不要合成一个函数。

## 站点配置缓存

`site_gate_service::config_str` 读 `phpyun_admin_config` 走 `cache::get_or_load`（L1 moka + L2 Redis，key `site_setting:{name}`，TTL 30s）。后台 `admin_upsert` / `admin_delete` 按 key `invalidate`。PHP 批量改配置清 L1（`invalidate_all_config`）；L2 最多 30s 过期。

## 薪资 / 日期 / 分类

- 前台薪资只 [`formatSalary`](../../../web/layers/ui/app/utils/site.ts)；不要把 i18n 文案塞进 `salaryType`。
- 日期：`models` 的 `*_n` 用 `phpyun_core::utils::{fmt_date, fmt_dt}`（站点时区）。前端吃 `*_n`，不要再造一套。
- 职位分类树：`useJobCats()`，key `job-cats-${locale}`。兼职 `usePartCats()`。

## 明确不做

- 分页参数名：公网 `page`+`page_size`，侧栏 `limit`，后台 `perPage` 别名。只文档化，不盲改。
- 不要嗅探 User-Agent 分 PC/H5。CSS `.site-pc` / `.site-h5`。
- 旧 WAP 公开读 GET+POST 双挂不要删；新接口不要无故再双挂。
- 后台 PhpOut / AdminPaged 别名不拆。
- 上传仍走 `/api/upload`（multipart），不塞进 `useApi` 信封。
