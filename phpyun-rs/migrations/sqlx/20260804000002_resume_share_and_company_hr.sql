CREATE TABLE IF NOT EXISTS `phpyun_rs_resume_share_tokens` (
    `token` VARCHAR(64) NOT NULL,
    `uid` INT UNSIGNED NOT NULL,
    `view_count` INT UNSIGNED NOT NULL DEFAULT 0,
    `expires_at` INT UNSIGNED NOT NULL,
    `revoked_at` INT UNSIGNED NOT NULL DEFAULT 0,
    `created_at` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`token`),
    KEY `ix_uid_created` (`uid`, `created_at`),
    KEY `ix_expires` (`expires_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `phpyun_rs_company_hrs` (
    `company_uid` INT UNSIGNED NOT NULL,
    `hr_uid` INT UNSIGNED NOT NULL,
    `role` VARCHAR(32) NOT NULL DEFAULT 'member',
    `joined_at` INT UNSIGNED NOT NULL,
    `status` TINYINT NOT NULL DEFAULT 1,
    PRIMARY KEY (`company_uid`, `hr_uid`),
    KEY `ix_hr_status` (`hr_uid`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
