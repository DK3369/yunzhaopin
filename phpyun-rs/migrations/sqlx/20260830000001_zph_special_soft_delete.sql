-- Soft-delete marker for job-fair / special topic main tables.
-- Idempotent: skip ADD COLUMN if `deleted` already exists.
-- Apply on jobs only (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

SET @db := DATABASE();

SET @exist := (
  SELECT COUNT(*) FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = @db AND TABLE_NAME = 'phpyun_zhaopinhui' AND COLUMN_NAME = 'deleted'
);
SET @sql := IF(
  @exist = 0,
  'ALTER TABLE `phpyun_zhaopinhui` ADD COLUMN `deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT ''0=active,1=deleted''',
  'SELECT 1'
);
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @exist := (
  SELECT COUNT(*) FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = @db AND TABLE_NAME = 'phpyun_special' AND COLUMN_NAME = 'deleted'
);
SET @sql := IF(
  @exist = 0,
  'ALTER TABLE `phpyun_special` ADD COLUMN `deleted` TINYINT(1) NOT NULL DEFAULT 0 COMMENT ''0=active,1=deleted''',
  'SELECT 1'
);
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
