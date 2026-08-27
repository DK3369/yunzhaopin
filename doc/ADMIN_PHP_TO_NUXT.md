# 管理后台：PHP 完整复刻到 Nuxt 4 + api-admin

> 2026-08-27 · 分支 `feat/frontend-backend-split`
> 总方案仍见 [FRONTEND_BACKEND_SPLIT.md](./FRONTEND_BACKEND_SPLIT.md)（T3 crate 拆分、T11 后台）。本文是 **T11 的执行约定**：按 PHP 源码 1:1 迁 UI 与接口，不自造页面。

## 已定接口（不要改）

前后端分离文档 **§2.2 / §2.3 / T3 / T11** 已经定过，后台必须走 **admin 接口**，不是 App 接口：

| 项 | 约定 |
|---|---|
| Crate | `phpyun-rs/crates/products/recruit/api-admin`（包名 `phpyun-api-admin`） |
| URL | 一律 **`/v1/admin/*`** |
| 登录 | `POST /v1/admin/login` 公开；其余走 `admin_guard`（JWT **`usertype=3`**） |
| OpenAPI | **`/api-docs/admin/openapi.json`**；`apps/admin` 只吃这份。App 用 `/api-docs/v1/openapi.json`（`/v1/wap` + `/v1/mcenter`） |
| 装配 | `apps/server` merge `phpyun_api_admin::router`；路径快照 [snapshots/admin_paths.txt](./snapshots/admin_paths.txt) |
| 业务 | 写在 `phpyun-services`；handler 禁止 sqlx/redis/moka/reqwest |
| App 契约 | **禁止**把后台能力塞进 `/v1/wap`、`/v1/mcenter` |

Nuxt 调用链：`apps/admin` → BFF `/api/proxy/v1/admin/...` → Rust `/v1/admin/...`。

**不要**做通用 `POST /v1/admin/invoke`（把 PHP 的 `m/c/a` 当万能入口）。缺能力就在 **api-admin 加显式 POST**（与现有 `/v1/admin/jobs`、`/v1/admin/companies`、`/v1/admin/menu` 同一风格），`utoipa` 进 admin spec。

PHP 页里的 `httpPost('m=user&c=company_job&a=index', params)`：在 **web** 做一层同名适配器，翻译成 `/v1/admin/jobs` 等；页面字符串可以暂时保留，映射表只放适配器。

## 工作边界

- **`uploads/` 只读**：PHP 后台当规格，不改（含 `uploads/admin`、`uploads/app/template/admin`、语言包）。
- **前端只写 `web/`**：Nuxt 4、`apps/admin`、`ssr: false`、Element Plus。CSS/图从 PHP **复制进** `web/apps/admin`。
- **业务只写 `phpyun-rs/` 的 api-admin + services**。

## UI 规格（对照 PHP，不创造）

- 壳：`uploads/app/template/admin/index.htm` + `adstyle/allcss/index.css`
- 路由：`uploads/app/template/admin/js/router.js`（约 118 个 path，一 path 一页）
- 页面：约 110 个 `*.html` + 约 219 个 `component/*.vue`
- 登录落地：`/index` 或 `/jobtai`（与 PHP `admin_get_user_login` 一致）
- 菜单 `/v1/admin/menu` 的 **route = PHP `path` 原样**，禁止折成 `/jobs`、`/system/settings`

迁到 web 时只做语法：

- `httpVueLoader` → `import`
- `window.parent.homeapp.$route.query` → `useRoute().query`
- Element UI 2 → Plus：`slot-scope`、`:visible.sync`、`slot="footer"`、`size="mini"`
- 目录对齐：`web/apps/admin/app/admin-php/...` 对照 `uploads/app/template/admin/...`

禁止继续用：`phpScreens.ts` 多页映射一页、`PhpPending` 当产品页、自造 `pages/jobs` / `pages/companies` / `pages/system/settings` 当主 UI。

## 仓库落点（2026-08-27 起）

| 项 | 路径 |
|---|---|
| PHP 模板迁入（只做语法） | [`web/apps/admin/app/admin-php/`](../web/apps/admin/app/admin-php/) |
| 一 path 一页 | [`web/apps/admin/app/pages/*.vue`](../web/apps/admin/app/pages/)（与 `router.js` 118 条对齐） |
| `httpPost` 适配器 | [`web/apps/admin/app/utils/httpPost.ts`](../web/apps/admin/app/utils/httpPost.ts) + 映射表 [`phpMap.ts`](../web/apps/admin/app/utils/phpMap.ts) |
| 静态资源 / 语言包 | [`web/apps/admin/public/php-admin/`](../web/apps/admin/public/php-admin/) |
| 首页统计 | `POST /v1/admin/dashboard/home-data`、`ajax-statis`、`month-statis`、`msg-num`、`chart`（`getweb`/`resumetj`/…） |
| 配置批量保存 | `POST /v1/admin/site-settings/batch` |
| PHP 缓存占位 | `POST /v1/admin/cache/php-dicts`（`common/cache`、`getCacheData`） |

映射表示例：`m=user&c=company_job&a=index` → `POST /v1/admin/jobs`；`m=system&c=set_config&a=save` → `POST /v1/admin/site-settings/batch`。未列入映射表的 `m/c/a` **不会**走万能 invoke，页面会收到「未映射」错误，然后在 api-admin 补显式路由。

## 实施顺序

每页对照 PHP 同一 path（筛选项、列、批量、抽屉、对应 `/v1/admin/*`），浏览器点一遍。

1. 壳 + 登录 + `httpPost` → `/v1/admin/*`
2. `/index`（`hometop` / `homecenter` / `indexright` / `homebottom`；补 `monthStatis` 等到 dashboard）
3. 会员主路径：`/companyjob` `/companycrm` `/resume` 等；对齐 `jobs*` `companies*` `resumes*`
4. 系统设置：`/set` `/payset` `/seoset` **各页各迁**
5. `router.js` 其余 path：缺的 **只加 api-admin 路由**

不跑会改 `uploads/` 的批量脚本。
