-- Durable audit log for Rust API write operations.
-- Additive only: no legacy PHPYun table is modified.
CREATE TABLE IF NOT EXISTS `phpyun_rs_audit_log` (
    `id`         BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `actor_uid`  INT UNSIGNED NULL,
    `actor_ip`   VARCHAR(64) NOT NULL DEFAULT '',
    `actor_ua`   VARCHAR(512) NOT NULL DEFAULT '',
    `action`     VARCHAR(128) NOT NULL,
    `target`     VARCHAR(255) NOT NULL DEFAULT '',
    `success`    TINYINT NOT NULL DEFAULT 1,
    `meta`       JSON NULL,
    `created_at` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`id`),
    KEY `ix_action_created` (`action`, `created_at`),
    KEY `ix_actor_created` (`actor_uid`, `created_at`),
    KEY `ix_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
