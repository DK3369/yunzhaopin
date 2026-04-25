-- Soft-delete support: add a `status` column to legacy tables that lack one.
-- Convention: status=0/1 active, status=2 deleted (no physical DELETE).
-- All list/detail queries must add `AND status != 2` to filter deleted rows out.
--
-- Note: `IF NOT EXISTS` is supported from MySQL 5.7; for older versions guard with schema-info checks.

-- phpyun_blacklist
ALTER TABLE `phpyun_blacklist`
    ADD COLUMN `status` INT(2) NOT NULL DEFAULT 0 COMMENT '0/1=active, 2=deleted';

-- phpyun_talent_pool
ALTER TABLE `phpyun_talent_pool`
    ADD COLUMN `status` INT(2) NOT NULL DEFAULT 0 COMMENT '0/1=active, 2=deleted';

-- phpyun_resumeout
ALTER TABLE `phpyun_resumeout`
    ADD COLUMN `status` INT(2) NOT NULL DEFAULT 0 COMMENT '0/1=active, 2=deleted';

-- phpyun_company_job_link (work addresses)
ALTER TABLE `phpyun_company_job_link`
    ADD COLUMN `status` INT(2) NOT NULL DEFAULT 0 COMMENT '0/1=active, 2=deleted';
