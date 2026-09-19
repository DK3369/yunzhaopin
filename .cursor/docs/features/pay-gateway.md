# 三方支付网关

OV6 自用，也给外部商户 HMAC 调用。进程仍是 Rust `:3003`，**不**单独拆支付服务。System 的支付设置页（`payset.vue` / `set_payset`）**不动**；支付宝/微信/银行/财务充值订单仍不进 Payment。

## 表（`jobs`，utf8mb4）

手工执行：

1. [`phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql`](../../phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql)
2. [`phpyun-rs/migrations/sqlx/20260919000004_pay_gateway_console.sql`](../../phpyun-rs/migrations/sqlx/20260919000004_pay_gateway_console.sql)（`allow_ips`、OV6 Stripe `config_json` 从 `sy_stripe_*` 拷一次、`stripe_order` 灌账本）

`RUN_MIGRATIONS_ON_BOOT=false`。

| 表 | 作用 |
|---|---|
| `phpyun_rs_pay_merchant` | 商户。种子 `ov6`。`api_key` / `api_secret` 给 HMAC；`allow_ips` 一行一个 IPv4/IPv6（可 IPv4 CIDR）。空名单 = 拒绝 |
| `phpyun_rs_pay_method` | 每商户渠道：`stripe` / `gcash` / `paymaya`；`status` = `active` \| `paused`；`config_json` 存密钥 |
| `phpyun_rs_pay_order` | 网关账本：`pay_no`、商户单号、金额、渠道、状态。历史 Stripe Session 已从 `phpyun_rs_stripe_order` 灌入 |

Stripe Session 缓存仍用 `phpyun_rs_stripe_order`（OV6 会员单）。**真收只读** method `config_json.secret_key` / `webhook_secret` / `currency`，**不再**读 `sy_stripe_sk` / `sy_stripe_whsec` / 站点开关 `stripe=1`。payset 旧字段可继续显示，不认。

## 渠道

| code | 本轮 |
|---|---|
| `stripe` | Hosted Checkout 真收；密钥只在 Payment Methods 编辑；`charge_ready` = `secret_key` 非空 |
| `gcash` / `paymaya` | 可添加/暂停/删除；`create` 返回 `not_configured` |

后台：一级 **支付**（系统后面，id=1070），子页订单 `/payment/orders`、支付方式 `/payment/methods`、商户 `/payment/merchants`（库里是两级：1070 → 1071/1072/1073，没有中间分组）。侧栏点「订单」本身进页；点顶栏「支付」会打开第一页订单。空表也要有搜索条、表头、分页。页面根节点用 `.pay-page`，不要 `.moduleElenAl`（绝对定位 height:100% 会把表裁成白板）。

## 调用

```
OV6 收银台 POST /v1/mcenter/orders/pay   JWT 会员
  → 写 phpyun_rs_pay_order → Stripe Checkout
  → webhook /callback/stripe 或 /callback/pay/stripe（验 Stripe 签，不走商户 IP）
  → pay_order=paid；ov6 再 settle_paid_checked(company_order)

管理员 POST /v1/admin/pay/*   JWT usertype=9

外部 HMAC POST /v1/pay/*
  → 先 HMAC，再命中该商户 allow_ips；名单空或 IP 不在 → 403 ip_not_allowed
  → 网关订单 + pay_url；不碰会员账
  → 已付后 POST merchant.notify_url（尽力，不重试队列）
```

HMAC：`Authorization: HMAC-SHA256 key_id=... ts=... sign=hex(hmac(secret, ts + "\n" + POST + "\n" + path + "\n" + sha256hex(body)))`。path 固定为 `/v1/pay/orders`、`/v1/pay/methods/list`、`/v1/pay/orders/detail`。时钟差 300s。`api_secret` 为空的商户（种子 ov6）不能走 HMAC。

查询订单用 **POST** `/v1/pay/orders/detail`（全站业务 API POST-only，没有 GET 豁免）。

## 禁做

- 不要改 `web/apps/admin/app/pages/payset.vue` / `sy_alipay` 当网关开关
- 不要再把 `sy_stripe_sk` 当网关密钥
- 不要把 Stripe/HMAC 密钥提交 git
- 不要当 GCash/PayMaya 已真收
- 不要把网关密钥写进 `uploads/` PHP
- 不要把 `/v1/admin/pay`、`/v1/mcenter/orders/pay`、`/callback/pay/{method}` 套进商户 IP 名单
