-- App 缺口：面试多维评价 + 资讯栏目订阅（Rust 专用表，不碰 PHP 表）。
-- RUN_MIGRATIONS_ON_BOOT=false，现网手工执行本文件。

CREATE TABLE IF NOT EXISTS `phpyun_rs_interview_review` (
    `id`         BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `yqms_id`    INT UNSIGNED    NOT NULL,
    `rater_uid`  INT UNSIGNED    NOT NULL,
    `ratee_uid`  INT UNSIGNED    NOT NULL,
    `dimensions` TEXT            NOT NULL,
    `total`      INT UNSIGNED    NOT NULL DEFAULT 0,
    `comment`    VARCHAR(1000)   NOT NULL DEFAULT '',
    `created_at` INT UNSIGNED    NOT NULL,
    `updated_at` INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_rater_yqms` (`rater_uid`, `yqms_id`),
    KEY `ix_yqms` (`yqms_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `phpyun_rs_article_channel_sub` (
    `uid`        INT UNSIGNED NOT NULL,
    `group_ids`  TEXT         NOT NULL,
    `updated_at` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
