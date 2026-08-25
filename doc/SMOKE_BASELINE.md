# endpoint_smoke 基线

测试：`phpyun-handlers` 的 `tests/endpoint_smoke.rs`（仍 `#[ignore]`，需要 MySQL + Redis）。

```bash
cd phpyun-rs
cargo test -p phpyun-handlers --test endpoint_smoke -- --ignored --nocapture
```

Admin 路由在拆分后由 `apps/server` 的 `assemble` 挂载；该冒烟扫的是 v1 OpenAPI（wap + mcenter）。OpenAPI 契约测试：

```bash
cargo test -p phpyun-handlers --test openapi_snapshot
cargo test -p phpyun-rs --test openapi_contract
```

在本环境跑通 cargo 后，把 5xx 清单按模块记在本文件「失败分类」下。
