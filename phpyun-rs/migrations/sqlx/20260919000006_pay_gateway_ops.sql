-- Payment console ops: notify log, catalog placeholders, overview/notifies nav.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`).

CREATE TABLE IF NOT EXISTS `phpyun_rs_pay_notify` (
    `id`           BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `pay_no`       VARCHAR(64)     NOT NULL DEFAULT '',
    `merchant_id`  BIGINT UNSIGNED NOT NULL DEFAULT 0,
    `event`        VARCHAR(16)     NOT NULL DEFAULT '' COMMENT 'paid / refunded',
    `url`          VARCHAR(512)    NOT NULL DEFAULT '',
    `body`         TEXT            NULL,
    `http_status`  INT             NOT NULL DEFAULT 0,
    `ok`           TINYINT         NOT NULL DEFAULT 0,
    `error`        VARCHAR(255)    NOT NULL DEFAULT '',
    `ctime`        INT             NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    KEY `idx_pay_no` (`pay_no`),
    KEY `idx_ok_ctime` (`ok`, `ctime`),
    KEY `idx_merchant_ctime` (`merchant_id`, `ctime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Pay merchant notify attempts';

INSERT INTO `phpyun_rs_pay_method`
    (`merchant_id`, `code`, `name`, `status`, `config_json`, `sort`, `ctime`, `updated_at`)
SELECT m.id, 'paypal', 'PayPal', 'paused', '{}', 4, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM `phpyun_rs_pay_merchant` m
WHERE m.code = 'ov6'
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_method` x WHERE x.merchant_id = m.id AND x.code = 'paypal'
  );

INSERT INTO `phpyun_rs_pay_method`
    (`merchant_id`, `code`, `name`, `status`, `config_json`, `sort`, `ctime`, `updated_at`)
SELECT m.id, 'grabpay', 'GrabPay', 'paused', '{}', 5, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()
FROM `phpyun_rs_pay_merchant` m
WHERE m.code = 'ov6'
  AND NOT EXISTS (
      SELECT 1 FROM `phpyun_rs_pay_method` x WHERE x.merchant_id = m.id AND x.code = 'grabpay'
  );

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1075, '概览', 1074, '', 1, '0', 0, 0, 1, 3, '/payment/overview'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1075 OR `path` = '/payment/overview'
);

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1076, '回调', 1074, '', 1, '0', 5, 0, 1, 3, '/payment/notifies'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1076 OR `path` = '/payment/notifies'
);

UPDATE `phpyun_admin_navigation` SET `sort` = 1 WHERE `id` = 1071 AND `keyid` = 1074;
UPDATE `phpyun_admin_navigation` SET `sort` = 2 WHERE `id` = 1072 AND `keyid` = 1074;
UPDATE `phpyun_admin_navigation` SET `sort` = 3 WHERE `id` = 1073 AND `keyid` = 1074;

UPDATE `phpyun_admin_user_group`
SET `group_power` = CONCAT(
    'a:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 2,
    ':{',
    TRIM(TRAILING '}' FROM SUBSTRING(`group_power`, LOCATE('{', `group_power`) + 1)),
    'i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED),
    ';i:1075;i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 1,
    ';i:1076;}'
)
WHERE `id` = 1
  AND `group_power` LIKE 'a:%'
  AND `group_power` LIKE '%i:1074;%'
  AND `group_power` NOT LIKE '%i:1075;%';
