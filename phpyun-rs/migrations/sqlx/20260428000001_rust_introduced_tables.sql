-- Backfill the Rust-introduced tables that were referenced from `crates/models`
-- but never had a CREATE TABLE migration. Without these the corresponding
-- handlers return 500 at runtime ("table doesn't exist").
--
-- This migration is purely additive — no PHP table is touched. The naming
-- prefix `phpyun_rs_*` keeps Rust-only tables visually segregated from the
-- shared `phpyun_*` PHP-compat schema.

-- ==================== chat ====================
-- Peer-to-peer messages. `conv_key` is the canonical "min(a,b)-max(a,b)"
-- string so both directions of a conversation share an index.
CREATE TABLE IF NOT EXISTS `phpyun_rs_chat` (
    `id`           BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `sender_uid`   INT UNSIGNED    NOT NULL,
    `receiver_uid` INT UNSIGNED    NOT NULL,
    `conv_key`     VARCHAR(64)     NOT NULL,
    `body`         TEXT            NOT NULL,
    `is_read`      TINYINT         NOT NULL DEFAULT 0,
    `created_at`   INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    KEY `ix_conv_id`        (`conv_key`, `id`),
    KEY `ix_receiver_unread`(`receiver_uid`, `is_read`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ==================== broadcasts ====================
-- System-wide announcements written by admins. PHPYun has no equivalent
-- (its `phpyun_sysmsg` is per-recipient), so this is a fresh Rust table.
CREATE TABLE IF NOT EXISTS `phpyun_broadcast` (
    `id`              BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `title`           VARCHAR(200)    NOT NULL,
    `body`            TEXT            NOT NULL,
    -- 0=all / 1=jobseeker / 2=employer / 3=admin
    `target_usertype` TINYINT         NOT NULL DEFAULT 0,
    -- 0=draft / 1=published
    `status`          TINYINT         NOT NULL DEFAULT 1,
    `issuer_uid`      INT UNSIGNED    NOT NULL,
    `created_at`      INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    KEY `ix_status_id` (`status`, `id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Per-user read receipts. UNIQUE on (uid, broadcast_id) so the
-- `INSERT IGNORE INTO ... mark_read` pattern in the repo is idempotent.
CREATE TABLE IF NOT EXISTS `phpyun_rs_broadcast_reads` (
    `uid`          INT UNSIGNED NOT NULL,
    `broadcast_id` BIGINT UNSIGNED NOT NULL,
    `read_at`      INT UNSIGNED NOT NULL,
    PRIMARY KEY (`uid`, `broadcast_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ==================== HR invite codes ====================
-- Codes a company posts so a coworker can self-claim their HR seat.
CREATE TABLE IF NOT EXISTS `phpyun_rs_company_invite_codes` (
    `id`          BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `company_uid` INT UNSIGNED    NOT NULL,
    `code`        VARCHAR(64)     NOT NULL,
    `note`        VARCHAR(200)    NOT NULL DEFAULT '',
    `max_uses`    INT UNSIGNED    NOT NULL DEFAULT 1,
    `used_count`  INT UNSIGNED    NOT NULL DEFAULT 0,
    `expires_at`  INT UNSIGNED    NOT NULL DEFAULT 0,
    -- 0=disabled / 1=active
    `status`      TINYINT         NOT NULL DEFAULT 1,
    `created_at`  INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `ix_company_status` (`company_uid`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ==================== user VIP state ====================
-- Per-user VIP package + window. PHPYun's VIP state lives in
-- `phpyun_company_pay`-derived flags; this Rust-side table is the canonical
-- source-of-truth for the new flow (paid → mark_paid → activate).
CREATE TABLE IF NOT EXISTS `phpyun_rs_user_vip` (
    `uid`          INT UNSIGNED    NOT NULL,
    `package_code` VARCHAR(64)     NOT NULL,
    `started_at`   INT UNSIGNED    NOT NULL,
    `expires_at`   INT UNSIGNED    NOT NULL,
    `updated_at`   INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`uid`),
    KEY `ix_expires` (`expires_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ==================== view footprints ====================
-- Generic "X viewed Y" footprint for jobs / companies / resumes. Used by
-- "recently viewed" sidebars and view-count statistics.
CREATE TABLE IF NOT EXISTS `phpyun_rs_views` (
    `id`         BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `viewer_uid` INT UNSIGNED    NOT NULL,
    -- 1=job / 2=company / 3=resume
    `kind`       TINYINT         NOT NULL,
    `target_id`  INT UNSIGNED    NOT NULL,
    `datetime`   INT UNSIGNED    NOT NULL,
    PRIMARY KEY (`id`),
    KEY `ix_viewer_kind_dt` (`viewer_uid`, `kind`, `datetime`),
    KEY `ix_target` (`kind`, `target_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
