# Web 前端（Nuxt 4）

pnpm workspace：`apps/site`（SSR，PC+H5+会员）+ `apps/admin`（SPA）+ `layers/base` + `layers/ui`。**无 Pinia**；会话用 `useAsyncData` / `useState`（如 `auth-me`）。

整机端口与 nginx 见 [overview.md](../overview.md)。会员壳细则见 [features/member-center.md](../features/member-center.md)。后台见 [admin.md](./admin.md)。页面域 → API 见 [routes.md](./routes.md)。

## 工作区

| 包 | 路径 | 作用 |
|---|---|---|
| `@phpyun/site` | `web/apps/site` | 公网页 + `/user` `/com`；现网 **`:3001`** |
| `@phpyun/admin` | `web/apps/admin` | `/admin/`；现网 unix socket；本机 dev 才 `:3002` |
| `@phpyun/layer-base` | `web/layers/base` | BFF、`useApi`、auth cookie、locale、信封 |
| `@phpyun/layer-ui` | `web/layers/ui` | `MemberShell`、`AppHeader`、`useSiteBoot`、`site.ts` |

两端 `extends: ['../../layers/base', '../../layers/ui']`。Node ≥22.19，pnpm 10.14。

## 开发命令

```bash
cd web
pnpm install
pnpm dev:site     # :3001；nuxt dev 时 /admin → 本机 :3002
pnpm dev:admin    # :3002，仅本机
pnpm build:site
pnpm build:admin  # generate
pnpm gen:types:site   # 需 Rust /api-docs/v1/openapi.json
pnpm gen:types:admin
```

`RUST_API_URL` / `NUXT_RUST_API` 必须指 `http://127.0.0.1:3003`。现网重启：`ops/restart.sh`（site + admin），不要另开 3002/3004/3005。

## PC vs H5

**不是**两套应用、**不是** UA 分流。

- 同一 file-based 路由：`apps/site/app/pages/`
- 模板双写 `.site-pc` / `.site-h5`；`app/assets/main.css` 用 `min-width:1200px` 显隐
- H5 职位卡标题（`.table-card-word` / `.tab_card_job_name` / `.comnew_jobname`）最多 3 行，在 `main.css` 收口，不要改 yunwap 单行 nowrap
- H5 首页搜索条是 `input.index_newedition_search_p`；yunwap 按 PHP `span` 写死 `5.33rem`，不要让白底截成半截灰胶囊，在 `main.css` 铺满
- 打包 CSS：`/legacy/site-pc.css` / `/legacy/site-h5.css`（见 `server/utils/legacyCss.ts`，磁盘在 `public/legacy/`）；皮肤与会员包按 `m=user|com` 拼
- `sy_wap_web==2` 时 `html.force-pc` 强制 PC

## 调 Rust：只走 BFF

```
Browser → /api/proxy/v1/... （site）或 /admin/api/proxy/v1/... （admin）
       → layers/base server proxy → Rust :3003
```

| 环节 | 入口 |
|---|---|
| 客户端 | [`useApi.ts`](../../web/layers/base/app/composables/useApi.ts)：`bffUrl('/api/proxy' + path)`，`credentials: 'include'` |
| 代理 | [`server/routes/api/proxy/[...path].ts`](../../web/layers/base/server/routes/api/proxy/[...path].ts) |
| Cookie | `token`（HttpOnly，`auth-cookie.ts`）→ 转发 `Authorization: Bearer` |
| 登录 BFF | `server/routes/api/auth/*`（login / me / refresh / admin-login…） |
| 语言 | Cookie `lang`（前台）/ `admin_lang`（后台）→ 请求头 `Accept-Language: en \| zh-CN`；**不要** `?lang=` |

`session_expired` 时 `useApi` 会打 `/api/auth/refresh` 再重试。mcenter 无 token → 401。

## 鉴权（前台）

1. `/login` → `/api/auth/login`（或 sms / oauth）→ 写 cookie
2. `useAuthMe` → `/api/auth/me` → `/v1/wap/me`
3. `usertype`：`1` → `/user`，`2` → `/com`
4. 中间件 [`member-role.global.ts`](../../web/apps/site/app/middleware/member-role.global.ts)：未登录进会员 → `/login?next=`；串端互跳

判断企业会员路径必须用 [`isComMemberPath`](../../web/layers/ui/app/utils/site.ts)（`/com` 或 `/com/...`）。**禁止** `path.startsWith('/com')`，会误伤公开「找企业」`/companies`。

`/user/**`、`/com/**` 在 `nuxt.config` 里 `ssr: false`。

## i18n（Vue）

- `@nuxtjs/i18n`：`zh` / `en`，`strategy: 'no_prefix'`，`detectBrowserLanguage: false`，默认 **en**
- 文案：`apps/site/i18n/locales/{zh,en}.json`（编号 key）
- 与 Rust `msg` / `dict_i18n` / packed **不是同一层**，见 [rust/i18n.md](../rust/i18n.md)

## 共享 composable（常改）

| 名字 | 层 | 用途 |
|---|---|---|
| `useApi` | base | 所有 Rust 调用 |
| `useAuthMe` | base | 当前用户 |
| `useLocaleScope` / `useLocaleAsyncKey` | base | cookie + SSR 缓存 key |
| `useSiteBoot` | ui | 一次 `GET /v1/wap/initjobs`（nav / footer / settings） |
| `useSiteSettings` / `useSiteChrome` | ui | 读 boot 里的配置与壳 |
| `useMemberNav` | ui | 会员左栏 |
| `useAdsBundle` | ui | 广告位 |

模块开关：`sy_{module}_web != 2`（`isPathModuleOn` / `isMemberModuleOn`）。新公开模块要在 `PATH_TO_MODULE` / `MODULE_PATH` 补映射。

## 加公开页（PC+H5）

1. 加 [`apps/site/app/pages/<route>.vue`](../../web/apps/site/app/pages/)（或子目录）
2. 模板写 `.site-pc` / `.site-h5`（或复用 `JobCard` 等）
3. `useApi().get/post('/v1/wap/...')`；文案 `$t('…')` 进 `i18n/locales`
4. 若新模块：[`site.ts`](../../web/layers/ui/app/utils/site.ts) 的 `PATH_TO_MODULE` / 导航映射补一项
5. 需要新接口时走 [dev-playbook.md](../dev-playbook.md)「新 App/前台接口」

## 加会员页

1. `pages/user/foo.vue` 或 `pages/com/foo.vue`
2. `useMemberNav` 加入口；用 `MemberShell` / `MemberPanel`
3. API 走 `/v1/mcenter/...`；CSS / 壳约定见 [member-center.md](../features/member-center.md)
4. 依赖站点模块的入口补 `MEMBER_PATH_TO_MODULE`

## 目录速查（site）

```
apps/site/app/
  pages/          # 公开 + user/ + com/
  components/     # 页专属（少）
  middleware/     # member-role、public-detail
  assets/main.css
  utils/
apps/site/server/
  middleware/     # 00-admin-host（现网转 admin sock）、02-rust-docs
  routes/         # callback、legacy css、sitemap
  utils/legacyCss.ts
apps/site/i18n/locales/
```

`composables/` 与 `stores/` 不在 site 里——逻辑在 layers。

## 禁做

- 浏览器直连 `:3003` 做业务（应用 BFF）
- 会员路径用 `startsWith('/com')`
- 把后台能力塞进 `/v1/wap` / `/v1/mcenter`
- 改 `uploads/` 模板「顺便修皮」；只改 `web/`
- 现网再绑 admin TCP 或 start `test-jobs-phpyun-admin-edge`
- 运行时把 `legacyCss` 指回 `uploads/`；用户文件走仓库根 `storage/upload/`（`/data/upload` 运行时读盘，不要 `publicAssets` 构建拷贝）
