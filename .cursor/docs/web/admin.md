# 管理后台（`@phpyun/admin`）

Nuxt 4 SPA（`ssr: false`，`baseURL=/admin/`）。UI 从 PHP 后台 1:1 迁过来，**不自造页面**。工作约定长文：[doc/ADMIN_PHP_TO_NUXT.md](../../../doc/ADMIN_PHP_TO_NUXT.md)。

整机：现网 nginx `/admin/` → site `:3001` → admin **unix socket** `/var/tmp/phpyun-admin.sock`（不占 TCP）。见 [overview.md](../overview.md)。

## 目录

| 路径 | 作用 |
|---|---|
| `apps/admin/app/pages/*.vue` | 薄路由壳（对齐 PHP `router.js` path） |
| `apps/admin/app/admin-php/` | 真 UI：`user/` `neirong/` `system/` `yunying/` `tool/` `index/` + `component/` |
| `apps/admin/app/utils/phpMap.ts` | `m=&c=&a=` → 显式 `/v1/admin/...` |
| `apps/admin/app/utils/httpPost.ts` | 页面仍调 PHP 风格字符串 |
| `apps/admin/public/php-admin/` | 旧 CSS/JS 静态 |
| `apps/admin/i18n/locales/` | 后台文案；`phpLc` / `lc()` |

布局：`layouts/default.vue`（菜单）+ `blank.vue`（登录等）。鉴权中间件 `middleware/auth.global.ts`：要求 `usertype === 9`。

## 调用链

```
admin-php 页 httpPost('m=…&c=…&a=…', body)
  → phpMap.ts 映射为 /v1/admin/...
  → bffUrl('/api/proxy/...')（带 /admin 前缀）
  → site :3001 → Rust :3003
```

登录：`POST /admin/api/auth/admin-login` → Rust `POST /v1/admin/login`，cookie 仍是 `token`。

未映射的 `m/c/a`：**报错「未映射的后台接口」**，不要在 Rust 做万能 `invoke`。缺能力就在 `phpyun-api-admin` 加显式路由，并在 `phpMap.ts` 加一行。

## 两类后台 API

| 类型 | 路径 | OpenAPI |
|---|---|---|
| 具名 | `/v1/admin/login` `/menu` `/jobs` `/users` `/site-settings`… | 进 `doc/snapshots/admin_paths.txt` |
| PHP 长尾 | `POST /v1/admin/php-content/{module}/{action}` | **不进** OpenAPI；实现 `admin_php_content_service.rs` 的 `dispatch` |

缺口以 **`phpMap.ts` + `dispatch`** 为准，不要从过期的 `doc/API_GAP.md` / `ADMIN_PROGRESS.md` 反推。

## 菜单

侧栏数据来自 `POST /v1/admin/menu`（库表），不是只改前端路由。新页面若要在后台导航出现，还需库侧菜单 / 权限（对照 PHP 后台菜单配置）。

支付网关三页 `pages/payment/{orders,methods,merchants}.vue` 直接 `useApi()` 调 `/v1/admin/pay/*`，**不要**套 `admin-php/system` 或改 `payset.vue`。菜单 id 1070–1073。见 [pay-gateway.md](../features/pay-gateway.md)。

## 本机 vs 现网

| 环境 | Admin 怎么跑 |
|---|---|
| 本机开发 | `pnpm dev:admin` → `:3002`；`pnpm dev:site` 的 `nitro.devProxy` 把 `/admin` 转到 3002 |
| 现网 | systemd `test-jobs-phpyun-admin`：`NITRO_UNIX_SOCKET=/var/tmp/phpyun-admin.sock`；site 用 `ADMIN_SOCK` + `00-admin-host.ts` 转发 |

**不要** start `test-jobs-phpyun-admin-edge`，不要把现网 admin 绑回 TCP `:3002`。

## 加后台页（配方）

1. 对照 `uploads/` 里对应 PHP Vue（**只读**），在 `admin-php/.../` 写业务组件
2. `pages/mypage.vue`：`import PhpPage from '~/admin-php/...'` 再导出
3. 新接口：`phpyun-api-admin` handler → `services` → `models`；`phpMap.ts` 映射 `m/c/a`
4. 长尾且暂不具名：在 `admin_php_content_service` 的 `dispatch` 加分支（仍禁止万能 invoke 入口）
5. 改公开具名路径时更新 `doc/snapshots/admin_paths.txt` + `admin.openapi.json`
6. 菜单若需入口：改库菜单 / 走后台「导航」配置，不只加 `pages/`

端到端见 [dev-playbook.md](../dev-playbook.md)。命名与 SQL：[api-naming.mdc](../../rules/api-naming.mdc)。鉴权：[rust/security.md](../rust/security.md)（`usertype=9`，admin 回查库 fail-closed）。

## 禁做

- 改 `uploads/admin` 或 PHP 后台源码「修一版再拷」——直接在 `web/apps/admin` 改
- `POST /v1/admin/invoke` 或把 `m/c/a` 透传进 SQL
- 把后台专用能力塞进 `/v1/wap` / `/v1/mcenter`
- 信 body 里的 uid；管理员身份以 JWT + 库为准
- 用过期进度文当未做清单
