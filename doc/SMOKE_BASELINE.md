# endpoint_smoke 基线（T2）

记录时间：2026-08-26。环境：`PHPYUN_ENV_FILE` → `phpyun-rs/.env.dev`（MySQL `phpyun_test` @ 127.0.0.1:3306，Redis db 15）。

`phpyun_test` 当时：**177 张 `phpyun_` 表结构 + 合计 14 行**（会话/审计/1 份简历种子），几乎空库。列表类 200 + empty list 是数据空，不是路由挂了。

## 怎么跑

```bash
cd phpyun-rs
cargo test -p phpyun-handlers --test endpoint_smoke -- --ignored --nocapture
```

该冒烟扫的是 **v1 OpenAPI 的 POST**（`/v1/wap` + `/v1/mcenter`，405 条）。`/v1/admin` 在 `api-admin`，由 `apps/server` 装配，不在这份 library 冒烟里。Admin 路由表见 `doc/snapshots/admin_paths.txt`（102 条）。

OpenAPI / 路由快照（供 T3 逐条比对）：

- `doc/snapshots/v1.openapi.json` / `v1_paths.txt`（405 paths）
- `doc/snapshots/admin.openapi.json` / `admin_paths.txt`（72 paths）

```bash
cargo test -p phpyun-handlers --test openapi_snapshot
cargo test -p phpyun-rs --test openapi_contract
```

## 分类

| HTTP | 条数 | 分类 |
|---|---|---|
| 200 | 119 | 空 `{}` 也能过的读接口；空库下多数 `data` 是空列表 |
| 400 | 245 | 校验失败（空 body），handler 可达 |
| 401 | 14 | 鉴权（无/错 token 或角色） |
| 403 | 24 | 角色/权限 |
| 422 | 3 | 业务规则 |
| 500 | **0** | — |

4xx 视为正常。5xx 才算冒烟失败。

先前唯一 5xx 是 `POST /v1/mcenter/remarks/list`（SQL 列名按 API 字段写，对不上 `phpyun_resume_remark` 真表）。T2 已把 repo 对齐为 `uid/eid/comid/ctime/remark`，JSON 形状仍是 `target_uid/note/updated_at`。该路径现为 **400**（空 body 校验），不再 500。

`data=null`（非 5xx）：`/v1/wap/site/sub-sites/match`（空库下业务空结果）。

```
========== ENDPOINT SMOKE REPORT ==========
Total POST endpoints probed: 405
  HTTP 200 : 119 endpoint(s)
  HTTP 400 : 245 endpoint(s)
  HTTP 401 : 14 endpoint(s)
  HTTP 403 : 24 endpoint(s)
  HTTP 422 : 3 endpoint(s)
  HTTP 500 : 0 endpoint(s)

200-response data audit (43/119 carry non-empty data):
  ⚠ data=null (1):
       /v1/wap/site/sub-sites/match
  ⚠ data is empty list (75): 见仓库内上次完整列表；空库预期，此处不重复粘贴。

No 5xx responses across 405 endpoints
===========================================
```
"sn": 20260826125216847762",