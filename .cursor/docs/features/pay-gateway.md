# 三方支付网关

OV6 自用，也给外部商户 HMAC 调用。进程仍是 Rust `:3003`，**不**单独拆支付服务。System 的支付设置页（`payset.vue` / `set_payset`）**不动**。

## 表（`jobs`，utf8mb4）

手工执行 [`phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql`](../../phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql)（`RUN_MIGRATIONS_ON_BOOT=false`）。

| 表 | 作用 |
|---|---|
| `phpyun_rs_pay_merchant` | 商户。种子 `ov6`。`api_key` / `api_secret` 给 HMAC |
| `phpyun_rs_pay_method` | 每商户渠道：`stripe` / `gcash` / `paymaya`；`status` = `active` \| `paused`；`config_json` 存密钥 |
| `phpyun_rs_pay_order` | 网关账本：`pay_no`、商户单号、金额、渠道、状态 |

Stripe Session 缓存仍用 `phpyun_rs_stripe_order`（OV6 会员单）。站点 `sy_stripe_sk` 作 OV6 Stripe 回退，不删。

## 渠道

| code | 本轮 |
|---|---|
| `stripe` | Hosted Checkout 真收；密钥优先 method `config_json.secret_key` |
| `gcash` / `paymaya` | 可添加/暂停/删除；`create` 返回 `not_configured` |

后台：一级 **支付**（系统后面，id=1070），子页订单 `/payment/orders`、支付方式 `/payment/methods`、商户 `/payment/merchants`。

## 调用

```
OV6 收银台 POST /v1/mcenter/orders/pay
  → 写 phpyun_rs_pay_order → Stripe Checkout
  → webhook /callback/stripe 或 /callback/pay/stripe
  → pay_order=paid；ov6 再 settle_paid_checked(company_order)

外部 HMAC POST /v1/pay/orders
  → 网关订单 + pay_url；不碰会员账
  → 已付后 POST merchant.notify_url（尽力，不重试队列）
```

HMAC：`Authorization: HMAC-SHA256 key_id=... ts=... sign=hex(hmac(secret, ts + "\n" + POST + "\n" + path + "\n" + sha256hex(body)))`。path 固定为 `/v1/pay/orders`、`/v1/pay/methods/list`、`/v1/pay/orders/detail`。时钟差 300s。`api_secret` 为空的商户（种子 ov6）不能走 HMAC。

查询订单用 **POST** `/v1/pay/orders/detail`（全站业务 API POST-only，没有 GET 豁免）。

## 禁做

- 不要改 `web/apps/admin/app/pages/payset.vue` / `sy_alipay` 当网关开关
- 不要把 Stripe/HMAC 密钥提交 git
- 不要当 GCash/PayMaya 已真收
- 不要把网关密钥写进 `uploads/` PHP
