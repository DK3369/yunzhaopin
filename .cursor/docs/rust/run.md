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

unit 的 `ExecStart` 是 **debug** binary：`phpyun-rs/target/debug/phpyun-rs`。  
`PHPYUN_ENV_FILE=/www/wwwroot/zzzz.com/phpyun-rs/.env`，`BIND=127.0.0.1:3003`。

`APP_ENV`：`dev` / `test` / `prod`。debug 默认同 `.env.dev`，release 同 `.env.pro`。现网 unit 显式指 `.env`（库 **jobs**）。`.env.dev` 是测试库 `phpyun_test`。改业务只写 **jobs**，不要写库名 `phpyun`。

## 探活

| 地址 | 用途 |
|---|---|
| `http://127.0.0.1:3003/health` | 进程活着 |
| `http://127.0.0.1:3003/ready` | 依赖就绪 |
| `http://127.0.0.1:3003/dev/token` | 仅 debug：求职者 / 企业 / 后台 JWT |
| `http://127.0.0.1:3003/api-docs/v1/openapi.json` | App 契约（dev/test） |
| `http://127.0.0.1:3003/api-docs/admin/openapi.json` | Admin 契约 |

公网：nginx `/yapi/` `/callback/` `/v1/` → `:3003`。nginx `/` 与 `/admin/` → site `:3001`。`RUST_API_URL` 必须是 `http://127.0.0.1:3003`。

## 验证改接口

- `php` 不涉及；Rust：`php -l` 无对应项，用 `cargo build -p phpyun-rs --offline -j 1`。
- 能 `curl` 则打相关路径，确认不是 500（PHP 筛选项经常是字符串，见 [api.md](./api.md)）。
- 全量 `openapi_snapshot` 可能链接 OOM；优先 `cargo test -p phpyun-core --lib` 或相关 crate `--lib`。
