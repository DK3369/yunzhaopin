-- Fix `phpyun_region` ID columns to BIGINT UNSIGNED so they decode cleanly
-- into the entity's `u64` fields. (Original migration omitted UNSIGNED.)
-- Data is preserved — values were already non-negative.

ALTER TABLE `phpyun_region`
    MODIFY COLUMN `id`        BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    MODIFY COLUMN `parent_id` BIGINT UNSIGNED NULL COMMENT 'Self-ref; NULL for country-level rows';
