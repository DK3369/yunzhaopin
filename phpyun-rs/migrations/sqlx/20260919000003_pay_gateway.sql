-- Third-party payment gateway (merchants / methods / orders) + admin nav after 系统.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

CREATE TABLE IF NOT EXISTS `phpyun_rs_pay_merchant` (
    `id`          BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `code`        VARCHAR(64)     NOT NULL DEFAULT '' COMMENT 'ident_ok, unique',
    `name`        VARCHAR(128)    NOT NULL DEFAULT '',
    `api_key`     VARCHAR(64)     NOT NULL DEFAULT '' COMMENT 'HMAC key_id',
    `api_secret`  VARCHAR(128)    NOT NULL DEFAULT '' COMMENT 'HMAC secret, never in git',
    `notify_url`  VARCHAR(512)    NOT NULL DEFAULT '',
    `return_url`  VARCHAR(512)    NOT NULL DEFAULT '',
    `status`      VARCHAR(16)     NOT NULL DEFAULT 'active' COMMENT 'active / paused',
    `ctime`       INT             NOT NULL DEFAULT 0,
    `updated_at`  INT             NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    UNIQUE KEY `uk_api_key` (`api_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Pay gateway merchants';

CREATE TABLE IF NOT EXISTS `phpyun_rs_pay_method` (
    `id`           BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `merchant_id`  BIGINT UNSIGNED NOT NULL DEFAULT 0,
    `code`         VARCHAR(64)     NOT NULL DEFAULT '' COMMENT 'stripe / gcash / paymaya',
    `name`         VARCHAR(128)    NOT NULL DEFAULT '',
    `status`       VARCHAR(16)     NOT NULL DEFAULT 'paused' COMMENT 'active / paused',
    `config_json`  TEXT            NULL COMMENT 'secrets; read API masks',
    `sort`         INT             NOT NULL DEFAULT 0,
    `ctime`        INT             NOT NULL DEFAULT 0,
    `updated_at`   INT             NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_merchant_code` (`merchant_id`, `code`),
    KEY `idx_merchant_status` (`merchant_id`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Pay gateway methods';

CREATE TABLE IF NOT EXISTS `phpyun_rs_pay_order` (
    `id`                 BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `pay_no`             VARCHAR(64)     NOT NULL DEFAULT '' COMMENT 'gateway order no',
    `merchant_id`        BIGINT UNSIGNED NOT NULL DEFAULT 0,
    `merchant_order_no`  VARCHAR(64)     NOT NULL DEFAULT '',
    `method_code`        VARCHAR(64)     NOT NULL DEFAULT '',
    `amount_cents`       INT             NOT NULL DEFAULT 0,
    `currency`           VARCHAR(16)     NOT NULL DEFAULT 'usd',
    `status`             VARCHAR(16)     NOT NULL DEFAULT 'pending' COMMENT 'pending/paid/failed/cancelled',
    `channel_ref`        VARCHAR(255)    NULL COMMENT 'e.g. Stripe session id',
    `pay_url`            VARCHAR(1024)   NOT NULL DEFAULT '',
    `subject`            VARCHAR(255)    NOT NULL DEFAULT '',
    `paid_at`            INT             NOT NULL DEFAULT 0,
    `ctime`              INT             NOT NULL DEFAULT 0,
    `updated_at`         INT             NOT NULL DEFAULT 0,
    `extra_json`         TEXT            NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_pay_no` (`pay_no`),
    UNIQUE KEY `uk_merchant_order_method` (`merchant_id`, `merchant_order_no`, `method_code`),
    UNIQUE KEY `uk_channel_ref` (`channel_ref`),
    KEY `idx_merchant_ctime` (`merchant_id`, `ctime`),
    KEY `idx_status_ctime` (`status`, `ctime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Pay gateway ledger';

INSERT INTO `phpyun_rs_pay_merchant`
    (`code`, `name`, `api_key`, `api_secret`, `notify_url`, `return_url`, `status`, `ctime`, `updated_at`)
SELECT 'ov6', 'OV6', 'ov6', '', '', '', 'active', UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM DUAL
WHERE NOT EXISTS (SELECT 1 FROM `phpyun_rs_pay_merchant` WHERE `code` = 'ov6');

INSERT INTO `phpyun_rs_pay_method`
    (`merchant_id`, `code`, `name`, `status`, `config_json`, `sort`, `ctime`, `updated_at`)
SELECT m.id, 'stripe', 'Stripe', 'active', '{}', 1, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM `phpyun_rs_pay_merchant` m
WHERE m.code = 'ov6'
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_method` x WHERE x.merchant_id = m.id AND x.code = 'stripe'
  );

INSERT INTO `phpyun_rs_pay_method`
    (`merchant_id`, `code`, `name`, `status`, `config_json`, `sort`, `ctime`, `updated_at`)
SELECT m.id, 'gcash', 'GCash', 'paused', '{}', 2, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM `phpyun_rs_pay_merchant` m
WHERE m.code = 'ov6'
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_method` x WHERE x.merchant_id = m.id AND x.code = 'gcash'
  );

INSERT INTO `phpyun_rs_pay_method`
    (`merchant_id`, `code`, `name`, `status`, `config_json`, `sort`, `ctime`, `updated_at`)
SELECT m.id, 'paymaya', 'PayMaya', 'paused', '{}', 3, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM `phpyun_rs_pay_merchant` m
WHERE m.code = 'ov6'
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_method` x WHERE x.merchant_id = m.id AND x.code = 'paymaya'
  );

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1070, '支付', 0, '', 1, 'nav_pay', 6, 0, 1, 1, ''
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1070 OR (`keyid` = 0 AND `name` = '支付')
);

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1071, '订单', 1070, '', 1, '0', 1, 0, 1, 2, '/payment/orders'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1071 OR `path` = '/payment/orders'
);

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1072, '支付方式', 1070, '', 1, '0', 2, 0, 1, 2, '/payment/methods'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1072 OR `path` = '/payment/methods'
);

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1073, '商户', 1070, '', 1, '0', 3, 0, 1, 2, '/payment/merchants'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1073 OR `path` = '/payment/merchants'
);

UPDATE `phpyun_admin_user_group`
SET `group_power` = CONCAT(
    'a:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 4,
    ':{',
    TRIM(TRAILING '}' FROM SUBSTRING(`group_power`, LOCATE('{', `group_power`) + 1)),
    'i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED),
    ';i:1070;i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 1,
    ';i:1071;i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 2,
    ';i:1072;i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 3,
    ';i:1073;}'
)
WHERE `id` = 1
  AND `group_power` LIKE 'a:%'
  AND `group_power` NOT LIKE '%i:1070;%'
  AND `group_power` NOT LIKE '%s:4:"1070"%';
