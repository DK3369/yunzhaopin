-- Payment console: copy OV6 Stripe keys, HMAC IP allowlist, backfill ledger.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

ALTER TABLE `phpyun_rs_pay_merchant`
    ADD COLUMN `allow_ips` TEXT NULL COMMENT 'HMAC allowlist, one IP/CIDR per line; empty = deny' AFTER `return_url`;

UPDATE `phpyun_rs_pay_method` x
INNER JOIN `phpyun_rs_pay_merchant` m ON m.id = x.merchant_id AND m.code = 'ov6'
SET x.config_json = JSON_OBJECT(
        'secret_key', COALESCE((SELECT `config` FROM `phpyun_admin_config` WHERE `name` = 'sy_stripe_sk' LIMIT 1), ''),
        'webhook_secret', COALESCE((SELECT `config` FROM `phpyun_admin_config` WHERE `name` = 'sy_stripe_whsec' LIMIT 1), ''),
        'currency', COALESCE(NULLIF((SELECT `config` FROM `phpyun_admin_config` WHERE `name` = 'sy_stripe_currency' LIMIT 1), ''), 'usd')
    ),
    x.updated_at = UNIX_TIMESTAMP()
WHERE x.code = 'stripe'
  AND COALESCE(JSON_UNQUOTE(JSON_EXTRACT(IFNULL(NULLIF(x.config_json, ''), '{}'), '$.secret_key')), '') = '';

INSERT INTO `phpyun_rs_pay_order` (
    `pay_no`, `merchant_id`, `merchant_order_no`, `method_code`, `amount_cents`, `currency`,
    `status`, `channel_ref`, `pay_url`, `subject`, `paid_at`, `ctime`, `updated_at`, `extra_json`
)
SELECT
    CONCAT('s', s.id),
    m.id,
    s.order_no,
    'stripe',
    s.amount_cents,
    LOWER(COALESCE(NULLIF(s.req_currency, ''), NULLIF(s.stripe_currency, ''), 'usd')),
    CASE s.order_state
        WHEN 1 THEN 'paid'
        WHEN 0 THEN 'pending'
        WHEN 2 THEN 'cancelled'
        ELSE 'failed'
    END,
    NULLIF(s.stripe_session_id, ''),
    LEFT(COALESCE(s.stripe_url, ''), 1024),
    LEFT(COALESCE(s.subject, ''), 255),
    CASE WHEN s.order_state = 1 THEN CAST(s.paid_at AS SIGNED) ELSE 0 END,
    CAST(s.created_at AS SIGNED),
    CAST(s.updated_at AS SIGNED),
    '{}'
FROM `phpyun_rs_stripe_order` s
INNER JOIN `phpyun_rs_pay_merchant` m ON m.code = 'ov6'
WHERE s.order_no <> ''
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_order` p
      WHERE p.merchant_id = m.id
        AND p.merchant_order_no = s.order_no
        AND p.method_code = 'stripe'
  )
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_order` p3
      WHERE p3.pay_no = CONCAT('s', s.id)
  )
  AND (
      s.stripe_session_id IS NULL
      OR s.stripe_session_id = ''
      OR NOT EXISTS (
          SELECT 1 FROM `phpyun_rs_pay_order` p2
          WHERE p2.channel_ref = s.stripe_session_id
      )
  );
