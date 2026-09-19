# 三方支付网关

OV6 自用，也给外部商户 HMAC 调用。进程仍是 Rust `:3003`，**不**单独拆支付服务。System 的支付设置页（`payset.vue` / `set_payset`）**不动**；支付宝/微信/银行/财务充值订单仍不进 Payment。

## 表（`jobs`，utf8mb4）

手工执行：

1. [`phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql`](../../phpyun-rs/migrations/sqlx/20260919000003_pay_gateway.sql)
2. [`phpyun-rs/migrations/sqlx/20260919000004_pay_gateway_console.sql`](../../phpyun-rs/migrations/sqlx/20260919000004_pay_gateway_console.sql)（`allow_ips`、OV6 Stripe `config_json` 从 `sy_stripe_*` 拷一次、`stripe_order` 灌账本）
3. [`phpyun-rs/migrations/sqlx/20260919000005_pay_nav_section.sql`](../../phpyun-rs/migrations/sqlx/20260919000005_pay_nav_section.sql)（菜单改三级：1070 支付 → 1074 网关 → 1071/1072/1073）
4. [`phpyun-rs/migrations/sqlx/20260919000006_pay_gateway_ops.sql`](../../phpyun-rs/migrations/sqlx/20260919000006_pay_gateway_ops.sql)（`pay_notify`、OV6 paypal/grabpay 占位、1075 概览 / 1076 回调）

`RUN_MIGRATIONS_ON_BOOT=false`。

| 表 | 作用 |
|---|---|
| `phpyun_rs_pay_merchant` | 商户。种子 `ov6`。`api_key` / `api_secret` 给 HMAC；`allow_ips` 一行一个 IPv4/IPv6（可 IPv4 CIDR）。空名单 = 拒绝 |
| `phpyun_rs_pay_method` | 每商户渠道；`status` = `active` \| `paused`；`config_json` 存密钥 |
| `phpyun_rs_pay_order` | 网关账本：`pay_no`、商户单号、金额、渠道、状态 `pending/paid/failed/cancelled/refunded` |
| `phpyun_rs_pay_notify` | 出站商户回调尝试：`event`=`paid`/`refunded`，先插日志再 POST，后台可重试。密钥不进表 |

Stripe Session 缓存仍用 `phpyun_rs_stripe_order`（OV6 会员单）。**真收只读** method `config_json.secret_key` / `webhook_secret` / `currency`，**不再**读 `sy_stripe_sk` / `sy_stripe_whsec` / 站点开关 `stripe=1`。payset 旧字段可继续显示，不认。

## 渠道

静态目录（`pay_adapter::CHANNELS`，`ident_ok` + `match`）：

| code | 本轮 |
|---|---|
| `stripe` | Hosted Checkout 真收；密钥只在 Payment Methods 编辑；`charge_ready` = `secret_key` 非空。关单 expire Session；退款 `POST /v1/refunds`（PI 来自 `stripe_order` 或拉 Session） |
| `gcash` / `paymaya` / `paypal` / `grabpay` | 目录占位；`create` 返回 `not_configured`。**不当已真收** |

网关退款只改 `pay_order=refunded` 并回调外部商户；**不自动冲** OV6 会员套餐/积分（求职/招聘财务退款仍走原路径）。

后台菜单必须三级（和运营→财务→充值订单一样），否则侧栏会把页面画成空文件夹：

- 1070 支付（顶栏）
- 1074 网关（侧栏分组）
- 1075 `/payment/overview` 概览（sort 最前，点顶栏 Payment 进这里）
- 1071 `/payment/orders`、1072 `/payment/methods`、1073 `/payment/merchants`、1076 `/payment/notifies`

不要留在 System 的 `/payset`。空表也要有搜索条、表头、分页。筛选项第一项是 **All**。页面根节点用 `.pay-page`，不要 `.moduleElenAl`。

## 调用

```
OV6 收银台 POST /v1/mcenter/orders/pay   JWT 会员
  → 写 phpyun_rs_pay_order → Stripe Checkout
  → webhook /callback/stripe 或 /callback/pay/stripe（验 Stripe 签，不走商户 IP）
  → pay_order=paid；ov6 再 settle_paid_checked(company_order)

管理员 POST /v1/admin/pay/*   JWT usertype=9
  overview / channels/list / orders/{list,close,refund} / notifies/{list,retry}

外部 HMAC POST /v1/pay/*
  → 先 HMAC，再命中该商户 allow_ips；名单空或 IP 不在 → 403 ip_not_allowed
  → 网关订单 + pay_url；不碰会员账
  → 已付/退款后 POST merchant.notify_url（先落 phpyun_rs_pay_notify 再 POST；后台可重试）
```

HMAC：`Authorization: HMAC-SHA256 key_id=... ts=... sign=hex(hmac(secret, ts + "\n" + POST + "\n" + path + "\n" + sha256hex(body)))`。path 参与签名：`/v1/pay/methods/list`、`/v1/pay/orders`、`/v1/pay/orders/detail`、`/v1/pay/orders/close`、`/v1/pay/orders/refund`。时钟差 300s。`api_secret` 为空的商户（种子 ov6）不能走 HMAC。

查询订单用 **POST** `/v1/pay/orders/detail`（全站业务 API POST-only）。

## 禁做

- 不要改 `web/apps/admin/app/pages/payset.vue` / `sy_alipay` 当网关开关
- 不要再把 `sy_stripe_sk` 当网关密钥
- 不要把 Stripe/HMAC 密钥提交 git
- 不要当 PayPal/GrabPay/GCash/PayMaya 已真收
- 不要把网关密钥写进 `uploads/` PHP
- 不要把 `/v1/admin/pay`、`/v1/mcenter/orders/pay`、`/callback/pay/{method}`、出站 `notify_url` 套进商户 IP 名单
- 不要在网关退款时自动冲 OV6 会员账本
