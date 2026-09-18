-- Seeker monthly VIP catalog + gift columns.
-- RUN_MIGRATIONS_ON_BOOT=false: apply on `jobs` by hand.

CREATE TABLE IF NOT EXISTS `phpyun_rs_seeker_vip_pack` (
    `id`            INT UNSIGNED    NOT NULL AUTO_INCREMENT,
    `code`          VARCHAR(64)     NOT NULL,
    `name`          VARCHAR(120)    NOT NULL,
    `months`        SMALLINT        NOT NULL DEFAULT 1,
    `price_cents`   INT             NOT NULL DEFAULT 0,
    `chat`          TINYINT         NOT NULL DEFAULT 1,
    `resume_top`    TINYINT         NOT NULL DEFAULT 1,
    `tpl_all`       TINYINT         NOT NULL DEFAULT 1,
    `refresh_free`  TINYINT         NOT NULL DEFAULT 1,
    `sort`          INT             NOT NULL DEFAULT 0,
    `display`       TINYINT         NOT NULL DEFAULT 1,
    `deleted`       TINYINT         NOT NULL DEFAULT 0,
    `created_at`    INT UNSIGNED    NOT NULL DEFAULT 0,
    `updated_at`    INT UNSIGNED    NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `ix_display_sort` (`display`, `sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `phpyun_rs_seeker_vip_pack`
    (`code`, `name`, `months`, `price_cents`, `chat`, `resume_top`, `tpl_all`, `refresh_free`, `sort`, `display`, `deleted`, `created_at`, `updated_at`)
VALUES
    ('month_1', '月卡', 1, 2900, 1, 1, 1, 1, 10, 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
    ('month_3', '季卡', 3, 7900, 1, 1, 1, 1, 20, 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
    ('month_12', '年卡', 12, 19900, 1, 1, 1, 1, 30, 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP())
ON DUPLICATE KEY UPDATE `name` = VALUES(`name`);

SET @exist := (
    SELECT COUNT(*) FROM information_schema.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'phpyun_change' AND COLUMN_NAME = 'to_uid'
);
SET @sql := IF(@exist = 0,
    'ALTER TABLE `phpyun_change` ADD COLUMN `to_uid` INT UNSIGNED NOT NULL DEFAULT 0 AFTER `uid`',
    'SELECT 1');
PREPARE stmt FROM @sql; EXECUTE stmt; DEALLOCATE PREPARE stmt;

SET @exist := (
    SELECT COUNT(*) FROM information_schema.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'phpyun_reward' AND COLUMN_NAME = 'kind'
);
SET @sql := IF(@exist = 0,
    'ALTER TABLE `phpyun_reward` ADD COLUMN `kind` VARCHAR(32) NOT NULL DEFAULT ''goods'' AFTER `name`',
    'SELECT 1');
PREPARE stmt FROM @sql; EXECUTE stmt; DEALLOCATE PREPARE stmt;

INSERT INTO `phpyun_reward`
    (`name`, `kind`, `integral`, `stock`, `restriction`, `nid`, `tnid`, `status`, `sdate`, `content`, `pic`, `rec`, `hot`, `num`, `sort`)
SELECT '简历刷新', 'resume_refresh', 20, 999999, 0, 0, 0, 1, UNIX_TIMESTAMP(), '刷新简历更新时间，可送给求职者。', '', 1, 0, 0, 0
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_reward` WHERE `kind` = 'resume_refresh' LIMIT 1
);
