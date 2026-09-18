-- Employer VIP display names: VIP 0–6. RUN_MIGRATIONS_ON_BOOT=false: apply on `jobs` by hand.
-- VIP 0 = free / unpaid (id=3). VIP 1–3 rename 铜/银/金. VIP 4–6 clone 金牌 quotas.

UPDATE `phpyun_company_rating`
SET `name` = 'VIP 0', `explains` = 'VIP 0', `sort` = 0, `display` = 1
WHERE `id` = 3;

UPDATE `phpyun_company_rating`
SET `name` = 'VIP 1', `explains` = 'VIP 1', `sort` = 1
WHERE `id` = 4;

UPDATE `phpyun_company_rating`
SET `name` = 'VIP 2', `explains` = 'VIP 2', `sort` = 2
WHERE `id` = 5;

UPDATE `phpyun_company_rating`
SET `name` = 'VIP 3', `explains` = 'VIP 3', `sort` = 3
WHERE `id` = 6;

UPDATE `phpyun_company_rating`
SET `name` = 'VIP 0', `explains` = 'VIP 0'
WHERE `id` = 27 AND `name` = '免费会员';

UPDATE `phpyun_company_rating`
SET `display` = 0
WHERE `type` = 2 AND COALESCE(`deleted`, 0) = 0;

UPDATE `phpyun_company_statis` SET `rating_name` = 'VIP 0' WHERE `rating` = 3;
UPDATE `phpyun_company_statis` SET `rating_name` = 'VIP 1' WHERE `rating` = 4;
UPDATE `phpyun_company_statis` SET `rating_name` = 'VIP 2' WHERE `rating` = 5;
UPDATE `phpyun_company_statis` SET `rating_name` = 'VIP 3' WHERE `rating` = 6;

UPDATE `phpyun_company` SET `rating_name` = 'VIP 0' WHERE `rating` = 3;
UPDATE `phpyun_company` SET `rating_name` = 'VIP 1' WHERE `rating` = 4;
UPDATE `phpyun_company` SET `rating_name` = 'VIP 2' WHERE `rating` = 5;
UPDATE `phpyun_company` SET `rating_name` = 'VIP 3' WHERE `rating` = 6;

UPDATE `phpyun_admin_config` SET `config` = '3' WHERE `name` = 'com_rating';

INSERT INTO `phpyun_company_rating` (
    `name`, `service_price`, `integral_buy`, `yh_price`, `yh_integral`,
    `time_start`, `time_end`, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, `sort`, `display`, `explains`, `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, `service_time`, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, `deleted`
)
SELECT
    'VIP 4', '499', `integral_buy`, `yh_price`, `yh_integral`,
    0, 0, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, 4, 1, 'VIP 4', `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, 30, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, 0
FROM `phpyun_company_rating`
WHERE `id` = 6
  AND (SELECT COUNT(*) FROM (
      SELECT `id` FROM `phpyun_company_rating` WHERE `name` = 'VIP 4' AND COALESCE(`deleted`, 0) = 0
  ) _vip4) = 0;

INSERT INTO `phpyun_company_rating` (
    `name`, `service_price`, `integral_buy`, `yh_price`, `yh_integral`,
    `time_start`, `time_end`, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, `sort`, `display`, `explains`, `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, `service_time`, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, `deleted`
)
SELECT
    'VIP 5', '599', `integral_buy`, `yh_price`, `yh_integral`,
    0, 0, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, 5, 1, 'VIP 5', `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, 30, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, 0
FROM `phpyun_company_rating`
WHERE `id` = 6
  AND (SELECT COUNT(*) FROM (
      SELECT `id` FROM `phpyun_company_rating` WHERE `name` = 'VIP 5' AND COALESCE(`deleted`, 0) = 0
  ) _vip5) = 0;

INSERT INTO `phpyun_company_rating` (
    `name`, `service_price`, `integral_buy`, `yh_price`, `yh_integral`,
    `time_start`, `time_end`, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, `sort`, `display`, `explains`, `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, `service_time`, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, `deleted`
)
SELECT
    'VIP 6', '699', `integral_buy`, `yh_price`, `yh_integral`,
    0, 0, `resume`, `job_num`, `interview`, `editjob_num`,
    `breakjob_num`, 6, 1, 'VIP 6', `com_pic`, `com_color`, `type`,
    `lt_resume`, `lt_job_num`, `lt_editjob_num`, `lt_breakjob_num`, `category`,
    `msg_num`, 30, `coupon`, `part_num`, `editpart_num`, `breakpart_num`,
    `zph_num`, `service_discount`, `jobrec`, `top_num`, `urgent_num`, `rec_num`,
    `sons_num`, `xcx`, `xcx_num`, `freelook_num`, `freerefresh_num`, `suspend_num`,
    `max_time`, 0
FROM `phpyun_company_rating`
WHERE `id` = 6
  AND (SELECT COUNT(*) FROM (
      SELECT `id` FROM `phpyun_company_rating` WHERE `name` = 'VIP 6' AND COALESCE(`deleted`, 0) = 0
  ) _vip6) = 0;
