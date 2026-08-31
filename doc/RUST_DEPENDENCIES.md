# Rust 依赖清单（phpyun-rs）

> 2026-08-31 · 以仓库内 `phpyun-rs/Cargo.lock` 为准，不是 IDE 计数。

## 怎么数

| 口径 | 数量 | 含义 |
|---|---:|---|
| 本仓库 crate | 7 | `phpyun-*` workspace members |
| 锁文件 crate 名 | **398** | `[[package]]` 去重后的 name |
| 锁文件 (name, version) | **425** | 同一名字多版本会重复计 |
| 外部 (name, version) | 418 | 去掉 7 个本仓库 crate |
| 直接依赖 | 33 行（约 40 个 crate） | `[workspace.dependencies]` |
| Cargo.lock 行数 | 4393 | 含 checksum / 依赖列表 |

IDE 或 `cargo tree` 有时显示 **600+**：那是把重复边、多版本、以及 Linux 根本不会编的 `windows_*` / `wasm-bindgen` 也算进去。直接依赖大约 40 个，其余几乎全是 axum / tokio / sqlx / rustls 间接拉进来的。再减到一百以内要换栈，这次不做。

## 这次减掉的虚胖（feature，不改接口）

| 改动 | 效果 |
|---|---|
| `sqlx` `default-features = false`，只开 `mysql` + `runtime-tokio-rustls` + `chrono` + `migrate` + `macros` + `json` | 运行时不再编 Postgres/SQLite 驱动 |
| `tower_governor` 关掉默认 `tonic`，只留 `axum` | 去掉 gRPC 栈（`tonic` 已不在锁里） |
| `reqwest` `rustls` → `rustls-no-provider` + 显式 `rustls`/`ring` | 去掉 aws-lc / quinn；启动时 `install_default` ring provider |

锁文件里仍可能列出 sqlx 的 **optional** 包名（`sqlx-postgres`、`sqlx-sqlite`、`libsqlite3-sys`）。当前 Linux 编译图不会用它们（不会进 binary）。Cargo 会把 crate 声明过的 optional 依赖记在 lock 条目里。

## 直接依赖（workspace）

| crate | 特征 | 干什么 | 能否去掉 |
|---|---|---|---|
| axum | 0.8 | HTTP 框架 | 要 |
| tower | 0.5 limit+util | 并发上限 / ServiceExt | 要 |
| tower-http | 0.6 gzip/cors/trace/… | 中间件 | gzip 可改 nginx 后关 |
| tower_governor | 0.8 axum only | 按 IP 限流 | 要（已关 tonic） |
| tokio | 1.52 | 异步运行时 | 要 |
| tokio-util | 0.7 rt | CancellationToken | 要 |
| tokio-stream | 0.1 | Redis pubsub | 要 |
| sqlx | 0.8 mysql+rustls+macros | MySQL | 要（已关 default/any） |
| redis | 1.2 connection-manager | 缓存 / 限流 / pubsub | 要 |
| reqwest | 0.13 json+rustls-no-provider | 微信/短信出站 HTTP | 要 |
| rustls | 0.23 ring | 给 reqwest 装 process-wide crypto provider | 要（避免 aws-lc） |
| jsonwebtoken | 10 rust_crypto | JWT | 要 |
| argon2 | 0.5 | 密码哈希 | 要 |
| md-5 | 0.10 | PHP 旧密码 / 支付通知 | 要 |
| sha1 | 0.10 | 微信 OA 签名 | 要 |
| serde / serde_json | 1 | JSON | 要 |
| tracing / tracing-subscriber | 0.1 / 0.3 | 日志 | 要 |
| metrics / metrics-exporter-prometheus | 0.24 / 0.16 http-listener | :9091 | 要 |
| anyhow | 1 | 启动期错误 | 要 |
| arc-swap | 1 | 字典/地区热缓存 | 要 |
| rust-i18n | 3 | 后端文案 | 要 |
| chrono | 0.4 serde | 时间 | 要 |
| uuid | 1 v7 | 订单号/上传 key | 要 |
| dotenvy | 0.15 | 读 .env | 要 |
| cron | 0.16 | 调度表达式 | 要 |
| async-trait | 0.1 | dyn Storage/SMS/OAuth | 要 |
| base64 | 0.22 | 验证码图 / 幂等缓存 | 要 |
| bytes | 1 | storage / events | 要 |
| captcha | 0.0.9 | 验证码 PNG | 要（会拉 image/png） |
| moka | 0.12 future | 进程内缓存 | 要 |
| validator | 0.20 derive | 请求校验 | 要 |
| axum-test | 17 | handlers 集成测试 | 仅 dev |
| utoipa | 5 axum_extras+chrono | OpenAPI JSON | 开发契约；prod 不挂路由 |

## 各 workspace crate 声明的依赖

| crate | 依赖 |
|---|---|
| `phpyun-rs (server)` | anyhow, axum, tokio, tracing, handlers, services, api-admin, core |
| `phpyun-core` | axum, tower, tower-http, tower_governor, tokio*, sqlx, redis, reqwest, rustls, serde*, tracing*, anyhow, uuid, moka, metrics*, validator, utoipa, async-trait, base64, bytes, jsonwebtoken, chrono, dotenvy, cron, rust-i18n |
| `phpyun-auth` | argon2, md-5, tokio, anyhow, core |
| `phpyun-models` | sqlx, serde*, utoipa, chrono, core |
| `phpyun-services` | serde*, tokio*, tracing, uuid, captcha, base64, sha1, md-5, sqlx, core, auth, models, arc-swap, chrono |
| `phpyun-handlers (api)` | axum, tokio, serde*, validator, tracing, core, services, models, utoipa, uuid；dev: axum-test, tower, tracing-subscriber, sqlx |
| `phpyun-api-admin` | axum, serde*, validator, tracing, utoipa, core, services, handlers, models, uuid |

## 按来源分组（锁文件里还在的关键包）

### HTTP 服务

| crate | version | 谁直接依赖它 |
|---|---|---|
| `axum` | 0.8.9 | axum-test, phpyun-api-admin, phpyun-core, phpyun-handlers, phpyun-rs, tower_governor |
| `tower` | 0.5.3 | axum, axum-test, phpyun-core, phpyun-handlers, reqwest, tower-http, tower_governor |
| `tower-http` | 0.6.11 | phpyun-core, reqwest |
| `tower_governor` | 0.8.0 | phpyun-core |
| `hyper` | 1.11.0 | axum, axum-test, hyper-rustls, hyper-util, metrics-exporter-prometheus, reqwest |
| `hyper-util` | 0.1.20 | axum, axum-test, hyper-rustls, metrics-exporter-prometheus, reqwest |
| `http` | 1.5.0 | axum, axum-core, axum-test, h2, http-body, http-body-util, hyper, hyper-rustls, hyper-util, reqwest, rust-multipart-rfc7578_2, tower-http, tower_governor |
| `http-body` | 1.1.0 | axum, axum-core, http-body-util, hyper, hyper-util, reqwest, tower-http |
| `http-body-util` | 0.1.4 | axum, axum-core, axum-test, metrics-exporter-prometheus, reqwest, tower-http |

### 运行时

| crate | version | 谁直接依赖它 |
|---|---|---|
| `tokio` | 1.53.1 | async-compression, axum, axum-test, combine, h2, hyper, hyper-rustls, hyper-util, metrics-exporter-prometheus, phpyun-auth, phpyun-core, phpyun-handlers, phpyun-rs, phpyun-services, redis, reqwest, sqlx-core, sqlx-macros-core, tokio-rustls, tokio-stream, tokio-util, tower, tower-http |
| `tokio-util` | 0.7.19 | combine, h2, phpyun-core, redis, tower, tower-http |
| `tokio-stream` | 0.1.19 | phpyun-core, phpyun-services, sqlx-core |
| `tokio-rustls` | 0.26.4 | hyper-rustls, reqwest |
| `futures-util` | 0.3.33 | axum, futures-executor, governor, hyper-util, js-sys, moka, redis, rust-multipart-rfc7578_2, sqlx-core, sqlx-mysql, sqlx-postgres, sqlx-sqlite, tokio-util, tower, tower-http |
| `futures-core` | 0.3.33 | axum-core, combine, flume, futures-channel, futures-executor, futures-intrusive, futures-util, h2, http-body-util, hyper, reqwest, rust-multipart-rfc7578_2, sqlx-core, sqlx-mysql, sqlx-postgres, sqlx-sqlite, sync_wrapper, tokio-stream, tokio-util, tower, tower-http |
| `pin-project-lite` | 0.2.17 | async-compression, async-lock, axum, axum-core, combine, event-listener, event-listener-strategy, futures-util, http-body-util, hyper, hyper-util, redis, reqwest, tokio, tokio-stream, tokio-util, tower, tower-http, tracing |

### MySQL

| crate | version | 谁直接依赖它 |
|---|---|---|
| `sqlx` | 0.8.6 | phpyun-core, phpyun-handlers, phpyun-models, phpyun-services |
| `sqlx-core` | 0.8.6 | sqlx, sqlx-macros, sqlx-macros-core, sqlx-mysql, sqlx-postgres, sqlx-sqlite |
| `sqlx-mysql` | 0.8.6 | sqlx, sqlx-macros-core |
| `sqlx-macros` | 0.8.6 | sqlx |
| `sqlx-macros-core` | 0.8.6 | sqlx-macros |

### Redis

| crate | version | 谁直接依赖它 |
|---|---|---|
| `redis` | 1.5.0 | phpyun-core |
| `combine` | 4.6.7 | jni, redis |
| `arcstr` | 1.2.0 | redis |

### 出站 HTTP / TLS

| crate | version | 谁直接依赖它 |
|---|---|---|
| `reqwest` | 0.13.4 | phpyun-core |
| `hyper-rustls` | 0.27.9 | reqwest |
| `rustls` | 0.23.43 | hyper-rustls, reqwest, rustls-platform-verifier, sqlx-core, tokio-rustls |
| `rustls-webpki` | 0.103.13 | rustls, rustls-platform-verifier |
| `rustls-pki-types` | 1.15.1 | reqwest, rustls, rustls-native-certs, rustls-webpki, webpki-root-certs, webpki-roots |
| `rustls-platform-verifier` | 0.7.0 | reqwest |
| `ring` | 0.17.14 | rustls, rustls-webpki |
| `webpki-roots` | 0.26.11, 1.0.9 | sqlx-core, webpki-roots |

### 验证码 PNG

| crate | version | 谁直接依赖它 |
|---|---|---|
| `captcha` | 0.0.9 | phpyun-services |
| `image` | 0.24.9 | captcha |
| `png` | 0.17.16 | image |
| `lodepng` | 3.12.2 | captcha |
| `hound` | 3.5.1 | captcha |

### OpenAPI

| crate | version | 谁直接依赖它 |
|---|---|---|
| `utoipa` | 5.5.0 | phpyun-api-admin, phpyun-core, phpyun-handlers, phpyun-models, phpyun-rs |
| `utoipa-gen` | 5.5.0 | utoipa |

### i18n

| crate | version | 谁直接依赖它 |
|---|---|---|
| `rust-i18n` | 3.1.5 | phpyun-core |
| `rust-i18n-macro` | 3.1.5 | rust-i18n |
| `rust-i18n-support` | 3.1.5 | rust-i18n, rust-i18n-macro |
| `globwalk` | 0.8.1 | rust-i18n, rust-i18n-support |
| `regex` | 1.13.1 | rust-i18n, rust-i18n-support, utoipa-gen, validator |

### 可观测

| crate | version | 谁直接依赖它 |
|---|---|---|
| `tracing` | 0.1.44 | axum, axum-core, h2, hyper-util, metrics-exporter-prometheus, phpyun-api-admin, phpyun-core, phpyun-handlers, phpyun-rs, phpyun-services, sqlx-core, sqlx-mysql, sqlx-postgres, sqlx-sqlite, tower, tower-http, tower_governor, tracing-subscriber |
| `tracing-subscriber` | 0.3.23 | phpyun-core, phpyun-handlers |
| `metrics` | 0.24.6 | metrics-exporter-prometheus, metrics-util, phpyun-core |
| `metrics-exporter-prometheus` | 0.16.2 | phpyun-core |
| `metrics-util` | 0.19.1 | metrics-exporter-prometheus |

### 测试（仅 lock / dev）

| crate | version | 谁直接依赖它 |
|---|---|---|
| `axum-test` | 17.3.0 | phpyun-handlers |

## 故意先不动

| 项 | 原因 |
|---|---|
| captcha → image/png/lodepng/hound | 注册/登录验证码 PNG |
| utoipa | 开发环境 OpenAPI JSON |
| rust-i18n | 后端文案 |
| tower-http gzip | 现网 nginx 未开 gzip |
| moka / arc-swap / cron / sha1 / md-5 | 热路径或 PHP 兼容 |

## 完整锁文件（全部 crate）

共 **425** 条 `(name, version)`，按名字排序。`本仓库` = workspace member，其余为 crates.io。

| crate | version | 来源 |
|---|---|---|
| `adler2` | 2.0.1 | 外部 |
| `aho-corasick` | 1.1.5 | 外部 |
| `allocator-api2` | 0.2.21 | 外部 |
| `android_system_properties` | 0.1.6 | 外部 |
| `anyhow` | 1.0.104 | 外部 |
| `arc-swap` | 1.9.2 | 外部 |
| `arcstr` | 1.2.0 | 外部 |
| `argon2` | 0.5.3 | 外部 |
| `assert-json-diff` | 2.0.2 | 外部 |
| `async-compression` | 0.4.43 | 外部 |
| `async-lock` | 3.4.2 | 外部 |
| `async-trait` | 0.1.91 | 外部 |
| `atoi` | 2.0.0 | 外部 |
| `atomic-waker` | 1.1.2 | 外部 |
| `auto-future` | 1.0.0 | 外部 |
| `autocfg` | 1.5.1 | 外部 |
| `axum` | 0.8.9 | 外部 |
| `axum-core` | 0.5.6 | 外部 |
| `axum-test` | 17.3.0 | 外部 |
| `backon` | 1.6.0 | 外部 |
| `base16ct` | 0.2.0 | 外部 |
| `base62` | 2.2.4 | 外部 |
| `base64` | 0.13.1 | 外部 |
| `base64` | 0.22.1 | 外部 |
| `base64ct` | 1.8.3 | 外部 |
| `bitflags` | 1.3.2 | 外部 |
| `bitflags` | 2.13.1 | 外部 |
| `blake2` | 0.10.6 | 外部 |
| `block-buffer` | 0.10.4 | 外部 |
| `bstr` | 1.13.0 | 外部 |
| `bumpalo` | 3.20.3 | 外部 |
| `bytemuck` | 1.25.2 | 外部 |
| `byteorder` | 1.5.0 | 外部 |
| `bytes` | 1.12.1 | 外部 |
| `bytesize` | 2.7.0 | 外部 |
| `captcha` | 0.0.9 | 外部 |
| `cc` | 1.4.0 | 外部 |
| `cfg-if` | 1.0.4 | 外部 |
| `chrono` | 0.4.45 | 外部 |
| `color_quant` | 1.1.0 | 外部 |
| `combine` | 4.6.7 | 外部 |
| `compression-codecs` | 0.4.38 | 外部 |
| `compression-core` | 0.4.32 | 外部 |
| `const-oid` | 0.9.6 | 外部 |
| `cookie` | 0.18.1 | 外部 |
| `core-foundation` | 0.10.1 | 外部 |
| `core-foundation-sys` | 0.8.7 | 外部 |
| `cpufeatures` | 0.2.17 | 外部 |
| `crc` | 3.4.0 | 外部 |
| `crc-catalog` | 2.5.0 | 外部 |
| `crc32fast` | 1.5.0 | 外部 |
| `cron` | 0.16.0 | 外部 |
| `crossbeam-channel` | 0.5.16 | 外部 |
| `crossbeam-deque` | 0.8.7 | 外部 |
| `crossbeam-epoch` | 0.9.20 | 外部 |
| `crossbeam-queue` | 0.3.13 | 外部 |
| `crossbeam-utils` | 0.8.22 | 外部 |
| `crypto-bigint` | 0.5.5 | 外部 |
| `crypto-common` | 0.1.6 | 外部 |
| `curve25519-dalek` | 4.1.3 | 外部 |
| `curve25519-dalek-derive` | 0.1.1 | 外部 |
| `darling` | 0.23.0 | 外部 |
| `darling_core` | 0.23.0 | 外部 |
| `darling_macro` | 0.23.0 | 外部 |
| `dashmap` | 6.2.1 | 外部 |
| `der` | 0.7.10 | 外部 |
| `deranged` | 0.5.8 | 外部 |
| `diff` | 0.1.13 | 外部 |
| `digest` | 0.10.7 | 外部 |
| `displaydoc` | 0.2.7 | 外部 |
| `dotenvy` | 0.15.7 | 外部 |
| `ecdsa` | 0.16.9 | 外部 |
| `ed25519` | 2.2.3 | 外部 |
| `ed25519-dalek` | 2.2.0 | 外部 |
| `either` | 1.17.0 | 外部 |
| `elliptic-curve` | 0.13.8 | 外部 |
| `equivalent` | 1.0.2 | 外部 |
| `errno` | 0.3.14 | 外部 |
| `etcetera` | 0.8.0 | 外部 |
| `event-listener` | 5.4.2 | 外部 |
| `event-listener-strategy` | 0.5.4 | 外部 |
| `fastrand` | 2.5.0 | 外部 |
| `fdeflate` | 0.3.7 | 外部 |
| `ff` | 0.13.1 | 外部 |
| `fiat-crypto` | 0.2.9 | 外部 |
| `find-msvc-tools` | 0.1.9 | 外部 |
| `flate2` | 1.1.9 | 外部 |
| `flume` | 0.11.1 | 外部 |
| `fnv` | 1.0.7 | 外部 |
| `foldhash` | 0.1.5 | 外部 |
| `foldhash` | 0.2.0 | 外部 |
| `form_urlencoded` | 1.2.2 | 外部 |
| `forwarded-header-value` | 0.1.1 | 外部 |
| `futures-channel` | 0.3.33 | 外部 |
| `futures-core` | 0.3.33 | 外部 |
| `futures-executor` | 0.3.33 | 外部 |
| `futures-intrusive` | 0.5.0 | 外部 |
| `futures-io` | 0.3.33 | 外部 |
| `futures-macro` | 0.3.33 | 外部 |
| `futures-sink` | 0.3.33 | 外部 |
| `futures-task` | 0.3.33 | 外部 |
| `futures-timer` | 3.0.4 | 外部 |
| `futures-util` | 0.3.33 | 外部 |
| `generic-array` | 0.14.9 | 外部 |
| `getrandom` | 0.2.17 | 外部 |
| `getrandom` | 0.3.4 | 外部 |
| `getrandom` | 0.4.3 | 外部 |
| `glob` | 0.3.4 | 外部 |
| `globset` | 0.4.20 | 外部 |
| `globwalk` | 0.8.1 | 外部 |
| `governor` | 0.10.4 | 外部 |
| `group` | 0.13.0 | 外部 |
| `h2` | 0.4.15 | 外部 |
| `hashbrown` | 0.14.5 | 外部 |
| `hashbrown` | 0.15.5 | 外部 |
| `hashbrown` | 0.16.1 | 外部 |
| `hashbrown` | 0.17.1 | 外部 |
| `hashlink` | 0.10.0 | 外部 |
| `heck` | 0.5.0 | 外部 |
| `hex` | 0.4.3 | 外部 |
| `hkdf` | 0.12.4 | 外部 |
| `hmac` | 0.12.1 | 外部 |
| `home` | 0.5.12 | 外部 |
| `hound` | 3.5.1 | 外部 |
| `http` | 1.5.0 | 外部 |
| `http-body` | 1.1.0 | 外部 |
| `http-body-util` | 0.1.4 | 外部 |
| `httparse` | 1.10.1 | 外部 |
| `httpdate` | 1.0.3 | 外部 |
| `hyper` | 1.11.0 | 外部 |
| `hyper-rustls` | 0.27.9 | 外部 |
| `hyper-util` | 0.1.20 | 外部 |
| `iana-time-zone` | 0.1.65 | 外部 |
| `iana-time-zone-haiku` | 0.1.2 | 外部 |
| `icu_collections` | 2.2.0 | 外部 |
| `icu_locale_core` | 2.2.0 | 外部 |
| `icu_normalizer` | 2.2.0 | 外部 |
| `icu_normalizer_data` | 2.2.0 | 外部 |
| `icu_properties` | 2.2.0 | 外部 |
| `icu_properties_data` | 2.2.0 | 外部 |
| `icu_provider` | 2.2.0 | 外部 |
| `ident_case` | 1.0.1 | 外部 |
| `idna` | 1.1.0 | 外部 |
| `idna_adapter` | 1.2.2 | 外部 |
| `ignore` | 0.4.33 | 外部 |
| `image` | 0.24.9 | 外部 |
| `indexmap` | 2.14.0 | 外部 |
| `ipnet` | 2.12.1 | 外部 |
| `itertools` | 0.11.0 | 外部 |
| `itoa` | 1.0.18 | 外部 |
| `jni` | 0.22.4 | 外部 |
| `jni-macros` | 0.22.4 | 外部 |
| `jni-sys` | 0.4.1 | 外部 |
| `jni-sys-macros` | 0.4.1 | 外部 |
| `js-sys` | 0.3.103 | 外部 |
| `jsonwebtoken` | 10.4.0 | 外部 |
| `lazy_static` | 1.5.0 | 外部 |
| `libc` | 0.2.189 | 外部 |
| `libm` | 0.2.16 | 外部 |
| `libredox` | 0.1.19 | 外部 |
| `libsqlite3-sys` | 0.30.1 | 外部 |
| `litemap` | 0.8.2 | 外部 |
| `lock_api` | 0.4.14 | 外部 |
| `lodepng` | 3.12.2 | 外部 |
| `log` | 0.4.33 | 外部 |
| `matchers` | 0.2.0 | 外部 |
| `matchit` | 0.8.4 | 外部 |
| `md-5` | 0.10.6 | 外部 |
| `memchr` | 2.8.3 | 外部 |
| `metrics` | 0.24.6 | 外部 |
| `metrics-exporter-prometheus` | 0.16.2 | 外部 |
| `metrics-util` | 0.19.1 | 外部 |
| `mime` | 0.3.17 | 外部 |
| `miniz_oxide` | 0.8.9 | 外部 |
| `mio` | 1.2.2 | 外部 |
| `moka` | 0.12.15 | 外部 |
| `nonempty` | 0.7.0 | 外部 |
| `nonzero_ext` | 0.3.0 | 外部 |
| `normpath` | 1.5.1 | 外部 |
| `nu-ansi-term` | 0.50.3 | 外部 |
| `num-bigint` | 0.4.8 | 外部 |
| `num-bigint-dig` | 0.8.6 | 外部 |
| `num-conv` | 0.2.2 | 外部 |
| `num-integer` | 0.1.46 | 外部 |
| `num-iter` | 0.1.46 | 外部 |
| `num-traits` | 0.2.19 | 外部 |
| `once_cell` | 1.21.4 | 外部 |
| `openssl-probe` | 0.2.1 | 外部 |
| `p256` | 0.13.2 | 外部 |
| `p384` | 0.13.1 | 外部 |
| `parking` | 2.2.1 | 外部 |
| `parking_lot` | 0.12.5 | 外部 |
| `parking_lot_core` | 0.9.12 | 外部 |
| `password-hash` | 0.5.0 | 外部 |
| `pem-rfc7468` | 0.7.0 | 外部 |
| `percent-encoding` | 2.3.2 | 外部 |
| `phf` | 0.11.3 | 外部 |
| `phf_generator` | 0.11.3 | 外部 |
| `phf_macros` | 0.11.3 | 外部 |
| `phf_shared` | 0.11.3 | 外部 |
| `phpyun-api-admin` | 0.1.2 | 本仓库 |
| `phpyun-auth` | 0.1.2 | 本仓库 |
| `phpyun-core` | 0.1.2 | 本仓库 |
| `phpyun-handlers` | 0.1.2 | 本仓库 |
| `phpyun-models` | 0.1.2 | 本仓库 |
| `phpyun-rs` | 0.1.2 | 本仓库 |
| `phpyun-services` | 0.1.2 | 本仓库 |
| `pin-project` | 1.1.13 | 外部 |
| `pin-project-internal` | 1.1.13 | 外部 |
| `pin-project-lite` | 0.2.17 | 外部 |
| `pkcs1` | 0.7.5 | 外部 |
| `pkcs8` | 0.10.2 | 外部 |
| `pkg-config` | 0.3.33 | 外部 |
| `plain` | 0.2.3 | 外部 |
| `png` | 0.17.16 | 外部 |
| `portable-atomic` | 1.14.0 | 外部 |
| `potential_utf` | 0.1.5 | 外部 |
| `powerfmt` | 0.2.0 | 外部 |
| `ppv-lite86` | 0.2.21 | 外部 |
| `pretty_assertions` | 1.4.1 | 外部 |
| `primeorder` | 0.13.6 | 外部 |
| `proc-macro-error-attr3` | 3.1.0 | 外部 |
| `proc-macro-error3` | 3.1.0 | 外部 |
| `proc-macro2` | 1.0.107 | 外部 |
| `quanta` | 0.12.6 | 外部 |
| `quote` | 1.0.47 | 外部 |
| `r-efi` | 5.3.0 | 外部 |
| `r-efi` | 6.0.0 | 外部 |
| `rand` | 0.8.7 | 外部 |
| `rand` | 0.9.5 | 外部 |
| `rand_chacha` | 0.3.1 | 外部 |
| `rand_chacha` | 0.9.0 | 外部 |
| `rand_core` | 0.6.4 | 外部 |
| `rand_core` | 0.9.5 | 外部 |
| `rand_xoshiro` | 0.7.0 | 外部 |
| `rapidhash` | 4.5.1 | 外部 |
| `raw-cpuid` | 11.6.0 | 外部 |
| `redis` | 1.5.0 | 外部 |
| `redox_syscall` | 0.5.18 | 外部 |
| `redox_syscall` | 0.9.1 | 外部 |
| `regex` | 1.13.1 | 外部 |
| `regex-automata` | 0.4.18 | 外部 |
| `regex-syntax` | 0.8.11 | 外部 |
| `reqwest` | 0.13.4 | 外部 |
| `reserve-port` | 2.5.0 | 外部 |
| `rfc6979` | 0.4.0 | 外部 |
| `rgb` | 0.8.53 | 外部 |
| `ring` | 0.17.14 | 外部 |
| `rsa` | 0.9.10 | 外部 |
| `rust-i18n` | 3.1.5 | 外部 |
| `rust-i18n-macro` | 3.1.5 | 外部 |
| `rust-i18n-support` | 3.1.5 | 外部 |
| `rust-multipart-rfc7578_2` | 0.8.0 | 外部 |
| `rustc_version` | 0.4.1 | 外部 |
| `rustls` | 0.23.43 | 外部 |
| `rustls-native-certs` | 0.8.4 | 外部 |
| `rustls-pki-types` | 1.15.1 | 外部 |
| `rustls-platform-verifier` | 0.7.0 | 外部 |
| `rustls-platform-verifier-android` | 0.1.1 | 外部 |
| `rustls-webpki` | 0.103.13 | 外部 |
| `rustversion` | 1.0.23 | 外部 |
| `ryu` | 1.0.23 | 外部 |
| `same-file` | 1.0.6 | 外部 |
| `schannel` | 0.1.29 | 外部 |
| `scopeguard` | 1.2.0 | 外部 |
| `sec1` | 0.7.3 | 外部 |
| `security-framework` | 3.7.0 | 外部 |
| `security-framework-sys` | 2.17.0 | 外部 |
| `semver` | 1.0.28 | 外部 |
| `serde` | 1.0.229 | 外部 |
| `serde_core` | 1.0.229 | 外部 |
| `serde_derive` | 1.0.229 | 外部 |
| `serde_json` | 1.0.151 | 外部 |
| `serde_path_to_error` | 0.1.20 | 外部 |
| `serde_spanned` | 0.6.9 | 外部 |
| `serde_urlencoded` | 0.7.1 | 外部 |
| `serde_yaml` | 0.9.34+deprecated | 外部 |
| `sha1` | 0.10.7 | 外部 |
| `sha1_smol` | 1.0.1 | 外部 |
| `sha2` | 0.10.9 | 外部 |
| `sharded-slab` | 0.1.7 | 外部 |
| `shlex` | 2.0.1 | 外部 |
| `signal-hook-registry` | 1.4.8 | 外部 |
| `signature` | 2.2.0 | 外部 |
| `simd-adler32` | 0.3.10 | 外部 |
| `simd_cesu8` | 1.2.0 | 外部 |
| `simdutf8` | 0.1.5 | 外部 |
| `siphasher` | 1.0.3 | 外部 |
| `sketches-ddsketch` | 0.3.1 | 外部 |
| `slab` | 0.4.12 | 外部 |
| `smallvec` | 1.15.2 | 外部 |
| `socket2` | 0.6.5 | 外部 |
| `spin` | 0.9.9 | 外部 |
| `spinning_top` | 0.3.0 | 外部 |
| `spki` | 0.7.3 | 外部 |
| `sqlx` | 0.8.6 | 外部 |
| `sqlx-core` | 0.8.6 | 外部 |
| `sqlx-macros` | 0.8.6 | 外部 |
| `sqlx-macros-core` | 0.8.6 | 外部 |
| `sqlx-mysql` | 0.8.6 | 外部 |
| `sqlx-postgres` | 0.8.6 | 外部 |
| `sqlx-sqlite` | 0.8.6 | 外部 |
| `stable_deref_trait` | 1.2.1 | 外部 |
| `stringprep` | 0.1.5 | 外部 |
| `strsim` | 0.11.1 | 外部 |
| `subtle` | 2.6.1 | 外部 |
| `syn` | 2.0.119 | 外部 |
| `syn` | 3.0.3 | 外部 |
| `sync_wrapper` | 1.0.2 | 外部 |
| `synstructure` | 0.13.2 | 外部 |
| `tagptr` | 0.2.0 | 外部 |
| `thiserror` | 1.0.69 | 外部 |
| `thiserror` | 2.0.19 | 外部 |
| `thiserror-impl` | 1.0.69 | 外部 |
| `thiserror-impl` | 2.0.19 | 外部 |
| `thread_local` | 1.1.10 | 外部 |
| `time` | 0.3.55 | 外部 |
| `time-core` | 0.1.9 | 外部 |
| `time-macros` | 0.2.32 | 外部 |
| `tinystr` | 0.8.3 | 外部 |
| `tinyvec` | 1.12.0 | 外部 |
| `tinyvec_macros` | 0.1.1 | 外部 |
| `tokio` | 1.53.1 | 外部 |
| `tokio-macros` | 2.7.2 | 外部 |
| `tokio-rustls` | 0.26.4 | 外部 |
| `tokio-stream` | 0.1.19 | 外部 |
| `tokio-util` | 0.7.19 | 外部 |
| `toml` | 0.8.23 | 外部 |
| `toml_datetime` | 0.6.11 | 外部 |
| `toml_edit` | 0.22.27 | 外部 |
| `toml_write` | 0.1.2 | 外部 |
| `tower` | 0.5.3 | 外部 |
| `tower-http` | 0.6.11 | 外部 |
| `tower-layer` | 0.3.3 | 外部 |
| `tower-service` | 0.3.3 | 外部 |
| `tower_governor` | 0.8.0 | 外部 |
| `tracing` | 0.1.44 | 外部 |
| `tracing-attributes` | 0.1.31 | 外部 |
| `tracing-core` | 0.1.36 | 外部 |
| `tracing-log` | 0.2.0 | 外部 |
| `tracing-serde` | 0.2.0 | 外部 |
| `tracing-subscriber` | 0.3.23 | 外部 |
| `triomphe` | 0.1.16 | 外部 |
| `try-lock` | 0.2.5 | 外部 |
| `typenum` | 1.20.1 | 外部 |
| `unicode-bidi` | 0.3.18 | 外部 |
| `unicode-ident` | 1.0.24 | 外部 |
| `unicode-normalization` | 0.1.25 | 外部 |
| `unicode-properties` | 0.1.4 | 外部 |
| `unsafe-libyaml` | 0.2.11 | 外部 |
| `untrusted` | 0.9.0 | 外部 |
| `url` | 2.5.8 | 外部 |
| `utf8_iter` | 1.0.4 | 外部 |
| `utoipa` | 5.5.0 | 外部 |
| `utoipa-gen` | 5.5.0 | 外部 |
| `uuid` | 1.24.0 | 外部 |
| `validator` | 0.20.0 | 外部 |
| `validator_derive` | 0.20.1 | 外部 |
| `valuable` | 0.1.1 | 外部 |
| `vcpkg` | 0.2.15 | 外部 |
| `version_check` | 0.9.5 | 外部 |
| `walkdir` | 2.5.0 | 外部 |
| `want` | 0.3.1 | 外部 |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | 外部 |
| `wasip2` | 1.0.4+wasi-0.2.12 | 外部 |
| `wasite` | 0.1.0 | 外部 |
| `wasm-bindgen` | 0.2.126 | 外部 |
| `wasm-bindgen-futures` | 0.4.76 | 外部 |
| `wasm-bindgen-macro` | 0.2.126 | 外部 |
| `wasm-bindgen-macro-support` | 0.2.126 | 外部 |
| `wasm-bindgen-shared` | 0.2.126 | 外部 |
| `web-sys` | 0.3.103 | 外部 |
| `web-time` | 1.1.0 | 外部 |
| `webpki-root-certs` | 1.0.9 | 外部 |
| `webpki-roots` | 0.26.11 | 外部 |
| `webpki-roots` | 1.0.9 | 外部 |
| `whoami` | 1.6.1 | 外部 |
| `winapi` | 0.3.9 | 外部 |
| `winapi-i686-pc-windows-gnu` | 0.4.0 | 外部 |
| `winapi-util` | 0.1.11 | 外部 |
| `winapi-x86_64-pc-windows-gnu` | 0.4.0 | 外部 |
| `windows-core` | 0.62.2 | 外部 |
| `windows-implement` | 0.60.2 | 外部 |
| `windows-interface` | 0.59.3 | 外部 |
| `windows-link` | 0.2.1 | 外部 |
| `windows-result` | 0.4.1 | 外部 |
| `windows-strings` | 0.5.1 | 外部 |
| `windows-sys` | 0.48.0 | 外部 |
| `windows-sys` | 0.52.0 | 外部 |
| `windows-sys` | 0.61.2 | 外部 |
| `windows-targets` | 0.48.5 | 外部 |
| `windows-targets` | 0.52.6 | 外部 |
| `windows_aarch64_gnullvm` | 0.48.5 | 外部 |
| `windows_aarch64_gnullvm` | 0.52.6 | 外部 |
| `windows_aarch64_msvc` | 0.48.5 | 外部 |
| `windows_aarch64_msvc` | 0.52.6 | 外部 |
| `windows_i686_gnu` | 0.48.5 | 外部 |
| `windows_i686_gnu` | 0.52.6 | 外部 |
| `windows_i686_gnullvm` | 0.52.6 | 外部 |
| `windows_i686_msvc` | 0.48.5 | 外部 |
| `windows_i686_msvc` | 0.52.6 | 外部 |
| `windows_x86_64_gnu` | 0.48.5 | 外部 |
| `windows_x86_64_gnu` | 0.52.6 | 外部 |
| `windows_x86_64_gnullvm` | 0.48.5 | 外部 |
| `windows_x86_64_gnullvm` | 0.52.6 | 外部 |
| `windows_x86_64_msvc` | 0.48.5 | 外部 |
| `windows_x86_64_msvc` | 0.52.6 | 外部 |
| `winnow` | 0.7.15 | 外部 |
| `wit-bindgen` | 0.57.1 | 外部 |
| `writeable` | 0.6.3 | 外部 |
| `xxhash-rust` | 0.8.18 | 外部 |
| `yansi` | 1.0.1 | 外部 |
| `yoke` | 0.8.3 | 外部 |
| `yoke-derive` | 0.8.2 | 外部 |
| `zerocopy` | 0.8.56 | 外部 |
| `zerocopy-derive` | 0.8.56 | 外部 |
| `zerofrom` | 0.1.8 | 外部 |
| `zerofrom-derive` | 0.1.7 | 外部 |
| `zeroize` | 1.9.0 | 外部 |
| `zeroize_derive` | 1.5.0 | 外部 |
| `zerotrie` | 0.2.4 | 外部 |
| `zerovec` | 0.11.6 | 外部 |
| `zerovec-derive` | 0.11.3 | 外部 |
| `zlib-rs` | 0.6.7 | 外部 |
| `zmij` | 1.0.23 | 外部 |

多版本（同一名字出现多次）：

`hashbrown`×4, `getrandom`×3, `windows-sys`×3, `base64`×2, `bitflags`×2, `foldhash`×2, `r-efi`×2, `rand`×2, `rand_chacha`×2, `rand_core`×2, `redox_syscall`×2, `syn`×2, `thiserror`×2, `thiserror-impl`×2, `webpki-roots`×2, `windows-targets`×2, `windows_aarch64_gnullvm`×2, `windows_aarch64_msvc`×2, `windows_i686_gnu`×2, `windows_i686_msvc`×2, `windows_x86_64_gnu`×2, `windows_x86_64_gnullvm`×2, `windows_x86_64_msvc`×2

