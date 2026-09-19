-- Empty stripe_session_id cannot be UNIQUE. NULL = Session not created yet.
-- RUN_MIGRATIONS_ON_BOOT=false: apply on `jobs` by hand.

UPDATE `phpyun_rs_stripe_order`
   SET `stripe_session_id` = NULL
 WHERE `stripe_session_id` IS NULL
    OR TRIM(`stripe_session_id`) = '';

ALTER TABLE `phpyun_rs_stripe_order`
    MODIFY `stripe_session_id` VARCHAR(255) NULL DEFAULT NULL;

ALTER TABLE `phpyun_rs_stripe_order`
    DROP INDEX `ix_session`,
    ADD UNIQUE KEY `uk_session` (`stripe_session_id`);
