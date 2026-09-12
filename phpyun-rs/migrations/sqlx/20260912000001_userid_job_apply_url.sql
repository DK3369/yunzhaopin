-- Official apply URL on resume applications (scrape jobs). Additive.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

ALTER TABLE `phpyun_userid_job`
    ADD COLUMN `apply_url` VARCHAR(512) NOT NULL DEFAULT '' AFTER `job_name`;
