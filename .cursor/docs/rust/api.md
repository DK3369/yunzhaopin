# 契约与请求形状

## 信封

```json
{ "code": 200, "key": "ok", "msg": "ok", "data": { } }
```

BFF（`web/layers/base/server/routes/api/proxy`）把 `code !== 200` 写成 **HTTP 状态 = code**，body 仍是信封。前台 `httpPost` 在 4xx 时仍可能 `resolve` 成 `{ error: 1, data: ... }`，Vue 必须容忍 `data.list` 为空。

`msg` 给客户端的是**已翻译文本**，不是 i18n key。见 [i18n.md](./i18n.md)。

## 成功应答（只约束新接口）

有业务 payload → `ApiResponse::data(...)`。无 payload → `ApiResponse::message("ok")`（信封里 `key`/`msg` 为已译的 ok）。**不要**再给新接口加 `{ok:true}` 或第二套成功形状。旧 handler 不扫。

## 分页

Query：`page`、`page_size`（1..=200）。侧栏常用 `limit`。PHP 后台列表还要 `list` / `total` / `page` / 有时 `limit`、`pageSizes`、`perPage` 别名（`Paged` 或 `AdminPaged`）。**不要**为统一去改已有参数名。

PHP Vue 筛选项经常是 **字符串** `"0"` / `""`。`Option<i32>` 必须用 `de_loose_i32_opt`（或同类宽松反序列化），否则 `ValidatedJson` → HTTP 400。`StatusFilterBody` 已这样处理。新增 admin 列表 body 照抄，不要用裸 `i32` 去接 PHP。

软删列表：`COALESCE(deleted,0)=0`。`format!("{PREDICATE}")` 才能插进 SQL；写进 **普通字符串字面量** 会把 `{PREDICATE}` 原样发给 MySQL。

## 后台映射

PHP 页继续 `httpPost('m=&c=&a=')` → `web/apps/admin/app/utils/phpMap.ts` → 显式 `/v1/admin/...`。未映射返回「未映射的后台接口」，不要在 Rust 做万能 invoke。

## 改路由时动快照

| 前缀 | 快照 |
|---|---|
| App | `doc/snapshots/v1_paths.txt`、`v1.openapi.json` |
| Admin 具名 | `doc/snapshots/admin_paths.txt`、`admin.openapi.json` |
| php-content | **不进** admin 快照 |

现网（仅 `APP_ENV=dev|test`）：Swagger UI `:3003/docs/`（公网 `/yapi/docs/`），JSON `:3003/api-docs/v1|v2|admin/openapi.json`。`APP_ENV=prod` 不挂这两类路由。

[API_V1_SUMMARY.md](../../../phpyun-rs/docs/API_V1_SUMMARY.md) 里的「305 条」是旧口径；以快照和 `utoipa` 为准。

已合并 / 即将失效（App 以 OpenAPI `deprecated` 为准）见 [api-merged.md](./api-merged.md)。
