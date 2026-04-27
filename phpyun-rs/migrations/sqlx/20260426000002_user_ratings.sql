-- User-facing ratings (rater scores a target with 1-5 stars + optional comment).
--
-- PHPYun's `phpyun_company_rating` is a VIP-package config table (43 cols), NOT
-- the user-rating table I previously assumed. The Rust rating service has been
-- shadow-writing into it with non-existent columns; this migration creates a
-- proper Rust-side table with the correct schema and the rating repo is
-- repointed at it. PHPYun's `phpyun_company_rating` stays untouched.
--
-- `phpyun_rs_rating`         — one row per (rater, target) pair.
-- `phpyun_rs_rating_aggregate` — cached count + avg (per target) for hot reads.
-- The two tables are kept in sync by `rating_repo::upsert` inside one tx.

CREATE TABLE IF NOT EXISTS `phpyun_rs_rating` (
    `id`          BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `rater_uid`   INT UNSIGNED    NOT NULL,
    `target_uid`  INT UNSIGNED    NOT NULL,
    -- 1=company / 2=jobseeker / 3=lt (headhunter).
    `target_kind` TINYINT         NOT NULL,
    `stars`       TINYINT         NOT NULL COMMENT '1..5',
    `comment`     VARCHAR(1024)   NOT NULL DEFAULT '',
    -- 0=hidden by admin / 1=visible. Default visible.
    `status`      TINYINT         NOT NULL DEFAULT 1,
    `created_at`  INT UNSIGNED    NOT NULL,
    `updated_at`  INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_rater_target` (`rater_uid`, `target_uid`, `target_kind`),
    KEY `ix_target` (`target_uid`, `target_kind`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `phpyun_rs_rating_aggregate` (
    `target_uid`  INT UNSIGNED    NOT NULL,
    `target_kind` TINYINT         NOT NULL,
    -- Number of ratings.
    `count`       INT UNSIGNED    NOT NULL DEFAULT 0,
    -- Sum of stars (avoid float drift; avg = sum / count, x100 cached).
    `sum_stars`   INT UNSIGNED    NOT NULL DEFAULT 0,
    -- Average × 100 (so 4.27 stars → 427). Saves client-side division.
    `avg_x100`    INT UNSIGNED    NOT NULL DEFAULT 0,
    `updated_at`  INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`target_uid`, `target_kind`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
