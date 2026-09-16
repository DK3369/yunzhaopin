# 已合并 / 即将失效接口

路径以 [`doc/snapshots/v1_paths.txt`](../../../doc/snapshots/v1_paths.txt) 为准。App 看 OpenAPI：现网 `:3003/api-docs/v1/openapi.json`（快照 `doc/snapshots/v1.openapi.json`）。即将失效操作带 `"deprecated": true`，`description` 写替代路径。

v1 **只加法**。本文不承诺摘除日期；摘路由须另批任务并改快照。

## 即将失效（仍注册，新集成勿用）

本站前端已迁完。勿再扩展旧模块。

| 旧路径（仍挂） | 改用 |
|---|---|
| `POST /v1/mcenter/company/products`、`/list`、`/update` | `POST /v1/mcenter/company-contents/{list,create,detail,update,delete}`，body `kind=product` |
| `POST /v1/mcenter/company/news`、`/list`、`/update` | 同上，`kind=news` |
| `GET/POST /v1/wap/dict/cities`、`/dict/cities/by-province` | `/v1/wap/regions`、`/v1/wap/regions/children` |

公开读 `GET/POST /v1/wap/company/...` 产品/新闻列表不是这套，不要当废弃。

## 已合并完成（独立路径已不在快照，勿再恢复）

独立 `delete` / `remove` 并进对应 `/update`，body `"status": 2` 软删：

- 简历子资源：`/v1/mcenter/resume/{expects,edus,works,projects,skills,languages,trainings,certs,others}/update`
- `POST /v1/mcenter/interview-templates/update`
- 旧 `company/products|news` 的独立 delete（openapi 注释；现走 `/update` + `status:2`，新代码走 `company-contents/delete`）
- 职位单条 `delete_job` → `POST /v1/mcenter/jobs/batch/delete`
- 后台同类：`/v1/admin/ads`、`/nav`、`/desc-classes` 的 `update` + `status:2`

## 并存勿混（不标即将失效）

| 路径 | 口径 |
|---|---|
| `/v1/mcenter/follows*` 与 `/favorites*` | 故意双栈。favorites `{kind,target_id}`：1 职位收藏 / 2 关注企业 / 3 关注用户。follows 仍用 `{target_kind,target_uid}`。新代码优先 favorites；site 企业关注仍打 follows。 |
| `/v1/mcenter/fans` vs `followers` | 语义不同：招聘「对我感兴趣」用 `fans`，**不要** `followers`。见 [member-center.md](../features/member-center.md)。 |

## 不算合并

公开读 GET+POST 双挂（旧 WAP 不要删）、`/v2` 复用 v1 handler、admin `php-content`、账号 `account-merge`。
