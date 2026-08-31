# 依赖

完整表：[doc/RUST_DEPENDENCIES.md](../../../doc/RUST_DEPENDENCIES.md)（以 `phpyun-rs/Cargo.lock` 为准，不要信 IDE「600+」）。

## 计数口径（会变，改 lock 后刷新原文）

- workspace 自己的 crate：7 个 `phpyun-*`
- 锁文件 crate 名约 400；Linux **不编** `cfg(windows)` 那一撮，但 lock 里删不掉
- 直接依赖在 `[workspace.dependencies]`

## 不要做

- 不要为了「少 Windows 包」手改 `Cargo.lock` 或设 `[build] target = x86_64-unknown-linux-gnu`（会把 binary 挪出 systemd 用的 `target/debug/phpyun-rs`）。
- 不要把 `reqwest` 改回 aws-lc；现网是 `rustls-no-provider` + `ring`，启动前 `install_default`。
- 不要给 sqlx 打开 default（会把 postgres/sqlite 编进来）。`tower_governor` 不要默认 feature（会拉 tonic）。
- 砍 `utoipa` / `captcha` / `metrics-exporter-prometheus` / `rust-i18n` 会改行为，须用户点名。

编的时候：`TMPDIR=/var/tmp/cargo-tmp`、`CARGO_TARGET_DIR=phpyun-rs/target`、`-j 1`、默认 `--offline`。
