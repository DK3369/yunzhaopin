# 现网怎么编、怎么跑

本机 **一份** Rust：systemd `test-jobs-phpyun-rs-3003`，**`:3003`**，metrics **`:9091`**，库 **`jobs`**。没有 `:3000`。

## 日常

```bash
/www/wwwroot/zzzz.com/ops/restart.sh                 # rust + site + admin，不编译
/www/wwwroot/zzzz.com/ops/restart.sh rust --build    # cargo --offline 后再重启 :3003
/www/wwwroot/zzzz.com/ops/restart.sh status
```

`--online` 才允许 cargo 联网。不要 `nohup`、不要 `systemctl start test-jobs-phpyun-rs`。

## 手工编译（脚本里的同等命令）

```bash
cd /www/wwwroot/zzzz.com/phpyun-rs
TMPDIR=/var/tmp/cargo-tmp CARGO_TARGET_DIR=/www/wwwroot/zzzz.com/phpyun-rs/target \
  cargo build -p phpyun-rs --offline -j 1
sudo systemctl restart test-jobs-phpyun-rs-3003
```

首次编 `utoipa-swagger-ui` 会下 Swagger UI zip。无网时先有 zip，再 `SWAGGER_UI_DOWNLOAD_URL=file:///var/tmp/swagger-ui-v5.17.14.zip`。`APP_ENV=prod` 不挂 `/docs` 和 `/api-docs`。

unit 的 `ExecStart` 是 **debug** binary：`phpyun-rs/target/debug/phpyun-rs`。  
`PHPYUN_ENV_FILE=/www/wwwroot/zzzz.com/phpyun-rs/.env`，`BIND=127.0.0.1:3003`。

`APP_ENV`：`dev` / `test` / `prod`。debug 默认同 `.env.dev`，release 同 `.env.pro`。现网 unit 显式指 `.env`（库 **jobs**）。`.env.dev` 是测试库 `phpyun_test`。改业务只写 **jobs**，不要写库名 `phpyun`。文本列 **utf8mb4**（见 [utf8.mdc](../../rules/utf8.mdc)）。

## 探活

| 地址 | 用途 |
|---|---|
| `http://127.0.0.1:3003/health` | 进程活着 |
| `http://127.0.0.1:3003/ready` | 依赖就绪 |
| `http://127.0.0.1:3003/dev/token` | 仅 debug：求职者 / 企业 / 后台 JWT。后台是 **`usertype=9`**；校园 `3` 不能当 admin |
| `http://127.0.0.1:3003/docs/` | Swagger UI（仅 dev/test） |
| `https://job1.ov6.com/docs/` | 同上，经 `:3001` 转到 `:3003`（PC/H5 同域） |
| `http://127.0.0.1:3003/api-docs/v1/openapi.json` | App 契约（dev/test） |
| `http://127.0.0.1:3003/api-docs/admin/openapi.json` | Admin 契约 |

PC/H5 与后台浏览器接口走 `https://job1.ov6.com/api/proxy/v1/...`（`:3001` BFF → `:3003`）。不要用 `job1` 的 `/yapi/`（隧道直连 `:3001`，没有这条）。`RUST_API_URL` 必须是 `http://127.0.0.1:3003`。

## 编译产物（`target` vs `target-link`）

| 目录 | 用途 |
|---|---|
| `phpyun-rs/target` | **现网唯一产物目录**。`CARGO_TARGET_DIR`、`.cargo/config.toml`、systemd `ExecStart` 都指这里的 `target/debug/phpyun-rs`。 |
| `phpyun-rs/target-link` | **旧目录，不要再建**。2026-08 留下的第二份 debug，二进制还叫 `app`，现网不用。 |

这台机约 8G RAM，链接容易 OOM，所以 **`-j 1`**、`profile.dev` 只用 `debug = "line-tables-only"`。debug 增量缓存会自己涨到几十 G；磁盘紧时只删下面这些，**不要** `cargo clean`（会逼全量重编，systemd 还指着 `target/debug/phpyun-rs`）：

```bash
rm -rf /www/wwwroot/zzzz.com/phpyun-rs/target-link
rm -rf /www/wwwroot/zzzz.com/phpyun-rs/target/debug/incremental
rm -rf /www/wwwroot/zzzz.com/phpyun-rs/target/release
```

下次 `ops/restart.sh rust --build` 会慢一截（重做增量），不必从零编依赖。

## 验证改接口

- `php` 不涉及；Rust：`php -l` 无对应项，用 `cargo build -p phpyun-rs --offline -j 1`。
- 能 `curl` 则打相关路径，确认不是 500（PHP 筛选项经常是字符串，见 [api.md](./api.md)）。
- 全量 `openapi_snapshot` 可能链接 OOM；优先 `cargo test -p phpyun-core --lib` 或相关 crate `--lib`。
