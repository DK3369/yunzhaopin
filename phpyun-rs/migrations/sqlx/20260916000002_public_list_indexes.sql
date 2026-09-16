-- Public list composite indexes (job / company / resume / expect / apply / look_job).
-- Write-only: RUN_MIGRATIONS_ON_BOOT=false. Execute on live `jobs` by hand
-- (pt-online-schema-change or overnight window). Do not run from this round.

-- 1 职位公开列表主过滤 + 排序
CREATE INDEX ix_job_public_lastupdate
  ON phpyun_company_job (state, status, r_status, lastupdate);

-- 2 职位分站
CREATE INDEX ix_job_public_did_lastupdate
  ON phpyun_company_job (state, status, r_status, did, lastupdate);

-- 3 推荐 / 紧急侧栏
CREATE INDEX ix_job_public_rec_time
  ON phpyun_company_job (state, status, r_status, rec_time);
CREATE INDEX ix_job_public_urgent_time
  ON phpyun_company_job (state, status, r_status, urgent_time);

-- 4 企业公开列表（默认 jobtime/hits）
CREATE INDEX ix_company_public_jobtime
  ON phpyun_company (r_status, jobtime, hits);

-- 5 企业按刷新时间
CREATE INDEX ix_company_public_lastupdate
  ON phpyun_company (r_status, lastupdate);

-- 6 企业推荐位
CREATE INDEX ix_company_rec_hot
  ON phpyun_company (r_status, rec, hotstart, hottime);

-- 7 简历公开列表
CREATE INDEX ix_resume_public_lastupdate
  ON phpyun_resume (status, r_status, lastupdate);

-- 8 简历分站
CREATE INDEX ix_resume_public_did_lastupdate
  ON phpyun_resume (status, r_status, did, lastupdate);

-- 9 简历列表 EXISTS expect
CREATE INDEX ix_expect_defaults_public
  ON phpyun_resume_expect (uid, defaults, state, status, r_status);

-- 10 投递批查 / look_job 去重（列表 stamp / browse）
CREATE INDEX ix_userid_job_uid_job_isdel
  ON phpyun_userid_job (uid, job_id, isdel);
CREATE INDEX ix_look_job_uid_jobid
  ON phpyun_look_job (uid, job_id);
