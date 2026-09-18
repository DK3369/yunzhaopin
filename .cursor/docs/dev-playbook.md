# 开发配方（端到端）

动手前先读 [overview.md](./overview.md)。改 Rust 再读 [rust/README.md](./rust/README.md)；改页面再读 [web/README.md](./web/README.md)。细节以代码为准。

## 0. 通用禁做

- 不改 `uploads/` PHP 业务代码
- Handler / api-admin **禁止** `sqlx` / `redis` / `moka` / `reqwest` / 业务规则
- `/v1/wap` + `/v1/mcenter` **只加法**；破坏性走 `/v2`
- 不做万能 `invoke`；标识符走 `ident_ok`，值绑定，富文本 `sanitize_html`（`api-naming.mdc`）
- 现网只绑 Rust `:3003`、Web `:3001`；不起 `:3000`

---

## 1. 新 App / 前台接口

**谁用：** Flutter、PC/H5（经 BFF）。

| 步骤 | 改哪里 |
|---|---|
| 1. 表 / SQL | `phpyun-rs/crates/products/recruit/models/src/<domain>/{entity,repo,mod}.rs`；新表优先 `phpyun_rs_*`，迁移放 `migrations/sqlx/`（现网常 `RUN_MIGRATIONS_ON_BOOT=false`，手工执行） |
| 2. 业务 | `.../services/src/<name>_service.rs` |
| 3. Handler | `.../api/src/v1/wap/foo.rs` 或 `mcenter/foo.rs` → `pub fn routes()` → 在 `mod.rs` `.merge(...)` |
| 4. GET 别名 | 公开读若需 GET：登记该模块 `GET_ALLOWED_PATHS`（默认 POST-only） |
| 5. 契约 | 更新 `doc/snapshots/v1_paths.txt` + `v1.openapi.json`；跑 `cargo test -p phpyun-handlers --test openapi_snapshot`（全量可能 OOM，优先相关断言） |
| 6. 前端 | `useApi().get/post('/v1/wap/...')` 或 `/v1/mcenter/...`；类型可 `pnpm gen:types:site` |

参考：`mcenter/chat.rs` → `chat_service` → `models/chat/`。信封见 [rust/api.md](./rust/api.md)。

**验：**

```bash
curl -i http://127.0.0.1:3003/health
# 现网
ops/restart.sh rust --build
# 经 BFF（需登录的带 cookie）
curl -i 'http://127.0.0.1:3001/api/proxy/v1/wap/...' 
```

---

## 2. 新后台接口

**谁用：** 仅 `web/apps/admin`。

| 步骤 | 改哪里 |
|---|---|
| 1–2 | 同 §1：models → services |
| 3. Handler | `.../api-admin/src/v1/foo.rs` → `mod.rs` merge；路径 `/v1/admin/...` |
| 4. 映射 | [`web/apps/admin/app/utils/phpMap.ts`](../../web/apps/admin/app/utils/phpMap.ts) 增加 `m=&c=&a=` → 新 path；`transformReq/Res` 对齐 PHP 形状 |
| 5. 页面 | 需要时改 `admin-php/...`；薄壳 `pages/*.vue` |
| 6. 长尾 | 暂不具名化时可走 `admin_php_content_service::dispatch`（仍禁止通用 invoke HTTP） |
| 7. 契约 | 具名路径更新 `doc/snapshots/admin_paths.txt` + `admin.openapi.json`；`pnpm gen:types:admin` |

PHP 列表筛选项常用字符串 `"0"` / `""`：Rust 用 `de_loose_i32_opt`。见 [rust/api.md](./rust/api.md)、[web/admin.md](./web/admin.md)。

**验：** 后台登录后打开对应页；Network 里应是 `/admin/api/proxy/v1/admin/...`，HTTP 200，列表有数据。`usertype` 必须为 `9`。

---

## 3. 新 PC / H5 页

| 步骤 | 改哪里 |
|---|---|
| 1. 路由 | `web/apps/site/app/pages/<path>.vue` |
| 2. 切皮 | 模板 `.site-pc` / `.site-h5`（或复用 layers/ui 组件） |
| 3. 数据 | `useApi()`；公开用 `/v1/wap`，会员用 `/v1/mcenter` |
| 4. 文案 | `apps/site/i18n/locales/{zh,en}.json` |
| 5. 模块 | 新模块补 [`site.ts`](../../web/layers/ui/app/utils/site.ts) `PATH_TO_MODULE` / `MODULE_PATH` |
| 6. 会员 | `/user` 或 `/com` 下加页；`useMemberNav`；约定见 [member-center.md](./features/member-center.md) |

**验：** `pnpm dev:site` 或现网 `:3001`；桌面宽度 ≥1200 与手机宽度各看一遍；会员页确认未登录会跳 `/login?next=`。

---

## 4. 改文案（四层，勿混）

详见 [rust/i18n.md](./rust/i18n.md)。

| 改什么 | 动哪里 | 不要动 |
|---|---|---|
| 按钮 / 表头 / 页面字 | `web/apps/site|admin/i18n/locales/{zh,en}.json` | Rust `locales/`、`uploads/` 语言包 |
| 接口 `msg` / 枚举标签 | `phpyun-rs/locales/{en,zh-CN,zh-TW}.json` + `enum_labels` | 把 Vue 1.6 万 key 整包打进 Rust |
| 地区 / 职位类名等运营词 | `phpyun_dict_i18n`（及 bundled `data/*-en.json` 补缺） | 请求时现场拼中英文 |
| 历史 blob（套餐名、日志 content） | 展示用 `packedText` / `packedLog` | 用户自写正文不要 packed |
| CMS `/get/:id` | `phpyun-rs/cms-pages/{zh-CN,en}/` | Vue 大 JSON |

默认语言 **en**；前台 cookie `lang`，后台 `admin_lang`。BFF 只发单条 `Accept-Language`。

---

## 5. 编译与重启速查

```bash
# Rust（现网推荐）
ops/restart.sh rust --build

# 手工等价
cd phpyun-rs
TMPDIR=/var/tmp/cargo-tmp CARGO_TARGET_DIR=$PWD/target \
  cargo build -p phpyun-rs --offline -j 1
sudo systemctl restart test-jobs-phpyun-rs-3003

# Web
ops/restart.sh site    # 或 status 看 unit 名
# 本机
cd web && RUST_API_URL=http://127.0.0.1:3003 pnpm dev:site
```

磁盘紧：删 `target-link`、`target/debug/incremental`、`target/release`，**不要** `cargo clean`。见 [rust/run.md](./rust/run.md)。

## 6. 完成前自检

| 改动 | 至少做 |
|---|---|
| Rust | `cargo build -p phpyun-rs --offline -j 1`；改公开路由则 snapshot 测试；能 curl 则打相关 path |
| Web 页 | 浏览器走主路径；会员再测未登录 / 串端；布局测 PC+H5 |
| 后台 | 登录后打开页；确认 phpMap 命中、非「未映射」 |
| i18n | zh / en 各看一眼；接口错误 `msg` 已是译文 |

声称「好了」之前必须自己跑过验证（`verify-before-done`）。
