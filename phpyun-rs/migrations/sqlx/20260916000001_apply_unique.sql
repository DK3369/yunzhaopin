-- Unique apply per member+job. Write-only: RUN_MIGRATIONS_ON_BOOT=false,
-- execute on live `jobs` by hand. Duplicate rows must be cleaned first:
--   SELECT uid, job_id, COUNT(*) c FROM phpyun_userid_job GROUP BY uid, job_id HAVING c>1;

ALTER TABLE `phpyun_userid_job`
    ADD UNIQUE KEY `uk_userid_job_uid_job` (`uid`, `job_id`);
