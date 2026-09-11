-- English job scrape (career-ops). Additive Rust-only tables.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

CREATE TABLE IF NOT EXISTS `phpyun_rs_job_scrape_item` (
    `id`           BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `source_url`   VARCHAR(512)    NOT NULL,
    `job_id`       INT UNSIGNED     NOT NULL DEFAULT 0,
    `company_uid`  INT UNSIGNED     NOT NULL DEFAULT 0,
    `role`         VARCHAR(255)    NOT NULL DEFAULT '',
    `company_name` VARCHAR(255)    NOT NULL DEFAULT '',
    `created_at`   INT UNSIGNED     NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_source_url` (`source_url`),
    KEY `ix_job_id` (`job_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `phpyun_rs_job_scrape_log` (
    `id`          BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `started_at`  INT UNSIGNED     NOT NULL DEFAULT 0,
    `finished_at` INT UNSIGNED     NOT NULL DEFAULT 0,
    `fetched`     INT              NOT NULL DEFAULT 0,
    `inserted`    INT              NOT NULL DEFAULT 0,
    `skipped`     INT              NOT NULL DEFAULT 0,
    `error`       VARCHAR(500)     NOT NULL DEFAULT '',
    PRIMARY KEY (`id`),
    KEY `ix_started` (`started_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
