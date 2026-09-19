# Site 页面域 → API（域地图）

精确路径以 [doc/snapshots/v1_paths.txt](../../../doc/snapshots/v1_paths.txt) 与 OpenAPI 为准。本文只标 **业务域 → 路由前缀 → 主要 API 前缀 / composable**，方便找入口。

页面代码：[`web/apps/site/app/pages/`](../../../web/apps/site/app/pages/)。调用一律 `useApi()` → BFF → Rust。

## 图例

| 前缀 | 含义 |
|---|---|
| `/v1/wap/*` | 公开读 / 登录注册；多数可 GET 别名 |
| `/v1/mcenter/*` | 已登录会员（整树 `member_guard`） |
| boot | `useSiteBoot` → `GET /v1/wap/initjobs`（nav / footer / settings） |

## 公开站

| 域 | 典型路由 | 主要 API | 备注 |
|---|---|---|---|
| 首页 / 搜索 | `/` `/search` `/map` `/top` | `initjobs`、`/v1/wap/jobs`、广告 | chrome 用 boot，不要再打旧 `/v1/wap/nav` |
| 职位 | `/jobs` `/jobs/:id` | `/v1/wap/jobs`、`jobs/detail` 等 | 模块 `sy_job_web` |
| 企业 | `/companies` `/companies/:uid`… | `/v1/wap/companies`、sidebar | **不是**会员 `/com` |
| 简历（公开） | `/resumes` `/resumes/:uid` | `/v1/wap/resumes`… | |
| 资讯 | `/articles` `/articles/:id` `/articles/channels` | `/v1/wap/articles`… | 后台标题/分类外链是**同域** `/articles/{id}` 与 `?nid=`（现网 Nuxt 在 `job1.ov6.com`）。不要拼 `WEB_BASE_URL`（`test-jobs.ov6.com` 仍是 PHP，`/articles` 会 nginx 404），禁止 `index.php?m=news`。旧 `/index.php?m=news` 由 site `php-index.global.ts` 302 过来。 |
| 公告 | `/announcements`… | `/v1/wap/announcements`… | |
| 招聘会 | `/fairs` `/fairs/:id` | `/v1/wap/zph`… | 模块 `zph` |
| 问答 | `/questions`… | `/v1/wap/qna`… | 模块 `ask` |
| 兼职 / 微聘 | `/parts` `/once` `/tiny` | `/v1/wap/parts`、`once`、`tiny-resumes` | |
| 公招 | `/gongzhao`… | `/v1/wap/gongzhao` | |
| 测评 | `/eval`… | 与 evaluate 模块相关 | |
| CMS 单页 | `/get/:id` `/pages/...` | `/v1/wap/descriptions/get`；正文见 `cms-pages/` | 页脚见 [pc-footer.md](../features/pc-footer.md) |
| 登录注册 | `/login` `/register` `/forgetpw` `/oauth-bind` `/app-login` | `/v1/wap/login`、`captcha`、`sms`、`oauth/*`、`refresh` | BFF `/api/auth/*` 写 cookie |
| 认领 / 邀请 / 订阅 | `/claim` `/invite` `/subscribe` | `/v1/wap/claim`、subscribe 等 | |
| 分享落地 | `/share/job|company|resume/...` | wap 详情 + share token | |
| 海报 | `/poster/...` | 详情数据拼海报 | |
| 其它 | `/links` `/services` `/download` `/advice` `/site` `/hr` `/redeem` `/specials` `/quick-apply` `/pay/stripe` | 对应 wap 或 settings | `/advice` 登录后可进会员壳；`/pay/stripe` 是 Stripe Hosted Checkout 公开回跳。网关见 [pay-gateway.md](../features/pay-gateway.md) |

## 求职会员 `/user`

`usertype=1`。壳：`MemberShell` + `useMemberNav`。Dashboard：`POST /v1/mcenter/dashboard/full`（async key `user-dash`）。

| 域 | 典型路由 | 主要 API |
|---|---|---|
| 首页 | `/user` | `dashboard/full`、`resume/bundle`、广告 |
| 简历 | `/user/resume` `/user/expects` `/user/resume/*` | `/v1/mcenter/resume/*` |
| 投递 / 面试 | `/user/applications` `/user/interviews` | applications、interviews |
| 消息 / 私信 | `/user/messages` `/user/chat` `/user/inbox` `/user/outbox` | messages、chat（见 [chat.md](../rust/chat.md)） |
| 收藏 / 关注 / 谁看过 | `/user/favorites` `/user/follows` `/user/views` `/user/looks` | favorites、follows、views |
| 财务 | `/user/pay` `/user/cashier` `/user/finance` `/user/integral`… | pay、integral、orders |
| 账号 | `/user/account` `/user/password` `/user/binding` `/user/set` `/user/privacy` `/user/ident` | account、sessions、password、blacklist |
| 其它 | `/user/parts` `/user/recommend` `/user/searches` `/user/eval-logs` `/user/invite`… | 对应 mcenter；模块开关见 member-center |

Stripe 付完回跳公开页 `/pay/stripe`（`order_no` + `session_id`），不进 `/user` `/com` 登录门闩；登录后再 `stripe-return`。

## 管理后台 `/admin`

Nuxt `baseURL=/admin/`。支付网关页（系统后面一级 **支付**，库菜单 1070–1073）不走 phpMap：

| 页 | 路由 | API |
|---|---|---|
| 订单 | `/admin/payment/orders` | `POST /v1/admin/pay/orders/list` |
| 支付方式 | `/admin/payment/methods` | `POST /v1/admin/pay/methods/{list,save,status,delete}` |
| 商户 | `/admin/payment/merchants` | `POST /v1/admin/pay/merchants/{list,save,status}` |

不要改 System `/admin/payset`。口径见 [pay-gateway.md](../features/pay-gateway.md)。

## 招聘会员 `/com`

`usertype=2`。Dashboard：`POST /v1/mcenter/com-dashboard/full`（key `com-dash`）。VIP：`/v1/mcenter/vip/current`。

| 域 | 典型路由 | 主要 API |
|---|---|---|
| 首页 | `/com` | `com-dashboard/full`、`vip/current`、`recommend/resumes` |
| 职位 | `/com/jobs` `/com/jobs/new` | `jobs/*`、`jobs/check`、promote、refresh |
| 收到的简历 / 面试 | `/com/applications` `/com/interviews` `/com/downloads` | applications、interviews、downloads |
| 企业资料 | `/com/profile` `/com/cert` `/com/addresses` `/com/gallery` `/com/products` `/com/map` `/com/banners` | company、addresses、certs… |
| 协作 | `/com/hrs` `/com/sub-accounts` | hrs、invite codes（`phpyun_rs_*`） |
| 消息 | `/com/messages` `/com/chat` `/com/job-messages` `/com/broadcasts` | messages、chat |
| 财务 / 套餐 | `/com/pay` `/com/orders` `/com/added` `/com/services` `/com/member-right` `/com/integral`… | vip、packs、redeem、integral |
| 运营 | `/com/fairs` `/com/specials` `/com/parts` `/com/talent*` `/com/stats` `/com/tongji` `/com/finder` | zph、special、talent… |
| 账号 | `/com/account` `/com/password` `/com/binding` | 同求职侧 account/sessions |

会员 CSS / 顶栏 / 禁止事项：[features/member-center.md](../features/member-center.md)。

## 常用 composable ↔ 域

| Composable | 覆盖 |
|---|---|
| `useSiteBoot` / `useSiteSettings` / `useSiteChrome` | 公开壳、模块开关、页脚 |
| `useMemberNav` | `/user` `/com` 左栏 |
| `useAdsBundle` | 广告 slot |
| `usePublicDicts` / `useJobCats` / `useRegionCascade` | 字典、职位类、地区 |
| `useAuthMe` | 全站登录态 |
| `useListLoginGate` | 列表页未登录拦截 |

## 改路由时

- 新公开页：补 `PATH_TO_MODULE`（若有模块开关）
- 新会员入口：补 `useMemberNav` + 必要时 `MEMBER_PATH_TO_MODULE`
- 新 API：更新 `doc/snapshots/`，见 [dev-playbook.md](../dev-playbook.md)
